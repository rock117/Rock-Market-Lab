use rocket::{get, State};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::stock::holder_per_capita_service;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/holder-per-capita")]
pub async fn get_holder_per_capita(conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<holder_per_capita_service::HolderPerCapitaItem>>> {
    info!("获取人均持股列表请求: /api/holder-per-capita");
    let conn = conn as &DatabaseConnection;
    let items = holder_per_capita_service::get_holder_per_capita_list(conn).await?;
    WebResponse::new(items).into_result()
}
