use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::warn;

use super::boss_pvp_logic;
use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

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
    let rsp = protocol::NtfCallScript {
        api: "Chapter_NewPrologueEnterLevel".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_DealLevelSettlement")]
async fn deal_level_settlement(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let cmd = param["sCmd"].as_str().with_context(|| format!("Expected 'sCmd' but not found in param {:?}", param))?;

    match cmd {
        "Chapter_NewPrologueSettlement" => new_prologue_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "BossPvpLogic_LevelSettlement" => boss_pvp_logic::level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "BossPvpLogic_LevelFail" => boss_pvp_logic::level_fail(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        _ => {
            warn!("Chapter_DealLevelSettlement fn: {} not exists", param["sCmd"]);
        }
    }
    Ok(())
}

async fn new_prologue_settlement(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    deal_level_settlement_level_logger(conn.clone(), msgdata, param.clone()).await?;
    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "Chapter_NewPrologueSettlement",
            "tbParam": {
                "bWaitServer": param["bWaitServer"],
                // "tbShowAward": [], // WIP
            }
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn deal_level_settlement_level_logger(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_i64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))? as u32;
    let nstar = param["nStar"].as_i64().unwrap_or(-1);

    let mut me = conn.manager().await;
    let level_times = me.get_attribute(22, nid).await? + 1;
    me.set_attribute(22, nid, level_times).await?;
    if nstar > 0 {
        me.set_attribute(22, nid, nstar as u32).await?;
    }
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
    Ok(())
}
