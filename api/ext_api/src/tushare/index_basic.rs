use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// 指数列表 https://tushare.pro/document/2?doc_id=94
pub async fn index_basic() -> anyhow::Result<Vec<entity::index::Model>> {
    let res = call_api_as::<entity::index::Model>(request!(Api::IndexBasic, {}, ["ts_code",
        "name",
        "fullname",
        "market",
        "publisher",
        "index_type",
        "category",
        "base_date",
        "base_point",
        "list_date",
        "weight_rule",
        "desc",
        "exp_date"])).await?;
    Ok(res.items)
}