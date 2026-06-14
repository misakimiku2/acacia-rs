use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GmServerConfig {
    pub enable: bool,
}

impl Default for GmServerConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}
