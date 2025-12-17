use entity::sea_orm::DatabaseConnection;
use entity::sea_orm as sea_orm;
use entity::stock;
use entity::sea_orm::{ColumnTrait, Condition, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};
use entity::sea_orm::sea_query::{Expr, Func};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct StockSearchItem {
    pub ts_code: String,
    pub name: Option<String>,
}

pub async fn search_stocks(keyword: &str, db: &DatabaseConnection) -> anyhow::Result<Vec<StockSearchItem>> {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        return Ok(vec![]);
    }

    let keyword = keyword.to_lowercase();
    let pattern = format!("%{}%", keyword);

    let condition = Condition::any()
        .add(Expr::expr(Func::lower(Expr::col(stock::Column::TsCode))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(stock::Column::Name))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(stock::Column::NamePy))).like(&pattern));

    let stocks = stock::Entity::find()
        .select_only()
        .column(stock::Column::TsCode)
        .column(stock::Column::Name)
        .filter(condition)
        .into_model::<StockSearchItem>()
        .all(db)
        .await?;
    Ok(stocks)
}