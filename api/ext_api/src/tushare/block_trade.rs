use chrono::NaiveDate;

use entity::block_trade::Model as BlockTrade;

use crate::tushare::call_api_as;
use tushare_api::{Api, TushareRequest, fields, params, request};

///大宗交易
pub async fn block_trade(ts_code: &str) -> anyhow::Result<Vec<BlockTrade>> {
    let res = call_api_as::<BlockTrade, 0>(
        request!(Api::Custom("block_trade".into()), {"ts_code" => ts_code},
            [
            "ts_code",
            "trade_date",
             "price",
            "vol",
            "amount",
            "buyer",
            "seller",
        ]),
    )
    .await?;
    Ok(res.items)
}
