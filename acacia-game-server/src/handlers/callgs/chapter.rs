use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::warn;

use super::boss_pvp_logic;
use super::gacha_try;
use super::tower;
use crate::base::conn::Conn;
use crate::handlers::callgs::role;
use crate::logic::chapter::{chapter_level_drop, chapter_level_logger};
use crate::logic::drop::drop;
use acacia_commons::lua_utils::lua_serde;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

/// 属性组：20 = 章节星级奖励领取位图（sid = (章节ID<<8)|难度，bit N = 第 N 档）
const ATTR_CHAPTER_STAR_AWARD: u32 = 20;
/// 属性组：21 = 关卡星级（低 3 位为星数）
const ATTR_LEVEL_STAR: u32 = 21;

#[callgs_reg("Chapter_GetStarAward")]
async fn get_star_award(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let difficult = param["nDifficult"].as_u64().unwrap_or(0) as u32;
    let chapter_id = param["nChapterID"].as_u64().unwrap_or(0) as u32;
    // nIndex < 0 表示一键领取全部可领档位
    let award_index = param["nIndex"].as_i64().unwrap_or(-1);

    let mut granted: Vec<Vec<u32>> = Vec::new();

    let chapter = if chapter_id == 0 || difficult == 0 {
        None
    } else {
        tables().chapter_main.find_one("id_difficult_1", (chapter_id, difficult))
    };

    let Some(chapter) = chapter else {
        // 章节未配置：回空奖励，至少不让客户端卡在“连接失败”
        conn.send_script("Chapter_GetStarAward", json!({"tbAward": granted})).await?;
        return Ok(());
    };

    // StarAward: {{星数, {{g,d,p,l,n},...}}, ...}
    let awards: Vec<(u32, Vec<Vec<u32>>)> = chapter
        .star_award
        .as_deref()
        .and_then(|s| lua_serde::from_str(s).ok())
        .unwrap_or_default();
    let levels = chapter.level.clone().unwrap_or_default();
    let attr_sid = (chapter_id << 8) | difficult;

    let mut me = conn.manager().await;

    // 章节星数 = 章节内所有关卡星数之和
    let mut star_count = 0u32;
    for lv in &levels {
        star_count += (me.get_attribute(ATTR_LEVEL_STAR, *lv).await? & 0b111).count_ones();
    }

    let mut claimed = me.get_attribute(ATTR_CHAPTER_STAR_AWARD, attr_sid).await?;

    for (i, (required, rewards)) in awards.iter().enumerate() {
        let index = (i + 1) as u32;
        if index > 15 {
            break;
        }
        if award_index >= 0 && award_index != index as i64 {
            continue;
        }
        let bit = 1u32 << index;
        if claimed & bit != 0 || star_count < *required {
            continue;
        }
        for row in rewards {
            if row.len() >= 5 {
                me.add_item_count(row[0] as u16, row[1] as u16, row[2] as u16, row[3] as u16, row[4]).await?;
                granted.push(row.clone());
            }
        }
        claimed |= bit;
    }

    me.set_attribute(ATTR_CHAPTER_STAR_AWARD, attr_sid, claimed).await?;
    me.sync().await?;

    conn.send_script("Chapter_GetStarAward", json!({"tbAward": granted})).await?;
    Ok(())
}

#[callgs_reg("Chapter_LevelFail")]
async fn level_fail(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

/// 通用空结算回执：把客户端发来的 sCmd 原样回一次，tbParam 为空数组。
async fn settlement_ack(conn: Arc<Conn>, cmd: &str) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": cmd,
            "tbParam": []
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

/// 日常（养成材料 / 金币）关卡结算。tbParam: {nID, nStar, nSeed}
///
/// 日常关卡的配置在独立的 daily/level 表（ID 段 211+，不在 chapter/level 里），
/// 属性沿用 22（通过次数）与 21（低 3 位星数 + bit8 标记日常已完成）。
async fn daily_level_settlement(conn: Arc<Conn>, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().unwrap_or(0) as u32;
    let nstar = param["nStar"].as_i64().unwrap_or(0);

    let mut drop_result: Vec<Vec<u32>> = Vec::new();
    let mut me = conn.manager().await;

    let times = me.get_attribute(22, nid).await?;

    match tables().daily_level.find_one("id_1", nid) {
        Some(level_data) => {
            if let Some(player_exp) = level_data.player_exp {
                me.add_exp(player_exp).await?;
            }
            // 首通奖励只在第一次结算时发放
            if times == 0 {
                if let Some(ids) = &level_data.first_drop_id {
                    for drop_id in ids {
                        for ((g, d, p, l), num) in drop(*drop_id)? {
                            me.add_item_count(g, d, p, l, num).await?;
                            drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
                        }
                    }
                }
            }
            if let Some(ids) = &level_data.base_drop_id {
                for drop_id in ids {
                    for ((g, d, p, l), num) in drop(*drop_id)? {
                        me.add_item_count(g, d, p, l, num).await?;
                        drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
                    }
                }
            }
            if let Some(ids) = &level_data.random_drop_id {
                for drop_id in ids {
                    for ((g, d, p, l), num) in drop(*drop_id)? {
                        me.add_item_count(g, d, p, l, num).await?;
                        drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
                    }
                }
            }
        }
        None => warn!("Daily_LevelSettlement: level {nid} not found in daily/level table"),
    }

    me.set_attribute(22, nid, times.saturating_add(1)).await?;
    let old_state = me.get_attribute(21, nid).await?;
    let star_bits = if nstar > 0 { (nstar as u32) & 0b111 } else { 0 };
    me.set_attribute(21, nid, old_state | star_bits | (1 << 8)).await?;

    me.sync().await?;

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "Daily_LevelSettlement",
            "tbParam": drop_result
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_DealLevelSettlement")]
async fn deal_level_settlement(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let cmd = param["sCmd"].as_str().with_context(|| format!("Expected 'sCmd' but not found in param {:?}", param))?;

    match cmd {
        "Chapter_LevelSettlement" => level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "Chapter_NewPrologueSettlement" => new_prologue_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "BossPvpLogic_LevelSettlement" => boss_pvp_logic::level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "BossPvpLogic_LevelFail" => boss_pvp_logic::level_fail(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "GachaTry_LevelSettlement" => gacha_try::level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "Role_LevelSettlement" => role::level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "Daily_LevelSettlement" => daily_level_settlement(conn.clone(), param["tbParam"].clone()).await?,
        // 爬塔关卡结算：写星数位图 + 通关记录（首通奖励依赖）
        "TowerLevel_LevelSettlement" => tower::level_settlement(conn.clone(), param["tbParam"].clone()).await?,
        // 以下模式暂无奖励结算逻辑，回空结算包避免客户端卡在等待响应
        "TowerEventChapter_LevelSettlement" | "Rogue3D_LevelSettlement" => {
            settlement_ack(conn.clone(), cmd).await?
        }
        _ => {
            warn!("Chapter_DealLevelSettlement fn: {} not exists", param["sCmd"]);
        }
    }
    Ok(())
}

#[callgs_reg("Chapter_SyncGuideLevelPassData")]
async fn sync_guide_level_pass_data(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let tb_data: Vec<(u32, u32)> = serde_json::from_value(param["tbData"].clone()).with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;

    let mut me = conn.manager().await;
    for (nlevel_id, npass_time) in &tb_data {
        me.set_attribute(22, *nlevel_id, *npass_time).await?;
    }
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Chapter_EnterLevel".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_NewPrologueEnterLevel")]
async fn new_prologue_enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp: protocol::NtfCallScript = protocol::NtfCallScript {
        api: "Chapter_NewPrologueEnterLevel".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    let drop_result = chapter_level_drop(&mut me, &param).await?;
    chapter_level_logger(&mut me, &param).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "Chapter_LevelSettlement",
            "tbParam": [drop_result]
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_NewPrologueSettlement")]
async fn new_prologue_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    let mut show_award = Vec::new();
    let drop_result = chapter_level_drop(&mut me, &param).await?;
    if !drop_result.is_empty() {
        show_award.push(drop_result);
    }
    chapter_level_logger(&mut me, &param).await?;
    me.sync().await?;

    call_script!({"bWaitServer": param["bWaitServer"], "tbShowAward": show_award});
    Ok(())
}
