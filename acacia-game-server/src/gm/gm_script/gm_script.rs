use std::sync::Arc;

use super::gm_script_add::GMScriptAdd;
use super::gm_script_gacha::GMScriptGacha;
use super::gm_script_un_lock_all_level::GMScriptUnLockAllLevel;
use super::gm_script_un_lock_level::GMScriptUnLockLevel;
use crate::base::conn::Conn;
use acacia_commons::lua_parser;

#[derive(Debug)]
pub enum GMScript {
    UnLockAllLevel(GMScriptUnLockAllLevel),
    UnLockLevel(GMScriptUnLockLevel),
    Add(GMScriptAdd),
    Gacha(GMScriptGacha),
}

impl GMScript {
    pub fn from_code(code: &str) -> Option<Self> {
        let (func_name, raw_args) = lua_parser::parse_function_call(code)?;
        let args = lua_parser::parse_function_args(raw_args);
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
            _ => {
                println!("Unknown Code: {} {}", func_name, raw_args);
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
        }
    }
}
