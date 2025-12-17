use anyhow::anyhow;
use chrono::NaiveDate;
use entity::sea_orm::DatabaseConnection;
use rocket::{get, State};
use rocket::FromForm;
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

#[derive(Debug, Clone)]
pub enum MarginBalanceKind {
    Exchange(String),
    Stock(String),
}

#[derive(Debug, Clone)]
pub struct MarginBalanceParams {
    pub kind: MarginBalanceKind,
    pub start_date: String,
    pub end_date: String,
}

#[derive(FromForm, Debug)]
pub struct RawMarginBalanceParams {
    pub exchange: Option<String>,
    pub stock: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[get("/api/margin/balance?<params..>")]
pub async fn get_margin_balance(
    params: RawMarginBalanceParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<MarginBalanceResp>>> {
    let conn = conn as &DatabaseConnection;
    let start_date = NaiveDate::parse_from_str(&params.start_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("start_date format error: {}", e))?;
    let end_date = NaiveDate::parse_from_str(&params.end_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("end_date format error: {}", e))?;

    let params = MarginBalanceParams {
        kind: if let Some(exchange) = params.exchange {
            MarginBalanceKind::Exchange(exchange)
        } else if let Some(stock) = params.stock {
            MarginBalanceKind::Stock(stock)
        } else {
            MarginBalanceKind::Exchange("ALL".to_string())
        },
        start_date: params.start_date,
        end_date: params.end_date,
    };

    let points = match &params.kind {
        MarginBalanceKind::Exchange(exchange) => {
            margin_service::get_margin_balance(conn, exchange, &start_date, &end_date).await?
        }
        MarginBalanceKind::Stock(ts_code) => {
            margin_service::get_stock_margin_balance(conn, ts_code, &start_date, &end_date).await?
        }
    };

    let resp = points
        .into_iter()
        .map(|p| MarginBalanceResp {
            date: p.date,
            margin_balance: p.margin_balance,
        })
        .collect();

    WebResponse::new(resp).into_result()
}
