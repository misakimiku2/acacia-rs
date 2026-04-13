use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Rogue3D_CheckOpenAct")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Rogue3D_CheckOpenAct".to_string(),
        arg: json!({
            "nStartTime": 1689127200,
            "nEndTime": 1797710400,
            "nId": 1,
            "bOpen": true,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
