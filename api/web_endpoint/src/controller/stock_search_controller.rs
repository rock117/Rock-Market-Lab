use rocket::{get, State};
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use service::stock::stock_search_service;

#[get("/api/stocks/search?<keyword>")]
pub async fn search_stock(keyword: &str,  conn: &State<DatabaseConnection>) -> Json<WebResponse<Vec<entity::stock::Model>>> {
    let conn = conn as &DatabaseConnection;
    let stocks = stock_search_service::search_stocks(keyword, &conn).await.unwrap();
    Json(WebResponse::new(stocks))
}
