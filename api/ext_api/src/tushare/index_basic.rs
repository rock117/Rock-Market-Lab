use map_macro::hash_map;
use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// 指数列表 https://tushare.pro/document/2?doc_id=94
pub async fn index_basic() -> anyhow::Result<Vec<entity::index::Model>> {
    let params = &hash_map! {};
    let fields = &[
        "ts_code",
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
        "exp_date",
    ];
    call_tushare_api_as::<0, entity::index::Model>(Api::index_basic, params, fields).await
}