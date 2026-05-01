use std::time::{SystemTime, UNIX_EPOCH};

use crate::base::config::CONFIG;

pub struct ServerTime;

impl ServerTime {
    pub fn now() -> i64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 + CONFIG.game_server.time_offset + 8 * 3600
    }
}
