use anyhow::{Context, bail};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::logic::girl::{calc_girl_exp, get_girl_break_materials, new_girl};
use crate::logic::girl_spine::{SpineConfigs, get_girl_spine_max_count};
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
    let Ok(girl_id) = u64::deserialize(&param["Id"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'Id' as u64 in param {:?}", param);
    };
    let Ok(materials) = Vec::<Material>::deserialize(&param["tbMaterials"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'tbMaterials' as Vec<Material> in param {:?}", param);
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

#[callgs_reg("GirlCard_ReceiveProLevelAward")]
async fn receive_pro_level_award(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["nID"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nID' as u64 in param {:?}", param);
    };
    let pro_level: Option<u32> = match param.get("nProLevel") {
        None => None,
        Some(value) => {
            let Ok(pro_level) = u32::deserialize(value) else {
                conn.send_err("error.BadParam").await?;
                bail!("Expected 'nProLevel' as u32 in param {:?}", param);
            };
            if pro_level > 3 {
                conn.send_err("error.BadParam").await?;
                bail!("Bad pro_level {}", pro_level);
            }
            Some(pro_level)
        }
    };

    let mut me = conn.manager().await;
    let Some(girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };
    let current_pro_level = girl.enhance.as_ref().map_or(0, |e| e.pro_level);
    let girl_gdpl = get_template(girl.template);

    let mut configs = SpineConfigs::from(&me, &girl).await?;
    let config = configs.get_config();
    config.tbr.resize(3, 0);

    let mut reward_count: u32 = 0;
    if let Some(level) = pro_level {
        if current_pro_level < level {
            conn.send_err("tip.star_not_enough").await?;
            return Ok(());
        }
        if config.tbr[level as usize - 1] == 1 {
            conn.send_err("tip.star_award_geted").await?;
            bail!("Award already received, girl_id: {}, pro_level: {}", girl_id, level);
        }
        config.tbr[level as usize - 1] = 1;
        reward_count += 1;
    } else {
        for (index, value) in config.tbr.iter_mut().enumerate() {
            if current_pro_level < (index + 1) as u32 {
                continue;
            }
            if *value == 1 {
                continue;
            }
            *value = 1;
            reward_count += 1;
        }
    }
    me.add_item_count(5, 21, 1, 4, reward_count).await?;
    me.set_str_attribute(30, girl_gdpl.1 as u32, configs.to_string()?).await?;
    me.sync().await?;

    conn.send_script("GirlCard_ReceiveProLevelAward", json!({"tbAward": [[5, 21, 1, 4, reward_count]]})).await?;
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

#[callgs_reg("GirlCard_UnLockCharacter")]
async fn un_lock_character(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(g) = u16::deserialize(&param["g"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'g' as u16 in param {:?}", param);
    };
    let Ok(d) = u16::deserialize(&param["d"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'd' as u16 in param {:?}", param);
    };
    let Ok(p) = u16::deserialize(&param["p"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'p' as u16 in param {:?}", param);
    };
    let Ok(l) = u16::deserialize(&param["l"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'l' as u16 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(card_template_data) = tables().card_template.find_one("gdpl", [g, d, p, l]) else {
        bail!("The specified card_template_data does not exist. gdpl={:?}", [g, d, p, l]);
    };
    let Some(pieces) = &card_template_data.pieces else {
        bail!("The specified card_template_data does not have pieces. gdpl={:?}", [g, d, p, l]);
    };
    let mut pieces_items = HashMap::<(u16, u16, u16, u16), u32>::new();
    for (g, d, p, l, n) in pieces {
        pieces_items.insert((*g, *d, *p, *l), *n);
    }

    for (mat_gdpl, mat_num) in &pieces_items {
        let item_count = me.get_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3).await?;
        if item_count < *mat_num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
    }

    for (mat_gdpl, mat_num) in &pieces_items {
        me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
    }
    new_girl(&mut me, g, d, p, l).await?;
    me.sync().await?;

    conn.send_script("GirlCard_UnLockCharacter", ()).await?;
    Ok(())
}

#[callgs_reg("GirlCard_SetRoleLikeFlag")]
async fn set_role_like_flag(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["nCardID"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nCardID' as u64 in param {:?}", param);
    };
    let Ok(flag) = u8::deserialize(&param["nFlag"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nFlag' as u8 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };
    if flag != 0 {
        girl.flag |= 64;
    } else {
        girl.flag &= !64;
    }
    me.set_item(girl).await?;
    me.sync().await?;

    conn.send_script("GirlCard_SetRoleLikeFlag", ()).await?;
    Ok(())
}

#[callgs_reg("GirlCard_UpBySpecialBreak")]
async fn up_by_special_break(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(girl_id) = u64::deserialize(&param["nCardId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nCardId' as u64 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };
    let current_break = girl.enhance.as_ref().map_or(0, |e| e.r#break);
    let cached_break_mat = match get_girl_break_materials(&girl, current_break + 1) {
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
    enhance.r#break += 1;

    for (mat_gdpl, mat_num) in &cached_break_mat {
        me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
    }
    me.set_item(girl).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}
