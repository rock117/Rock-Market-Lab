use entity::us_basic::Model as Stock;
use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn us_basic(offset: usize, limit: usize) -> anyhow::Result<Vec<Stock>> {
    let res = call_api_as::<Stock>(request!(Api::UsBasic,
         {"offset" => format!("{}", offset).as_str(), "limit" => format!("{}", limit).as_str()},
        [
                                        "ts_code",
                                          "name",
                                          "enname",
                                          "classify",
                                          "list_date",
                                         "delist_date"
                                          ])).await?;
    Ok(res.items)
}


