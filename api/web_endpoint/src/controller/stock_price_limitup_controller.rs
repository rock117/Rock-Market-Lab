use rocket::{FromForm, get, State};
use rocket::serde::json::Json;
use serde_derive::Deserialize;
use tracing::info;
use entity::sea_orm::DatabaseConnection;
use service::stock::filter::*;
use crate::response::WebResponse;
use service::stock::filter::stock_price_limit_service::*;

// #[derive(Debug, Deserialize, FromForm)]
// pub struct FilterParams {
//
// }

#[get("/api/stocks/filter?<past_ndays>")]
pub async fn stock_price_limitup(past_ndays: u64, conn: &State<DatabaseConnection>) -> Json<WebResponse<LimitupStocks>> {
    let conn = conn as &DatabaseConnection;
    let data = stock_price_limit_service::filter_continue_price_limit(past_ndays, &conn).await.unwrap();
    Json(WebResponse::new(data))
}
