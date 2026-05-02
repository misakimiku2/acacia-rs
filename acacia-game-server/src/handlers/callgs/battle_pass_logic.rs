use acacia_data::tables::tables;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::{conn::Conn, time::ServerTime};
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("BattlePassLogic_ClientRefresh")]
async fn client_refresh(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let mut me = conn.manager().await;

    let now = ServerTime::now();
    let mut nid = 0;
    for battle_timelist_data in tables().battlepass_timelist.all() {
        if now >= battle_timelist_data.start_time && now <= battle_timelist_data.end_time {
            nid = battle_timelist_data.id;
            break;
        }
    }
    me.set_attribute(25, 1, nid).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "BattlePassLogic_ClientRefresh".to_string(),
        arg: json!({}).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
