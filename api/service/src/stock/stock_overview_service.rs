use anyhow::{bail, anyhow};
use chrono::Local;
use serde::{Deserialize, Serialize};
use entity::{stock, stock_daily, stock_daily_basic, trade_calendar, finance_indicator, cache_data};
use entity::sea_orm::{DatabaseConnection, TransactionTrait};

use entity::sea_orm::EntityTrait;
use entity::sea_orm::ActiveModelTrait;
use entity::sea_orm::ColumnTrait;
use entity::sea_orm::QueryFilter;
use entity::sea_orm::QueryOrder;

use itertools::Itertools;
use num_traits::ToPrimitive;
use tracing::info;
use common::data_type::{AllSingle, TsCode};
use entity::sea_orm::prelude::Decimal;

use common::finance::*;
use entity::sea_orm::sqlx::encode::IsNull::No;
use crate::trade_calendar_service;
use common::web::request::{StockQueryParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockOverviewResponse {
    total: usize,
    data: Vec<StockOverView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockOverView {
    pub name: String,
    pub ts_code: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub market: Option<String>,
    pub list_date: String,
    pub close: Option<f64>,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub pct_chg: Option<f64>,
    pub turnover_rate: Option<f64>, // 换手率
    pub amount: Option<f64>, // 成交额 （千元）

    pub pct_chg5: Option<f64>,
    pub pct_chg10: Option<f64>,
    pub pct_chg20: Option<f64>,
    pub pct_chg60: Option<f64>,
    pub pct_chg_from_this_year: Option<f64>, // 今年涨跌幅

    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    pub ma60: Option<f64>,
    pub ma250: Option<f64>,

    pub gross_margin: Option<f64>, // 毛利率
    pub roe: Option<f64>, // roe
    pub total_mv: Option<f64>, // 总市值(万元)
    pub circ_mv: Option<f64>, // 流通市值
}

pub async fn get_stock_overviews(param: &StockQueryParams, conn: &DatabaseConnection) -> anyhow::Result<StockOverviewResponse> {
    
    let data: Option<Vec<StockOverView>> = get_from_cache()?;
    if let Some(data) = data {
        return Ok(get_paging_data(data, param))
    }
    let tx = conn.begin().await?;
    let stocks: Vec<stock::Model> = stock::Entity::find().all(conn).await?;
    let dates = trade_calendar_service::get_trade_calendar(250, conn).await?;
    info!("stock overview, dates len: {}, start_date: {}, end_date: {}", dates.len(), dates[dates.len() - 1].cal_date, dates[0].cal_date);
    let year_begin = trade_calendar_service::get_year_begin_trade_calendar(conn).await?;

    let start_date_call = dates.last();
    let end_date_call = dates.first();
    let (Some(start_date_call), Some(end_date_call)) = (start_date_call, end_date_call) else {
        bail!("start_date or end_date is None: {:?}, {:?}", start_date_call, end_date_call)
    };
    let start_date = &start_date_call.cal_date;
    let end_date = &end_date_call.cal_date;
    let start_date = if start_date < &year_begin {
        start_date
    } else {
        &year_begin
    };

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
        let pct_chg = stock_prices[0].pct_chg.map(|v| v.to_f64()).flatten();
        let amount = stock_prices[0].amount.to_f64();
        let low = stock_prices[0].low.to_f64();
        let high = stock_prices[0].high.to_f64();

        let prices = stock_prices.iter().map(|v| v.close.to_f64()).collect::<Option<Vec<f64>>>().ok_or(anyhow!("stock_prices {} is None", tscode))?;
        let finance_indicator: Option<finance_indicator::Model> = get_finance_indicator(tscode, conn).await?;
        let stock_daily_basic: stock_daily_basic::Model = get_stock_daily_basic(tscode, conn).await?;
        let this_year_begin_price = get_price_of_date(&year_begin, &stock_prices).await;

        if this_year_begin_price.is_none() {
            info!("this_year_begin_price is None: tscode -> {}, year_begin -> {}", tscode, year_begin);
        }
        views.push(create_stock_overview(&stock, &stock_daily_basic, finance_indicator.as_ref(), &prices, this_year_begin_price, pct_chg, amount, low, high));
        curr += 1;
        if curr % 50 == 0 {
            info!("get_stock_overviews process {} / {}", curr, stocks.len());
        }
    }
    tx.commit().await?;
    save_to_cache(&views)?;
    Ok(get_paging_data(views, param))
}

fn get_from_cache() -> anyhow::Result<Option<Vec<StockOverView>>> {
    let key = "stock_overviews";
    let file = format!("{}/{}-cache.json", r#"C:\rock\coding\code\my\rust\Rock-Market-Lab\api\tmp\data"#, key);
    if std::fs::exists(&file).unwrap_or(false)  {
        let data = std::fs::read_to_string(&file)?;
        let data: Vec<StockOverView> = serde_json::from_str(&data)?;
        return Ok(Some(data))
    }
    Ok(None)
}

fn save_to_cache(views: &Vec<StockOverView>) -> anyhow::Result<()> {
    let key = "stock_overviews";
    let file = format!("{}/{}-cache.json", r#"C:\rock\coding\code\my\rust\Rock-Market-Lab\api\tmp\data"#, key);
    std::fs::write(&file, serde_json::to_string(views)?)?;
    Ok(())
}
fn get_paging_data(mut views: Vec<StockOverView>, param: &StockQueryParams) -> StockOverviewResponse {
    info!("before filter, views num: {}", views.len());
    views = filter_stocks(views, param);
    info!("after filter, views num: {}", views.len());
    sort(&mut views, &param.order_by, &param.order);
    let result_views = common::paging::get_paging_data(&views, param.page, param.page_size);
    StockOverviewResponse {total: views.len(), data: result_views}
}

fn filter_stocks(views: Vec<StockOverView>, param: &StockQueryParams) -> Vec<StockOverView> {
    let views = views.into_iter().filter(|v| filter_selected(&v.market, &param.market) && filter_selected(&v.area, &param.area) && filter_selected(&v.industry, &param.industry)).collect::<Vec<StockOverView>>();
    views
}

fn filter_selected(data: &Option<String>, selected: &AllSingle<String>) -> bool {
    match (data, selected) {
        (_, AllSingle::All) => true,
        (None, _) => false,
        (Some(v), AllSingle::Single(s)) => v == s
    }
}

fn sort(views: &mut [StockOverView], sort_by: &str, order: &str) { // ascending descending
    if sort_by == "pct_chg" {
        views.sort_by(|a, b| {
            match (a.pct_chg, b.pct_chg) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "pct_chg5" {
        views.sort_by(|a, b| {
            match (a.pct_chg5, b.pct_chg5) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "pct_chg10" {
        views.sort_by(|a, b| {
            match (a.pct_chg10, b.pct_chg10) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "pct_chg20" {
        views.sort_by(|a, b| {
            match (a.pct_chg20, b.pct_chg20) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "pct_chg60" {
        views.sort_by(|a, b| {
            match (a.pct_chg60, b.pct_chg60) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "total_mv" {
        views.sort_by(|a, b| {
            match (a.total_mv, b.total_mv) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "turnover_rate" {
        views.sort_by(|a, b| {
            match (a.turnover_rate, b.turnover_rate) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "roe" {
        views.sort_by(|a, b| {
            match (a.roe, b.roe) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "amount" {
        views.sort_by(|a, b| {
            match (a.amount, b.amount) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(chg1), Some(chg2)) => if order == "ascending" {chg1.partial_cmp(&chg2).unwrap_or(std::cmp::Ordering::Equal)} else {chg2.partial_cmp(&chg1).unwrap_or(std::cmp::Ordering::Equal)},
            }
        });
    }

    if sort_by == "list_date" {
        views.sort_by(|a, b| {
            match (&a.list_date, &b.list_date) {
                (chg1, chg2) => if order == "ascending" { chg1.cmp(&chg2) } else { chg2.cmp(&chg1) },
            }
        });
    }
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

fn create_stock_overview(stock: &stock::Model, stock_daily_basic: &stock_daily_basic::Model, finance_indicator: Option<&finance_indicator::Model>, stock_prices: &[f64],
                         this_year_begin_price: Option<f64>,
                         pct_chgv: Option<f64>,
                         amount: Option<f64>,
                         low: Option<f64>,
                         high: Option<f64>,
) -> StockOverView {
    let (roe, gross_margin) = if let Some(finance_indicator) = finance_indicator {
        (finance_indicator.roe.map(|v| v.to_f64()).flatten(), finance_indicator.gross_margin.map(|v| v.to_f64()).flatten())
    } else {
        (None, None)
    };
    let pct_chg_from_this_year = if let Some(this_year_begin_price) = this_year_begin_price {
        Some(common::finance::pct_chg(this_year_begin_price, stock_daily_basic.close.unwrap().to_f64().unwrap()))
    } else {
        None
    };

    let overview = StockOverView {
        name: stock.name.clone().unwrap_or("".to_string()),
        ts_code: stock.ts_code.clone(),
        area: stock.area.clone(),
        industry: stock.industry.clone(),
        market: stock.market.clone(),
        list_date: stock.list_date.clone().unwrap_or("".to_string()),
        close: stock_daily_basic.close.map(|v| v.to_f64()).flatten(),
        low,
        high,
        pct_chg: pct_chgv,
        turnover_rate: stock_daily_basic.turnover_rate.clone().map(|v| v.to_f64()).flatten(),
        amount,
        pct_chg5: pct_chg(stock_prices, 5),
        pct_chg10: pct_chg(stock_prices, 10),
        pct_chg20: pct_chg(stock_prices, 20),
        pct_chg60: pct_chg(stock_prices, 60),
        pct_chg_from_this_year,
        ma5: ma::<5>(stock_prices),
        ma10: ma::<10>(stock_prices),
        ma20: ma::<20>(stock_prices),
        ma60: ma::<60>(stock_prices),
        ma250: ma::<250>(stock_prices),
        gross_margin,
        roe,
        total_mv: stock_daily_basic.total_mv.map(|v| v.to_f64()).flatten(),
        circ_mv: stock_daily_basic.circ_mv.map(|v| v.to_f64()).flatten(),
    };
    overview
}

async fn get_price_of_date(date: &str, stock_dailies : &Vec<stock_daily::Model>) -> Option<f64> {
    for stock_daily in stock_dailies {
        if stock_daily.trade_date.as_str() >= date {
            return stock_daily.close.to_f64()
        }
    }
    None
}


fn pct_chg(stock_prices: &[f64], n: usize) -> Option<f64> {
    if stock_prices.len() < n + 1 {
        return None;
    }
    let curr = stock_prices[0];
    let prev = stock_prices[n];
    Some(common::finance::pct_chg(prev, curr))
}
