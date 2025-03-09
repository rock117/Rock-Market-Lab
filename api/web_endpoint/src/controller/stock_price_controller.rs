use anyhow::anyhow;
use chrono::NaiveDate;
use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use entity::stock_daily;
use service::stock::stock_price_service;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
#[get("/api/stocks/price?<ts_code>&<start>&<end>")]
pub async fn stock_price(ts_code: &str, start: &str, end: &str, conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<stock_daily::Model>>> {
    let conn = conn as &DatabaseConnection;
    let start = NaiveDate::parse_from_str(start, common::date::FORMAT_DASH).map_err(|e| anyhow!(e))?;
    let end = NaiveDate::parse_from_str(end, common::date::FORMAT_DASH).map_err(|e| anyhow!(e))?;
    let data = stock_price_service::get_stock_prices(ts_code, &start, &end, &conn).await?;
    WebResponse::new(data).into_result()
}
