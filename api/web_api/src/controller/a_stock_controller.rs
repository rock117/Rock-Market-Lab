use rocket::{get, State};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::stock::a_stock_service;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/a-stocks")]
pub async fn get_a_stocks(conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<a_stock_service::AStockOverview>>> {
    info!("获取A股列表请求: /api/a-stocks");
    let conn = conn as &DatabaseConnection;
    let items = a_stock_service::get_all_a_stocks(conn).await?;
    WebResponse::new(items).into_result()
}
