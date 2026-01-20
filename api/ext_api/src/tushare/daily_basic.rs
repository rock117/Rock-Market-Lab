use chrono::NaiveDate;
use tushare_api::{Api, request, fields, params};
use entity::stock_daily_basic;
use crate::tushare::{call_api_as, TUSHARE_CLIENT};
use tushare_api::TushareRequest;


/// # 每日指标
/// - `ts_code`: TS股票代码, tscode可以是逗号隔开, 空标识获取所有股票
/// - `trade_date`: 交易日期
pub async fn daily_basic(start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<stock_daily_basic::Model>> {
    let start_date = start.format("%Y%m%d").to_string();
    let end_date = end.format("%Y%m%d").to_string();
    let req = request!(Api::DailyBasic, {
            "start_date" => start_date.as_str(), "end_date" => end_date.as_str(),
        }, [
            "ts_code",
            "trade_date",
            "close",
            "turnover_rate",
            "turnover_rate_f",
            "volume_ratio",
            "pe",
            "pe_ttm",
            "pb",
            "ps",
            "ps_ttm",
            "dv_ratio",
            "dv_ttm",
            "total_share",
            "float_share",
            "free_share",
            "total_mv",
            "circ_mv",
        ]);
    let res = call_api_as::<stock_daily_basic::Model>(req.clone()).await?;
    Ok(res.items)
}