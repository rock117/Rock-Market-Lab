use chrono::NaiveDate;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

pub async fn index_daily(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<entity::index_daily::Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<entity::index_daily::Model, 500>(request!(Api::IndexDaily, {
         "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str()
    }, [
         "ts_code" ,
        "trade_date" ,
        "close" ,
        "open" ,
        "high" ,
        "low" ,
        "pre_close" ,
        "change" ,
        "pct_chg" ,
        "vol" ,
        "amount"
    ])).await?;
    Ok(res.items)
}