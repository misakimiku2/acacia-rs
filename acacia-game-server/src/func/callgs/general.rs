use anyhow::Context;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("ChangeMainScene")]
async fn change_main_scene(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nId"].as_u64().with_context(|| format!("Expected 'nId' but not found in param {:?}", param))?;
    let manager = conn.manager().await;
    let attr = manager.attrs.set_attr(132, 1, nid as u32).await?;

    let rsp = protocol::NtfSyncPlayer {
        custom: HashMap::from([attr]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "ChangeMainScene".to_string(),
        arg: "[]".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("EnterGirlRoom")]
async fn enter_girl_room(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "EnterGirlRoom".to_string(),
        arg: json!({
            "bOpen": true,
            "nCardId": param["nCardId"],
            "nSkinId": param["nSkinId"],
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("RecordConfession")]
async fn record_confession(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nidx = param["nIdx"].as_u64().with_context(|| format!("Expected 'nIdx' but not found in param {:?}", param))? as u32;
    
    let manager = conn.manager().await;
    let attr = manager.attrs.set_attr(132, nidx + 10, 1).await?;

    let rsp = protocol::NtfSyncPlayer {
        custom: HashMap::from([attr]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "RecordConfession".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
