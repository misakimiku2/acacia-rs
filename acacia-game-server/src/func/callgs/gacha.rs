use anyhow::Context;
use serde_json::{Value, json};
use std::{sync::Arc, vec};

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("Gacha_Launch")]
async fn change(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let nid = param["nId"].as_i64().with_context(|| format!("Gacha_Launch: nId not found in param {:?}", param))?;
    let ntime = param["nTime"].as_u64().with_context(|| format!("Gacha_Launch: nTime not found in param {:?}", param))?;
    let tb_award = match nid {
        79 | 179 => vec![1, 8, 4, 1],
        80 | 180 => vec![2, 1, 33, 1],
        81 | 181 => vec![1, 13, 3, 1],
        82 | 182 => vec![2, 4, 32, 1],
        91 | 191 => vec![1, 1, 3, 1],
        92 | 192 => vec![2, 10, 3, 1],
        93 | 193 => vec![1, 5, 4, 1],
        94 | 194 => vec![2, 1, 35, 1],
        95 | 195 => vec![1, 1, 2, 1],
        96 | 196 => vec![2, 4, 18, 1],
        97 | 197 => vec![1, 7, 2, 1],
        98 | 198 => vec![2, 1, 27, 1],
        _ => vec![1, 1, 3, 1],
    };
    let tb_awards = vec![tb_award; ntime as usize];
    let tb_trigger = vec![true; ntime as usize];
    let tb_new: Vec<u64> = (1..=ntime).collect();
    let rsp = protocol::NtfCallScript {
        api: "Gacha_Launch".to_string(),
        arg: json!({
            "tbAwards": tb_awards,
            "nBoxCount": 0,
            "tbTrigger": tb_trigger,
            "nId": param["nId"],
            "tbNew": tb_new,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
