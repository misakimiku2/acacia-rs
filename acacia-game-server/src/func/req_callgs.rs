use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::dispatcher::HandlerResult;
use acacia_derive::func_reg;
use acacia_game_msg::protocol::{self, Pf};

#[func_reg(Pf::ReqCallgs)]
async fn req_callgs(_conn: Arc<Conn>, _msgdata: protocol::ReqCallGs) -> anyhow::Result<HandlerResult> {
    Ok(HandlerResult::Ok)
}
