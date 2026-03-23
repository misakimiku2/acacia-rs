use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::None)]
async fn req_callgs(_: Arc<Conn>, _: core::Empty) -> anyhow::Result<()> {
    Ok(())
}
