use anyhow::{Context, bail};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::logic::girl::{calc_girl_exp, get_girl_break_materials};
use crate::logic::girl_spine::get_girl_spine_max_count;
use acacia_commons::bits::get_template;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Material {
    id: u64,
    num: u32,
}

#[callgs_reg("GirlCard_UpdateLevel")]
async fn update_level(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(girl_id) = param["Id"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'Id' but not found in param {:?}", param);
    };
    let materials: Vec<Material> = match serde_json::from_value(param["tbMaterials"].clone()) {
        Ok(v) => v,
        Err(_) => {
            conn.send_err("error.BadParam").await?;
            bail!("Expected 'tbMaterials' but not found in param {:?}", param);
        }
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };

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
    me.sync().await?;

    conn.send_script("GirlCard_UpdateLevel", ()).await?;
    Ok(())
}

#[callgs_reg("GirlCard_ProLevelPromote")]
async fn pro_level_promote(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["nID"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nID' as u64 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };

    let spine_max_count = get_girl_spine_max_count(&girl);
    let max_pro_level = match spine_max_count {
        1 | 2 => 1,
        3 | 4 => 2,
        5 | 6 => 3,
        _ => 0,
    };
    let enhance = girl.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    if enhance.pro_level + 1 > max_pro_level {
        conn.send_err("error.girlcard_update_failed").await?;
        bail!("Girl card pro_level reached max: current pro_level={}, max_pro_level={}", enhance.pro_level, max_pro_level);
    }
    enhance.pro_level += 1;
    me.set_item(girl).await?;
    me.sync().await?;

    conn.send_script("GirlCard_ProLevelPromote", ()).await?;
    Ok(())
}

#[callgs_reg("GirlCard_UpByBreak")]
async fn up_by_break(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["pId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'pId' as u64 in param {:?}", param);
    };
    let Ok(break_lv) = u32::deserialize(&param["nBreakLv"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nBreakLv' as u32 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };
    let cached_break_mat = match get_girl_break_materials(&girl, break_lv) {
        Ok(v) => v,
        Err(e) => {
            conn.send_err("error.BadParam").await?;
            return Err(e);
        }
    };

    for (mat_gdpl, mat_num) in &cached_break_mat {
        let item_count = me.get_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3).await?;
        if item_count < *mat_num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
    }

    let enhance = girl.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    if enhance.r#break + 1 != break_lv {
        conn.send_err("error.girlcard_update_failed").await?;
        bail!("Girl card break mismatched: current break={}, break_lv={}", enhance.r#break, break_lv);
    }
    enhance.r#break += 1;

    for (mat_gdpl, mat_num) in &cached_break_mat {
        me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
    }
    me.set_item(girl).await?;
    me.sync().await?;

    conn.send_script("GirlCard_UpByBreak", ()).await?;
    Ok(())
}
