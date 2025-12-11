//! 单次涨停策略
//! 
//! 筛选过去N天内仅出现一次涨停的股票
//! 
//! 核心条件：
//! 1. 在指定的N天内，股票仅出现一次涨停（涨幅接近10%或20%）
//! 2. 统计N天内上涨的天数
//! 3. 计算N天内的累计涨幅
//! 
//! 策略逻辑：
//! - 单次涨停后持续上涨，说明资金持续流入，趋势强劲
//! - 避免频繁涨停（可能是游资炒作）
//! - 关注涨停后的持续性

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    SecurityData,
};

/// 单次涨停策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SingleLimitUpConfig {
    /// 分析周期（天数）
    pub analysis_period: usize,
    
    /// 涨停判断容差（百分比）- 允许略低于涨停价
    pub limit_up_tolerance: f64,
    
    /// 最小上涨天数要求
    pub min_up_days: usize,
    
    /// 最小累计涨幅要求（百分比）
    pub min_total_gain: f64,
}

impl Default for SingleLimitUpConfig {
    fn default() -> Self {
        Self {
            analysis_period: 20,        // 分析过去20个交易日
            limit_up_tolerance: 0.5,    // 容差0.5%
            min_up_days: 5,            // 至少10天上涨
            min_total_gain: 20.0,       // 累计涨幅至少20%
        }
    }
}

impl StrategyConfigTrait for SingleLimitUpConfig {
    fn strategy_name(&self) -> &str {
        "单次涨停策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.analysis_period < 5 {
            anyhow::bail!("分析周期至少需要5天");
        }
        if self.limit_up_tolerance < 0.0 || self.limit_up_tolerance > 5.0 {
            anyhow::bail!("涨停容差应在0%-5%之间");
        }
        if self.min_up_days > self.analysis_period {
            anyhow::bail!("最小上涨天数不能超过分析周期");
        }
        Ok(())
    }
}

/// 单次涨停策略
pub struct SingleLimitUpStrategy {
    config: SingleLimitUpConfig,
}

impl SingleLimitUpStrategy {
    pub fn new(config: SingleLimitUpConfig) -> Self {
        Self { config }
    }
    
    /// 根据股票代码获取涨停阈值
    /// - 688和300开头：20%（科创板、创业板）
    /// - 920开头：30%（北交所）
    /// - 其他：10%（主板、中小板）
    fn get_limit_up_threshold(&self, stock_code: &str) -> f64 {
        if stock_code.starts_with("688") || stock_code.starts_with("300") {
            20.0
        } else if stock_code.starts_with("920") {
            30.0
        } else {
            10.0
        }
    }
    
    /// 判断某天是否为涨停
    fn is_limit_up(&self, data: &SecurityData) -> bool {
        if let Some(pct_change) = data.pct_change {
            let threshold = self.get_limit_up_threshold(&data.symbol);
            let lower_bound = threshold - self.config.limit_up_tolerance;
            let upper_bound = threshold + self.config.limit_up_tolerance;
            pct_change >= lower_bound && pct_change <= upper_bound
        } else {
            false
        }
    }
    
    /// 判断某天是否上涨
    fn is_up_day(&self, data: &SecurityData) -> bool {
        if let Some(pct_change) = data.pct_change {
            pct_change > 0.0
        } else {
            data.close > data.open
        }
    }
    
    /// 计算累计涨幅
    fn calculate_total_gain(&self, data: &[SecurityData]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let first_close = data.first().unwrap().close;
        let last_close = data.last().unwrap().close;
        
        ((last_close - first_close) / first_close) * 100.0
    }
}

impl TradingStrategy for SingleLimitUpStrategy {
    type Config = SingleLimitUpConfig;
    
    fn name(&self) -> &str {
        "单次涨停策略"
    }
    
    fn description(&self) -> &str {
        "筛选过去N天内仅出现一次涨停的股票，关注涨停后的持续性"
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
        Ok(StrategyResult::SingleLimitUp(result))
    }
}

impl SingleLimitUpStrategy {
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<SingleLimitUpResult> {
        if data.len() < self.config.analysis_period {
            anyhow::bail!(
                "数据不足: 需要 {} 个数据点，实际 {} 个",
                self.config.analysis_period,
                data.len()
            );
        }
        
        // 按日期排序（从旧到新）
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 取最近N天的数据
        let analysis_data = &sorted_data[sorted_data.len() - self.config.analysis_period..];
        
        let latest = analysis_data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 收集近n日每天的涨跌幅
        let daily_changes: Vec<f64> = analysis_data.iter()
            .map(|day| day.pct_change.unwrap_or(0.0))
            .collect();
        
        // 检查当天条件：收盘价必须大于开盘价，或者当天是涨停日
        let is_today_bullish = latest.close > latest.open;
        let is_today_limit_up = self.is_limit_up(latest);
        
        if !is_today_bullish && !is_today_limit_up {
            debug!(
                "股票 {} 当天收盘价({})未高于开盘价({})且非涨停，不符合条件",
                symbol, latest.close, latest.open
            );
            return Ok(SingleLimitUpResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price,
                pct_chg: latest.pct_change.unwrap_or(0.0),
                strategy_signal: StrategySignal::Hold,
                signal_strength: 0,
                analysis_description: "当天收盘价未高于开盘价且非涨停，不符合买入条件".to_string(),
                risk_level: 3,
                limit_up_count: 0,
                limit_up_date: String::new(),
                up_days: 0,
                total_gain_pct: 0.0,
                analysis_period: self.config.analysis_period,
                daily_changes: daily_changes.clone(),
            });
        }
        
        // 1. 统计涨停次数
        let mut limit_up_count = 0;
        let mut limit_up_date = String::new();
        
        for day_data in analysis_data {
            if self.is_limit_up(day_data) {
                limit_up_count += 1;
                limit_up_date = day_data.trade_date.clone();
            }
        }
        
        debug!(
            "股票 {} 在过去 {} 天内涨停次数: {}",
            symbol, self.config.analysis_period, limit_up_count
        );
        
        // 2. 统计上涨天数
        let up_days = analysis_data.iter()
            .filter(|d| self.is_up_day(d))
            .count();
        
        // 3. 计算累计涨幅
        let total_gain_pct = self.calculate_total_gain(analysis_data);
        
        // 4. 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            limit_up_count,
            up_days,
            total_gain_pct,
            is_today_bullish,
            is_today_limit_up,
        );
        
        // 5. 生成分析说明
        let analysis_description = self.generate_description(
            limit_up_count,
            up_days,
            total_gain_pct,
            &limit_up_date,
        );
        
        info!(
            "股票 {}: 涨停{}次, 上涨{}天, 累计涨幅{:.2}%, 信号={:?}",
            symbol, limit_up_count, up_days, total_gain_pct, strategy_signal
        );
        
        Ok(SingleLimitUpResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            limit_up_count,
            limit_up_date,
            up_days,
            total_gain_pct,
            analysis_period: self.config.analysis_period,
            daily_changes,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        limit_up_count: usize,
        up_days: usize,
        total_gain_pct: f64,
        is_today_bullish: bool,
        is_today_limit_up: bool,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 当天K线状态评分（20分）- 优先级最高
        if is_today_limit_up {
            // 当天涨停，给予最高分
            signal_strength += 20;
        } else if is_today_bullish {
            // 当天阳线，给予较高分
            signal_strength += 15;
        }
        
        // 核心条件：仅一次涨停（35分）
        if limit_up_count == 1 {
            signal_strength += 35;
        } else if limit_up_count == 0 {
            // 没有涨停，不符合策略
            return (StrategySignal::Hold, 0, 3);
        } else {
            // 多次涨停，可能是游资炒作，风险较高
            risk_level = 4;
            signal_strength += 10;
        }
        
        // 上涨天数评分（25分）
        let up_days_ratio = up_days as f64 / self.config.analysis_period as f64;
        if up_days_ratio >= 0.7 {
            // 70%以上的天数上涨
            signal_strength += 25;
        } else if up_days_ratio >= 0.5 {
            // 50%-70%的天数上涨
            signal_strength += 18;
        } else if up_days >= self.config.min_up_days {
            // 达到最小上涨天数要求
            signal_strength += 10;
        }
        
        // 累计涨幅评分（20分）
        if total_gain_pct >= self.config.min_total_gain * 2.0 {
            signal_strength += 20;
        } else if total_gain_pct >= self.config.min_total_gain * 1.5 {
            signal_strength += 15;
        } else if total_gain_pct >= self.config.min_total_gain {
            signal_strength += 10;
        }
        
        // 根据累计涨幅调整风险等级
        if total_gain_pct > 50.0 {
            risk_level = 4; // 涨幅过大，风险较高
        } else if total_gain_pct < 10.0 {
            risk_level = 2; // 涨幅较小，风险较低
        }
        
        // 根据信号强度确定策略信号
        let strategy_signal = if signal_strength >= 80 {
            StrategySignal::StrongBuy
        } else if signal_strength >= 60 {
            StrategySignal::Buy
        } else if signal_strength >= 40 {
            StrategySignal::Hold
        } else if signal_strength >= 20 {
            StrategySignal::Sell
        } else {
            StrategySignal::StrongSell
        };
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        limit_up_count: usize,
        up_days: usize,
        total_gain_pct: f64,
        limit_up_date: &str,
    ) -> String {
        let mut desc = Vec::new();
        
        // 涨停情况
        if limit_up_count == 1 {
            let formatted_date = if limit_up_date.len() == 8 {
                format!(
                    "{}-{}-{}",
                    &limit_up_date[0..4],
                    &limit_up_date[4..6],
                    &limit_up_date[6..8]
                )
            } else {
                limit_up_date.to_string()
            };
            desc.push(format!(
                "✓ 期间内仅一次涨停（{}），符合单次涨停条件",
                formatted_date
            ));
        } else if limit_up_count == 0 {
            desc.push("✗ 期间内无涨停，不符合策略条件".to_string());
        } else {
            desc.push(format!(
                "⚠ 期间内涨停{}次，可能存在游资炒作风险",
                limit_up_count
            ));
        }
        
        // 上涨天数
        let up_days_ratio = (up_days as f64 / self.config.analysis_period as f64) * 100.0;
        if up_days >= self.config.min_up_days {
            desc.push(format!(
                "✓ 上涨天数{}天（{:.1}%），趋势持续性良好",
                up_days, up_days_ratio
            ));
        } else {
            desc.push(format!(
                "✗ 上涨天数仅{}天（{:.1}%），持续性不足",
                up_days, up_days_ratio
            ));
        }
        
        // 累计涨幅
        if total_gain_pct >= self.config.min_total_gain {
            desc.push(format!(
                "✓ 累计涨幅{:.2}%，涨幅达标",
                total_gain_pct
            ));
        } else {
            desc.push(format!(
                "✗ 累计涨幅仅{:.2}%，涨幅不足",
                total_gain_pct
            ));
        }
        
        desc.join("；")
    }
}

/// 单次涨停策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleLimitUpResult {
    /// 股票代码
    pub stock_code: String,
    /// 分析日期
    pub analysis_date: NaiveDate,
    /// 当前价格
    pub current_price: f64,
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    /// 策略信号
    pub strategy_signal: StrategySignal,
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    /// 分析说明
    pub analysis_description: String,
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 策略特有字段
    /// 涨停次数
    pub limit_up_count: usize,
    /// 涨停日期（如果有）
    pub limit_up_date: String,
    /// 上涨天数
    pub up_days: usize,
    /// 累计涨幅（百分比）
    pub total_gain_pct: f64,
    /// 分析周期
    pub analysis_period: usize,
    /// 近n日每天的涨跌幅（百分比）- 按日期顺序排列
    pub daily_changes: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data(days: usize, limit_up_on_day: Option<usize>) -> Vec<SecurityData> {
        let mut data = Vec::new();
        let mut price = 10.0;
        
        for i in 0..days {
            let pct_change = if Some(i) == limit_up_on_day {
                9.95 // 涨停
            } else if i % 3 == 0 {
                -1.0 // 下跌
            } else {
                2.0 // 上涨
            };
            
            let new_price = price * (1.0 + pct_change / 100.0);
            
            data.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("2024{:02}{:02}", 1, i + 1),
                open: price,
                high: new_price.max(price),
                low: new_price.min(price),
                close: new_price,
                pre_close: Some(price),
                change: Some(new_price - price),
                pct_change: Some(pct_change),
                volume: 1000000.0,
                amount: 10000000.0,
                security_type: super::traits::SecurityType::Stock,
                time_frame: super::traits::TimeFrame::Daily,
                financial_data: None,
            });
            
            price = new_price;
        }
        
        data
    }
    
    #[test]
    fn test_single_limit_up() {
        let config = SingleLimitUpConfig::default();
        let strategy = SingleLimitUpStrategy::new(config);
        
        // 创建测试数据：第10天涨停
        let data = create_test_data(20, Some(10));
        
        let result = strategy.analyze_internal("000001.SZ", &data).unwrap();
        
        assert_eq!(result.limit_up_count, 1);
        assert!(result.up_days > 0);
        println!("结果: {:?}", result);
    }
}
