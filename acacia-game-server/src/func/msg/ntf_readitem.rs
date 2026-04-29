use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::NtfReaditem)]
async fn ntf_readitem(conn: Arc<Conn>, msgdata: core::IdArray) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    for id in msgdata.ids {
        if let Some(mut item) = me.get_item(id).await? {
            item.flag |= 4;
            me.set_item(item).await?;
        } else {
            warn!("Item {} not found.", id)
        }
    }
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
    Ok(())
}
