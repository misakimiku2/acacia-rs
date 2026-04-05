use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Chapter_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Chapter_EnterLevel".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_DealLevelSettlement")]
async fn deal_level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let level_id = param["tbParam"]["nID"].as_i64().unwrap_or(-1);
    if level_id > 0 {
        let level_times = manager.attrs.get_attr(22, level_id as u32).await?.0 + 1;
        let attr = manager.attrs.set_attr(22, level_id as u32, level_times).await?;
        let rsp = protocol::NtfSyncPlayer {
            custom: HashMap::<u32, u32>::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

        let level_star = param["tbParam"]["nID"].as_i64().unwrap_or(-1);
        if level_star > 0 {
            let attr = manager.attrs.set_attr(21, level_id as u32, level_star as u32).await?;
            let rsp = protocol::NtfSyncPlayer {
                custom: HashMap::<u32, u32>::from([attr]),
                ..protocol::NtfSyncPlayer::default()
            };
            conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
        }
    }

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": param["sCmd"],
            "tbParam": []
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
