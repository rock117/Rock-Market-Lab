use chrono::NaiveDate;
use map_macro::hash_map;

use entity::stock_daily::Model as StockDaily;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 股票日线行情行情
pub async fn daily(start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<StockDaily>> {
    let start_date = start.format("%Y%m%d").to_string();
    let end_date = end.format("%Y%m%d").to_string();
    call_tushare_api_as::<500, StockDaily>(Api::daily,
                        &hash_map! {"start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
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
