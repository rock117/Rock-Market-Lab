use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{Datelike, Duration, Local, Months, NaiveDate};
use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use entity::{stock_daily, stock_daily_basic};
use rust_decimal::prelude::ToPrimitive;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockHistoryPoint {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pct_chg: f64,
    pub turnover_rate: f64,
}

#[derive(Debug, Clone)]
pub enum TimePeriodUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone)]
pub struct TimePeriod {
    pub value: u32,
    pub unit: TimePeriodUnit,
}

pub fn parse_time_period(s: &str) -> anyhow::Result<TimePeriod> {
    let s = s.trim();
    if s.is_empty() {
        return Err(anyhow!("time_period is empty"));
    }

    let (num_part, unit_part) = s.split_at(s.len() - 1);
    let value: u32 = num_part
        .parse()
        .map_err(|_| anyhow!("invalid time_period number: {}", s))?;

    let unit = match unit_part {
        "d" | "D" => TimePeriodUnit::Day,
        "w" | "W" => TimePeriodUnit::Week,
        "m" | "M" => TimePeriodUnit::Month,
        "y" | "Y" => TimePeriodUnit::Year,
        _ => return Err(anyhow!("invalid time_period unit: {}", s)),
    };

    Ok(TimePeriod { value, unit })
}

fn format_yyyymmdd(d: &NaiveDate) -> String {
    d.format("%Y%m%d").to_string()
}

fn parse_trade_date_yyyymmdd(s: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|e| anyhow!("invalid trade_date {}: {}", s, e))
}

fn format_dash(d: &NaiveDate) -> String {
    d.format(common::date::FORMAT_DASH).to_string()
}

pub fn resolve_date_range_from_period(period: &TimePeriod) -> (NaiveDate, NaiveDate) {
    let end = Local::now().date_naive();

    let start = match period.unit {
        TimePeriodUnit::Day => end - Duration::days(period.value as i64),
        TimePeriodUnit::Week => end - Duration::days((period.value as i64) * 7),
        TimePeriodUnit::Month => {
            let months = Months::new(period.value);
            end.checked_sub_months(months).unwrap_or_else(|| {
                // fallback: roughly 30d * n
                end - Duration::days((period.value as i64) * 30)
            })
        }
        TimePeriodUnit::Year => {
            let months = Months::new(period.value.saturating_mul(12));
            end.checked_sub_months(months).unwrap_or_else(|| {
                // fallback: roughly 365d * n
                end - Duration::days((period.value as i64) * 365)
            })
        }
    };

    // Keep start <= end
    let start = NaiveDate::from_ymd_opt(start.year(), start.month(), start.day()).unwrap_or(start);

    (start, end)
}

pub async fn get_stock_history(
    conn: &DatabaseConnection,
    ts_code: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> anyhow::Result<Vec<StockHistoryPoint>> {
    let start = format_yyyymmdd(start_date);
    let end = format_yyyymmdd(end_date);

    let daily_rows = stock_daily::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily::Column::TsCode, ts_code))
        .filter(stock_daily::Column::TradeDate.gte(start.clone()))
        .filter(stock_daily::Column::TradeDate.lte(end.clone()))
        .order_by_asc(stock_daily::Column::TradeDate)
        .all(conn)
        .await?;

    let basic_rows = stock_daily_basic::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily_basic::Column::TsCode, ts_code))
        .filter(stock_daily_basic::Column::TradeDate.gte(start))
        .filter(stock_daily_basic::Column::TradeDate.lte(end))
        .all(conn)
        .await?;

    let mut turnover_by_date: HashMap<String, f64> = HashMap::new();
    for r in basic_rows {
        let tr = r.turnover_rate.and_then(|d| d.to_f64()).unwrap_or(0.0);
        turnover_by_date.insert(r.trade_date, tr);
    }

    let mut out = Vec::with_capacity(daily_rows.len());
    for r in daily_rows {
        let date = parse_trade_date_yyyymmdd(&r.trade_date)?;
        let turnover_rate = turnover_by_date.get(&r.trade_date).copied().unwrap_or(0.0);

        out.push(StockHistoryPoint {
            date: format_dash(&date),
            open: r.open.to_f64().unwrap_or(0.0),
            high: r.high.to_f64().unwrap_or(0.0),
            low: r.low.to_f64().unwrap_or(0.0),
            close: r.close.to_f64().unwrap_or(0.0),
            pct_chg: r.pct_chg.and_then(|d| d.to_f64()).unwrap_or(0.0),
            turnover_rate,
        });
    }

    Ok(out)
}
