use std::collections::HashMap;

use entity::ths_member::Model as ThsMember;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

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

    call_tushare_api_as::<50, ThsMember>(Api::ths_member,
                                      &params,
                                      &vec![
                                          "ts_code",
                                          "con_code",
                                          "con_name",
                                          "weight",
                                          "in_date",
                                          "out_date",
                                          "is_new"
                                      ]).await
}

