use anyhow::Result;
use std::collections::HashMap;

use crate::db::db;
use crate::models::PlayerStrAttr;

pub struct PlayerStrAttrManager {
    pid: u64,
}

impl PlayerStrAttrManager {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn get_all(&self) -> Result<HashMap<u32, String>> {
        let mut resp = db().query("SELECT * FROM player_str_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        let str_attrs: Vec<PlayerStrAttr> = resp.take(0)?;
        let result = str_attrs.into_iter().map(|str_attr| (str_attr.key, str_attr.value)).collect();
        Ok(result)
    }

    pub async fn get(&self, key: u32) -> Result<String> {
        let mut resp = db()
            .query("SELECT * FROM ONLY type::record('player_str_attr', [$pid, $key])")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .await?;
        let str_attr: Option<PlayerStrAttr> = resp.take(0)?;
        let result = str_attr.map(|a| a.value).unwrap_or(String::new());
        Ok(result)
    }

    pub async fn set(&self, key: u32, value: String) -> Result<()> {
        db().query("UPSERT type::record('player_str_attr', [$pid, $key]) MERGE { pid: $pid, key: $key, value: $value }")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .bind(("value", value))
            .await?;
        Ok(())
    }

    pub async fn del(&self, key: u32) -> Result<()> {
        db().query("DELETE ONLY type::record('player_str_attr', [$pid, $key])")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .await?;
        Ok(())
    }

    pub async fn del_all(&self) -> Result<()> {
        db().query("DELETE FROM player_str_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        Ok(())
    }
}
