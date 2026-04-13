use crate::schemas::{self, FurniturePosData};
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
        #[path="house/gift.txt"]
        #[index("gift_id_1", gift_id)]
        pub gift: DataTable<schemas::GiftData>,

        #[path="house/gift_exchange.txt"]
        #[index("index_1", index)]
        #[index("gift_id_1", gift_id)]
        pub gift_exchange: DataTable<schemas::GiftExchangeData>,

        #[path="house/FurniturePos.txt"]
        #[index("area_id_1", area_id)]
        pub furniture_pos: DataTable<schemas::FurniturePosData>,

        #[path="item/templates/dorm_gift.txt"]
        #[index("gdpl", genre, detail, particular, level)]
        pub dorm_gift_template: DataTable<schemas::DormGiftTemplateData>,
    }
}
