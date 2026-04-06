use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_commons::bits::set_attribute;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol::{self, NtfSyncPlayer, Pf};

#[callgs_reg("BossPvpLogic_GetOpenID")]
async fn get_opem_id(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let mut rsp = protocol::NtfCallScript {
        api: "BossPvpLogic_GetOpenID".to_string(),
        arg: "{\"nID\": 1}".to_string(),
        extra_sync: Some(NtfSyncPlayer {
            custom_str: HashMap::<u32, String>::new(),
            ..protocol::NtfSyncPlayer::default()
        }),
    };
    if let Some(sync) = rsp.extra_sync.as_mut() {
        sync.custom_str.insert(set_attribute(51, 0)?, "1".to_string());
        sync.custom_str.insert(set_attribute(51, 1)?, "4".to_string());
        for i in (1..=246).rev() {
            sync.custom_str.insert(set_attribute(51, 10 * i + 100 + 8)?, "5".to_string());
        }
    }
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("BossPvpLogic_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "BossPvpLogic_EnterLevel".to_string(),
        arg: "{\"nSeed\": 0}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
