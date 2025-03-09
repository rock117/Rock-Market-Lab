use std::collections::HashMap;
use entity::sea_orm::DatabaseConnection;
use service::security::{SecurityMonthly, SecurityType, Year};
use rocket::serde::json::Json;
use crate::response::WebResponse;
use rocket::{post, State};
use serde_derive::Deserialize;
use service::security::security_monthly_service;

#[derive(Deserialize)]
struct HistoryQuery {
    ts_code: String,
    r#type: SecurityType,
    years: Vec<Year>
}

// #[post("/api/securities/history", format = "json", data = "<query>")]
// pub async fn security_history(query: Json<HistoryQuery>, conn: &State<DatabaseConnection>) -> anyhow::Result<Json<WebResponse<HashMap<Year, Vec<SecurityMonthly>>>>> {
//     let conn = conn as &DatabaseConnection;
//     let datas = security_monthly_service::get_security_monthly_by_years(query.r#type, &query.ts_code, &query.years, &conn).await?;
//     Ok(Json(WebResponse::new(datas)))
// }
