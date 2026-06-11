use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GirlSkin_Change")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let card_id = param["CardId"].as_u64().with_context(|| format!("Expected 'CardId' but not found in param {:?}", param))?;
    let skin_id = param["Id"].as_u64().with_context(|| format!("Expected 'CardId' but not found in param {:?}", param))?;

    let database = conn.database().await;
    let mut girl = database.items.find_one(card_id).await?.with_context(|| format!("Girl not found {}", card_id))?;
    let before_skin_id = girl.slots.get(&5).with_context(|| format!("Girl skin slot not found {:?}", girl))?;
    let before_skin = database
        .items
        .find_one(*before_skin_id)
        .await?
        .with_context(|| format!("Girl before skin not found {:?}", before_skin_id))?;
    girl.slots.insert(5, skin_id);
    database.items.update_one(girl.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![before_skin, girl],
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
    let nskin_id = param["nSkinId"].as_u64().with_context(|| format!("Expected 'nSkinId' but not found in param {:?}", param))?;
    let ntype = param["nType"].as_i64().with_context(|| format!("Expected 'nType' but not found in param {:?}", param))?;

    let database = conn.database().await;
    let mut skin = database.items.find_one(nskin_id).await?.with_context(|| format!("Skin not found {}", nskin_id))?;
    skin.slots.insert(11, ntype as u64);
    database.items.update_one(skin.clone()).await?;

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
