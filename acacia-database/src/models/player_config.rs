use surrealdb_types::SurrealValue;

#[derive(Debug, Default, SurrealValue)]
pub struct PlayerConfig {
    pub pid: u64,
    pub token: String,
    pub ip: String,
    pub sdk_username: String,
    pub sdk_password: String,
    pub sdk_phone_number: String,
}
