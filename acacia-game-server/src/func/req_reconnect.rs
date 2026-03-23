use std::collections::HashMap;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqReconnect)]
async fn req_reconnect(conn: Arc<Conn>, msgdata: protocol::ReqReconnect) -> anyhow::Result<()> {
    /*
    conn.bind_uid(10000);
    let mut rsp = protocol::RspReconnect {
        session_id: "123456".into(),
        data: Some(core::Player {
            pid: conn.uid().unwrap(),
            sign: String::from("telegram @acacia_rs"),
            attrs: HashMap::from([(262144, 5), (273211, 4), (272327, 2)]),
            ..Default::default()
        }),
        ..Default::default()
    };
    conn.send_msg(Pf::RspReconnect as u32, &rsp, 0).await?; */
    Ok(())
}
