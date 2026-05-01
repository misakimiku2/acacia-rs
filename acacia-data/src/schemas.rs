use crate::manager::tsv::{with_datetime, with_datetime_option, with_gdpl, with_lua_table};
use serde::Deserialize;

pub type GDPL = (u16, u16, u16, u16);
pub type GDPLN = (u16, u16, u16, u16, u32);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivityData {
    pub id: u32,
    #[serde(deserialize_with = "with_datetime")]
    pub start_time: i64,
    #[serde(deserialize_with = "with_datetime_option")]
    pub end_time: Option<i64>,
}

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
pub struct CardTemplateData {
    pub genre: u16,
    pub detail: u16,
    pub particular: u16,
    pub level: u16,
    #[serde(alias = "BreakMatID")]
    pub break_mat_id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub same_convert: Vec<GDPLN>,
    #[serde(alias = "DefaultWeaponGPDL")]
    #[serde(deserialize_with = "with_lua_table")]
    pub default_weapon_gpdl: GDPL,
    #[serde(alias = "profile")]
    #[serde(deserialize_with = "with_lua_table")]
    pub profile: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub pieces: Option<Vec<GDPLN>>,
    #[serde(alias = "SpineID")]
    pub spine_id: u32,
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
    pub evolution_gold: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub weapon_parts_award: Option<GDPL>,
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemSpineData {
    #[serde(alias = "ID")]
    pub id: u32,
    pub node1_req: u32,
    pub node2_req: u32,
    pub node3_req: u32,
    pub node4_req: u32,
    pub node5_req: u32,
    pub node6_req: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemNodeConditionData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(deserialize_with = "with_lua_table")]
    pub node1_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node1_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node2_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node2_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node3_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node3_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node4_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node4_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node5_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node5_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node6_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node6_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node7_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node7_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node8_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node8_condition: Vec<(u32, u32)>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node9_cost: Vec<GDPLN>,
    #[serde(deserialize_with = "with_lua_table")]
    pub node9_condition: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CardBreakData {
    #[serde(alias = "ID")]
    pub id: u32,

    #[serde(alias = "1Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_1: Vec<GDPLN>,
    #[serde(alias = "1Items2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_2: Vec<GDPLN>,
    #[serde(alias = "1Items3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_3: Vec<GDPLN>,
    #[serde(alias = "1Items4")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_4: Vec<GDPLN>,
    #[serde(alias = "1Items5")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_5: Vec<GDPLN>,
    #[serde(alias = "1Items6")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_6: Vec<GDPLN>,
    #[serde(alias = "1Items7")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_7: Vec<GDPLN>,
    #[serde(alias = "1Items8")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_8: Vec<GDPLN>,
    #[serde(alias = "1Items9")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_9: Vec<GDPLN>,

    #[serde(alias = "2Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_1: Vec<GDPLN>,
    #[serde(alias = "2Items2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_2: Vec<GDPLN>,
    #[serde(alias = "2Items3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_3: Vec<GDPLN>,
    #[serde(alias = "2Items4")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_4: Vec<GDPLN>,
    #[serde(alias = "2Items5")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_5: Vec<GDPLN>,
    #[serde(alias = "2Items6")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_6: Vec<GDPLN>,
    #[serde(alias = "2Items7")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_7: Vec<GDPLN>,
    #[serde(alias = "2Items8")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_8: Vec<GDPLN>,
    #[serde(alias = "2Items9")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_9: Vec<GDPLN>,

    #[serde(alias = "3Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_1: Vec<GDPLN>,
    #[serde(alias = "3Items2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_2: Vec<GDPLN>,
    #[serde(alias = "3Items3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_3: Vec<GDPLN>,
    #[serde(alias = "3Items4")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_4: Vec<GDPLN>,
    #[serde(alias = "3Items5")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_5: Vec<GDPLN>,
    #[serde(alias = "3Items6")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_6: Vec<GDPLN>,
    #[serde(alias = "3Items7")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_7: Vec<GDPLN>,
    #[serde(alias = "3Items8")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_8: Vec<GDPLN>,
    #[serde(alias = "3Items9")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_9: Vec<GDPLN>,

    #[serde(alias = "4Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_1: Vec<GDPLN>,
    #[serde(alias = "4Items2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_2: Vec<GDPLN>,
    #[serde(alias = "4Items3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_3: Vec<GDPLN>,
    #[serde(alias = "4Items4")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_4: Vec<GDPLN>,
    #[serde(alias = "4Items5")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_5: Vec<GDPLN>,
    #[serde(alias = "4Items6")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_6: Vec<GDPLN>,
    #[serde(alias = "4Items7")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_7: Vec<GDPLN>,
    #[serde(alias = "4Items8")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_8: Vec<GDPLN>,
    #[serde(alias = "4Items9")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_9: Vec<GDPLN>,

    #[serde(alias = "5Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_1: Vec<GDPLN>,
    #[serde(alias = "5Items2")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_2: Vec<GDPLN>,
    #[serde(alias = "5Items3")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_3: Vec<GDPLN>,
    #[serde(alias = "5Items4")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_4: Vec<GDPLN>,
    #[serde(alias = "5Items5")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_5: Vec<GDPLN>,
    #[serde(alias = "5Items6")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_6: Vec<GDPLN>,
    #[serde(alias = "5Items7")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_7: Vec<GDPLN>,
    #[serde(alias = "5Items8")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_8: Vec<GDPLN>,
    #[serde(alias = "5Items9")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items5_9: Vec<GDPLN>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CardBreakNewData {
    #[serde(alias = "ID")]
    pub id: u32,
    #[serde(alias = "1Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items1_1: Vec<GDPLN>,
    #[serde(alias = "2Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items2_1: Vec<GDPLN>,
    #[serde(alias = "3Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items3_1: Vec<GDPLN>,
    #[serde(alias = "4Items1")]
    #[serde(deserialize_with = "with_lua_table")]
    pub items4_1: Vec<GDPLN>,
}
