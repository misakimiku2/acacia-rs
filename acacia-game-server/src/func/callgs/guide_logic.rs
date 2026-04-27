use anyhow::Context;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("GuideLogic_SetBeginner")]
async fn set_beginner(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let ntype = param["nType"].as_u64().with_context(|| format!("Expected 'nType' but not found in param {:?}", param))? as u32;

    let mut me = conn.manager().await;
    me.set_attribute(187, 1, ntype).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "GuideLogic_SetBeginner".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
