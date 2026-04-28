use anyhow::Context;
use serde_json::Value;
use acacia_data::tables::tables;

use super::manager::PlayerManager;

pub async fn chapter_level_logger(me: &mut PlayerManager, param: &Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_i64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))? as u32;
    let nstar = param["nStar"].as_i64().unwrap_or(-1);

    let level_times = me.get_attribute(22, nid).await? + 1;
    me.set_attribute(22, nid, level_times).await?;
    println!("22 {nid} {level_times}");
    if nstar > 0 {
        me.set_attribute(21, nid, nstar as u32).await?;
    }
    Ok(())
}
