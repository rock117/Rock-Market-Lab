use std::str::FromStr;
use anyhow::anyhow;
use derive_more::Display;
use futures::FutureExt;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use entity::sea_orm::prelude::Decimal;
use entity::{fund_daily, index_daily, index_monthly, index_weekly, stock_daily, stock_monthly, stock_weekly};
use crate::security::SecurityType::Stock;
pub use compare::security_history_compare_service;

pub mod security_search_service;
pub mod security_daily_service;
mod compare;
pub mod stock_asset_service;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Display)]
pub enum SecurityType {
    Index,
    Stock,
    Fund,
}

#[derive(Serialize, Debug, Clone)]
pub struct Security {
    ts_code: String,
    name: Option<String>,
    r#type: SecurityType,
}

#[derive(Debug, Clone, Serialize)]
pub struct SecurityPrice {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub pre_close: Option<f64>,
    pub change: Option<f64>,
    pub pct_chg: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
}

pub type Year = u32;

impl SecurityPrice {

    pub fn from_fund_daily(data: fund_daily::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.to_f64(),
            high: data.high.to_f64(),
            low: data.low.to_f64(),
            close: data.close.to_f64(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol:data.vol.to_f64(),
            amount: data.amount.to_f64(),
        }
    }

    pub fn from_stock_daily(data: stock_daily::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.to_f64(),
            high: data.high.to_f64(),
            low: data.low.to_f64(),
            close: data.close.to_f64(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol:data.vol.to_f64(),
            amount: data.amount.to_f64(),
        }
    }

    pub fn from_stock_weekly(data: stock_weekly::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.to_f64(),
            high: data.high.to_f64(),
            low: data.low.to_f64(),
            close: data.close.to_f64(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol: data.vol.to_f64(),
            amount: data.amount.to_f64(),
        }
    }

    pub fn from_stock_monthly(data: stock_monthly::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.to_f64(),
            high: data.high.to_f64(),
            low: data.low.to_f64(),
            close: data.close.to_f64(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol: data.vol.to_f64(),
            amount: data.amount.to_f64()
        }
    }

    pub fn from_index_daily(data: index_daily::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.map(|v| v.to_f64()).flatten(),
            high: data.high.map(|v| v.to_f64()).flatten(),
            low: data.low.map(|v| v.to_f64()).flatten(),
            close: data.close.map(|v| v.to_f64()).flatten(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol: data.vol.map(|v| v.to_f64()).flatten(),
            amount: data.amount.map(|v| v.to_f64()).flatten(),
        }
    }

    pub fn from_index_weekly(data: index_weekly::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.map(|v| v.to_f64()).flatten(),
            high: data.high.map(|v| v.to_f64()).flatten(),
            low: data.low.map(|v| v.to_f64()).flatten(),
            close: data.close.map(|v| v.to_f64()).flatten(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol: data.vol.map(|v| v.to_f64()).flatten(),
            amount: data.amount.map(|v| v.to_f64()).flatten(),
        }
    }

    pub fn from_index_monthly(data: index_monthly::Model) -> SecurityPrice {
        SecurityPrice {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open.map(|v| v.to_f64()).flatten(),
            high: data.high.map(|v| v.to_f64()).flatten(),
            low: data.low.map(|v| v.to_f64()).flatten(),
            close: data.close.map(|v| v.to_f64()).flatten(),
            pre_close: data.pre_close.map(|v| v.to_f64()).flatten(),
            change: data.change.map(|v| v.to_f64()).flatten(),
            pct_chg: data.pct_chg.map(|v| v.to_f64()).flatten(),
            vol: data.vol.map(|v| v.to_f64()).flatten(),
            amount: data.amount.map(|v| v.to_f64()).flatten(),
        }
    }
}

impl FromStr for SecurityType{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Stock" => Ok(Stock),
            "Index" => Ok(SecurityType::Index),
            "Fund" => Ok(SecurityType::Fund),
            _ => Err(anyhow!("Unknown security type: {}", s)),
        }
    }
}
