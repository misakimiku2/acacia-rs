use std::collections::HashMap;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::base::dispatcher::HandlerResult;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqLogin)]
async fn req_login(conn: Arc<Conn>, msgdata: protocol::ReqLogin) -> anyhow::Result<HandlerResult> {
    conn.bind_uid(10000);
    let mut rsp = protocol::RspLogin {
        session_id: "123456".into(),
        data: Some(core::Player {
            pid: conn.uid().unwrap(),
            attrs: HashMap::from([(262144, 5), (273211, 4), (272327, 2)]),
            ..Default::default()
        }),
        ..Default::default()
    };
    if let Some(ref client_profile) = msgdata.client_profile {
        if !client_profile.resource_version.starts_with("3.5.") {
            rsp.error_code = 108;
        }
    } else {
        rsp.error_code = 304;
    }
    conn.send_msg(Pf::RspLogin as u32, &rsp, 0).await?;
    Ok(HandlerResult::Ok)
}
