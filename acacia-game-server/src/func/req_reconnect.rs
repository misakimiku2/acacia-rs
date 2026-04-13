use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_commons::time::ServerTime;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqReconnect)]
async fn req_reconnect(conn: Arc<Conn>, _: protocol::ReqReconnect) -> anyhow::Result<()> {
    conn.bind_uid(10000).await;

    let database = conn.database().await;
    let rsp = protocol::RspReconnect {
        session_id: "123456".into(),
        data: Some(core::Player {
            level: 100,
            pid: conn.uid().unwrap(),
            sign: String::from("telegram @acacia_rs"),
            vigor: 240,
            items: database.items.find_all().await?,
            solutions: database.solutions.find_all().await?,
            attrs: database.attrs.find_all().await?,
            str_attrs: database.str_attrs.find_all().await?,
            show_items: vec![0, 0, 0, 10040101],
            ..Default::default()
        }),
        timestamp: ServerTime::now() as u32,
        ..Default::default()
    };

    conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
    Ok(())
}
