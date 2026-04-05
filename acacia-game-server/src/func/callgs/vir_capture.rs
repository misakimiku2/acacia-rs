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
