use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_commons::time::ServerTime;
use acacia_data::init_data::init_data;
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqLogin)]
async fn req_login(conn: Arc<Conn>, msgdata: protocol::ReqLogin) -> anyhow::Result<()> {
    conn.bind_uid(10000).await;
    init_userdata(&conn).await?;

    let manager = conn.manager().await;
    let mut rsp = protocol::RspLogin {
        session_id: "123456".into(),
        data: Some(core::Player {
            level: 100,
            pid: conn.uid().unwrap(),
            sign: String::from("telegram @acacia_rs"),
            vigor: 240,
            items: manager.items.get_all().await?,
            solutions: manager.solutions.get_all().await?,
            attrs: manager.attrs.get_all().await?,
            str_attrs: manager.str_attrs.get_all().await?,
            show_items: vec![0, 0, 0, 10040101],
            ..Default::default()
        }),
        timestamp: ServerTime::now() as u32,
        ..Default::default()
    };

    if let Some(ref client_profile) = msgdata.client_profile {
        if !client_profile.resource_version.starts_with("3.5.") {
            rsp.error_code = 108;
        }
    } else {
        rsp.error_code = 304;
    }

    conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
    Ok(())
}

async fn init_userdata(conn: &Arc<Conn>) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    if manager.attrs.get_all().await?.len() == 0 {
        manager.attrs.set_many(init_data().attrs.iter().map(|(&k, &v)| (k, v))).await?;
        manager.items.set_many(init_data().items.iter().cloned()).await?;
    }
    manager.attrs.set_attr(4, 0, 5).await?;
    Ok(())
}
