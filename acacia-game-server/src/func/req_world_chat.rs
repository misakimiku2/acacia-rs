use anyhow::Context;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::ReqWorldChat)]
async fn req_world_chat(conn: Arc<Conn>, msgdata: core::ChatMsg) -> anyhow::Result<()> {
    let mut rsp = msgdata.clone();
    rsp.r#type = core::ChatType::World as i32;
    rsp.sender = conn.uid().with_context(|| format!("The pid should not be None"))?;
    conn.send_msg(Pf::RspWorldChat, &rsp, 0).await?;
    Ok(())
}
