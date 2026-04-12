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
        #[path="house/gift_exchange.txt"]
        #[index("index_1", index)]
        #[index("gift_id_1", gift_id)]
        pub gift_exchange: DataTable<schemas::GiftExchangeData>,
    }
}
