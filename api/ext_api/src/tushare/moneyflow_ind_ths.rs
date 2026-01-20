use chrono::NaiveDate;
use entity::moneyflow_industry_ths::Model as MoneyflowIndustryThs;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

pub async fn moneyflow_ind_ths(start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<MoneyflowIndustryThs>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<MoneyflowIndustryThs>(request!(Api::MoneyflowIndustryThs, {
        "trade_date" => start_date.as_str(),
        "start_date" => end_date.as_str(),
    }, [
        "trade_date",
        "ts_code",
        "industry",
        "lead_stock",
        "close",
        "pct_change",
        "company_num",
        "pct_change_stock",
        "close_price",
        "net_buy_amount",
        "net_sell_amount",
        "net_amount"
    ])).await?;
    Ok(res.items)
}