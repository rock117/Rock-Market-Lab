use serde::{Deserialize, Serialize};
use dashmap::DashMap;
use once_cell::sync::Lazy;

static CACHE: Lazy<DashMap<String, String>> = Lazy::new(|| DashMap::new());

pub fn put<T:Serialize>(key: String, value: &T) -> anyhow::Result<()>{
    CACHE.insert(key, serde_json::to_string(value)?);
    Ok(())
}

pub fn get<T: for<'a> Deserialize<'a>>(key: &str) -> anyhow::Result<Option<T>>{
    let data = CACHE.get(key);
    match data {
        None => Ok(None),
        Some(data) => serde_json::from_str::<T>(&data.value()).map_err(|e| anyhow::anyhow!(e)).map(|v| Some(v)),
    }
}