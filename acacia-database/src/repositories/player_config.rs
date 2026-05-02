use anyhow::Result;

use crate::db::db;
use crate::models::PlayerConfig;

pub struct PlayerConfigRepository {
    pid: u64,
}

impl PlayerConfigRepository {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn find_one(&self) -> Result<Option<PlayerConfig>> {
        let mut resp = db().query("SELECT * FROM ONLY type::record('player_config', $pid)").bind(("pid", self.pid)).await?;
        let config: Option<PlayerConfig> = resp.take(0)?;
        Ok(config)
    }

    pub async fn update_one(&self, src_config: PlayerConfig) -> Result<()> {
        db().query("UPSERT type::record('player_config', $pid) MERGE $player")
            .bind(("pid", self.pid))
            .bind(("player", src_config))
            .await?;
        Ok(())
    }

    pub async fn delete_one(&self) -> Result<()> {
        db().query("DELETE ONLY type::record('player_config', $pid)").bind(("pid", self.pid)).await?;
        Ok(())
    }
}
