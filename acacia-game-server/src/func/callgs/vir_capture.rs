use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("VirCapture_CheckOpenAct")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let nid = 2;
    let attr = conn.manager().await.attrs.set_attr(128, 1, nid).await?;
    let rsp = protocol::NtfSyncPlayer {
        custom: HashMap::<u32, u32>::from([attr]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "VirCapture_CheckOpenAct".to_string(),
        arg: json!({
            "nStartTime": 1720641600,
            "nId": nid,
            "bOpen": true,
            "nEndTime": 1924891200,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("VirCapture_SwitchDayNight")]
async fn switch_day_night(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let map_id = param["nMapId"].as_i64().unwrap();
    let map_index = if map_id == 5810 {
        1
    } else {
        return Ok(());
    };

    let manager = conn.manager().await;
    let map_base_id = 10000 + (map_index - 1) * 3000;
    let mut attr = manager.attrs.get_attr(128, map_base_id + 7).await?;
    attr.1 = match attr.1 {
        1 => 2,
        2 => 1,
        _ => 1,
    };
    manager.attrs.set(attr.0, attr.1).await?;

    let rsp = protocol::NtfCallScript {
        api: "VirCapture_SwitchDayNight".to_string(),
        arg: json!({"nMapId": param["nMapId"]}).to_string(),
        extra_sync: Some(protocol::NtfSyncPlayer {
            custom: HashMap::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        }),
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
