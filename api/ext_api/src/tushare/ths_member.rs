use std::collections::HashMap;

use entity::ths_member::Model as ThsMember;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;

/// # 获取同花顺指数数据
///
/// # Arguments
///
/// * `ts_code` - 板块代码代码, None 取全部
/// * `conn_code` - 股票代码
pub async fn ths_member(ts_code: Option<&str>, conn_code: Option<&str>) -> anyhow::Result<Vec<ThsMember>> {
    let mut params = HashMap::new();
    if let Some(ts_code) = ts_code {
        params.insert("ts_code", ts_code);
    }
    if let Some(conn_code) = conn_code {
        params.insert("conn_code", conn_code);
    }
    let params = params.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<HashMap<String, String>>();
    let fields = fields![
                                           "ts_code",
                                          "con_code",
                                          "con_name",
                                          "weight",
                                          "in_date",
                                          "out_date",
                                          "is_new"
                                          ];
    let req = TushareRequest {
        api_name: Api::ThsMember,
        params,
        fields,
    };
    let res = call_api_as::<ThsMember>(req).await?;
    Ok(res.items)
}

