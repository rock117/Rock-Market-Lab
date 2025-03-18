use anyhow::anyhow;
use chrono::NaiveDate;
use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use service::security::{security_daily_service, SecurityPrice, SecurityType};
use std::str::FromStr;
use crate::result::IntoResult;

#[get("/api/securities/price?<type>&<ts_code>&<start>&<end>")]
pub async fn get_security_price(r#type: &str, ts_code: &str, start: &str, end: &str, conn: &State<DatabaseConnection>) ->  crate::result::Result<WebResponse<Vec<SecurityPrice>>> {
    let t = SecurityType::from_str(r#type)?;
    let conn = conn as &DatabaseConnection;
    let start = NaiveDate::parse_from_str(start, common::date::FORMAT_DASH).map_err(|e| anyhow!(e))?;
    let end = NaiveDate::parse_from_str(end, common::date::FORMAT_DASH).map_err(|e| anyhow!(e))?;
    let data = security_daily_service::get_security_daily(t, ts_code, &start, &end, &conn).await?;
    WebResponse::new(data).into_result()
}