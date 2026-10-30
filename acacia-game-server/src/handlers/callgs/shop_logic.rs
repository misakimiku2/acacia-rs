//! 商店相关接口。
//!
//! 上游 acacia 的 `ShopLogic_GetGoodsList` 是 `goodsList: []`（WIP），
//! 导致所有游戏货币商店渲染空白 —— 客户端拿到空列表就没东西可画。
//!
//! 商品配置在 `shop/goods` 表（489 条，按 ShopId 分组）：
//!   GDPLN  = 商品内容 [genre, detail, particular, level, num]
//!   Price1 = 价格     [货币类型, ...]
//!   Price2 = 第二价格（空表示单一货币）
//!
//! 注意：客户端 Lua 在加密 pak 里，无法确认它到底读哪些字段名，
//! 所以每项同时输出 goodsId/nGoodsId/GoodsId/id 等多种变体，多打的字段客户端会忽略。

use serde_json::{Value, json};
use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

#[callgs_reg("ShopLogic_GetGoodsList")]
async fn get_goods_list(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let shop_id = param["nShopId"]
        .as_u64()
        .or_else(|| param["shopId"].as_u64())
        .or_else(|| param["nShopID"].as_u64())
        .unwrap_or(0) as u32;

    // 临时诊断：确认客户端真实传参（确认后可删）
    warn!("ShopLogic_GetGoodsList param={param}");

    let goods = tables().shop_goods.find("shop_id_1", shop_id);
    let count = goods.len();

    let list: Vec<Value> = goods
        .iter()
        .map(|g| {
            let gdpln = g.gdpln.clone().unwrap_or_default();
            let price1 = g.price1.clone().unwrap_or_default();
            let price2 = g.price2.clone().unwrap_or_default();
            let limit_type = g.limit_type.unwrap_or(0);
            let limit_num = g.limit_num.unwrap_or(0);
            let class = g.class.clone().unwrap_or_default();
            let weight = g.weight.unwrap_or(0);
            json!({
                // id：覆盖常见命名习惯
                "goodsId": g.goods_id,
                "nGoodsId": g.goods_id,
                "GoodsId": g.goods_id,
                "id": g.goods_id,
                "nId": g.goods_id,
                "nID": g.goods_id,
                // 商品内容 [g,d,p,l,n]
                "gdpln": gdpln.clone(),
                "GDPLN": gdpln.clone(),
                "tbGDPLN": gdpln.clone(),
                "item": gdpln.clone(),
                "tbItem": gdpln.clone(),
                "nItem": gdpln.clone(),
                "award": gdpln.clone(),
                "tbAward": gdpln.clone(),
                // 价格（主）
                "price1": price1.clone(),
                "Price1": price1.clone(),
                "tbPrice1": price1.clone(),
                "price": price1.clone(),
                "tbPrice": price1.clone(),
                "cost": price1.clone(),
                "tbCost": price1.clone(),
                "nCost": price1.clone(),
                // 价格（次）
                "price2": price2.clone(),
                "Price2": price2.clone(),
                "tbPrice2": price2.clone(),
                "cost2": price2.clone(),
                // 限购
                "limitType": limit_type,
                "LimitType": limit_type,
                "nLimitType": limit_type,
                "limitNum": limit_num,
                "LimitNum": limit_num,
                "nLimitNum": limit_num,
                // 其他
                "class": class.clone(),
                "Class": class,
                "weight": weight,
                "Weight": weight,
            })
        })
        .collect();

    // 客户端对 {商品ID: 值} 的映射有反应（给数字 0 时界面会算出乱值），
    // 说明它按 map 遍历、再把 value 当对象取字段 —— 这里把 value 填成完整商品对象
    let mut goods_map = serde_json::Map::new();
    for (g, obj) in goods.iter().zip(list.iter()) {
        goods_map.insert(g.goods_id.to_string(), obj.clone());
    }

    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_GetGoodsList".to_string(),
        arg: json!({
            "shopId": shop_id,
            "nShopId": shop_id,
            // 映射形式：商品ID -> 商品对象
            "goodsList": Value::Object(goods_map),
            // 数组形式作为兜底，多个键名
            "tbGoods": list.clone(),
            "tbGoodsList": list.clone(),
            "goods": list,
        })
        .to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;

    warn!("ShopLogic_GetGoodsList: shopId={shop_id} -> {count} 个商品");
    Ok(())
}

/// 商店开放时间。MikuSB 这里返回 `{}`（而不是 `[]`），
/// 用 `[]` 可能让客户端把页签判成“未开放”。
#[callgs_reg("ShopLogic_GetOpenTime")]
async fn get_open_time(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_GetOpenTime".to_string(),
        arg: "{}".to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}
