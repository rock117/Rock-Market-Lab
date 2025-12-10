use anyhow::Result;
use serde::{Deserialize, Serialize};
use entity::sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait, JoinType, QuerySelect, RelationTrait};
use entity::{us_stock, us_company_info};
use entity::sea_orm;
/// 美股列表响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsStockResponse {
    /// 股票代码
    #[serde(rename = "tsCode")]
    pub ts_code: String,
    /// 交易所ID
    #[serde(rename = "exchangeId")]
    pub exchange_id: String,
    /// 股票名称
    pub name: String,
    /// 业务描述
    #[serde(rename = "businessDescription")]
    pub business_description: String,
    /// 业务所在国家
    #[serde(rename = "businessCountry")]
    pub business_country: String,
    /// 行业板块名称
    #[serde(rename = "sectorName")]
    pub sector_name: String,
    /// 具体行业名称
    #[serde(rename = "industryName")]
    pub industry_name: String,
    /// 公司网址
    #[serde(rename = "webAddress")]
    pub web_address: String,
}

/// 美股列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsStockQueryParams {
    /// 页码，从1开始
    pub page: Option<u64>,
    /// 每页大小，默认20
    pub page_size: Option<u64>,
}

/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsStockListResponse {
    /// 股票列表
    pub data: Vec<UsStockResponse>,
    /// 总数量
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
}

/// 获取美股列表
pub async fn get_us_stock_list(
    params: &UsStockQueryParams,
    conn: &DatabaseConnection,
) -> Result<UsStockListResponse> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    // 获取总数
    let total = us_stock::Entity::find().count(conn).await?;

    // 使用 JOIN 查询获取股票和公司信息
    let query_results = us_stock::Entity::find()
        .join(JoinType::LeftJoin, us_stock::Relation::UsCompanyInfo.def())
        .select_only()
        .column_as(us_stock::Column::Symbol, "symbol")
        .column_as(us_stock::Column::ExchangeId, "exchange_id")
        .column_as(us_stock::Column::Name, "name")
        .column_as(us_company_info::Column::BusinessDescription, "business_description")
        .column_as(us_company_info::Column::BusinessCountry, "business_country")
        .column_as(us_company_info::Column::SectorName, "sector_name")
        .column_as(us_company_info::Column::IndustryName, "industry_name")
        .column_as(us_company_info::Column::WebAddress, "web_address")
        .offset(offset)
        .limit(page_size)
        .into_model::<UsStockQueryResult>()
        .all(conn)
        .await?;

    // 转换为响应格式
    let data: Vec<UsStockResponse> = query_results
        .into_iter()
        .map(|result| UsStockResponse {
            ts_code: result.symbol,
            exchange_id: result.exchange_id,
            name: result.name.unwrap_or_default(),
            business_description: result.business_description.unwrap_or_default(),
            business_country: result.business_country.unwrap_or_default(),
            sector_name: result.sector_name.unwrap_or_default(),
            industry_name: result.industry_name.unwrap_or_default(),
            web_address: result.web_address.unwrap_or_default(),
        })
        .collect();

    let total_pages = (total + page_size - 1) / page_size;

    Ok(UsStockListResponse {
        data,
        total,
        page,
        page_size,
        total_pages,
    })
}

/// 查询结果结构（用于接收数据库 JOIN 查询结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsStockQueryResult {
    pub symbol: String,
    pub exchange_id: String,
    pub name: Option<String>,
    pub business_description: Option<String>,
    pub business_country: Option<String>,
    pub sector_name: Option<String>,
    pub industry_name: Option<String>,
    pub web_address: Option<String>,
}

impl sea_orm::FromQueryResult for UsStockQueryResult {
    fn from_query_result(res: &sea_orm::QueryResult, _pre: &str) -> std::result::Result<Self, sea_orm::DbErr> {
        Ok(Self {
            symbol: res.try_get("", "symbol")?,
            exchange_id: res.try_get("", "exchange_id")?,
            name: res.try_get("", "name").ok(),
            business_description: res.try_get("", "business_description").ok(),
            business_country: res.try_get("", "business_country").ok(),
            sector_name: res.try_get("", "sector_name").ok(),
            industry_name: res.try_get("", "industry_name").ok(),
            web_address: res.try_get("", "web_address").ok(),
        })
    }
}

