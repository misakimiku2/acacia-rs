use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Weapon_ReplacePart")]
async fn replace_part(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let slot_id = param["Type"].as_i64().unwrap() as u32;
    let part_id = param["PartId"].as_i64().unwrap();

    let mut weapon = manager.items.get(param["Id"].as_u64().unwrap()).await?.unwrap();
    if part_id == -1 {
        weapon.slots.remove(&slot_id);
    } else {
        weapon.slots.insert(slot_id, part_id as u64);
    }
    manager.items.set(weapon.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![weapon],
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "Weapon_ReplacePart".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
