use std::str::FromStr;
use anyhow::anyhow;
use derive_more::Display;
use serde::Serialize;
use entity::sea_orm::prelude::Decimal;
use entity::{index_daily, index_monthly, stock_daily};
use crate::security::SecurityType::Stock;
pub mod security_search_service;
pub mod security_daily_service;
mod security_monthly_service;

#[derive(Debug, Copy, Clone, Serialize, Display)]
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
pub struct SecurityDaily {
    pub ts_code: String,
    pub trade_date: String,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Option<Decimal>,
    pub pre_close: Option<Decimal>,
    pub change: Option<Decimal>,
    pub pct_chg: Option<Decimal>,
    pub vol: Option<Decimal>,
    pub amount: Option<Decimal>,
}

pub type SecurityMonthly = SecurityDaily;


impl SecurityDaily {
    pub fn from_stock_daily(data: stock_daily::Model) -> SecurityDaily {
        SecurityDaily {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: Some(data.open),
            high: Some(data.high),
            low: Some(data.low),
            close: Some(data.close),
            pre_close: data.pre_close,
            change: data.change,
            pct_chg: data.pct_chg,
            vol: Some(data.vol),
            amount: Some(data.amount),
        }
    }

    pub fn from_index_daily(data: index_daily::Model) -> SecurityDaily {
        SecurityDaily {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
            pre_close: data.pre_close,
            change: data.change,
            pct_chg: data.pct_chg,
            vol: data.vol,
            amount: data.amount,
        }
    }

    pub fn from_index_monthly(data: index_monthly::Model) -> SecurityDaily {
        SecurityDaily {
            ts_code: data.ts_code,
            trade_date: data.trade_date,
            open: data.open,
            high: data.high,
            low: data.low,
            close: data.close,
            pre_close: data.pre_close,
            change: data.change,
            pct_chg: data.pct_chg,
            vol: data.vol,
            amount: data.amount,
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
