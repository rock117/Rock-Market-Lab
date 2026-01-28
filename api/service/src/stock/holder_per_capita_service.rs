use std::collections::HashMap;

use entity::sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::{stock, stock_daily_basic, stock_holder_number};
use entity::sea_orm::prelude::Decimal;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HolderPerCapitaItem {
    pub ts_code: String,
    pub name: String,
    pub holder_num: Option<i32>,
    pub total_mv: Option<f64>,
    pub circ_mv: Option<f64>,
    pub per_capita_mv: Option<f64>,
    pub per_capita_share: Option<f64>,
    pub end_date: Option<String>,
    pub close: Option<f64>,
    pub total_share: Option<f64>,
}

pub async fn get_holder_per_capita_list(conn: &DatabaseConnection) -> anyhow::Result<Vec<HolderPerCapitaItem>> {
    let stocks = stock::Entity::find().all(conn).await?;
    if stocks.is_empty() {
        return Ok(vec![]);
    }

    let latest_trade_date: Option<String> = stock_daily_basic::Entity::find()
        .select_only()
        .column(stock_daily_basic::Column::TradeDate)
        .order_by_desc(stock_daily_basic::Column::TradeDate)
        .limit(1)
        .into_tuple::<String>()
        .one(conn)
        .await?;

    let latest_trade_date = match latest_trade_date {
        Some(d) => d,
        None => return Ok(vec![]),
    };

    let basics = stock_daily_basic::Entity::find()
        .filter(<stock_daily_basic::Column as ColumnTrait>::eq(
            &stock_daily_basic::Column::TradeDate,
            latest_trade_date.clone(),
        ))
        .all(conn)
        .await?;

    let basic_map: HashMap<String, stock_daily_basic::Model> = basics
        .into_iter()
        .map(|m| (m.ts_code.clone(), m))
        .collect();

    let holders = stock_holder_number::Entity::find()
        .order_by_desc(stock_holder_number::Column::EndDate)
        .all(conn)
        .await?;

    let mut holder_map: HashMap<String, stock_holder_number::Model> = HashMap::new();
    for h in holders {
        holder_map.entry(h.ts_code.clone()).or_insert(h);
    }

    let mut items: Vec<HolderPerCapitaItem> = Vec::with_capacity(stocks.len());

    for s in stocks {
        let ts_code = s.ts_code.clone();
        let name = s.name.clone().unwrap_or_else(|| "-".to_string());

        let basic = basic_map.get(&ts_code);
        let holder = holder_map.get(&ts_code);

        let total_mv = basic.and_then(|b| b.total_mv).map(decimal_to_f64);
        let circ_mv = basic.and_then(|b| b.circ_mv).map(decimal_to_f64);
        let close = basic.and_then(|b| b.close).map(decimal_to_f64);
        let total_share = basic.and_then(|b| b.total_share).map(decimal_to_f64);

        let holder_num = holder.and_then(|h| h.holder_num);
        let end_date = holder.map(|h| h.end_date.clone());

        let per_capita_mv = match (total_mv, holder_num) {
            (Some(mv), Some(num)) if num > 0 => Some(mv * 10000.0 / num as f64),
            _ => None,
        };

        let per_capita_share = match (total_share, holder_num) {
            (Some(share), Some(num)) if num > 0 => Some(share * 10000.0 / num as f64),
            _ => None,
        };

        items.push(HolderPerCapitaItem {
            ts_code,
            name,
            holder_num,
            total_mv,
            circ_mv,
            per_capita_mv,
            per_capita_share,
            end_date,
            close,
            total_share,
        });
    }

    items.sort_by(|a, b| {
        let a_val = a.per_capita_mv.unwrap_or(f64::MIN);
        let b_val = b.per_capita_mv.unwrap_or(f64::MIN);
        b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(items)
}

fn decimal_to_f64(d: Decimal) -> f64 {
    use std::str::FromStr;
    f64::from_str(&d.to_string()).unwrap_or(0.0)
}
