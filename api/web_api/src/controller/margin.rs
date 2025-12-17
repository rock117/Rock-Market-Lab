use anyhow::anyhow;
use chrono::NaiveDate;
use entity::sea_orm::DatabaseConnection;
use rocket::{get, State};
use serde_derive::{Deserialize, Serialize};
use service::margin_service;
use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[derive(Deserialize, Debug)]
pub struct MarginQuery {
    date_range: String,
    statics_type: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MarginDetailQuery {
    pub trade_date: String,
    pub ts_code: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct MarginDetail;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarginBalanceResp {
    pub date: String,
    pub margin_balance: f64,
}

#[get("/api/margin/balance?<exchange>&<start_date>&<end_date>")]
pub async fn get_margin_balance(
    exchange: &str,
    start_date: &str,
    end_date: &str,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<MarginBalanceResp>>> {
    let conn = conn as &DatabaseConnection;
    let start_date = NaiveDate::parse_from_str(start_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("start_date format error: {}", e))?;
    let end_date = NaiveDate::parse_from_str(end_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("end_date format error: {}", e))?;

    let points = margin_service::get_margin_balance(conn, exchange, &start_date, &end_date).await?;
    let resp = points
        .into_iter()
        .map(|p| MarginBalanceResp {
            date: p.date,
            margin_balance: p.margin_balance,
        })
        .collect();

    WebResponse::new(resp).into_result()
}
