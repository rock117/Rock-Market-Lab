use rocket::{get, post, State};
use rocket::serde::json::Json;
use serde::Deserialize;

use entity::sea_orm::DatabaseConnection;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[derive(Debug, Deserialize)]
pub struct DcIndexQueryRequest {
    pub trade_dates: Vec<String>,
}

#[get("/api/dc_index")]
pub async fn list_dc_index_latest_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<entity::dc_index::Model>>> {
    let conn = conn as &DatabaseConnection;
    let rows = service::dc_service::list_dc_index_latest(conn).await?;
    WebResponse::new(rows).into_result()
}

#[get("/api/dc_index/trade_dates")]
pub async fn list_dc_index_trade_dates_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<String>>> {
    let conn = conn as &DatabaseConnection;
    let rows = service::dc_service::list_dc_index_trade_dates(conn).await?;
    WebResponse::new(rows).into_result()
}

#[post("/api/dc_index/query", data = "<request>")]
pub async fn query_dc_index_handler(
    request: Json<DcIndexQueryRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<entity::dc_index::Model>>> {
    let conn = conn as &DatabaseConnection;
    let rows = service::dc_service::list_dc_index_by_trade_dates(conn, &request.trade_dates).await?;
    WebResponse::new(rows).into_result()
}

#[get("/api/dc_index/<ts_code>/members?<trade_date>")]
pub async fn list_dc_members_handler(
    ts_code: &str,
    trade_date: &str,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<entity::dc_member::Model>>> {
    let conn = conn as &DatabaseConnection;
    let rows = service::dc_service::list_dc_members_by_concept(conn, ts_code, trade_date).await?;
    WebResponse::new(rows).into_result()
}
