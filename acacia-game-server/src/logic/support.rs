use acacia_commons::bits::set_template;
use acacia_data::tables::tables;
use acacia_game_msg::core;
use anyhow::bail;
use rand::RngExt;

use crate::logic::{player::PlayerManager, riki::riki_unlock_item};

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
