pub mod manager {
    pub mod data_table;
    pub mod data_tables;
    pub mod tsv;
}
pub mod init_data;
pub mod schemas;
pub mod tables;

pub use init_data::init_data;
pub use tables::tables;

#[cfg(test)]
mod tests;
