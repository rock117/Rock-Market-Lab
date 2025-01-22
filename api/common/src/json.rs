use anyhow::{anyhow, bail};
use reqwest::{Response, StatusCode};
use serde::{de, Serialize};

pub fn to_json<T>(value: &T) -> anyhow::Result<String>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(value).map_err(|e| anyhow!(e))
}

pub fn from_json<'a, T>(json: &'a str) -> anyhow::Result<T>
where
    T: de::Deserialize<'a>,
{
    serde_json::from_str(json).map_err(|e| anyhow!(e))
}
