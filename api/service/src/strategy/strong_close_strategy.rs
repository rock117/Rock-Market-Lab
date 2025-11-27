//! 强势收盘策略 (Strong Close Strategy)
//! 
//! 策略思想：筛选出最近N天中大多数时间收盘价强势的股票
//! - 收盘价 > 最低价
//! - 收盘价 > 开盘价  
//! - 收盘价接近最高价

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

/// 强势收盘策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongCloseConfig {
    /// 回溯天数
    pub lookback_days: usize,
    
    /// 强势天数阈值（至少有多少天满足条件）
    pub min_strong_days: usize,
    
    /// 收盘价接近最高价的阈值（百分比）
    /// 例如：2.0 表示收盘价距离最高价不超过2%
    pub close_to_high_threshold_pct: f64,
}

impl Default for StrongCloseConfig {
    fn default() -> Self {
        Self {
            lookback_days: 10,
            min_strong_days: 7,  // 10天中至少7天强势
            close_to_high_threshold_pct: 0.2f64,
        }
    }
}

impl StrategyConfig for StrongCloseConfig {
    fn strategy_name(&self) -> &str {
        "强势收盘策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.lookback_days
    }
    
    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("lookback_days 必须大于 0");
        }
        
        if self.min_strong_days == 0 {
            bail!("min_strong_days 必须大于 0");
        }
        
        if self.min_strong_days > self.lookback_days {
            bail!("min_strong_days 不能大于 lookback_days");
        }
        
        if self.close_to_high_threshold_pct < 0.0 || self.close_to_high_threshold_pct > 10.0 {
            bail!("close_to_high_threshold_pct 必须在 0-10 之间");
        }
        
        Ok(())
    }
}

/// 强势收盘策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongCloseResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 强势天数
    pub strong_days_count: usize,
    
    /// 强势天数占比（百分比）
    pub strong_days_ratio_pct: f64,
    
    /// 每日强势详情（true表示该天强势）
    pub daily_strong_flags: Vec<bool>,
    
    /// 连续强势天数
    pub consecutive_strong_days: usize,
    
    /// 平均收盘价位置（相对于当日最高最低价的位置，0-100）
    pub avg_close_position_pct: f64,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 强势收盘策略
pub struct StrongCloseStrategy {
    config: StrongCloseConfig,
}

impl StrongCloseStrategy {
    pub fn new(config: StrongCloseConfig) -> Self {
        Self { config }
    }
    
    /// 标准配置（10天中至少7天强势）
    pub fn standard() -> StrongCloseConfig {
        StrongCloseConfig::default()
    }
    
    /// 激进配置（5天中至少4天强势，短线）
    pub fn aggressive() -> StrongCloseConfig {
        StrongCloseConfig {
            lookback_days: 5,
            min_strong_days: 4,
            close_to_high_threshold_pct: 3.0,
        }
    }
    
    /// 稳健配置（20天中至少15天强势，中线）
    pub fn conservative() -> StrongCloseConfig {
        StrongCloseConfig {
            lookback_days: 20,
            min_strong_days: 15,
            close_to_high_threshold_pct: 1.5,
        }
    }
    
    /// 超强势配置（10天中至少9天强势）
    pub fn super_strong() -> StrongCloseConfig {
        StrongCloseConfig {
            lookback_days: 10,
            min_strong_days: 9,
            close_to_high_threshold_pct: 1.0,
        }
    }
}

impl TradingStrategy for StrongCloseStrategy {
    type Config = StrongCloseConfig;
    
    fn name(&self) -> &str {
        "强势收盘策略"
    }
    
    fn description(&self) -> &str {
        "筛选出最近N天中大多数时间收盘价强势的股票（收盘价>最低价、收盘价>开盘价、收盘价接近最高价）"
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
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::StrongClose(result))
    }
}

impl StrongCloseStrategy {
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<StrongCloseResult> {
        if data.is_empty() {
            bail!("数据为空");
        }
        
        if data.len() < self.config.lookback_days {
            bail!("数据不足，需要至少{}天数据，实际{}天", 
                  self.config.lookback_days, data.len());
        }
        
        // 获取最近N天的数据
        let recent_data = &data[data.len() - self.config.lookback_days..];
        let latest = data.last().unwrap();
        
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 分析每一天是否强势
        let daily_strong_flags: Vec<bool> = recent_data.iter()
            .map(|d| self.is_strong_close(d))
            .collect();
        
        // 统计强势天数
        let strong_days_count = daily_strong_flags.iter().filter(|&&x| x).count();
        let strong_days_ratio_pct = (strong_days_count as f64 / self.config.lookback_days as f64) * 100.0;
        
        // 检查是否满足最小强势天数要求
        if strong_days_count < self.config.min_strong_days {
            bail!("强势天数不足，需要至少{}天，实际{}天", 
                  self.config.min_strong_days, strong_days_count);
        }
        
        // 计算连续强势天数（从最后一天往前数）
        let consecutive_strong_days = daily_strong_flags.iter()
            .rev()
            .take_while(|&&x| x)
            .count();
        
        // 计算平均收盘价位置
        let avg_close_position_pct = self.calculate_avg_close_position(recent_data);
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            strong_days_count,
            strong_days_ratio_pct,
            consecutive_strong_days,
            avg_close_position_pct,
        );
        
        let analysis_description = self.generate_description(
            strong_days_count,
            strong_days_ratio_pct,
            consecutive_strong_days,
            avg_close_position_pct,
        );
        
        Ok(StrongCloseResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            strong_days_count,
            strong_days_ratio_pct,
            daily_strong_flags,
            consecutive_strong_days,
            avg_close_position_pct,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 判断某一天是否为强势收盘
    fn is_strong_close(&self, data: &SecurityData) -> bool {
        // 条件1：收盘价 > 最低价
        if data.close <= data.low {
            return false;
        }
        
        // 条件2：收盘价 > 开盘价（阳线）
        if data.close <= data.open {
            return false;
        }
        
        // 条件3：收盘价接近最高价
        let distance_to_high_pct = if data.high > 0.0 {
            ((data.high - data.close) / data.high) * 100.0
        } else {
            100.0
        };
        
        if distance_to_high_pct > self.config.close_to_high_threshold_pct {
            return false;
        }
        
        true
    }
    
    /// 计算平均收盘价位置（相对于当日最高最低价）
    /// 返回值：0-100，越大表示收盘价越接近最高价
    fn calculate_avg_close_position(&self, data: &[SecurityData]) -> f64 {
        let positions: Vec<f64> = data.iter()
            .filter_map(|d| {
                let range = d.high - d.low;
                if range > 0.0 {
                    Some(((d.close - d.low) / range) * 100.0)
                } else {
                    None
                }
            })
            .collect();
        
        if positions.is_empty() {
            return 50.0;
        }
        
        positions.iter().sum::<f64>() / positions.len() as f64
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        strong_days_count: usize,
        strong_days_ratio_pct: f64,
        consecutive_strong_days: usize,
        avg_close_position_pct: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 1. 强势天数占比（40分）
        if strong_days_ratio_pct >= 90.0 {
            signal_strength += 40;
        } else if strong_days_ratio_pct >= 80.0 {
            signal_strength += 35;
        } else if strong_days_ratio_pct >= 70.0 {
            signal_strength += 30;
        } else {
            signal_strength += 20;
        }
        
        // 2. 连续强势天数（30分）
        if consecutive_strong_days >= 5 {
            signal_strength += 30;
        } else if consecutive_strong_days >= 3 {
            signal_strength += 25;
        } else if consecutive_strong_days >= 2 {
            signal_strength += 20;
        } else {
            signal_strength += 10;
        }
        
        // 3. 平均收盘价位置（30分）
        if avg_close_position_pct >= 90.0 {
            signal_strength += 30;
        } else if avg_close_position_pct >= 80.0 {
            signal_strength += 25;
        } else if avg_close_position_pct >= 70.0 {
            signal_strength += 20;
        } else {
            signal_strength += 10;
        }
        
        // 确定风险等级
        risk_level = if signal_strength >= 85 {
            2  // 低风险
        } else if signal_strength >= 70 {
            3  // 中等风险
        } else {
            4  // 较高风险
        };
        
        // 确定策略信号
        let strategy_signal = if signal_strength >= 80 {
            StrategySignal::StrongBuy
        } else if signal_strength >= 60 {
            StrategySignal::Buy
        } else {
            StrategySignal::Hold
        };
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        strong_days_count: usize,
        strong_days_ratio_pct: f64,
        consecutive_strong_days: usize,
        avg_close_position_pct: f64,
    ) -> String {
        format!(
            "近{}天中有{}天强势收盘（占比{:.1}%），连续强势{}天，平均收盘价位置{:.1}%。{}",
            self.config.lookback_days,
            strong_days_count,
            strong_days_ratio_pct,
            consecutive_strong_days,
            avg_close_position_pct,
            if strong_days_ratio_pct >= 80.0 && consecutive_strong_days >= 3 {
                "持续强势，买盘力量强劲"
            } else if strong_days_ratio_pct >= 70.0 {
                "整体强势，可关注"
            } else {
                "满足基本条件"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::traits::{SecurityType, TimeFrame};
    
    fn create_strong_close_data() -> Vec<SecurityData> {
        let mut data = Vec::new();
        let base_date = 20240101;
        
        // 创建10天强势收盘的数据
        for i in 0..10 {
            let open = 10.0 + i as f64 * 0.1;
            let close = open + 0.3;  // 阳线
            let high = close + 0.05;  // 收盘价接近最高价
            let low = open - 0.1;
            
            data.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("{}", base_date + i),
                open,
                close,
                high,
                low,
                pre_close: Some(open - 0.1),
                change: Some(0.3),
                pct_change: Some(3.0),
                volume: 1000000.0,
                amount: 10000000.0,
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
            });
        }
        
        data
    }
    
    #[test]
    fn test_strong_close_strategy() {
        let config = StrongCloseStrategy::standard();
        let mut strategy = StrongCloseStrategy::new(config);
        
        let data = create_strong_close_data();
        let result = strategy.analyze("000001.SZ", &data);
        
        assert!(result.is_ok());
        let result = result.unwrap();
        
        // 解包 StrategyResult 枚举
        if let StrategyResult::StrongClose(r) = result {
            assert_eq!(r.stock_code, "000001.SZ");
            assert!(r.strong_days_count >= 7);
            assert!(r.strong_days_ratio_pct >= 70.0);
            assert!(r.signal_strength > 0);
        } else {
            panic!("Expected StrongClose result");
        }
    }
    
    #[test]
    fn test_is_strong_close() {
        let config = StrongCloseConfig::default();
        let strategy = StrongCloseStrategy::new(config);
        
        // 强势收盘：收盘价 > 开盘价，收盘价接近最高价
        let strong_data = SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20240101".to_string(),
            open: 10.0,
            close: 10.5,
            high: 10.52,  // 收盘价距离最高价不到2%
            low: 9.8,
            pre_close: Some(9.9),
            change: Some(0.5),
            pct_change: Some(5.0),
            volume: 1000000.0,
            amount: 10000000.0,
            time_frame: TimeFrame::Daily,
            security_type: SecurityType::Stock,
            financial_data: None,
        };
        
        assert!(strategy.is_strong_close(&strong_data));
        
        // 弱势收盘：收盘价 < 开盘价
        let weak_data = SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20240101".to_string(),
            open: 10.0,
            close: 9.8,
            high: 10.2,
            low: 9.7,
            pre_close: Some(10.0),
            change: Some(-0.2),
            pct_change: Some(-2.0),
            volume: 1000000.0,
            amount: 10000000.0,
            time_frame: TimeFrame::Daily,
            security_type: SecurityType::Stock,
            financial_data: None,
        };
        
        assert!(!strategy.is_strong_close(&weak_data));
    }
}
