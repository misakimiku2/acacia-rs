use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GirlWeaponSkin_Change")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let mut girl = manager.items.get(param["nCardId"].as_u64().unwrap()).await?.unwrap();

    let skin_id = param["nSkinId"].as_i64().unwrap();
    if skin_id == -1 {
        girl.slots.insert(6, 0);
    } else {
        girl.slots.insert(6, skin_id as u64);
    }
    manager.items.set(girl.clone()).await?;

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
