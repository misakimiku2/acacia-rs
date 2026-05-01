use anyhow::bail;
use std::collections::HashMap;

use crate::logic::{player::PlayerManager, riki::riki_unlock_item};
use acacia_commons::bits::{get_template, set_template};
use acacia_data::tables::tables;
use acacia_game_msg::core;

const GIRL_EXP_TABLE: [u32; 81] = [
    0, 400, 600, 800, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000, 3100, 3200, 3300, 3400, 3500, 3500, 3600, 3600,
    3700, 3700, 3800, 3800, 3900, 3900, 4000, 4200, 4500, 4800, 5100, 5400, 5700, 6000, 6300, 6600, 7000, 8000, 8300, 8600, 9900, 10500, 11200, 11800, 12500, 13200, 13900, 14100, 14900, 15600, 16400,
    17200, 18000, 18800, 19600, 20500, 21300, 24000, 27000, 30000, 33000, 36000, 39000, 42000, 45000, 48000, 51000, 0,
];

pub fn calc_girl_exp(current_level: u32, current_exp: u32, add_exp: u32) -> (u32, u32) {
    let mut level = current_level;
    let mut exp = current_exp;
    let mut gain = add_exp;
    loop {
        if level >= 80 {
            return (80, 0);
        }
        let need = GIRL_EXP_TABLE[level as usize];
        if need == 0 {
            return (level, 0);
        }
        let remain = need.saturating_sub(exp);
        if gain >= remain {
            gain -= remain;
            level += 1;
            exp = 0;
        } else {
            exp += gain;
            return (level, exp);
        }
    }
}

pub fn get_girl_break_materials(girl: &core::Item, break_lv: u32) -> anyhow::Result<HashMap<(u16, u16, u16, u16), u32>> {
    let girl_gdpl = get_template(girl.template);
    let Some(card_template_data) = tables().card_template.find_one("gdpl", [girl_gdpl.0, girl_gdpl.1, girl_gdpl.2, girl_gdpl.3]) else {
        bail!("The specified card_template_data does not exist. gdpl={:?}", girl_gdpl);
    };
    let mat = if let Some(card_break_data) = tables().card_break.find_one("id_1", card_template_data.break_mat_id) {
        match break_lv {
            1 => &card_break_data.items1_1,
            2 => &card_break_data.items1_2,
            3 => &card_break_data.items1_3,
            4 => &card_break_data.items1_4,
            5 => &card_break_data.items1_5,
            6 => &card_break_data.items1_6,
            7 => &card_break_data.items1_7,
            8 => &card_break_data.items1_8,
            9 => &card_break_data.items1_9,

            10 => &card_break_data.items2_1,
            11 => &card_break_data.items2_2,
            12 => &card_break_data.items2_3,
            13 => &card_break_data.items2_4,
            14 => &card_break_data.items2_5,
            15 => &card_break_data.items2_6,
            16 => &card_break_data.items2_7,
            17 => &card_break_data.items2_8,
            18 => &card_break_data.items2_9,

            19 => &card_break_data.items3_1,
            20 => &card_break_data.items3_2,
            21 => &card_break_data.items3_3,
            22 => &card_break_data.items3_4,
            23 => &card_break_data.items3_5,
            24 => &card_break_data.items3_6,
            25 => &card_break_data.items3_7,
            26 => &card_break_data.items3_8,
            27 => &card_break_data.items3_9,

            28 => &card_break_data.items4_1,
            29 => &card_break_data.items4_2,
            30 => &card_break_data.items4_3,
            31 => &card_break_data.items4_4,
            32 => &card_break_data.items4_5,
            33 => &card_break_data.items4_6,
            34 => &card_break_data.items4_7,
            35 => &card_break_data.items4_8,
            36 => &card_break_data.items4_9,

            37 => &card_break_data.items5_1,
            38 => &card_break_data.items5_2,
            39 => &card_break_data.items5_3,
            40 => &card_break_data.items5_4,
            41 => &card_break_data.items5_5,
            42 => &card_break_data.items5_6,
            43 => &card_break_data.items5_7,
            44 => &card_break_data.items5_8,
            45 => &card_break_data.items5_9,
            _ => bail!("Invalid girl break_lv: {}", break_lv),
        }
    } else if let Some(card_break_new_data) = tables().card_break_new.find_one("id_1", card_template_data.break_mat_id) {
        match break_lv {
            1 => &card_break_new_data.items1_1,
            2 => &card_break_new_data.items2_1,
            3 => &card_break_new_data.items3_1,
            4 => &card_break_new_data.items4_1,
            _ => bail!("Invalid girl break_lv: {}", break_lv),
        }
    } else {
        bail!("Invalid girl break_mat_id: {}", card_template_data.break_mat_id);
    };
    let mut result = HashMap::<(u16, u16, u16, u16), u32>::new();
    for (g, d, p, l, n) in mat {
        result.insert((*g, *d, *p, *l), *n);
    }
    Ok(result)
}

pub async fn new_girl(me: &mut PlayerManager, g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<()> {
    let Some(card_template_data) = tables().card_template.find_one("gdpl", [g, d, p, l]) else {
        bail!("The specified card_template_data does not exist. gdpl={:?}", [g, d, p, l]);
    };
    if !me.get_items_by_gdpl(g, d, p, l).await?.is_empty() {
        bail!("Girl already exists. gdpl={:?}", [g, d, p, l]);
    }
    

    let mut girl = core::Item {
        id: me.new_item_id().await?,
        template: set_template(g, d, p, l)?,
        count: 1,
        enhance: Some(core::Enhance { level: 1, ..Default::default() }),
        ..Default::default()
    };

    let df_weapon = card_template_data.default_weapon_gpdl;
    let mut weapon = core::Item {
        id: me.new_item_id().await?,
        template: set_template(df_weapon.0, df_weapon.1, df_weapon.2, df_weapon.3)?,
        count: 1,
        enhance: Some(core::Enhance { level: 1, ..Default::default() }),
        ..Default::default()
    };
    weapon.flag |= 1;

    let skins = me.get_items_by_gdpl(7, d, p, 1).await?;
    let mut skin = match skins.into_iter().next() {
        Some(skin) => skin,
        None => core::Item {
            id: me.new_item_id().await?,
            template: set_template(7, d, p, 1)?,
            count: 1,
            ..Default::default()
        },
    };
    skin.flag |= 1;

    girl.slots.insert(4, weapon.id);
    girl.slots.insert(5, skin.id);

    me.set_item(girl).await?;
    me.set_item(weapon).await?;
    me.set_item(skin).await?;
    for profile in &card_template_data.profile {
        me.add_item_count(profile.0, profile.1, profile.2, profile.3, profile.4).await?;
    }
    riki_unlock_item(me, 1, g, d, p, l).await?;
    riki_unlock_item(me, 2, df_weapon.0, df_weapon.1, df_weapon.2, df_weapon.3).await?;
    Ok(())
}
