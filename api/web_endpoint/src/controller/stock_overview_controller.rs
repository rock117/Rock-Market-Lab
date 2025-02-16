use log::info;
use rocket::get;
use rocket::serde::json::Json;
use entity::sea_orm::DatabaseConnection;
use service::stock;
use service::stock::StockOverView;
use crate::{get_db_conn, init_log_context};
use crate::controller::stock_overview_controller;
use rocket::State;

#[get("/api/stocks")]
pub async fn stock_overview(conn: &State<DatabaseConnection>) -> Json<Vec<StockOverView>> {
    let conn = conn as &DatabaseConnection;
    let data = stock::get_stock_overviews(&conn).await.unwrap();
    Json(data)
}
