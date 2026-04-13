use anyhow::{Context, bail};
use serde_json::{Value, from_value, json};
use std::{collections::HashMap, sync::Arc};

use crate::base::conn::Conn;
use crate::logic::girl_love_level::GirlLoveLevel;
use crate::logic::girl_room_furniture::{GirlRoomFurniture, GirlRoomFurnitureFold};
use acacia_commons::bits::set_template;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[callgs_reg("House_Request")]
async fn request(conn: Arc<Conn>, msgdata: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let func_name = param["FuncName"].as_str().with_context(|| format!("Expected 'FuncName' but not found in param {:?}", param))?;
    match func_name {
        "GirlRegister" => request_girl_register(conn, msgdata, param).await?,
        "GirlRoomChange" => request_girl_room_change(conn, msgdata, param).await?,
        "ExchangeGift" => request_exchange_gift(conn, msgdata, param).await?,
        "GiveGiftToArea" => request_give_gift_to_area(conn, msgdata, param).await?,
        "ChangeFurnitureFoldByTmpId" => request_change_furniture_fold_by_tmp_id(conn, msgdata, param).await?,
        "ArcadeGameEnterMainUI" => request_arcade_game_enter_main_ui(conn, msgdata, param).await?,
        "ArcadeGameEnter" => request_arcade_game_enter(conn, msgdata, param).await?,
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
        for &room_id in &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24] {
            let _room_girl_id = manager.attrs.get_attr(101, 2550 + room_id).await?.1;
            if _room_girl_id == 0 {
                new_room_id = room_id
            }
        }
    }
    if new_room_id == 0 {
        bail!("Insufficient slots for FloorId: {}", floor_id);
    }

    let cost_item_id = set_template(5, 26, 1, 4)?;
    let Some(mut item) = manager.items.get_item_by_template(cost_item_id).await? else {
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

async fn request_exchange_gift(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let index = param["Index"].as_i64().with_context(|| format!("Expected 'Index' but not found in param {param:?}"))? as u32;
    let use_gold = param["UseGold"].as_bool().unwrap_or(false);

    let gift_exchange_data = tables()
        .gift_exchange
        .find_one("index_1", index)
        .with_context(|| format!("The specified gift_exchange_data does not exist."))?;

    let mut attrs = HashMap::<u32, u32>::new();
    let manager = conn.manager().await;
    if use_gold {
        let Some(gold_count) = gift_exchange_data.need_gold else {
            bail!("Gold cannot be used to buy this item {gift_exchange_data:?}");
        };
        let mut attr = manager.attrs.get_attr(1, 5).await?;
        if attr.1 < gold_count {
            bail!("Insufficient gold. ({}/{})", attr.1, gold_count);
        }
        attr.1 = attr.1 - gold_count;
        manager.attrs.set(attr.0, attr.1).await?;
        attrs.insert(attr.0, attr.1);
    } else {
        let money_count = gift_exchange_data.need_money;
        let mut attr = manager.attrs.get_attr(1, 27).await?;
        if attr.1 < money_count {
            bail!("Insufficient money. ({}/{})", attr.1, money_count);
        }
        attr.1 = attr.1 - money_count;
        manager.attrs.set(attr.0, attr.1).await?;
        attrs.insert(attr.0, attr.1);
    }

    let gift = gift_exchange_data.gift;
    let template_id = set_template(gift[0], gift[1], gift[2], gift[3])?;
    let item = if let Some(mut item) = manager.items.get_item_by_template(template_id).await? {
        item.count += 1;
        item
    } else {
        core::Item {
            id: manager.items.new_id().await?,
            template: template_id,
            count: 1,
            ..core::Item::default()
        }
    };
    manager.items.set(item.clone()).await?;

    let mut attr = manager.attrs.get_attr(101, 50 * gift[2] as u32 + 36).await?;
    attr.1 += 1;
    manager.attrs.set(attr.0, attr.1).await?;
    attrs.insert(attr.0, attr.1);

    let rsp = protocol::NtfSyncPlayer {
        items: vec![item],
        custom: attrs,
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "Award": [gift],
            "FuncName": "ExchangeGiftRsp",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn request_give_gift_to_area(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let area_id = param["AreaId"].as_i64().with_context(|| format!("Expected 'AreaId' but not found in param {param:?}"))? as u32;
    if area_id > 50 {
        bail!("AreaId: {area_id} not supported")
    }
    let manager = conn.manager().await;
    let girl_furniture = GirlRoomFurniture::new(&manager, area_id);

    let girl_id = area_id;
    let mut items = Vec::<core::Item>::new();
    let mut attrs = HashMap::<u32, u32>::new();
    let mut add_favor: u32 = 0;
    let mut new_level: u32 = 0;
    let mut add_fur_list: Vec<u32> = Vec::new();
    let mut set_girl_id: u32 = 0;

    let tb_gdpln: Vec<(u16, u16, u16, u16, u32)> = from_value(param["tbGDPLN"].clone())?;
    for gdpln in tb_gdpln {
        let dorm_gift_template_data = tables()
            .dorm_gift_template
            .find_one("gdpl", [gdpln.0, gdpln.1, gdpln.2, gdpln.3])
            .with_context(|| format!("The specified dorm_gift_template_data does not exist."))?;
        let gift_id = dorm_gift_template_data.param1;
        let gift_data = tables()
            .gift
            .find_one("gift_id_1", gift_id)
            .with_context(|| format!("The specified gift_data {gift_id} does not exist."))?;

        if gift_data.default_had.unwrap_or(0) == 0 {
            let cost_item_id = set_template(gdpln.0, gdpln.1, gdpln.2, gdpln.3)?;
            let Some(mut item) = manager.items.get_item_by_template(cost_item_id).await? else {
                bail!("Insufficient quantity for ItemId: {}", cost_item_id);
            };
            if item.count < gdpln.4 {
                bail!("Insufficient quantity for ItemId: {}", cost_item_id);
            }
            item.count = item.count - gdpln.4;
            manager.items.set(item.clone()).await?;
            items.push(item);
        }

        let furniture_attr = girl_furniture.set_by_tmp_id(gift_id, 1).await?;
        attrs.insert(furniture_attr.0, furniture_attr.1);

        let mut love_level_attr = manager.attrs.get_attr(101, girl_id * 50).await?;
        let mut love_level = GirlLoveLevel::from_value(love_level_attr.1);
        love_level.add_exp(gift_data.add_love_num * gdpln.4);
        love_level_attr.1 = love_level.as_value();
        manager.attrs.set(love_level_attr.0, love_level_attr.1).await?;
        attrs.insert(love_level_attr.0, love_level_attr.1);
        add_favor += gift_data.add_love_num * gdpln.4;
        new_level = love_level.get_level();

        let mut furniture_fold = GirlRoomFurnitureFold::from(&manager, area_id).await?;
        let attr = furniture_fold.set_by_tmp_id(gift_id, false).await?;
        attrs.insert(attr.0, attr.1);
        
        add_fur_list.push(gift_id);
        set_girl_id = gift_id;
    }

    let rsp = protocol::NtfSyncPlayer {
        items: items,
        custom: attrs,
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "AddFavor": add_favor,
            "NewLevel": new_level,
            "AddFurList": add_fur_list,
            "GirlId": area_id,
            "AreaId": area_id,
            "GiftId": set_girl_id,
            "FuncName": "GiveGiftToAreaSuccess",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn request_change_furniture_fold_by_tmp_id(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let is_fold = param["IsFold"].as_bool().with_context(|| format!("Expected 'IsFold' but not found in param {param:?}"))?;
    let tmp_id = param["TmpId"].as_i64().with_context(|| format!("Expected 'TmpId' but not found in param {param:?}"))? as u32;
    let area_id = param["AreaId"].as_i64().with_context(|| format!("Expected 'AreaId' but not found in param {param:?}"))? as u32;
    let manager = conn.manager().await;

    if area_id > 50 {
        bail!("AreaId: {area_id} not supported");
    }

    let mut attrs = HashMap::<u32, u32>::new();

    let mut furniture_fold = GirlRoomFurnitureFold::from(&manager, area_id).await?;
    let attr = furniture_fold.set_by_tmp_id(tmp_id, is_fold).await?;
    attrs.insert(attr.0, attr.1);

    let rsp = protocol::NtfSyncPlayer {
        custom: attrs,
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "IsFold": param["IsFold"],
            "AreaId": param["AreaId"],
            "TmpId": param["TmpId"],
            "FuncName": "FurnitureFoldChangeSuccess",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn request_arcade_game_enter_main_ui(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "tbUnlockGirlList": [1, 5, 13, 23, 10],
            "FuncName": "ArcadeGameEnterMainUI",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

async fn request_arcade_game_enter(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "House_Request".to_string(),
        arg: json!({
            "nSeed": 0,
            "FuncName": "ArcadeGameEnter",
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
