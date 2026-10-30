use axum::{Json, extract::ConnectInfo, http::StatusCode};
use std::{net::SocketAddr, sync::Arc};
use tracing::{debug, warn};

use crate::base::conn::{CONNS, Conn};
use acacia_commons::lua_utils::lua_func;

mod activity;
mod add;
mod add_all_item;
mod gacha;
mod house;
mod riki;
mod un_lock_all_level;
mod un_lock_level;
mod utility;

use activity::*;
use add::*;
use add_all_item::*;
use gacha::*;
use house::*;
use riki::*;
use un_lock_all_level::*;
use un_lock_level::*;
use utility::*;

#[derive(Debug)]
pub enum GMScript {
    UnLockAllLevel(UnLockAllLevel),
    UnLockLevel(UnLockLevel),
    Add(Add),
    AddAllItem(AddAllItem),
    Gacha(Gacha),
    House(House),
    Activity(Activity),
    Riki(Riki),
    SetAttr(SetAttr),
    PassGuide(PassGuide),
    MaxAllGirl(MaxAllGirl),
}

impl GMScript {
    pub fn from_code(code: &str) -> Option<Self> {
        let (func_name, raw_args) = lua_func::parse_function_call(code)?;
        let args = lua_func::parse_function_args(raw_args);
        let gm_script = match func_name {
            "GM.UnLockAllLevel" => Self::UnLockAllLevel(UnLockAllLevel::from_args(&args)),
            "GM.UnLockLevel" => Self::UnLockLevel(UnLockLevel::from_args(&args)),
            "GM.Add" => {
                if let Some(gm_script_add) = Add::from_args(&args) {
                    Self::Add(gm_script_add)
                } else {
                    return None;
                }
            }
            "GM.AddAllItem" => Self::AddAllItem(AddAllItem {}),
            "Gacha.GmTest" => Self::Gacha(Gacha::GmTest),
            "Gacha.GmSetTime" => Self::Gacha(Gacha::GmSetTime),
            "Gacha.GmSetTenTime" => Self::Gacha(Gacha::GmSetTenTime),
            "Gacha.PrintWeight" => Self::Gacha(Gacha::PrintWeight),
            "GM.Gacha.UnLockAllPool" => Self::Gacha(Gacha::UnLockAllPool),
            "GM.Gacha.ForcePickUp" => Self::Gacha(Gacha::ForcePickUp),
            "GM.House.AddGirlLoveNum" => Self::House(House::AddGirlLoveNum(HouseAddGirlLoveNum::from_args(&args))),
            "GM.House.AddAllGirlLoveNum" => Self::House(House::AddAllGirlLoveNum(HouseAddAllGirlLoveNum::from_args(&args))),
            "Activity.DoOpenActivity" => {
                if let Some(gm_script_activity_do_open_activity) = ActivityDoOpenActivity::from_args(&args) {
                    Self::Activity(Activity::DoOpenActivity(gm_script_activity_do_open_activity))
                } else {
                    warn!("Unknown Activity.DoOpenActivity args: {}", raw_args);
                    return None;
                }
            }
            "GM.Riki.GMUnLockRiki" => Self::Riki(Riki::GMUnLockRiki(RikiGMUnLockRiki::from_args(&args))),
            "GM.SetAttr" => Self::SetAttr(SetAttr::from_args(&args)),
            "GM.PassGuide" => Self::PassGuide(PassGuide {}),
            "GM.MaxAllGirl" => Self::MaxAllGirl(MaxAllGirl::from_args(&args)),
            _ => {
                warn!("Unknown Code: {} {}", func_name, raw_args);
                return None;
            }
        };
        Some(gm_script)
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::UnLockAllLevel(s) => s.run(conn).await,
            Self::UnLockLevel(s) => s.run(conn).await,
            Self::Add(s) => s.run(conn).await,
            Self::AddAllItem(s) => s.run(conn).await,
            Self::Gacha(s) => s.run(conn).await,
            Self::House(s) => s.run(conn).await,
            Self::Activity(s) => s.run(conn).await,
            Self::Riki(s) => s.run(conn).await,
            Self::SetAttr(s) => s.run(conn).await,
            Self::PassGuide(s) => s.run(conn).await,
            Self::MaxAllGirl(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GmScriptPayload {
    pid: u64,
    // target: i32,
    code: String,
}

pub async fn handle_gm_script(ConnectInfo(addr): ConnectInfo<SocketAddr>, Json(payload): Json<GmScriptPayload>) -> (StatusCode, &'static str) {
    let _client_ip = addr.ip();
    if let Some(conn) = CONNS.get_by_uid(payload.pid) {
        if let Some(gm_script) = GMScript::from_code(&payload.code) {
            debug!("GMScript: {:?} {:?}", conn.uid(), gm_script);
            gm_script.run(conn).await.expect("Failed to run GMScript");
        }
    }
    (StatusCode::OK, "success")
}
