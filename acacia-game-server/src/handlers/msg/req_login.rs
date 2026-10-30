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
    // 【本地私服改造】宽松登录：官方 XGSDK token 每次登录都变化，
    // 未匹配到已注册 token 时，自动落到默认账号 (pid=10001)
    let player_config = match PlayerConfigRepository::find_one_by_token(msgdata.token.clone()).await? {
        Some(existing) => existing,
        None => {
            let default_pid: u64 = 10001;
            let repo = PlayerConfigRepository::new(default_pid);
            match repo.find_one().await? {
                Some(existing) => existing,
                None => {
                    let config = acacia_database::models::PlayerConfig {
                        pid: default_pid,
                        ptoken: msgdata.token.clone(),
                        ..Default::default()
                    };
                    repo.update_one(config).await?;
                    repo.find_one()
                        .await?
                        .with_context(|| "failed to create default player config".to_string())?
                }
            }
        }
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
        tracing::info!("client resource_version: {}", client_profile.resource_version);
    } else {
        tracing::warn!("client_profile is None (continuing anyway)");
    }

    conn.send_msg(Pf::RspLogin, &rsp, 0).await?;
    conn.send_script("NtfWorldChatHostUrl", json!({"HostUrl": "https://snowbreak-ea-chat.amazingseasuncdn.com/chat/"}))
        .await?;
    Ok(())
}

/// 引导完成状态引导（参考 MikuSB BuildLobbyBootstrapAttrs）：
/// 客户端 GuideLogic 用属性 gid 4 判断每个引导的进度，值大于配置步数
/// （999 足够）视为已完成；gid 4 sid 0 是新手引导总开关（5 = 全部完成）。
/// 该状态缺失时客户端每次登录都会从序章 50174（开场白）重新进入新手引导。
/// 幂等：检测到 4.0 != 5 时才整体补写。
async fn ensure_guide_bootstrap(me: &mut crate::logic::player::PlayerManager) -> anyhow::Result<()> {
    if me.get_attribute(4, 0).await? == 5 {
        return Ok(());
    }
    tracing::info!("guide bootstrap: marking all guides as completed (gid 4)");
    me.set_attribute(4, 0, 5).await?;
    for guide_id in 1u32..=150 {
        me.set_attribute(4, guide_id, 999).await?;
    }
    for guide_id in 10000u32..=10300 {
        me.set_attribute(4, guide_id, 999).await?;
    }
    for guide_id in 11000u32..=11300 {
        me.set_attribute(4, guide_id, 999).await?;
    }
    for guide_id in 12000u32..=12100 {
        me.set_attribute(4, guide_id, 999).await?;
    }
    for guide_id in 22000u32..=22100 {
        me.set_attribute(4, guide_id, 999).await?;
    }
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

    ensure_guide_bootstrap(&mut me).await?;

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
