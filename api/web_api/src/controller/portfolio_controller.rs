use rocket::{get, post, delete, put, State};
use rocket::serde::json::Json;
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::portfolio_service::{
    create_portfolio, list_portfolios, get_portfolio, delete_portfolio,
    add_holding, remove_holding, get_holdings, update_holding_desc,
    CreatePortfolioRequest, PortfolioResponse, AddHoldingRequest, HoldingResponse, UpdateHoldingDescRequest,
};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

#[post("/api/portfolios", data = "<request>")]
pub async fn create_portfolio_handler(
    request: Json<CreatePortfolioRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<PortfolioResponse>> {
    info!("创建投资组合请求: {:?}", request);
    
    let conn = conn as &DatabaseConnection;
    let result = create_portfolio(conn, request.into_inner()).await?;
    
    WebResponse::new(result).into_result()
}

#[get("/api/portfolios")]
pub async fn list_portfolios_handler(
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<PortfolioResponse>>> {
    info!("获取投资组合列表");
    
    let conn = conn as &DatabaseConnection;
    let result = list_portfolios(conn).await?;
    
    WebResponse::new(result).into_result()
}

#[get("/api/portfolios/<portfolio_id>")]
pub async fn get_portfolio_handler(
    portfolio_id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<PortfolioResponse>> {
    info!("获取投资组合详情: {}", portfolio_id);
    
    let conn = conn as &DatabaseConnection;
    let result = get_portfolio(conn, portfolio_id).await?;
    
    WebResponse::new(result).into_result()
}

#[delete("/api/portfolios/<portfolio_id>")]
pub async fn delete_portfolio_handler(
    portfolio_id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<String>> {
    info!("删除投资组合: {}", portfolio_id);
    
    let conn = conn as &DatabaseConnection;
    delete_portfolio(conn, portfolio_id).await?;
    
    WebResponse::new(format!("Portfolio {} deleted successfully", portfolio_id)).into_result()
}

#[post("/api/portfolios/<portfolio_id>/holdings", data = "<request>")]
pub async fn add_holding_handler(
    portfolio_id: i32,
    request: Json<AddHoldingRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<HoldingResponse>> {
    info!("添加持仓到投资组合 {}: {:?}", portfolio_id, request);
    
    let conn = conn as &DatabaseConnection;
    let result = add_holding(conn, portfolio_id, request.into_inner()).await?;
    
    WebResponse::new(result).into_result()
}

#[get("/api/portfolios/<portfolio_id>/holdings")]
pub async fn get_holdings_handler(
    portfolio_id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<Vec<HoldingResponse>>> {
    info!("获取投资组合 {} 的持仓列表", portfolio_id);
    
    let conn = conn as &DatabaseConnection;
    let result = get_holdings(conn, portfolio_id).await?;
    
    WebResponse::new(result).into_result()
}

#[put("/api/portfolios/<portfolio_id>/holdings/<holding_id>", data = "<request>")]
pub async fn update_holding_desc_handler(
    portfolio_id: i32,
    holding_id: i32,
    request: Json<UpdateHoldingDescRequest>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<HoldingResponse>> {
    info!("更新投资组合 {} 的持仓 {} 描述", portfolio_id, holding_id);
    
    let conn = conn as &DatabaseConnection;
    let result = update_holding_desc(conn, portfolio_id, holding_id, request.into_inner()).await?;
    
    WebResponse::new(result).into_result()
}

#[delete("/api/portfolios/<portfolio_id>/holdings/<holding_id>")]
pub async fn remove_holding_handler(
    portfolio_id: i32,
    holding_id: i32,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<String>> {
    info!("从投资组合 {} 删除持仓 {}", portfolio_id, holding_id);
    
    let conn = conn as &DatabaseConnection;
    remove_holding(conn, portfolio_id, holding_id).await?;
    
    WebResponse::new(format!("Holding {} removed successfully", holding_id)).into_result()
}
