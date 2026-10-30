use super::item::GDPL;
use crate::manager::tsv::{with_datetime, with_gdpl, with_lua_table};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CashData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(alias = "GDPL")]
    #[serde(deserialize_with = "with_gdpl")]
    pub gdpl: Option<GDPL>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RenameData {
    pub times: u32,
    pub r#type: u32,
    pub num: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BattlepassTimelistData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_datetime")]
    pub start_time: i64,
    #[serde(deserialize_with = "with_datetime")]
    pub end_time: i64,
    #[serde(deserialize_with = "with_datetime")]
    pub buy_start_time: i64,
    #[serde(deserialize_with = "with_datetime")]
    pub buy_end_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChapterData {
    #[serde(alias = "ID")]
    pub id: u32,
    pub difficult: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub level: Option<Vec<u32>>,
    /// StarAward 形如 {{9,{{4,3,1,1,3000}}},{18,{{...}}}}，结构不规则，
    /// 这里保留原文由 handler 解析，避免单条数据解析失败导致整行被丢弃
    #[serde(alias = "StarAward")]
    pub star_award: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChapterLevelData {
    #[serde(alias = "ID")]
    pub id: u32,
    pub r#type: u32,
    pub player_exp: Option<u32>,
    pub role_exp: Option<u32>,
    #[serde(alias = "FirstDropID")]
    #[serde(deserialize_with = "with_lua_table")]
    pub first_drop_id: Option<Vec<u32>>,
    #[serde(alias = "BaseDropID")]
    #[serde(deserialize_with = "with_lua_table")]
    pub base_drop_id: Option<Vec<u32>>,
    #[serde(alias = "RandomDropID")]
    #[serde(deserialize_with = "with_lua_table")]
    pub random_drop_id: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DropData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub drop: Vec<[u32; 3]>,
}

/// 悖论迷宫（Rogue3D）难度配置：DifficultyId -> GameplayGroup
/// 客户端进图时用 GameplayGroup[0] 当 gameplay 配置的 group 去查表，
/// 不写这个属性就会报「找不到gameplay配置，group=0」并渲染空白场景
#[derive(Debug, Clone, Deserialize)]
pub struct Rogue3dDifficultData {
    #[serde(rename = "DifficultID")]
    pub difficult_id: u32,
    #[serde(rename = "GameplayGroup")]
    #[serde(deserialize_with = "with_lua_table")]
    pub gameplay_group: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DropGropData {
    #[serde(alias = "ID")]
    pub id: u32,
    pub rand_type: Option<u32>,
    pub grop: String,
}

/// 爬塔（地下清理）周期表：每个赛季周期对应一批关卡组
#[derive(Debug, Clone, Deserialize)]
pub struct ClimbTowerTimeData {
    #[serde(rename = "ID")]
    pub id: u32,
    /// 形如 "[202112010400]"，需去括号后按 %Y%m%d%H%M 解析
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,
    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,
    #[serde(rename = "Level1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub level1: Option<Vec<Vec<u32>>>,
    #[serde(rename = "Level2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub level2: Option<Vec<Vec<u32>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClimbTowerDiffData {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Level1")]
    pub level1: Option<u32>,
    #[serde(rename = "Level2")]
    pub level2: Option<u32>,
}

/// 爬塔层奖励。Diff 为空表示通用（不区分难度）。
#[derive(Debug, Clone, Deserialize)]
pub struct ClimbTowerAwardData {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Diff")]
    pub diff: Option<String>,
    #[serde(rename = "FirstAward")]
    #[serde(deserialize_with = "with_lua_table")]
    pub first_award: Option<Vec<Vec<u32>>>,
    #[serde(rename = "StarCount1")]
    pub star_count1: Option<u32>,
    #[serde(rename = "StarAward1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub star_award1: Option<Vec<Vec<u32>>>,
    #[serde(rename = "StarCount2")]
    pub star_count2: Option<u32>,
    #[serde(rename = "StarAward2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub star_award2: Option<Vec<Vec<u32>>>,
    #[serde(rename = "StarCount3")]
    pub star_count3: Option<u32>,
    #[serde(rename = "StarAward3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub star_award3: Option<Vec<Vec<u32>>>,
}

/// 爬塔关卡序号表：TowerID -> 实际关卡 ID
#[derive(Debug, Clone, Deserialize)]
pub struct ClimbTowerLevelOrderData {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "LevelID")]
    pub level_id: Option<u32>,
}

/// 商店商品（游戏货币商店，非氪金）
/// GDPLN = 商品内容 [genre, detail, particular, level, num]
/// Price1/Price2 = 价格 [货币类型, ...]，Price2 为空表示只有一种货币
#[derive(Debug, Clone, Deserialize)]
pub struct ShopGoodsData {
    #[serde(rename = "ShopId")]
    pub shop_id: u32,
    #[serde(rename = "GoodsId")]
    pub goods_id: u32,
    #[serde(rename = "GDPLN")]
    #[serde(deserialize_with = "with_lua_table")]
    pub gdpln: Option<Vec<u32>>,
    #[serde(rename = "Price1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub price1: Option<Vec<u32>>,
    #[serde(rename = "Price2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub price2: Option<Vec<u32>>,
    #[serde(rename = "LimitType")]
    pub limit_type: Option<u32>,
    #[serde(rename = "LimitNum")]
    pub limit_num: Option<u32>,
    #[serde(rename = "Weight")]
    pub weight: Option<u32>,
    #[serde(rename = "Class")]
    pub class: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RikiData {
    pub r#type: u32,
    pub id: u32,
    pub condition: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BosspvpBossChallengeData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_datetime")]
    pub start_time: i64,
    #[serde(deserialize_with = "with_datetime")]
    pub end_time: i64,
}

