use chrono::NaiveDate;
use common::constant::DATE_YMD;

use entity::us_daily::Model as UsDaily;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;
use tracing::{debug, info};

/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn us_daily(ts_code: &str, start_date: &str, end_date: &str) -> anyhow::Result<Vec<UsDaily>> {
    info!("fetch us_daily, ts_code: {}, start_date: {}, end_date: {}", ts_code, start_date, end_date);
    let req = request!(Api::UsDaily,
         {"start_date" => start_date, "end_date" => end_date, "ts_code" => ts_code},
        [
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
                                          ]);
    info!("req generated");
    let res = call_api_as::<UsDaily>(req).await?;
    Ok(res.items)
}