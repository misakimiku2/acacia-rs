use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tracing::warn;

use crate::base::conn::Conn;
use acacia_commons::{bits::set_attribute, lua_utils::lua_serde};
use acacia_game_msg::{protocol, protocol::Pf};

#[derive(Debug)]
pub enum Activity {
    DoOpenActivity(ActivityDoOpenActivity),
}

impl Activity {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::DoOpenActivity(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActivityDoOpenActivity {
    #[serde(rename = "nId")]
    pub id: u32,
    #[serde(rename = "nStartTime")]
    pub start_time: i64,
    #[serde(rename = "nEndTime")]
    pub end_time: i64,
}

impl ActivityDoOpenActivity {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let param = args.get(0)?;
        lua_serde::from_str::<Self>(param).inspect_err(|e| warn!("Activity.DoOpenActivity parse {param:?} error: {}", e)).ok()
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let rsp = protocol::NtfSyncPlayer {
            custom: HashMap::<u32, u32>::from([(set_attribute(50, self.id * 10)?, 1 as u32)]),
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

        let rsp = protocol::NtfCallScript {
            api: "ActivityQuest_Open".to_string(),
            arg: json!({
                "nId": self.id,
                "nCloseFlag": 0,
                "nStartTime": -1,
                "nEndTime": "203001014000".to_string(),
            })
            .to_string(),
            ..protocol::NtfCallScript::default()
        };
        conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
        Ok(())
    }
}
