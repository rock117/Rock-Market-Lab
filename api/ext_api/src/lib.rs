use anyhow::anyhow;
use reqwest::Response;

pub mod tushare;
pub mod mstar;

async fn resp_to_string(resp: Response) -> anyhow::Result<String> {
    String::from_utf8(resp.bytes().await?.as_ref().to_vec()).map_err(|e| anyhow!(e))
}