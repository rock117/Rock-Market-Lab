use chrono::NaiveDate;

use entity::dc_member::Model as DcMember;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 东方财富概念板块 https://tushare.pro/document/2?doc_id=362
pub async fn dc_member(ts_code: &str) -> anyhow::Result<Vec<DcMember>> {
    let res = call_api_as::<DcMember>(request!(Api::Custom("dc_member".into()), {
        "ts_code" => ts_code
    },
        ["ts_code",
        "trade_date",
        "name",
        "con_code",
    ])).await?;
    Ok(res.items)
}