use chrono::NaiveDate;
use map_macro::hash_map;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

pub async fn index_daily(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<entity::index_daily::Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let params = &hash_map! {
        "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str(),
    };
    let fields = &[
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
    ];
    call_tushare_api_as::<500, entity::index_daily::Model>(Api::index_daily, params, fields).await
}