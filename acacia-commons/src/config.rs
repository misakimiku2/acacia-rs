use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

mod database;
mod game_server;
mod gm_server;
use database::DatabaseConfig;
use game_server::GameServerConfig;
use gm_server::GmServerConfig;

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
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

impl Config {
    pub fn load() -> Self {
        if std::path::Path::new("./data/Config.toml").exists() {
            let content = fs::read_to_string("./data/Config.toml").expect("Failed to read Config.toml");
            return toml::from_str(&content).expect("Failed to parse Config.toml");
        }
        let config = Self::default();
        let contents = toml::to_string_pretty(&config).expect("Failed to serialize config");
        fs::write("./data/Config.toml", contents).expect("Failed to create default Config.toml");
        return config;
    }
}
