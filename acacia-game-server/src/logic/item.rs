use std::collections::HashMap;

use crate::logic::player::PlayerManager;

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

pub fn items_check() {}
