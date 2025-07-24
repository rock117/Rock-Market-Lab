mod concurrent;
pub mod constant;
pub mod data_type;
pub mod dto;
mod eventbus;
pub mod finance;
mod fn_tool;
pub mod http;
pub mod json;
pub mod stastics;
mod to_param;
pub mod util;
pub mod calc;
mod pickup;
pub mod config;
pub mod cache;
pub mod paging;
pub mod db;
mod security_name;
pub mod web;
pub mod domain;

use anyhow::{anyhow, bail};
pub use data_type::SingleElement;

// pub use http::get;
// pub use http::post;
pub use bytes::Bytes;
use derive_more::Display;
use serde::Serialize;

use strum_macros::EnumString;
pub use security_name::get_security_pinyin;

static PY_API: &'static str = "http://localhost:18091/api/pinyin";

pub async fn get_first_chinese_letter(chinese_word: &str) -> anyhow::Result<String> {
    let resp = http::post(
        PY_API,
        Some(r#"{"word": "$"}"#.replace("$", chinese_word)),
        None,
    )
    .await?;
    if !resp.status().is_success() {
        bail!("http status not ok: {}", resp.status())
    }
    Ok(String::from_utf8(resp.bytes().await?.to_vec())?.to_string())
}

pub mod date {
    pub static FORMAT: &'static str = "%Y%m%d";
    pub static FORMAT_DASH: &'static str = "%Y-%m-%d";
}

pub trait ToAnyHowResult<T> {
    fn to_result(self) -> anyhow::Result<T>;
}

#[derive(Serialize, Debug, Copy, Clone, EnumString, Display)] // EnumString
pub enum ExchangeId {
    #[strum(serialize = "SSE")]
    SSE, //上交所
    #[strum(serialize = "SZSE")]
    SZSE, // 深交所
    #[strum(serialize = "BSE")]
    BSE, // 北交所
}

impl<T> ToAnyHowResult<T> for Option<T> {
    fn to_result(self) -> anyhow::Result<T> {
        self.ok_or(anyhow!("option no value"))
    }
}
