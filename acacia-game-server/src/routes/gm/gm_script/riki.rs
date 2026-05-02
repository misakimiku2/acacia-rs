use acacia_data::tables::tables;
use serde::Deserialize;
use std::sync::Arc;

use crate::{base::conn::Conn, logic::riki::get_task};
use acacia_game_msg::protocol::Pf;

#[derive(Debug)]
pub enum Riki {
    GMUnLockRiki(RikiGMUnLockRiki),
}

impl Riki {
    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::GMUnLockRiki(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RikiGMUnLockRiki {
    pub number: u32,
}

impl RikiGMUnLockRiki {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            number: args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        for riki_data in tables().riki.all() {
            let (ntask_id, nbit) = get_task(riki_data.id);
            let mut nvalue = me.get_attribute(103, ntask_id).await?;
            nvalue |= 1 << nbit;
            me.set_attribute(103, ntask_id, nvalue).await?;
        }
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
