use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;
use entity::stock::Model as Stock;


/// 获取基础信息数据，包括股票代码、名称、上市日期、退市日期等
pub async fn stock_basic() -> anyhow::Result<Vec<Stock>> {
    let req = request!(Api::StockBasic, {
            "list_status" => "L",
        }, [
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
        ]);
    let res = call_api_as::<Stock>(req.clone()).await?;
    Ok(res.items)
}