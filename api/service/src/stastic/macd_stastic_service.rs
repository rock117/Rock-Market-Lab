use num_traits::ToPrimitive;
use serde::Serialize;
use entity::sea_orm::{ColumnTrait, DatabaseConnection};
use entity::stock_daily;
use crate::stock::get_stock_list;
use crate::trade_calendar_service;

use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::EntityTrait;
use entity::sea_orm::QueryOrder;
use entity::sea_orm::QueryFilter;

use common::finance::ma;

#[derive(Debug, Serialize)]
pub struct MacdStastics {
    pub total: usize,
    pub up: MacdStastic, // 站上ma5/10/20/60/250 的股票数
    pub down: MacdStastic, // 跌破ma5/10/20/60/250 的股票数
}

#[derive(Debug, Serialize)]
pub struct MacdStastic {
    pub ma5_num: usize,
    pub ma10_num: usize,
    pub ma20_num: usize,
    pub ma60_num: usize,
    pub ma250_num: usize,
}

pub async fn macd_stastic(conn: &DatabaseConnection) -> anyhow::Result<MacdStastics> {
    let stock_list = get_stock_list(conn).await?;
    let dates = trade_calendar_service::get_trade_calendar(300, conn).await?;
    let dates = &dates[0..251];
    let start_date = dates[dates.len() - 1].cal_date.as_str();
    let end_date = dates[0].cal_date.as_str();
    let (mut ma5up_num, mut ma10up_num, mut ma20up_num, mut ma60up_num, mut ma250up_num) = (0, 0, 0, 0, 0);
    let (mut ma5down_num, mut ma10down_num, mut ma20down_num, mut ma60down_num, mut ma250down_num) = (0, 0, 0, 0, 0);
    let mut curr = 0;
    let total = stock_list.len();
    for stock in &stock_list {
        let dailies: Vec<stock_daily::Model> = stock_daily::Entity::find()
            .filter(ColumnTrait::eq(&stock_daily::Column::TsCode, &stock.ts_code))
            .filter(stock_daily::Column::TradeDate.gte(start_date))
            .filter(stock_daily::Column::TradeDate.lte(end_date))
            .order_by_desc(stock_daily::Column::TradeDate)
            .all(conn).await?;
        let prices = dailies.iter().map(|d| d.close.to_f64()).collect::<Option<Vec<f64>>>().unwrap_or(vec![]);
        let (ma5_up, ma10_up, ma20_up, ma60_up, ma250_up)  = calc_macd(&prices, |curr, ma| curr > ma);
        if ma5_up {
            ma5up_num += 1;
        } else {
            ma5down_num += 1;
        }
        if ma10_up {
            ma10up_num += 1;
        } else {
            ma10down_num += 1;
        }
        if ma20_up {
            ma20up_num += 1;
        } else {
            ma20down_num += 1;
        }
        if ma60_up {
            ma60up_num += 1;
        } else {
            ma60down_num += 1;
        }
        if ma250_up {
            ma250up_num += 1;
        } else {
            ma250down_num += 1;
        }
        curr += 1;
        if curr % 30 == 0 {
            println!("calc macd stastic progress: {}/{}", curr, total);
        }
    }
    let maup = MacdStastic {
        ma5_num: ma5up_num,
        ma10_num: ma10up_num,
        ma20_num: ma20up_num,
        ma60_num: ma60up_num,
        ma250_num: ma250up_num,
    };
    let madown = MacdStastic {
        ma5_num: ma5down_num,
        ma10_num: ma10down_num,
        ma20_num: ma20down_num,
        ma60_num: ma60down_num,
        ma250_num: ma250down_num,
    };
    Ok(MacdStastics {
        total,
        up: maup,
        down: madown,
    })
}

fn calc_macd<F>(all_prices: &[f64], f: F) -> (bool, bool, bool, bool, bool) where F: Fn(f64, f64) -> bool {
    let curr = all_prices[0];
    let prices = &all_prices[1..];
    let ma5 = ma::<5>(prices);
    let ma10 = ma::<10>(prices);
    let ma20 = ma::<20>(prices);
    let ma60 = ma::<60>(prices);
    let ma250 = ma::<250>(prices);

    let ma5_up = ma5.map_or(false, |v| f(curr, v));
    let ma10_up = ma10.map_or(false, |v| f(curr, v));
    let ma20_up = ma20.map_or(false, |v| f(curr, v));
    let ma60_up = ma60.map_or(false, |v| f(curr, v));
    let ma250_up = ma250.map_or(false, |v| f(curr, v));
    (ma5_up, ma10_up, ma20_up, ma60_up, ma250_up)
}