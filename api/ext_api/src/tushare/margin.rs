use chrono::NaiveDate;

use entity::margin::Model as Margin;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 融资融券交易汇总 https://tushare.pro/document/2?doc_id=58
pub async fn margin(exchange_id: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<Margin>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<Margin, 0>(request!(Api::Margin, {"exchange_id" => exchange_id, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
        [
        "trade_date",
        "exchange_id",
        "rzye",
        "rzmre",
        "rzche",
        "rqye", "rqmcl", "rzrqye", "rqyl"
    ])).await?;
    Ok(res.items)
}