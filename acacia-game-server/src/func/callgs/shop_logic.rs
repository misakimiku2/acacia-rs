use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("ShopLogic_GetGoodsList")]
async fn get_goods_list(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_GetGoodsList".to_string(),
        arg: json!({
            "shopId": param["nShopId"],
            "goodsList": [] // WIP
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("ShopLogic_GetOpenTime")]
async fn get_open_time(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_GetOpenTime".to_string(),
        arg: json!([]).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
