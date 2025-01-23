use chrono::NaiveDate;
use map_macro::hash_map;
use entity::moneyflow_industry_ths::Model as MoneyflowIndustryThs;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

pub async fn moneyflow_ind_ths(start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<MoneyflowIndustryThs>> {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let params = hash_map! {
        "trade_date" => start_date.as_str(),
        "start_date" => end_date.as_str(),
    };
    let fields = &[
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
    ];
    call_tushare_api_as::<500, MoneyflowIndustryThs>(Api::moneyflow_industry_ths, &params, fields).await
}