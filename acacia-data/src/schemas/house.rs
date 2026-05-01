use super::item::GDPLN;
use crate::manager::tsv::with_lua_table;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftExchangeData {
    pub shop_type: u32,
    pub index: u32,
    pub gift_id: u32,
    pub show_order: Option<u32>,
    #[serde(deserialize_with = "with_lua_table")]
    pub gift: [u16; 4],
    #[serde(deserialize_with = "with_lua_table")]
    pub need_items: Vec<GDPLN>,
    pub need_money: u32,
    pub need_gold: Option<u32>,
    pub sale_off_begin_time: Option<String>,
    pub sale_off_end_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftData {
    pub gift_id: u32,
    pub add_love_num: Option<u32>,
    pub default_had: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FurniturePosData {
    pub area_id: u32,
    pub group_id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub furniture_tmp_id: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupportFurnitureData {
    pub area_id: u32,
    pub index: u32,
    pub furniture_tmp_id: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HouseOpenGirlData {
    pub girl_id: u32,
}
