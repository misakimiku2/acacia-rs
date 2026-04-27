use anyhow::Context;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::ReqRename)]
async fn req_callgs(conn: Arc<Conn>, msgdata: core::SimpleString) -> anyhow::Result<()> {
    let database = conn.database().await;
    let mut player = database.info.find_one().await?.with_context(|| format!("Player {:?} not found.", conn.uid()))?;
    player.name = msgdata.data.clone();
    database.info.update_one(player).await?;
    conn.send_msg(Pf::RspRename, &msgdata, 0).await?;
    Ok(())
}
