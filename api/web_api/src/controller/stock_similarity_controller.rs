use entity::sea_orm::DatabaseConnection;
use rocket::{get, State};
use rocket::FromForm;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};
use service::stock::stock_similarity_service;

#[derive(FromForm, Debug)]
pub struct StockSimilarityParams {
    pub ts_code: String,
    pub days: Option<usize>,
    pub top: Option<usize>,
    pub algo: Option<String>,
}

#[get("/api/stocks/similarity?<params..>")]
pub async fn get_stock_similarity(
    params: StockSimilarityParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<stock_similarity_service::StockSimilarityResp>> {
    let conn = conn as &DatabaseConnection;

    let days = params.days.unwrap_or(60);
    let top = params.top.unwrap_or(50);

    let resp = stock_similarity_service::get_similar_stocks_with_kline_by_algo(
        conn,
        &params.ts_code,
        days,
        top,
        params.algo.as_deref(),
    )
    .await?;
    WebResponse::new(resp).into_result()
}
