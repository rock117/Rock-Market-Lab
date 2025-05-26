use std::collections::HashSet;
use rocket::{get, post, State};
use entity::sea_orm::DatabaseConnection;
use service::stock;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/stock/areas")]
pub async fn get_stock_areas(conn: &State<DatabaseConnection>) -> Result<WebResponse<HashSet<String>>> {
    let conn = conn as &DatabaseConnection;
    WebResponse::new(stock::get_stock_area_list(conn).await?).into_result()
}

#[get("/api/stock/industries")]
pub async fn get_stock_industries(conn: &State<DatabaseConnection>) -> Result<WebResponse<HashSet<String>>> {
    let conn = conn as &DatabaseConnection;
    WebResponse::new(stock::get_stock_industry_list(conn).await?).into_result()
}