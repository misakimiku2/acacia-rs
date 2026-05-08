use acacia_commons::bits::{get_template, set_template};
use acacia_data::tables::tables;
use acacia_game_msg::core;
use anyhow::{Context, bail};
use rand::{RngExt, seq::IndexedRandom};
use serde::Deserialize;

use crate::logic::{player::PlayerManager, riki::riki_unlock_item};

const SUS_EXP_TABLE: [u32; 17] = [0, 3000, 4800, 6000, 7800, 9600, 12000, 15600, 19200, 24600, 30600, 37800, 48000, 61800, 77400, 94200, 0];

pub fn calc_sus_exp(current_level: u32, current_exp: u32, add_exp: u32) -> (u32, u32) {
    let mut level = current_level;
    let mut exp = current_exp;
    let mut gain = add_exp;
    loop {
        if level >= 16 {
            return (16, 0);
        }
        let need = SUS_EXP_TABLE[level as usize];
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

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SusLevelUpMat {
    pub id: u64,
    pub num: u32,
}

pub async fn support_card_level_up(me: &PlayerManager, sus: &mut core::Item, mats: &Vec<SusLevelUpMat>) -> anyhow::Result<u32> {
    let sus_gdpl = get_template(sus.template);
    let Some(support_card_template_data) = tables().support_card_template.find_one("gdpl", [sus_gdpl.0, sus_gdpl.1, sus_gdpl.2, sus_gdpl.3]) else {
        bail!("support_card_template_data not found {:?}", sus_gdpl);
    };

    let mut total_upgrade_exp: u32 = 0;
    let mut total_consume_gold: u32 = 0;
    for mat in mats {
        let mat_item = me.get_item(mat.id).await?.with_context(|| format!("Item {} not found", mat.id))?;
        let gdpl = get_template(mat_item.template);

        if let Some(suplies_template_data) = tables().suplies_template.find_one("gdpl", [gdpl.0, gdpl.1, gdpl.2, gdpl.3]) {
            let supply_provide_exp = suplies_template_data.provide_exp.with_context(|| format!("Invalid EXP item {:?}", gdpl))?;
            let supply_consume_gold = suplies_template_data.consume_gold.unwrap_or(0);
            total_upgrade_exp = total_upgrade_exp
                .checked_add(supply_provide_exp.checked_mul(mat.num).context("EXP overflow")?)
                .context("EXP overflow")?;
            total_consume_gold = total_consume_gold
                .checked_add(supply_consume_gold.checked_mul(mat.num).context("Gold overflow")?)
                .context("Gold overflow")?;
        } else {
            bail!("item not found {:?}", gdpl);
        }
    }
    let enhance = sus.enhance.get_or_insert_with(|| core::Enhance { level: 1, ..Default::default() });
    let (new_level, new_exp) = calc_sus_exp(enhance.level, enhance.exp, total_upgrade_exp);
    if enhance.level < 16 && new_level >= 16 && support_card_template_data.affix_pool.len() >= 3 {
        let new_affix = random_affix_pool(support_card_template_data.affix_pool[2])?;
        enhance.affixs.resize(6, 0);
        enhance.affixs[4] = new_affix.0;
        enhance.affixs[5] = new_affix.1;
    }
    enhance.level = new_level;
    enhance.exp = new_exp;
    Ok(total_consume_gold)
}

fn random_affix_pool(id: u32) -> anyhow::Result<(u32, u32)> {
    let Some(support_affix_pool_data) = tables().support_affix_pool.find_one("id_1", id) else {
        bail!("The specified support_affix_pool_data does not exist. id={:?}", id);
    };
    let mut rng = rand::rng();
    let Some(&affix_id) = support_affix_pool_data.affix_group1.choose(&mut rng) else {
        bail!("The specified affix_group1 be empty. support_affix_pool_data={:?}", support_affix_pool_data);
    };
    Ok((affix_id, random_affix(affix_id)))
}

fn random_affix(id: u32) -> u32 {
    if id < 55 {
        let mut rng = rand::rng();
        return rng.random_range(1..=5);
    }
    1
}

pub async fn new_support(me: &mut PlayerManager, g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<()> {
    let Some(_) = tables().support_card_template.find_one("gdpl", [g, d, p, l]) else {
        bail!("The specified suplies_template_data does not exist. gdpl={:?}", [g, d, p, l]);
    };
    let support = core::Item {
        id: me.new_item_id().await?,
        template: set_template(g, d, p, l),
        count: 1,
        enhance: Some(core::Enhance {
            level: 1,
            affixs: vec![21, 5, 23, 5],
            ..Default::default()
        }),
        ..Default::default()
    };
    me.set_item(support).await?;
    riki_unlock_item(me, 3, g, d, p, l).await?;
    Ok(())
}
