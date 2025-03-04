use anyhow::anyhow;
use chrono::NaiveDate;
use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use entity::stock_daily;
use crate::response::WebResponse;
use service::security::{security_daily_service, SecurityDaily, SecurityType};
use std::str::FromStr;

#[get("/api/securities/price?<type>&<ts_code>&<start>&<end>")]
pub async fn get_security_price(r#type: &str, ts_code: &str, start: &str, end: &str, conn: &State<DatabaseConnection>) -> Json<WebResponse<Vec<SecurityDaily>>> {
    let t = SecurityType::from_str(r#type).unwrap();
    let conn = conn as &DatabaseConnection;
    let start = NaiveDate::parse_from_str(start, common::date::FORMAT_DASH).map_err(|e| anyhow!(e)).unwrap();
    let end = NaiveDate::parse_from_str(end, common::date::FORMAT_DASH).map_err(|e| anyhow!(e)).unwrap();
    let data = security_daily_service::get_security_daily(t, ts_code, &start, &end, &conn).await.unwrap();
    Json(WebResponse::new(data))
}
