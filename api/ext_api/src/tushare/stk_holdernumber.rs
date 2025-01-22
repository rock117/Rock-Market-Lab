use chrono::NaiveDate;
use map_macro::hash_map;

use entity::stock_holder_number::Model as StockHolderNumber;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 股东户数
pub async fn stk_holdernumber(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<StockHolderNumber>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    call_tushare_api_as::<500, StockHolderNumber>(Api::stk_holdernumber,
                        &hash_map! {"ts_code" => ts_code, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
                        &vec![
                            "ts_code",
                            "ann_date",
                            "end_date",
                            "holder_num",
                        ]).await
}