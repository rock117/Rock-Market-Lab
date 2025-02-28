use anyhow::anyhow;
use chrono::NaiveDate;
use num_traits::ToPrimitive;
use serde::Serialize;
use tracing::info;
use entity::sea_orm::DatabaseConnection;
use crate::stock_daily_service;
use crate::trade_calendar_service;
use common::finance::ma_n;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

#[derive(Debug, Clone, Serialize)]
pub struct StockBiasRatio {
    pub price: f64,
    pub ma5: Option<BiasRatio>,
    pub ma10: Option<BiasRatio>,
    pub ma20: Option<BiasRatio>,
    pub ma60: Option<BiasRatio>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BiasRatio {
    pub value: f64, // macd 值
    pub bias: String,
}

pub async fn get_bias_ratio(ts_code: &str, price: Option<f64>, conn: &DatabaseConnection) -> anyhow::Result<StockBiasRatio> {
    let trade_calendar = trade_calendar_service::get_trade_calendar(70, conn).await?;
    let dates = trade_calendar.into_iter().map(|x| x.cal_date).collect::<Vec<String>>();
    let end_date = NaiveDate::parse_from_str(dates[0].as_str(), "%Y%m%d").map_err(|e| anyhow!(e))?;
    let start_date = NaiveDate::parse_from_str(dates[61].as_str(), "%Y%m%d").map_err(|e| anyhow!(e))?;
    let prices = stock_daily_service::get_stock_daily(ts_code, &start_date, &end_date, conn).await?;
    let prices = prices.into_iter().map(|x| x.close.to_f64()).collect::<Option<Vec<f64>>>().ok_or(anyhow!("prices is none"))?;
    let (price, prev_prices) = if let Some(price) = price {
        (price, &prices[0..60])
    } else {
        (prices[0], &prices[1..61]) // &prices[1..61])
    };

    info!("price: {}, prev_prices len: {:?}, prices len: {}", price, prev_prices.len(), prices.len());
    let ratio = StockBiasRatio {
        price,
        ma5: get_bias_ratio_value(price, 5, prev_prices),
        ma10: get_bias_ratio_value(price, 10, prev_prices),
        ma20: get_bias_ratio_value(price, 20, prev_prices),
        ma60: get_bias_ratio_value(price, 60, prev_prices),
    };
    Ok(ratio)
}

fn get_bias_ratio_value(price: f64, n: usize, prices: &[f64]) -> Option<BiasRatio> {
    let ma = ma_n(n, prices);
    match ma {
        None => None,
        Some(ma) => Some(BiasRatio { value: ma, bias: to_pct((price - ma) / ma) }),
    }
}

fn to_pct(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}