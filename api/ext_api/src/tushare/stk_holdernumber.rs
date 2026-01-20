use chrono::NaiveDate;
use entity::stock_holder_number::Model as StockHolderNumber;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 股东户数
pub async fn stk_holdernumber(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<StockHolderNumber>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<StockHolderNumber>(request!(Api::StkHoldernumber,
                        {"ts_code" => ts_code, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
                        [
                            "ts_code",
                            "ann_date",
                            "end_date"                  ,
                            "holder_num",
                        ])).await?;
    Ok(res.items)
}