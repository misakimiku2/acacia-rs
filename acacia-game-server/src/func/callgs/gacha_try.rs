use acacia_commons::bits::{get_bit, set_bit};
use acacia_data::tables::tables;
use anyhow::bail;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::{base::conn::Conn, logic::item::Items};
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("GachaTry_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

pub async fn level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(act_id) = u32::deserialize(&param["nActId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nActId' as u32 in param {:?}", param);
    };
    let Ok(girl_idx) = u32::deserialize(&param["nGirlIdx"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nGirlIdx' as u32 in param {:?}", param);
    };

    let Some(activity_gacha_try_data) = tables().activity_gacha_try.find_one("act_id_1", act_id) else {
        bail!("The specified activity_gacha_try_data does not exist. act_id={:?}", act_id);
    };
    let Some(try_girl_id) = activity_gacha_try_data.try_girl.get(girl_idx as usize - 1) else {
        bail!("girl_idx out of range. activity_gacha_try_data={:?}, girl_idx={:?}", activity_gacha_try_data, girl_idx);
    };
    let Some(activity_gacha_try_girl_data) = tables().activity_gacha_try_girl.find_one("id_1", try_girl_id) else {
        bail!("The specified activity_gacha_try_girl_data does not exist. id={:?}", try_girl_id);
    };

    let mut me = conn.manager().await;
    let mut reward = Vec::new();
    let val = me.get_attribute(50, act_id * 10 + 3).await?;
    if !get_bit(val, girl_idx) {
        let val = set_bit(val, girl_idx, true);
        let items = Items::from(&activity_gacha_try_girl_data.reward);
        reward.push(items.as_vec());

        items.commit_add(&mut me).await?;
        me.set_attribute(50, act_id * 10 + 3, val).await?;
        me.sync().await?;
    }

    let arg = json!({
        "sCmd": "GachaTry_LevelSettlement",
        "tbParam": reward,
    });
    conn.send_script("Chapter_DealLevelSettlement", arg).await?;
    Ok(())
}
