use acacia_commons::{bits::set_attribute, lua_utils::lua_serde};
use anyhow::Context;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::{base::conn::Conn, logic::girl_love_level::GirlLoveLevel};
use acacia_game_msg::{protocol, protocol::Pf};

#[derive(Debug)]
pub enum GMScriptActivity {
    DoOpenActivity(GMScriptActivityDoOpenActivity),
}

impl GMScriptActivity {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::DoOpenActivity(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GMScriptActivityDoOpenActivity {
    #[serde(rename = "nId")]
    pub id: u32,
    #[serde(rename = "nStartTime")]
    pub start_time: i64,
    #[serde(rename = "nEndTime")]
    pub end_time: i64,
}

impl GMScriptActivityDoOpenActivity {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let param = args.get(0)?;
        lua_serde::from_str::<Self>(param)
            .inspect_err(|e| eprintln!("Activity.DoOpenActivity parse {param:?} error: {}", e))
            .ok()
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset");
        // let dt_start = DateTime::from_timestamp(self.start_time, 0).map(|dt| dt.with_timezone(&offset)).expect("Invalid timestamp");
        // let format_str = "%Y%m%d%H%M";
        // dt_start.format(format_str).to_string(),

        let rsp = protocol::NtfCallScript {
            api: "ActivityQuest_Open".to_string(),
            arg: json!({
                "nId": self.id,
                "nCloseFlag": 0,
                "nStartTime": "202001011005".to_string(),
                "nEndTime": "209901011005".to_string(),
            })
            .to_string(),
            ..protocol::NtfCallScript::default()
        };
        conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
        Ok(())
    }
}
