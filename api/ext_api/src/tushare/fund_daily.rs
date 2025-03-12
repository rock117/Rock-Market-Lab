use chrono::NaiveDate;
use map_macro::hash_map;

use entity::fund_daily::Model as FundDaily;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 基金日线行情行情
pub async fn fund_daily(tscode: &str, start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<FundDaily>> {
    let start_date = start.format("%Y%m%d").to_string();
    let end_date = end.format("%Y%m%d").to_string();
    let parmas = hash_map! {"ts_code" => tscode, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()};

    call_tushare_api_as::<500, FundDaily>(Api::fund_daily,
                                          &parmas,
                                          &vec![
                                              "ts_code",
                                              "trade_date",
                                              "open",
                                              "high",
                                              "low",
                                              "close",
                                              "pre_close",
                                              "change",
                                              "pct_chg",
                                              "vol",
                                              "amount",
                                          ]).await
}
