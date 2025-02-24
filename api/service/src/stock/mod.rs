use anyhow::anyhow;

use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;
use entity::stock;

mod stock_filter_service;
pub mod stock_overview_service;
pub mod filter;

pub async fn get_stock(ts_code: &str, conn: &DatabaseConnection) -> anyhow::Result<stock::Model> {
    let data = stock::Entity::find_by_id(ts_code).one(conn).await;
    match data {
        Err(err) => anyhow::bail!("get stock failed, ts_code: {}, error: {:?}", ts_code, err),
        Ok(data) => data.ok_or(anyhow!("stock not found, ts_code: {}", ts_code)),
    }
}

