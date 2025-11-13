use chrono::NaiveDate;
use entity::stk_holdertrade::Model as StkHoldertrade;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 股东减持
pub async fn stk_holdertrade(ts_code: &str) -> anyhow::Result<Vec<StkHoldertrade>> {
    let res = call_api_as::<StkHoldertrade, 500>(request!(Api::Custom("stk_holdertrade".into()),
                        {"ts_code" => ts_code},
                        [
                            "ts_code",
                            "ann_date",
                            "holder_name"                  ,
                            "holder_type",
                            "in_de",
                            "change_vol",
                            "change_ratio",
                            "after_share",
                            "after_ratio",
                            "avg_price",
                            "total_share",
                            "begin_date",
                            "close_date"
                        ])).await?;
    Ok(res.items)
}