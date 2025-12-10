use crate::diagnosis::{DiagnosisResult, StockDiagnosis};
use crate::strategy::traits::SecurityData;
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, Duration};
use entity::{stock_daily, stock_daily_basic};
use entity::sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

/// 获取股票诊断结果
/// 
/// # 参数
/// * `tscode` - 股票代码
/// * `conn` - 数据库连接
/// 
/// # 返回
/// 返回诊断结果或错误
pub async fn diagnosis(tscode: &str, conn: &DatabaseConnection) -> Result<DiagnosisResult> {
    // 计算90天前的日期
    let end_date = Local::now().date_naive();
    let start_date = end_date - Duration::days(90);
    
    // 获取股票数据（参考 stock_picker_service 的方法）
    let stock_data = get_stock_daily_data_with_basic(conn, tscode, &start_date, &end_date).await?;
    
    if stock_data.is_empty() {
        return Err(anyhow!("未找到股票 {} 的数据", tscode));
    }
    
    // 转换为 SecurityData（参考 stock_picker_service 的转换方式）
    let security_data: Vec<SecurityData> = stock_data
        .iter()
        .map(|(daily, basic)| SecurityData::from_daily((daily, basic)))
        .collect();
    
    if security_data.is_empty() {
        return Err(anyhow!("无法构建股票 {} 的分析数据", tscode));
    }
    
    // 执行诊断
    let diagnosis = StockDiagnosis::new();
    let result = diagnosis.diagnose(&security_data)?;
    
    Ok(result)
}

/// 获取股票日线数据和基本面数据（参考 stock_picker_service 的实现）
async fn get_stock_daily_data_with_basic(
    db: &DatabaseConnection,
    ts_code: &str,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<Vec<(stock_daily::Model, stock_daily_basic::Model)>> {
    let start = start_date.format("%Y%m%d").to_string();
    let end = end_date.format("%Y%m%d").to_string();

    // 获取日线数据
    let daily_data = stock_daily::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily::Column::TsCode, ts_code))
        .filter(stock_daily::Column::TradeDate.gte(&start))
        .filter(stock_daily::Column::TradeDate.lte(&end))
        .order_by_asc(stock_daily::Column::TradeDate)
        .all(db)
        .await?;

    // 获取基本面数据
    let basic_data = stock_daily_basic::Entity::find()
        .filter(ColumnTrait::eq(&stock_daily_basic::Column::TsCode, ts_code))
        .filter(stock_daily_basic::Column::TradeDate.gte(&start))
        .filter(stock_daily_basic::Column::TradeDate.lte(&end))
        .order_by_asc(stock_daily_basic::Column::TradeDate)
        .all(db)
        .await?;

    // 将两个数据集按日期匹配
    let mut result = Vec::new();
    for daily in daily_data {
        // 查找对应日期的基本面数据
        if let Some(basic) = basic_data.iter().find(|b| b.trade_date == daily.trade_date) {
            result.push((daily, basic.clone()));
        }
    }

    Ok(result)
}
