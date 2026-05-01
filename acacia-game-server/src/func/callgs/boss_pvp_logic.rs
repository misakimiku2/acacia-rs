use anyhow::Context;
use serde_json::{Value, json};
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
        sync.custom_str.insert(set_attribute(51, 0), "1".to_string());
        sync.custom_str.insert(set_attribute(51, 1), "4".to_string());
        for i in (1..=246).rev() {
            sync.custom_str.insert(set_attribute(51, 10 * i + 100 + 8), "5".to_string());
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

pub async fn level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;
    let time = param["nTime"].as_f64().with_context(|| format!("Expected 'nTime' but not found in param {:?}", param))?;

    let mut me = conn.manager().await;
    let level_attr_base_id = 10 * (nid as u32) + 100;
    let before_score: u64 = me.get_str_attribute(51, level_attr_base_id + 4).await?.parse().unwrap_or(0);
    let score = 40000 - ((time * 100.0) as u64);
    if before_score < score {
        me.set_str_attribute(51, level_attr_base_id + 4, score.to_string()).await?;
        me.set_str_attribute(51, level_attr_base_id + 5, time.to_string()).await?;
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
    }

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "BossPvpLogic_LevelSettlement",
            "tbParam": []
        })
        .to_string(),
        ..Default::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

pub async fn level_fail(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;
    let health_percentage = param["HealthPercentage"]
        .as_u64()
        .with_context(|| format!("Expected 'HealthPercentage' but not found in param {:?}", param))?;

    let mut me = conn.manager().await;
    let level_attr_base_id = 10 * (nid as u32) + 100;
    let before_score: u64 = me.get_str_attribute(51, level_attr_base_id + 4).await?.parse().unwrap_or(0);
    let score = (100 - health_percentage) * 80;
    if before_score < score {
        me.set_str_attribute(51, level_attr_base_id + 4, score.to_string()).await?;
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
    }

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": "BossPvpLogic_LevelFail",
            "tbParam": []
        })
        .to_string(),
        ..Default::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
