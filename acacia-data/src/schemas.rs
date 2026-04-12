use crate::manager::tsv::with_lua_table;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftExchangeData {
    pub shop_type: u32,
    pub index: u32,
    pub gift_id: u32,
    pub show_order: Option<u32>,

    #[serde(deserialize_with = "with_lua_table")]
    pub gift: Vec<u32>,
}
