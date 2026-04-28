use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::warn;

use super::boss_pvp_logic;
use crate::base::conn::Conn;
use crate::logic::chapter::chapter_level_logger;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Chapter_DealLevelSettlement")]
async fn deal_level_settlement(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let cmd = param["sCmd"].as_str().with_context(|| format!("Expected 'sCmd' but not found in param {:?}", param))?;

    match cmd {
        "Chapter_LevelSettlement" => level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "Chapter_NewPrologueSettlement" => new_prologue_settlement(conn.clone(), msgdata, param["tbParam"].clone(), true).await?,
        "BossPvpLogic_LevelSettlement" => boss_pvp_logic::level_settlement(conn.clone(), msgdata, param["tbParam"].clone()).await?,
        "BossPvpLogic_LevelFail" => boss_pvp_logic::level_fail(conn.clone(), msgdata, param["tbParam"].clone()).await?,
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
    chapter_level_logger(&mut me, &param).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "Chapter_LevelSettlement",
            "tbParam": {}
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_NewPrologueSettlement")]
async fn new_prologue_settlement_(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    new_prologue_settlement(conn, msgdata, param, true).await?;
    Ok(())
}

async fn new_prologue_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value, from_deal_level_settlement: bool) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    chapter_level_logger(&mut me, &param).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let tb_param = json!({
        "bWaitServer": param["bWaitServer"],
        "tbShowAward": [], // WIP
    });

    let rsp = if from_deal_level_settlement {
        protocol::NtfCallScript {
            api: "Chapter_DealLevelSettlement".to_string(),
            arg: json!({
                "sCmd": "Chapter_NewPrologueSettlement",
                "tbParam": tb_param,
            })
            .to_string(),
            ..protocol::NtfCallScript::default()
        }
    } else {
        protocol::NtfCallScript {
            api: "Chapter_NewPrologueSettlement".to_string(),
            arg: tb_param.to_string(),
            ..protocol::NtfCallScript::default()
        }
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
