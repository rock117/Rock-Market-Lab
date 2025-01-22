use chrono::NaiveDate;
use map_macro::hash_map;
use entity::index_daily_basic::Model;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

pub async fn index_daily_basic(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let params = &hash_map! {"ts_code" => ts_code, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()};
    let fields = &[
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
    ];
    call_tushare_api_as::<500, Model>(Api::index_daily_basic, params, fields).await
}