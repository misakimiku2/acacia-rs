use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::NtfLogout)]
async fn ntf_logout(_: Arc<Conn>, _: core::Empty) -> anyhow::Result<()> {
    Ok(())
}
