use anyhow::{Context, Result};

use std::collections::HashMap;

use entity::sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use entity::sea_orm::sea_query::Expr;
use entity::sea_orm::prelude::Decimal;
use entity::{dc_index, dc_member, stock_daily};
use serde::{Deserialize, Serialize};

use crate::pct_chg::PeriodPctChg;

pub async fn list_dc_index_latest(conn: &DatabaseConnection) -> Result<Vec<dc_index::Model>> {
    let pairs: Vec<(String, String)> = dc_index::Entity::find()
        .select_only()
        .column(dc_index::Column::TsCode)
        .column_as(Expr::col(dc_index::Column::TradeDate).max(), "max_trade_date")
        .group_by(dc_index::Column::TsCode)
        .order_by_asc(dc_index::Column::TsCode)
        .into_tuple::<(String, String)>()
        .all(conn)
        .await
        .context("Failed to query latest trade_date per dc_index.ts_code")?;

    let mut out = Vec::with_capacity(pairs.len());
    for (ts_code, trade_date) in pairs {
        if let Some(row) = dc_index::Entity::find_by_id((ts_code.clone(), trade_date.clone()))
            .one(conn)
            .await
            .context("Failed to fetch dc_index by id")?
        {
            out.push(row);
        }

    }

    out.sort_by(|a, b| a.ts_code.cmp(&b.ts_code));
    Ok(out)
}

pub async fn list_dc_index_trade_dates(conn: &DatabaseConnection) -> Result<Vec<String>> {
    let rows: Vec<String> = dc_index::Entity::find()
        .select_only()
        .column(dc_index::Column::TradeDate)
        .group_by(dc_index::Column::TradeDate)
        .order_by_desc(dc_index::Column::TradeDate)
        .into_tuple::<String>()
        .all(conn)
        .await
        .context("Failed to query distinct dc_index.trade_date")?;

    Ok(rows)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DcMemberEnriched {
    pub ts_code: String,
    pub trade_date: String,
    pub con_code: String,
    pub name: Option<String>,

    pub pct_chg_day: Option<f64>,
    pub pct_chg_latest: Option<f64>,

    pub pct5: Option<f64>,
    pub pct10: Option<f64>,
    pub pct20: Option<f64>,
    pub pct60: Option<f64>,
}

pub async fn list_dc_members_enriched_by_concept(
    conn: &DatabaseConnection,
    ts_code: &str,
    trade_date: &str,
) -> Result<Vec<DcMemberEnriched>> {
    // MySQL compatible approach: for each member stock, use correlated subqueries against stock_daily.
    // pctN computed by latest_close vs close at N trading days ago (LIMIT 1 OFFSET N).

    let members: Vec<dc_member::Model> = dc_member::Entity::find()
        .filter(ColumnTrait::eq(&dc_member::Column::TsCode, ts_code.to_string()))
        .filter(ColumnTrait::eq(&dc_member::Column::TradeDate, trade_date.to_string()))
        .order_by_asc(dc_member::Column::ConCode)
        .all(conn)
        .await
        .context("Failed to fetch dc_member rows")?;

    if members.is_empty() {
        return Ok(vec![]);
    }

    let cn_symbols: Vec<String> = members.iter().map(|m| m.con_code.clone()).collect();

    let mut pct_day_map: HashMap<String, Option<f64>> = HashMap::new();
    let mut pct_latest_map: HashMap<String, Option<f64>> = HashMap::new();
    let mut closes_desc_map: HashMap<String, Vec<Decimal>> = HashMap::new();

    let daily_rows = stock_daily::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily::Column::TradeDate, trade_date.to_string()))
        .filter(stock_daily::Column::TsCode.is_in(cn_symbols.clone()))
        .all(conn)
        .await
        .context("Failed to fetch stock_daily rows for selected trade_date")?;
    for d in daily_rows {
        let v = d.pct_chg.and_then(|x| x.to_string().parse::<f64>().ok());
        pct_day_map.insert(d.ts_code.clone(), v);
    }

    let latest_trade_date: Option<String> = stock_daily::Entity::find()
        .select_only()
        .column(stock_daily::Column::TradeDate)
        .order_by_desc(stock_daily::Column::TradeDate)
        .limit(1)
        .into_tuple::<String>()
        .one(conn)
        .await
        .context("Failed to fetch latest stock_daily.trade_date")?;

    if let Some(latest_trade_date) = latest_trade_date {
        let latest_dailies = stock_daily::Entity::find()
            .filter(ColumnTrait::eq(&stock_daily::Column::TradeDate, latest_trade_date.clone()))
            .filter(stock_daily::Column::TsCode.is_in(cn_symbols.clone()))
            .all(conn)
            .await
            .context("Failed to fetch latest stock_daily rows")?;

        for d in latest_dailies {
            let v = d.pct_chg.and_then(|x| x.to_string().parse::<f64>().ok());
            pct_latest_map.insert(d.ts_code.clone(), v);
        }

        let last_60_dates: Vec<String> = stock_daily::Entity::find()
            .select_only()
            .column(stock_daily::Column::TradeDate)
            .distinct()
            .order_by_desc(stock_daily::Column::TradeDate)
            .limit(65)
            .into_tuple::<String>()
            .all(conn)
            .await
            .context("Failed to fetch last trade_dates for pctN")?;

        if !last_60_dates.is_empty() {
            let rows = stock_daily::Entity::find()
                .filter(stock_daily::Column::TradeDate.is_in(last_60_dates))
                .filter(stock_daily::Column::TsCode.is_in(cn_symbols.clone()))
                .order_by_desc(stock_daily::Column::TradeDate)
                .all(conn)
                .await
                .context("Failed to fetch stock_daily rows for pctN")?;

            for r in rows {
                closes_desc_map.entry(r.ts_code.clone()).or_default().push(r.close);
            }
        }
    }

    let results = members
        .into_iter()
        .map(|m| {
            let closes_desc = closes_desc_map.get(&m.con_code).map(|v| v.as_slice()).unwrap_or(&[]);
            let pct = PeriodPctChg::from_closes_desc(closes_desc);
            let (pct5, pct10, pct20, pct60) = pct.to_f64_tuple();

            DcMemberEnriched {
                ts_code: m.ts_code,
                trade_date: m.trade_date,
                con_code: m.con_code.clone(),
                name: m.name,
                pct_chg_day: pct_day_map.get(&m.con_code).cloned().flatten(),
                pct_chg_latest: pct_latest_map.get(&m.con_code).cloned().flatten(),
                pct5,
                pct10,
                pct20,
                pct60,
            }
        })
        .collect();

    Ok(results)
}

pub async fn list_dc_index_by_trade_dates(
    conn: &DatabaseConnection,
    trade_dates: &[String],
) -> Result<Vec<dc_index::Model>> {
    let mut q = dc_index::Entity::find().order_by_desc(dc_index::Column::TradeDate);

    if !trade_dates.is_empty() {
        q = q.filter(dc_index::Column::TradeDate.is_in(trade_dates.to_vec()));
    }

    let rows = q
        .order_by_asc(dc_index::Column::TsCode)
        .all(conn)
        .await
        .context("Failed to fetch dc_index rows by trade_dates")?;

    Ok(rows)
}

pub async fn list_dc_members_by_concept(
    conn: &DatabaseConnection,
    ts_code: &str,
    trade_date: &str,
) -> Result<Vec<dc_member::Model>> {
    let rows = dc_member::Entity::find()
        .filter(ColumnTrait::eq(&dc_member::Column::TsCode, ts_code.to_string()))
        .filter(ColumnTrait::eq(&dc_member::Column::TradeDate, trade_date.to_string()))
        .order_by_asc(dc_member::Column::ConCode)
        .all(conn)
        .await
        .context("Failed to fetch dc_member rows")?;

    Ok(rows)
}
