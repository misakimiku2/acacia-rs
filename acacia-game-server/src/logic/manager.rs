use std::collections::HashMap;

use acacia_commons::bits;
use acacia_database::repositories::player::PlayerRepository;
use acacia_game_msg::{core, protocol};

pub struct PlayerManager {
    database: PlayerRepository,
    attrs: HashMap<u32, u32>,
    items: Vec<core::Item>,
    str_attrs: HashMap<u32, String>,
}

impl PlayerManager {
    pub fn new(database: PlayerRepository) -> Self {
        Self {
            database: database,
            attrs: HashMap::new(),
            items: Vec::new(),
            str_attrs: HashMap::new(),
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

    pub fn as_sync_player(self) -> protocol::NtfSyncPlayer {
        protocol::NtfSyncPlayer {
            custom: self.attrs,
            custom_str: self.str_attrs,
            items: self.items,
            ..Default::default()
        }
    }
}
