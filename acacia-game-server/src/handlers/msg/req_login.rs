use acacia_data::tables::tables;
use acacia_database::repositories::player_config::PlayerConfigRepository;
use anyhow::Context;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::base::time::ServerTime;
use crate::logic::activity::{activity_is_open, get_activity_diy_data, get_activity_sign_max_days, set_activity_diy_data};
use crate::{base::conn::Conn, logic::girl::new_girl};
use acacia_derive::func_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[func_reg(Pf::ReqLogin)]
async fn req_login(conn: Arc<Conn>, msgdata: protocol::ReqLogin) -> anyhow::Result<()> {
    let Some(player_config) = PlayerConfigRepository::find_one_by_token(msgdata.token.clone()).await? else {
        let rsp = protocol::RspLogin {
            error_code: 304,
            ..Default::default()
        };
        conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
        return Ok(());
    };
    conn.bind_uid(player_config.pid).await;
    init_userdata(&conn).await?;

    let database = conn.database().await;
    let mut player = database.info.find_one().await?.with_context(|| format!("The player {:?} has not been initialized", conn.uid()))?;
    player.last_login_time = ServerTime::now() as u32;
    database.info.update_one(player.clone()).await?;
    player.items = database.items.find_all().await?;
    player.solutions = database.solutions.find_all().await?;
    player.attrs = database.attrs.find_all().await?;
    player.str_attrs = database.str_attrs.find_all().await?;
    let enable_amend = HashMap::from([
        ("GROWTHACTION_SWITCH".to_string(), 1),
        ("LOCALIZATION_TIME_SWITCH".to_string(), 1688140800),
        ("SKIN_AMEND_SWITCH".to_string(), 1),
        ("BUTTOCK_SWITCH".to_string(), 1),
        ("ENABLE_SYNC_LOAD_OPTIMIZE".to_string(), 0),
        ("LOCALIZATION_TIME_OVER_SWITCH".to_string(), 0),
        ("AGE_PROTECT_LEVEL".to_string(), 27),
        ("BREASTS_SWITCH".to_string(), 1),
    ]);

    let mut rsp = protocol::RspLogin {
        session_id: msgdata.token,
        data: Some(player),
        timestamp: ServerTime::now() as u32,
        global_attrs: HashMap::new(),
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
    conn.send_script("NtfWorldChatHostUrl", json!({"HostUrl": "https://snowbreak-ea-chat.amazingseasuncdn.com/chat/"}))
        .await?;
    Ok(())
}

async fn init_userdata(conn: &Arc<Conn>) -> anyhow::Result<()> {
    let database = conn.database().await;
    let mut me = conn.manager().await;

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

        me.set_attribute(99, 6, 1).await?; // 绕过第一次登录 SDK 请求
        new_girl(&mut me, 1, 2, 1, 1).await?;
        me.add_item_count(9, 10, 1, 1, 1).await?;

        // database.attrs.update_many(init_data().attrs.iter().map(|(&k, &v)| (k, v))).await?;
        // database.items.update_many(init_data().items.iter().cloned()).await?;
        // database.attrs.set_attr(4, 0, 5).await?;
    }

    let now_month = ServerTime::month_u32();
    if now_month != me.get_attribute(0, 2).await? {
        for activity_data in tables().activity.find("class_1", "signin_monthly") {
            if !activity_is_open(&activity_data) {
                continue;
            };
            if tables().activity_sign_day.find_one("activity_id_1", activity_data.id).is_some() {
                set_activity_diy_data(&mut me, activity_data.id, 3, 0).await?;
                set_activity_diy_data(&mut me, activity_data.id, 4, 0).await?;
            };
        }
        me.set_attribute(0, 2, now_month).await?;
    }

    let now_day = ServerTime::day_u32();
    if now_day != me.get_attribute(0, 0).await? {
        for activity_data in tables().activity.find("class_1", "signin_monthly") {
            if !activity_is_open(&activity_data) {
                continue;
            };
            let day_sign_days = get_activity_diy_data(&me, activity_data.id, 3).await?;
            let day_sign_status = get_activity_diy_data(&me, activity_data.id, 4).await?;
            if day_sign_status != 1 && day_sign_days < get_activity_sign_max_days(activity_data.id) {
                set_activity_diy_data(&mut me, activity_data.id, 3, day_sign_days + 1).await?;
                set_activity_diy_data(&mut me, activity_data.id, 4, 1).await?;
            };
        }
        me.set_attribute(0, 0, now_day).await?;
    }
    Ok(())
}
