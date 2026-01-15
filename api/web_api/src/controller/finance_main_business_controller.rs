use rocket::{get, State};
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::finance_main_business_service::{
    get_finance_main_business_bz_items, get_finance_main_business_end_dates, get_finance_main_business_list,
    FinanceMainBusinessListResponse, FinanceMainBusinessQueryParams,
};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct FinanceMainBusinessParams {
    pub r#type: String,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub end_dates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct FinanceMainBusinessBzItemsParams {
    pub r#type: String,
}


#[get("/api/finance/main-business?<params..>")]
pub async fn get_finance_main_business(
    params: FinanceMainBusinessParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<FinanceMainBusinessListResponse>> {
    info!("主营业务查询: {:?}", params);
    let conn = conn as &DatabaseConnection;

    let query_params = FinanceMainBusinessQueryParams {
        r#type: params.r#type,
        page: params.page,
        page_size: params.page_size,
        sort_by: params.sort_by,
        sort_dir: params.sort_dir,
        end_dates: params.end_dates,
    };

    let result = get_finance_main_business_list(&query_params, conn).await?;
    WebResponse::new(result).into_result()
}

#[get("/api/finance/main-business/end-dates")]
pub async fn get_finance_main_business_end_dates_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<String>>> {
    let conn = conn as &DatabaseConnection;

    let result = get_finance_main_business_end_dates(conn).await?;
    WebResponse::new(result).into_result()
}

#[get("/api/finance/main-business/bz-items?<params..>")]
pub async fn get_finance_main_business_bz_items_handler(
    params: FinanceMainBusinessBzItemsParams,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<String>>> {
    info!("主营业务来源查询: {:?}", params);
    let conn = conn as &DatabaseConnection;

    let result = get_finance_main_business_bz_items(&params.r#type, conn).await?;
    WebResponse::new(result).into_result()
}
