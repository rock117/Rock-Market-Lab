use std::collections::HashMap;
use std::fmt::Write;
use std::num::NonZeroU32;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail};
use async_rate_limit::limiters::RateLimiter;
use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use map_macro::hash_map;
use once_cell::sync::Lazy;
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde::ser::{SerializeMap, SerializeStruct};
use tokio::sync::Semaphore;
use tracing::{debug, warn};

use common::data_type::NumOrString;
use common::http;
use common::json::from_json;
use common::util::csv_util;

pub use daily::*;
pub use stock_basic::*;
pub use trade_cal::*;
pub use fina_indicator::*;
pub use stk_holdernumber::*;
pub use fina_mainbz::*;
pub use margin::*;
pub use balancesheet::*;
pub use income::*;
pub use cashflow::*;
pub use daily_basic::*;
use entity::sea_orm::sqlx::encode::IsNull::No;
pub use fund_basic::*;
pub use index_basic::*;
pub use index_daily::*;
pub use index_weekly::*;
pub use index_monthly::*;

pub use moneyflow::*;
pub use moneyflow_ind_ths::*;
pub use margin_detail::*;

use crate::tushare::model::{Api, ApiParam, TushareApiResp};

mod stock_basic;
mod trade_cal;
mod daily;
mod model;
mod fina_indicator;
mod stk_holdernumber;
mod fina_mainbz;
mod margin;
mod balancesheet;
mod income;
mod cashflow;
mod daily_basic;
pub mod fund_basic;
mod index_basic;
mod index_daily;
mod index_daily_basic;
mod moneyflow;
mod moneyflow_ind_ths;
mod margin_detail;
mod index_weekly;
mod index_monthly;

static TUSHARE_TOKEN: Lazy<String> =  Lazy::new(|| common::config::AppConfig::new().expect("failed to get config").tushare_token());

static TUSHARE_API: &'static str = "http://api.tushare.pro";
static TOKEN: Lazy<String> = Lazy::new(|| "token".to_string());
static CALL_LIMI: Lazy<Arc<Mutex<SlidingWindowRateLimiter>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SlidingWindowRateLimiter::new(Duration::from_secs(60), 500)))
});

static max_requests_per_minute: usize = 500;
static semaphore: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(max_requests_per_minute)));


/// add rate limit TODO
async fn call_tushare_api<'a, const N: u64>(param: &ApiParam<'a>) -> anyhow::Result<TushareApiResp> {
    // let limiter = CALL_LIMI.clone();
    // let mut limiter = limiter.lock().map_err(|e| anyhow!(e.to_string()))?;
    // limiter.wait_until_ready().await; // 等待限流器许可
    tokio::time::sleep(Duration::from_millis(N)).await;

    debug!(
        "calling Tushare API, url: {}, param: {:?}",
        TUSHARE_API, serde_json::to_string(param)
    );
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


async fn call_tushare_api_as<const N: u64, T: DeserializeOwned>(api: Api, params: &HashMap<&str, &str>, fields: &[&str]) -> anyhow::Result<Vec<T>> {
    let param = ApiParam {
        api_name: api,
        token: &TUSHARE_TOKEN,
        params,
        fields,
    };
    let res = call_tushare_api::<N>(&param).await?;
    res.data.ok_or(anyhow!("no data in margin resp"))?.to_structs::<T>()
}

async fn resp_to_string(resp: Response) -> anyhow::Result<String> {
    String::from_utf8(resp.bytes().await?.as_ref().to_vec()).map_err(|e| anyhow!(e))
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
        if resp.status() == StatusCode::GATEWAY_TIMEOUT || resp.status() == StatusCode::BAD_GATEWAY {
            continue;
        }
        return Ok(resp);
    }
    resp_opt.ok_or(anyhow!("retry {} times to call tushare_api, but failed", retry_num))?
}

