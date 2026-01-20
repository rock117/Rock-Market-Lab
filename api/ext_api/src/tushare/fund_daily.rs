use chrono::NaiveDate;
use map_macro::hash_map;

use entity::fund_daily::Model as FundDaily;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;


/// 基金日线行情行情
pub async fn fund_daily(tscode: &str, start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<FundDaily>> {
    let start_date = start.format("%Y%m%d").to_string();
    let end_date = end.format("%Y%m%d").to_string();
    let parmas = hash_map! {"ts_code".into() => tscode.into(), "start_date".into() => start_date, "end_date".into() => end_date};
    let fields = fields!["ts_code",
                                              "trade_date",
                                              "open",
                                              "high",
                                              "low",
                                              "close",
                                              "pre_close",
                                              "change",
                                              "pct_chg",
                                              "vol",
                                              "amount"];
    let request = TushareRequest {
        api_name: Api::FundDaily,
        params: parmas,
        fields,
    };
    let res = call_api_as::<FundDaily>(request).await?;
    Ok(res.items)
}
