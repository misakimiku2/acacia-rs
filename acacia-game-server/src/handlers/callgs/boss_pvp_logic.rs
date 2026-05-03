use acacia_data::tables::tables;
use anyhow::Context;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::{conn::Conn, time::ServerTime};
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("BossPvpLogic_GetOpenID")]
async fn get_opem_id(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let current_time = ServerTime::now();
    let mut current_id = 0;
    for bosspvp_boss_challenge_data in tables().bosspvp_boss_challenge.all() {
        if bosspvp_boss_challenge_data.start_time > current_time {
            continue;
        }
        if bosspvp_boss_challenge_data.end_time < current_time {
            continue;
        }
        current_id = bosspvp_boss_challenge_data.id;
    }

    let mut me = conn.manager().await;
    me.set_str_attribute(51, 0, current_id.to_string()).await?;
    if current_id != 0 {
        me.set_str_attribute(51, 1, "4".to_string()).await?;
    } else {
        me.set_str_attribute(51, 1, "0".to_string()).await?;
    }
    me.sync().await?;

    /*
    if let Some(sync) = rsp.extra_sync.as_mut() {
        sync.custom_str.insert(set_attribute(51, 0), "1".to_string());
        sync.custom_str.insert(set_attribute(51, 1), "4".to_string());
        for i in (1..=246).rev() {
            sync.custom_str.insert(set_attribute(51, 10 * i + 100 + 8), "5".to_string());
        }
    } */
    call_script!({"nID": current_id});
    Ok(())
}

#[callgs_reg("BossPvpLogic_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({"nSeed": 0});
    Ok(())
}

pub async fn level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;
    let time = param["nTime"].as_f64().with_context(|| format!("Expected 'nTime' but not found in param {:?}", param))?;

    /*
    let mut me = conn.manager().await;
    let level_attr_base_id = 10 * (nid as u32) + 100;
    let before_score: u64 = me.get_str_attribute(51, level_attr_base_id + 4).await?.parse().unwrap_or(0);
    let score = 40000 - ((time * 100.0) as u64);
    if before_score < score {
        me.set_str_attribute(51, level_attr_base_id + 4, score.to_string()).await?;
        me.set_str_attribute(51, level_attr_base_id + 5, time.to_string()).await?;
        me.sync().await?;
    } */

    let arg = json!({
        "sCmd": "BossPvpLogic_LevelSettlement",
        "tbParam": []
    });
    conn.send_script("Chapter_DealLevelSettlement", arg).await?;
    Ok(())
}

pub async fn level_fail(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;
    let health_percentage = param["HealthPercentage"]
        .as_u64()
        .with_context(|| format!("Expected 'HealthPercentage' but not found in param {:?}", param))?;

    /*
    let mut me = conn.manager().await;
    let level_attr_base_id = 10 * (nid as u32) + 100;
    let before_score: u64 = me.get_str_attribute(51, level_attr_base_id + 4).await?.parse().unwrap_or(0);
    let score = (100 - health_percentage) * 80;
    if before_score < score {
        me.set_str_attribute(51, level_attr_base_id + 4, score.to_string()).await?;
        me.sync().await?;
    } */

    let arg = json!({
        "sCmd": "BossPvpLogic_LevelFail",
        "tbParam": []
    });
    conn.send_script("Chapter_DealLevelSettlement", arg).await?;
    Ok(())
}
