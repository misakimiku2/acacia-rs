use super::item::{GDPL, GDPLN};
use crate::manager::tsv::{with_datetime_option, with_lua_table};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityData {
    pub id: u32,
    pub class: String,
    #[serde(deserialize_with = "with_datetime_option")]
    pub start_time: Option<i64>,
    #[serde(deserialize_with = "with_datetime_option")]
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityGachaTryData {
    pub act_id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub try_girl: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityGachaTryGirlData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub girl: GDPL,
    #[serde(deserialize_with = "with_lua_table")]
    pub reward: Vec<GDPLN>,
}
