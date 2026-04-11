use anyhow::{Context, bail};
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use acacia_commons::bits::set_template;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("House_Request")]
async fn request(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let func_name = param["FuncName"].as_str().with_context(|| format!("Expected 'FuncName' but not found in param {:?}", param))?;
    match func_name {
        "GirlRegister" => request_girl_register(conn, msgdata, param).await?,
        "GirlRoomChange" => request_girl_room_change(conn, msgdata, param).await?,
        _ => {}
    };
    Ok(())
}

async fn request_girl_register(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let floor_id = param["FloorId"].as_i64().with_context(|| format!("Expected 'FloorId' but not found in param {:?}", param))?;
    let girl_id = param["GirlId"].as_u64().with_context(|| format!("Expected 'GirlId' but not found in param {:?}", param))? as u32;

    let floor_room_list: &[u32; 8] = match floor_id {
        2 => &[1, 2, 3, 4, 5, 6, 7, 8],
        3 => &[9, 10, 11, 12, 13, 14, 15, 16],
        4 => &[17, 18, 19, 20, 21, 22, 23, 24],
        _ => bail!("Invalid FloorId: {}", floor_id),
    };
    let manager = conn.manager().await;

    let mut new_room_id = 0;
    for &room_id in floor_room_list {
        let _room_girl_id = manager.attrs.get_attr(101, 2550 + room_id).await?.1;
        if _room_girl_id == 0 {
            new_room_id = room_id
        }
    }
    if new_room_id == 0 {
        bail!("Insufficient slots for FloorId: {}", floor_id);
    }

    let cost_item_id = set_template(5, 26, 1, 4)?;
    let Some(mut item) = manager.items.get_from_template(cost_item_id).await? else {
        bail!("Insufficient quantity for ItemId: {}", cost_item_id);
    };
    if item.count < 1 {
        bail!("Insufficient quantity for ItemId: {}", cost_item_id);
    }
    item.count = item.count - 1;
    manager.items.set(item.clone()).await?;

    let attr_1 = manager.attrs.set_attr(101, 2550 + new_room_id, girl_id).await?;
    let attr_2 = manager.attrs.set_attr(101, 50 * girl_id + 1, new_room_id).await?;
    let rsp = protocol::NtfSyncPlayer {
        items: vec![item],
        custom: HashMap::<u32, u32>::from([attr_1, attr_2]),
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "FloorId": param["FloorId"],
            "FuncName": "GirlRegisterSuccess",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn request_girl_room_change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let room_id = param["RoomId"].as_i64().with_context(|| format!("Expected 'RoomId' but not found in param {:?}", param))? as u32;
    let girl_id = param["GirlId"].as_u64().with_context(|| format!("Expected 'GirlId' but not found in param {:?}", param))? as u32;
    let manager = conn.manager().await;
    let old_room_id = manager.attrs.get_attr(101, 50 * girl_id + 1).await?.1;

    let mut attrs = HashMap::<u32, u32>::new();
    if room_id == 0 {
        let attr = manager.attrs.set_attr(101, 2550 + old_room_id, 0).await?;
        attrs.insert(attr.0, attr.1);
        let attr = manager.attrs.set_attr(101, 50 * girl_id + 1, 100).await?;
        attrs.insert(attr.0, attr.1);
    } else {
        let occupant_girl_id = manager.attrs.get_attr(101, 2550 + room_id).await?.1;

        if old_room_id != 0 && old_room_id != 100 {
            let attr = manager.attrs.set_attr(101, 2550 + old_room_id, 0).await?;
            attrs.insert(attr.0, attr.1);
        }

        let attr = manager.attrs.set_attr(101, 2550 + room_id, girl_id).await?;
        attrs.insert(attr.0, attr.1);

        if occupant_girl_id != 0 {
            let attr = manager.attrs.set_attr(101, 50 * occupant_girl_id + 1, 100).await?;
            attrs.insert(attr.0, attr.1);
        }

        let attr = manager.attrs.set_attr(101, 50 * girl_id + 1, room_id).await?;
        attrs.insert(attr.0, attr.1);
    }

    let rsp = protocol::NtfSyncPlayer {
        custom: attrs,
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "GirlId": param["GirlId"],
            "NewRoomId": param["RoomId"],
            "OldRoomId": old_room_id,
            "FuncName": "GirlRoomChangeSuccess",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
