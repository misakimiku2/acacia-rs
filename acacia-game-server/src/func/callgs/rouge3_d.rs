use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Rogue3D_CheckOpenAct")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
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

    let mut me = conn.manager().await;
    me.set_attribute(124, 5, ndiff_id).await?;

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
async fn enter_season_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_EnterLevel".to_string(),
        arg: json!({
            "nSeed": 0,
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

#[callgs_reg("Rogue3D_EnterSeasonLevel")]
async fn enter_season_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_EnterSeasonLevel".to_string(),
        arg: json!({
            "nSeed": 0,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
