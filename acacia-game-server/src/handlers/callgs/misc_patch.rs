//! 补齐客户端会调用、但上游未实现的 CallGS 接口。
//!
//! 背景：这些 api 没有 handler 时，服务端只在日志里 warn 一句然后什么都不回，
//! 客户端等不到响应就弹「连接失败」。这里统一给出最小可用应答。
//! 语义参考 MikuSB（work/MikuSB-main/GameServer/Server/CallGS/Handlers/Daily 与 Misc）。

use serde_json::Value;
use std::sync::Arc;

use crate::base::conn::Conn;
use acacia_derive::callgs_reg;
use acacia_game_msg::protocol;

/// 服务器时间查询。客户端 ZoneTime.lua 里 sTime1/sTime2 是写死的，
/// nTime1/nTime2 返回 false 时它会忽略这次校正，直接用本地时间（同 MikuSB）。
#[callgs_reg("ZoneTime_ReqTime")]
async fn zone_time_req_time(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!("ZoneTime_ChangeTime", {"nTime1": false, "nTime2": false});
    Ok(())
}

/// 日常活动信息：返回空表，客户端按“无进行中活动”处理。
#[callgs_reg("Daily_GetActivityInfo")]
async fn daily_get_activity_info(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

/// 进入日常（养成材料）关卡，返回随机种子。与 Role_EnterLevel 一致用 0。
#[callgs_reg("Daily_EnterLevel")]
async fn daily_enter_level(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({"nSeed": 0});
    Ok(())
}

/// 日常关卡自选套装，原样回传客户端提交的 SuitId。
#[callgs_reg("Daily_SetSelectSuit")]
async fn daily_set_select_suit(conn: Arc<Conn>, _: protocol::ReqCallGs, param: Value) -> anyhow::Result<()> {
    call_script!({"SuitId": param["Suit"].clone()});
    Ok(())
}

/// 活动封面刷新：无活动数据可下发。
#[callgs_reg("ActivityFace_Update")]
async fn activity_face_update(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

/// 在线活动请求：无活动数据可下发。
#[callgs_reg("OnlineActivity_Request")]
async fn online_activity_request(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}

/// 成就上报：客户端单向通知，`s2c` 表里没有这个命令。
/// 回包会触发客户端 `s2c.Dispatch error. No command named`，所以这里保持沉默。
#[callgs_reg("Achievement_OnAddSomeOne")]
async fn achievement_on_add_some_one(_: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    Ok(())
}

/// 预约奖励领取：同样是单向通知，客户端没有对应的 s2c 命令，不回包。
#[callgs_reg("ReservationReward_Gain")]
async fn reservation_reward_gain(_: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    Ok(())
}

/// 联机突击战令刷新：无战令数据，回空避免客户端卡住。
#[callgs_reg("OnlineAssaultOpsBattlePass_ClientRefresh")]
async fn online_assault_ops_bp_refresh(conn: Arc<Conn>, _: protocol::ReqCallGs, _: Value) -> anyhow::Result<()> {
    call_script!({});
    Ok(())
}
