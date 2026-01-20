use std::collections::HashMap;

use entity::ths_daily::Model as ThsDaily;

use tushare_api::{Api, fields, params, request, TushareRequest};
use crate::tushare::call_api_as;


/// # 获取同花顺指数数据
///
/// # Arguments
///
/// * `ts_code` - 指数代码, None 取全部
/// * `trade_date` - 交易日
/// * `start_date` - 开始日期
/// * `end_date` - 结束日期
pub async fn ths_daily(ts_code: Option<&str>, trade_date: Option<&str>, start_date: Option<&str>, end_date: Option<&str>) -> anyhow::Result<Vec<ThsDaily>> {
    let mut params = HashMap::new();
    if let Some(ts_code) = ts_code {
        params.insert("ts_code", ts_code);
    }
    if let Some(trade_date) = trade_date {
        params.insert("trade_date", trade_date);
    }
    if let Some(start_date) = start_date {
        params.insert("start_date", start_date);
    }
    if let Some(end_date) = end_date {
        params.insert("end_date", end_date);
    }
    let params = params.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect::<HashMap<String, String>>();
    let fields = fields![
                                          "ts_code",
                                          "trade_date",
                                          "close",
                                          "open",
                                          "high",
                                          "low",
                                          "pre_close",
                                          "avg_price",
                                          "change",
                                          "pct_change",
                                          "vol",
                                          "turnover_rate",
                                          "total_mv",
                                          "float_mv",
                                          ];
    let req = TushareRequest {
        api_name: Api::ThsDaily,
        params,
        fields,
    };
    let res = call_api_as::<ThsDaily>(req).await?;
    Ok(res.items)
}

