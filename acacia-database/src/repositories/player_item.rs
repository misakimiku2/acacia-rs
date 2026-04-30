use anyhow::Result;

use crate::db::db;
use crate::models::PlayerItem;
use acacia_game_msg::core;

pub struct PlayerItemRepository {
    pid: u64,
}

impl PlayerItemRepository {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    // pub async fn new_id(&self) -> Result<u64> {
    //    let mut resp: surrealdb::IndexedResults = db()
    //         .query("UPSERT type::record('counter', $pid) SET player_item_counter += 1 RETURN player_item_counter")
    //         .bind(("pid", self.pid))
    //         .await?;
    //     let id: Option<u64> = resp.take("player_item_counter")?;
    //     Ok(id.unwrap_or(1))
    // }

    pub async fn find_all(&self) -> Result<Vec<core::Item>> {
        let mut resp = db().query("SELECT * FROM player_item WHERE pid = $pid").bind(("pid", self.pid)).await?;
        let items: Vec<PlayerItem> = resp.take(0)?;
        let result = items.into_iter().map(PlayerItem::to_prost).collect();
        Ok(result)
    }

    pub async fn find_one(&self, sid: u64) -> Result<Option<core::Item>> {
        let mut resp = db()
            .query("SELECT * FROM ONLY type::record('player_item', [$pid, $sid])")
            .bind(("pid", self.pid))
            .bind(("sid", sid))
            .await?;
        let item: Option<PlayerItem> = resp.take(0)?;
        let result = item.map(PlayerItem::to_prost);
        Ok(result)
    }

    pub async fn update_one(&self, src_item: core::Item) -> Result<()> {
        let src_item: core::Item = src_item.into();
        db().query("UPSERT type::record('player_item', [$pid, $item.sid]) MERGE object::extend({ pid: $pid }, $item)")
            .bind(("pid", self.pid))
            .bind(("item", PlayerItem::from_prost(src_item)))
            .await?;
        Ok(())
    }

    pub async fn update_many(&self, src_items: impl IntoIterator<Item: Into<core::Item>>) -> Result<()> {
        let mut player_items = Vec::new();
        for src_item in src_items {
            let src_item: core::Item = src_item.into();
            player_items.push(PlayerItem::from_prost(src_item));
        }
        db().query(
            "FOR $item IN $items {
                UPSERT type::record('player_item', [$pid, $item.sid]) MERGE object::extend({ pid: $pid }, $item);
            }",
        )
        .bind(("pid", self.pid))
        .bind(("items", player_items))
        .await?;
        Ok(())
    }

    pub async fn delete_one(&self, sid: u64) -> Result<()> {
        db().query("DELETE ONLY type::record('player_item', [$pid, $sid])").bind(("pid", self.pid)).bind(("sid", sid)).await?;
        db().query("UPSERT ONLY type::record('counter', $pid) SET player_item_counter = 0").bind(("pid", self.pid)).await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<()> {
        db().query("DELETE FROM player_item WHERE pid = $pid").bind(("pid", self.pid)).await?;
        Ok(())
    }

    pub async fn find_one_by_template(&self, template: u64) -> Result<Option<core::Item>> {
        let mut resp = db()
            .query("SELECT * FROM player_item WHERE pid = $pid AND template = $template LIMIT 1")
            .bind(("pid", self.pid))
            .bind(("template", template))
            .await?;
        let item = resp.take::<Vec<PlayerItem>>(0)?.into_iter().next().map(|i| i.to_prost());
        Ok(item)
    }

    pub async fn find_many_by_template(&self, template: u64) -> Result<Vec<core::Item>> {
        let mut resp = db()
            .query("SELECT * FROM player_item WHERE pid = $pid AND template = $template")
            .bind(("pid", self.pid))
            .bind(("template", template))
            .await?;
        let items = resp.take::<Vec<PlayerItem>>(0)?.into_iter().map(|i| i.to_prost()).collect();
        Ok(items)
    }

    pub async fn find_one_by_slot_value(&self, slot_id: u32, slot_value: u64) -> Result<Option<core::Item>> {
        let slot_key = slot_id.to_string();
        let mut resp = db()
            .query("SELECT * FROM player_item WHERE pid = $pid AND slots[$slot_key] = $slot_value LIMIT 1")
            .bind(("pid", self.pid))
            .bind(("slot_key", slot_key))
            .bind(("slot_value", slot_value))
            .await?;
        let item = resp.take::<Vec<PlayerItem>>(0)?.into_iter().next().map(PlayerItem::to_prost);
        Ok(item)
    }

    pub async fn find_many_by_slot_value(&self, slot_id: u32, slot_value: u64) -> Result<Vec<core::Item>> {
        let slot_key = slot_id.to_string();
        let mut resp = db()
            .query("SELECT * FROM player_item WHERE pid = $pid AND slots[$slot_key] = $slot_value")
            .bind(("pid", self.pid))
            .bind(("slot_key", slot_key))
            .bind(("slot_value", slot_value))
            .await?;
        let items = resp.take::<Vec<PlayerItem>>(0)?.into_iter().map(|i| i.to_prost()).collect();
        Ok(items)
    }
}
