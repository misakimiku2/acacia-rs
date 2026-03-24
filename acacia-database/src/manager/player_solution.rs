use acacia_game_msg::core;
use anyhow::Result;

use crate::db::db;
use crate::models::PlayerLineup;

pub struct PlayerSolutionManager {
    pid: u64,
}

impl PlayerSolutionManager {
    pub fn new(pid: u64) -> Self {
        Self { pid }
    }

    pub async fn get_all(&self) -> Result<Vec<core::Lineup>> {
        let mut resp = db().query("SELECT * FROM player_solution WHERE pid = $pid").bind(("pid", self.pid)).await?;
        let solutions: Vec<PlayerLineup> = resp.take(0)?;
        let result = solutions.into_iter().map(PlayerLineup::to_prost).collect();
        Ok(result)
    }

    pub async fn get(&self, index: u32) -> Result<core::Lineup> {
        let mut resp = db()
            .query("SELECT * FROM ONLY type::record('player_solution', [$pid, $index])")
            .bind(("pid", self.pid))
            .bind(("index", index))
            .await?;
        let solution: Option<PlayerLineup> = resp.take(0)?;
        let result = PlayerLineup::to_prost(solution.unwrap());
        Ok(result)
    }

    pub async fn update(&self, src_solution: core::Lineup) -> Result<Option<core::Lineup>> {
        let mut resp = db()
            .query("UPSERT type::record('player_solution', [$pid, $index]) MERGE object::extend($solution, { pid: $pid })")
            .bind(("pid", self.pid))
            .bind(("index", src_solution.index))
            .bind(("solution", PlayerLineup::from_prost(src_solution)))
            .await?;
        let solution: Option<PlayerLineup> = resp.take(0)?;
        let result = solution.map(PlayerLineup::to_prost);
        Ok(result)
    }

    pub async fn del(&self, index: u32) -> Result<()> {
        db().query("DELETE ONLY type::record('player_solution', [$pid, $index])")
            .bind(("pid", self.pid))
            .bind(("index", index))
            .await?;
        Ok(())
    }

    pub async fn del_all(&self) -> Result<()> {
        db().query("DELETE FROM player_solution WHERE pid = $pid").bind(("pid", self.pid)).await?;
        Ok(())
    }
}
