use serde_json::{Value, json};
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("ShopLogic_GetGoodsList")]
async fn check_open_act(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
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
