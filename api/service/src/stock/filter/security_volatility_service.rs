use anyhow::anyhow;
use chrono::NaiveDate;
use derive_more::Display;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use common::calc::{DailyTradeRecord, calculate_volatility};
use entity::stock_daily::Model as StockDaily;
use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;

use entity::stock_daily;
use entity::stock;

use super::super::stock_price_service;

#[derive(Debug, Deserialize, Copy, Clone, Display, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Sort {
    Asc,
    Desc
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
impl VolatilityFilter {
    pub fn new(num: u64, days: u64, sort: Sort, r#type: Type) -> Self {
        Self {
            num,
            days,
            sort,
            r#type,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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
    
    // 收集所有股票代码
    let ts_codes: Vec<String> = stocks.iter().map(|s| s.ts_code.clone()).collect();
    
    // 批量查询所有股票的价格数据
    let grouped_prices = stock_price_service::get_stock_prices_batch(&ts_codes, &start, &end, conn).await?;
    
    // 处理每个股票的价格数据
    let mut volatilities = Vec::new();
    
    for (ts_code, prices) in grouped_prices {
        let records = prices
            .iter()
            .map(|p| from_stock_daily(p))
            .collect::<anyhow::Result<Vec<DailyTradeRecord>>>()?;
        
        if records.len() > 1 {
            let metrics = calculate_volatility(&records);
            volatilities.push(SecurityVolatility {
                ts_code,
                volatility: metrics.calculate_score(),
            });
        }
    }

    if filter.sort == Sort::Asc {
        volatilities.sort_by(|a, b| a.volatility.partial_cmp(&b.volatility).unwrap());
    } else {
        volatilities.sort_by(|a, b| b.volatility.partial_cmp(&a.volatility).unwrap());
    }

    Ok(volatilities.into_iter().take(filter.num as usize).collect())
}