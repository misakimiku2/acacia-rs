use anyhow::bail;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

const SHOWITEM_CARD: [usize; 5] = [0, 1, 2, 6, 7];
const SHOWITEM_SKIN: [usize; 5] = [8, 9, 10, 11, 12];
const SHOWITEM_BADGE: [usize; 3] = [13, 14, 15];

#[callgs_reg("PlayerSetting_ChangeShowCard")]
async fn change_show_card(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nid) = param["nID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nID' but not found in param {:?}", param);
    };

    let mut me = conn.manager().await;
    me.set_show_item(3, nid).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_SetProfileFace")]
async fn set_profile_face(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nhead_item_id) = param["nHeadItemID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nHeadItemID' but not found in param {:?}", param);
    };
    let nframe_item_id = param["nFrameItemID"].as_u64().unwrap_or(0);

    let mut me = conn.manager().await;
    me.set_show_item(4, nhead_item_id).await?;
    me.set_show_item(5, nframe_item_id ).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_ChangeAccountShowCard")]
async fn change_account_show_card(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nitem_id) = param["nItemID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nItemID' but not found in param {:?}", param);
    };
    let Some(nindex) = param["nIndex"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nIndex' but not found in param {:?}", param);
    };
    let showitem_index = SHOWITEM_CARD[nindex as usize - 1];

    let mut me = conn.manager().await;
    if nitem_id > 0 {
        me.set_show_item(showitem_index, nitem_id as u64).await?;
    } else {
        me.set_show_item(showitem_index, 0).await?;
    }
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_ChangeAccountShowSkin")]
async fn change_account_show_skin(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nitem_id) = param["nItemID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nItemID' but not found in param {:?}", param);
    };
    let Some(nindex) = param["nIndex"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nIndex' but not found in param {:?}", param);
    };
    let showitem_index = SHOWITEM_SKIN[nindex as usize - 1];

    let mut me = conn.manager().await;
    if nitem_id > 0 {
        me.set_show_item(showitem_index, nitem_id as u64).await?;
    } else {
        me.set_show_item(showitem_index, 0).await?;
    }
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_SetShowCover")]
async fn set_show_cover(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nid) = param["nID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nID' but not found in param {:?}", param);
    };

    let mut me = conn.manager().await;
    me.set_show_item(16, nid).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_SetShowNameCard")]
async fn set_show_name_card(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nid) = param["nID"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nID' but not found in param {:?}", param);
    };

    let mut me = conn.manager().await;
    me.set_show_item(17, nid).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}

#[callgs_reg("PlayerSetting_SetBirthday")]
async fn set_birthday(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(nday) = param["nDay"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nDay' but not found in param {:?}", param);
    };
    let Some(nmouth) = param["nMouth"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nMouth' but not found in param {:?}", param);
    };

    let mut me = conn.manager().await;
    me.set_show_attr(21, nmouth as u32).await?;
    me.set_show_attr(22, nday as u32).await?;
    // me.set_attribute(151, 1, 2026).await?;
    me.sync().await?;

    call_script!(null);
    Ok(())
}
