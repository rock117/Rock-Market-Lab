use rocket::{get, State};

use entity::sea_orm::DatabaseConnection;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/dc_index")]
pub async fn list_dc_index_latest_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<entity::dc_index::Model>>> {
    let conn = conn as &DatabaseConnection;
    let rows = service::dc_service::list_dc_index_latest(conn).await?;
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
