use rocket::{get, State};
use serde::{Deserialize, Serialize};
use tracing::info;

use entity::sea_orm::DatabaseConnection;
use service::diagnosis::{diagnosis, DiagnosisResult};

use crate::response::WebResponse;
use crate::result::{IntoResult, Result};

/// 股票诊断请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockDiagnosisParams {
    /// 股票代码
    pub tscode: String,
}

/// 股票诊断接口
/// 
/// # 参数
/// * `tscode` - 股票代码，例如: 000001.SZ
/// 
/// # 返回
/// 返回股票的综合诊断结果，包括技术指标分析和投资建议
#[get("/api/stock/diagnosis?<tscode>")]
pub async fn stock_diagnosis(
    tscode: String,
    conn: &State<DatabaseConnection>
) -> Result<WebResponse<DiagnosisResult>> {
    info!("股票诊断请求 - 股票代码: {}", tscode);
    
    let conn = conn as &DatabaseConnection;
    
    // 调用诊断服务
    let diagnosis_result = diagnosis(&tscode, conn).await?;
    
    info!("股票 {} 诊断完成", tscode);
    
    WebResponse::new(diagnosis_result).into_result()
}
