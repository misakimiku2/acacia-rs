use anyhow::{Context, bail};
use tracing::warn;

use crate::logic::player::PlayerManager;
use acacia_commons::bits::get_template;
use acacia_data::tables::tables;
use acacia_game_msg::core;

const WEAPON_EXP_TABLE: [u32; 80] = [
    0, 100, 100, 200, 200, 300, 400, 400, 400, 500, 600, 700, 700, 800, 800, 800, 900, 1000, 1100, 1100, 1200, 1300, 1400, 1500, 1600, 1600, 1700, 1800, 1900, 2000, 2000, 2100, 2200, 2300, 2400,
    2500, 2700, 2700, 2800, 2800, 2900, 3200, 3300, 3600, 3700, 4000, 4100, 4400, 4600, 4800, 5000, 5200, 5500, 5600, 5900, 6100, 6400, 6600, 6800, 7200, 7400, 7700, 8100, 8600, 9000, 9400, 9900,
    10300, 10800, 11200, 11700, 13100, 14300, 15600, 16800, 18100, 19500, 20800, 22100, 23500,
];

const SSR_WEAPON_EXP_TABLE: [u32; 80] = [
    0, 200, 200, 300, 300, 400, 500, 500, 600, 700, 800, 900, 900, 1000, 1100, 1100, 1200, 1300, 1400, 1400, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000,
    3200, 3400, 3400, 3500, 3600, 3700, 4000, 4200, 4500, 4700, 5000, 5200, 5500, 5800, 6000, 6300, 6600, 6900, 7100, 7400, 7700, 8000, 8300, 8600, 9000, 9300, 9700, 10200, 10800, 11300, 11800,
    12400, 12900, 13500, 14100, 14700, 16400, 17900, 19500, 21100, 22700, 24400, 26000, 27700, 29400,
];

fn get_weapon_max_level(break_lv: u32) -> u32 {
    // break_level_limit.txt
    match break_lv {
        0 => 20,
        1 => 30,
        2 => 40,
        3 => 50,
        4 => 60,
        5 => 70,
        6 => 80,
        _ => 80,
    }
}

fn get_weapon_break_demand_level(break_lv: u32) -> u32 {
    match break_lv {
        0 => 0,
        1 => 10,
        2 => 20,
        3 => 30,
        4 => 40,
        5 => 50,
        6 => 60,
        _ => 999,
    }
}

fn get_weapon_break_materials(weapon: &core::Item, break_lv: u32) -> Option<&'static Vec<acacia_data::schemas::GDPLN>> {
    let weapon_gdpl = get_template(weapon.template);
    let Some(weapon_template_data) = tables().weapon_template.find_one("gdpl", [weapon_gdpl.0, weapon_gdpl.1, weapon_gdpl.2, weapon_gdpl.3]) else {
        warn!("The specified support_furniture_data does not exist. gdpl={:?}", weapon_gdpl);
        return None;
    };
    let Some(item_break_data) = tables().item_break.find_one("id_1", weapon_template_data.break_mat_id) else {
        warn!("The specified item_break_data does not exist. id={}", weapon_template_data.break_mat_id);
        return None;
    };
    match break_lv {
        1 => Some(&item_break_data.items1),
        2 => Some(&item_break_data.items2),
        3 => Some(&item_break_data.items3),
        4 => Some(&item_break_data.items4),
        5 => Some(&item_break_data.items5),
        6 => Some(&item_break_data.items6),
        _ => None,
    }
}

fn calc_weapon_exp(current_level: u32, current_exp: u32, add_exp: u32, color: u32, max_level: u32) -> (u32, u32) {
    let table = if color >= 5 { &SSR_WEAPON_EXP_TABLE } else { &WEAPON_EXP_TABLE };
    let hard_cap = max_level.min(80);
    let mut level = current_level;
    let mut exp = current_exp;
    let mut gain = add_exp;
    loop {
        if level >= hard_cap {
            return (hard_cap, 0);
        }
        let need = table[level as usize];
        if need == 0 {
            return (level, 0);
        }
        let remain = need.saturating_sub(exp);
        if gain >= remain {
            gain -= remain;
            level += 1;
            exp = 0;
            if level >= hard_cap {
                return (level, 0);
            }
        } else {
            exp += gain;
            return (level, exp);
        }
    }
}

pub async fn weapon_level_up(me: &PlayerManager, weapon: &mut core::Item, mat: &Vec<(u64, u32)>) -> anyhow::Result<u32> {
    let weapon_gdpl = get_template(weapon.template);
    let Some(weapon_template_data) = tables().weapon_template.find_one("gdpl", [weapon_gdpl.0, weapon_gdpl.1, weapon_gdpl.2, weapon_gdpl.3]) else {
        bail!("weapon_template_data not found {:?}", weapon_gdpl);
    };

    let mut total_upgrade_exp: u32 = 0;
    let mut total_consume_gold: u32 = 0;
    for (mat_item_id, mat_count) in mat {
        let mat_item = me.get_item(*mat_item_id).await?.with_context(|| format!("Item {} not found", mat_item_id))?;
        let gdpl = get_template(mat_item.template);

        if let Some(suplies_template_data) = tables().suplies_template.find_one("gdpl", [gdpl.0, gdpl.1, gdpl.2, gdpl.3]) {
            let supply_provide_exp = suplies_template_data.provide_exp.with_context(|| format!("Invalid EXP item {:?}", gdpl))?;
            let supply_consume_gold = suplies_template_data.consume_gold.unwrap_or(0);
            total_upgrade_exp = total_upgrade_exp
                .checked_add(supply_provide_exp.checked_mul(*mat_count).context("EXP overflow")?)
                .context("EXP overflow")?;
            total_consume_gold = total_consume_gold
                .checked_add(supply_consume_gold.checked_mul(*mat_count).context("Gold overflow")?)
                .context("Gold overflow")?;
        } else if let Some(weapon_template_data) = tables().weapon_template.find_one("gdpl", [gdpl.0, gdpl.1, gdpl.2, gdpl.3]) {
            let supply_provide_exp = weapon_template_data.provide_exp.unwrap_or(0);
            let supply_consume_gold = weapon_template_data.consume_gold.unwrap_or(0);
            total_upgrade_exp = total_upgrade_exp
                .checked_add(supply_provide_exp.checked_mul(*mat_count).context("EXP overflow")?)
                .context("EXP overflow")?;
            total_consume_gold = total_consume_gold
                .checked_add(supply_consume_gold.checked_mul(*mat_count).context("Gold overflow")?)
                .context("Gold overflow")?;
        } else {
            bail!("item not found {:?}", gdpl);
        }
    }
    let enhance = weapon.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    let current_max_level = get_weapon_max_level(enhance.r#break);
    let (new_level, new_exp) = calc_weapon_exp(enhance.level, enhance.exp, total_upgrade_exp, weapon_template_data.color, current_max_level);
    enhance.level = new_level;
    enhance.exp = new_exp;
    Ok(total_consume_gold)
}

pub async fn weapon_break_up(me: &PlayerManager, weapon: &mut core::Item) -> anyhow::Result<(u32, &'static Vec<acacia_data::schemas::GDPLN>)> {
    let current_level = weapon.enhance.as_ref().map_or(1, |e| e.level);
    let current_break = weapon.enhance.as_ref().map_or(0, |e| e.r#break);
    let current_max_level = get_weapon_max_level(current_break);
    if me.level().await? < get_weapon_break_demand_level(current_break + 1) {
        bail!("Insufficient account level.");
    }
    if current_level < current_max_level {
        bail!("Weapon level mismatc.");
    };
    let mat = get_weapon_break_materials(&weapon, current_break + 1).with_context(|| format!("break materials missing: weapon={:?}, break={}", weapon, current_break + 1))?;
    let mut total_consume_gold: u32 = 0;

    for gdpln in mat {
        if let Some(suplies_template_data) = tables().suplies_template.find_one("gdpl", [gdpln.0, gdpln.1, gdpln.2, gdpln.3]) {
            let supply_consume_gold = suplies_template_data.consume_gold.unwrap_or(0);
            total_consume_gold = total_consume_gold
                .checked_add(supply_consume_gold.checked_mul(gdpln.4).context("Gold overflow")?)
                .context("Gold overflow")?;
        } else {
            bail!("item not found {:?}", gdpln);
        }
    }

    let enhance = weapon.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    enhance.r#break = current_break + 1;
    Ok((total_consume_gold, mat))
}

pub fn weapon_has_parts_reward(weapon: &core::Item) -> anyhow::Result<bool> {
    let weapon_gdpl = get_template(weapon.template);
    let Some(weapon_template_data) = tables().weapon_template.find_one("gdpl", [weapon_gdpl.0, weapon_gdpl.1, weapon_gdpl.2, weapon_gdpl.3]) else {
        bail!("weapon_template_data not found {:?}", weapon_gdpl);
    };
    Ok(weapon_template_data.weapon_parts_award.is_some())
}


pub fn weapon_set_equip(weapon: &mut core::Item, enabled: bool) {
    if enabled {
        weapon.flag |= 1;
    } else {
        weapon.flag &= !1;
    }
}
