use anyhow::Context;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::time::ServerTime;
use acacia_data::init_data::init_data;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqLogin)]
async fn req_login(conn: Arc<Conn>, msgdata: protocol::ReqLogin) -> anyhow::Result<()> {
    conn.bind_uid(10000).await;
    init_userdata(&conn).await?;

    let database = conn.database().await;
    let mut player = database.info.find_one().await?.with_context(|| format!("The player {:?} has not been initialized", conn.uid()))?;
    player.last_login_time = ServerTime::now() as u32;
    database.info.update_one(player.clone()).await?;
    player.items = database.items.find_all().await?;
    player.solutions = database.solutions.find_all().await?;
    player.attrs = database.attrs.find_all().await?;
    player.str_attrs = database.str_attrs.find_all().await?;

    let mut rsp = protocol::RspLogin {
        session_id: "123456".into(),
        data: Some(player),
        timestamp: ServerTime::now() as u32,
        ..Default::default()
    };

    if let Some(ref client_profile) = msgdata.client_profile {
        if !client_profile.resource_version.starts_with("3.5.") {
            // rsp.error_code = 108;
        }
    } else {
        rsp.error_code = 304;
    }

    conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
    Ok(())
}

async fn init_userdata(conn: &Arc<Conn>) -> anyhow::Result<()> {
    let database = conn.database().await;
    if database.info.find_one().await?.is_none() {
        let player = core::Player {
            pid: conn.uid().with_context(|| format!("The pid should not be None"))?,
            name: String::from("分析员"),
            // sign: String::from("telegram @acacia_rs"),
            level: 1,
            vigor: 240,
            create_time: ServerTime::now() as u32,
            ..Default::default()
        };
        database.info.update_one(player).await?;
        database.attrs.set_attr(99, 6, 1).await?; // 绕过第一次登录 SDK 请求
        // database.attrs.update_many(init_data().attrs.iter().map(|(&k, &v)| (k, v))).await?;
        // database.items.update_many(init_data().items.iter().cloned()).await?;
    }
    // database.attrs.set_attr(4, 0, 5).await?;
    Ok(())
}
