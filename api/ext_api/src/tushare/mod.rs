use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::anyhow;
use async_rate_limit::sliding_window::SlidingWindowRateLimiter;
use once_cell::sync::Lazy;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tracing::info;
use tushare_api::{FromTushareData, LogConfig, LogLevel, TushareClient, TushareEntityList, TushareRequest, TushareResult};

pub use balancesheet::*;
pub use cashflow::*;
use common::http;
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
pub use etf_basic::*;
pub use fund_portfolio::*;
pub use stk_holdertrade::*;
pub use dc_index::*;
pub use dc_member::*;
pub use block_trade::*;
pub use hm_detail::*;

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
mod etf_basic;
mod fund_portfolio;
mod stk_holdertrade;
mod dc_index;
mod dc_member;
mod block_trade;
mod hm_detail;
mod limit_list_d;

static TUSHARE_TOKEN: Lazy<String> = Lazy::new(|| {
    common::config::AppConfig::new()
        .expect("failed to get config")
        .tushare_token()
});

static CALL_LIMI: Lazy<Arc<Mutex<SlidingWindowRateLimiter>>> = Lazy::new(|| {
    Arc::new(Mutex::new(SlidingWindowRateLimiter::new(
        Duration::from_secs(60),
        500,
    )))
});

static max_requests_per_minute: usize = 500;
static semaphore: Lazy<Arc<Semaphore>> =
    Lazy::new(|| Arc::new(Semaphore::new(max_requests_per_minute)));

static TUSHARE_CLIENT: Lazy<TushareClient> = Lazy::new(|| {
    let mut log = LogConfig::default();
    log.log_responses_err = true;
    TushareClient::builder()
        .with_token(TUSHARE_TOKEN.as_str())
        .with_log_level(LogLevel::Info)
        .with_log_config(log)
        .log_requests(true)
        .log_responses(false)
        .log_sensitive_data(false) // 生产环境建议设为 false
        .log_performance(true)
        .with_connect_timeout(Duration::from_secs(300))
        .with_timeout(Duration::from_secs(300))
        .build()
        .unwrap()
});


pub async fn call_api_as<T, const N: u64>(request: TushareRequest) -> TushareResult<TushareEntityList<T>> where T: FromTushareData + std::fmt::Debug {
    // let data = TUSHARE_CLIENT.call_api(request.clone()).await;
    // info!("call_api, response: {:?}", data);
    let data = TUSHARE_CLIENT.call_api_as::<T>(request).await;
  //  info!("call_api_as, response: {:?}", data);
  //  sleep(Duration::from_millis(N)).await;
    data
}
