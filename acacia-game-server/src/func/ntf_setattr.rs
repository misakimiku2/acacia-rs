use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::dispatcher::HandlerResult;
use acacia_derive::func_reg;
use acacia_game_msg::protocol::{self, Pf};

#[func_reg(Pf::NtfSetattr)]
async fn ntf_setattr(_conn: Arc<Conn>, _msgdata: protocol::NtfSetAttr) -> anyhow::Result<HandlerResult> {
    Ok(HandlerResult::Ok)
}
