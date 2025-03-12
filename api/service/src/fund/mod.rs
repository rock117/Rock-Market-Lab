use chrono::{Datelike, DateTime, NaiveDate, Utc};
use futures::StreamExt;
use itertools::Itertools;
use entity::{fund_daily, stock_daily};
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::sea_orm::{QueryFilter, QueryOrder, EntityTrait};

pub async fn get_fund_daily(ts_code: &str, start: &NaiveDate, end: &NaiveDate, conn: &DatabaseConnection) -> anyhow::Result<Vec<fund_daily::Model>> {
    let start = start.format("%Y%m%d").to_string();
    let end = end.format("%Y%m%d").to_string();
    let fund_dailies = fund_daily::Entity::find()
        .filter(fund_daily::Column::TsCode.eq(ts_code))
        .filter(fund_daily::Column::TradeDate.gte(&start))
        .filter(fund_daily::Column::TradeDate.lte(&end))
        .order_by_desc(fund_daily::Column::TradeDate)
        .all(conn).await?;
    Ok(fund_dailies)
}

pub async fn get_fund_weekly(ts_code: &str, start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<fund_daily::Model>> {
    todo!()
}

pub async fn get_fund_monthly(ts_code: &str, start: &NaiveDate, end: &NaiveDate) -> anyhow::Result<Vec<fund_daily::Model>> {
    todo!()
}

fn filter_week_end_data(prices: Vec<fund_daily::Model>) ->Vec<fund_daily::Model> {
    let mut grouped_prices = prices
        .into_iter()
        .group_by(|price| {
            let date = DateTime::parse_from_str(&price.trade_date, "%Y%m%d").unwrap().with_timezone(&Utc);
            (date.year(), date.iso_week().week())
        });

    let mut filtered_prices = Vec::new();
    for (_, group) in &grouped_prices {
        let last_price = group.last().unwrap();
        filtered_prices.push(last_price.clone());
    }
    filtered_prices
}

fn filter_month_end_data(prices: Vec<fund_daily::Model>) -> Vec<fund_daily::Model> {
    let mut grouped_prices = prices
        .into_iter()
        .group_by(|price| {
            let date = DateTime::parse_from_str(&price.trade_date, "%Y%m%d").unwrap().with_timezone(&Utc);
            (date.year(), date.month())
        });

    let mut filtered_prices = Vec::new();
    for (_, group) in &grouped_prices {
        let last_price = group.last().unwrap();
        filtered_prices.push(last_price.clone());
    }
    // while let Some((_, group)) = grouped_prices.next() {
    //     let last_price = group.last().unwrap();
    //     filtered_prices.push(last_price.clone());
    // }

    filtered_prices
}

#[cfg(test)]
mod tests {
    use entity::sea_orm::prelude::Decimal;
    use super::*;

    #[test]
    fn test_filter_week_end_data() {
        // 创建测试数据：跨越两周的数据
        Decimal::new(1, 1);
        let test_data = vec![
            create_fund_daily_data("20240101"),
            create_fund_daily_data("20240101"),
            create_fund_daily_data("20240105"),
            create_fund_daily_data("20240105"),
            create_fund_daily_data("20240112"),
            create_fund_daily_data("20240112"),
        ];

        let filtered_data = filter_week_end_data(test_data);

        // 验证结果
        assert_eq!(filtered_data.len(), 2, "应该只返回两周的数据");
        assert_eq!(filtered_data[0].trade_date, "20240112", "第二周应该返回12号的数据");
        assert_eq!(filtered_data[1].trade_date, "20240105", "第一周应该返回5号的数据");
        
        // 验证值是否正确
        assert_eq!(filtered_data[0].close, dec!(1.3), "第二周收盘价应该是1.3");
        assert_eq!(filtered_data[1].close, dec!(1.1), "第一周收盘价应该是1.1");
    }

    fn create_fund_daily_data(date: &str) -> fund_daily::Model {
        fund_daily::Model {
            ts_code: "000001.OF".to_string(),
            trade_date: date.to_string(),
            open: Decimal::new(3, 3),
            high: Decimal::new(3, 3),
            low: Decimal::new(3, 3),
            close: Decimal::new(3, 3),
            pre_close: None,
            change: None,
            pct_chg: None,
            vol: Decimal::new(3, 3),
            amount: Decimal::new(3, 3),
        }
    }
}