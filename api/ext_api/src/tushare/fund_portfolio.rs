use chrono::NaiveDate;
use map_macro::hash_map;

use entity::fund_portfolio::Model as FundPortfolio;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;


/// 基金日线行情行情
pub async fn fund_portfolio(tscode: &str) -> anyhow::Result<Vec<FundPortfolio>> {
    let parmas = hash_map! {"ts_code".into() => tscode.into()};
    let fields = fields!["ts_code",
                                              "ann_date",
                                              "end_date",
                                              "symbol",
                                              "mkv",
                                              "amount",
                                              "stk_mkv_ratio",
                                              "stk_float_ratio"
                                              ];
    let request = TushareRequest {
        api_name: Api::FundPortfolio,
        params: parmas,
        fields,
    };
    let res = call_api_as::<FundPortfolio, 500>(request).await?;
    Ok(res.items)
}
