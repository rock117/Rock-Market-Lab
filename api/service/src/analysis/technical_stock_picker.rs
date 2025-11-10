//! 技术分析选股模块
//! 
//! 基于技术指标进行股票筛选和分析

use anyhow::Result;
use chrono::{NaiveDate, Duration};
use rust_decimal::Decimal;
use common::indicators::{ma, ema, rsi, macd, IndicatorResult};
use entity::stock_daily;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};

/// 辅助函数：将 Decimal 转换为 f64
fn decimal_to_f64(decimal: &Decimal) -> f64 {
    decimal.to_string().parse().unwrap_or(0.0)
}

/// 辅助函数：解析日期字符串
fn parse_date_string(date_str: &str) -> NaiveDate {
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

/// 股票技术分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAnalysisResult {
    /// 股票代码
    pub stock_code: String,
    /// 股票名称
    pub stock_name: Option<String>,
    /// 分析日期
    pub analysis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 开盘价
    pub opening_price: f64,
    /// 最高价
    pub daily_high: f64,
    /// 最低价
    pub daily_low: f64,
    /// 成交量
    pub volume: f64,
    
    // 技术指标
    /// 5日均线
    pub ma5: Option<f64>,
    /// 20日均线
    pub ma20: Option<f64>,
    /// 60日均线
    pub ma60: Option<f64>,
    /// 12日指数移动平均
    pub ema12: Option<f64>,
    /// 26日指数移动平均
    pub ema26: Option<f64>,
    /// RSI(14)
    pub rsi14: Option<f64>,
    /// MACD线
    pub macd_line: Option<f64>,
    /// MACD信号线
    pub macd_signal: Option<f64>,
    /// MACD柱状图
    pub macd_histogram: Option<f64>,
    /// 成交量5日均线
    pub volume_ma5: Option<f64>,
    
    // 分析结果
    /// 短期趋势（MA5 > MA20）
    pub short_trend: bool,
    /// 长期趋势（MA20 > MA60）
    pub long_trend: bool,
    /// 整体看涨
    pub overall_bullish: bool,
    /// RSI超卖
    pub oversold: bool,
    /// RSI超买
    pub overbought: bool,
    /// RSI中性
    pub rsi_neutral: bool,
    /// 成交量放大
    pub volume_surge: bool,
    /// 综合分析结果
    pub analysis_result: String,
    /// 分析评分 (0-100)
    pub score: u8,
}

/// 选股条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPickingCriteria {
    /// 最小RSI值
    pub min_rsi: Option<f64>,
    /// 最大RSI值
    pub max_rsi: Option<f64>,
    /// 要求短期趋势向上
    pub require_short_trend: bool,
    /// 要求长期趋势向上
    pub require_long_trend: bool,
    /// 要求成交量放大
    pub require_volume_surge: bool,
    /// 最小评分
    pub min_score: u8,
    /// 最大返回数量
    pub max_results: usize,
}

impl Default for StockPickingCriteria {
    fn default() -> Self {
        Self {
            min_rsi: None,
            max_rsi: None,
            require_short_trend: false,
            require_long_trend: false,
            require_volume_surge: false,
            min_score: 60,
            max_results: 50,
        }
    }
}

/// 技术分析选股器
pub struct TechnicalStockPicker {
    // 这里可以添加数据库连接等
}

impl TechnicalStockPicker {
    /// 创建新的选股器
    pub fn new() -> Self {
        Self {}
    }
    
    /// 对单只股票进行技术分析
    pub fn analyze_stock(&self, stock_code: &str, daily_data: &[stock_daily::Model]) -> Result<TechnicalAnalysisResult> {
        if daily_data.is_empty() {
            return Err(anyhow::anyhow!("股票 {} 没有日线数据", stock_code));
        }
        
        // 按日期排序
        let mut sorted_data = daily_data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 获取最新数据
        let latest = sorted_data.last().unwrap();
        
        // 提取价格和成交量数据
        let closes: Vec<f64> = sorted_data.iter().map(|d| decimal_to_f64(&d.close)).collect();
        let opens: Vec<f64> = sorted_data.iter().map(|d| decimal_to_f64(&d.open)).collect();
        let highs: Vec<f64> = sorted_data.iter().map(|d| decimal_to_f64(&d.high)).collect();
        let lows: Vec<f64> = sorted_data.iter().map(|d| decimal_to_f64(&d.low)).collect();
        let volumes: Vec<f64> = sorted_data.iter().map(|d| decimal_to_f64(&d.vol)).collect();
        
        // 计算技术指标
        let ma5_values = ma(&closes, 5).unwrap_or_default();
        let ma20_values = ma(&closes, 20).unwrap_or_default();
        let ma60_values = ma(&closes, 60).unwrap_or_default();
        let ema12_values = ema(&closes, 12).unwrap_or_default();
        let ema26_values = ema(&closes, 26).unwrap_or_default();
        let rsi14_values = rsi(&closes, 14).unwrap_or_default();
        let macd_values = macd(&closes, 12, 26, 9).unwrap_or_default();
        let volume_ma5_values = ma(&volumes, 5).unwrap_or_default();
        
        // 获取最新指标值
        let ma5 = ma5_values.last().copied();
        let ma20 = ma20_values.last().copied();
        let ma60 = ma60_values.last().copied();
        let ema12 = ema12_values.last().copied();
        let ema26 = ema26_values.last().copied();
        let rsi14 = rsi14_values.last().copied();
        let volume_ma5 = volume_ma5_values.last().copied();
        
        let (macd_line, macd_signal, macd_histogram) = macd_values.last()
            .map(|(line, signal, hist)| (Some(*line), Some(*signal), Some(*hist)))
            .unwrap_or((None, None, None));
        
        // 趋势判断
        let short_trend = ma5.zip(ma20).map(|(ma5, ma20)| ma5 > ma20).unwrap_or(false);
        let long_trend = ma20.zip(ma60).map(|(ma20, ma60)| ma20 > ma60).unwrap_or(false);
        let overall_bullish = short_trend && long_trend;
        
        // RSI超买超卖判断
        let oversold = rsi14.map(|rsi| rsi < 30.0).unwrap_or(false);
        let overbought = rsi14.map(|rsi| rsi > 70.0).unwrap_or(false);
        let rsi_neutral = rsi14.map(|rsi| rsi >= 30.0 && rsi <= 70.0).unwrap_or(false);
        
        // 成交量分析
        let current_volume = decimal_to_f64(&latest.vol);
        let volume_surge = volume_ma5
            .map(|vol_ma5| current_volume > vol_ma5 * 1.5)
            .unwrap_or(false);
        
        // 综合分析结果
        let (analysis_result, score) = self.generate_analysis_result(
            overall_bullish, rsi_neutral, volume_surge, oversold, overbought
        );
        
        Ok(TechnicalAnalysisResult {
            stock_code: stock_code.to_string(),
            stock_name: None, // 可以从其他表查询
            analysis_date: parse_date_string(&latest.trade_date),
            current_price: decimal_to_f64(&latest.close),
            opening_price: decimal_to_f64(&latest.open),
            daily_high: decimal_to_f64(&latest.high),
            daily_low: decimal_to_f64(&latest.low),
            volume: current_volume,
            
            ma5,
            ma20,
            ma60,
            ema12,
            ema26,
            rsi14,
            macd_line,
            macd_signal,
            macd_histogram,
            volume_ma5,
            
            short_trend,
            long_trend,
            overall_bullish,
            oversold,
            overbought,
            rsi_neutral,
            volume_surge,
            analysis_result,
            score,
        })
    }
    
    /// 生成综合分析结果
    fn generate_analysis_result(
        &self,
        overall_bullish: bool,
        rsi_neutral: bool,
        volume_surge: bool,
        oversold: bool,
        overbought: bool,
    ) -> (String, u8) {
        if overall_bullish && rsi_neutral && volume_surge {
            ("强烈看涨：趋势向上，RSI正常，成交量放大".to_string(), 90)
        } else if overall_bullish && !overbought {
            ("看涨：趋势向上，RSI未超买".to_string(), 75)
        } else if !overall_bullish && !oversold {
            ("看跌：趋势向下，RSI未超卖".to_string(), 25)
        } else if oversold {
            ("超卖反弹机会".to_string(), 65)
        } else if overbought {
            ("超买回调风险".to_string(), 35)
        } else {
            ("震荡整理".to_string(), 50)
        }
    }
    
    /// 批量分析股票
    pub fn analyze_stocks(&self, stocks_data: HashMap<String, Vec<stock_daily::Model>>) -> Result<Vec<TechnicalAnalysisResult>> {
        let mut results = Vec::new();
        
        for (stock_code, daily_data) in stocks_data {
            match self.analyze_stock(&stock_code, &daily_data) {
                Ok(analysis) => results.push(analysis),
                Err(e) => {
                    warn!("分析股票 {} 失败: {}", stock_code, e);
                }
            }
        }
        
        // 按评分排序
        results.sort_by(|a, b| b.score.cmp(&a.score));
        
        Ok(results)
    }
    
    /// 根据条件筛选股票
    pub fn pick_stocks(
        &self,
        stocks_data: HashMap<String, Vec<stock_daily::Model>>,
        criteria: &StockPickingCriteria,
    ) -> Result<Vec<TechnicalAnalysisResult>> {
        let mut all_results = self.analyze_stocks(stocks_data)?;
        
        // 应用筛选条件
        all_results.retain(|result| {
            // 检查RSI范围
            if let Some(min_rsi) = criteria.min_rsi {
                if result.rsi14.map(|rsi| rsi < min_rsi).unwrap_or(true) {
                    return false;
                }
            }
            
            if let Some(max_rsi) = criteria.max_rsi {
                if result.rsi14.map(|rsi| rsi > max_rsi).unwrap_or(true) {
                    return false;
                }
            }
            
            // 检查趋势要求
            if criteria.require_short_trend && !result.short_trend {
                return false;
            }
            
            if criteria.require_long_trend && !result.long_trend {
                return false;
            }
            
            // 检查成交量要求
            if criteria.require_volume_surge && !result.volume_surge {
                return false;
            }
            
            // 检查最小评分
            if result.score < criteria.min_score {
                return false;
            }
            
            true
        });
        
        // 限制返回数量
        all_results.truncate(criteria.max_results);
        
        info!("技术分析选股完成，共筛选出 {} 只股票", all_results.len());
        
        Ok(all_results)
    }
    
    /// 获取强烈看涨的股票
    pub fn get_strong_bullish_stocks(
        &self,
        stocks_data: HashMap<String, Vec<stock_daily::Model>>,
        max_count: usize,
    ) -> Result<Vec<TechnicalAnalysisResult>> {
        let criteria = StockPickingCriteria {
            min_rsi: Some(30.0),
            max_rsi: Some(70.0),
            require_short_trend: true,
            require_long_trend: true,
            require_volume_surge: true,
            min_score: 80,
            max_results: max_count,
        };
        
        self.pick_stocks(stocks_data, &criteria)
    }
    
    /// 获取超卖反弹机会股票
    pub fn get_oversold_opportunities(
        &self,
        stocks_data: HashMap<String, Vec<stock_daily::Model>>,
        max_count: usize,
    ) -> Result<Vec<TechnicalAnalysisResult>> {
        let criteria = StockPickingCriteria {
            min_rsi: None,
            max_rsi: Some(30.0),
            require_short_trend: false,
            require_long_trend: false,
            require_volume_surge: false,
            min_score: 60,
            max_results: max_count,
        };
        
        self.pick_stocks(stocks_data, &criteria)
    }
    
    /// 获取趋势向上且RSI正常的股票
    pub fn get_uptrend_normal_rsi_stocks(
        &self,
        stocks_data: HashMap<String, Vec<stock_daily::Model>>,
        max_count: usize,
    ) -> Result<Vec<TechnicalAnalysisResult>> {
        let criteria = StockPickingCriteria {
            min_rsi: Some(30.0),
            max_rsi: Some(70.0),
            require_short_trend: true,
            require_long_trend: false,
            require_volume_surge: false,
            min_score: 70,
            max_results: max_count,
        };
        
        self.pick_stocks(stocks_data, &criteria)
    }
}

impl Default for TechnicalStockPicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    
    fn create_test_stock_data() -> Vec<stock_daily::Model> {
        use rust_decimal::Decimal;
        
        vec![
            stock_daily::Model {
                ts_code: "000001.SZ".to_string(),
                trade_date: "20240101".to_string(),
                open: Decimal::new(100, 1), // 10.0
                high: Decimal::new(105, 1), // 10.5
                low: Decimal::new(98, 1),   // 9.8
                close: Decimal::new(102, 1), // 10.2
                pre_close: Some(Decimal::new(100, 1)), // 10.0
                change: Some(Decimal::new(2, 1)), // 0.2
                pct_chg: Some(Decimal::new(20, 1)), // 2.0
                vol: Decimal::new(1000000, 0), // 1000000
                amount: Decimal::new(10200000, 0), // 10200000.0
            },
            stock_daily::Model {
                ts_code: "000001.SZ".to_string(),
                trade_date: "20240102".to_string(),
                open: Decimal::new(102, 1), // 10.2
                high: Decimal::new(108, 1), // 10.8
                low: Decimal::new(100, 1),  // 10.0
                close: Decimal::new(105, 1), // 10.5
                pre_close: Some(Decimal::new(102, 1)), // 10.2
                change: Some(Decimal::new(3, 1)), // 0.3
                pct_chg: Some(Decimal::new(294, 2)), // 2.94
                vol: Decimal::new(1200000, 0), // 1200000
                amount: Decimal::new(12600000, 0), // 12600000.0
            },
            // 添加更多测试数据...
        ]
    }
    
    #[test]
    fn test_analyze_stock() {
        let picker = TechnicalStockPicker::new();
        let test_data = create_test_stock_data();
        
        let result = picker.analyze_stock("000001.SZ", &test_data);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.stock_code, "000001.SZ");
        assert!(analysis.current_price > 0.0);
    }
    
    #[test]
    fn test_pick_stocks_with_criteria() {
        let picker = TechnicalStockPicker::new();
        let mut stocks_data = HashMap::new();
        stocks_data.insert("000001.SZ".to_string(), create_test_stock_data());
        
        let criteria = StockPickingCriteria {
            min_score: 0,
            max_results: 10,
            ..Default::default()
        };
        
        let result = picker.pick_stocks(stocks_data, &criteria);
        assert!(result.is_ok());
    }
}
