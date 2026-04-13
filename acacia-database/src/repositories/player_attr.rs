use anyhow::Result;
use std::collections::HashMap;

use crate::db::db;
use crate::models::PlayerAttr;
use acacia_commons::bits::set_attribute;

pub struct PlayerAttrRepository {
    pid: u64,
}

impl PlayerAttrRepository {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn find_all(&self) -> Result<HashMap<u32, u32>> {
        let mut resp = db().query("SELECT * FROM player_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        let attrs: Vec<PlayerAttr> = resp.take(0)?;
        let result = attrs.into_iter().map(|attr| (attr.key, attr.value)).collect();
        Ok(result)
    }

    pub async fn find_one(&self, key: u32) -> Result<u32> {
        let mut resp = db()
            .query("SELECT * FROM ONLY type::record('player_attr', [$pid, $key])")
            .bind(("pid", self.pid))
            .bind(("key", key))
            .await?;
        let attr: Option<PlayerAttr> = resp.take(0)?;
        let result = attr.map(|a| a.value).unwrap_or(0);
        Ok(result)
    }

    pub async fn update_one(&self, attr: (u32, u32)) -> Result<()> {
        db().query("UPSERT type::record('player_attr', [$pid, $key]) MERGE { pid: $pid, key: $key, value: $value }")
            .bind(("pid", self.pid))
            .bind(("key", attr.0))
            .bind(("value", attr.1))
            .await?;
        Ok(())
    }

    pub async fn update_many(&self, attrs: impl IntoIterator<Item = (u32, u32)>) -> Result<()> {
        let attrs: Vec<_> = attrs.into_iter().map(|(key, value)| PlayerAttr { key, value }).collect();
        db().query(
            "FOR $attr IN $attrs {
                UPSERT type::record('player_attr', [$pid, $attr.key]) MERGE object::extend({ pid: $pid }, $attr);
            }",
        )
        .bind(("pid", self.pid))
        .bind(("attrs", attrs))
        .await?;
        Ok(())
    }

    pub async fn delete_one(&self, key: u32) -> Result<()> {
        db().query("DELETE ONLY type::record('player_attr', [$pid, $key])").bind(("pid", self.pid)).bind(("key", key)).await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<()> {
        db().query("DELETE FROM player_attr WHERE pid = $pid").bind(("pid", self.pid)).await?;
        Ok(())
    }

    pub async fn get_attr(&self, gid: u32, sid: u32) -> anyhow::Result<(u32, u32)> {
        let key = set_attribute(gid, sid)?;
        let value = self.find_one(key).await?;
        Ok((key, value))
    }

    pub async fn set_attr(&self, gid: u32, sid: u32, value: u32) -> anyhow::Result<(u32, u32)> {
        let key = set_attribute(gid, sid)?;
        self.update_one((key, value)).await?;
        Ok((key, value))
    }
}
