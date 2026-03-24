use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("ChangeMainScene")]
async fn change_main_scene(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let attr = manager.attrs.set_attr(132, 1, param["nId"].as_u64().unwrap() as u32).await?;

    let rsp = protocol::NtfSyncPlayer {
        custom: HashMap::from([attr]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "ChangeMainScene".to_string(),
        arg: "[]".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
