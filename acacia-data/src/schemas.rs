use crate::manager::tsv::{with_datetime, with_gdpl, with_lua_table};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GDPL(pub u16, pub u16, pub u16, pub u16);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GDPLN(pub u16, pub u16, pub u16, pub u16, pub u32);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CashData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(alias = "GDPL")]
    #[serde(deserialize_with = "with_gdpl")]
    pub gdpl: Option<(u16, u16, u16, u16)>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RikiData {
    pub r#type: u32,
    pub id: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeaponTemplateData {
    pub genre: u16,
    pub detail: u16,
    pub particular: u16,
    pub level: u16,
    pub color: u32,
    #[serde(alias = "EvolutionMatID")]
    pub evolution_mat_id: u32,
    #[serde(alias = "BreakMatID")]
    pub break_mat_id: u32,
    #[serde(alias = "LevelLimitID")]
    pub level_limit_id: u32,
    #[serde(alias = "BreakLimitID")]
    pub break_limit_id: u32,
    pub provide_exp: Option<u32>,
    pub consume_gold: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupliesTemplateData {
    pub genre: u16,
    pub detail: u16,
    pub particular: u16,
    pub level: u16,
    pub provide_exp: Option<u32>,
    pub consume_gold: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DormGiftTemplateData {
    pub genre: u16,
    pub detail: u16,
    pub particular: u16,
    pub level: u16,
    pub param1: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemBreakData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub items1: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub items2: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub items3: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub items4: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub items5: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub items6: Vec<GDPLN>,
}
