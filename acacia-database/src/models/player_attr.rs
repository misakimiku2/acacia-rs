use surrealdb_types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct PlayerAttr {
    pub key: u32,
    pub value: u32,
}
