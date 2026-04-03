use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[callgs_reg("Lineup_Update")]
async fn update(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let manager = conn.manager().await;
    let solution = core::Lineup {
        index: param["index"].as_u64().unwrap() as u32,
        name: param["name"].as_str().unwrap().to_string(),
        member1: param["member1"].as_u64().unwrap(),
        member2: param["member2"].as_u64().unwrap(),
        member3: param["member3"].as_u64().unwrap(),
    };
    manager.solutions.update(solution.clone()).await?;
    let rsp = protocol::NtfSyncLineup {
        lineup: Some(solution),
        ..protocol::NtfSyncLineup::default()
    };
    conn.send_msg(Pf::NtfSynclineup, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "UpdateLineup".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
