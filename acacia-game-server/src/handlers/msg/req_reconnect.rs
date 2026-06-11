use acacia_database::repositories::player_config::PlayerConfigRepository;
use anyhow::Context;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::time::ServerTime;
use acacia_derive::func_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[func_reg(Pf::ReqReconnect)]
async fn req_reconnect(conn: Arc<Conn>, msgdata: protocol::ReqReconnect) -> anyhow::Result<()> {
    let Some(player_config) = PlayerConfigRepository::find_one_by_token(msgdata.session_id.clone()).await? else {
        let rsp = protocol::RspLogin {
            error_code: 304,
            ..Default::default()
        };
        conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
        return Ok(());
    };
    conn.bind_uid(player_config.pid).await;

    let database = conn.database().await;
    let mut player = database.info.find_one().await?.with_context(|| format!("The player {:?} has not been initialized", conn.uid()))?;
    player.last_login_time = ServerTime::now() as u32;
    database.info.update_one(player.clone()).await?;
    player.items = database.items.find_all().await?;
    player.solutions = database.solutions.find_all().await?;
    player.attrs = database.attrs.find_all().await?;
    player.str_attrs = database.str_attrs.find_all().await?;

    let rsp = protocol::RspReconnect {
        session_id: "123456".into(),
        data: Some(player),
        timestamp: ServerTime::now() as u32,
        ..Default::default()
    };
    conn.send_msg(Pf::RspReconnect, &rsp, 0).await?;
    Ok(())
}
