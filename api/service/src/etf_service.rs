use entity::{etf, fund_portfolio};
use entity::sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use entity::sea_orm::sea_query::{Expr, Func};
use entity::sea_orm::QuerySelect;
use entity::stock;
use std::collections::HashMap;

pub async fn get_etf_list(conn: &DatabaseConnection) -> anyhow::Result<Vec<etf::Model>> {
    let items = etf::Entity::find()
        .order_by_asc(etf::Column::TsCode)
        .all(conn)
        .await?;
    Ok(items)
}

pub async fn get_stock_name_map(
    conn: &DatabaseConnection,
    ts_codes: Vec<String>,
) -> anyhow::Result<HashMap<String, Option<String>>> {
    if ts_codes.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = stock::Entity::find()
        .select_only()
        .column(stock::Column::TsCode)
        .column(stock::Column::Name)
        .filter(stock::Column::TsCode.is_in(ts_codes))
        .into_tuple::<(String, Option<String>)>()
        .all(conn)
        .await?;

    Ok(rows.into_iter().collect())
}

pub async fn search_etfs(conn: &DatabaseConnection, keyword: &str) -> anyhow::Result<Vec<etf::Model>> {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        return Ok(vec![]);
    }

    let keyword = keyword.to_lowercase();
    let pattern = format!("%{}%", keyword);

    let condition = Condition::any()
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::TsCode))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::Csname))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::Cname))).like(&pattern));

    let items = etf::Entity::find()
        .filter(condition)
        .order_by_asc(etf::Column::TsCode)
        .limit(50)
        .all(conn)
        .await?;

    Ok(items)
}

pub async fn get_etf_holdings_latest(
    conn: &DatabaseConnection,
    ts_code: &str,
) -> anyhow::Result<Vec<fund_portfolio::Model>> {
    let latest = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .order_by_desc(fund_portfolio::Column::EndDate)
        .one(conn)
        .await?;

    let Some(latest) = latest else {
        return Ok(vec![]);
    };

    let items = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .filter(ColumnTrait::eq(
            &fund_portfolio::Column::EndDate,
            latest.end_date.clone(),
        ))
        .order_by_desc(fund_portfolio::Column::Mkv)
        .all(conn)
        .await?;

    Ok(items)
}
