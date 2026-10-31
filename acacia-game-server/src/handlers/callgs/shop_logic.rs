//! 商店相关接口。
//!
//! 2026-10-30 ~ 31 由客户端字节码确认（反汇编见 `work/HANDOFF-shop-reverse.md`）：
//!
//! * **列表**：响应里 `goodsList` 的每个元素**只需要 `goodsid`（小写、数字）**，
//!   展示字段全部由客户端 `ShopLogic.tbGoodsConf` 本地提供（`LoadGoodsConf` 建表）。
//! * **能否购买**：不看响应，看**玩家存档属性**
//!   `(GID=1, shopId)` → `{"listversion":n,"tbgoods":[{"goodsid":n,"discountindex":0}]}`；
//!   `listversion` 缺了整段会被丢弃（`GetShopData` 里有 nil 判断）。
//! * **已购次数**：`(GID=1, 1000+goodsid)` → `{"buynum":n}`。
//! * **刷新次数**：同一个商店 JSON 里的 `refreshnum`（`GetShopRefreshNnm`）。
//!
//! 私服定位：商品与货币都无限，所以刷新不扣费、也不累计 `refreshnum`
//! （恒为 0 → 界面显示 0/5，刷新不限次）。

use serde_json::{Value, json};
use std::sync::Arc;
use tracing::{info, warn};

use crate::base::conn::Conn;
use crate::logic::player::PlayerManager;
use acacia_data::tables::tables;
use acacia_derive::callgs_reg;
use acacia_game_msg::{protocol, protocol::Pf};

/// 商店存档属性所在的组号（探针实测 `ShopLogic.GID`）
const SHOP_GID: u32 = 1;
/// 每商品属性 id 的基址（探针实测 `ShopLogic.GoodsStart`）
const GOODS_START: u32 = 1000;

/// 某个商店当前该显示的 `<goodsid>` 列表。
/// 私服不做随机库存，就是配置表里该 ShopId 的全部商品。
fn shop_goods_ids(shop_id: u32) -> Vec<u32> {
    tables()
        .shop_goods
        .find("shop_id_1", shop_id)
        .iter()
        .map(|g| g.goods_id)
        .collect()
}

/// 下发商店存档属性（客户端「能不能买」「已购几次」「刷新几次」都读这里）
async fn push_shop_attrs(
    me: &mut PlayerManager,
    shop_id: u32,
    ids: &[u32],
) -> anyhow::Result<()> {
    // 保留已有 refreshnum（私服里恒为 0）
    let prev = me
        .get_str_attribute(SHOP_GID, shop_id)
        .await
        .unwrap_or_default();
    let refreshnum = serde_json::from_str::<Value>(&prev)
        .ok()
        .and_then(|v| v["refreshnum"].as_u64())
        .unwrap_or(0);

    let tbgoods: Vec<Value> = ids
        .iter()
        .map(|id| json!({ "goodsid": id, "discountindex": 0 }))
        .collect();

    me.set_str_attribute(
        SHOP_GID,
        shop_id,
        json!({ "listversion": 1, "refreshnum": refreshnum, "tbgoods": tbgoods }).to_string(),
    )
    .await?;

    // 每商品已购次数：只补缺，别每次开商店把 buynum 清零
    for id in ids {
        let key = GOODS_START + id;
        if me.get_str_attribute(SHOP_GID, key).await?.is_empty() {
            me.set_str_attribute(SHOP_GID, key, json!({ "buynum": 0 }).to_string())
                .await?;
        }
    }
    Ok(())
}

/// 发「商品列表」通知。刷新后也复用它，让界面直接重画。
async fn send_goods_list(conn: &Arc<Conn>, shop_id: u32, ids: &[u32]) -> anyhow::Result<()> {
    let list: Vec<Value> = ids.iter().map(|id| json!({ "goodsid": id })).collect();
    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_GetGoodsList".to_string(),
        arg: json!({ "shopId": shop_id, "goodsList": list }).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
    Ok(())
}

#[callgs_reg("ShopLogic_GetGoodsList")]
async fn get_goods_list(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let shop_id = param["nShopId"]
        .as_u64()
        .or_else(|| param["shopId"].as_u64())
        .unwrap_or(0) as u32;

    let ids = shop_goods_ids(shop_id);
    let count = ids.len();

    {
        let mut me = conn.manager().await;
        push_shop_attrs(&mut me, shop_id, &ids).await?;
        me.sync().await?;
    }

    send_goods_list(&conn, shop_id, &ids).await?;
    info!("ShopLogic_GetGoodsList: shopId={shop_id} -> {count} 个商品");
    Ok(())
}

/// 刷新商店。
///
/// 客户端（`ShopLogic.UpdateShop`，行 845-887）先本地检查刷新次数
/// `GetShopRefreshNnm < nRefreshLimits`、货币够不够 `tbRefreshFee`，弹确认框，
/// 确认后才发 `ShopLogic_UpdateShop {nShopId}`。
///
/// 私服处理：不扣费、`refreshnum` 保持 0（= 不限次），重下属性 + 重发列表，
/// 界面直接重画。官方语义里这里是「重掷一批商品」，我们没有随机库存的概念。
#[callgs_reg("ShopLogic_UpdateShop")]
async fn update_shop(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let shop_id = param["nShopId"]
        .as_u64()
        .or_else(|| param["shopId"].as_u64())
        .unwrap_or(0) as u32;
    if shop_id == 0 {
        warn!("ShopLogic_UpdateShop: 缺少 nShopId, param={param}");
        return Ok(());
    }

    let ids = shop_goods_ids(shop_id);
    {
        let mut me = conn.manager().await;
        push_shop_attrs(&mut me, shop_id, &ids).await?;
        me.sync().await?;
    }

    // 刷新后让界面重画：复用 GetGoodsList 的通知
    send_goods_list(&conn, shop_id, &ids).await?;

    // 回一个同名的，万一客户端有对应处理
    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_UpdateShop".to_string(),
        arg: json!({ "nShopId": shop_id }).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;

    info!(
        "ShopLogic_UpdateShop: shopId={shop_id} 已重发列表（{} 个商品）",
        ids.len()
    );
    Ok(())
}

/// 购买。
///
/// 客户端（`ShopLogic.BuyGoods`，行 690-746）在发请求前已完成全部本地校验
/// （限购 `nLimitType/nLimitNum`、价格 `GetBuyPrice`、余额 `Cash:GetMoneyCount`），
/// 然后 `me:CallGS("ShopLogic_BuyGoods", {nGoodsId, nBuyMode, nCount})`。
///
/// * 价格：`Price1 = {现金表ID, 单价}`。`cash.txt` 的 `ID` 就是第一个数
///   （1=MONEY / 2=GOLD / 3=SILVER），每行自带 `GDPL`，直接喂 `remove_item_count`。
/// * 商品：`GDPLN = {genre, detail, particular, level, num}`，`num` 是单次数量。
#[callgs_reg("ShopLogic_BuyGoods")]
async fn buy_goods(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    let goods_id = param["nGoodsId"].as_u64().unwrap_or(0) as u32;
    let count = param["nCount"].as_u64().unwrap_or(1).max(1) as u32;
    if goods_id == 0 {
        warn!("ShopLogic_BuyGoods: 缺少 nGoodsId, param={param}");
        return Ok(());
    }

    let cfg = tables()
        .shop_goods
        .all()
        .iter()
        .find(|g| g.goods_id == goods_id)
        .cloned();

    let Some(cfg) = cfg else {
        warn!("ShopLogic_BuyGoods: 商品 {goods_id} 不在 shop_goods 表里");
        return Ok(());
    };

    let gdpln = cfg.gdpln.clone().unwrap_or_default();
    let price = cfg.price1.clone().unwrap_or_default();
    if gdpln.len() < 5 || price.len() < 2 {
        warn!("ShopLogic_BuyGoods: 商品 {goods_id} 的 GDPLN/Price1 不完整 gdpln={gdpln:?} price={price:?}");
        return Ok(());
    }

    let (g, d, p, l) = (
        gdpln[0] as u16,
        gdpln[1] as u16,
        gdpln[2] as u16,
        gdpln[3] as u16,
    );
    let give = gdpln[4].saturating_mul(count);

    let cash_id = price[0];
    let unit = price[1];
    let total = unit.saturating_mul(count);
    let cash_gdpl = tables()
        .cash
        .all()
        .iter()
        .find(|c| c.id == cash_id)
        .and_then(|c| c.gdpl);

    let mut me = conn.manager().await;

    if let Some((cg, cd, cp, cl)) = cash_gdpl {
        if let Err(e) = me.remove_item_count(cg, cd, cp, cl, total).await {
            warn!("ShopLogic_BuyGoods: 商品 {goods_id} 扣费失败 {e}（需要 {total}）");
            return Ok(());
        }
    } else {
        warn!("ShopLogic_BuyGoods: 价格里的现金 ID {cash_id} 在 cash 表里找不到，跳过扣费");
    }

    me.add_item_count(g, d, p, l, give).await?;

    // 已购次数（客户端 GetBuyNum 读的就是这个）
    let key = GOODS_START + goods_id;
    let cur = me.get_str_attribute(SHOP_GID, key).await.unwrap_or_default();
    let buynum = serde_json::from_str::<Value>(&cur)
        .ok()
        .and_then(|v| v["buynum"].as_u64())
        .unwrap_or(0) as u32;
    me.set_str_attribute(
        SHOP_GID,
        key,
        json!({ "buynum": buynum.saturating_add(count) }).to_string(),
    )
    .await?;

    me.sync().await?; // sync 会消费 me，顺带释放锁

    info!("ShopLogic_BuyGoods: 商品 {goods_id} x{count} -> 发 {g},{d},{p},{l} x{give}，花费 {total}");

    let rsp = protocol::NtfCallScript {
        api: "ShopLogic_BuyGoods".to_string(),
        arg: json!({ "nGoodsId": goods_id, "nCount": count }).to_string(),
        ..protocol::NtfCallScript::default()
    };
    conn.send_msg(Pf::NtfScript, &rsp, 0).await?;
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
