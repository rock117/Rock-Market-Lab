//! 价量K线策略
//! 
//! 基于价格、成交量和K线形态的综合交易策略

use anyhow::Result;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tracing::{info, warn, debug};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    PriceVolumeCandlestickResult, StrategyInfo, StrategyType, RiskLevel, SecurityData
};

/// K线形态枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CandlestickPattern {
    /// 锤子线（看涨）
    Hammer,
    /// 倒锤子线（看涨）
    InvertedHammer,
    /// 上吊线（看跌）
    HangingMan,
    /// 流星线（看跌）
    ShootingStar,
    /// 十字星（变盘）
    Doji,
    /// 长阳线（看涨）
    LongBullish,
    /// 长阴线（看跌）
    LongBearish,
    /// 小阳线
    SmallBullish,
    /// 小阴线
    SmallBearish,
    /// 普通K线
    Normal,
}

/// 成交量信号
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolumeSignal {
    /// 放量上涨
    VolumeUptrend,
    /// 放量下跌
    VolumeDowntrend,
    /// 缩量上涨
    LowVolumeUptrend,
    /// 缩量下跌
    LowVolumeDowntrend,
    /// 成交量正常
    Normal,
    /// 异常放量
    AbnormalVolume,
}

// 移除本地的 StrategySignal 定义，使用 traits 中的

/// 价量K线策略特定的分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceVolumeAnalysisResult {
    /// K线形态
    pub candlestick_pattern: CandlestickPattern,
    /// 成交量信号
    pub volume_signal: VolumeSignal,
}

/// 价量K线策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceVolumeStrategyConfig {
    /// 分析周期（天数）
    pub analysis_period: usize,
    /// 成交量均线周期
    pub volume_ma_period: usize,
    /// 价格波动阈值（百分比）
    pub price_volatility_threshold: f64,
    /// 成交量放大倍数阈值
    pub volume_amplification_threshold: f64,
    /// K线实体大小阈值（百分比）
    pub candlestick_body_threshold: f64,
}

impl Default for PriceVolumeStrategyConfig {
    fn default() -> Self {
        Self {
            analysis_period: 20,
            volume_ma_period: 5,
            price_volatility_threshold: 3.0,
            volume_amplification_threshold: 1.5,
            candlestick_body_threshold: 2.0,
        }
    }
}

impl StrategyConfigTrait for PriceVolumeStrategyConfig {
    fn strategy_name(&self) -> &str {
        "PriceVolumeCandlestick"
    }
    
    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.analysis_period == 0 {
            return Err(anyhow::anyhow!("分析周期不能为0"));
        }
        if self.volume_ma_period == 0 {
            return Err(anyhow::anyhow!("成交量均线周期不能为0"));
        }
        if self.price_volatility_threshold <= 0.0 {
            return Err(anyhow::anyhow!("价格波动阈值必须大于0"));
        }
        if self.volume_amplification_threshold <= 1.0 {
            return Err(anyhow::anyhow!("成交量放大阈值必须大于1"));
        }
        Ok(())
    }
}

/// 价量K线策略
pub struct PriceVolumeCandlestickStrategy {
    config: PriceVolumeStrategyConfig,
    /// 历史数据缓存
    data_cache: VecDeque<SecurityData>,
}

impl PriceVolumeCandlestickStrategy {
    /// 创建新的策略实例
    pub fn new(config: PriceVolumeStrategyConfig) -> Self {
        Self {
            config,
            data_cache: VecDeque::new(),
        }
    }
    
    /// 分析证券数据（内部方法）
    fn analyze_internal(&mut self, symbol: &str, data: &[SecurityData]) -> Result<PriceVolumeCandlestickResult> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("证券 {} 没有数据", symbol));
        }
        
        // 按日期排序
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 获取最新数据
        let latest = sorted_data.last().unwrap();
        let current_price = latest.close;
        
        // 分析K线形态
        let candlestick_pattern = self.analyze_candlestick_pattern(&sorted_data)?;
        
        // 分析成交量信号
        let volume_signal = self.analyze_volume_signal(&sorted_data)?;
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = 
            self.generate_strategy_signal(&candlestick_pattern, &volume_signal, &sorted_data)?;
        
        // 生成分析说明
        let analysis_description = self.generate_analysis_description(
            &candlestick_pattern, &volume_signal, &strategy_signal
        );
        
        // 计算价格波动率和成交量比率
        let price_volatility = self.calculate_price_volatility(&sorted_data);
        let volume_ratio = self.calculate_volume_ratio(&sorted_data);
        
        Ok(PriceVolumeCandlestickResult {
            stock_code: symbol.to_string(),
            analysis_date: self.parse_date_string(&latest.trade_date),
            current_price,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            candlestick_pattern: format!("{:?}", candlestick_pattern),
            volume_signal: format!("{:?}", volume_signal),
            price_volatility,
            volume_ratio,
        })
    }
    
    /// 分析K线形态
    fn analyze_candlestick_pattern(&self, data: &[SecurityData]) -> Result<CandlestickPattern> {
        if data.is_empty() {
            return Ok(CandlestickPattern::Normal);
        }
        
        let latest = data.last().unwrap();
        let open = latest.open;
        let high = latest.high;
        let low = latest.low;
        let close = latest.close;
        
        // 计算K线各部分长度
        let body_size = (close - open).abs();
        let upper_shadow = high - open.max(close);
        let lower_shadow = open.min(close) - low;
        let total_range = high - low;
        
        // 避免除零
        if total_range == 0.0 {
            return Ok(CandlestickPattern::Doji);
        }
        
        let body_ratio = body_size / total_range;
        let upper_shadow_ratio = upper_shadow / total_range;
        let lower_shadow_ratio = lower_shadow / total_range;
        
        // 判断K线形态
        let pattern = if body_ratio < 0.1 {
            // 十字星
            CandlestickPattern::Doji
        } else if body_ratio > 0.7 {
            // 长K线
            if close > open {
                CandlestickPattern::LongBullish
            } else {
                CandlestickPattern::LongBearish
            }
        } else if lower_shadow_ratio > 0.6 && upper_shadow_ratio < 0.1 {
            // 下影线很长，上影线很短
            if close > open {
                CandlestickPattern::Hammer
            } else {
                CandlestickPattern::HangingMan
            }
        } else if upper_shadow_ratio > 0.6 && lower_shadow_ratio < 0.1 {
            // 上影线很长，下影线很短
            if close > open {
                CandlestickPattern::InvertedHammer
            } else {
                CandlestickPattern::ShootingStar
            }
        } else if body_ratio < 0.3 {
            // 小K线
            if close > open {
                CandlestickPattern::SmallBullish
            } else {
                CandlestickPattern::SmallBearish
            }
        } else {
            CandlestickPattern::Normal
        };
        
        Ok(pattern)
    }
    
    /// 分析成交量信号
    fn analyze_volume_signal(&self, data: &[SecurityData]) -> Result<VolumeSignal> {
        if data.len() < self.config.volume_ma_period {
            return Ok(VolumeSignal::Normal);
        }
        
        let latest = data.last().unwrap();
        let current_volume = latest.volume;
        let current_close = latest.close;
        
        // 计算成交量均线
        let volume_sum: f64 = data.iter()
            .rev()
            .take(self.config.volume_ma_period)
            .map(|d| d.volume)
            .sum();
        let volume_ma = volume_sum / self.config.volume_ma_period as f64;
        
        // 获取前一日收盘价
        let prev_close = if data.len() > 1 {
            data[data.len() - 2].close
        } else {
            current_close
        };
        
        let price_change_ratio = (current_close - prev_close) / prev_close;
        let volume_ratio = current_volume / volume_ma;
        
        // 判断成交量信号
        let signal = if volume_ratio > self.config.volume_amplification_threshold {
            if price_change_ratio > 0.02 {
                VolumeSignal::VolumeUptrend
            } else if price_change_ratio < -0.02 {
                VolumeSignal::VolumeDowntrend
            } else {
                VolumeSignal::AbnormalVolume
            }
        } else if volume_ratio < 0.8 {
            if price_change_ratio > 0.01 {
                VolumeSignal::LowVolumeUptrend
            } else if price_change_ratio < -0.01 {
                VolumeSignal::LowVolumeDowntrend
            } else {
                VolumeSignal::Normal
            }
        } else {
            VolumeSignal::Normal
        };
        
        Ok(signal)
    }
    
    /// 生成策略信号
    fn generate_strategy_signal(
        &self,
        candlestick: &CandlestickPattern,
        volume: &VolumeSignal,
        data: &[SecurityData],
    ) -> Result<(StrategySignal, u8, u8)> {
        let mut buy_score = 0i32;
        let mut sell_score = 0i32;
        
        // K线形态评分
        match candlestick {
            CandlestickPattern::Hammer | CandlestickPattern::InvertedHammer => buy_score += 30,
            CandlestickPattern::LongBullish => buy_score += 25,
            CandlestickPattern::SmallBullish => buy_score += 10,
            CandlestickPattern::HangingMan | CandlestickPattern::ShootingStar => sell_score += 30,
            CandlestickPattern::LongBearish => sell_score += 25,
            CandlestickPattern::SmallBearish => sell_score += 10,
            CandlestickPattern::Doji => {}, // 中性
            CandlestickPattern::Normal => {},
        }
        
        // 成交量信号评分
        match volume {
            VolumeSignal::VolumeUptrend => buy_score += 35,
            VolumeSignal::LowVolumeUptrend => buy_score += 15,
            VolumeSignal::VolumeDowntrend => sell_score += 35,
            VolumeSignal::LowVolumeDowntrend => sell_score += 15,
            VolumeSignal::AbnormalVolume => sell_score += 10, // 异常放量通常不好
            VolumeSignal::Normal => {},
        }
        
        // 趋势评分
        if let Some(trend_score) = self.calculate_trend_score(data) {
            if trend_score > 0.0 {
                buy_score += (trend_score * 20.0) as i32;
            } else {
                sell_score += (-trend_score * 20.0) as i32;
            }
        }
        
        // 生成最终信号
        let net_score = buy_score - sell_score;
        let signal_strength = (buy_score.max(sell_score) as f64 / 100.0 * 100.0).min(100.0) as u8;
        
        let (signal, risk_level) = if net_score >= 50 {
            (StrategySignal::StrongBuy, 3)
        } else if net_score >= 20 {
            (StrategySignal::Buy, 2)
        } else if net_score <= -50 {
            (StrategySignal::StrongSell, 4)
        } else if net_score <= -20 {
            (StrategySignal::Sell, 3)
        } else {
            (StrategySignal::Hold, 2)
        };
        
        Ok((signal, signal_strength, risk_level))
    }
    
    /// 计算趋势评分
    fn calculate_trend_score(&self, data: &[SecurityData]) -> Option<f64> {
        if data.len() < 5 {
            return None;
        }
        
        let recent_data = &data[data.len().saturating_sub(5)..];
        let prices: Vec<f64> = recent_data.iter()
            .map(|d| d.close)
            .collect();
        
        // 简单线性回归计算趋势
        let n = prices.len() as f64;
        let x_sum: f64 = (0..prices.len()).map(|i| i as f64).sum();
        let y_sum: f64 = prices.iter().sum();
        let xy_sum: f64 = prices.iter().enumerate()
            .map(|(i, &price)| i as f64 * price)
            .sum();
        let x2_sum: f64 = (0..prices.len()).map(|i| (i as f64).powi(2)).sum();
        
        let slope = (n * xy_sum - x_sum * y_sum) / (n * x2_sum - x_sum.powi(2));
        let avg_price = y_sum / n;
        
        // 标准化斜率
        Some(slope / avg_price)
    }
    
    /// 生成分析说明
    fn generate_analysis_description(
        &self,
        candlestick: &CandlestickPattern,
        volume: &VolumeSignal,
        signal: &StrategySignal,
    ) -> String {
        let pattern_desc = match candlestick {
            CandlestickPattern::Hammer => "出现锤子线，底部反转信号",
            CandlestickPattern::InvertedHammer => "出现倒锤子线，可能反转",
            CandlestickPattern::HangingMan => "出现上吊线，顶部反转信号",
            CandlestickPattern::ShootingStar => "出现流星线，看跌信号",
            CandlestickPattern::Doji => "出现十字星，变盘信号",
            CandlestickPattern::LongBullish => "长阳线，强烈看涨",
            CandlestickPattern::LongBearish => "长阴线，强烈看跌",
            CandlestickPattern::SmallBullish => "小阳线，温和看涨",
            CandlestickPattern::SmallBearish => "小阴线，温和看跌",
            CandlestickPattern::Normal => "普通K线",
        };
        
        let volume_desc = match volume {
            VolumeSignal::VolumeUptrend => "放量上涨，买盘活跃",
            VolumeSignal::VolumeDowntrend => "放量下跌，卖盘汹涌",
            VolumeSignal::LowVolumeUptrend => "缩量上涨，上涨乏力",
            VolumeSignal::LowVolumeDowntrend => "缩量下跌，抛压减轻",
            VolumeSignal::AbnormalVolume => "异常放量，需要谨慎",
            VolumeSignal::Normal => "成交量正常",
        };
        
        let signal_desc = match signal {
            StrategySignal::StrongBuy => "强烈建议买入",
            StrategySignal::Buy => "建议买入",
            StrategySignal::Hold => "建议持有观望",
            StrategySignal::Sell => "建议卖出",
            StrategySignal::StrongSell => "强烈建议卖出",
        };
        
        format!("{}；{}；{}", pattern_desc, volume_desc, signal_desc)
    }
    
    /// 辅助函数：将 Decimal 转换为 f64
    fn decimal_to_f64(&self, decimal: &Decimal) -> f64 {
        decimal.to_string().parse().unwrap_or(0.0)
    }
    
    /// 辅助函数：解析日期字符串
    fn parse_date_string(&self, date_str: &str) -> NaiveDate {
        if date_str.len() == 8 {
            let year = date_str[0..4].parse().unwrap_or(2024);
            let month = date_str[4..6].parse().unwrap_or(1);
            let day = date_str[6..8].parse().unwrap_or(1);
            NaiveDate::from_ymd_opt(year, month, day)
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        } else {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        }
    }
    
    /// 计算价格波动率
    fn calculate_price_volatility(&self, data: &[SecurityData]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        let recent_data = &data[data.len().saturating_sub(self.config.analysis_period)..];
        let prices: Vec<f64> = recent_data.iter().map(|d| d.close).collect();
        
        let mean = prices.iter().sum::<f64>() / prices.len() as f64;
        let variance = prices.iter()
            .map(|p| (p - mean).powi(2))
            .sum::<f64>() / prices.len() as f64;
        
        variance.sqrt() / mean * 100.0
    }
    
    /// 计算成交量比率
    fn calculate_volume_ratio(&self, data: &[SecurityData]) -> f64 {
        if data.len() < self.config.volume_ma_period + 1 {
            return 1.0;
        }
        
        let latest_volume = data.last().unwrap().volume;
        let recent_volumes: Vec<f64> = data.iter()
            .rev()
            .skip(1)
            .take(self.config.volume_ma_period)
            .map(|d| d.volume)
            .collect();
        
        let avg_volume = recent_volumes.iter().sum::<f64>() / recent_volumes.len() as f64;
        
        latest_volume / avg_volume
    }
    
    /// 批量分析多只证券
    pub fn batch_analyze(&mut self, securities_data: &[(String, Vec<SecurityData>)]) -> Vec<StrategyResult> {
        let mut results = Vec::new();
        
        for (symbol, daily_data) in securities_data {
            match self.analyze(symbol, daily_data) {
                Ok(result) => results.push(result),
                Err(e) => warn!("分析证券 {} 失败: {}", symbol, e),
            }
        }
        
        // 按信号强度排序
        results.sort_by(|a, b| b.signal_strength().cmp(&a.signal_strength()));
        
        results
    }
}

// 实现 TradingStrategy trait
impl TradingStrategy for PriceVolumeCandlestickStrategy {
    type Config = PriceVolumeStrategyConfig;
    
    fn name(&self) -> &str {
        "价量K线策略"
    }
    
    fn description(&self) -> &str {
        "基于价格、成交量和K线形态的综合交易策略，能够识别经典K线形态并分析成交量配合情况"
    }
    
    fn config(&self) -> &Self::Config {
        &self.config
    }
    
    fn update_config(&mut self, config: Self::Config) -> Result<()> {
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn analyze(&mut self, symbol: &str, data: &[SecurityData]) -> Result<StrategyResult> {
        self.validate_data(data)?;
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::PriceVolumeCandlestick(result))
    }
    
    fn required_data_points(&self) -> usize {
        self.config.analysis_period.max(self.config.volume_ma_period)
    }
    
    fn reset(&mut self) {
        self.data_cache.clear();
    }
}

// 实现 Default trait
impl Default for PriceVolumeCandlestickStrategy {
    fn default() -> Self {
        Self::new(PriceVolumeStrategyConfig::default())
    }
}

// 策略信息和便捷方法
impl PriceVolumeCandlestickStrategy {
    /// 获取策略信息
    pub fn info() -> StrategyInfo {
        StrategyInfo {
            name: "价量K线策略".to_string(),
            version: "1.0.0".to_string(),
            description: "基于价格、成交量和K线形态的综合交易策略".to_string(),
            strategy_type: StrategyType::Technical,
            applicable_markets: vec!["A股".to_string(), "港股".to_string(), "美股".to_string()],
            time_frames: vec!["日线".to_string(), "周线".to_string()],
            risk_level: RiskLevel::Medium,
        }
    }
    
    /// 使用保守配置创建策略
    pub fn conservative() -> Self {
        let config = PriceVolumeStrategyConfig {
            analysis_period: 30,
            volume_ma_period: 10,
            price_volatility_threshold: 2.0,
            volume_amplification_threshold: 1.2,
            candlestick_body_threshold: 1.5,
        };
        Self::new(config)
    }
    
    /// 使用激进配置创建策略
    pub fn aggressive() -> Self {
        let config = PriceVolumeStrategyConfig {
            analysis_period: 10,
            volume_ma_period: 3,
            price_volatility_threshold: 5.0,
            volume_amplification_threshold: 2.5,
            candlestick_body_threshold: 3.0,
        };
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    
    fn create_test_data() -> Vec<SecurityData> {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        vec![
            SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: "20240101".to_string(),
                open: 10.00,
                high: 10.50,
                low: 9.80,
                close: 10.20,
                pre_close: Some(10.00),
                change: Some(0.20),
                pct_change: Some(2.00),
                volume: 1000000.0,
                amount: 10200000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: "20240102".to_string(),
                open: 10.20,
                high: 10.80,
                low: 10.00,
                close: 10.50,
                pre_close: Some(10.20),
                change: Some(0.30),
                pct_change: Some(2.94),
                volume: 1200000.0,
                amount: 12600000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            },
            // 添加更多测试数据...
        ]
    }
    
    #[test]
    fn test_analyze_candlestick_pattern() {
        let strategy = PriceVolumeCandlestickStrategy::default();
        let data = create_test_data();
        
        let pattern = strategy.analyze_candlestick_pattern(&data).unwrap();
        assert_ne!(pattern, CandlestickPattern::Normal);
    }
    
    #[test]
    fn test_strategy_analysis() {
        use crate::strategy::traits::TradingStrategy;
        
        let mut strategy = PriceVolumeCandlestickStrategy::default();
        let data = create_test_data();
        
        let result = strategy.analyze("000001.SZ", &data);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.stock_code(), "000001.SZ");
        assert!(analysis.signal_strength() <= 100);
    }
    
    #[test]
    fn test_calculate_trend_score_uptrend() {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        let strategy = PriceVolumeCandlestickStrategy::default();
        
        // 创建明显上升趋势的数据：10.0 -> 10.5 -> 11.0 -> 11.5 -> 12.0
        let data = vec![
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240101".to_string(),
                open: 9.8, high: 10.2, low: 9.7, close: 10.0,
                pre_close: Some(9.8), change: Some(0.2), pct_change: Some(2.04),
                volume: 1000000.0, amount: 10000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240102".to_string(),
                open: 10.0, high: 10.7, low: 9.9, close: 10.5,
                pre_close: Some(10.0), change: Some(0.5), pct_change: Some(5.0),
                volume: 1100000.0, amount: 11000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240103".to_string(),
                open: 10.5, high: 11.2, low: 10.4, close: 11.0,
                pre_close: Some(10.5), change: Some(0.5), pct_change: Some(4.76),
                volume: 1200000.0, amount: 12000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240104".to_string(),
                open: 11.0, high: 11.7, low: 10.9, close: 11.5,
                pre_close: Some(11.0), change: Some(0.5), pct_change: Some(4.55),
                volume: 1300000.0, amount: 13000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240105".to_string(),
                open: 11.5, high: 12.2, low: 11.4, close: 12.0,
                pre_close: Some(11.5), change: Some(0.5), pct_change: Some(4.35),
                volume: 1400000.0, amount: 14000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
        ];
        
        let trend_score = strategy.calculate_trend_score(&data);
        assert!(trend_score.is_some());
        
        let score = trend_score.unwrap();
        // 上升趋势应该是正值
        assert!(score > 0.0, "上升趋势的分数应该为正，实际: {}", score);
        // 标准化后的分数应该在合理范围内（约0.04左右，即4%的趋势）
        assert!(score > 0.02 && score < 0.10, "趋势分数应该在合理范围内，实际: {}", score);
    }
    
    #[test]
    fn test_calculate_trend_score_downtrend() {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        let strategy = PriceVolumeCandlestickStrategy::default();
        
        // 创建明显下降趋势的数据：12.0 -> 11.5 -> 11.0 -> 10.5 -> 10.0
        let data = vec![
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240101".to_string(),
                open: 12.2, high: 12.3, low: 11.8, close: 12.0,
                pre_close: Some(12.2), change: Some(-0.2), pct_change: Some(-1.64),
                volume: 1000000.0, amount: 12000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240102".to_string(),
                open: 12.0, high: 12.0, low: 11.3, close: 11.5,
                pre_close: Some(12.0), change: Some(-0.5), pct_change: Some(-4.17),
                volume: 1100000.0, amount: 11500000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240103".to_string(),
                open: 11.5, high: 11.6, low: 10.8, close: 11.0,
                pre_close: Some(11.5), change: Some(-0.5), pct_change: Some(-4.35),
                volume: 1200000.0, amount: 11000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240104".to_string(),
                open: 11.0, high: 11.1, low: 10.3, close: 10.5,
                pre_close: Some(11.0), change: Some(-0.5), pct_change: Some(-4.55),
                volume: 1300000.0, amount: 10500000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240105".to_string(),
                open: 10.5, high: 10.6, low: 9.8, close: 10.0,
                pre_close: Some(10.5), change: Some(-0.5), pct_change: Some(-4.76),
                volume: 1400000.0, amount: 10000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
        ];
        
        let trend_score = strategy.calculate_trend_score(&data);
        assert!(trend_score.is_some());
        
        let score = trend_score.unwrap();
        // 下降趋势应该是负值
        assert!(score < 0.0, "下降趋势的分数应该为负，实际: {}", score);
        // 标准化后的分数应该在合理范围内（约-0.04左右，即-4%的趋势）
        assert!(score < -0.02 && score > -0.10, "趋势分数应该在合理范围内，实际: {}", score);
    }
    
    #[test]
    fn test_calculate_trend_score_sideways() {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        let strategy = PriceVolumeCandlestickStrategy::default();
        
        // 创建横盘震荡的数据：10.0 -> 10.1 -> 9.9 -> 10.0 -> 10.1
        let data = vec![
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240101".to_string(),
                open: 9.9, high: 10.2, low: 9.8, close: 10.0,
                pre_close: Some(9.9), change: Some(0.1), pct_change: Some(1.01),
                volume: 1000000.0, amount: 10000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240102".to_string(),
                open: 10.0, high: 10.3, low: 9.9, close: 10.1,
                pre_close: Some(10.0), change: Some(0.1), pct_change: Some(1.0),
                volume: 1000000.0, amount: 10100000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240103".to_string(),
                open: 10.1, high: 10.2, low: 9.7, close: 9.9,
                pre_close: Some(10.1), change: Some(-0.2), pct_change: Some(-1.98),
                volume: 1000000.0, amount: 9900000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240104".to_string(),
                open: 9.9, high: 10.2, low: 9.8, close: 10.0,
                pre_close: Some(9.9), change: Some(0.1), pct_change: Some(1.01),
                volume: 1000000.0, amount: 10000000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240105".to_string(),
                open: 10.0, high: 10.3, low: 9.9, close: 10.1,
                pre_close: Some(10.0), change: Some(0.1), pct_change: Some(1.0),
                volume: 1000000.0, amount: 10100000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
        ];
        
        let trend_score = strategy.calculate_trend_score(&data);
        assert!(trend_score.is_some());
        
        let score = trend_score.unwrap();
        // 横盘趋势应该接近0
        assert!(score.abs() < 0.02, "横盘趋势的分数应该接近0，实际: {}", score);
    }
    
    #[test]
    fn test_calculate_trend_score_insufficient_data() {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        let strategy = PriceVolumeCandlestickStrategy::default();
        
        // 只有3个数据点，少于要求的5个
        let data = vec![
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240101".to_string(),
                open: 10.0, high: 10.5, low: 9.8, close: 10.2,
                pre_close: Some(10.0), change: Some(0.2), pct_change: Some(2.0),
                volume: 1000000.0, amount: 10200000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240102".to_string(),
                open: 10.2, high: 10.8, low: 10.0, close: 10.5,
                pre_close: Some(10.2), change: Some(0.3), pct_change: Some(2.94),
                volume: 1200000.0, amount: 12600000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
            SecurityData {
                symbol: "TEST".to_string(),
                trade_date: "20240103".to_string(),
                open: 10.5, high: 11.0, low: 10.3, close: 10.8,
                pre_close: Some(10.5), change: Some(0.3), pct_change: Some(2.86),
                volume: 1300000.0, amount: 14040000.0,
                security_type: SecurityType::Stock, time_frame: TimeFrame::Daily,
            },
        ];
        
        let trend_score = strategy.calculate_trend_score(&data);
        // 数据不足应该返回 None
        assert!(trend_score.is_none(), "数据不足时应该返回 None");
    }
    
    #[test]
    fn test_calculate_trend_score_with_more_data() {
        use crate::strategy::traits::{SecurityType, TimeFrame};
        
        let strategy = PriceVolumeCandlestickStrategy::default();
        
        // 创建10个数据点，但只会使用最后5个
        let mut data = vec![];
        for i in 0..10 {
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("2024010{}", i),
                open: 10.0 + i as f64 * 0.1,
                high: 10.5 + i as f64 * 0.1,
                low: 9.8 + i as f64 * 0.1,
                close: 10.0 + i as f64 * 0.2, // 稳定上升
                pre_close: Some(10.0 + (i as f64 - 1.0) * 0.2),
                change: Some(0.2),
                pct_change: Some(2.0),
                volume: 1000000.0,
                amount: 10000000.0,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
            });
        }
        
        let trend_score = strategy.calculate_trend_score(&data);
        assert!(trend_score.is_some());
        
        let score = trend_score.unwrap();
        // 应该只使用最后5个数据点计算趋势
        println!("score = {}", score);
        assert!(score > 0.0, "上升趋势应该为正值，实际: {}", score);
    }
}
