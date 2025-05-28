use crate::json::from_json;
use anyhow::{anyhow, bail};
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::{Body, Client, Error, Response, StatusCode};
use serde::de;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::time::Duration;
use tokio::time::Instant;
use tracing::{debug, error, info, instrument, warn};

static CLIENT: Lazy<Client> = Lazy::new(|| build_client(120, 30));

trait Headers {
    fn to_map(self) -> anyhow::Result<HashMap<String, String>>;
}

impl Headers for String {
    fn to_map(self) -> anyhow::Result<HashMap<String, String>> {
        todo!()
    }
}

fn build_client(conn_timeout: u64, read_timeout: u64) -> Client {
    reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(read_timeout))
        .connect_timeout(Duration::from_secs(conn_timeout))
        .build()
        .unwrap()
}

#[instrument]
pub async fn get(url: &str, headers: Option<&HashMap<&str, &str>>) -> anyhow::Result<Response> {
    let instant = Instant::now();
    let mut req_builder = CLIENT.get(url);
    if let Some(headers) = headers {
        for header in headers {
            req_builder = req_builder.header(*header.0, *header.1);
        }
    }
    let data = req_builder.send().await;
    info!("GET {} cost {} ms", url, instant.elapsed().as_millis()); // TODO
    Ok(data?)
}

#[instrument]
pub async fn post<T: Into<Body> + Debug>(
    url: &str,
    body: Option<T>,
    headers: Option<&HashMap<String, String>>,
) -> anyhow::Result<Response> {
    let instant = Instant::now();
    let mut req_builder = CLIENT.post(url);
    if let Some(headers) = headers {
        for header in headers {
            req_builder = req_builder.header(header.0, header.1);
        }
    }
    if let Some(body) = body {
        req_builder = req_builder.body(body);
    }
    let data = req_builder.send().await;
    match data {
        Ok(data) => {
            Ok(data)
        }
        Err(e) => {
            log_response("POST", url, instant.elapsed().as_millis(), 0u16, Some(&e));
            bail!(e)
        }
    }
}

fn log_response<E: Debug>(method: &str, url: &str, cost: u128, status: u16, error: Option<&E>) {
    match error {
        None => debug!(
            "{} {} complete, status = {}, cost {} ms ",
            method, url, status, cost
        ),
        Some(err) => warn!(
            "{} {} complete, status = {}, cost {} ms, error: {:?}  ",
            method, url, status, cost, err
        ),
    }
}

pub async fn to_string(resp: Response) -> anyhow::Result<String> {
    if resp.status() != StatusCode::OK {
        bail!(
            "http status not ok: {}, header: {:?}, body: {}",
            resp.status(),
            resp.headers().clone(),
            String::from_utf8(resp.bytes().await?.as_ref().to_vec()).map_err(|e| anyhow!(e))?
        );
    }
    let resp = String::from_utf8(resp.bytes().await?.as_ref().to_vec()).map_err(|e| anyhow!(e))?;
    Ok(resp)
}

// pub async fn to_object<'a, T>(resp: Response) -> anyhow::Result<T> where T: de::Deserialize<'a>,{
//     let s = to_string(resp).await?;
//     serde_json::from_str(&s).map_err(|e| anyhow!(e))
// }
//

pub trait ToBytes {
    async fn to_bytes(self) -> anyhow::Result<Bytes>;
}
pub trait ToString {
    async fn to_string(self) -> anyhow::Result<String>;
}

impl ToBytes for Response {
    async fn to_bytes(self) -> anyhow::Result<Bytes> {
        println!("tobytes, http status: {}", self.status());
        if self.status() != StatusCode::OK {
            bail!(
                "http status not ok: {}, header: {:?}, body: {}",
                self.status(),
                self.headers().clone(),
                String::from_utf8(self.bytes().await?.as_ref().to_vec()).map_err(|e| anyhow!(e))?
            );
        }
        Ok(self.bytes().await?)
    }
}

impl ToString for Response {
    async fn to_string(self) -> anyhow::Result<String> {
        let bytes = self.to_bytes().await?;
        let resp = String::from_utf8(bytes.as_ref().to_vec()).map_err(|e| anyhow!(e))?;
        Ok(resp)
    }
}
