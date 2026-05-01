use acacia_commons::bits::set_bit;
use anyhow::bail;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::{
    base::conn::Conn,
    logic::{
        activity::{get_activity_diy_data, set_activity_diy_data},
        item::Items,
    },
};
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

#[callgs_reg("Activity_Flag")]
async fn flag(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(nid) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let Ok(ntype) = u32::deserialize(&param["nType"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nType' as u32 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let val = get_activity_diy_data(&me, nid, 1).await?;
    let val = set_bit(val, ntype, true);
    set_activity_diy_data(&mut me, nid, 1, val).await?;
    me.sync().await?;

    call_script!("ActivityQuest_Flag", {"nId": nid});
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
    } else if ntype == 2 {
        get_award_short_sign(conn.clone(), msgdata, param.clone()).await?;
    }
    Ok(())
}

async fn get_award_day_sign(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(activity_id) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let mut me = conn.manager().await;
    let day_id = get_activity_diy_data(&me, activity_id, 3).await?;
    let day_status = get_activity_diy_data(&me, activity_id, 4).await?;
    if day_status == 2 {
        conn.send_err("tip.re_sign").await?;
        bail!("day_status={:?}", day_status);
    }
    let Some(activity_sign_day_data) = tables().activity_sign_day.find_one("activity_id_day_id_1", (activity_id, day_id)) else {
        conn.send_err("error.congif_rewardid_err").await?;
        bail!("The specified activity_sign_day_data does not exist. activity_id={:?}, day_id={:?}", activity_id, day_id);
    };
    let items = Items::from(&activity_sign_day_data.sign_reward);
    set_activity_diy_data(&mut me, activity_id, 4, 2).await?;
    items.commit_add(&mut me).await?;
    me.sync().await?;

    conn.send_script("DaySign", json!({"nType": 1, "nId": activity_id, "tbRewards": items.as_vec()})).await?;
    Ok(())
}

async fn get_award_short_sign(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(activity_id) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let mut me = conn.manager().await;
    let day_id = get_activity_diy_data(&me, activity_id, 3).await?;
    let day_status = get_activity_diy_data(&me, activity_id, 4).await?;
    if day_status == 2 {
        conn.send_err("tip.re_sign").await?;
        bail!("day_status={:?}", day_status);
    }
    let Some(activity_sign_short_data) = tables().activity_sign_short.find_one("activity_id_day_id_1", (activity_id, day_id)) else {
        conn.send_err("error.congif_rewardid_err").await?;
        bail!("The specified activity_sign_short_data does not exist. activity_id={:?}, day_id={:?}", activity_id, day_id);
    };
    let items = Items::from(&activity_sign_short_data.sign_reward);
    set_activity_diy_data(&mut me, activity_id, 4, 2).await?;
    items.commit_add(&mut me).await?;
    me.sync().await?;

    conn.send_script("ShortSign", json!({"nType": 1, "nId": activity_id, "tbRewards": items.as_vec()})).await?;
    Ok(())
}
