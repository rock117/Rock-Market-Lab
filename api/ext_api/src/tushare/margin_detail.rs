use chrono::NaiveDate;
use map_macro::hash_map;
use entity::stock_margin_detail::Model as StockMarginDetailModel;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 股票融资融券明细
pub async fn margin_detail(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<StockMarginDetailModel>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let params = hash_map! {
        "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str(),
    };
    let fields = &[
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
    ];
    call_tushare_api_as::<500, StockMarginDetailModel>(Api::margin_detail, &params, fields).await
}