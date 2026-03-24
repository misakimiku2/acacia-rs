use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GirlSkin_Change")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let mut girl = manager.items.get(param["CardId"].as_u64().unwrap()).await?.unwrap();
    girl.slots.insert(5, param["Id"].as_u64().unwrap());
    let girl = manager.items.update(girl).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![girl],
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
