use anyhow::anyhow;
use common::http;
use quick_xml::de::from_str;
use reqwest::Response;
use serde::Deserialize;
use crate::resp_to_string;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TokenEntity {
    pub is_success: bool,
    pub token: String,
    pub expire_date: String,
}

async fn login() -> anyhow::Result<TokenEntity> {
    let username = "";
    let password = "";
    let url = format!("https://equityapi.morningstar.com/WSLogin/Login.asmx/Login?email={}&password={}", username, password);
    let resp = http::get(&url, None).await?;
    let xml = resp_to_string(resp).await?;
    let token_entity: TokenEntity = from_str(&xml)?;
    Ok(token_entity)
}

