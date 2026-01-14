use chrono::NaiveDate;

use entity::finance_main_business::Model as FinanceMainBusiness;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 主营业务构成
/// 类型：P按产品 D按地区 I按行业（请输入大写字母P或者D）
pub async fn fina_mainbz(ts_code: &str, type_: &str, start_date: &NaiveDate, end_date: &NaiveDate) -> anyhow::Result<Vec<FinanceMainBusiness>>{
    let start_date = start_date.format("%Y%m%d").to_string();
    let end_date = end_date.format("%Y%m%d").to_string();
    let res = call_api_as::<FinanceMainBusiness, 500>(request!(Api::FinaMainbz,
                       {"ts_code" => ts_code, "type" => type_, "start_date" => start_date.as_str(), "end_date" => end_date.as_str()},
                        [
                            "ts_code",
                            "end_date",
                            "bz_item",
                            "bz_sales",
                            "bz_profit",
                            "bz_cost",
                            "curr_type",
                            "update_flag",
                        ])).await?;
    return Ok(res.items);
}