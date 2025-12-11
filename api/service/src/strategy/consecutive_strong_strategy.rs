//! 连续强势股策略
//! 
//! 筛选出近n天每天都是强势股的股票
//! 强势股定义：收盘价 > 最低价 且 收盘价 > 开盘价

use super::traits::*;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// 连续强势股策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveStrongConfig {
    /// 分析周期（天数）- 默认 5 天
    pub analysis_period: usize,
    
    /// 要求的最少连续强势天数 - 默认等于分析周期（即全部天数都强势）
    pub min_consecutive_days: usize,
}

impl Default for ConsecutiveStrongConfig {
    fn default() -> Self {
        Self {
            analysis_period: 5,
            min_consecutive_days: 5,
        }
    }
}

impl StrategyConfig for ConsecutiveStrongConfig {
    fn strategy_name(&self) -> &str {
        "consecutive_strong"
    }
    
    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.analysis_period == 0 {
            bail!("分析周期不能为0");
        }
        if self.min_consecutive_days == 0 {
            bail!("最少连续强势天数不能为0");
        }
        if self.min_consecutive_days > self.analysis_period {
            bail!("最少连续强势天数不能大于分析周期");
        }
        Ok(())
    }
}

/// 连续强势股策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveStrongResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// 分析周期
    pub analysis_period: usize,
    
    /// 实际连续强势天数
    pub consecutive_strong_days: usize,
    
    /// 强势天数占比
    pub strong_days_ratio: f64,
    
    /// 每日是否强势（true=强势，false=弱势）
    pub daily_strong_flags: Vec<bool>,
    
    /// 每日涨幅（%）
    pub daily_changes: Vec<f64>,
    
    /// 累计涨幅（%）
    pub total_gain_pct: f64,
    
    /// 平均日涨幅（%）
    pub avg_daily_gain: f64,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 连续强势股策略
pub struct ConsecutiveStrongStrategy {
    config: ConsecutiveStrongConfig,
}

impl ConsecutiveStrongStrategy {
    pub fn new(config: ConsecutiveStrongConfig) -> Self {
        Self { config }
    }
    
    /// 创建3天连续强势配置
    pub fn three_days() -> ConsecutiveStrongConfig {
        ConsecutiveStrongConfig {
            analysis_period: 3,
            min_consecutive_days: 3,
        }
    }
    
    /// 创建5天连续强势配置
    pub fn five_days() -> ConsecutiveStrongConfig {
        ConsecutiveStrongConfig {
            analysis_period: 5,
            min_consecutive_days: 5,
        }
    }
    
    /// 创建10天连续强势配置
    pub fn ten_days() -> ConsecutiveStrongConfig {
        ConsecutiveStrongConfig {
            analysis_period: 10,
            min_consecutive_days: 10,
        }
    }
    
    /// 创建宽松配置（10天中至少8天强势）
    pub fn relaxed() -> ConsecutiveStrongConfig {
        ConsecutiveStrongConfig {
            analysis_period: 10,
            min_consecutive_days: 8,
        }
    }
    
    /// 判断单日是否为强势股
    /// 强势定义：收盘价 > 最低价 且 收盘价 > 开盘价
    fn is_strong_day(&self, data: &SecurityData) -> bool {
        data.close > data.low && data.close > data.open
    }
    
    /// 计算连续强势天数（从最近一天往前数）
    fn count_consecutive_strong_days(&self, daily_flags: &[bool]) -> usize {
        let mut count = 0;
        for &is_strong in daily_flags.iter().rev() {
            if is_strong {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<ConsecutiveStrongResult> {
        if data.len() < self.config.analysis_period {
            bail!("数据不足，需要至少{}天数据，当前只有{}天", 
                  self.config.analysis_period, data.len());
        }
        
        // 获取分析窗口数据
        let analysis_data = &data[data.len() - self.config.analysis_period..];
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 判断每日是否强势
        let daily_strong_flags: Vec<bool> = analysis_data
            .iter()
            .map(|d| self.is_strong_day(d))
            .collect();
        
        // 计算每日涨跌幅
        let daily_changes: Vec<f64> = analysis_data
            .iter()
            .map(|d| d.pct_change.unwrap_or(0.0))
            .collect();
        
        // 计算连续强势天数
        let consecutive_strong_days = self.count_consecutive_strong_days(&daily_strong_flags);
        
        // 计算强势天数占比
        let strong_count = daily_strong_flags.iter().filter(|&&x| x).count();
        let strong_days_ratio = strong_count as f64 / daily_strong_flags.len() as f64;
        
        // 计算累计涨幅和平均日涨幅
        let total_gain_pct: f64 = daily_changes.iter().sum();
        let avg_daily_gain = total_gain_pct / daily_changes.len() as f64;
        
        // 判断是否满足条件
        let meets_criteria = consecutive_strong_days >= self.config.min_consecutive_days;
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = if meets_criteria {
            self.generate_signal(
                consecutive_strong_days,
                strong_days_ratio,
                total_gain_pct,
                avg_daily_gain,
            )
        } else {
            (StrategySignal::Hold, 0, 3)
        };
        
        let analysis_description = self.generate_description(
            consecutive_strong_days,
            strong_days_ratio,
            total_gain_pct,
            avg_daily_gain,
            meets_criteria,
        );
        
        Ok(ConsecutiveStrongResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            analysis_period: self.config.analysis_period,
            consecutive_strong_days,
            strong_days_ratio,
            daily_strong_flags,
            daily_changes,
            total_gain_pct,
            avg_daily_gain,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        consecutive_days: usize,
        strong_ratio: f64,
        total_gain: f64,
        avg_gain: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 连续天数评分（40分）
        let days_score = ((consecutive_days as f64 / self.config.analysis_period as f64) * 40.0) as u8;
        signal_strength += days_score;
        
        // 强势占比评分（30分）
        let ratio_score = (strong_ratio * 30.0) as u8;
        signal_strength += ratio_score;
        
        // 累计涨幅评分（20分）
        let gain_score = if total_gain > 20.0 {
            20
        } else if total_gain > 10.0 {
            15
        } else if total_gain > 5.0 {
            10
        } else if total_gain > 0.0 {
            5
        } else {
            0
        };
        signal_strength += gain_score;
        
        // 平均日涨幅评分（10分）
        let avg_score = if avg_gain > 3.0 {
            10
        } else if avg_gain > 2.0 {
            8
        } else if avg_gain > 1.0 {
            5
        } else if avg_gain > 0.0 {
            3
        } else {
            0
        };
        signal_strength += avg_score;
        
        // 根据涨幅调整风险等级
        if total_gain > 15.0 {
            risk_level = 2; // 涨幅大，风险较低
        } else if total_gain > 8.0 {
            risk_level = 3; // 中等风险
        } else {
            risk_level = 4; // 涨幅小，风险较高
        }
        
        // 根据连续天数调整风险
        if consecutive_days >= self.config.analysis_period {
            risk_level = risk_level.saturating_sub(1); // 全部连续，降低风险
        }
        
        let strategy_signal = if signal_strength >= 80 {
            StrategySignal::StrongBuy
        } else if signal_strength >= 65 {
            StrategySignal::Buy
        } else if signal_strength >= 50 {
            StrategySignal::Hold
        } else {
            StrategySignal::Sell
        };
        
        (strategy_signal, signal_strength, risk_level.max(1).min(5))
    }
    
    /// 生成分析描述
    fn generate_description(
        &self,
        consecutive_days: usize,
        strong_ratio: f64,
        total_gain: f64,
        avg_gain: f64,
        meets_criteria: bool,
    ) -> String {
        if !meets_criteria {
            return format!(
                "连续强势天数不足：最近{}天中仅{}天连续强势（要求至少{}天），强势占比{:.1}%",
                self.config.analysis_period,
                consecutive_days,
                self.config.min_consecutive_days,
                strong_ratio * 100.0
            );
        }
        
        format!(
            "连续{}天强势（{}天中{:.0}%），累计涨幅{:.2}%，日均涨幅{:.2}%",
            consecutive_days,
            self.config.analysis_period,
            strong_ratio * 100.0,
            total_gain,
            avg_gain
        )
    }
}

impl TradingStrategy for ConsecutiveStrongStrategy {
    type Config = ConsecutiveStrongConfig;
    
    fn name(&self) -> &str {
        "连续强势股策略"
    }
    
    fn description(&self) -> &str {
        "筛选近n天每天都是强势股的股票（强势定义：收盘价>最低价且收盘价>开盘价）"
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
        info!("开始分析股票 {} 的连续强势情况", symbol);
        
        let result = self.analyze_internal(symbol, data)?;
        
        debug!(
            "股票 {} 分析完成：连续{}天强势，信号强度{}",
            symbol, result.consecutive_strong_days, result.signal_strength
        );
        
        Ok(StrategyResult::ConsecutiveStrong(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data(days: usize, all_strong: bool) -> Vec<SecurityData> {
        let mut data = Vec::new();
        let base_date = 20240101;
        
        for i in 0..days {
            let (open, close, high, low) = if all_strong {
                // 强势：收盘 > 开盘 且 收盘 > 最低
                (100.0 + i as f64, 102.0 + i as f64, 103.0 + i as f64, 99.0 + i as f64)
            } else {
                // 弱势：收盘 < 开盘
                (102.0 + i as f64, 100.0 + i as f64, 103.0 + i as f64, 99.0 + i as f64)
            };
            
            data.push(SecurityData {
                trade_date: format!("{}", base_date + i),
                open,
                close,
                high,
                low,
                volume: 1000000.0,
                amount: 100000000.0,
                pct_change: Some(2.0),
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
            });
        }
        
        data
    }
    
    #[test]
    fn test_consecutive_strong_all_strong() {
        let config = ConsecutiveStrongConfig {
            analysis_period: 5,
            min_consecutive_days: 5,
        };
        let mut strategy = ConsecutiveStrongStrategy::new(config);
        let data = create_test_data(5, true);
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::ConsecutiveStrong(r) = result {
            assert_eq!(r.consecutive_strong_days, 5);
            assert_eq!(r.strong_days_ratio, 1.0);
            assert!(r.signal_strength > 50);
        } else {
            panic!("Expected ConsecutiveStrong result");
        }
    }
    
    #[test]
    fn test_consecutive_strong_not_strong() {
        let config = ConsecutiveStrongConfig {
            analysis_period: 5,
            min_consecutive_days: 5,
        };
        let mut strategy = ConsecutiveStrongStrategy::new(config);
        let data = create_test_data(5, false);
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::ConsecutiveStrong(r) = result {
            assert_eq!(r.consecutive_strong_days, 0);
            assert_eq!(r.signal_strength, 0);
        } else {
            panic!("Expected ConsecutiveStrong result");
        }
    }
}
