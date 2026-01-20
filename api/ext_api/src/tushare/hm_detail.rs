/// 游资每日明细 https://tushare.pro/document/2?doc_id=312

use chrono::NaiveDate;
use entity::hm_detail::Model as HmDetail;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

pub async fn get_hm_detail(ts_code: &str, start_date: &NaiveDate, end_date: &NaiveDate) ->  anyhow::Result<Vec<HmDetail>>  {
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<HmDetail>(request!(Api::Custom("hm_detail".into()),
        {"ts_code" => ts_code, "start_date" => start_date, "end_date" => end_date},
        [
                                          "trade_date",
                                          "ts_code",
                                          "ts_name",
                                          "buy_amount",
                                            "sell_amount",
                                            "net_amount",
                                            "hm_name",
                                            "hm_orgs",
                                            "tag"
                                          ])).await?;
    Ok(res.items)
}