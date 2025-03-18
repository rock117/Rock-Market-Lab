use entity::sea_orm::DatabaseConnection;
use service::stock;

pub async fn get_stock_areas(conn: &DatabaseConnection) -> anyhow::Result<Vec<String>> {
    stock::get_stock_area_list(conn).await
}

pub async fn get_stock_industries(conn: &DatabaseConnection) -> anyhow::Result<Vec<String>> {
    stock::get_stock_industry_list(conn).await
}
