use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::warn;

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
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(file_path)?;
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
                    warn!("Failed to parse line: {e}");
                    continue;
                }
                csv::ErrorKind::UnequalLengths { .. } => {
                    warn!("Failed to parse line: {e}");
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
