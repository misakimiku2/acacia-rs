use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

/// 悖论迷宫属性组
const ATTR_GID: u32 = 124;
const SID_CUR_DIFF: u32 = 5;
const SID_GAMEPLAY_ID: u32 = 6;
/// 四个难度的解锁位（LevelPassStart = 100，故 101..=104），值 1 = 已解锁
const SID_UNLOCK_DIFF_START: u32 = 101;
const SID_UNLOCK_DIFF_END: u32 = 104;

/// 初始化难度解锁位。
/// 不写这四个属性，难度选择界面拿不到解锁状态会整片空白。
async fn ensure_unlock_state(conn: &Arc<Conn>) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    let mut changed = false;
    for sid in SID_UNLOCK_DIFF_START..=SID_UNLOCK_DIFF_END {
        if me.get_attribute(ATTR_GID, sid).await? < 1 {
            me.set_attribute(ATTR_GID, sid, 1).await?;
            changed = true;
        }
    }
    if changed {
        me.sync().await?;
    }
    Ok(())
}

#[callgs_reg("Rogue3D_CheckOpenAct")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    ensure_unlock_state(&conn).await?;

    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_CheckOpenAct".to_string(),
        arg: json!({
            "nStartTime": 1689127200,
            "nEndTime": 1797710400,
            "nId": 1,
            "bOpen": true,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Rogue3D_SelectMode")]
async fn select_mode(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    ensure_unlock_state(&conn).await?;

    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_SelectMode".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Rogue3D_SelectDiff")]
async fn select_diff(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let ndiff_id = param["nDiffId"].as_i64().with_context(|| format!("Expected 'nDiffId' but not found in param {param:?}"))? as u32;

    // 关键：客户端进图时用 GameplayGroup[0] 当 gameplay 配置的 group 查表，
    // 只写 CurDiff 不写 GameplayId 就会「找不到gameplay配置，group=0」→ 空白场景
    let gameplay_id = tables()
        .rogue3d_difficult
        .find_one("difficult_id_1", ndiff_id)
        .and_then(|cfg| cfg.gameplay_group.as_ref())
        .and_then(|groups| groups.first().copied());

    let mut me = conn.manager().await;
    me.set_attribute(ATTR_GID, SID_CUR_DIFF, ndiff_id).await?;
    match gameplay_id {
        Some(gid) => me.set_attribute(ATTR_GID, SID_GAMEPLAY_ID, gid).await?,
        None => warn!("Rogue3D_SelectDiff: difficulty {ndiff_id} 没有 GameplayGroup 配置"),
    }

    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_SelectDiff".to_string(),
        extra_sync: Some(me.as_sync_player()),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Rogue3D_SelectTalent")]
async fn select_talent(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let ntalent_id = param["nTalentId"].as_i64().with_context(|| format!("Expected 'nTalentId' but not found in param {param:?}"))? as u32;

    let mut me = conn.manager().await;
    me.set_attribute(124, 7, ntalent_id).await?;

    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_SelectTalent".to_string(),
        extra_sync: Some(me.as_sync_player()),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Rogue3D_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    // 客户端用这个种子生成迷宫地图，固定 0 会导致每局布局完全一样
    let seed = rand::random::<u32>() % 1_000_000_000 + 1;
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_EnterLevel".to_string(),
        arg: json!({
            "nSeed": seed,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Rogue3D_SelectSeasonTalent")]
async fn select_season_talent(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_SelectSeasonTalent".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

// 以下四个：客户端 s2c handler 是空的，只需要回执，否则会卡在等待响应。

#[callgs_reg("Rogue3D_RoomSettlement")]
async fn room_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

#[callgs_reg("Rogue3D_SaveFightData")]
async fn save_fight_data(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

#[callgs_reg("Rogue3D_SeasonRoomSettlement")]
async fn season_room_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

#[callgs_reg("Rogue3D_SaveSeasonFightData")]
async fn save_season_fight_data(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

#[callgs_reg("Rogue3D_EnterSeasonLevel")]
async fn enter_season_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let seed = rand::random::<u32>() % 1_000_000_000 + 1;
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_EnterSeasonLevel".to_string(),
        arg: json!({
            "nSeed": seed,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
