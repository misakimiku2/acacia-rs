use chrono::{DateTime, FixedOffset};

pub fn to_datetime(src: i64) -> String {
    let offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset");
    return DateTime::from_timestamp(src, 0)
        .map(|dt| dt.with_timezone(&offset))
        .expect("Invalid timestamp")
        .format("%Y%m%d%H%M")
        .to_string();
}
