//! 股票选股服务
//! 
//! 整合技术分析和数据获取，提供完整的选股功能

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};

use super::technical_stock_picker::{TechnicalStockPicker, TechnicalAnalysisResult, StockPickingCriteria};
use super::stock_data_provider::{StockDataProvider, DataProviderConfig};

/// 选股服务
pub struct StockPickingService {
    /// 技术分析器
    technical_picker: TechnicalStockPicker,
    /// 数据提供者
    data_provider: StockDataProvider,
    /// 配置
    config: DataProviderConfig,
}

/// 选股请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPickingRequest {
    /// 选股策略类型
    pub strategy: StockPickingStrategy,
    /// 最大返回数量
    pub max_results: Option<usize>,
    /// 指定股票代码列表（可选）
    pub stock_codes: Option<Vec<String>>,
    /// 指定市场（可选）
    pub market: Option<String>,
    /// 自定义筛选条件（可选）
    pub custom_criteria: Option<StockPickingCriteria>,
}

/// 选股策略枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StockPickingStrategy {
    /// 强烈看涨策略
    StrongBullish,
    /// 超卖反弹机会
    OversoldOpportunity,
    /// 趋势向上且RSI正常
    UptrendNormalRsi,
    /// 自定义条件
    Custom,
    /// 全面分析
    Comprehensive,
}

/// 选股响应结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPickingResponse {
    /// 选股策略
    pub strategy: StockPickingStrategy,
    /// 分析结果列表
    pub results: Vec<TechnicalAnalysisResult>,
    /// 总分析股票数量
    pub total_analyzed: usize,
    /// 筛选后股票数量
    pub filtered_count: usize,
    /// 分析时间
    pub analysis_time: String,
    /// 数据日期范围
    pub data_date_range: Option<(NaiveDate, NaiveDate)>,
}

impl StockPickingService {
    /// 创建新的选股服务
    pub fn new() -> Self {
        Self {
            technical_picker: TechnicalStockPicker::new(),
            data_provider: StockDataProvider::new(),
            config: DataProviderConfig::default(),
        }
    }
    
    /// 使用自定义配置创建选股服务
    pub fn with_config(config: DataProviderConfig) -> Self {
        Self {
            technical_picker: TechnicalStockPicker::new(),
            data_provider: StockDataProvider::new(),
            config,
        }
    }
    
    /// 执行选股
    pub async fn pick_stocks(&self, request: StockPickingRequest) -> Result<StockPickingResponse> {
        let start_time = chrono::Utc::now();
        
        info!("开始执行选股，策略: {:?}", request.strategy);
        
        // 获取股票数据
        let stocks_data = self.get_stocks_data(&request).await?;
        let total_analyzed = stocks_data.len();
        
        if stocks_data.is_empty() {
            warn!("没有获取到任何股票数据");
            return Ok(StockPickingResponse {
                strategy: request.strategy,
                results: vec![],
                total_analyzed: 0,
                filtered_count: 0,
                analysis_time: format!("{:.2}s", 0.0),
                data_date_range: None,
            });
        }
        
        // 计算数据日期范围
        let date_range = self.calculate_date_range(&stocks_data);
        
        // 根据策略执行选股
        let results = match request.strategy {
            StockPickingStrategy::StrongBullish => {
                let max_count = request.max_results.unwrap_or(20);
                self.technical_picker.get_strong_bullish_stocks(stocks_data, max_count)?
            }
            StockPickingStrategy::OversoldOpportunity => {
                let max_count = request.max_results.unwrap_or(30);
                self.technical_picker.get_oversold_opportunities(stocks_data, max_count)?
            }
            StockPickingStrategy::UptrendNormalRsi => {
                let max_count = request.max_results.unwrap_or(50);
                self.technical_picker.get_uptrend_normal_rsi_stocks(stocks_data, max_count)?
            }
            StockPickingStrategy::Custom => {
                let criteria = request.custom_criteria.unwrap_or_default();
                self.technical_picker.pick_stocks(stocks_data, &criteria)?
            }
            StockPickingStrategy::Comprehensive => {
                // 全面分析，返回所有分析结果
                let mut all_results = self.technical_picker.analyze_stocks(stocks_data)?;
                let max_count = request.max_results.unwrap_or(100);
                all_results.truncate(max_count);
                all_results
            }
        };
        
        let filtered_count = results.len();
        let elapsed = chrono::Utc::now().signed_duration_since(start_time);
        let analysis_time = format!("{:.2}s", elapsed.num_milliseconds() as f64 / 1000.0);
        
        info!(
            "选股完成，分析了 {} 只股票，筛选出 {} 只股票，耗时 {}",
            total_analyzed, filtered_count, analysis_time
        );
        
        Ok(StockPickingResponse {
            strategy: request.strategy,
            results,
            total_analyzed,
            filtered_count,
            analysis_time,
            data_date_range: date_range,
        })
    }
    
    /// 获取股票数据
    async fn get_stocks_data(&self, request: &StockPickingRequest) -> Result<HashMap<String, Vec<entity::stock_daily::Model>>> {
        if let Some(ref stock_codes) = request.stock_codes {
            // 使用指定的股票代码
            self.data_provider
                .get_multiple_stocks_data(stock_codes, self.config.default_days, None)
                .await
        } else if let Some(ref market) = request.market {
            // 使用指定市场的股票
            let stock_codes = self.data_provider.get_stock_codes_by_market(market).await?;
            self.data_provider
                .get_multiple_stocks_data(&stock_codes, self.config.default_days, None)
                .await
        } else {
            // 使用活跃股票
            self.data_provider
                .get_recent_active_stocks_data(
                    self.config.default_days,
                    Some(self.config.min_volume),
                    Some(self.config.max_stocks),
                )
                .await
        }
    }
    
    /// 计算数据日期范围
    fn calculate_date_range(&self, stocks_data: &HashMap<String, Vec<entity::stock_daily::Model>>) -> Option<(NaiveDate, NaiveDate)> {
        let mut min_date: Option<NaiveDate> = None;
        let mut max_date: Option<NaiveDate> = None;
        
        for (_, data) in stocks_data {
            if let (Some(first), Some(last)) = (data.first(), data.last()) {
                let start_date = self.parse_date_string(&first.trade_date);
                let end_date = self.parse_date_string(&last.trade_date);
                let actual_start = start_date.min(end_date);
                let actual_end = start_date.max(end_date);
                
                min_date = Some(min_date.map_or(actual_start, |d| d.min(actual_start)));
                max_date = Some(max_date.map_or(actual_end, |d| d.max(actual_end)));
            }
        }
        
        min_date.zip(max_date)
    }
    
    /// 辅助函数：解析日期字符串
    fn parse_date_string(&self, date_str: &str) -> NaiveDate {
        // 假设日期格式为 "YYYYMMDD"
        if date_str.len() == 8 {
            let year = date_str[0..4].parse().unwrap_or(2024);
            let month = date_str[4..6].parse().unwrap_or(1);
            let day = date_str[6..8].parse().unwrap_or(1);
            NaiveDate::from_ymd_opt(year, month, day).unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        } else {
            // 尝试其他格式，如 "YYYY-MM-DD"
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        }
    }
    
    /// 获取强烈看涨股票（便捷方法）
    pub async fn get_strong_bullish_stocks(&self, max_count: usize) -> Result<Vec<TechnicalAnalysisResult>> {
        let request = StockPickingRequest {
            strategy: StockPickingStrategy::StrongBullish,
            max_results: Some(max_count),
            stock_codes: None,
            market: None,
            custom_criteria: None,
        };
        
        let response = self.pick_stocks(request).await?;
        Ok(response.results)
    }
    
    /// 获取超卖反弹机会股票（便捷方法）
    pub async fn get_oversold_opportunities(&self, max_count: usize) -> Result<Vec<TechnicalAnalysisResult>> {
        let request = StockPickingRequest {
            strategy: StockPickingStrategy::OversoldOpportunity,
            max_results: Some(max_count),
            stock_codes: None,
            market: None,
            custom_criteria: None,
        };
        
        let response = self.pick_stocks(request).await?;
        Ok(response.results)
    }
    
    /// 分析指定股票列表
    pub async fn analyze_specific_stocks(&self, stock_codes: Vec<String>) -> Result<Vec<TechnicalAnalysisResult>> {
        let request = StockPickingRequest {
            strategy: StockPickingStrategy::Comprehensive,
            max_results: None,
            stock_codes: Some(stock_codes),
            market: None,
            custom_criteria: None,
        };
        
        let response = self.pick_stocks(request).await?;
        Ok(response.results)
    }
    
    /// 按市场分析股票
    pub async fn analyze_market_stocks(&self, market: String, max_count: usize) -> Result<Vec<TechnicalAnalysisResult>> {
        let request = StockPickingRequest {
            strategy: StockPickingStrategy::Comprehensive,
            max_results: Some(max_count),
            stock_codes: None,
            market: Some(market),
            custom_criteria: None,
        };
        
        let response = self.pick_stocks(request).await?;
        Ok(response.results)
    }
}

impl Default for StockPickingService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_pick_stocks_strong_bullish() {
        let service = StockPickingService::new();
        let request = StockPickingRequest {
            strategy: StockPickingStrategy::StrongBullish,
            max_results: Some(10),
            stock_codes: None,
            market: None,
            custom_criteria: None,
        };
        
        let result = service.pick_stocks(request).await;
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.results.len() <= 10);
    }
    
    #[tokio::test]
    async fn test_analyze_specific_stocks() {
        let service = StockPickingService::new();
        let stock_codes = vec!["000001.SZ".to_string(), "600000.SH".to_string()];
        
        let result = service.analyze_specific_stocks(stock_codes).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_strong_bullish_stocks() {
        let service = StockPickingService::new();
        let result = service.get_strong_bullish_stocks(5).await;
        assert!(result.is_ok());
        
        let stocks = result.unwrap();
        assert!(stocks.len() <= 5);
    }
}
