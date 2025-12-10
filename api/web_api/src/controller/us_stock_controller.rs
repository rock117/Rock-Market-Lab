use rocket::{get, State};
use serde::{Deserialize, Serialize};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::us_stock_service::{get_us_stock_list, UsStockQueryParams, UsStockListResponse};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

/// 美股列表查询参数（用于接收URL参数）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsStockParams {
    /// 页码，从1开始
    pub page: Option<u64>,
    /// 每页大小，默认20
    pub page_size: Option<u64>,
}

/// 获取美股列表接口
/// 
/// # 参数
/// * `page` - 页码，从1开始，默认1
/// * `page_size` - 每页大小，默认20
/// 
/// # 返回
/// 返回分页的美股列表数据
/// 
/// # 示例
/// ```
/// GET /api/us-stocks?page=1&page_size=20
/// ```
#[get("/api/us-stocks?<params..>")]
pub async fn get_us_stocks(
    params: UsStockParams,
    conn: &State<DatabaseConnection>
) -> Result<WebResponse<UsStockListResponse>> {
    info!("获取美股列表请求 - 参数: {:?}", params);
    
    let conn = conn as &DatabaseConnection;
    
    // 转换参数格式
    let query_params = UsStockQueryParams {
        page: params.page,
        page_size: params.page_size,
    };
    
    // 调用服务层
    let result = get_us_stock_list(&query_params, conn).await?;
    
    info!(
        "美股列表查询完成 - 总数: {}, 当前页: {}/{}", 
        result.total, 
        result.page, 
        result.total_pages
    );
    
    WebResponse::new(result).into_result()
}

