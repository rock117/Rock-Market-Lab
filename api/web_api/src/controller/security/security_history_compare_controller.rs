use std::collections::HashMap;
use entity::sea_orm::DatabaseConnection;
use service::security::{SecurityPrice, SecurityType, Year};
use rocket::serde::json::Json;
use crate::response::WebResponse;
use rocket::{post, State};
use serde_derive::Deserialize;
use common::data_type::period::Period;
use service::security::security_history_compare_service;
use crate::result::{IntoResult, Result};

#[derive(Deserialize)]
struct HistoryQuery {
    #[serde(rename = "tsCode")]
    ts_code: String,
    r#type: SecurityType,
    years: Vec<Year>,
    period: Period,
}

#[post("/api/securities/history/compare", format = "json", data = "<query>")]
pub async fn security_history_compare(query: Json<HistoryQuery>, conn: &State<DatabaseConnection>) -> Result<WebResponse<HashMap<Year, Vec<SecurityPrice>>>> {
    let conn = conn as &DatabaseConnection;
    let datas = security_history_compare_service::get_security_by_years(query.r#type, &query.ts_code, query.period, &query.years, &conn).await?;
    WebResponse::new(datas).into_result()
}
