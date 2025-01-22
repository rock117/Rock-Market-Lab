use derive_more::Display;
use map_macro::hash_map;
use common::finance::ma;

use entity::fund;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 基金列表 https://tushare.pro/document/2?doc_id=19
#[derive(Debug, Clone, Copy, Display)]
pub enum FundMarket {
    /// 场内
    O,
    /// 场外
    E,
}
pub async fn fund_basic(market: FundMarket) -> anyhow::Result<Vec<fund::Model>> {
    let market = market.to_string();
    let params = &hash_map! {"market" => market.as_str()}; //E只取场内
    let fields = &[
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
    call_tushare_api_as::<0, fund::Model>(Api::fund_basic, params, fields).await
}