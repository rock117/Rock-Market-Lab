use anyhow::anyhow;
use chrono::NaiveDate;
use derive_more::Display;
use num_traits::ToPrimitive;
use serde::Deserialize;
use common::calc::{DailyTradeRecord, calculate_volatility};
use entity::stock_daily::Model as StockDaily;
use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;

use entity::stock_daily;
use entity::stock;

use super::super::stock_price_service;

#[derive(Debug, Deserialize, Copy, Clone, Display)]
enum Sort {
    asc, desc
}
#[derive(Debug, Deserialize, Copy, Clone, Display)]
enum Type {
    Stock, Fund, Index, ThsIndex
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct VolatilityFilter {
    pub num: u64, // topN
    pub days: u64, // n个交易日
    pub sort: Sort, // asc: 波动性小，desc: 波动性大
    pub r#type: Type, // 类型 Stock, Fund, Index, ThsIndex
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityVolatility {
    ts_code: String,
    volatility: f64,
}

fn from_stock_daily(stock_daily: &StockDaily) -> anyhow::Result<DailyTradeRecord> {
    let record = DailyTradeRecord {
        date: NaiveDate::parse_from_str(&stock_daily.trade_date, "%Y%m%d").map_err(|e| anyhow!(e))?,
        price: stock_daily.close.clone().to_f64().ok_or(anyhow!("no value"))?,
        volume: stock_daily.vol.clone().to_f64().ok_or(anyhow!("no value"))?
    };
    Ok(record)
}


pub async fn filter(filter: &VolatilityFilter, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityVolatility>> {
    let stocks = stock::Entity::find().all(conn).await?;
    let (start, end) = common::util::date_util::get_start_end_from_now(filter.days).map_err(|e| anyhow!(e))?;
    let mut volatilities = Vec::new();
    for stock in stocks {
        let prices = stock_price_service::get_stock_prices(&stock.ts_code, &start, &end, conn).await?;
        let records = prices.into_iter().map(|v| from_stock_daily(&v)).collect::<anyhow::Result<Vec<DailyTradeRecord>>>()?;
        let metrics = calculate_volatility(&records);
        volatilities.push(SecurityVolatility {
            ts_code: stock.ts_code,
            volatility: metrics.avg_daily_volatility,
        });
    }
    Ok(volatilities)
}