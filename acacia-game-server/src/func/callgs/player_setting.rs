use anyhow::Context;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

const SHOWITEM_CARD: [usize; 5] = [0, 1, 2, 6, 7];
const SHOWITEM_SKIN: [usize; 5] = [8, 9, 10, 11, 12];
const SHOWITEM_BADGE: [usize; 3] = [13, 14, 15];

#[callgs_reg("PlayerSetting_ChangeShowCard")]
async fn change_show_card(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nID"].as_u64().with_context(|| format!("Expected 'nID' but not found in param {:?}", param))?;

    let mut me = conn.manager().await;
    me.set_show_item(3, nid).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "PlayerSetting_ChangeShowCard".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("PlayerSetting_ChangeAccountShowCard")]
async fn change_account_show_card(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nitem_id = param["nItemID"].as_u64().with_context(|| format!("Expected 'nItemID' but not found in param {:?}", param))?;
    let nindex = param["nIndex"].as_u64().with_context(|| format!("Expected 'nIndex' but not found in param {:?}", param))? as usize;

    let mut me = conn.manager().await;
    me.set_show_item(SHOWITEM_CARD[nindex - 1], nitem_id).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "PlayerSetting_ChangeAccountShowCard".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
