use std::sync::Arc;
use tracing::warn;

use super::gm_script_activity::{GMScriptActivity, GMScriptActivityDoOpenActivity};
use super::gm_script_add::GMScriptAdd;
use super::gm_script_gacha::GMScriptGacha;
use super::gm_script_house::{GMScriptHouse, GMScriptHouseAddAllGirlLoveNum, GMScriptHouseAddGirlLoveNum};
use super::gm_script_riki::{GMScriptRiki, GMScriptRikiGMUnLockRiki};
use super::gm_script_un_lock_all_level::GMScriptUnLockAllLevel;
use super::gm_script_un_lock_level::GMScriptUnLockLevel;
use crate::base::conn::Conn;
use acacia_commons::lua_utils::lua_func;

#[derive(Debug)]
pub enum GMScript {
    UnLockAllLevel(GMScriptUnLockAllLevel),
    UnLockLevel(GMScriptUnLockLevel),
    Add(GMScriptAdd),
    Gacha(GMScriptGacha),
    House(GMScriptHouse),
    Activity(GMScriptActivity),
    Riki(GMScriptRiki),
}

impl GMScript {
    pub fn from_code(code: &str) -> Option<Self> {
        let (func_name, raw_args) = lua_func::parse_function_call(code)?;
        let args = lua_func::parse_function_args(raw_args);
        let gm_script = match func_name {
            "GM.UnLockAllLevel" => Self::UnLockAllLevel(GMScriptUnLockAllLevel::from_args(&args)),
            "GM.UnLockLevel" => Self::UnLockLevel(GMScriptUnLockLevel::from_args(&args)),
            "GM.Add" => {
                if let Some(gm_script_add) = GMScriptAdd::from_args(&args) {
                    Self::Add(gm_script_add)
                } else {
                    return None;
                }
            }
            "Gacha.GmTest" => Self::Gacha(GMScriptGacha::GmTest),
            "Gacha.GmSetTime" => Self::Gacha(GMScriptGacha::GmSetTime),
            "Gacha.GmSetTenTime" => Self::Gacha(GMScriptGacha::GmSetTenTime),
            "Gacha.PrintWeight" => Self::Gacha(GMScriptGacha::PrintWeight),
            "GM.Gacha.UnLockAllPool" => Self::Gacha(GMScriptGacha::UnLockAllPool),
            "GM.Gacha.ForcePickUp" => Self::Gacha(GMScriptGacha::ForcePickUp),
            "GM.House.AddGirlLoveNum" => Self::House(GMScriptHouse::AddGirlLoveNum(GMScriptHouseAddGirlLoveNum::from_args(&args))),
            "GM.House.AddAllGirlLoveNum" => Self::House(GMScriptHouse::AddAllGirlLoveNum(GMScriptHouseAddAllGirlLoveNum::from_args(&args))),
            "Activity.DoOpenActivity" => {
                if let Some(gm_script_activity_do_open_activity) = GMScriptActivityDoOpenActivity::from_args(&args) {
                    Self::Activity(GMScriptActivity::DoOpenActivity(gm_script_activity_do_open_activity))
                } else {
                    warn!("Unknown Activity.DoOpenActivity args: {}", raw_args);
                    return None;
                }
            }
            "GM.Riki.GMUnLockRiki" => Self::Riki(GMScriptRiki::GMUnLockRiki(GMScriptRikiGMUnLockRiki::from_args(&args))),
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
            Self::Gacha(s) => s.run(conn).await,
            Self::House(s) => s.run(conn).await,
            Self::Activity(s) => s.run(conn).await,
            Self::Riki(s) => s.run(conn).await,
        }
    }
}
