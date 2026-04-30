use acacia_commons::bits::set_template;
use anyhow::Context;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use crate::base::time::ServerTime;
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

        let item = core::Item {
            id: database.items.new_id().await?,
            template: set_template(1, 2, 1, 1)?,
            count: 1,
            enhance: Some(core::Enhance { level: 1, ..Default::default() }),
            slots: HashMap::from([(4, 2), (5, 3)]),
            ..Default::default()
        };
        database.items.update_one(item).await?;

        let item = core::Item {
            id: database.items.new_id().await?,
            template: set_template(2, 1, 255, 1)?,
            count: 1,
            flag: 0b1,
            enhance: Some(core::Enhance { level: 1, ..Default::default() }),
            ..Default::default()
        };
        database.items.update_one(item).await?;

        let item = core::Item {
            id: database.items.new_id().await?,
            template: set_template(7, 2, 1, 1)?,
            count: 1,
            flag: 0b1,
            ..Default::default()
        };
        database.items.update_one(item).await?;

        let item = core::Item {
            id: database.items.new_id().await?,
            template: set_template(9, 1, 2, 1)?,
            count: 1,
            ..Default::default()
        };
        database.items.update_one(item).await?;

        let item = core::Item {
            id: database.items.new_id().await?,
            template: set_template(9, 10, 1, 1)?,
            count: 1,
            ..Default::default()
        };
        database.items.update_one(item).await?;

        // database.attrs.update_many(init_data().attrs.iter().map(|(&k, &v)| (k, v))).await?;
        // database.items.update_many(init_data().items.iter().cloned()).await?;
        // database.attrs.set_attr(4, 0, 5).await?;
    }

    let rsp = protocol::NtfCallScript {
        api: "NtfWorldChatHostUrl".to_string(),
        arg: json!({"HostUrl": "https://snowbreak-ea-chat.amazingseasuncdn.com/chat/"}).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
