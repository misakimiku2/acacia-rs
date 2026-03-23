use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::protocol::{self, Pf};

#[func_reg(Pf::NtfSetattr)]
async fn ntf_setattr(_conn: Arc<Conn>, _msgdata: protocol::NtfSetAttr) -> anyhow::Result<()> {
    Ok(())
}
