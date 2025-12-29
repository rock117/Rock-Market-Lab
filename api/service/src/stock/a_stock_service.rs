use std::collections::HashMap;

use anyhow::anyhow;
use entity::sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use entity::{stock, stock_daily, stock_daily_basic};
use entity::cn_security_info;
use entity::sea_orm::prelude::Decimal;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AStockOverview {
    pub ts_code: String,
    pub name: String,
    pub name_py: Option<String>,
    pub list_date: Option<String>,
    pub close: Option<Decimal>,
    pub pct_chg: Option<Decimal>,
    pub pct5: Option<Decimal>,
    pub pct10: Option<Decimal>,
    pub pct20: Option<Decimal>,
    pub pct60: Option<Decimal>,
    pub concepts: Option<String>,
    pub pe: Option<Decimal>,
    pub dv_ratio: Option<Decimal>,
    pub total_mv: Option<Decimal>,
}

fn calc_period_pct_chg(closes_desc: &[Decimal], days: usize) -> Option<Decimal> {
    // closes_desc: [today, yesterday, ...] (desc)
    // days: window size including today, e.g. days=5 means [day1..day5] => (day5-day1)/day1
    if days < 2 {
        return None;
    }
    if closes_desc.len() < days {
        return None;
    }
    let today = closes_desc.get(0).copied()?;
    let past = closes_desc.get(days - 1).copied()?;
    if past.is_zero() {
        return None;
    }
    Some((today - past) / past * Decimal::from(100i64))
}

pub async fn get_all_a_stocks(conn: &DatabaseConnection) -> anyhow::Result<Vec<AStockOverview>> {
    // 1) 股票基础信息
    let stocks = stock::Entity::find().all(conn).await?;
    if stocks.is_empty() {
        return Ok(vec![]);
    }

    // 1.1) 概念信息（cn_security_info.concepts）
    let ts_codes: Vec<String> = stocks.iter().map(|s| s.ts_code.clone()).collect();
    let cn_infos = cn_security_info::Entity::find()
        .filter(<cn_security_info::Column as ColumnTrait>::is_in(
            &cn_security_info::Column::Secucode,
            ts_codes.clone(),
        ))
        .all(conn)
        .await?;
    let concepts_map: std::collections::HashMap<String, Option<String>> = cn_infos
        .into_iter()
        .map(|m| (m.secucode, m.concepts))
        .collect();

    // 2) 全市场最新交易日（用于取 close/pct_chg 和 pe/dv_ratio/total_mv）
    let latest_trade_date: Option<String> = stock_daily::Entity::find()
        .select_only()
        .column(stock_daily::Column::TradeDate)
        .order_by_desc(stock_daily::Column::TradeDate)
        .limit(1)
        .into_tuple::<String>()
        .one(conn)
        .await?;

    let latest_trade_date = latest_trade_date.ok_or(anyhow!("stock_daily is empty"))?;

    // 3) 最近60个交易日（distinct）用于计算MA
    let last_60_dates: Vec<String> = stock_daily::Entity::find()
        .select_only()
        .column(stock_daily::Column::TradeDate)
        .distinct()
        .order_by_desc(stock_daily::Column::TradeDate)
        .limit(65)
        .into_tuple::<String>()
        .all(conn)
        .await?;

    // 4) 最新日线（close/pct_chg）
    let latest_dailies = stock_daily::Entity::find()
        .filter(<stock_daily::Column as ColumnTrait>::eq(
            &stock_daily::Column::TradeDate,
            latest_trade_date.clone(),
        ))
        .all(conn)
        .await?;
    let latest_daily_map: HashMap<String, stock_daily::Model> = latest_dailies
        .into_iter()
        .map(|m| (m.ts_code.clone(), m))
        .collect();

    // 5) 最新每日指标（pe/dv_ratio/total_mv）
    let latest_basics = stock_daily_basic::Entity::find()
        .filter(<stock_daily_basic::Column as ColumnTrait>::eq(
            &stock_daily_basic::Column::TradeDate,
            latest_trade_date.clone(),
        ))
        .all(conn)
        .await?;
    let latest_basic_map: HashMap<String, stock_daily_basic::Model> = latest_basics
        .into_iter()
        .map(|m| (m.ts_code.clone(), m))
        .collect();

    // 6) 近60日 close 序列（按 trade_date desc 排序）
    let ma_dailies = stock_daily::Entity::find()
        .filter(<stock_daily::Column as ColumnTrait>::is_in(
            &stock_daily::Column::TradeDate,
            last_60_dates,
        ))
        .order_by_desc(stock_daily::Column::TradeDate)
        .all(conn)
        .await?;

    let mut closes_map: HashMap<String, Vec<Decimal>> = HashMap::new();
    for d in ma_dailies {
        closes_map.entry(d.ts_code.clone()).or_default().push(d.close);
    }

    // 7) 组装返回
    let mut items: Vec<AStockOverview> = Vec::with_capacity(stocks.len());
    for s in stocks {
        let ts_code = s.ts_code.clone();
        let name = s.name.clone().unwrap_or_else(|| "-".to_string());

        let (close, pct_chg) = match latest_daily_map.get(&ts_code) {
            Some(d) => (Some(d.close), d.pct_chg),
            None => (None, None),
        };

        let (pe, dv_ratio, total_mv) = match latest_basic_map.get(&ts_code) {
            Some(b) => (b.pe, b.dv_ratio, b.total_mv),
            None => (None, None, None),
        };

        let closes_desc = closes_map.get(&ts_code).map(|v| v.as_slice()).unwrap_or(&[]);

        items.push(AStockOverview {
            ts_code: ts_code.clone(),
            name,
            name_py: s.name_py,
            list_date: s.list_date,
            close,
            pct_chg,
            pct5: calc_period_pct_chg(closes_desc, 5),
            pct10: calc_period_pct_chg(closes_desc, 10),
            pct20: calc_period_pct_chg(closes_desc, 20),
            pct60: calc_period_pct_chg(closes_desc, 60),
            concepts: concepts_map.get(&ts_code).cloned().flatten(),
            pe,
            dv_ratio,
            total_mv,
        });
    }

    items.sort_by(|a, b| a.ts_code.cmp(&b.ts_code));
    Ok(items)
}
