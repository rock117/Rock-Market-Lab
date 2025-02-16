use axum::extract::State;
use entity::sea_orm::DatabaseConnection;
use service::stock;
use service::stock::StockOverView;
use crate::domain::ToAppResult;

pub async fn stock_overview(State(conn): State<DatabaseConnection>) -> crate::result::Result<Vec<StockOverView>> {
    stock::get_stock_overviews(&conn).await.to_app_result()
}

pub async fn stock_overview2(conn: DatabaseConnection) -> crate::result::Result<Vec<StockOverView>> {
    stock::get_stock_overviews(&conn).await.to_app_result()
}