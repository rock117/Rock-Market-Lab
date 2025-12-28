use anyhow::anyhow;
use chrono::NaiveDate;
use entity::sea_orm::DatabaseConnection;
use rocket::{get, State};
use rocket::FromForm;
use serde_derive::Serialize;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
use service::stock::stock_history_service;

#[derive(FromForm, Debug)]
pub struct StockHistoryParams {
    pub ts_code: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub time_period: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockHistoryResp {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pct_chg: f64,
    pub date: String,
    pub turnover_rate: f64,
    pub amount: Option<f64>,
}

#[get("/api/stocks/history?<params..>")]
pub async fn get_stock_history(
    params: StockHistoryParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<StockHistoryResp>>> {
    let conn = conn as &DatabaseConnection;

    let (start_date, end_date) = if let (Some(s), Some(e)) = (&params.start_date, &params.end_date) {
        let start = NaiveDate::parse_from_str(s, common::date::FORMAT_DASH)
            .map_err(|err| anyhow!("start_date format error: {}", err))?;
        let end = NaiveDate::parse_from_str(e, common::date::FORMAT_DASH)
            .map_err(|err| anyhow!("end_date format error: {}", err))?;
        (start, end)
    } else if let Some(tp) = &params.time_period {
        let p = stock_history_service::parse_time_period(tp)?;
        stock_history_service::resolve_date_range_from_period(&p)
    } else {
        return Err(anyhow!("either start_date/end_date or time_period is required").into());
    };

    let points = stock_history_service::get_stock_history(conn, &params.ts_code, &start_date, &end_date).await?;

    let resp = points
        .into_iter()
        .map(|p| StockHistoryResp {
            open: p.open,
            high: p.high,
            low: p.low,
            close: p.close,
            pct_chg: p.pct_chg,
            date: p.date,
            turnover_rate: p.turnover_rate,
            amount: p.amount,
        })
        .collect();

    WebResponse::new(resp).into_result()
}
