use chrono::NaiveDate;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 个股资金流向 https://tushare.pro/document/2?doc_id=170
pub async fn moneyflow(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<entity::moneyflow::Model>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<entity::moneyflow::Model, 500>(request!(Api::Moneyflow,  {
        "ts_code" => ts_code,
        "start_date" => start_date.as_str(),
        "end_date" => end_date.as_str(),
    }, [
        "ts_code",
        "trade_date",
        "buy_sm_vol",
        "buy_sm_amount",
        "sell_sm_vol",
        "sell_sm_amount",
        "buy_md_vol",
        "buy_md_amount",
        "sell_md_vol",
        "sell_md_amount",
        "buy_lg_vol",
        "buy_lg_amount",
        "sell_lg_vol",
        "sell_lg_amount",
        "buy_elg_vol",
        "buy_elg_amount",
        "sell_elg_vol",
        "sell_elg_amount",
        "net_mf_vol",
        "net_mf_amount",
    ])).await?;
    Ok(res.items)
}