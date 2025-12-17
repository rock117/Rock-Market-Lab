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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginBalanceKind {
    Exchange,
    Stock,
}

impl<'v> rocket::form::FromFormField<'v> for MarginBalanceKind {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        match field.value.to_ascii_lowercase().as_str() {
            "exchange" => Ok(Self::Exchange),
            "stock" => Ok(Self::Stock),
            _ => Err(rocket::form::Error::validation("type must be 'exchange' or 'stock'").into()),
        }
    }
}

#[derive(FromForm, Debug)]
pub struct MarginBalanceParams {
    #[field(name = "type")]
    pub kind: Option<MarginBalanceKind>,
    pub exchange: Option<String>,
    pub ts_code: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[get("/api/margin/balance?<params..>")]
pub async fn get_margin_balance(
    params: MarginBalanceParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<MarginBalanceResp>>> {
    let conn = conn as &DatabaseConnection;
    let start_date = NaiveDate::parse_from_str(&params.start_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("start_date format error: {}", e))?;
    let end_date = NaiveDate::parse_from_str(&params.end_date, common::date::FORMAT_DASH)
        .map_err(|e| anyhow!("end_date format error: {}", e))?;

    let is_stock = params.kind == Some(MarginBalanceKind::Stock) || params.ts_code.is_some();

    let points = if is_stock {
        let ts_code = params
            .ts_code
            .as_deref()
            .ok_or_else(|| anyhow!("ts_code is required when type=stock"))?;
        margin_service::get_stock_margin_balance(conn, ts_code, &start_date, &end_date).await?
    } else {
        let exchange = params.exchange.as_deref().unwrap_or("ALL");
        margin_service::get_margin_balance(conn, exchange, &start_date, &end_date).await?
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
