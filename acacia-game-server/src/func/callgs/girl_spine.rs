use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::{base::conn::Conn, logic::girl_spine::girl_spine_unlock};
use acacia_commons::bits::get_template;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[derive(Debug, Default, Serialize, Deserialize)]
struct SpineConfig {
    ns: u64,
    tbn: (u32, u32),
    tbr: Vec<i32>,
}

#[callgs_reg("GirlSpine_ChildUnLock")]
async fn child_un_lock(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(girl_id) = param["pId"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'pId' but not found in param {:?}", param);
    };
    let Some(mast_idx) = param["tbInfo"]["Indx"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'tbInfo.Indx' but not found in param {:?}", param);
    };
    let Some(sub_idx) = param["tbInfo"]["InSubIdx"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'tbInfo.InSubIdx' but not found in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut girl) = me.get_item(girl_id).await? else {
        conn.send_err("error.card_not_found").await?;
        bail!("Girl not found {}", girl_id);
    };
    let girl_gdpl = get_template(girl.template);
    let mat = match girl_spine_unlock(&mut girl, mast_idx as usize, sub_idx as usize).await {
        Ok(v) => v,
        Err(e) => {
            conn.send_err("error.girlcard_update_failed").await?;
            return Err(e);
        }
    };
    for (mat_gdpl, mat_num) in &mat {
        let item_count = me.get_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3).await?;
        if item_count < *mat_num {
            conn.send_err("error.mat_not_found").await?;
            return Ok(());
        }
    }

    let mut configs: Vec<Option<SpineConfig>> = match me.get_str_attribute(30, girl_gdpl.1 as u32).await?.trim() {
        "" => Vec::new(),
        s => serde_json::from_str(s)?,
    };
    configs.resize_with(girl_gdpl.2 as usize, || None);
    let config = configs[girl_gdpl.2 as usize - 1].get_or_insert_with(SpineConfig::default);
    let ns = match sub_idx {
        9 => 0,
        _ => mast_idx,
    };
    if config.ns != ns {
        config.ns = ns;
        me.set_str_attribute(30, girl_gdpl.1 as u32, serde_json::to_string(&configs)?).await?;
    }

    for (mat_gdpl, mat_num) in &mat {
        me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
    }
    me.set_item(girl).await?;
    me.sync().await?;

    conn.send_script("GirlSpine_ChildUnLock", json!({"tb": {"D": girl_gdpl.1, "pId": girl_id, "MastIdx": mast_idx, "SubIdx": sub_idx}}))
        .await?;
    Ok(())
}

#[callgs_reg("GirlSpine_UnlockNodeOneKey")]
async fn unlock_node_one_key(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Some(girl_id) = param["pId"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'pId' but not found in param {:?}", param);
    };
    let Some(mast_idx) = param["nIdx"].as_u64() else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nIdx' but not found in param {:?}", param);
    };
    let sub_idxs: Vec<u64> = match serde_json::from_value(param["tbOneKey"].clone()) {
        Ok(v) => v,
        Err(_) => {
            conn.send_err("error.BadParam").await?;
            bail!("Expected 'tbOneKey' but not found in param {:?}", param);
        }
    };

    for sub_idx in sub_idxs {
        let mut me = conn.manager().await;
        let Some(mut girl) = me.get_item(girl_id).await? else {
            conn.send_err("error.card_not_found").await?;
            bail!("Girl not found {}", girl_id);
        };
        let girl_gdpl = get_template(girl.template);

        let mat = match girl_spine_unlock(&mut girl, mast_idx as usize, sub_idx as usize).await {
            Ok(v) => v,
            Err(e) => {
                conn.send_err("error.girlcard_update_failed").await?;
                return Err(e);
            }
        };
        for (mat_gdpl, mat_num) in &mat {
            let item_count = me.get_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3).await?;
            if item_count < *mat_num {
                conn.send_err("error.mat_not_found").await?;
                return Ok(());
            }
        }

        let mut configs: Vec<Option<SpineConfig>> = match me.get_str_attribute(30, girl_gdpl.1 as u32).await?.trim() {
            "" => Vec::new(),
            s => serde_json::from_str(s)?,
        };
        configs.resize_with(girl_gdpl.2 as usize, || None);
        let config = configs[girl_gdpl.2 as usize - 1].get_or_insert_with(SpineConfig::default);
        let ns = match sub_idx {
            9 => 0,
            _ => mast_idx,
        };
        if config.ns != ns {
            config.ns = ns;
            me.set_str_attribute(30, girl_gdpl.1 as u32, serde_json::to_string(&configs)?).await?;
        }

        for (mat_gdpl, mat_num) in &mat {
            me.remove_item_count(mat_gdpl.0, mat_gdpl.1, mat_gdpl.2, mat_gdpl.3, *mat_num).await?;
        }
        me.set_item(girl).await?;
        me.sync().await?;

        conn.send_script("GirlSpine_ChildUnLock", json!({"tb": {"D": girl_gdpl.1, "pId": girl_id, "MastIdx": mast_idx, "SubIdx": sub_idx}}))
            .await?;
    }
    Ok(())
}
