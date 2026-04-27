use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: String,
    pub game_server: GameServerConfig,
    pub gm_server: GmServerConfig,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log: "info".to_string(),
            game_server: GameServerConfig::default(),
            gm_server: GmServerConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct GmServerConfig {
    pub enable: bool,
}

impl Default for GmServerConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub mode: String,
    pub address: String,
    pub username: String,
    pub password: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            mode: if cfg!(feature = "full") { "local".to_string() } else { "remote".to_string() },
            address: "ws://127.0.0.1:8000".to_string(),
            username: "acacia-root".to_string(),
            password: "acacia-secret".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path: &str = if std::path::Path::new("./gameserver.toml").exists() {
            "./gameserver.toml"
        } else if std::path::Path::new("./acacia-game-server/gameserver.toml").exists() {
            "./acacia-game-server/gameserver.toml"
        } else if std::path::Path::new("./data/gameserver.toml").exists() {
            "./data/gameserver.toml"
        } else {
            return Self::default();
        };
        let content = fs::read_to_string(path).expect("Failed to read gameserver.toml");
        toml::from_str(&content).expect("Failed to parse gameserver.toml")
    }
}
