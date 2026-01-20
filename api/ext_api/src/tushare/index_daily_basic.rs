use chrono::NaiveDate;
use entity::index_daily_basic::Model;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;


pub async fn index_daily_basic(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<Model>(request!(Api::IndexDailyBasic,
        {"ts_code" => ts_code, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()}, [
        "ts_code",
        "trade_date",
        "total_mv",
        "float_mv",
        "total_share",
        "float_share",
        "free_share",
        "turnover_rate",
        "turnover_rate_f",
        "pe",
        "pe_ttm",
        "pb"
    ])).await?;
    Ok(res.items)
}