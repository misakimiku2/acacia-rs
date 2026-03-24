use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::dispatcher::CALLGS_DISPATCHER;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqCallgs)]
async fn req_callgs(conn: Arc<Conn>, msgdata: protocol::ReqCallGs) -> anyhow::Result<()> {
    CALLGS_DISPATCHER.dispatch(&conn, msgdata).await?;
    conn.send_msg(Pf::RspCallgs, &core::Empty {}, 0).await?;
    Ok(())
}
