use anyhow::Ok;
use std::sync::Arc;

use crate::base::conn::Conn;

#[derive(Debug)]
pub enum GMScriptGacha {
    GmTest,
    GmSetTime,
    GmSetTenTime,
    PrintWeight,
    UnLockAllPool,
    ForcePickUp,
}

impl GMScriptGacha {
    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}
