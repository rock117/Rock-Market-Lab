use anyhow::anyhow;
use chrono::NaiveDate;
use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use entity::stock_daily;
use service::stock::stock_price_service;
use crate::response::WebResponse;

#[get("/api/stocks/price?<ts_code>&<start>&<end>")]
pub async fn stock_price(ts_code: &str, start: &str, end: &str, conn: &State<DatabaseConnection>) -> Json<WebResponse<Vec<stock_daily::Model>>> {
    let conn = conn as &DatabaseConnection;
    let start = NaiveDate::parse_from_str(start, common::date::FORMAT_DASH).map_err(|e| anyhow!(e)).unwrap();
    let end = NaiveDate::parse_from_str(end, common::date::FORMAT_DASH).map_err(|e| anyhow!(e)).unwrap();
    let data = stock_price_service::get_stock_prices(ts_code, &start, &end, &conn).await.unwrap();
    Json(WebResponse::new(data))
}
