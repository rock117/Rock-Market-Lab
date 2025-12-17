use rocket::{get, State};
use entity::sea_orm::DatabaseConnection;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
use service::stock::stock_search_service;
use service::stock::stock_search_service::StockSearchItem;

#[get("/api/stocks/search?<keyword>")]
pub async fn search_stocks(keyword: &str, conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<StockSearchItem>>> {
    let conn = conn as &DatabaseConnection;
    let stocks = stock_search_service::search_stocks(keyword, conn).await?;
    WebResponse::new(stocks).into_result()
}
