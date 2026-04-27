use anyhow::Context;
use std::collections::HashMap;

use acacia_commons::bits;
use acacia_database::repositories::player::PlayerRepository;
use acacia_game_msg::{core, protocol};

pub struct PlayerManager {
    database: PlayerRepository,
    attrs: HashMap<u32, u32>,
    items: Vec<core::Item>,
    str_attrs: HashMap<u32, String>,
    show_items: Vec<u64>,
}

impl PlayerManager {
    pub fn new(database: PlayerRepository) -> Self {
        Self {
            database: database,
            attrs: HashMap::new(),
            items: Vec::new(),
            str_attrs: HashMap::new(),
            show_items: Vec::new(),
        }
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
            items: self.items,
            show_items: self.show_items,
            ..Default::default()
        }
    }
}
