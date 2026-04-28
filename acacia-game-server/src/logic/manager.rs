use acacia_data::tables::tables;
use anyhow::{Context, Ok};
use std::collections::HashMap;

use acacia_commons::bits::{self, set_template};
use acacia_database::repositories::player::PlayerRepository;
use acacia_game_msg::{core, protocol};

pub struct PlayerManager {
    database: PlayerRepository,
    attrs: HashMap<u32, u32>,
    items: HashMap<u64, core::Item>,
    str_attrs: HashMap<u32, String>,
    show_items: Vec<u64>,
}

impl PlayerManager {
    pub fn new(database: PlayerRepository) -> Self {
        Self {
            database: database,
            attrs: HashMap::new(),
            items: HashMap::new(),
            str_attrs: HashMap::new(),
            show_items: Vec::new(),
        }
    }

    pub async fn get_item(&self, id: u64) -> anyhow::Result<Option<core::Item>> {
        Ok(self.database.items.find_one(id).await?)
    }

    pub async fn set_item(&mut self, item: core::Item) -> anyhow::Result<()> {
        self.database.items.update_one(item.clone()).await?;
        self.items.insert(item.id, item);
        Ok(())
    }

    pub async fn get_item_count(&self, g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<u32> {
        if let Some(cash_data) = tables().cash.find_one("gdpl_1", Some((g, d, p, l))) {
            return self.get_attribute(1, 1 + cash_data.id * 2).await;
        }
        let template_id = set_template(g, d, p, l)?;
        let items = self.database.items.find_many_by_template(template_id).await?;
        let count = items.iter().map(|item| item.count).sum();
        Ok(count)
    }

    pub async fn set_item_count(&mut self, g: u16, d: u16, p: u16, l: u16, n: u32) -> anyhow::Result<()> {
        if let Some(cash_data) = tables().cash.find_one("gdpl_1", Some((g, d, p, l))) {
            self.set_attribute(1, 1 + cash_data.id * 2, n).await?;
            return Ok(());
        }
        let current_count = self.get_item_count(g, d, p, l).await?;
        if n == current_count {
            return Ok(());
        }
        if n > current_count {
            let add_count = n - current_count;
            self.add_item_count(g, d, p, l, add_count).await?;
        } else {
            let remove_count = current_count - n;
            self.remove_item_count(g, d, p, l, remove_count).await?;
        }
        Ok(())
    }

    pub async fn add_item_count(&mut self, g: u16, d: u16, p: u16, l: u16, n: u32) -> anyhow::Result<()> {
        if n == 0 {
            return Ok(());
        }
        if let Some(cash_data) = tables().cash.find_one("gdpl_1", Some((g, d, p, l))) {
            let current_count = self.get_attribute(1, 1 + cash_data.id * 2).await?;
            self.set_attribute(1, 1 + cash_data.id * 2, current_count + n).await?;
            return Ok(());
        }
        let template_id = set_template(g, d, p, l)?;
        let item = if let Some(mut item) = self.database.items.find_one_by_template(template_id).await? {
            item.count += n;
            item
        } else {
            core::Item {
                id: self.database.items.new_id().await?,
                template: template_id,
                count: n,
                ..Default::default()
            }
        };
        self.set_item(item).await?;
        Ok(())
    }

    pub async fn remove_item_count(&mut self, g: u16, d: u16, p: u16, l: u16, n: u32) -> anyhow::Result<()> {
        if n == 0 {
            return Ok(());
        }
        if let Some(cash_data) = tables().cash.find_one("gdpl_1", Some((g, d, p, l))) {
            let current_count = self.get_attribute(1, 1 + cash_data.id * 2).await?;
            if current_count < n {
                anyhow::bail!("Insufficient item quantity {}/{}", n, current_count);
            }
            self.set_attribute(1, 1 + cash_data.id * 2, current_count - n).await?;
            return Ok(());
        }
        let template_id = set_template(g, d, p, l)?;
        let mut items = self.database.items.find_many_by_template(template_id).await?;
        let current_count: u32 = items.iter().map(|item| item.count).sum();
        if current_count < n {
            anyhow::bail!("Insufficient item quantity {}/{}", n, current_count);
        }
        let mut left_remove_count = n;
        let mut changed_items: Vec<core::Item> = Vec::new();
        for item in items.iter_mut() {
            if left_remove_count == 0 {
                break;
            }
            if item.count <= left_remove_count {
                left_remove_count -= item.count;
                item.count = 0;
            } else {
                item.count -= left_remove_count;
                left_remove_count = 0;
            }
            changed_items.push(item.clone());
        }
        for item in changed_items {
            self.set_item(item).await?;
        }
        Ok(())
    }

    pub async fn get_items_by_gdpl(&self, g: u16, d: u16, p: u16, l: u16) -> anyhow::Result<Vec<core::Item>> {
        let template_id = set_template(g, d, p, l)?;
        Ok(self.database.items.find_many_by_template(template_id).await?)
    }

    pub async fn get_attribute(&self, gid: u32, id: u32) -> anyhow::Result<u32> {
        self.database.attrs.find_one(bits::set_attribute(gid, id)?).await
    }

    pub async fn set_attribute(&mut self, gid: u32, id: u32, value: u32) -> anyhow::Result<()> {
        let key = bits::set_attribute(gid, id)?;
        self.database.attrs.update_one((key, value)).await?;
        self.attrs.insert(key, value);
        Ok(())
    }

    pub async fn get_str_attribute(&self, gid: u32, id: u32) -> anyhow::Result<String> {
        self.database.str_attrs.find_one(bits::set_attribute(gid, id)?).await
    }

    pub async fn set_str_attribute(&mut self, gid: u32, id: u32, value: String) -> anyhow::Result<()> {
        let key = bits::set_attribute(gid, id)?;
        self.database.str_attrs.update_one((key, value.clone())).await?;
        self.str_attrs.insert(key, value);
        Ok(())
    }

    pub async fn get_show_item(&self, idx: usize) -> anyhow::Result<u64> {
        let player = self.database.info.find_one().await?.with_context(|| format!("The pid should not be None"))?;
        Ok(player.show_items.get(idx).copied().unwrap_or(0))
    }

    pub async fn set_show_item(&mut self, idx: usize, value: u64) -> anyhow::Result<()> {
        let mut player = self.database.info.find_one().await?.with_context(|| format!("The pid should not be None"))?;
        if idx >= player.show_items.len() {
            player.show_items.resize(idx + 1, 0);
        }
        player.show_items[idx] = value;
        self.show_items = player.show_items.clone();
        self.database.info.update_one(player).await?;
        Ok(())
    }

    pub fn as_sync_player(self) -> protocol::NtfSyncPlayer {
        protocol::NtfSyncPlayer {
            custom: self.attrs,
            custom_str: self.str_attrs,
            items: self.items.into_values().collect(),
            show_items: self.show_items,
            ..Default::default()
        }
    }
}
