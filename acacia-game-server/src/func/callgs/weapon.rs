use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::base::conn::Conn;
use crate::logic::weapon::{weapon_break_up, weapon_level_up};
use acacia_derive::callgs_reg;
use acacia_game_msg::{core, protocol, protocol::Pf};

#[derive(Debug, Clone, Deserialize)]
struct BreakUpgradeStage(u32, Vec<(u64, u32)>, u8);

#[callgs_reg("Weapon_OneKeyToMax")]
async fn one_key_to_max(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let weapon_id = param["Id"].as_u64().with_context(|| format!("Expected 'Id' but not found in param {:?}", param))?;
    let break_upgrade_mat: Vec<BreakUpgradeStage> = serde_json::from_value(param["tbBreakUpgradeMat"].clone())?;

    let mut me = conn.manager().await;
    let mut weapon = me.get_item(weapon_id).await?.with_context(|| format!("Weapon not found {}", weapon_id))?;
    let weapon_before_level = weapon.enhance.as_ref().map_or(0, |e| e.level);

    let mut cached_level_mat: HashMap<u64, u32> = HashMap::new();
    let mut cached_break_mat: HashMap<(u16, u16, u16, u16), u32> = HashMap::new();
    let mut total_consume_gold: u32 = 0;
    for BreakUpgradeStage(target_break, stage_mat, break_flag) in break_upgrade_mat {
        let current_break = weapon.enhance.as_ref().map_or(0, |e| e.r#break);
        if target_break < current_break {
            conn.send_err("error.weapon_break_cmd_err").await?;
            return Ok(());
        }

        let stage_consume_gold = weapon_level_up(&me, &mut weapon, &stage_mat).await?;
        total_consume_gold = total_consume_gold.checked_add(stage_consume_gold).context("Gold overflow")?;
        for m in stage_mat {
            *cached_level_mat.entry(m.0).or_insert(0) += m.1;
        }

        if break_flag != 1 {
            continue;
        }
        let (stage_consume_gold, stage_break_mat) = weapon_break_up(&me, &mut weapon).await?;
        total_consume_gold = total_consume_gold.checked_add(stage_consume_gold).context("Gold overflow")?;
        for gdpln in stage_break_mat {
            let gdpl = (gdpln.0, gdpln.1, gdpln.2, gdpln.3);
            *cached_break_mat.entry(gdpl).or_insert(0) += gdpln.4;
        }
    }

    let mut cached_items = Vec::<core::Item>::new();
    for (mat_id, mat_num) in &cached_level_mat {
        let Some(mut item) = me.get_item(*mat_id).await? else {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        };
        if item.count < *mat_num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
        item.count -= mat_num;
        cached_items.push(item)
    }

    for (mat_gdpl, mat_num) in &cached_break_mat {
        let item_count = me.get_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3).await?;
        if item_count < *mat_num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
    }

    let mut current_gold = me.get_money(3).await?;
    if current_gold < total_consume_gold {
        conn.send_err("error.gold_not_enough").await?;
        return Ok(());
    }
    current_gold -= total_consume_gold;

    me.set_money(3, current_gold).await?;
    for item in cached_items {
        me.set_item(item).await?;
    }
    for (mat_gdpl, mat_num) in &cached_break_mat {
        me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
    }
    let weapon_after_level = weapon.enhance.as_ref().map_or(0, |e| e.level);
    let b_max_unlock = weapon_before_level < 50 && weapon_after_level >= 50;
    me.set_item(weapon).await?;
    conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "Weapon_OneKeyToMax".to_string(),
        arg: serde_json::to_string(&serde_json::json!({
            "bMaxUnLock": b_max_unlock,
        }))?,
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("Weapon_ReplacePart")]
async fn replace_part(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let database = conn.database().await;
    let slot_id = param["Type"].as_i64().with_context(|| format!("Expected 'Type' but not found in param {:?}", param))? as u32;
    let part_id = param["PartId"].as_i64().with_context(|| format!("Expected 'PartId' but not found in param {:?}", param))?;
    let weapon_id = param["Id"].as_u64().with_context(|| format!("Expected 'Id' but not found in param {:?}", param))?;

    let mut weapon = database.items.find_one(weapon_id).await?.with_context(|| format!("Weapon not found {}", weapon_id))?;
    if part_id == -1 {
        weapon.slots.remove(&slot_id);
    } else {
        weapon.slots.insert(slot_id, part_id as u64);
    }
    database.items.update_one(weapon.clone()).await?;

    let rsp = protocol::NtfSyncPlayer {
        items: vec![weapon],
        ..protocol::NtfSyncPlayer::default()
    };
    conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;

    let rsp = protocol::NtfCallScript {
        api: "Weapon_ReplacePart".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
