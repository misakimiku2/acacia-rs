use std::collections::HashMap;

use acacia_data::tables::tables;
use anyhow::Ok;

use crate::logic::{girl::new_girl, player::PlayerManager, support::new_support, weapon::new_weapon};

#[derive(Debug, Default)]
pub struct Items {
    items: HashMap<(u16, u16, u16, u16), u32>,
}

impl Items {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn from(gdplns: &Vec<(u16, u16, u16, u16, u32)>) -> Self {
        let mut items = HashMap::<(u16, u16, u16, u16), u32>::new();
        for (g, d, p, l, n) in gdplns {
            items.insert((*g, *d, *p, *l), *n);
        }
        Self { items }
    }

    pub async fn commit_add(&self, me: &mut PlayerManager) -> anyhow::Result<()> {
        for ((g, d, p, l), num) in &self.items {
            me.add_item_count(*g, *d, *p, *l, *num).await?;
        }
        Ok(())
    }

    pub fn as_vec(&self) -> Vec<(u16, u16, u16, u16, u32)> {
        let mut result = Vec::<(u16, u16, u16, u16, u32)>::new();
        for ((g, d, p, l), num) in &self.items {
            result.push((*g, *d, *p, *l, *num));
        }
        result
    }
}

pub async fn new_item(me: &mut PlayerManager, g: u16, d: u16, p: u16, l: u16, n: u32) -> anyhow::Result<()> {
    if let Some(_) = tables().card_template.find_one("gdpl", [g, d, p, l]) {
        new_girl(me, g, d, p, l).await?;
    } else if let Some(_) = tables().weapon_template.find_one("gdpl", [g, d, p, l]) {
        new_weapon(me, g, d, p, l).await?;
    } else if let Some(_) = tables().support_card_template.find_one("gdpl", [g, d, p, l]) {
        new_support(me, g, d, p, l).await?;
    } else {
        me.add_item_count(g, d, p, l, n).await?;
    }
    Ok(())
}
