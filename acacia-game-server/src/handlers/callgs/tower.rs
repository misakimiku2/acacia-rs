//! 地下清理（爬塔 / ClimbTower）相关接口。
//!
//! 上游 acacia 一个都没实现 —— 客户端点开该模式后请求无人应答，
//! 界面切到 Connection 弹「连接失败」。语义参考 MikuSB
//! (GameServer/Server/CallGS/Handlers/Tower/ClimbTowerLogic_*.cs)。
//!
//! 属性组 3（AttrIds.Tower）：
//!   sid 1 = 当前周期 ID，2 = 基础区进度，3 = 进阶区进度，
//!   4 = 当前难度，5 = 历史最高难度，
//!   100+周期ID = 奖励领取位，10000+关卡ID = 每关星数位图。

use chrono::NaiveDateTime;
use serde_json::Value;
use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

const ATTR_GID: u32 = 3;
const SID_TIME: u32 = 1;
const SID_PROGRESS_BASIC: u32 = 2;
const SID_PROGRESS_ADVANCED: u32 = 3;
const SID_DIFF: u32 = 4;
const SID_HISTORY_DIFF: u32 = 5;
const SID_REWARD_STATE_BASE: u32 = 100;
const SID_LEVEL_STATE_BASE: u32 = 10000;

/// "[202112010400]" -> unix timestamp
fn parse_cfg_time(raw: &str) -> Option<i64> {
    let s = raw.trim().trim_start_matches('[').trim_end_matches(']');
    if s.len() != 12 {
        return None;
    }
    NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M").ok().map(|dt| dt.and_utc().timestamp())
}

/// 当前周期：优先 now ∈ [start, end)，否则退化为最近一个已开始的周期
fn current_cycle() -> Option<u32> {
    let now = chrono::Utc::now().timestamp();
    let mut latest: Option<(i64, u32)> = None;
    let mut earliest: Option<(i64, u32)> = None;

    for cfg in tables().climb_tower_time.all() {
        let (Some(s), Some(e)) = (
            cfg.start_time.as_deref().and_then(parse_cfg_time),
            cfg.end_time.as_deref().and_then(parse_cfg_time),
        ) else {
            continue;
        };

        if s <= now && now < e {
            return Some(cfg.id);
        }
        if earliest.map_or(true, |(es, _)| s < es) {
            earliest = Some((s, cfg.id));
        }
        if s <= now && latest.map_or(true, |(ls, _)| s > ls) {
            latest = Some((s, cfg.id));
        }
    }

    latest.or(earliest).map(|(_, id)| id)
}

/// 周期检查：返回 timeID，客户端据此加载本赛季关卡配置。
/// 周期变化时更新记录，避免沿用上赛季进度。
#[callgs_reg("ClimbTowerLogic_CheckCycleLevel")]
async fn check_cycle_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let Some(cycle_id) = current_cycle() else {
        warn!("ClimbTowerLogic_CheckCycleLevel: climb_tower_time 表为空或无有效周期");
        call_script!({});
        return Ok(());
    };

    let mut me = conn.manager().await;
    let stored = me.get_attribute(ATTR_GID, SID_TIME).await?;
    if stored != cycle_id {
        me.set_attribute(ATTR_GID, SID_TIME, cycle_id).await?;
        me.sync().await?;
    }

    call_script!({"timeID": cycle_id});
    Ok(())
}

/// 选择难度。不能超过「历史最高难度 + 1」。
#[callgs_reg("ClimbTowerLogic_SetLevelDiff")]
async fn set_level_diff(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let diff = param["nDiff"].as_i64().unwrap_or(0);
    if diff <= 0 {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }

    let diff_u = diff as u32;
    if tables().climb_tower_diff.find_one("id_1", diff_u).is_none() {
        warn!("ClimbTowerLogic_SetLevelDiff: 难度 {diff_u} 不在 climb_tower_diff 表里");
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }

    let mut me = conn.manager().await;
    let his = me.get_attribute(ATTR_GID, SID_HISTORY_DIFF).await?;
    if diff_u > his.saturating_add(1) {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }

    me.set_attribute(ATTR_GID, SID_DIFF, diff_u).await?;
    let rsp = protocol::NtfCallScript {
        api: "ClimbTowerLogic_SetLevelDiff".to_string(),
        extra_sync: Some(me.as_sync_player()),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

/// 记录关卡进度：按区域写入星数位图，并更新当前进度指针。
#[callgs_reg("ClimbTowerLogic_RecordProgres")]
async fn record_progress(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().unwrap_or(0) as u32;
    let area = param["nArea"].as_i64().unwrap_or(0);
    let star = param["nStar"].as_i64().unwrap_or(0);

    if nid == 0 || area <= 0 {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }

    let area_idx = (area.clamp(1, 3) - 1) as u32;
    let mut me = conn.manager().await;

    let sid = SID_LEVEL_STATE_BASE + nid;
    let mut state = me.get_attribute(ATTR_GID, sid).await?;
    for i in 0..3u32 {
        if (star >> i) & 1 != 0 {
            state |= 1 << (area_idx * 3 + i);
        }
    }
    me.set_attribute(ATTR_GID, sid, state).await?;

    // 进度打包 (下一区域 << 24) | 关卡ID；三区域打满则清零
    let progress_sid = if area >= 4 { SID_PROGRESS_ADVANCED } else { SID_PROGRESS_BASIC };
    let packed = if area >= 3 {
        0
    } else {
        (((area as u32) + 1) << 24) | (nid & 0x00FF_FFFF)
    };
    me.set_attribute(ATTR_GID, progress_sid, packed).await?;

    me.sync().await?;
    call_script!({});
    Ok(())
}

/// 领取层奖励。
///
/// 四个组：0 = 首通，1/2/3 = 层内累计星数达标档位。
/// 每组占 4 bit（offset = group*4），已领取则该组位非零。
#[callgs_reg("ClimbTowerLogic_GetReward")]
async fn get_reward(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let layer_raw = param["nLayer"].as_i64().unwrap_or(0);
    let group_param = param["nGroup"].as_i64();

    if layer_raw <= 0 {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }
    let layer = layer_raw as u32;

    let Some(cycle_id) = current_cycle() else {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    };
    let Some(cycle) = tables().climb_tower_time.find_one("id_1", cycle_id) else {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    };

    let basic = cycle.level1.clone().unwrap_or_default();
    let advanced = cycle.level2.clone().unwrap_or_default();

    let mut me = conn.manager().await;

    // 基础区（Level1）难度固定 1；进阶区（Level2）用玩家当前选择的难度
    let (tower_ids, diff) = if layer as usize <= basic.len() {
        (basic[(layer - 1) as usize].clone(), 1u32)
    } else {
        let idx = (layer as usize) - basic.len();
        if idx == 0 || idx > advanced.len() {
            warn!(
                "ClimbTowerLogic_GetReward: 第 {layer} 层超出范围（基础区 {} 层 / 进阶区 {} 层）",
                basic.len(),
                advanced.len()
            );
            call_script!({"sErr": "error.BadParam"});
            return Ok(());
        }
        let d = me.get_attribute(ATTR_GID, SID_DIFF).await?;
        (advanced[idx - 1].clone(), if d == 0 { 1 } else { d })
    };

    // 奖励按难度分档：Diff 为空的通用配置，否则必须匹配当前难度
    let cfg = tables().climb_tower_award.find("id_1", layer).into_iter().find(|c| {
        let cfg_diff = c.diff.as_deref().map(str::trim).unwrap_or("");
        cfg_diff.is_empty() || cfg_diff.parse::<u32>().ok() == Some(diff)
    });

    let Some(cfg) = cfg else {
        warn!("ClimbTowerLogic_GetReward: 第 {layer} 层（难度 {diff}）没有奖励配置");
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    };

    let groups: Vec<i64> = match group_param {
        Some(g) if (0..=3).contains(&g) => vec![g],
        _ => vec![0, 1, 2, 3],
    };

    let reward_sid = SID_REWARD_STATE_BASE + layer;
    let mut claimed = me.get_attribute(ATTR_GID, reward_sid).await?;

    // 本层累计星数
    let mut layer_star = 0u32;
    for tid in &tower_ids {
        let v = me.get_attribute(ATTR_GID, SID_LEVEL_STATE_BASE + tid).await?;
        layer_star += (v & 0x1FF).count_ones();
    }

    // 首通判定：看过关记录；兼容旧存档（结算走过空回执、未写通过次数）时用星数兜底
    let mut passed = false;
    for tid in &tower_ids {
        let level_id = tables()
            .climb_tower_levelorder
            .find_one("id_1", *tid)
            .and_then(|o| o.level_id)
            .unwrap_or(*tid);
        if me.get_attribute(22, level_id).await? > 0 {
            passed = true;
            break;
        }
    }
    if !passed && layer_star > 0 {
        passed = true;
    }

    let mut granted: Vec<Vec<u32>> = Vec::new();

    for g in groups {
        let offset = (g as u32) * 4;
        if (claimed >> offset) & 0xF != 0 {
            continue;
        }

        let (ok, rewards) = match g {
            0 => {
                let list = cfg.first_award.clone().unwrap_or_default();
                (passed && !list.is_empty(), list)
            }
            1 => {
                let need = cfg.star_count1.unwrap_or(0);
                (need > 0 && layer_star >= need, cfg.star_award1.clone().unwrap_or_default())
            }
            2 => {
                let need = cfg.star_count2.unwrap_or(0);
                (need > 0 && layer_star >= need, cfg.star_award2.clone().unwrap_or_default())
            }
            3 => {
                let need = cfg.star_count3.unwrap_or(0);
                (need > 0 && layer_star >= need, cfg.star_award3.clone().unwrap_or_default())
            }
            _ => (false, Vec::new()),
        };

        if !ok {
            continue;
        }

        for row in &rewards {
            if row.len() >= 5 {
                me.add_item_count(row[0] as u16, row[1] as u16, row[2] as u16, row[3] as u16, row[4])
                    .await?;
                granted.push(row.clone());
            }
        }
        claimed |= 1 << offset;
    }

    if granted.is_empty() {
        call_script!({"sErr": "error.BadParam"});
        return Ok(());
    }

    me.set_attribute(ATTR_GID, reward_sid, claimed).await?;
    me.sync().await?;

    call_script!({"tbRewards": granted});
    Ok(())
}

/// 爬塔关卡结算（由 Chapter_DealLevelSettlement 的 sCmd 分发进来）。
/// param: {nID = 实际关卡ID, nTowerID = 爬塔关卡ID, nStar = 星数掩码}
pub async fn level_settlement(conn: Arc<Conn>, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().unwrap_or(0) as u32;
    let tower_id = param["nTowerID"].as_u64().unwrap_or(0) as u32;
    let star = param["nStar"].as_i64().unwrap_or(0);

    let mut me = conn.manager().await;

    if nid == 0 || tower_id == 0 {
        warn!("TowerLevel_LevelSettlement: 参数不完整 {param:?}");
        let rsp = protocol::NtfCallScript {
            api: "Chapter_DealLevelSettlement".to_string(),
            arg: serde_json::json!({"sCmd": "TowerLevel_LevelSettlement", "tbParam": {"sErr": "error.BadParam"}})
                .to_string(),
            ..protocol::NtfCallScript::default()
        };
        conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
        return Ok(());
    }

    // 最终区域（area=3）的星数位图占 bit 6..8
    let sid = SID_LEVEL_STATE_BASE + tower_id;
    let mut state = me.get_attribute(ATTR_GID, sid).await?;
    for i in 0..3u32 {
        if (star >> i) & 1 != 0 {
            state |= 1 << (6 + i);
        }
    }
    me.set_attribute(ATTR_GID, sid, state).await?;

    me.set_attribute(ATTR_GID, SID_PROGRESS_BASIC, 0).await?;
    me.set_attribute(ATTR_GID, SID_PROGRESS_ADVANCED, 0).await?;

    // 通关记录：首通奖励依赖这个属性
    let pass = me.get_attribute(22, nid).await?;
    me.set_attribute(22, nid, pass.saturating_add(1).max(1)).await?;

    me.sync().await?;

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: serde_json::json!({"sCmd": "TowerLevel_LevelSettlement", "tbParam": {}}).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("TowerLevel_EnterLevel")]
async fn tower_level_enter(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let seed = rand::random::<u32>() % 1_000_000_000 + 1;
    call_script!({"nSeed": seed});
    Ok(())
}

#[callgs_reg("TowerEventChapter_EnterLevel")]
async fn tower_event_enter(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let seed = rand::random::<u32>() % 1_000_000_000 + 1;
    call_script!({"nSeed": seed});
    Ok(())
}
