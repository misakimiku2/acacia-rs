use acacia_data::tables::tables;
use anyhow::Context;
use serde_json::Value;
use tracing::warn;

use super::drop::drop;
use super::player::PlayerManager;

pub async fn chapter_level_logger(me: &mut PlayerManager, param: &Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_i64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))? as u32;
    let nstar = param["nStar"].as_i64().unwrap_or(-1);

    let level_times = me.get_attribute(22, nid).await? + 1;
    me.set_attribute(22, nid, level_times).await?;
    if nstar > 0 {
        me.set_attribute(21, nid, nstar as u32).await?;
    } else if matches!(nid, 10121 | 10122 | 10123 | 10124) {
        me.set_attribute(21, nid, 7).await?;
    } else if nid < 50000 {
        warn!("nStar -1 appeared in battle level {nid}.");
    }
    Ok(())
}

pub async fn chapter_level_drop(me: &mut PlayerManager, param: &Value) -> anyhow::Result<Vec<Vec<u32>>> {
    let nid = param["nID"].as_i64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))? as u32;
    let level_data = tables()
        .chapter_level
        .find_one("id_1", nid)
        .with_context(|| format!("The specified chapter_level_data does not exist."))?;

    if let Some(player_exp) = &level_data.player_exp {
        me.add_exp(*player_exp).await?;
    }

    let mut drop_result: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
    if let Some(first_drop_id) = &level_data.first_drop_id {
        if me.get_attribute(22, nid).await? == 0 {
            for drop_id in first_drop_id {
                for ((g, d, p, l), num) in drop(*drop_id)? {
                    me.add_item_count(g, d, p, l, num).await?;
                    drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
                }
            }
        }
    }

    if let Some(base_drop_id) = &level_data.base_drop_id {
        for drop_id in base_drop_id {
            for ((g, d, p, l), num) in drop(*drop_id)? {
                me.add_item_count(g, d, p, l, num).await?;
                drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
            }
        }
    }

    if let Some(random_drop_id) = &level_data.random_drop_id {
        for drop_id in random_drop_id {
            for ((g, d, p, l), num) in drop(*drop_id)? {
                me.add_item_count(g, d, p, l, num).await?;
                drop_result.push(vec![g as u32, d as u32, p as u32, l as u32, num]);
            }
        }
    }
    Ok(drop_result)
}
