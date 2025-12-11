use std::collections::HashSet;
use anyhow::anyhow;
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;
use entity::stock;
use entity::sea_orm::EntityOrSelect;
use entity::sea_orm::QuerySelect;

mod stock_filter_service;
pub mod stock_overview_service;
pub mod filter;
pub mod stock_bias_ratio_service;
pub mod stock_search_service;
pub mod stock_price_service;
pub mod volume_distribution_service;

pub async fn get_stock(ts_code: &str, conn: &DatabaseConnection) -> anyhow::Result<stock::Model> {
    let data = stock::Entity::find_by_id(ts_code).one(conn).await;
    match data {
        Err(err) => anyhow::bail!("get stock failed, ts_code: {}, error: {:?}", ts_code, err),
        Ok(data) => data.ok_or(anyhow!("stock not found, ts_code: {}", ts_code)),
    }
}

pub async fn get_stock_list(conn: &DatabaseConnection) -> anyhow::Result<Vec<stock::Model>> {
    stock::Entity::find().all(conn).await.map_err(|err| anyhow!("get stock list failed, error: {:?}", err))
}

pub async fn get_stock_area_list(conn: &DatabaseConnection) -> anyhow::Result<HashSet<String>> {
    let areas: Vec<stock::Model> = stock::Entity::find().all(conn).await.map_err(|err| anyhow!("get stock area list failed, error: {:?}", err))?;
    println!("areas num: {}", areas.len());
    let areas = areas.into_iter().filter(|v| v.area.is_some()).map(|v| v.area.or(Some("null".into()))).collect::<Option<HashSet<String>>>();
    areas.ok_or(anyhow!("get stock area list failed"))
}

pub async fn get_stock_industry_list(conn: &DatabaseConnection) -> anyhow::Result<HashSet<String>> {
    let industries: Vec<stock::Model> = stock::Entity::find().all(conn).await.map_err(|err| anyhow!("get stock industry list failed, error: {:?}", err))?;
    println!("industries num: {}", industries.len());
    let industries = industries.into_iter().map(|v| v.industry.or(Some("null".into()))).collect::<Option<HashSet<String>>>();
    industries.ok_or(anyhow!("get stock industry list failed"))
}