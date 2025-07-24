use entity::sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use entity::stock_daily_basic;


#[derive(Debug, Deserialize)]
pub struct AssetParam {
    pub rate: f64,
}

#[derive(Debug, Serialize)]
pub struct AssetResponse {
    pub securities: Vec<AssetResponseItem>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct AssetResponseItem {
    pub name: String,
    pub ts_code: String,
    pub net_asset: f64, //归属于母公司的净资产（即净值）,
    pub market_value: f64, // 市值
    pub rate: f64,
}

pub async fn get_asset(
    param: &AssetParam,
    conn: &DatabaseConnection,
) -> anyhow::Result<AssetResponse> {
    unimplemented!()
}
