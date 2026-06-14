use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GameServerConfig {
    pub host: String,
    pub port: u16,
    pub time_offset: i64,
}

impl Default for GameServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5200,
            time_offset: 0,
        }
    }
}
