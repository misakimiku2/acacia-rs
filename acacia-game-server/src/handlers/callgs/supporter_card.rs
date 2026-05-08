use anyhow::bail;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::{base::conn::Conn, logic::support::{SusLevelUpMat, support_card_level_up}};
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol};

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
        conn.send_err("error.logistic_cmd_err").await?;
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


#[callgs_reg("SupporterCard_Upgrade")]
async fn upgrade(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(eq_id) = u64::deserialize(&param["Id"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'Id' as u32 in param {:?}", param);
    };
    let Ok(mats) = Vec::<SusLevelUpMat>::deserialize(&param["tbMaterials"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'tbMaterials' as Vec<(u64, u32)> in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut sus) = me.get_item(eq_id).await? else {
        conn.send_err("error.logistics_not").await?;
        bail!("SupporterCard {eq_id} not found");
    };
    let total_consume_gold = match support_card_level_up(&me, &mut sus, &mats).await {
        Ok(v) => v,
        Err(e) => {
            conn.send_err("error.logistic__update_failed").await?;
            return Err(e);
        }
    };

    let mut cached_items = Vec::<core::Item>::new();
    for mat in &mats {
        let Some(mut item) = me.get_item(mat.id).await? else {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        };
        if item.count < mat.num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
        item.count -= mat.num;
        cached_items.push(item);
    }

    let mut current_gold = me.get_money(3).await?;
    if current_gold < total_consume_gold {
        conn.send_err("error.gold_not_enough").await?;
        return Ok(());
    }
    current_gold -= total_consume_gold;

    me.set_money(3, current_gold).await?;
    for item in cached_items {
        me.set_item(item).await?;
    }
    me.set_item(sus).await?;
    me.sync().await?;

    call_script!("Logistics_Upgrade", null);
    Ok(())
}