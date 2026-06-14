use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub mode: String,
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            mode: "default".to_string(),
            address: "ws://127.0.0.1:8000".to_string(),
            username: "acacia-root".to_string(),
            password: "acacia-secret".to_string(),
        }
    }
}
