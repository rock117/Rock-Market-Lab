use chrono::NaiveDate;
use map_macro::hash_map;
use common::constant::DATE_YMD;

use entity::us_daily::Model as UsDaily;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn us_daily(trade_date: &NaiveDate) -> anyhow::Result<Vec<UsDaily>> {
    let trade_date = trade_date.format(DATE_YMD).to_string();
    call_tushare_api_as::<500, UsDaily>(Api::us_daily,
                                      &hash_map! {"trade_date" => trade_date.as_str()},
                                      &vec![
                                          "ts_code",
                                          "trade_date",
                                          "close",
                                          "open",
                                          "high",
                                          "low",
                                          "pre_close",
                                          "change",
                                          "pct_change",
                                          "vol",
                                          "amount",
                                          "vwap",
                                          "turnover_ratio",
                                          "total_mv",
                                          "pe",
                                          "pb"
                                      ]).await
}