//! 长期底部反转策略
//! 
//! 用于识别经过长期下跌（1-2年）后，出现底部企稳并开始反转的股票

use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, 
    StrategyResult, StrategySignal, SecurityData,
};

/// 长期底部反转策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LongTermBottomReversalConfig {
    /// 长期分析周期（天数）- 需要的总数据量，默认 480 天（约2年）
    pub long_term_period: usize,
    
    /// 底部确认周期（天数）- 用于判断横盘整理的时间窗口，默认 60 天
    pub bottom_confirm_period: usize,
    
    /// 反转确认周期（天数）- 用于判断向上趋势的时间窗口，默认 20 天
    pub reversal_confirm_period: usize,
    
    /// 从历史高点最小跌幅（百分比）- 确认长期下跌的阈值，默认 50.0%
    pub min_decline_from_high: f64,
    
    /// 历史高点回溯周期（天数）- 查找历史最高价的时间范围，默认 720 天（约2年）
    pub high_lookback_period: usize,
    
    /// 成交量萎缩阈值（相对历史平均）- 地量判断标准，默认 0.4（萎缩到40%以下）
    pub volume_shrink_threshold: f64,
    
    /// 历史成交量计算周期（天数）- 计算历史平均成交量的时间范围，默认 240 天（约1年）
    pub historical_volume_period: usize,
    
    /// 横盘价格波动范围（百分比）- 判断是否横盘的价格波动上限，默认 15.0%
    pub consolidation_range: f64,
    
    /// 横盘最短持续天数 - 横盘整理的最少天数要求，默认 30 天
    pub min_consolidation_days: usize,
    
    /// 底部价格容差（百分比）- 判断多次探底时的价格容差范围，默认 3.0%
    pub bottom_tolerance: f64,
    
    /// 最少探底次数 - 确认底部支撑的最少触底次数，默认 3 次
    pub min_bottom_tests: usize,
    
    /// 阳线占比阈值 - 反转确认期内阳线的最低占比，默认 0.6（60%）
    pub bullish_ratio_threshold: f64,
    
    /// 温和放量阈值（相对地量期均值）- 判断温和放量的倍数，默认 1.3 倍
    pub moderate_volume_threshold: f64,
    
    /// 价格抬升幅度（百分比，相对底部）- 价格从底部抬升的最小幅度，默认 5.0%
    pub price_lift_threshold: f64,
    
    /// 均线多头排列判断 - 是否要求 MA5 > MA10 > MA20，默认 true
    pub require_ma_bullish: bool,
}

impl Default for LongTermBottomReversalConfig {
    fn default() -> Self {
        Self {
            long_term_period: 480,
            bottom_confirm_period: 60,
            reversal_confirm_period: 20,
            min_decline_from_high: 50.0,
            high_lookback_period: 720,
            volume_shrink_threshold: 0.4,
            historical_volume_period: 240,
            consolidation_range: 15.0,
            min_consolidation_days: 30,
            bottom_tolerance: 3.0,
            min_bottom_tests: 3,
            bullish_ratio_threshold: 0.6,
            moderate_volume_threshold: 1.3,
            price_lift_threshold: 5.0,
            require_ma_bullish: true,
        }
    }
}

impl StrategyConfigTrait for LongTermBottomReversalConfig {
    fn strategy_name(&self) -> &str {
        "LongTermBottomReversal"
    }
    
    fn analysis_period(&self) -> usize {
        self.long_term_period
    }
    
    fn validate(&self) -> Result<()> {
        if self.long_term_period < 240 {
            return Err(anyhow::anyhow!("长期分析周期至少需要240天"));
        }
        Ok(())
    }
}

/// 长期底部反转策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermBottomReversalResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
    pub historical_high: f64,
    pub decline_from_high: f64,
    pub is_long_term_decline: bool,
    pub is_volume_shrinkage: bool,
    pub volume_ratio: f64,
    pub is_consolidation: bool,
    pub consolidation_days: usize,
    pub consolidation_range: f64,
    pub is_multiple_bottom_tests: bool,
    pub bottom_test_count: usize,
    pub bottom_price: f64,
    pub is_reversal_signal: bool,
    pub recent_bullish_ratio: f64,
    pub price_lift_from_bottom: f64,
    pub is_ma_bullish: bool,
}

/// 长期底部反转策略
pub struct LongTermBottomReversalStrategy {
    config: LongTermBottomReversalConfig,
}

impl LongTermBottomReversalStrategy {
    /// 创建新的长期底部反转策略实例
    pub fn new(config: LongTermBottomReversalConfig) -> Self {
        Self { config }
    }
    
    /// 创建保守配置的策略实例
    /// 
    /// 保守配置特点：
    /// - 要求更大的跌幅（60%）
    /// - 更明显的地量（30%）
    /// - 更长的横盘时间（45天）
    /// - 更多的探底次数（4次）
    pub fn conservative() -> Self {
        Self {
            config: LongTermBottomReversalConfig {
                min_decline_from_high: 60.0,
                volume_shrink_threshold: 0.3,
                min_consolidation_days: 45,
                min_bottom_tests: 4,
                ..Default::default()
            },
        }
    }
    
    /// 创建激进配置的策略实例
    /// 
    /// 激进配置特点：
    /// - 接受较小的跌幅（40%）
    /// - 地量要求较低（50%）
    /// - 较短的横盘时间（20天）
    /// - 较少的探底次数（2次）
    pub fn aggressive() -> Self {
        Self {
            config: LongTermBottomReversalConfig {
                min_decline_from_high: 40.0,
                volume_shrink_threshold: 0.5,
                min_consolidation_days: 20,
                min_bottom_tests: 2,
                ..Default::default()
            },
        }
    }
    
    /// 内部分析方法
    /// 
    /// 执行完整的底部反转分析流程：
    /// 1. 检查长期下跌
    /// 2. 检查地量特征
    /// 3. 检查横盘整理
    /// 4. 检查多次探底
    /// 5. 检查反转苗头
    /// 6. 生成策略信号
    /// 7. 生成分析描述
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) 
        -> Result<LongTermBottomReversalResult> {
        
        // 1. 检查长期下跌
        let (is_long_term_decline, historical_high, decline_from_high) = 
            self.check_long_term_decline(data)?;
        
        
        // 2. 检查地量特征
        let (is_volume_shrinkage, volume_ratio) = 
            self.check_volume_shrinkage(data)?;
        
        // 3. 检查横盘整理
        let (is_consolidation, consolidation_days, consolidation_range) = 
            self.check_consolidation(data)?;
        
        // 4. 检查多次探底
        let (is_multiple_bottom_tests, bottom_test_count, bottom_price) = 
            self.check_multiple_bottom_tests(data)?;
        
        // 5. 检查反转苗头
        let (is_reversal_signal, recent_bullish_ratio, price_lift_from_bottom, is_ma_bullish) = 
            self.check_reversal_signal(data, bottom_price)?;
        
        // 6. 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = 
            self.generate_signal(
                is_long_term_decline, is_volume_shrinkage, is_consolidation,
                is_multiple_bottom_tests, is_reversal_signal, decline_from_high,
                volume_ratio, consolidation_days, bottom_test_count, price_lift_from_bottom,
            );
        
        // 7. 生成分析描述
        let analysis_description = self.generate_description(
            is_long_term_decline, is_volume_shrinkage, is_consolidation,
            is_multiple_bottom_tests, is_reversal_signal, decline_from_high,
            consolidation_days, bottom_test_count,
        );
        
        let latest = data.last().unwrap();
        
        Ok(LongTermBottomReversalResult {
            stock_code: symbol.to_string(),
            analysis_date: NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?,
            current_price: latest.close,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            historical_high,
            decline_from_high,
            is_long_term_decline,
            is_volume_shrinkage,
            volume_ratio,
            is_consolidation,
            consolidation_days,
            consolidation_range,
            is_multiple_bottom_tests,
            bottom_test_count,
            bottom_price,
            is_reversal_signal,
            recent_bullish_ratio,
            price_lift_from_bottom,
            is_ma_bullish,
        })
    }
    
    /// 检查是否为长期下跌
    /// 
    /// 判断逻辑：
    /// - 在回溯期内找到历史最高价
    /// - 计算当前价格相对历史高点的跌幅
    /// - 跌幅超过配置阈值则确认长期下跌
    /// 
    /// # 返回值
    /// (是否长期下跌, 历史最高价, 跌幅百分比)
    fn check_long_term_decline(&self, data: &[SecurityData]) -> Result<(bool, f64, f64)> {
        // 获取回溯期数据
        let lookback_data = if data.len() > self.config.high_lookback_period {
            &data[data.len() - self.config.high_lookback_period..]
        } else {
            data
        };
        
        let historical_high = lookback_data.iter()
            .map(|d| d.high)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        
        let current_price = data.last().unwrap().close;
        let decline_from_high = ((historical_high - current_price) / historical_high) * 100.0;
        let is_long_term_decline = decline_from_high >= self.config.min_decline_from_high;
        
        debug!("长期下跌检查: 历史高点={:.2}, 跌幅={:.2}%", historical_high, decline_from_high);
        Ok((is_long_term_decline, historical_high, decline_from_high))
    }
    
    /// 检查成交量是否萎缩到地量
    /// 
    /// 判断逻辑：
    /// - 计算历史期的平均成交量
    /// - 计算近期（底部确认期）的平均成交量
    /// - 比较两者的比例，判断是否萎缩到阈值以下
    /// 
    /// # 返回值
    /// (是否地量, 成交量比例)
    fn check_volume_shrinkage(&self, data: &[SecurityData]) -> Result<(bool, f64)> {
        // 获取历史期数据（排除底部确认期）
        let historical_data = if data.len() > self.config.historical_volume_period {
            &data[data.len() - self.config.historical_volume_period..data.len() - self.config.bottom_confirm_period]
        } else {
            &data[..data.len().saturating_sub(self.config.bottom_confirm_period)]
        };
        
        if historical_data.is_empty() {
            return Ok((false, 1.0));
        }
        
        let historical_avg_volume = historical_data.iter().map(|d| d.volume).sum::<f64>() / historical_data.len() as f64;
        let recent_data = &data[data.len().saturating_sub(self.config.bottom_confirm_period)..];
        let recent_avg_volume = recent_data.iter().map(|d| d.volume).sum::<f64>() / recent_data.len() as f64;
        
        let volume_ratio = if historical_avg_volume > 0.0 { recent_avg_volume / historical_avg_volume } else { 1.0 };
        let is_volume_shrinkage = volume_ratio <= self.config.volume_shrink_threshold;
        
        Ok((is_volume_shrinkage, volume_ratio))
    }
    
    /// 检查是否横盘整理
    /// 
    /// 判断逻辑：
    /// - 在底部确认期内找到最高价和最低价
    /// - 计算价格波动范围
    /// - 波动范围小于阈值且持续时间足够长则确认横盘
    /// 
    /// # 返回值
    /// (是否横盘, 横盘天数, 价格波动范围百分比)
    fn check_consolidation(&self, data: &[SecurityData]) -> Result<(bool, usize, f64)> {
        // 获取底部确认期数据
        let consolidation_data = &data[data.len().saturating_sub(self.config.bottom_confirm_period)..];
        let min_price = consolidation_data.iter().map(|d| d.low).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
        let max_price = consolidation_data.iter().map(|d| d.high).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
        let consolidation_range = if min_price > 0.0 { ((max_price - min_price) / min_price) * 100.0 } else { 0.0 };
        let consolidation_days = consolidation_data.len();
        let is_consolidation = consolidation_range <= self.config.consolidation_range && consolidation_days >= self.config.min_consolidation_days;
        
        Ok((is_consolidation, consolidation_days, consolidation_range))
    }
    
    /// 检查是否多次探底
    /// 
    /// 判断逻辑：
    /// - 在底部确认期内找到最低价
    /// - 统计价格触及底部区域（容差范围内）的次数
    /// - 去重连续的触底（避免重复计数）
    /// - 触底次数达到阈值则确认多次探底
    /// 
    /// # 返回值
    /// (是否多次探底, 探底次数, 底部价格)
    fn check_multiple_bottom_tests(&self, data: &[SecurityData]) -> Result<(bool, usize, f64)> {
        // 获取底部确认期数据
        let test_data = &data[data.len().saturating_sub(self.config.bottom_confirm_period)..];
        let bottom_price = test_data.iter().map(|d| d.low).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
        let tolerance = bottom_price * self.config.bottom_tolerance / 100.0;
        let (lower_bound, upper_bound) = (bottom_price - tolerance, bottom_price + tolerance);
        
        let mut bottom_test_count = 0;
        let mut in_bottom_zone = false;
        for d in test_data {
            if d.low >= lower_bound && d.low <= upper_bound {
                if !in_bottom_zone {
                    bottom_test_count += 1;
                    in_bottom_zone = true;
                }
            } else {
                in_bottom_zone = false;
            }
        }
        
        let is_multiple_bottom_tests = bottom_test_count >= self.config.min_bottom_tests;
        Ok((is_multiple_bottom_tests, bottom_test_count, bottom_price))
    }
    
    /// 检查是否出现反转苗头
    /// 
    /// 判断逻辑：
    /// - 计算反转确认期内的阳线占比
    /// - 计算价格相对底部的抬升幅度
    /// - 检查均线是否多头排列（可选）
    /// - 检查是否温和放量
    /// - 所有条件满足则确认反转苗头
    /// 
    /// # 参数
    /// - `bottom_price`: 底部价格
    /// 
    /// # 返回值
    /// (是否反转, 阳线占比, 价格抬升幅度, 是否均线多头)
    fn check_reversal_signal(&self, data: &[SecurityData], bottom_price: f64) -> Result<(bool, f64, f64, bool)> {
        // 获取反转确认期数据
        let reversal_data = &data[data.len().saturating_sub(self.config.reversal_confirm_period)..];
        let bullish_count = reversal_data.iter().filter(|d| d.close > d.open).count();
        let recent_bullish_ratio = bullish_count as f64 / reversal_data.len() as f64;
        
        let current_price = data.last().unwrap().close;
        let price_lift_from_bottom = if bottom_price > 0.0 { ((current_price - bottom_price) / bottom_price) * 100.0 } else { 0.0 };
        
        let is_ma_bullish = if self.config.require_ma_bullish { self.check_ma_bullish_alignment(data)? } else { true };
        let is_moderate_volume = self.check_moderate_volume(reversal_data)?;
        
        let is_reversal_signal = recent_bullish_ratio >= self.config.bullish_ratio_threshold &&
            price_lift_from_bottom >= self.config.price_lift_threshold && is_ma_bullish && is_moderate_volume;
        
        Ok((is_reversal_signal, recent_bullish_ratio, price_lift_from_bottom, is_ma_bullish))
    }
    
    /// 检查均线是否多头排列
    /// 
    /// 判断 MA5 > MA10 > MA20
    fn check_ma_bullish_alignment(&self, data: &[SecurityData]) -> Result<bool> {
        if data.len() < 20 { return Ok(false); }
        let ma5 = self.calculate_ma(data, 5);
        let ma10 = self.calculate_ma(data, 10);
        let ma20 = self.calculate_ma(data, 20);
        Ok(ma5 > ma10 && ma10 > ma20)
    }
    
    /// 计算移动平均线
    /// 
    /// # 参数
    /// - `period`: 均线周期
    fn calculate_ma(&self, data: &[SecurityData], period: usize) -> f64 {
        let recent = &data[data.len().saturating_sub(period)..];
        if recent.is_empty() { return 0.0; }
        recent.iter().map(|d| d.close).sum::<f64>() / recent.len() as f64
    }
    
    /// 检查是否温和放量
    /// 
    /// 判断逻辑：
    /// - 将数据分为前后两段
    /// - 计算前段（地量期）和后段的平均成交量
    /// - 后段成交量有所增加但不是暴涨（1.3-2.0倍）
    fn check_moderate_volume(&self, data: &[SecurityData]) -> Result<bool> {
        if data.len() < 2 { return Ok(false); }
        let early_data = &data[..data.len() / 2];
        let early_avg_volume = early_data.iter().map(|d| d.volume).sum::<f64>() / early_data.len() as f64;
        let recent_data = &data[data.len() / 2..];
        let recent_avg_volume = recent_data.iter().map(|d| d.volume).sum::<f64>() / recent_data.len() as f64;
        let volume_increase_ratio = if early_avg_volume > 0.0 { recent_avg_volume / early_avg_volume } else { 1.0 };
        Ok(volume_increase_ratio >= self.config.moderate_volume_threshold && volume_increase_ratio <= 2.0)
    }
    
    /// 生成策略信号
    /// 
    /// 评分规则（总分100）：
    /// - 长期下跌确认：15分（跌幅>70%额外降低风险）
    /// - 地量特征：20分（萎缩<30%额外5分+降低风险）
    /// - 横盘整理：20分（横盘>60天额外5分）
    /// - 多次探底：20分（探底>=4次额外5分+降低风险）
    /// - 反转苗头：25分（抬升>10%额外5分+降低风险）
    /// 
    /// 信号等级：
    /// - >= 85分：强烈买入
    /// - >= 70分：买入
    /// - >= 50分：持有
    /// - < 50分：卖出
    /// 
    /// # 返回值
    /// (策略信号, 信号强度, 风险等级)
    #[allow(clippy::too_many_arguments)]
    fn generate_signal(&self, is_long_term_decline: bool, is_volume_shrinkage: bool, is_consolidation: bool,
        is_multiple_bottom_tests: bool, is_reversal_signal: bool, decline_from_high: f64, volume_ratio: f64,
        consolidation_days: usize, bottom_test_count: usize, price_lift_from_bottom: f64) -> (StrategySignal, u8, u8) {
        
        let mut signal_strength = 0u8;
        let mut risk_level = 5u8;
        
        if is_long_term_decline { signal_strength += 15; if decline_from_high > 70.0 { risk_level = risk_level.saturating_sub(1); } }
        if is_volume_shrinkage { signal_strength += 20; if volume_ratio < 0.3 { signal_strength += 5; risk_level = risk_level.saturating_sub(1); } }
        if is_consolidation { signal_strength += 20; if consolidation_days > 60 { signal_strength += 5; } }
        if is_multiple_bottom_tests { signal_strength += 20; if bottom_test_count >= 4 { signal_strength += 5; risk_level = risk_level.saturating_sub(1); } }
        if is_reversal_signal { signal_strength += 25; if price_lift_from_bottom > 10.0 { signal_strength += 5; risk_level = risk_level.saturating_sub(1); } }
        
        let strategy_signal = if signal_strength >= 85 { StrategySignal::StrongBuy }
            else if signal_strength >= 70 { StrategySignal::Buy }
            else if signal_strength >= 50 { StrategySignal::Hold }
            else { StrategySignal::Sell };
        
        (strategy_signal, signal_strength, risk_level.max(2))
    }
    
    /// 生成分析描述
    /// 
    /// 根据各项检查结果生成人类可读的分析说明
    fn generate_description(&self, is_long_term_decline: bool, is_volume_shrinkage: bool, is_consolidation: bool,
        is_multiple_bottom_tests: bool, is_reversal_signal: bool, decline_from_high: f64,
        consolidation_days: usize, bottom_test_count: usize) -> String {
        
        let mut desc = Vec::new();
        if is_long_term_decline { desc.push(format!("从高点回调{:.1}%", decline_from_high)); }
        if is_volume_shrinkage { desc.push("地量".to_string()); }
        if is_consolidation { desc.push(format!("横盘{}天", consolidation_days)); }
        if is_multiple_bottom_tests { desc.push(format!("{}次探底", bottom_test_count)); }
        if is_reversal_signal { desc.push("反转苗头".to_string()); }
        desc.join("；")
    }
}

impl Default for LongTermBottomReversalStrategy {
    fn default() -> Self {
        Self::new(LongTermBottomReversalConfig::default())
    }
}

impl TradingStrategy for LongTermBottomReversalStrategy {
    type Config = LongTermBottomReversalConfig;
    
    fn name(&self) -> &str { "长期底部反转策略" }
    fn description(&self) -> &str { "识别长期下跌后的底部反转" }
    fn config(&self) -> &Self::Config { &self.config }
    fn update_config(&mut self, config: Self::Config) -> Result<()> { 
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn analyze(&mut self, symbol: &str, data: &[SecurityData]) -> Result<StrategyResult> {
        let result = self.analyze_internal(symbol, data)?;
        Ok(StrategyResult::LongTermBottomReversal(result))
    }
}
