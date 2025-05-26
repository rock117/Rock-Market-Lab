use anyhow::anyhow;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::stock_daily;
use entity::stock;
use crate::trade_calendar_service;

use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct VolumnFilter {
    pub rate: f64,
    pub days: u64
}

#[derive(Debug, Serialize)]
pub struct VolumnFilterResult {
    pub items: Vec<VolumnFilterResultItem>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct VolumnFilterResultItem {
    pub name: String,
    pub ts_code: String,
    pub vol: f64,
    pub date: String,
    pub avg_vol: f64,
    pub rate: f64,
}

pub async fn filter(filter: &VolumnFilter, conn: &DatabaseConnection) -> anyhow::Result<VolumnFilterResult> {
    let calendars = trade_calendar_service::get_trade_calendar(filter.days, conn).await?;
    let start_date = calendars.last().ok_or(anyhow!("no start date"))?.cal_date.clone();
    let stocks = stock::Entity::find().all(conn).await.map_err(|err| anyhow!("get stock list failed, error: {:?}", err))?;
    let mut items = vec![];
    for stock in stocks {
        let stock_dailies: Vec<stock_daily::Model> = stock_daily::Entity::find()
            .filter(stock_daily::Column::TsCode.eq(&stock.ts_code))
            .filter(stock_daily::Column::TradeDate.gte(&start_date))
            .order_by_desc(stock_daily::Column::TradeDate)
            .all(conn)
            .await?;
        let meet = meet_filter(filter.rate, stock_dailies);
        if meet.0 {
            items.push(VolumnFilterResultItem {
                name: stock.name.clone().unwrap_or("".into()),
                ts_code: stock.ts_code,
                vol: meet.3,
                date: meet.2.clone(),
                avg_vol: meet.1,
                rate: meet.3 / meet.1
            })
        }
    }
    items.sort_by(|a, b| b.rate.partial_cmp(&a.rate).unwrap());
    let total = items.len();
    Ok(VolumnFilterResult { items, total })
}

fn meet_filter(rate: f64, mut stock_dailies: Vec<stock_daily::Model>) -> (bool, f64, String, f64) {
    if stock_dailies.is_empty() {
        return (false, 0f64, "".into(), 0f64);
    }
  //  stock_dailies.sort_by_key(|v| std::cmp::Reverse(v.vol));
    stock_dailies.sort_by(|a, b| b.vol.partial_cmp(&a.vol).unwrap());

    let max_data = stock_dailies.remove(0);
    let max = max_data.vol.clone().to_f64().unwrap_or(0f64);
    let date = max_data.trade_date.clone();
    let avg = stock_dailies.iter().map(|v| v.vol.to_f64().unwrap_or(0f64)).sum::<f64>() / stock_dailies.len() as f64;
    (max / avg >= rate, avg, date, max)
}



