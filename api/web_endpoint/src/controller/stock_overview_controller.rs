use rocket::{FromForm, get};
use rocket::http::uri::Query;
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::stock;
use service::stock::stock_overview_service::*;
use crate::{get_db_conn, init_log_context};
use crate::controller::stock_overview_controller;
use rocket::State;
use serde_derive::Deserialize;
use tracing::info;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[derive(Debug, Deserialize, FromForm)]
struct StockQueryParams {
    page: usize,
    page_size: usize,
    order_by: String,
    order: String, // prop: 'pct_chg', order: 'ascending' descending
}

#[get("/api/stocks?<params..>")]
pub async fn stock_overview(params: StockQueryParams, conn: &State<DatabaseConnection>) -> Result<WebResponse<StockOverviewResponse>> {
    info!("stock_overview params: {:?}", params);
    let conn = conn as &DatabaseConnection;
    let data = get_stock_overviews(params.page, params.page_size, &params.order_by, &params.order, &conn).await?;
    WebResponse::new(data).into_result()
}
