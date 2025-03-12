use std::collections::HashMap;
use anyhow::anyhow;
use chrono::NaiveDate;
use common::data_type::period::Period;
use entity::sea_orm::{QueryFilter, QueryOrder, EntityTrait};

use entity::{fund_daily, index_daily, index_monthly, index_weekly, stock_daily, stock_monthly, stock_weekly};
use entity::sea_orm::{ColumnTrait, DatabaseConnection};

use crate::security::{SecurityPrice, SecurityType, Year};

pub async fn get_security_by_years(r#type: SecurityType, ts_code: &str, period: Period, years: &[Year], conn: &DatabaseConnection) -> anyhow::Result<HashMap<Year, Vec<SecurityPrice>>> {
    let mut all = HashMap::new();
    for year in years {
        let (start, end) = get_year_begin_end(*year)?;
        let start = start.format("%Y%m%d").to_string();
        let end = end.format("%Y%m%d").to_string();
        let datas = match r#type {
            SecurityType::Index => get_index_history(ts_code, period, &start, &end, conn).await?,
            SecurityType::Stock => get_stock_history(ts_code, period, &start, &end, conn).await?,
            SecurityType::Fund => get_fund_history(ts_code, period, &start, &end, conn).await?
        };
        all.insert(*year, datas);
    }
    Ok(all)
}

async fn get_stock_history(ts_code: &str, period: Period, start: &str, end: &str, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityPrice>> {
    let data = match period {
        Period::Day => {
            stock_daily::Entity::find()
                .filter(stock_daily::Column::TsCode.eq(ts_code))
                .filter(stock_daily::Column::TradeDate.gte(start))
                .filter(stock_daily::Column::TradeDate.lte(end))
                .order_by_desc(stock_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_stock_daily(d)).collect()
        }
        Period::Week => {
            stock_weekly::Entity::find()
                .filter(stock_weekly::Column::TsCode.eq(ts_code))
                .filter(stock_weekly::Column::TradeDate.gte(start))
                .filter(stock_weekly::Column::TradeDate.lte(end))
                .order_by_desc(stock_weekly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_stock_weekly(d)).collect()
        }
        Period::Month => {
            stock_monthly::Entity::find()
                .filter(stock_monthly::Column::TsCode.eq(ts_code))
                .filter(stock_monthly::Column::TradeDate.gte(start))
                .filter(stock_monthly::Column::TradeDate.lte(end))
                .order_by_desc(stock_monthly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_stock_monthly(d)).collect()
        }
    };
    Ok(data)
}


async fn get_index_history(ts_code: &str, period: Period, start: &str, end: &str, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityPrice>> {
    let data = match period {
        Period::Day => {
            index_daily::Entity::find()
                .filter(index_daily::Column::TsCode.eq(ts_code))
                .filter(index_daily::Column::TradeDate.gte(start))
                .filter(index_daily::Column::TradeDate.lte(end))
                .order_by_desc(index_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_daily(d)).collect()
        }
        Period::Week => {
            index_weekly::Entity::find()
                .filter(index_weekly::Column::TsCode.eq(ts_code))
                .filter(index_weekly::Column::TradeDate.gte(start))
                .filter(index_weekly::Column::TradeDate.lte(end))
                .order_by_desc(index_weekly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_weekly(d)).collect()
        }
        Period::Month => {
            index_monthly::Entity::find()
                .filter(index_monthly::Column::TsCode.eq(ts_code))
                .filter(index_monthly::Column::TradeDate.gte(start))
                .filter(index_monthly::Column::TradeDate.lte(end))
                .order_by_desc(index_monthly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_monthly(d)).collect()
        }
    };
    Ok(data)
}

async fn get_fund_history(ts_code: &str, period: Period, start: &str, end: &str, conn: &DatabaseConnection) -> anyhow::Result<Vec<SecurityPrice>> {
    let data = match period {
        Period::Day => {
            fund_daily::Entity::find()
                .filter(fund_daily::Column::TsCode.eq(ts_code))
                .filter(fund_daily::Column::TradeDate.gte(start))
                .filter(fund_daily::Column::TradeDate.lte(end))
                .order_by_desc(fund_daily::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_fund_daily(d)).collect()
        }
        Period::Week => {
            index_weekly::Entity::find()
                .filter(index_weekly::Column::TsCode.eq(ts_code))
                .filter(index_weekly::Column::TradeDate.gte(start))
                .filter(index_weekly::Column::TradeDate.lte(end))
                .order_by_desc(index_weekly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_weekly(d)).collect()
        }
        Period::Month => {
            index_monthly::Entity::find()
                .filter(index_monthly::Column::TsCode.eq(ts_code))
                .filter(index_monthly::Column::TradeDate.gte(start))
                .filter(index_monthly::Column::TradeDate.lte(end))
                .order_by_desc(index_monthly::Column::TradeDate)
                .all(conn).await?.into_iter().map(|d| SecurityPrice::from_index_monthly(d)).collect()
        }
    };
    Ok(data)
}


fn get_year_begin_end(year: u32) -> anyhow::Result<(NaiveDate, NaiveDate)> {
    let start = NaiveDate::from_ymd_opt(year as i32, 1, 1).ok_or(anyhow!("invalid year"))?;
    let end = NaiveDate::from_ymd_opt(year as i32, 12, 31).ok_or(anyhow!("invalid year"))?;
    Ok((start, end))
}