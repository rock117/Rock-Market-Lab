use std::collections::HashMap;
use common::domain::ThsIndexType;

use entity::ths_index::Model as ThsIndex;

use crate::tushare::call_tushare_api_as;
use crate::tushare::model::Api;

/// # 获取同花顺指数数据
///
/// # Arguments
///
/// * `ts_code` - 指数代码, None 取全部
/// * `exchange` - 市场类型A-a股 HK-港股 US-美股
/// * `type_` -  指数类型 N-概念指数 I-行业指数 R-地域指数 S-同花顺特色指数 ST-同花顺风格指数 TH-同花顺主题指数 BB-同花顺宽基指数
pub async fn ths_index(ts_code: Option<&str>, exchange: Option<&str>, type_: Option<ThsIndexType>) -> anyhow::Result<Vec<ThsIndex>> {
    let mut params = HashMap::new();
    if let Some(ts_code) = ts_code {
        params.insert("ts_code", ts_code);
    }
    if let Some(exchange) = exchange {
        params.insert("exchange", exchange);
    }
    if let Some(type_) = type_ {
        let t = type_.as_str();
        params.insert("type", t);
    }

    call_tushare_api_as::<500, ThsIndex>(Api::ths_index,
                                      &params,
                                      &vec![
                                          "ts_code",
                                          "name",
                                          "count",
                                          "exchange",
                                          "list_date",
                                          "type"
                                      ]).await
}

