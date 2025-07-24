use std::time::Instant;
use anyhow::anyhow;
use chrono::NaiveDate;
use derive_more::Display;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use tracing::info;
use common::calc::{DailyTradeRecord, calculate_volatility};
use entity::stock_daily::Model as StockDaily;
use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;

use entity::stock_daily;
use entity::stock;

use super::super::stock_price_service;
use crate::trade_calendar_service;

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
    pub max_price_swing: f64, // 最大价格波动幅度
}

#[derive(Debug, Serialize, Clone)]
pub struct SecurityVolatility {
    ts_code: String,
    name: String,
    volatility: f64,
    max_price_swing: f64,
    avg_price: f64,
    max_price: f64,
    min_price: f64,
}


#[derive(Debug, Serialize, Clone)]
pub struct VolatilityResponse {
    total: u64,
    securities: Vec<SecurityVolatility>,
    start_date: String,
    end_date: String,
}

fn from_stock_daily(stock_daily: &StockDaily) -> anyhow::Result<DailyTradeRecord> {
    let record = DailyTradeRecord {
        date: NaiveDate::parse_from_str(&stock_daily.trade_date, "%Y%m%d").map_err(|e| anyhow!(e))?,
        price: stock_daily.close.clone().to_f64().ok_or(anyhow!("no value"))?,
        volume: stock_daily.vol.clone().to_f64().ok_or(anyhow!("no value"))?
    };
    Ok(record)
}


pub async fn filter(filter: &VolatilityFilter, conn: &DatabaseConnection) -> anyhow::Result<VolatilityResponse> {
    let stocks = stock::Entity::find().all(conn).await?;


    let dates = trade_calendar_service::get_trade_calendar(filter.days, conn).await?.into_iter().map(|c| c.cal_date).collect::<Vec<String>>();
    let start =  NaiveDate::parse_from_str(&dates[dates.len() - 1].clone(), "%Y%m%d").unwrap();
    let end = NaiveDate::parse_from_str(&dates[0].clone(), "%Y%m%d").unwrap();

    // 收集所有股票代码
    let ts_codes: Vec<String> = stocks.iter().map(|s| s.ts_code.clone()).collect();

    // 批量查询所有股票的价格数据
    let instant = Instant::now();
    let grouped_prices = stock_price_service::get_stock_prices_batch(&ts_codes, &start, &end, conn).await?;
    info!("get stock prices batch cost: {:?}, start: {:?}, end: {:?}", instant.elapsed(), start, end);
    
    // 处理每个股票的价格数据
    let mut volatilities = Vec::new();

    let instant = Instant::now();
    for (ts_code, prices) in grouped_prices {
        let records = prices
            .iter()
            .map(|p| from_stock_daily(p))
            .collect::<anyhow::Result<Vec<DailyTradeRecord>>>()?;
        
        if records.len() > 1 {
            let name: String = "".into();
            let metrics = calculate_volatility(&records);
            if metrics.max_price_swing <= filter.max_price_swing {
                volatilities.push(SecurityVolatility {
                    ts_code,
                    name,
                    volatility: metrics.volatility(),   
                    max_price_swing: metrics.max_price_swing,
                    avg_price: metrics.avg_price,
                    max_price: metrics.max_price,
                    min_price: metrics.min_price,
                });
            }
        }
    }
    info!("calculate volatility cost: {:?}", instant.elapsed());

    if filter.sort == Sort::Asc {
        volatilities.sort_by(|a, b| a.volatility.partial_cmp(&b.volatility).unwrap());
    } else {
        volatilities.sort_by(|a, b| b.volatility.partial_cmp(&a.volatility).unwrap());
    }
    let mut volatilities =  volatilities.into_iter().take(filter.num as usize).collect::<Vec<SecurityVolatility>>();
    for v in volatilities.iter_mut() {
        let stock = stock::Entity::find_by_id(&v.ts_code)
            .one(conn)
            .await?
            .ok_or(anyhow!("stock not found"))?;
        v.name = stock.name.unwrap_or_default();
    }
    let resp = VolatilityResponse {
        total: volatilities.len() as u64,
        securities: volatilities,
        start_date: start.format("%Y%m%d").to_string(),
        end_date: end.format("%Y%m%d").to_string(),
    };
    Ok(resp)
}