use chrono::NaiveDate;
use map_macro::hash_map;

use entity::margin::Model as Margin;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 融资融券交易汇总 https://tushare.pro/document/2?doc_id=58
pub async fn margin(exchange_id: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<Margin>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let params = &hash_map! {"exchange_id" => exchange_id, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()};
    let fields =  &[
        "trade_date",
        "exchange_id",
        "rzye",
        "rzmre",
        "rzche",
        "rqye", "rqmcl", "rzrqye", "rqyl"
    ];
    call_tushare_api_as::<0, Margin>(Api::margin, params, fields).await
}