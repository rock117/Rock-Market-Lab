pub mod stock_price_limit_service;
mod macd_filter_service;
mod stock_price_filter_service;

use num_traits::ToPrimitive;
use common::finance::stock;
use entity::stock_daily;
use common::finance::stock::*;

pub fn filter_price_limit_num_stocks(stock_prices: &[stock_daily::Model], start: &str, end: &str) -> Vec<stock_daily::Model> {
    let stock_prices = stock_prices.iter().filter(|s| s.trade_date.as_str() >= start && s.trade_date.as_str() <= end).collect::<Vec<&stock_daily::Model>>();
    let mut limitup_prices =vec![];
    for stock_price in stock_prices {
        if is_price_limitup(stock_price) {
            limitup_prices.push(stock_price.clone());
        }
    }
    limitup_prices
}

fn is_price_limitup(stock: &stock_daily::Model) -> bool {
    stock::is_price_limitup(&InvestmentPrice {
        ts_code: stock.ts_code.clone(),
        pct_chg: stock.pct_chg.map(|v| v.to_f64()).flatten().unwrap_or(0f64),
        high: stock.high.to_f64().unwrap_or(0f64),
        close: stock.close.to_f64().unwrap_or(0f64),
    })
}