use chrono::NaiveDate;
use num_traits::ToPrimitive;
use serde::Serialize;
use tracing::info;
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::sea_orm::prelude::Decimal;
use entity::stock_daily;
use entity::trade_calendar;
use crate::trade_calendar_service;

use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::QueryFilter;

use common::finance::*;

#[derive(Serialize, Debug)]
pub struct LimitupStocks {
    pub total: usize,
    pub start_date: String,
    pub end_date: String,
    pub stocks: Vec<LimitupStock>,
}

#[derive(Serialize, Debug)]
pub struct LimitupStock {
    pub ts_code: String,
    pub name: String,
    #[serde(flatten)]
    pub info: StasticInfo,

    pub price: Option<f64>,
    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    // 乖离率
}

#[derive(Serialize, Debug)]
struct StasticInfo {
    pub continue_limitup_days: usize,
    pub limitup_days: usize,
    pub up_days: usize,
    pub down_days: usize,
    pub total_pct_chg: f64,
}


pub async fn filter_continue_price_limit(past_ndays: u64, conn: &DatabaseConnection) -> anyhow::Result<LimitupStocks> {
    let mut cal_dates = trade_calendar_service::get_trade_calendar(past_ndays, conn).await?;
    cal_dates[0].cal_date = "20250221".into();
    let first_date = &cal_dates[0].cal_date;
    let stock_dailies: Vec<stock_daily::Model> = stock_daily::Entity::find()
        .filter(stock_daily::Column::TradeDate.eq(first_date))
        .all(conn)
        .await?;
    let limitup_stocks = filter_price_limit_stocks(stock_dailies);
    info!("limitup_stocks size: {:?}", limitup_stocks.len());

    let start_date = &cal_dates[cal_dates.len() - 1].cal_date;
    let mut results: Vec<LimitupStock> = vec![];
    for stock in &limitup_stocks {
        let stock_dailies = get_stock_dailies(&stock.ts_code, start_date, first_date, conn).await?;
        let limitup_num = get_price_limit_num_of_stock(&stock_dailies).await;
        if limitup_num.continue_limitup_days > 0 {
            let name = crate::stock::get_stock(&stock.ts_code, conn).await?.name.clone().unwrap_or("".into());
            let price = stock_dailies[0].close.to_f64().clone();
            let prices = stock_dailies.iter().map(|s| s.close.to_f64()).collect::<Option<Vec<f64>>>();
            let prices = prices.unwrap_or(vec![]);
            results.push(LimitupStock {
                ts_code: stock.ts_code.clone(),
                ma5: ma::<5>(&prices),
                ma10: ma::<10>(&prices),
                ma20: ma::<20>(&prices),
                info: limitup_num,
                price,
                name,
            });
        }
    }
   results.sort_by(|a, b| b.info.continue_limitup_days.cmp(&a.info.continue_limitup_days));
    let stocks = LimitupStocks {
        total: results.len(),
        start_date: start_date.clone(),
        end_date: first_date.clone(),
        stocks: results,
    };
    Ok(stocks)
}

async fn get_stock_dailies(tscode: &str, start: &str, end: &str, conn: &DatabaseConnection) -> anyhow::Result<Vec<stock_daily::Model>> {
    let stock_dailies: Vec<stock_daily::Model> = stock_daily::Entity::find()
        .filter(stock_daily::Column::TsCode.eq(tscode))
        .filter(stock_daily::Column::TradeDate.gte(start))
        .filter(stock_daily::Column::TradeDate.lte(end))
        .order_by_desc(stock_daily::Column::TradeDate)
        .all(conn)
        .await?;
    Ok(stock_dailies)
}

async fn get_price_limit_num_of_stock(stocks: &[stock_daily::Model]) -> StasticInfo {
    let mut continue_limitup_days = 0;
    let mut limitup_days = 0;
    let mut limitup_calc = true;
    let mut up_days = 0;
    let mut down_days = 0;
    for stock in stocks {
        if is_price_limitup(stock) {
            if limitup_calc {
                continue_limitup_days += 1;
            }
            limitup_days += 1;
        } else {
            limitup_calc = false;
        }

        if stock.pct_chg.map_or(false, |pct| pct > Decimal::ZERO) {
            up_days += 1;
        }
        if stock.pct_chg.map_or(false, |pct| pct < Decimal::ZERO) {
            down_days += 1;
        }
    }
    let prices = stocks.iter().map(|s| s.close.to_f64()).collect::<Option<Vec<f64>>>().unwrap_or(vec![]);
    let total_pct_chg = pct_chg(prices[stocks.len() - 1], prices[0]);
    StasticInfo {
        limitup_days,
        continue_limitup_days,
        up_days,
        down_days,
        total_pct_chg,
    }
}

fn filter_price_limit_stocks(stocks: Vec<stock_daily::Model>) -> Vec<stock_daily::Model> {
    stocks.into_iter().filter(|s| is_price_limitup(s)).collect()
}

fn is_price_limitup(stock: &stock_daily::Model) -> bool {
    let tscode = &stock.ts_code;
    let pct_chg = stock.pct_chg.map(|v| v.to_f64()).flatten().unwrap_or(0f64);
    let limitup: f64 = if tscode.ends_with("BJ") {
        30f64
    } else if tscode.starts_with("688") {
        20f64
    } else if tscode.starts_with("300") {
        20f64
    } else {
        10f64
    };
    let delta = pct_chg - limitup;
    if stock.close == stock.high {
        // info!("tscode: {}, pct_chg: {}, delta: {}, limitup:{}, close: {}, high: {}, limitup: {}", tscode, pct_chg, delta.abs(), limitup, stock.close , stock.high, delta.abs() < 0.01 && stock.close == stock.high);
    }
    delta.abs() < 0.01 && stock.close == stock.high
}

fn is_price_inc(stock: &stock_daily::Model) -> bool {
    match stock.pct_chg {
        None => false,
        Some(pct_chg) => {
            match pct_chg.to_f64() {
                None => false,
                Some(pct_chg) => pct_chg > 0f64,
            }
        }
    }
}