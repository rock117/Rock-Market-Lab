use entity::sea_orm::DatabaseConnection;
use rocket::{get, State};
use rocket::FromForm;
use serde_derive::Serialize;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
use service::etf_service;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EtfItemResp {
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
}

#[derive(FromForm, Debug)]
pub struct EtfHoldingsParams {
    pub ts_code: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EtfHoldingResp {
    pub ts_code: String,
    pub ann_date: String,
    pub end_date: String,
    pub symbol: String,
    pub mkv: String,
    pub amount: String,
    pub stk_mkv_ratio: Option<String>,
    pub stk_float_ratio: Option<String>,
}

#[get("/api/etf/list")]
pub async fn get_etf_list(conn: &State<DatabaseConnection>) -> Result<WebResponse<Vec<EtfItemResp>>> {
    let conn = conn as &DatabaseConnection;
    let rows = etf_service::get_etf_list(conn).await?;

    let resp = rows
        .into_iter()
        .map(|r| EtfItemResp {
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
        })
        .collect();

    WebResponse::new(resp).into_result()
}

#[get("/api/etf/holdings?<params..>")]
pub async fn get_etf_holdings(
    params: EtfHoldingsParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<EtfHoldingResp>>> {
    let conn = conn as &DatabaseConnection;
    let rows = etf_service::get_etf_holdings_latest(conn, &params.ts_code).await?;

    let resp = rows
        .into_iter()
        .map(|r| EtfHoldingResp {
            ts_code: r.ts_code,
            ann_date: r.ann_date,
            end_date: r.end_date,
            symbol: r.symbol,
            mkv: r.mkv.to_string(),
            amount: r.amount.to_string(),
            stk_mkv_ratio: r.stk_mkv_ratio.map(|v| v.to_string()),
            stk_float_ratio: r.stk_float_ratio.map(|v| v.to_string()),
        })
        .collect();

    WebResponse::new(resp).into_result()
}
