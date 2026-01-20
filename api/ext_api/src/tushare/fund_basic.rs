use map_macro::hash_map;
use common::finance::ma;

use entity::fund;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;


/// 基金列表 https://tushare.pro/document/2?doc_id=19
#[derive(Debug, Clone, Copy)]
pub enum FundMarket {
    /// 场内
    O,
    /// 场外
    E,
}

impl std::fmt::Display for FundMarket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FundMarket::O => write!(f, "O"),
            FundMarket::E => write!(f, "E"),
        }
    }
}
pub async fn fund_basic(market: FundMarket) -> anyhow::Result<Vec<fund::Model>> {
    let market = market.to_string();
    let params = hash_map! {"market".into() => market}; //E只取场内
    let fields= fields![
        "ts_code",
        "name",
        "management",
        "custodian",
        "fund_type",
        "found_date",
        "due_date",
        "list_date",
        "issue_date",
        "delist_date",
        "issue_amount",
        "m_fee",
        "c_fee",
        "duration_year",
        "p_value",
        "min_amount",
        "exp_return",
        "benchmark",
        "status",
        "invest_type",
        "type",
        "trustee",
        "purc_startdate",
        "redm_startdate",
        "market",
    ];
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params,
        fields,
    };

    let res = call_api_as::<fund::Model>(request).await?;
    Ok(res.items)
}