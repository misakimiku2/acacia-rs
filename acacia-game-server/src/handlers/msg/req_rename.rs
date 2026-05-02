use acacia_data::tables::tables;
use anyhow::Context;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol::Pf};

#[func_reg(Pf::ReqRename)]
async fn req_rename(conn: Arc<Conn>, msgdata: core::SimpleString) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    let times = me.get_attribute(99, 6).await? + 1;
    let rename_data = tables().rename.find_one("times_1", times.min(30)).with_context(|| format!("Rename not found times={}", times))?;
    let mut money = me.get_money(rename_data.r#type).await?;
    if money < rename_data.num {
        conn.send_err("error.404").await?;
        return Ok(());
    }
    money -= rename_data.num;
    me.set_money(rename_data.r#type, money).await?;
    me.set_attribute(99, 6, times).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let database = conn.database().await;
    let mut player = database.info.find_one().await?.with_context(|| format!("Player {:?} not found.", conn.uid()))?;
    player.name = msgdata.data.clone();
    database.info.update_one(player).await?;
    conn.send_msg(Pf::RspRename, &msgdata, 0).await?;
    Ok(())
}
