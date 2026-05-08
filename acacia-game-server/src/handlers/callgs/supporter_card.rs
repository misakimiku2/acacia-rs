use anyhow::bail;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("SupporterCard_Equip")]
async fn equip(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["EqId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'EqId' as u32 in param {:?}", param);
    };
    let Ok(slot_id) = u32::deserialize(&param["EqSlot"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'EqSlot' as u32 in param {:?}", param);
    };
    let Ok(eq_id) = u64::deserialize(&param["beEqId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'beEqId' as u32 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.weapon_exchange_cmd_err").await?;
        bail!("Girl {girl_id} not found");
    };
    if let Some(mut before_girl) = me.get_items_by_slot(slot_id, eq_id).await?.into_iter().next() {
        before_girl.slots.insert(slot_id, 0);
        me.set_item(before_girl).await?;
    };
    girl.slots.insert(slot_id, eq_id);
    me.set_item(girl).await?;
    me.sync().await?;

    call_script!("Logistics_Equip", null);
    Ok(())
}
