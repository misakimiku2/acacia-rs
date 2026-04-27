use acacia_game_msg::core;
use anyhow::Result;

use crate::db::db;
use crate::models::PlayerInfo;

pub struct PlayerInfoRepository {
    pid: u64,
}

impl PlayerInfoRepository {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn find_one(&self) -> Result<Option<core::Player>> {
        let mut resp = db().query("SELECT * FROM ONLY type::record('player_info', $pid)").bind(("pid", self.pid)).await?;
        let player: Option<PlayerInfo> = resp.take(0)?;
        let result = player.map(PlayerInfo::to_prost);
        Ok(result)
    }

    pub async fn update_one(&self, src_player: core::Player) -> Result<Option<core::Player>> {
        let mut resp = db()
            .query("UPSERT type::record('player_info', $pid) MERGE $player")
            .bind(("pid", self.pid))
            .bind(("player", PlayerInfo::from_prost(src_player)))
            .await?;
        let player: Option<PlayerInfo> = resp.take(0)?;
        let result = player.map(PlayerInfo::to_prost);
        Ok(result)
    }

    pub async fn delete_one(&self) -> Result<()> {
        db().query("DELETE ONLY type::record('player_info', $pid)").bind(("pid", self.pid)).await?;
        Ok(())
    }
}
