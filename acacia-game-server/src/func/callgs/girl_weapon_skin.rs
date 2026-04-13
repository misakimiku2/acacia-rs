use anyhow::Context;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GirlWeaponSkin_Change")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let ncard_id = param["nCardId"].as_u64().with_context(|| format!("Expected 'nCardId' but not found in param {:?}", param))?;
    let nskin_id = param["nSkinId"].as_i64().with_context(|| format!("Expected 'nSkinId' but not found in param {:?}", param))?;

    let database = conn.database().await;
    let mut girl = database.items.find_one(ncard_id).await?.with_context(|| format!("Girl not found {}", ncard_id))?;

    if nskin_id == -1 {
        girl.slots.remove(&6);
    } else {
        girl.slots.insert(6, nskin_id as u64);
    }
    database.items.update_one(girl.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![girl],
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "GirlWeaponSkin_Change".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
