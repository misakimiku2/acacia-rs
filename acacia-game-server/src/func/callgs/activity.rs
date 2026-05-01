use anyhow::bail;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("Activity_Refresh")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(nid) = u32::deserialize(&param["nId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nId' as u32 in param {:?}", param);
    };
    let Some(activity_data) = tables().activity.find_one("id_1", nid) else {
        bail!("The specified activity_data does not exist. id={:?}", nid);
    };

    let mut me = conn.manager().await;
    me.set_attribute(50, nid * 10, activity_data.start_time as u32).await?;
    me.sync().await?;

    call_script!({"nId": param["nId"]});
    Ok(())
}
