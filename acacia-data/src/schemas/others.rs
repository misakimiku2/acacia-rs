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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DropGropData {
    #[serde(alias = "ID")]
    pub id: u32,
    pub rand_type: Option<u32>,
    pub grop: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RikiData {
    pub r#type: u32,
    pub id: u32,
    pub condition: String,
}
