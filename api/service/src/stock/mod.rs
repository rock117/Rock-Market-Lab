use anyhow::{bail, anyhow};
use serde::Serialize;
use entity::{stock, stock_daily, stock_daily_basic, trade_calendar, finance_indicator};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

use itertools::Itertools;
use num_traits::ToPrimitive;
use tracing::info;
use common::data_type::TsCode;
use entity::sea_orm::prelude::Decimal;

use common::finance::*;
use entity::sea_orm::sqlx::encode::IsNull::No;

use super::trade_calendar_service;

#[derive(Debug, Serialize)]
pub struct StockOverView {
    pub name: String,
    pub ts_code: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub market: Option<String>,
    pub list_date: String,
    pub price: Option<f64>,

    pub pct_chg5: Option<f64>,
    pub pct_chg10: Option<f64>,
    pub pct_chg20: Option<f64>,
    pub pct_chg60: Option<f64>,
    pub pct_chg250: Option<f64>,
    pub pct_chg_from_this_year: Option<f64>, // 今年涨跌幅
    pub pct_chg_from_list_date: Option<f64>, // 上市以来涨跌幅

    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    pub ma60: Option<f64>,
    pub ma250: Option<f64>,

    pub roe: Option<f64>, // roe
    pub total_mv: Option<f64>, // 总市值(万元)
    pub circ_mv: Option<f64>, // 流通市值
}


pub async fn get_stock_overviews(conn: &DatabaseConnection) -> anyhow::Result<Vec<StockOverView>> {
    let tx = conn.begin().await?;
    let stocks: Vec<stock::Model> = stock::Entity::find().all(conn).await?;
    let dates = trade_calendar_service::get_trade_calendar(250, conn).await?;
    let year_begin = trade_calendar_service::get_year_begin_trade_calendar(conn).await?;

    let start_date_call = dates.last();
    let end_date_call = dates.first();
    let (Some(start_date_call), Some(end_date_call)) = (start_date_call, end_date_call) else {
        bail!("start_date or end_date is None: {:?}, {:?}", start_date_call, end_date_call)
    };
    let start_date = &start_date_call.cal_date;
    let end_date = &end_date_call.cal_date;
    let mut views = vec![];
    let mut curr = 0;
    for stock in &stocks {
        let tscode = stock.ts_code.as_str();
        let stock_prices: Vec<stock_daily::Model> = stock_daily::Entity::find()
            .filter(stock_daily::Column::TsCode.eq(tscode))
            .filter(stock_daily::Column::TradeDate.gte(start_date))
            .filter(stock_daily::Column::TradeDate.lte(end_date))
            .order_by_desc(stock_daily::Column::TradeDate)
            .all(conn)
            .await?;
        let prices = stock_prices.iter().map(|v| v.close.to_f64()).collect::<Option<Vec<f64>>>().ok_or(anyhow!("stock_prices {} is None", tscode))?;
        let finance_indicator: Option<finance_indicator::Model> = get_finance_indicator(tscode, conn).await?;
        let stock_daily_basic: stock_daily_basic::Model = get_stock_daily_basic(tscode, conn).await?;
        let list_date_price = get_price(tscode, &stock.list_date.clone().unwrap_or("".into()), conn).await?;
        let this_year_begin_price = get_price(tscode, &year_begin, conn).await?;
        if list_date_price.is_none() {
            info!("list_date_price is None: tscode -> {}, list date -> {:?}", tscode, stock.list_date);
        }
        if this_year_begin_price.is_none() {
            info!("this_year_begin_price is None: tscode -> {}, year_begin -> {}", tscode, year_begin);
        }
        views.push(create_stock_overview(&stock, &stock_daily_basic, &finance_indicator, &prices, this_year_begin_price, list_date_price));
        curr += 1;
        if curr % 50 == 0 {
            info!("get_stock_overviews process {} / {}", curr, stocks.len());
        }
    }
    tx.commit().await?;
    Ok(views)
}

async fn get_stock_daily_basic(ts_code: &str, conn: &DatabaseConnection) -> anyhow::Result<stock_daily_basic::Model> {
    let stock_daily_basics: Vec<stock_daily_basic::Model> = stock_daily_basic::Entity::find()
        .filter(stock_daily_basic::Column::TsCode.eq(ts_code))
        .order_by_desc(stock_daily_basic::Column::TradeDate)
        .all(conn)
        .await?;
    stock_daily_basics.first().cloned().ok_or(anyhow!("stock_daily_basic {} is None", ts_code))
}

async fn get_finance_indicator(ts_code: &str, conn: &DatabaseConnection) -> anyhow::Result<Option<finance_indicator::Model>> {
    let finance_indicators: Vec<finance_indicator::Model> = finance_indicator::Entity::find()
        .filter(finance_indicator::Column::TsCode.eq(ts_code))
        .order_by_desc(finance_indicator::Column::EndDate)
        .all(conn)
        .await?;
    Ok(finance_indicators.first().cloned())
}

fn create_stock_overview(stock: &stock::Model, stock_daily_basic: &stock_daily_basic::Model, finance_indicator: &Option<finance_indicator::Model>, stock_prices: &[f64],
                         this_year_begin_price: Option<f64>, list_date_price: Option<f64>,
) -> StockOverView {
    let roe = if let Some(finance_indicator) = finance_indicator {
        finance_indicator.roe.map(|v| v.to_f64()).flatten()
    } else {
        None
    };
    let pct_chg_from_this_year = if let Some(this_year_begin_price) = this_year_begin_price {
        Some(common::finance::pct_chg(this_year_begin_price, stock_daily_basic.close.unwrap().to_f64().unwrap()))
    } else {
        None
    };

    let pct_chg_from_list_date = if let Some(list_date_price) = list_date_price {
        Some(common::finance::pct_chg(list_date_price, stock_daily_basic.close.unwrap().to_f64().unwrap()))
    } else {
        None
    };

    let overview = StockOverView {
        name: stock.name.clone().unwrap_or("".to_string()),
        ts_code: stock.ts_code.clone(),
        area: stock.area.clone(),
        industry: stock.area.clone(),
        market: stock.area.clone(),
        list_date: stock.list_date.clone().unwrap_or("".to_string()),
        price: stock_daily_basic.close.map(|v| v.to_f64()).flatten(),
        pct_chg5: pct_chg(stock_prices, 5),
        pct_chg10: pct_chg(stock_prices, 10),
        pct_chg20: pct_chg(stock_prices, 20),
        pct_chg60: pct_chg(stock_prices, 60),
        pct_chg250: pct_chg(stock_prices, 250),
        pct_chg_from_this_year,
        pct_chg_from_list_date,
        ma5: ma::<5>(stock_prices),
        ma10: ma::<10>(stock_prices),
        ma20: ma::<20>(stock_prices),
        ma60: ma::<60>(stock_prices),
        ma250: ma::<250>(stock_prices),
        roe,
        total_mv: stock_daily_basic.total_mv.map(|v| v.to_f64()).flatten(),
        circ_mv: stock_daily_basic.circ_mv.map(|v| v.to_f64()).flatten(),
    };
    overview
}

async fn get_price(ts_code: &str, date: &str, conn: &DatabaseConnection) -> anyhow::Result<Option<f64>> {
    let stock_daily: Option<stock_daily::Model> = stock_daily::Entity::find()
        .filter(stock_daily::Column::TsCode.eq(ts_code))
        .filter(stock_daily::Column::TradeDate.eq(date))
        .one(conn).await?;
    Ok(stock_daily.map(|v| v.close.to_f64()).flatten())
}


fn pct_chg(stock_prices: &[f64], n: usize) -> Option<f64> {
    if stock_prices.len() < n + 1 {
        return None;
    }
    let curr = stock_prices[0];
    let prev = stock_prices[n];
    Some(common::finance::pct_chg(prev, curr))
}





