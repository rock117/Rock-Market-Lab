use anyhow::bail;
use serde::Serialize;
use entity::{stock, stock_daily, stock_daily_basic, trade_calendar, finance_indicator};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

use axum::{extract::State, Json};
use num_traits::ToPrimitive;
use entity::sea_orm::prelude::Decimal;

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


pub async fn get_stock_overviews(State(conn): State<DatabaseConnection>) -> anyhow::Result<Vec<StockOverView>> {
    let stocks: Vec<stock::Model> = stock::Entity::find().all(&conn).await?;
    let stock_daily_basics: Vec<stock_daily_basic::Model> = stock_daily_basic::Entity::find().all(&conn).await?;

    let dates =  trade_calendar_service::get_trade_calendar(250, &conn).await?;

    let start_date_call = dates.last();
    let end_date_call = dates.first();
    let (Some(start_date_call), Some(end_date_call)) = (start_date_call, end_date_call) else {
        bail!("start_date or end_date is None: {:?}, {:?}", start_date_call, end_date_call)
    };
    let start_date = &start_date_call.cal_date;
    let end_date = &end_date_call.cal_date;

    for stock in &stocks {
        let name = stock.name.clone().unwrap_or("".into());
        let tscode = stock.ts_code.as_str();

        let stock_prices = stock_daily::Entity::find()
            .filter(stock_daily::Column::TsCode.gt(tscode))
            .filter(stock_daily::Column::TradeDate.gte(start_date))
            .filter(stock_daily::Column::TradeDate.lte(end_date))
            .order_by_desc(stock_daily::Column::TradeDate) // 按日期升序排序
            .all(&conn)
            .await?;
        let finance_indicator: finance_indicator::Model = todo!();
        let stock_daily_basic: stock_daily_basic::Model = todo!();
        create_stock_overview(&stock, &stock_daily_basic, &finance_indicator, &stock_prices);
    }
    bail!("")
}


fn create_stock_overview(stock: &stock::Model, stock_daily_basic: &stock_daily_basic::Model, finance_indicator: &finance_indicator::Model, stock_prices: &[stock_daily::Model]) -> StockOverView {
    let ovewerview = StockOverView {
        name: stock.name.clone().unwrap_or("".to_string()),
        ts_code: stock.ts_code.clone(),
        area: stock.area.clone(),
        industry: stock.area.clone(),
        market: stock.area.clone(),
        list_date: stock.list_date.clone().unwrap_or("".to_string()),
        price: stock_daily_basic.close.map(|v| v.to_f64()).flatten(),
        pct_chg5: None,
        pct_chg10: None,
        pct_chg20: None,
        pct_chg60: None,
        pct_chg250: None,
        pct_chg_from_this_year: None,
        pct_chg_from_list_date: None,
        ma5: None,
        ma10: None,
        ma20: None,
        ma60: None,
        ma250: None,
        roe: finance_indicator.roe.map(|v| v.to_f64()).flatten(),
        total_mv: stock_daily_basic.total_mv.map(|v| v.to_f64()).flatten(),
        circ_mv: stock_daily_basic.circ_mv.map(|v| v.to_f64()).flatten(),
    };
    ovewerview
}








