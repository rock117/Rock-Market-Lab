use anyhow::Result;
use chrono::NaiveDate;
use entity::margin;
use entity::margin_detail;
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use rust_decimal::prelude::ToPrimitive;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct MarginBalancePoint {
    pub date: String,
    pub margin_balance: f64,
}

pub async fn get_margin_balance(
    conn: &DatabaseConnection,
    exchange: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<MarginBalancePoint>> {
    let start = start_date.format("%Y%m%d").to_string();
    let end = end_date.format("%Y%m%d").to_string();

    let mut query = margin::Entity::find()
        .filter(margin::Column::TradeDate.gte(start))
        .filter(margin::Column::TradeDate.lte(end));

    if exchange != "ALL" {
        query = query.filter(ColumnTrait::eq(&margin::Column::ExchangeId, exchange));
    }

    let rows = query.all(conn).await?;

    let mut by_date: BTreeMap<String, f64> = BTreeMap::new();
    for row in rows {
        let v = row.rzye.and_then(|d| d.to_f64()).unwrap_or(0.0);
        *by_date.entry(row.trade_date).or_insert(0.0) += v;
    }

    Ok(by_date
        .into_iter()
        .map(|(date, margin_balance)| MarginBalancePoint { date, margin_balance })
        .collect())
}

pub async fn get_stock_margin_balance(
    conn: &DatabaseConnection,
    ts_code: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<MarginBalancePoint>> {
    let start = start_date.format("%Y%m%d").to_string();
    let end = end_date.format("%Y%m%d").to_string();

    let rows = margin_detail::Entity::find()
        .filter(ColumnTrait::eq(&margin_detail::Column::TsCode, ts_code))
        .filter(margin_detail::Column::TradeDate.gte(start))
        .filter(margin_detail::Column::TradeDate.lte(end))
        .order_by_asc(margin_detail::Column::TradeDate)
        .all(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| MarginBalancePoint {
            date: row.trade_date,
            margin_balance: row.rqye.and_then(|d| d.to_f64()).unwrap_or(0.0),
        })
        .collect())
}
