//! 低位下影线策略
//! 
//! 识别股价在相对低位出现长下影线的反转信号
//! 
//! ## 策略原理
//! 
//! 下影线是技术分析中的重要信号，特别是在股价相对低位时：
//! 
//! ### 1. 下影线的含义
//! - **下影线长度**: 反映盘中最低价与收盘价（或开盘价）的差距
//! - **支撑信号**: 长下影线表示盘中虽然大幅下探，但最终获得强力支撑
//! - **买盘力量**: 说明在更低价位有大量买盘承接，多方力量强劲
//! 
//! ### 2. 低位的判断标准
//! - **相对位置**: 当前价格在近期价格区间的相对位置
//! - **历史低点**: 距离近期最低点的距离
//! - **回调幅度**: 从近期高点的回调程度
//! 
//! ### 3. 策略核心逻辑
//! - 识别相对低位（价格在近期区间的下半部分）
//! - 检测长下影线（下影线长度占全天振幅的比例）
//! - 验证反转信号（收盘价相对开盘价的位置）
//! - 评估成交量配合（放量下影线更可靠）
//! 
//! ### 4. 应用场景
//! - **短期反弹**: 适合捕捉短期反弹机会
//! - **止跌信号**: 判断下跌趋势是否出现转机
//! - **支撑确认**: 确认关键支撑位的有效性
//! 
//! ## 风险提示
//! - 下影线只是短期信号，需要结合趋势分析
//! - 在强势下跌趋势中，下影线可能只是中继反弹
//! - 需要关注后续几天的价格确认

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    SecurityData,
};

/// 低位下影线策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LowShadowConfig {
    /// 分析周期（天数）- 用于判断相对低位
    pub analysis_period: usize,
    
    /// 最小下影线长度比例（占全天振幅的百分比）
    pub min_lower_shadow_ratio: f64,
    
    /// 低位判断阈值（价格在近期区间的位置，0-1之间）
    /// 0.3表示价格在近期区间的下30%被认为是低位
    pub low_position_threshold: f64,
    
    /// 最小实体比例（避免十字星等无效信号）
    pub min_body_ratio: f64,
    
    /// 要求收盘价高于开盘价（阳线下影线更可靠）
    pub require_bullish_close: bool,
    
    /// 成交量放大倍数要求（相对于平均成交量）
    pub min_volume_ratio: f64,
    
    /// 最大上影线比例（避免上下都有长影线的情况）
    pub max_upper_shadow_ratio: f64,
}

impl Default for LowShadowConfig {
    fn default() -> Self {
        Self {
            analysis_period: 20,           // 分析过去20天判断低位
            min_lower_shadow_ratio: 0.4,   // 下影线至少占振幅40%
            low_position_threshold: 0.3,   // 价格在近期区间下30%
            min_body_ratio: 0.1,          // 实体至少占振幅10%
            require_bullish_close: true,   // 要求阳线
            min_volume_ratio: 1.2,        // 成交量至少是平均的1.2倍
            max_upper_shadow_ratio: 0.2,  // 上影线不超过振幅20%
        }
    }
}

impl StrategyConfigTrait for LowShadowConfig {
    fn strategy_name(&self) -> &str {
        "低位下影线策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.analysis_period < 5 {
            anyhow::bail!("分析周期至少需要5天");
        }
        if self.min_lower_shadow_ratio < 0.1 || self.min_lower_shadow_ratio > 0.8 {
            anyhow::bail!("下影线比例应在10%-80%之间");
        }
        if self.low_position_threshold < 0.1 || self.low_position_threshold > 0.5 {
            anyhow::bail!("低位阈值应在10%-50%之间");
        }
        if self.min_body_ratio < 0.05 || self.min_body_ratio > 0.3 {
            anyhow::bail!("最小实体比例应在5%-30%之间");
        }
        if self.min_volume_ratio < 0.5 || self.min_volume_ratio > 5.0 {
            anyhow::bail!("成交量倍数应在0.5-5.0之间");
        }
        if self.max_upper_shadow_ratio < 0.1 || self.max_upper_shadow_ratio > 0.5 {
            anyhow::bail!("最大上影线比例应在10%-50%之间");
        }
        Ok(())
    }
}

/// 低位下影线策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowShadowResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
    
    // 策略特有字段
    /// 下影线长度比例
    pub lower_shadow_ratio: f64,
    
    /// 上影线长度比例
    pub upper_shadow_ratio: f64,
    
    /// 实体长度比例
    pub body_ratio: f64,
    
    /// 价格在近期区间的位置（0-1）
    pub price_position_in_range: f64,
    
    /// 是否为阳线
    pub is_bullish: bool,
    
    /// 成交量比率（相对于平均成交量）
    pub volume_ratio: f64,
    
    /// 近期最高价
    pub recent_high: f64,
    
    /// 近期最低价
    pub recent_low: f64,
    
    /// 全天振幅
    pub daily_range: f64,
    
    /// 下影线绝对长度
    pub lower_shadow_length: f64,
    
    /// 支撑强度评分（0-100）
    pub support_strength: u8,
}

/// 低位下影线策略
pub struct LowShadowStrategy {
    config: LowShadowConfig,
}

impl LowShadowStrategy {
    pub fn new(config: LowShadowConfig) -> Self {
        Self { config }
    }
    
    /// 创建保守配置（更严格的条件）
    pub fn conservative() -> Self {
        Self {
            config: LowShadowConfig {
                min_lower_shadow_ratio: 0.5,   // 下影线至少50%
                low_position_threshold: 0.25,  // 价格在下25%
                require_bullish_close: true,   // 必须阳线
                min_volume_ratio: 1.5,         // 成交量1.5倍
                ..Default::default()
            },
        }
    }
    
    /// 创建激进配置（较宽松的条件）
    pub fn aggressive() -> Self {
        Self {
            config: LowShadowConfig {
                min_lower_shadow_ratio: 0.3,   // 下影线30%即可
                low_position_threshold: 0.4,   // 价格在下40%
                require_bullish_close: false,  // 不要求阳线
                min_volume_ratio: 1.0,         // 成交量正常即可
                ..Default::default()
            },
        }
    }
    
    /// 计算K线各部分的比例
    fn calculate_candlestick_ratios(&self, data: &SecurityData) -> (f64, f64, f64, f64) {
        let open = data.open;
        let close = data.close;
        let high = data.high;
        let low = data.low;
        
        // 避免除零
        if high == low {
            return (0.0, 0.0, 0.0, 0.0);
        }
        
        let range = high - low;
        let body = (close - open).abs();
        let is_bullish = close > open;
        
        // 计算上下影线长度
        let upper_shadow = if is_bullish {
            high - close
        } else {
            high - open
        };
        
        let lower_shadow = if is_bullish {
            open - low
        } else {
            close - low
        };
        
        // 计算比例
        let upper_shadow_ratio = upper_shadow / range;
        let lower_shadow_ratio = lower_shadow / range;
        let body_ratio = body / range;
        
        (upper_shadow_ratio, lower_shadow_ratio, body_ratio, range)
    }
    
    /// 计算价格在近期区间的位置
    fn calculate_price_position(&self, current_price: f64, data: &[SecurityData]) -> (f64, f64, f64) {
        let recent_high = data.iter()
            .map(|d| d.high)
            .fold(f64::NEG_INFINITY, f64::max);
        
        let recent_low = data.iter()
            .map(|d| d.low)
            .fold(f64::INFINITY, f64::min);
        
        let range = recent_high - recent_low;
        let position = if range > 0.0 {
            (current_price - recent_low) / range
        } else {
            0.5 // 如果没有波动，认为在中间位置
        };
        
        (position, recent_high, recent_low)
    }
    
    /// 计算平均成交量
    fn calculate_average_volume(&self, data: &[SecurityData]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let total_volume: f64 = data.iter().map(|d| d.volume).sum();
        total_volume / data.len() as f64
    }
    
    /// 计算支撑强度评分
    fn calculate_support_strength(
        &self,
        lower_shadow_ratio: f64,
        volume_ratio: f64,
        price_position: f64,
        is_bullish: bool,
    ) -> u8 {
        let mut score = 0.0;
        
        // 下影线长度评分（40分）
        let shadow_score = (lower_shadow_ratio / 0.6).min(1.0) * 40.0;
        score += shadow_score;
        
        // 成交量评分（25分）
        let volume_score = ((volume_ratio - 1.0) / 2.0).min(1.0).max(0.0) * 25.0;
        score += volume_score;
        
        // 低位评分（20分）- 越低位越好
        let position_score = (1.0 - price_position) * 20.0;
        score += position_score;
        
        // 阳线加分（15分）
        if is_bullish {
            score += 15.0;
        }
        
        score.min(100.0) as u8
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<LowShadowResult> {
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
        
        // 取最近N天的数据用于分析低位
        let analysis_data = &sorted_data[sorted_data.len() - self.config.analysis_period..];
        let latest = analysis_data.last().unwrap();
        
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 计算K线形态比例
        let (upper_shadow_ratio, lower_shadow_ratio, body_ratio, daily_range) = 
            self.calculate_candlestick_ratios(latest);
        
        // 判断是否为阳线
        let is_bullish = latest.close > latest.open;
        
        // 计算价格在近期区间的位置
        let (price_position_in_range, recent_high, recent_low) = 
            self.calculate_price_position(current_price, analysis_data);
        
        // 计算成交量比率
        let avg_volume = self.calculate_average_volume(analysis_data);
        let volume_ratio = if avg_volume > 0.0 {
            latest.volume / avg_volume
        } else {
            1.0
        };
        
        // 计算下影线绝对长度
        let lower_shadow_length = lower_shadow_ratio * daily_range;
        
        // 检查是否满足所有条件
        let mut conditions_met = Vec::new();
        let mut conditions_failed = Vec::new();
        
        // 1. 检查下影线长度
        if lower_shadow_ratio >= self.config.min_lower_shadow_ratio {
            conditions_met.push(format!("下影线长度{:.1}%符合要求", lower_shadow_ratio * 100.0));
        } else {
            conditions_failed.push(format!("下影线长度{:.1}%不足（要求{:.1}%）", 
                lower_shadow_ratio * 100.0, self.config.min_lower_shadow_ratio * 100.0));
        }
        
        // 2. 检查低位条件
        if price_position_in_range <= self.config.low_position_threshold {
            conditions_met.push(format!("处于相对低位（区间{:.1}%位置）", price_position_in_range * 100.0));
        } else {
            conditions_failed.push(format!("位置过高（区间{:.1}%位置，要求低于{:.1}%）", 
                price_position_in_range * 100.0, self.config.low_position_threshold * 100.0));
        }
        
        // 3. 检查实体大小
        if body_ratio >= self.config.min_body_ratio {
            conditions_met.push("实体大小适中".to_string());
        } else {
            conditions_failed.push(format!("实体过小{:.1}%（要求{:.1}%）", 
                body_ratio * 100.0, self.config.min_body_ratio * 100.0));
        }
        
        // 4. 检查阳线要求
        if !self.config.require_bullish_close || is_bullish {
            if is_bullish {
                conditions_met.push("阳线收盘".to_string());
            }
        } else {
            conditions_failed.push("要求阳线但实际为阴线".to_string());
        }
        
        // 5. 检查成交量
        if volume_ratio >= self.config.min_volume_ratio {
            conditions_met.push(format!("成交量放大{:.1}倍", volume_ratio));
        } else {
            conditions_failed.push(format!("成交量不足{:.1}倍（要求{:.1}倍）", 
                volume_ratio, self.config.min_volume_ratio));
        }
        
        // 6. 检查上影线
        if upper_shadow_ratio <= self.config.max_upper_shadow_ratio {
            conditions_met.push("上影线适中".to_string());
        } else {
            conditions_failed.push(format!("上影线过长{:.1}%（要求不超过{:.1}%）", 
                upper_shadow_ratio * 100.0, self.config.max_upper_shadow_ratio * 100.0));
        }
        
        // 计算支撑强度
        let support_strength = self.calculate_support_strength(
            lower_shadow_ratio,
            volume_ratio,
            price_position_in_range,
            is_bullish,
        );
        
        // 判断是否满足所有条件
        let all_conditions_met = conditions_failed.is_empty();
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = if all_conditions_met {
            self.generate_signal(
                lower_shadow_ratio,
                volume_ratio,
                price_position_in_range,
                support_strength,
                is_bullish,
            )
        } else {
            (StrategySignal::Hold, 0, 3)
        };
        
        // 生成分析说明
        let analysis_description = if all_conditions_met {
            format!(
                "低位下影线信号：{} | 支撑强度{}分",
                conditions_met.join("，"),
                support_strength
            )
        } else {
            format!(
                "不符合条件：{}",
                conditions_failed.join("；")
            )
        };
        
        debug!(
            "股票 {}: 下影线{:.1}%, 位置{:.1}%, 成交量{:.1}倍, 支撑强度{}, 信号={:?}",
            symbol, lower_shadow_ratio * 100.0, price_position_in_range * 100.0, 
            volume_ratio, support_strength, strategy_signal
        );
        
        Ok(LowShadowResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            lower_shadow_ratio,
            upper_shadow_ratio,
            body_ratio,
            price_position_in_range,
            is_bullish,
            volume_ratio,
            recent_high,
            recent_low,
            daily_range,
            lower_shadow_length,
            support_strength,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        lower_shadow_ratio: f64,
        volume_ratio: f64,
        price_position: f64,
        support_strength: u8,
        is_bullish: bool,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 下影线长度评分（30分）
        let shadow_score = ((lower_shadow_ratio / 0.6).min(1.0) * 30.0) as u8;
        signal_strength += shadow_score;
        
        // 成交量评分（25分）
        let volume_score = (((volume_ratio - 1.0) / 2.0).min(1.0).max(0.0) * 25.0) as u8;
        signal_strength += volume_score;
        
        // 低位评分（25分）
        let position_score = ((1.0 - price_position) * 25.0) as u8;
        signal_strength += position_score;
        
        // 阳线加分（20分）
        if is_bullish {
            signal_strength += 20;
        }
        
        // 根据支撑强度调整风险等级
        if support_strength >= 80 {
            risk_level = 2; // 强支撑，低风险
        } else if support_strength >= 60 {
            risk_level = 3; // 中等支撑，中等风险
        } else {
            risk_level = 4; // 弱支撑，较高风险
        }
        
        // 根据位置调整风险
        if price_position < 0.2 {
            risk_level = risk_level.saturating_sub(1); // 极低位，降低风险
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
}

impl Default for LowShadowStrategy {
    fn default() -> Self {
        Self::new(LowShadowConfig::default())
    }
}

impl TradingStrategy for LowShadowStrategy {
    type Config = LowShadowConfig;
    
    fn name(&self) -> &str {
        "低位下影线策略"
    }
    
    fn description(&self) -> &str {
        "识别股价在相对低位出现长下影线的反转信号，捕捉短期反弹机会"
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
        Ok(StrategyResult::LowShadow(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data(
        open: f64, 
        high: f64, 
        low: f64, 
        close: f64, 
        volume: f64,
        date: &str
    ) -> SecurityData {
        SecurityData {
            trade_date: date.to_string(),
            symbol: "TEST001".to_string(),
            open,
            high,
            low,
            close,
            pre_close: Some(open),
            change: Some(close - open),
            volume,
            amount: volume * close,
            turnover_rate: Some(1.0),
            pct_change: Some(((close - open) / open) * 100.0),
            time_frame: crate::strategy::traits::TimeFrame::Daily,
            security_type: crate::strategy::traits::SecurityType::Stock,
            financial_data: None,
        }
    }
    
    #[test]
    fn test_perfect_low_shadow() {
        let mut strategy = LowShadowStrategy::default();
        
        // 创建测试数据：前19天价格在100-120区间，最后一天在低位出现长下影线
        let mut data = Vec::new();
        
        // 前19天的数据，价格逐渐下跌到低位
        for i in 0..19 {
            let price = 120.0 - (i as f64 * 1.0); // 从120跌到101
            data.push(create_test_data(
                price, price + 1.0, price - 1.0, price, 
                1000000.0, &format!("2024010{:02}", i + 1)
            ));
        }
        
        // 第20天：低位长下影线阳线，成交量放大
        data.push(create_test_data(
            102.0,  // 开盘
            104.0,  // 最高
            98.0,   // 最低（长下影线）
            103.5,  // 收盘（阳线）
            1500000.0, // 成交量放大1.5倍
            "20240120"
        ));
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::LowShadow(r) = result {
            assert!(r.lower_shadow_ratio > 0.4, "下影线比例应该>40%");
            assert!(r.price_position_in_range < 0.3, "应该在低位");
            assert!(r.is_bullish, "应该是阳线");
            assert!(r.volume_ratio > 1.2, "成交量应该放大");
            assert!(r.signal_strength > 60, "信号强度应该较高");
            assert!(matches!(r.strategy_signal, StrategySignal::Buy | StrategySignal::StrongBuy));
        } else {
            panic!("Expected LowShadow result");
        }
    }
    
    #[test]
    fn test_insufficient_shadow() {
        let mut strategy = LowShadowStrategy::default();
        
        // 创建短下影线的数据
        let mut data = Vec::new();
        for i in 0..20 {
            let price = 100.0 + i as f64;
            data.push(create_test_data(
                price, price + 0.5, price - 0.1, price + 0.3, // 很短的下影线
                1000000.0, &format!("2024010{:02}", i + 1)
            ));
        }
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::LowShadow(r) = result {
            assert!(r.lower_shadow_ratio < 0.4, "下影线比例应该<40%");
            assert_eq!(r.signal_strength, 0, "不符合条件时信号强度应该为0");
            assert_eq!(r.strategy_signal, StrategySignal::Hold);
        }
    }
    
    #[test]
    fn test_config_validation() {
        let config = LowShadowConfig::default();
        assert!(config.validate().is_ok());
        
        let invalid_config = LowShadowConfig {
            analysis_period: 2, // 太短
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
        
        let invalid_config2 = LowShadowConfig {
            min_lower_shadow_ratio: 0.9, // 太大
            ..Default::default()
        };
        assert!(invalid_config2.validate().is_err());
    }
}
