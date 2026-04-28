use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::ReqChangeWorldChannel)]
async fn req_change_world_channel(conn: Arc<Conn>, msgdata: core::SimpleUint) -> anyhow::Result<()> {
    conn.send_msg(Pf::RspChangeWorldChannel, &msgdata, 0).await?;
    Ok(())
}
