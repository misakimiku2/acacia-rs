pub mod manager {
    pub mod data_table;
    pub mod data_tables;
    pub mod parser;
    pub mod tsv;
}
pub mod schemas {
    mod activity;
    mod house;
    mod item;
    mod others;

    pub use activity::*;
    pub use house::*;
    pub use item::*;
    pub use others::*;
}
pub mod init_data;
pub mod tables;
