use rocket::{get};
use rocket::State;
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::stock;
use service::stock::stock_overview_service::*;

use crate::{get_db_conn, init_log_context};
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
use common::web::request::StockQueryParams;
#[get("/api/stocks?<params..>")]
pub async fn stock_overview(params: StockQueryParams, conn: &State<DatabaseConnection>) -> Result<WebResponse<StockOverviewResponse>> {
    info!("stock_overview params: {:?}", params);
    let conn = conn as &DatabaseConnection;
    let data = get_stock_overviews(&params, &conn).await?;
    WebResponse::new(data).into_result()
}
