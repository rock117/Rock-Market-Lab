use num_traits::ToPrimitive;
/// 两市涨跌幅 n日内 股价涨幅 > x%的股票

use entity::sea_orm::DatabaseConnection;
use entity::stock_daily;
use common::finance::ma;

pub async fn filter(prices: &[stock_daily::Model], start: &str, end: &str, pct_chg: f64) -> anyhow::Result<()> {
    /// filter prices by pct_chg from start to endpub async fn filter(prices: &[stock_daily::Model], start: &str, end: &str, pct_chg: f64) -> anyhow::Result<()> {
    let filtered_prices: Vec<_> = prices
        .iter()
        .filter(|price| {
            let date = price.trade_date.as_str(); // assuming stock_daily::Model has a date field
            let date_range = date >= start && date <= end;
            let pct_chg_condition = price.pct_chg.unwrap().to_f64().unwrap() > pct_chg; // assuming stock_daily::Model has a pct_chg field
            date_range && pct_chg_condition
        })
        .collect();

    // do something with the filtered prices
    Ok(())
}

fn add(a: usize, b: usize) -> usize {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(10, 20), 30);
    }
}