use chrono::NaiveDate;
use map_macro::hash_map;

use entity::stock_daily::Model as StockDaily;

use crate::tushare::{call_api_as, TUSHARE_CLIENT};
use tushare_api::{Api, LogLevel, request, fields, params, TushareClient, TushareRequest};
use entity::stock_daily_basic;

/// 股票日线行情行情
pub async fn daily(tscode: Option<&str>, start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<StockDaily>> {
    let start_date = start.format("%Y%m%d").to_string();
    let end_date = end.format("%Y%m%d").to_string();
    let params = match tscode {
        Some(tscode) => hash_map! {"ts_code" => tscode, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
        None => hash_map! {"start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
    }.iter ().map (|(k, v)| (k.to_string (), v.to_string ())).collect ();;
    let fields = fields![
        "ts_code",
        "trade_date",
        "open",
        "high",
        "low",
        "close",
        "pre_close",
        "change",
        "pct_chg",
        "vol",
        "amount"
    ];
    let req = TushareRequest {
        api_name: Api::Daily,
        params,
        fields,
    };
    let res = call_api_as::<StockDaily>(req.clone()).await?;
    Ok(res.items)
}
