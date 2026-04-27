use anyhow::{Context, Result, anyhow};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use super::tsv;

pub struct DataTable<T> {
    data: Vec<T>,
    indexes: HashMap<String, HashMap<String, Vec<usize>>>,
}

impl<T> fmt::Debug for DataTable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTable").field("data_count", &self.data.len()).field("indexes", &self.indexes).finish()
    }
}

impl<T: DeserializeOwned + Clone> DataTable<T> {
    pub fn load(table_name: &str) -> Result<Self> {
        let path = ["data/settings", "../data/settings"]
            .iter()
            .map(|base| PathBuf::from(base).join(table_name))
            .find(|p| p.exists())
            .ok_or_else(|| anyhow!("Cannot find table: {table_name}"))?;
    
        let data: Vec<T> = tsv::load_tsv(&path).with_context(|| format!("Failed to load table: data/settings/{table_name}"))?;
        Ok(Self { data, indexes: HashMap::new() })
    }

    pub fn create_index<K, F>(&mut self, index_name: &str, extractor: F)
    where
        K: Serialize,
        F: Fn(&T) -> K,
    {
        let mut index = HashMap::new();
        for (idx, item) in self.data.iter().enumerate() {
            let key_struct = extractor(item);
            // 将结构体序列化为 JSON 字符串作为 HashMap 的 Key
            let key_str = serde_json::to_string(&key_struct).unwrap_or_default();
            index.entry(key_str).or_insert_with(Vec::new).push(idx);
        }
        self.indexes.insert(index_name.to_string(), index);
    }

    pub fn find_one<K: Serialize>(&self, index_name: &str, key: K) -> Option<&T> {
        let key_str = serde_json::to_string(&key).unwrap_or_default();
        self.indexes
            .get(index_name)
            .and_then(|index| index.get(&key_str).and_then(|indices| indices.first().map(|&idx| &self.data[idx])))
    }

    pub fn find<K: Serialize>(&self, index_name: &str, key: K) -> Vec<&T> {
        let key_str = serde_json::to_string(&key).unwrap_or_default();
        self.indexes
            .get(index_name)
            .and_then(|index| index.get(&key_str).map(|indices| indices.iter().map(|&idx| &self.data[idx]).collect()))
            .unwrap_or_default()
    }

    pub fn all(&self) -> &[T] {
        &self.data
    }
}
