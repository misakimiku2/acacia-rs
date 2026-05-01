use anyhow::Result;
use std::collections::HashMap;

use crate::db::db;
use crate::models::PlayerStrAttr;
use acacia_commons::bits::set_attribute;

pub struct PlayerStrAttrRepository {
    pid: u64,
}

impl PlayerStrAttrRepository {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn find_all(&self) -> Result<HashMap<u32, String>> {
        let mut resp = db().query("SELECT * FROM player_str_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        let str_attrs: Vec<PlayerStrAttr> = resp.take(0)?;
        let result = str_attrs.into_iter().map(|str_attr| (str_attr.key, str_attr.value)).collect();
        Ok(result)
    }

    pub async fn find_one(&self, key: u32) -> Result<String> {
        let mut resp = db()
            .query("SELECT * FROM ONLY type::record('player_str_attr', [$pid, $key])")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .await?;
        let str_attr: Option<PlayerStrAttr> = resp.take(0)?;
        let result = str_attr.map(|a| a.value).unwrap_or(String::new());
        Ok(result)
    }

    pub async fn update_one(&self, str_attr: (u32, String)) -> Result<()> {
        db().query("UPSERT type::record('player_str_attr', [$pid, $key]) MERGE { pid: $pid, key: $key, value: $value }")
            .bind(("pid", self.pid))
            .bind(("key", str_attr.0))
            .bind(("value", str_attr.1))
            .await?;
        Ok(())
    }

    pub async fn delete_one(&self, key: u32) -> Result<()> {
        db().query("DELETE ONLY type::record('player_str_attr', [$pid, $key])")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<()> {
        db().query("DELETE FROM player_str_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        Ok(())
    }

    pub async fn get_attr(&self, gid: u32, sid: u32) -> anyhow::Result<(u32, String)> {
        let key = set_attribute(gid, sid);
        let value = self.find_one(key).await?;
        Ok((key, value))
    }

    pub async fn set_attr(&self, gid: u32, sid: u32, value: String) -> anyhow::Result<(u32, String)> {
        let key = set_attribute(gid, sid);
        self.update_one((key, value.clone())).await?;
        Ok((key, value))
    }
}
