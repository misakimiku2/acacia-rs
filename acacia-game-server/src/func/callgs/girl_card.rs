use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::{base::conn::Conn, logic::girl::calc_girl_exp};
use acacia_commons::bits::get_template;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Material {
    id: u64,
    num: u32,
}

#[callgs_reg("GirlCard_UpdateLevel")]
async fn update_level(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let girl_id = param["Id"].as_u64().with_context(|| format!("Expected 'Id' but not found in param {:?}", param))?;
    let materials: Vec<Material> = serde_json::from_value(param["tbMaterials"].clone())?;
    let mut me = conn.manager().await;
    let mut girl = me.get_item(girl_id).await?.with_context(|| format!("Girl not found {}", girl_id))?;

    let mut total_provide_exp: u32 = 0;
    let mut total_consume_gold: u32 = 0;
    let mut cached_items = Vec::new();
    for material in materials {
        let mut item = me.get_item(material.id).await?.with_context(|| format!("Item {} not found.", material.id))?;
        if item.count < material.num {
            conn.send_err("error.girlcard_cmd_err").await?;
            return Ok(());
        }
        let gdpl = get_template(item.template);
        let suplies_template_data = tables()
            .suplies_template
            .find_one("gdpl", [gdpl.0, gdpl.1, gdpl.2, gdpl.3])
            .with_context(|| format!("suplies_template_data {:?} not found.", gdpl))?;
        let supply_provide_exp = suplies_template_data.provide_exp.with_context(|| format!("Invalid EXP item {:?}", gdpl))?;
        let supply_consume_gold = suplies_template_data.consume_gold.unwrap_or(0);
        total_provide_exp = total_provide_exp
            .checked_add(supply_provide_exp.checked_mul(material.num).context("EXP overflow")?)
            .context("EXP overflow")?;
        total_consume_gold = total_consume_gold
            .checked_add(supply_consume_gold.checked_mul(material.num).context("Gold overflow")?)
            .context("Gold overflow")?;
        item.count -= material.num;
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

    let enhance = girl.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    let result = calc_girl_exp(enhance.level, enhance.exp, total_provide_exp);
    enhance.level = result.0;
    enhance.exp = result.1;
    me.set_item(girl).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "GirlCard_UpdateLevel".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
