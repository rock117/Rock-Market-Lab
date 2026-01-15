use rocket::{delete, get, post, put, State};
use rocket::serde::json::Json;
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::strategy_profile_service::{
    clone_strategy_profile, create_strategy_profile, delete_strategy_profile, get_strategy_profile,
    list_strategy_profiles, update_strategy_profile, CloneStrategyProfileRequest,
    CreateStrategyProfileRequest, StrategyProfileDto, UpdateStrategyProfileRequest,
};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[get("/api/strategy-profiles")]
pub async fn list_strategy_profiles_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<StrategyProfileDto>>> {
    info!("list strategy profiles");
    let conn = conn as &DatabaseConnection;
    let result = list_strategy_profiles(conn).await?;
    WebResponse::new(result).into_result()
}

#[put("/api/strategy-profiles/<id>", data = "<request>")]
pub async fn update_strategy_profile_handler(
    id: i32,
    request: Json<UpdateStrategyProfileRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<StrategyProfileDto>> {
    let conn = conn as &DatabaseConnection;
    let result = update_strategy_profile(conn, id, request.into_inner()).await?;
    WebResponse::new(result).into_result()
}

#[delete("/api/strategy-profiles/<id>")]
pub async fn delete_strategy_profile_handler(
    id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<String>> {
    let conn = conn as &DatabaseConnection;
    delete_strategy_profile(conn, id).await?;
    WebResponse::new(format!("Strategy profile {} deleted", id)).into_result()
}

#[get("/api/strategy-profiles/<id>")]
pub async fn get_strategy_profile_handler(
    id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<StrategyProfileDto>> {
    let conn = conn as &DatabaseConnection;
    let result = get_strategy_profile(conn, id).await?;
    WebResponse::new(result).into_result()
}

#[post("/api/strategy-profiles", data = "<request>")]
pub async fn create_strategy_profile_handler(
    request: Json<CreateStrategyProfileRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<StrategyProfileDto>> {
    info!("create strategy profile: {:?}", request);
    let conn = conn as &DatabaseConnection;
    let result = create_strategy_profile(conn, request.into_inner()).await?;
    WebResponse::new(result).into_result()
}

#[post("/api/strategy-profiles/<id>/clone", data = "<request>")]
pub async fn clone_strategy_profile_handler(
    id: i32,
    request: Option<Json<CloneStrategyProfileRequest>>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<StrategyProfileDto>> {
    let conn = conn as &DatabaseConnection;
    let req = request
        .map(|r| r.into_inner())
        .unwrap_or(CloneStrategyProfileRequest {
            name: None,
            description: None,
            template: None,
            settings: None,
        });
    let result = clone_strategy_profile(conn, id, req).await?;
    WebResponse::new(result).into_result()
}
