use chrono::{Local, Months, NaiveDate};
use rocket::{post, State};
use rocket::serde::json::{Json, Value as JsonValue};
use serde::{Deserialize, Serialize};
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use service::stock_picker_service::*;
use crate::result::IntoResult;

/// 选股请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPickRequest {
    /// 策略类型
    pub strategy: String,
    // TODO add concepts
    /// 策略设置（动态字段，根据 type 不同而不同）
    /// 使用 JsonValue 来接收任意 JSON 对象
    pub settings: Option<JsonValue>,
}

/// 选股响应
#[derive(Debug, Clone, Serialize)]
pub struct StockPickResponse {
    /// 筛选出的股票列表
    pub stocks: Vec<StockPickResult>,
    /// 总数量
    pub total: usize,
    /// 使用的策略类型
    pub strategy_type: String,
}

/// 简单选股接口（使用默认配置）
#[post("/api/stocks/pick", data = "<request>")]
pub async fn pick(conn: &State<DatabaseConnection>,   request: Json<StockPickRequest>,) -> crate::result::Result<WebResponse<Vec<StockPickResult>>> {
    let conn = conn as &DatabaseConnection;

    let picker_service = StockPickerService::new(conn.clone());
    let end = Local::now().date_naive();
    let start = end.checked_sub_months(Months::new(3)).unwrap();
    let req = request.into_inner();
    let strategy = req.strategy;
    let settings = req.settings;

    let datas = picker_service.pick_stocks(&start, &end, &strategy, settings).await?;
    WebResponse::new(datas).into_result()
}
