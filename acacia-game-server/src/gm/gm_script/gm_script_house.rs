use anyhow::Ok;
use std::{collections::HashMap, sync::Arc};

use crate::{base::conn::Conn, logic::girl_love_level::GirlLoveLevel};
use acacia_game_msg::{protocol, protocol::Pf};

#[derive(Debug)]
pub enum GMScriptHouse {
    AddGirlLoveNum(GMScriptHouseAddGirlLoveNum),
}

impl GMScriptHouse {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::AddGirlLoveNum(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug)]
pub struct GMScriptHouseAddGirlLoveNum {
    pub girl_id: u32,
    pub num: u32,
}

impl GMScriptHouseAddGirlLoveNum {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            girl_id: args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1),
            num: args.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1000),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let database = conn.database().await;
        let love_level = GirlLoveLevel::new(&database, self.girl_id);
        let attr = love_level.add(self.num).await?;

        let rsp = protocol::NtfSyncPlayer {
            custom: HashMap::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
        Ok(())
    }
}
