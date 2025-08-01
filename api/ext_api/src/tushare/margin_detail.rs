use chrono::NaiveDate;
use entity::margin_detail::Model as MarginDetail;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 股票融资融券明细 https://tushare.pro/document/2?doc_id=59
pub async fn margin_detail(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<MarginDetail>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<MarginDetail, 500>(request!(Api::MarginDetail, {
        "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str(),
    }, [
        "trade_date",
        "ts_code",
        "name",
        "rzye",
        "rqye",
        "rzmre",
        "rqyl",
        "rzche",
        "rqchl",
        "rqmcl",
        "rzrqye",
    ])).await?;
    Ok(res.items)
}