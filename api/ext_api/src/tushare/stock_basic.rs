use map_macro::hash_map;

use entity::stock::Model as Stock;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn stock_basic() -> anyhow::Result<Vec<Stock>> {
    call_tushare_api_as::<500, Stock>(Api::stock_basic,
                        &hash_map! {"list_stauts" => "L"},
                        &vec![
                            "ts_code",
                            "symbol",
                            "name",
                            "area",
                            "industry",
                            "list_date",
                            "exchange",
                            "market",
                            "list_status",
                            "delist_date",
                            "is_hs",
                            "cnspell",
                            "enname",
                            "fullname",
                            "curr_type",
                            "act_name",
                            "act_ent_type",
                        ]).await
}


mod tests {
    use common::json::to_json;

    use super::*;

    // #[tokio::test]
    async fn test_stock_basic() {
        let res = stock_basic().await.unwrap();
        std::fs::write(r#"C:\rock\coding\code\my\rust\programmer-investment-research\api\tmp\stock_basic.json"#, to_json(&res).unwrap()).unwrap();
        println!("{}", to_json(&res).unwrap().to_string());
    }
}