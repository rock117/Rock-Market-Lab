use chrono::NaiveDate;
use entity::limit_list_d;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 涨跌停和炸板数据 https://tushare.pro/document/2?doc_id=298
pub async fn limit_list_d(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) ->  anyhow::Result<Vec<limit_list_d::Model>>  {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<limit_list_d::Model>(request!(Api::Custom("limit_list_d".into()),
        {"ts_code" => ts_code, "start_date" => start_date, "end_date" => end_date},
        [
          "trade_date",
          "ts_code",
          "industry",
          "name",
          "close",
          "pct_chg",
          "amount",
          "limit_amount",
          "float_mv",
          "total_mv",
          "turnover_ratio",
          "fd_amount",
          "first_time",
          "last_time",
          "open_times",
          "up_stat",
          "limit_times",
          "limit"
        ])).await?;
    Ok(res.items)
}