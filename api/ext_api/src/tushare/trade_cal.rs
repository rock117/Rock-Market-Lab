use entity::trade_calendar::Model as TradeCalendar;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 获取交易日历数据
pub async fn trade_cal() -> anyhow::Result<Vec<TradeCalendar>> {
    let res = call_api_as::<TradeCalendar, 500>(request!(Api::TradeCal,
        {"exchange" => "SSE", "start_date" => "20200101", "end_date" => "20261231"},
        [
                                          "exchange",
                                          "cal_date",
                                          "is_open",
                                          "pretrade_date"
                                          ])).await?;
    Ok(res.items)
}
