use surrealdb_types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct PlayerStrAttr {
    pub key: u32,
    pub value: String,
}
