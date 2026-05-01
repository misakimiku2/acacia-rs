use chrono::{FixedOffset, NaiveDateTime, TimeZone};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::warn;

use super::parser::parse_gdpl;

pub fn with_gdpl<'de, D>(deserializer: D) -> Result<Option<(u16, u16, u16, u16)>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    match value {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => parse_gdpl::<D::Error>(&s).map(Some),
    }
}

pub fn with_datetime<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let offset = FixedOffset::east_opt(8 * 3600).ok_or_else(|| serde::de::Error::custom("The time_offset should not be error"))?;
    let naive = NaiveDateTime::parse_from_str(&s, "[%Y%m%d%H%M]").map_err(serde::de::Error::custom)?;
    let dt = offset.from_local_datetime(&naive).single().ok_or_else(|| serde::de::Error::custom("invalid datetime {s}"))?;
    Ok(dt.timestamp())
}

pub fn with_datetime_option<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    let Some(s) = opt else {
        return Ok(None);
    };
    if s.trim().is_empty() {
        return Ok(None);
    }
    let offset = FixedOffset::east_opt(8 * 3600).ok_or_else(|| serde::de::Error::custom("The time_offset should not be error"))?;
    let naive = NaiveDateTime::parse_from_str(&s, "[%Y%m%d%H%M]").map_err(serde::de::Error::custom)?;
    let dt = offset.from_local_datetime(&naive).single().ok_or_else(|| serde::de::Error::custom(format!("invalid datetime {s}")))?;
    Ok(Some(dt.timestamp()))
}

pub fn with_lua_table<'de, D, T>(deserializer: D) -> anyhow::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let s = String::deserialize(deserializer)?;
    acacia_commons::lua_utils::lua_serde::from_str::<T>(&s).map_err(serde::de::Error::custom)
}

pub fn load_tsv<T, P>(file_path: P) -> anyhow::Result<Vec<T>>
where
    P: AsRef<Path> + Debug,
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(&file_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut throwaway_line = String::new();
    buf_reader.read_line(&mut throwaway_line)?;

    let mut csv_reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(true).flexible(true).from_reader(buf_reader);

    let mut results = Vec::new();
    for result in csv_reader.deserialize() {
        match result {
            Ok(record) => results.push(record),
            Err(e) => match e.kind() {
                csv::ErrorKind::Deserialize { .. } => {
                    warn!("Failed to parse {file_path:?} line ({e})");
                    continue;
                }
                csv::ErrorKind::UnequalLengths { .. } => {
                    warn!("Failed to parse {file_path:?} line ({e})");
                    continue;
                }
                _ => return Err(e.into()),
            },
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas;

    #[test]
    fn test() {
        let v: Vec<schemas::GiftExchangeData> = load_tsv("../data/settings/house/gift_exchange.txt").unwrap();
        println!("{v:?}");
    }
}
