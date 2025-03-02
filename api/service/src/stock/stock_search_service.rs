use entity::sea_orm::DatabaseConnection;
use entity::stock;
use super::get_stock_list;

pub async fn search_stocks(keyword: &str, db: &DatabaseConnection) -> anyhow::Result<Vec<stock::Model>> {
    let stocks:Vec<stock::Model> = get_stock_list(db).await?;
    let stocks: Vec<stock::Model> = stocks
        .into_iter()
        .filter(|s| s.name_py.as_ref().map(|v| v.contains(keyword)).unwrap_or(false) || s.ts_code.contains(keyword) || s.name.as_ref().map(|name| name.contains(keyword)).unwrap_or(false))
        .collect();
    Ok(stocks)
}