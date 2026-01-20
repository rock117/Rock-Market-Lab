use chrono::NaiveDate;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 股票月线 https://tushare.pro/document/2?doc_id=145
pub async fn monthly(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<entity::stock_monthly::Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<entity::stock_monthly::Model>(request!(Api::Monthly,  {
        "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str(),
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
        "amount" ,
    ])).await?;
    Ok(res.items)
}