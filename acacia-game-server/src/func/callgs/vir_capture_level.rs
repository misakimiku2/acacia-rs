use anyhow::Ok;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

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

#[callgs_reg("VirCaptureLevel_SaveFightData")]
async fn save_fight_data(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let level_id = param["nLevelID"].as_i64().unwrap();
    let point_id = param["nRegionId"].as_i64().unwrap() as u32;
    let map_index = if level_id == 5810 {
        1
    } else {
        return Ok(());
    };

    let manager = conn.manager().await;
    let map_base_id = 10000 + (map_index - 1) * 3000;
    let mut attr = manager.attrs.get_attr(128, map_base_id + ((point_id - 500) / 30) + 1001).await?;
    attr.1 = attr.1 | (1 << ((point_id - 500) % 30));
    manager.attrs.set(attr.0, attr.1).await?;

    let rsp = protocol::NtfCallScript {
        api: "VirCaptureLevel_SaveFightData".to_string(),
        arg: json!({
            "tbRewards": [],
            "nSeed": param["nSeed"],
            "nLevelID": param["nLevelID"],
            "nRegionId": param["nRegionId"],
        })
        .to_string(),
        extra_sync: Some(protocol::NtfSyncPlayer {
            custom: HashMap::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        }),
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("VirCaptureLevel_SavePos")]
async fn save_pos(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let level_id = param["nLevelID"].as_i64().unwrap();
    let map_index = if level_id == 5810 {
        1
    } else {
        return Ok(());
    };
    let pos_x = param["nPosX"].as_i64().unwrap() as u32;
    let pos_y = param["nPosY"].as_i64().unwrap() as u32;
    let pos_z = param["nPosZ"].as_i64().unwrap() as u32;
    let toward = param["nToward"].as_i64().unwrap() as u32;

    let manager = conn.manager().await;
    let map_base_id = 10000 + (map_index - 1) * 3000;
    let pos_x_attr = manager.attrs.set_attr(128, map_base_id + 3, pos_x).await?;
    let pos_y_attr = manager.attrs.set_attr(128, map_base_id + 4, pos_y).await?;
    let pos_z_attr = manager.attrs.set_attr(128, map_base_id + 5, pos_z).await?;
    let toward_attr = manager.attrs.set_attr(128, map_base_id + 6, toward).await?;

    let rsp = protocol::NtfCallScript {
        api: "VirCaptureLevel_SavePos".to_string(),
        arg: json!({
            "tbRewards": [],
            "nSeed": param["nSeed"],
            "nLevelID": param["nLevelID"],
            "nRegionId": param["nRegionId"],
        })
        .to_string(),
        extra_sync: Some(protocol::NtfSyncPlayer {
            custom: HashMap::from([pos_x_attr, pos_y_attr, pos_z_attr, toward_attr]),
            ..protocol::NtfSyncPlayer::default()
        }),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
