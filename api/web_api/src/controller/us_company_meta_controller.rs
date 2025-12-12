use anyhow::anyhow;
use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use entity::sea_orm::EntityTrait;
use entity::us_company_info;

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsCompanyMetaResponse {
    pub industries: Vec<String>,
    pub sectors: Vec<String>,
}

#[get("/api/us-company/meta")]
pub async fn get_us_company_meta(conn: &State<DatabaseConnection>) -> Result<WebResponse<UsCompanyMetaResponse>> {
    info!("get_us_company_meta");

    let conn = conn as &DatabaseConnection;

    let companies = us_company_info::Entity::find().all(conn).await.map_err(|e| anyhow!(e))?;

    let mut industries_set = std::collections::HashSet::<String>::new();
    let mut sectors_set = std::collections::HashSet::<String>::new();

    for c in companies {
        if let Some(v) = c.industry_name_cn {
            let v = v.trim();
            if !v.is_empty() {
                industries_set.insert(v.to_string());
            }
        }
        if let Some(v) = c.sector_name_cn {
            let v = v.trim();
            if !v.is_empty() {
                sectors_set.insert(v.to_string());
            }
        }
    }

    let mut industries: Vec<String> = industries_set.into_iter().collect();
    let mut sectors: Vec<String> = sectors_set.into_iter().collect();
    industries.sort();
    sectors.sort();

    WebResponse::new(UsCompanyMetaResponse { industries, sectors }).into_result()
}
