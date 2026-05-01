use chrono::{DateTime, Datelike, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::base::config::CONFIG;

pub struct ServerTime;

impl ServerTime {
    pub fn now() -> i64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 + CONFIG.game_server.time_offset + 8 * 3600
    }

    pub fn day_u32() -> u32 {
        let ts = Self::now() - 4 * 3600;
        let dt: DateTime<Utc> = DateTime::from_timestamp(ts, 0).unwrap();
        let year = dt.year() as u32;
        let month = dt.month();
        let day = dt.day();
        year * 10000 + month * 100 + day
    }
    
    pub fn month_u32() -> u32 {
        let ts = Self::now() - 4 * 3600;
        let dt: DateTime<Utc> = DateTime::from_timestamp(ts, 0).unwrap();
        let year = dt.year() as u32;
        let month = dt.month();
        year * 100 + month
    }
}
