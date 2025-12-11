use rocket::{get, State};
use rocket::serde::json::Json;
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::stock::volume_distribution_service::{get_volume_distribution, VolumeDistributionResponse};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

/// 获取某个交易日的成交量分布分析
/// 
/// # 参数
/// - `trade_date`: 交易日期，格式 YYYYMMDD，例如 20240101
/// - `top_n`: 返回Top N股票详情，默认50
/// 
/// # 示例
/// ```
/// GET /api/volume-distribution?trade_date=20240101
/// GET /api/volume-distribution?trade_date=20240101&top_n=100
/// ```
/// 
/// # 返回数据说明
/// 
/// ## Top N 成交量占比
/// - `top10_pct`: Top 10股票成交量占总成交量的百分比
/// - `top30_pct`: Top 30股票成交量占总成交量的百分比
/// - `top50_pct`: Top 50股票成交量占总成交量的百分比
/// - `top100_pct`: Top 100股票成交量占总成交量的百分比
/// 
/// ## 成交量集中度指标
/// 
/// ### 1. HHI (赫芬达尔-赫希曼指数)
/// - **范围**: 0-10000
/// - **含义**: 所有股票市场份额的平方和，衡量市场集中程度
/// - **判断标准**:
///   - < 1500: 低集中度（市场竞争充分）
///   - 1500-2500: 中等集中度
///   - > 2500: 高集中度（市场较为集中）
/// - **应用**: 值越大表示成交量越集中在少数股票上
/// 
/// ### 2. Gini Coefficient (基尼系数)
/// - **范围**: 0-1
/// - **含义**: 衡量成交量分布的不均衡程度
/// - **判断标准**:
///   - < 0.3: 相对均匀（成交量分布较为均衡）
///   - 0.3-0.5: 中等不均
///   - > 0.5: 高度不均（成交量分布极不均衡）
/// - **应用**: 0表示完全均衡，1表示完全不均衡
/// 
/// ### 3. CR4/CR8 (集中度比率)
/// - **CR4**: 前4名股票成交量占比
/// - **CR8**: 前8名股票成交量占比
/// - **判断标准**:
///   - CR4 > 50%: 高度集中
///   - CR4 30-50%: 中度集中
///   - CR4 < 30%: 较为分散
/// 
/// ### 4. Entropy (熵指数)
/// - **含义**: 衡量成交量分布的混乱程度
/// - **应用**: 值越小表示越集中，值越大表示越分散
/// 
/// ## 指标应用场景
/// 
/// ### 市场分析
/// - **流动性分析**: 了解市场资金集中度，评估市场流动性风险
/// - **热点识别**: 发现资金流向和市场热点板块
/// - **风险评估**: 评估市场的健康程度和系统性风险
/// 
/// ### 投资决策
/// - **择时参考**: 高集中度可能预示市场风险（资金过度集中）
/// - **板块轮动**: 观察资金在不同股票间的流动情况
/// - **情绪判断**: 极端集中可能表示市场情绪过热或恐慌
/// 
/// ### 实战案例
/// - **牛市特征**: HHI较低，Gini系数适中，成交量分散到更多股票
/// - **熊市特征**: HHI较高，Gini系数高，成交量集中在少数股票
/// - **结构性行情**: Top10占比高，但Top100占比不高，说明热点集中
/// - **全面行情**: Top10占比适中，Top100占比高，说明普涨格局
#[get("/api/volume-distribution?<trade_date>&<top_n>")]
pub async fn get_volume_distribution_analysis(
    trade_date: String,
    top_n: Option<usize>,
    conn: &State<DatabaseConnection>,
) -> Result<WebResponse<VolumeDistributionResponse>> {
    info!("获取成交量分布分析: trade_date={}, top_n={:?}", trade_date, top_n);
    
    let conn = conn as &DatabaseConnection;
    let data = get_volume_distribution(conn, &trade_date, top_n).await?;
    
    WebResponse::new(data).into_result()
}
