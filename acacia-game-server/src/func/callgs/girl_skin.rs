use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GirlSkin_Change")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let mut girl = manager.items.get(param["CardId"].as_u64().unwrap()).await?.unwrap();
    let skin = manager.items.get(*girl.slots.get(&5).unwrap()).await?.unwrap();
    girl.slots.insert(5, param["Id"].as_u64().unwrap());
    manager.items.set(girl.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![skin, girl],
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "GirlSkin_Change".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("GirlSkin_ChangeSkinType")]
async fn change_skin_type(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let mut skin = manager.items.get(param["nSkinId"].as_u64().unwrap()).await?.unwrap();

    let ntype = param["nType"].as_i64().unwrap();
    skin.slots.insert(11, ntype as u64);
    manager.items.set(skin.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![skin],
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "GirlSkin_ChangeSkinType".to_string(),
        arg: json!({
            "nType": param["nType"]
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
