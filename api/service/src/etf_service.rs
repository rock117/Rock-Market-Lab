use entity::{etf, fund_portfolio};
use entity::fund_daily;
use entity::sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use entity::sea_orm::sea_query::{Expr, Func};
use entity::sea_orm::QuerySelect;
use entity::sea_orm::prelude::Decimal;
use entity::stock;
use std::collections::HashMap;
use num_traits::ToPrimitive;
#[derive(Debug, Clone, serde::Serialize)]
pub struct EtfOverview {
    pub ts_code: String,
    pub csname: String,
    pub cname: Option<String>,
    pub extname: Option<String>,
    pub index_code: Option<String>,
    pub index_name: Option<String>,
    pub setup_date: Option<String>,
    pub list_date: Option<String>,
    pub list_status: Option<String>,
    pub exchange: Option<String>,
    pub custod_name: Option<String>,
    pub mgr_name: Option<String>,
    pub mgt_fee: Option<String>,
    pub etf_type: Option<String>,

    pub close: Option<f64>,
    pub vol: Option<f64>,
    pub amount: Option<f64>,
    pub pct_chg: Option<f64>,
    pub pct5: Option<f64>,
    pub pct10: Option<f64>,
    pub pct20: Option<f64>,
    pub pct60: Option<f64>,
}

fn calc_period_pct_chg(closes_desc: &[Decimal], days: usize) -> Option<Decimal> {
    if days < 2 {
        return None;
    }
    if closes_desc.len() < days {
        return None;
    }
    let today = closes_desc.get(0).copied()?;
    let past = closes_desc.get(days - 1).copied()?;
    if past.is_zero() {
        return None;
    }
    Some((today - past) / past * Decimal::from(100i64))
}

pub async fn get_etf_list(conn: &DatabaseConnection) -> anyhow::Result<Vec<EtfOverview>> {
    let items = etf::Entity::find()
        .order_by_asc(etf::Column::TsCode)
        .all(conn)
        .await?;

    if items.is_empty() {
        return Ok(vec![]);
    }

    let latest_trade_date: Option<String> = fund_daily::Entity::find()
        .select_only()
        .column(fund_daily::Column::TradeDate)
        .order_by_desc(fund_daily::Column::TradeDate)
        .limit(1)
        .into_tuple::<String>()
        .one(conn)
        .await?;

    let Some(latest_trade_date) = latest_trade_date else {
        return Ok(items
            .into_iter()
            .map(|r| EtfOverview {
                ts_code: r.ts_code,
                csname: r.csname,
                cname: r.cname,
                extname: r.extname,
                index_code: r.index_code,
                index_name: r.index_name,
                setup_date: r.setup_date,
                list_date: r.list_date,
                list_status: r.list_status,
                exchange: r.exchange,
                custod_name: r.custod_name,
                mgr_name: r.mgr_name,
                mgt_fee: r.mgt_fee,
                etf_type: r.etf_type,
                close: None,
                vol: None,
                amount: None,
                pct_chg: None,
                pct5: None,
                pct10: None,
                pct20: None,
                pct60: None,
            })
            .collect());
    };

    let last_60_dates: Vec<String> = fund_daily::Entity::find()
        .select_only()
        .column(fund_daily::Column::TradeDate)
        .distinct()
        .order_by_desc(fund_daily::Column::TradeDate)
        .limit(65)
        .into_tuple::<String>()
        .all(conn)
        .await?;

    let latest_rows = fund_daily::Entity::find()
        .filter(<fund_daily::Column as ColumnTrait>::eq(
            &fund_daily::Column::TradeDate,
            latest_trade_date.clone(),
        ))
        .all(conn)
        .await?;
    let latest_map: HashMap<String, fund_daily::Model> = latest_rows
        .into_iter()
        .map(|m| (m.ts_code.clone(), m))
        .collect();

    let ma_rows = fund_daily::Entity::find()
        .filter(<fund_daily::Column as ColumnTrait>::is_in(
            &fund_daily::Column::TradeDate,
            last_60_dates,
        ))
        .order_by_desc(fund_daily::Column::TradeDate)
        .all(conn)
        .await?;

    let mut closes_map: HashMap<String, Vec<Decimal>> = HashMap::new();
    for d in ma_rows {
        closes_map.entry(d.ts_code.clone()).or_default().push(d.close);
    }

    let rows = items
        .into_iter()
        .map(|r| {
            let ts_code = r.ts_code.clone();
            let (close, vol, amount, pct_chg) = match latest_map.get(&ts_code) {
                Some(d) => (Some(d.close), Some(d.vol), Some(d.amount), d.pct_chg),
                None => (None, None, None, None),
            };
            let closes_desc = closes_map.get(&ts_code).map(|v| v.as_slice()).unwrap_or(&[]);
            EtfOverview {
                ts_code: r.ts_code,
                csname: r.csname,
                cname: r.cname,
                extname: r.extname,
                index_code: r.index_code,
                index_name: r.index_name,
                setup_date: r.setup_date,
                list_date: r.list_date,
                list_status: r.list_status,
                exchange: r.exchange,
                custod_name: r.custod_name,
                mgr_name: r.mgr_name,
                mgt_fee: r.mgt_fee,
                etf_type: r.etf_type,
                close: close.and_then(|v| v.to_f64()),
                vol: vol.and_then(|v| v.to_f64()),
                amount: amount.and_then(|v| v.to_f64()),
                pct_chg: pct_chg.and_then(|v| v.to_f64()),
                pct5: calc_period_pct_chg(closes_desc, 5).and_then(|v| v.to_f64()),
                pct10: calc_period_pct_chg(closes_desc, 10).and_then(|v| v.to_f64()),
                pct20: calc_period_pct_chg(closes_desc, 20).and_then(|v| v.to_f64()),
                pct60: calc_period_pct_chg(closes_desc, 60).and_then(|v| v.to_f64()),
            }
        })
        .collect();

    Ok(rows)
}

pub async fn get_stock_name_map(
    conn: &DatabaseConnection,
    ts_codes: Vec<String>,
) -> anyhow::Result<HashMap<String, Option<String>>> {
    if ts_codes.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = stock::Entity::find()
        .select_only()
        .column(stock::Column::TsCode)
        .column(stock::Column::Name)
        .filter(stock::Column::TsCode.is_in(ts_codes))
        .into_tuple::<(String, Option<String>)>()
        .all(conn)
        .await?;

    Ok(rows.into_iter().collect())
}

pub async fn search_etfs(conn: &DatabaseConnection, keyword: &str) -> anyhow::Result<Vec<etf::Model>> {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        return Ok(vec![]);
    }

    let keyword = keyword.to_lowercase();
    let pattern = format!("%{}%", keyword);

    let condition = Condition::any()
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::TsCode))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::Csname))).like(&pattern))
        .add(Expr::expr(Func::lower(Expr::col(etf::Column::Cname))).like(&pattern));

    let items = etf::Entity::find()
        .filter(condition)
        .order_by_asc(etf::Column::TsCode)
        .limit(50)
        .all(conn)
        .await?;

    Ok(items)
}

pub async fn get_etf_holdings_latest(
    conn: &DatabaseConnection,
    ts_code: &str,
) -> anyhow::Result<Vec<fund_portfolio::Model>> {
    let latest = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .order_by_desc(fund_portfolio::Column::EndDate)
        .one(conn)
        .await?;

    let Some(latest) = latest else {
        return Ok(vec![]);
    };

    let items = fund_portfolio::Entity::find()
        .filter(ColumnTrait::eq(&fund_portfolio::Column::TsCode, ts_code))
        .filter(ColumnTrait::eq(
            &fund_portfolio::Column::EndDate,
            latest.end_date.clone(),
        ))
        .order_by_desc(fund_portfolio::Column::Mkv)
        .all(conn)
        .await?;

    Ok(items)
}
