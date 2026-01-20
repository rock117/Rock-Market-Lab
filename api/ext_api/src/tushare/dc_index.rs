use chrono::NaiveDate;

use entity::dc_index::Model as DcIndex;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 东方财富概念板块 https://tushare.pro/document/2?doc_id=362
pub async fn dc_index() -> anyhow::Result<Vec<DcIndex>> {
    let res = call_api_as::<DcIndex>(request!(Api::Custom("dc_index".into()), {},
        ["ts_code",
        "trade_date",
        "name",
        "leading",
        "leading_code",
        "pct_change",
        "leading_pct", "total_mv", "turnover_rate", "up_num", "down_num"
    ])).await?;
    Ok(res.items)
}