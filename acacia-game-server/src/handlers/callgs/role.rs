use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol::{self, Pf};

#[callgs_reg("Role_EnterLevel")]
async fn enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({"nSeed": 0});
    Ok(())
}

pub async fn level_settlement(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let mut me = conn.manager().await;
    // let drop_result = chapter_level_drop(&mut me, &param).await?; // WIP
    // chapter_level_logger(&mut me, &param).await?;
    me.sync().await?;

    let arg = json!({
        "sCmd": "Role_LevelSettlement",
        //"tbParam": [drop_result]
    });
    conn.send_script("Chapter_DealLevelSettlement", &arg).await?;
    Ok(())
}
