use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("VirCaptureLevel_EnterLevel")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "VirCaptureLevel_EnterLevel".to_string(),
        arg: json!({"nSeed": 0}).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("VirCaptureLevel_ChangeFlag")]
async fn change_flag(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "VirCaptureLevel_SaveCapture".to_string(),
        arg: json!({
            "nRegionId": param["nRegionId"],
            "bClean": param["bClean"],
            "nSeed": param["nSeed"],
            "nLevelID": param["nLevelID"],
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("VirCaptureLevel_SaveCapture")] // WIP
async fn save_capture(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "VirCaptureLevel_SaveCapture".to_string(),
        arg: json!({}).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
