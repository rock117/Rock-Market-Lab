use std::collections::HashMap;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail};
use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
use once_cell::sync::Lazy;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tracing::warn;

use common::http;
use common::json::from_json;

use crate::resp_to_string;
pub use balancesheet::*;
pub use cashflow::*;
pub use daily::*;
pub use daily_basic::*;
pub use fina_indicator::*;
pub use fina_mainbz::*;
pub use fund_basic::*;
pub use fund_daily::*;
pub use income::*;
pub use index_basic::*;
pub use index_daily::*;
pub use index_monthly::*;
pub use index_weekly::*;
pub use margin::*;
pub use margin_detail::*;
pub use moneyflow::*;
pub use moneyflow_ind_ths::*;
pub use monthly::*;
pub use stk_holdernumber::*;
pub use stock_basic::*;
pub use ths_daily::*;
pub use ths_index::*;
pub use ths_member::*;
pub use trade_cal::*;
pub use us_basic::*;
pub use us_daily::*;

use crate::tushare::model::{Api, ApiParam, TushareApiResp};

mod balancesheet;
mod cashflow;
mod daily;
mod daily_basic;
mod fina_indicator;
mod fina_mainbz;
pub mod fund_basic;
mod fund_daily;
mod income;
mod index_basic;
mod index_daily;
mod index_daily_basic;
mod index_monthly;
mod index_weekly;
mod margin;
mod margin_detail;
mod model;
mod moneyflow;
mod moneyflow_ind_ths;
mod monthly;
mod stk_holdernumber;
mod stock_basic;
mod ths_daily;
mod ths_index;
mod ths_member;
mod trade_cal;
mod us_basic;
mod us_daily;

static TUSHARE_TOKEN: Lazy<String> = Lazy::new(|| {
    common::config::AppConfig::new()
        .expect("failed to get config")
        .tushare_token()
});

static TUSHARE_API: &'static str = "http://api.tushare.pro";
static TOKEN: Lazy<String> = Lazy::new(|| "token".to_string());
static CALL_LIMI: Lazy<Arc<Mutex<SlidingWindowRateLimiter>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SlidingWindowRateLimiter::new(
        Duration::from_secs(60),
        500,
    )))
});

static max_requests_per_minute: usize = 500;
static semaphore: Lazy<Arc<Semaphore>> =
    Lazy::new(|| Arc::new(Semaphore::new(max_requests_per_minute)));

/// add rate limit TODO
async fn call_tushare_api<'a, const N: u64>(
    param: &ApiParam<'a>,
) -> anyhow::Result<TushareApiResp> {
    // let limiter = CALL_LIMI.clone();
    // let mut limiter = limiter.lock().map_err(|e| anyhow!(e.to_string()))?;
    // limiter.wait_until_ready().await; // 等待限流器许可
    tokio::time::sleep(Duration::from_millis(N)).await;

    let inst = Instant::now();
    let mut resp = get_data(param, 3).await;
    let cost = inst.elapsed().as_secs();
    if cost >= 10 {
        warn!("tushare api call is slow, cost: {}s", cost);
    }
    let resp = resp?;
    if resp.status() != StatusCode::OK {
        bail!(
            "http status not ok: {}, header: {:?}, body: {}",
            resp.status(),
            resp.headers().clone(),
            resp_to_string(resp).await?
        );
    }
    let resp = resp_to_string(resp).await?;
    let resp = from_json::<TushareApiResp>(resp.as_str())?;

    if !resp.is_success() {
        bail!(
            "tushare api failed, code: {}, msg: {:?}",
            resp.code,
            resp.msg
        )
    }
    if resp.data.is_none() {
        bail!(
            "tushare api tushare-resp is null, code: {}, msg: {:?}",
            resp.code,
            resp.msg
        )
    }
    Ok(resp)
}

async fn call_tushare_api_as<const N: u64, T: DeserializeOwned>(
    api: Api,
    params: &HashMap<&str, &str>,
    fields: &[&str],
) -> anyhow::Result<Vec<T>> {
    let param = ApiParam {
        api_name: api,
        token: &TUSHARE_TOKEN,
        params,
        fields,
    };
    let res = call_tushare_api::<N>(&param).await?;
    res.data
        .ok_or(anyhow!("no data in margin resp"))?
        .to_structs::<T>()
}

async fn get_data<'a>(param: &ApiParam<'a>, retry_num: u32) -> anyhow::Result<Response> {
    let mut resp_opt: Option<anyhow::Result<Response>> = None;
    for i in 0..retry_num {
        let resp = http::post(TUSHARE_API, Some(serde_json::to_string(param)?), None).await;
        if let Err(e) = resp {
            resp_opt = Some(Err(e));
            continue;
        }
        let resp = resp?;
        if resp.status() == StatusCode::GATEWAY_TIMEOUT || resp.status() == StatusCode::BAD_GATEWAY
        {
            continue;
        }
        return Ok(resp);
    }
    resp_opt.ok_or(anyhow!(
        "retry {} times to call tushare_api, but failed",
        retry_num
    ))?
}
