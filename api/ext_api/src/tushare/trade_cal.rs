use map_macro::hash_map;

use entity::trade_calendar::Model as TradeCalendar;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 获取交易日历数据
pub async fn trade_cal() -> anyhow::Result<Vec<TradeCalendar>> {
    call_tushare_api_as::<500, TradeCalendar>(Api::trade_cal,
                        &hash_map! {"exchange" => "SSE", "start_date" => "20000101", "end_date" => "20251231"},
                        &vec![
                            "exchange",
                            "cal_date",
                            "is_open",
                            "pretrade_date",
                        ]).await
}
