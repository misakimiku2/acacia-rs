use crate::schemas;
use std::sync::OnceLock;

static TABLES: OnceLock<Tables> = OnceLock::new();

pub fn tables() -> &'static Tables {
    TABLES.get().expect("Failed to use tables")
}

pub fn load_tables() -> anyhow::Result<()> {
    let tables = Tables::new()?;
    TABLES.set(tables).expect("Failed to load tables");
    Ok(())
}

crate::define_tables! {
    pub struct Tables {
        #[path="activity/activities.txt"]
        #[index("id_1", id)]
        pub activity: DataTable<schemas::ActivityData>,

        #[path="player/rename.txt"]
        #[index("times_1", times)]
        pub rename: DataTable<schemas::RenameData>,

        #[path="cash/cash.txt"]
        #[index("gdpl_1", gdpl)]
        pub cash: DataTable<schemas::CashData>,

        #[path="battlepass/timelist.txt"]
        #[index("id_1", id)]
        pub battlepass_timelist: DataTable<schemas::BattlepassTimelistData>,

        #[path="chapter/level.txt"]
        #[index("id_1", id)]
        pub chapter_level: DataTable<schemas::ChapterLevelData>,

        #[path="drop/drop.txt"]
        #[index("id_1", id)]
        pub drop: DataTable<schemas::DropData>,

        #[path="drop/drop_grop.txt"]
        #[index("id_1", id)]
        pub drop_grop: DataTable<schemas::DropGropData>,

        #[path="house/gift.txt"]
        #[index("gift_id_1", gift_id)]
        pub gift: DataTable<schemas::GiftData>,

        #[path="house/gift_exchange.txt"]
        #[index("index_1", index)]
        #[index("gift_id_1", gift_id)]
        pub gift_exchange: DataTable<schemas::GiftExchangeData>,

        #[path="house/FurniturePos.txt"]
        #[index("area_id_1", area_id)]
        #[index("area_id_group_id_1", area_id, group_id)]
        pub furniture_pos: DataTable<schemas::FurniturePosData>,

        #[path="house/support_furniture.txt"]
        #[index("area_id_1", area_id)]
        #[index("area_id_index_1", area_id, index)]
        #[index("furniture_tmp_id_1", furniture_tmp_id)]
        pub support_furniture: DataTable<schemas::SupportFurnitureData>,

        #[path="house/open_girl.txt"]
        pub house_open_girl: DataTable<schemas::HouseOpenGirlData>,

        #[path="item/templates/card.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub card_template: DataTable<schemas::CardTemplateData>,

        #[path="item/templates/weapon.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub weapon_template: DataTable<schemas::WeaponTemplateData>,

        #[path="item/templates/suplies.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub suplies_template: DataTable<schemas::SupliesTemplateData>,

        #[path="item/templates/dorm_gift.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub dorm_gift_template: DataTable<schemas::DormGiftTemplateData>,

        #[path="riki/Riki.txt"]
        pub riki: DataTable<schemas::RikiData>,

        #[path="item/break.txt"]
        #[index("id_1", id)]
        pub item_break: DataTable<schemas::ItemBreakData>,

        #[path="item/skill/spine.txt"]
        #[index("id_1", id)]
        pub item_spine: DataTable<schemas::ItemSpineData>,

        #[path="item/skill/nodecondition.txt"]
        #[index("id_1", id)]
        pub item_node_condition: DataTable<schemas::ItemNodeConditionData>,

        #[path="item/cardbreak/break.txt"]
        #[index("id_1", id)]
        pub card_break: DataTable<schemas::CardBreakData>,

        #[path="item/cardbreak/breaknew.txt"]
        #[index("id_1", id)]
        pub card_break_new: DataTable<schemas::CardBreakNewData>,
    }
}
