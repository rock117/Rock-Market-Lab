use map_macro::hash_map;

use entity::us_basic::Model as Stock;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn us_basic(offset: usize, limit: usize) -> anyhow::Result<Vec<Stock>> {
    call_tushare_api_as::<500, Stock>(Api::us_basic,
                                      &hash_map! {"offset" => format!("{}", offset).as_str(), "limit" => format!("{}", limit).as_str()},
                                      &vec![
                                          "ts_code",
                                          "name",
                                          "enname",
                                          "classify",
                                          "list_date",
                                         "delist_date"
                                      ]).await
}


mod tests {
    use common::json::to_json;

    use super::*;

    // #[tokio::test]
    async fn test_stock_basic() {
        let res = us_basic(100, 1).await.unwrap();
        std::fs::write(r#"C:\rock\coding\code\my\rust\programmer-investment-research\api\tmp\stock_basic.json"#, to_json(&res).unwrap()).unwrap();
        println!("{}", to_json(&res).unwrap().to_string());
    }
}