use anyhow::bail;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("Activity_Refresh")]
async fn refresh(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(nid) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let Some(activity_data) = tables().activity.find_one("id_1", nid) else {
        bail!("The specified activity_data does not exist. id={:?}", nid);
    };

    let mut me = conn.manager().await;
    me.set_attribute(50, nid * 10, activity_data.start_time.unwrap_or(0) as u32).await?;
    me.sync().await?;
    Ok(())
}

#[callgs_reg("Activity_GetAward")]
async fn get_award(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(ntype) = u32::deserialize(&param["nType"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nType' as u32 in param {:?}", param);
    };

    if ntype == 1 {
        get_award_day_sign(conn.clone(), msgdata, param.clone()).await?;
    }
    Ok(())
}

async fn get_award_day_sign(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(nid) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let mut me = conn.manager().await;
    me.set_attribute(50, nid * 10 + 4, 2).await?;
    me.sync().await?;

    conn.send_script("DaySign", json!({"nType": 1, "nId": nid, "tbRewards": [[4, 3, 1, 1, 2000]]})).await?;
    Ok(())
}
