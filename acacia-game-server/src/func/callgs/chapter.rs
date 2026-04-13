use anyhow::Context;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Chapter_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "Chapter_EnterLevel".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Chapter_DealLevelSettlement")]
async fn deal_level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let cmd = param["sCmd"].as_str().with_context(|| format!("Expected 'sCmd' but not found in param {:?}", param))?;
    let database = conn.database().await;

    match cmd {
        "BossPvpLogic_LevelSettlement" => {
            let level_id = param["tbParam"]["nID"].as_u64().with_context(|| format!("Expected 'tbParam.nID' but not found in param {:?}", param))?;
            let time = param["tbParam"]["nTime"]
                .as_f64()
                .with_context(|| format!("Expected 'tbParam.nTime' but not found in param {:?}", param))?;
            let level_attr_base_id = 10 * (level_id as u32) + 100;
            let before_score: u64 = database.str_attrs.get_attr(51, level_attr_base_id + 4).await?.1.parse().unwrap_or(0);
            let score = 40000 - ((time * 100.0) as u64);
            if before_score < score {
                let attr_1 = database.str_attrs.set_attr(51, level_attr_base_id + 4, score.to_string()).await?;
                let attr_2 = database.str_attrs.set_attr(51, level_attr_base_id + 5, time.to_string()).await?;
                let rsp = protocol::NtfSyncPlayer {
                    custom_str: HashMap::<u32, String>::from([attr_1, attr_2]),
                    ..protocol::NtfSyncPlayer::default()
                };
                conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
            }
        }
        "BossPvpLogic_LevelFail" => {
            let level_id = param["tbParam"]["nID"].as_u64().with_context(|| format!("Expected 'tbParam.nID' but not found in param {:?}", param))?;
            let health_percentage = param["tbParam"]["HealthPercentage"]
                .as_u64()
                .with_context(|| format!("Expected 'tbParam.HealthPercentage' but not found in param {:?}", param))?;
            let level_attr_base_id = 10 * (level_id as u32) + 100;
            let before_score: u64 = database.str_attrs.get_attr(51, level_attr_base_id + 4).await?.1.parse().unwrap_or(0);
            let score = (100 - health_percentage) * 80;
            if before_score < score {
                let attr = database.str_attrs.set_attr(51, 10 * (level_id as u32) + 100 + 4, score.to_string()).await?;
                let rsp = protocol::NtfSyncPlayer {
                    custom_str: HashMap::<u32, String>::from([attr]),
                    ..protocol::NtfSyncPlayer::default()
                };
                conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
            }
        }
        _ => {
            let level_id = param["tbParam"]["nID"].as_i64().unwrap_or(-1);

            if level_id > 0 {
                let level_times = database.attrs.get_attr(22, level_id as u32).await?.0 + 1;
                let attr = database.attrs.set_attr(22, level_id as u32, level_times).await?;
                let rsp = protocol::NtfSyncPlayer {
                    custom: HashMap::<u32, u32>::from([attr]),
                    ..protocol::NtfSyncPlayer::default()
                };
                conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

                let level_star = param["tbParam"]["nID"].as_i64().unwrap_or(-1);
                if level_star > 0 {
                    let attr = database.attrs.set_attr(21, level_id as u32, level_star as u32).await?;
                    let rsp = protocol::NtfSyncPlayer {
                        custom: HashMap::<u32, u32>::from([attr]),
                        ..protocol::NtfSyncPlayer::default()
                    };
                    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
                }
            }
        }
    }

    let rsp = protocol::NtfCallScript {
        api: "Chapter_DealLevelSettlement".to_string(),
        arg: json!({
            "sCmd": param["sCmd"],
            "tbParam": []
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
