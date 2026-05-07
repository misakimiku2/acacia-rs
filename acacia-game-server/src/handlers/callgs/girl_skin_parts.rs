use anyhow::bail;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_commons::bits::get_template;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

#[callgs_reg("GirlSkinParts_Update")]
async fn update(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let Ok(skin_id) = u64::deserialize(&param["nSkinId"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'nSkinId' as u32 in param {:?}", param);
    };
    let Ok(parts_id) = Vec::<u64>::deserialize(&param["tbPartsID"]) else {
        conn.send_err("error.BadParam").await?;
        bail!("Expected 'tbPartsID' as u32 in param {:?}", param);
    };

    let mut me = conn.manager().await;
    let Some(mut skin) = me.get_item(skin_id).await? else {
        conn.send_err("error.skin_not_found").await?;
        bail!("Skin not found {}", skin_id);
    };
    for part_id in parts_id {
        let Some(part) = me.get_item(part_id).await? else {
            conn.send_err("error.parts_not_found").await?;
            bail!("Parts not found {}", part_id);
        };
        let (_, d, _, _) = get_template(part.template);
        skin.slots.insert(d as u32, part_id);
    }
    me.set_item(skin).await?;
    me.sync().await?;

    call_script!({});
    Ok(())
}
