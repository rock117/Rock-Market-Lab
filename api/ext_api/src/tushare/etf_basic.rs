use chrono::NaiveDate;
use tushare_api::{Api, request, fields, params};
use entity::etf;
use crate::tushare::{call_api_as, TUSHARE_CLIENT};
use tushare_api::TushareRequest;


/// # 每日指标
/// - `ts_code`: TS股票代码, tscode可以是逗号隔开, 空标识获取所有股票
/// - `trade_date`: 交易日期
pub async fn etf_basic() -> anyhow::Result<Vec<etf::Model>> {
    let req = request!(Api::Custom("etf_basic".into()), {}, [
         "ts_code",
        "csname",
        "cname",
        "extname",
        "index_code",
        "index_name",
        "setup_date",
        "list_date",
        "list_status",
        "exchange",
        "custod_name",
        "mgr_name",
        "mgt_fee",
        "etf_type",
        ]);
    let res = call_api_as::<etf::Model, 500>(req.clone()).await?;
    Ok(res.items)
}