//! 涨停回调策略 (Limit Up Pullback Strategy)
//! 
//! ## 策略思想
//! 
//! ### 核心理念
//! 
//! **"强者恒强，回调即是机会"**
//! 
//! 涨停代表了市场最强的买盘力量，但涨停后往往会有短期回调。
//! 当股价回调到关键支撑位（如5日线、10日线）时，是二次介入的良机。
//! 
//! ### 策略逻辑
//! 
//! 1. **涨停确认** - 近N天内出现过涨停，证明有强势资金关注
//! 2. **回调到位** - 股价回调到5日或10日均线附近，提供安全边际
//! 3. **支撑有效** - 均线支撑未破，趋势仍然向上
//! 4. **量能配合** - 回调时缩量，反弹时放量
//! 
//! ### 技术原理
//! 
//! ```text
//! 价格
//!   ↑
//!   |    ★ 涨停板
//!   |   ╱ ╲
//!   |  ╱   ╲ ← 回调
//!   | ╱     ●━━━ 买入点（回调到均线）
//!   |╱    ╱ ← MA5/MA10
//!   |────╱────────→ 时间
//! ```
//! 
//! ### 为什么有效？
//! 
//! 1. **强势股特征**
//!    - 涨停说明有主力资金介入
//!    - 短期回调是正常的获利回吐
//!    - 强势股回调幅度有限
//! 
//! 2. **均线支撑**
//!    - 5日线：短期支撑，适合激进交易
//!    - 10日线：中期支撑，适合稳健交易
//!    - 均线是市场平均成本，有天然支撑作用
//! 
//! 3. **风险收益比**
//!    - 回调买入比追涨停更安全
//!    - 止损位明确（均线下方）
//!    - 盈亏比高（小止损，大空间）
//! 
//! ### 适用场景
//! 
//! ✅ **适合**：
//! - 牛市或震荡市
//! - 题材股、概念股
//! - 有明确催化剂的股票
//! 
//! ⚠️ **不适合**：
//! - 熊市（均线支撑失效）
//! - 一字涨停（无法回调）
//! - 基本面恶化的股票

use super::traits::*;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// 涨停回调策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitUpPullbackConfig {
    /// 回溯天数 - 在多少天内寻找涨停 - 默认 10 天
    pub lookback_days: usize,
    
    /// 涨停阈值（百分比）- 默认 9.8%（考虑误差）
    pub limit_up_threshold: f64,
    
    /// 使用的均线类型 - "MA5", "MA10", "BOTH"
    pub ma_type: String,
    
    /// 回调到均线的容差（百分比）- 默认 3%
    /// 允许价格在均线上下3%范围内
    pub pullback_tolerance: f64,
    
    /// 最小涨停次数 - 默认 1 次
    pub min_limit_up_count: usize,
    
    /// 是否要求缩量回调 - 默认 true
    pub require_volume_shrink: bool,
    
    /// 缩量比例阈值 - 默认 0.7（当前成交量 < 均量 * 0.7）
    pub volume_shrink_ratio: f64,
    
    /// 是否要求均线多头排列 - 默认 false
    pub require_ma_bullish: bool,
}

impl Default for LimitUpPullbackConfig {
    fn default() -> Self {
        Self {
            lookback_days: 10,
            limit_up_threshold: 9.8,
            ma_type: "BOTH".to_string(),
            pullback_tolerance: 3.0,
            min_limit_up_count: 1,
            require_volume_shrink: true,
            volume_shrink_ratio: 0.7,
            require_ma_bullish: false,
        }
    }
}

impl StrategyConfig for LimitUpPullbackConfig {
    fn strategy_name(&self) -> &str {
        "limit_up_pullback"
    }
    
    fn analysis_period(&self) -> usize {
        // 需要足够的数据来计算均线和寻找涨停
        self.lookback_days + 20
    }
    
    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("回溯天数不能为0");
        }
        if self.limit_up_threshold <= 0.0 {
            bail!("涨停阈值必须大于0");
        }
        if !["MA5", "MA10", "BOTH"].contains(&self.ma_type.as_str()) {
            bail!("均线类型必须是 MA5, MA10 或 BOTH");
        }
        if self.pullback_tolerance < 0.0 {
            bail!("回调容差不能为负数");
        }
        if self.min_limit_up_count == 0 {
            bail!("最小涨停次数不能为0");
        }
        if self.volume_shrink_ratio <= 0.0 || self.volume_shrink_ratio > 1.0 {
            bail!("缩量比例必须在 (0, 1] 范围内");
        }
        Ok(())
    }
}

/// 涨停回调策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitUpPullbackResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 涨停日期列表
    pub limit_up_dates: Vec<String>,
    
    /// 涨停次数
    pub limit_up_count: usize,
    
    /// 最近一次涨停日期
    pub last_limit_up_date: String,
    
    /// 最近一次涨停价格
    pub last_limit_up_price: f64,
    
    /// 涨停后最高价
    pub highest_after_limit_up: f64,
    
    /// 从涨停最高点回调幅度（百分比）
    pub pullback_from_high_pct: f64,
    
    /// 5日均线
    pub ma5: f64,
    
    /// 10日均线
    pub ma10: f64,
    
    /// 20日均线
    pub ma20: f64,
    
    /// 距离5日线的距离（百分比）
    pub distance_to_ma5_pct: f64,
    
    /// 距离10日线的距离（百分比）
    pub distance_to_ma10_pct: f64,
    
    /// 是否在5日线附近
    pub near_ma5: bool,
    
    /// 是否在10日线附近
    pub near_ma10: bool,
    
    /// 是否均线多头排列
    pub ma_bullish_aligned: bool,
    
    /// 当前成交量
    pub current_volume: f64,
    
    /// 5日平均成交量
    pub avg_volume_5d: f64,
    
    /// 是否缩量
    pub is_volume_shrink: bool,
    
    /// 量比
    pub volume_ratio: f64,
    
    /// 近 lookback_days 天的每日涨跌幅（百分比）
    /// 按时间顺序排列，最后一个是最近的
    pub daily_pct_changes: Vec<f64>,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 涨停回调策略
pub struct LimitUpPullbackStrategy {
    config: LimitUpPullbackConfig,
}

impl LimitUpPullbackStrategy {
    pub fn new(config: LimitUpPullbackConfig) -> Self {
        Self { config }
    }
    
    /// 标准配置（回调到5日或10日线）
    pub fn standard() -> LimitUpPullbackConfig {
        LimitUpPullbackConfig::default()
    }
    
    /// 激进配置（只看5日线，容差更大）
    pub fn aggressive() -> LimitUpPullbackConfig {
        LimitUpPullbackConfig {
            lookback_days: 5,
            limit_up_threshold: 9.5,
            ma_type: "MA5".to_string(),
            pullback_tolerance: 5.0,
            min_limit_up_count: 1,
            require_volume_shrink: false,
            volume_shrink_ratio: 0.8,
            require_ma_bullish: false,
        }
    }
    
    /// 稳健配置（只看10日线，要求缩量和多头排列）
    pub fn conservative() -> LimitUpPullbackConfig {
        LimitUpPullbackConfig {
            lookback_days: 15,
            limit_up_threshold: 9.9,
            ma_type: "MA10".to_string(),
            pullback_tolerance: 2.0,
            min_limit_up_count: 1,
            require_volume_shrink: true,
            volume_shrink_ratio: 0.6,
            require_ma_bullish: true,
        }
    }
    
    /// 强势股配置（多次涨停，回调幅度小）
    pub fn strong_stock() -> LimitUpPullbackConfig {
        LimitUpPullbackConfig {
            lookback_days: 10,
            limit_up_threshold: 9.8,
            ma_type: "MA5".to_string(),
            pullback_tolerance: 2.0,
            min_limit_up_count: 2,  // 至少2次涨停
            require_volume_shrink: true,
            volume_shrink_ratio: 0.7,
            require_ma_bullish: false,
        }
    }
    
    /// 计算简单移动平均线
    fn calculate_ma(&self, data: &[SecurityData], period: usize) -> f64 {
        if data.len() < period {
            return 0.0;
        }
        
        let start_idx = data.len() - period;
        let sum: f64 = data[start_idx..].iter().map(|d| d.close).sum();
        sum / period as f64
    }
    
    /// 判断是否涨停
    fn is_limit_up(&self, data: &SecurityData) -> bool {
        if let Some(pct_change) = data.pct_change {
            pct_change >= self.config.limit_up_threshold
        } else {
            false
        }
    }
    
    /// 查找涨停日期
    fn find_limit_up_dates(&self, data: &[SecurityData]) -> Vec<(String, f64)> {
        let lookback_start = if data.len() > self.config.lookback_days {
            data.len() - self.config.lookback_days
        } else {
            0
        };
        
        let mut limit_up_dates = Vec::new();
        
        for i in lookback_start..data.len() {
            if self.is_limit_up(&data[i]) {
                limit_up_dates.push((data[i].trade_date.clone(), data[i].close));
            }
        }
        
        limit_up_dates
    }
    
    /// 计算涨停后的最高价
    fn calculate_highest_after_limit_up(&self, data: &[SecurityData], limit_up_idx: usize) -> f64 {
        if limit_up_idx >= data.len() - 1 {
            return data[limit_up_idx].high;
        }
        
        data[limit_up_idx..].iter()
            .map(|d| d.high)
            .fold(f64::NEG_INFINITY, f64::max)
    }
    
    /// 检查是否在均线附近
    fn is_near_ma(&self, price: f64, ma: f64) -> bool {
        let distance_pct = ((price - ma) / ma * 100.0).abs();
        distance_pct <= self.config.pullback_tolerance
    }
    
    /// 计算平均成交量
    fn calculate_avg_volume(&self, data: &[SecurityData], period: usize) -> f64 {
        if data.len() < period {
            return 0.0;
        }
        
        let start_idx = data.len() - period;
        let sum: f64 = data[start_idx..].iter().map(|d| d.volume).sum();
        sum / period as f64
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<LimitUpPullbackResult> {
        let min_required = self.config.analysis_period();
        if data.len() < min_required {
            bail!("数据不足，需要至少{}天数据，当前只有{}天", min_required, data.len());
        }
        
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let current_volume = latest.volume;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 查找涨停日期
        let limit_up_dates_with_price = self.find_limit_up_dates(data);
        let limit_up_count = limit_up_dates_with_price.len();
        
        if limit_up_count < self.config.min_limit_up_count {
            bail!("涨停次数不足，需要至少{}次，实际{}次", 
                  self.config.min_limit_up_count, limit_up_count);
        }
        
        let limit_up_dates: Vec<String> = limit_up_dates_with_price.iter()
            .map(|(date, _)| date.clone())
            .collect();
        
        let (last_limit_up_date, last_limit_up_price) = limit_up_dates_with_price.last()
            .map(|(date, price)| (date.clone(), *price))
            .unwrap();
        
        // 找到最后一次涨停的索引
        let last_limit_up_idx = data.iter()
            .rposition(|d| d.trade_date == last_limit_up_date)
            .unwrap();
        
        // 计算涨停后最高价
        let highest_after_limit_up = self.calculate_highest_after_limit_up(data, last_limit_up_idx);
        
        // 计算回调幅度
        let pullback_from_high_pct = if highest_after_limit_up > 0.0 {
            ((highest_after_limit_up - current_price) / highest_after_limit_up) * 100.0
        } else {
            0.0
        };
        
        // 计算均线
        let ma5 = self.calculate_ma(data, 5);
        let ma10 = self.calculate_ma(data, 10);
        let ma20 = self.calculate_ma(data, 20);
        
        // 计算距离均线的距离
        let distance_to_ma5_pct = if ma5 > 0.0 {
            ((current_price - ma5) / ma5) * 100.0
        } else {
            0.0
        };
        
        let distance_to_ma10_pct = if ma10 > 0.0 {
            ((current_price - ma10) / ma10) * 100.0
        } else {
            0.0
        };
        
        // 判断是否在均线附近
        let near_ma5 = self.is_near_ma(current_price, ma5);
        let near_ma10 = self.is_near_ma(current_price, ma10);
        
        // 判断均线多头排列
        let ma_bullish_aligned = ma5 > ma10 && ma10 > ma20;
        
        // 计算成交量指标
        let avg_volume_5d = self.calculate_avg_volume(data, 5);
        let volume_ratio = if avg_volume_5d > 0.0 {
            current_volume / avg_volume_5d
        } else {
            1.0
        };
        let is_volume_shrink = volume_ratio < self.config.volume_shrink_ratio;
        
        // 收集近 lookback_days 天的每日涨跌幅
        let lookback_start = if data.len() > self.config.lookback_days {
            data.len() - self.config.lookback_days
        } else {
            0
        };
        
        let daily_pct_changes: Vec<f64> = data[lookback_start..].iter()
            .filter_map(|d| d.pct_change)
            .collect();
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            limit_up_count,
            near_ma5,
            near_ma10,
            distance_to_ma5_pct,
            distance_to_ma10_pct,
            pullback_from_high_pct,
            ma_bullish_aligned,
            is_volume_shrink,
            volume_ratio,
        );
        
        let analysis_description = self.generate_description(
            limit_up_count,
            &last_limit_up_date,
            pullback_from_high_pct,
            near_ma5,
            near_ma10,
            distance_to_ma5_pct,
            distance_to_ma10_pct,
            is_volume_shrink,
        );
        
        Ok(LimitUpPullbackResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            limit_up_dates,
            limit_up_count,
            last_limit_up_date,
            last_limit_up_price,
            highest_after_limit_up,
            pullback_from_high_pct,
            ma5,
            ma10,
            ma20,
            distance_to_ma5_pct,
            distance_to_ma10_pct,
            near_ma5,
            near_ma10,
            ma_bullish_aligned,
            current_volume,
            avg_volume_5d,
            is_volume_shrink,
            volume_ratio,
            daily_pct_changes,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        limit_up_count: usize,
        near_ma5: bool,
        near_ma10: bool,
        distance_to_ma5_pct: f64,
        distance_to_ma10_pct: f64,
        pullback_from_high_pct: f64,
        ma_bullish_aligned: bool,
        is_volume_shrink: bool,
        volume_ratio: f64,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 1. 涨停次数评分（25分）
        let limit_up_score = if limit_up_count >= 3 {
            25
        } else if limit_up_count >= 2 {
            20
        } else {
            15
        };
        signal_strength += limit_up_score;
        
        // 2. 均线位置评分（30分）
        let ma_score = match self.config.ma_type.as_str() {
            "MA5" => {
                if near_ma5 {
                    30
                } else if distance_to_ma5_pct.abs() < 5.0 {
                    20
                } else {
                    0
                }
            },
            "MA10" => {
                if near_ma10 {
                    30
                } else if distance_to_ma10_pct.abs() < 5.0 {
                    20
                } else {
                    0
                }
            },
            "BOTH" => {
                if near_ma5 || near_ma10 {
                    30
                } else if distance_to_ma5_pct.abs() < 5.0 || distance_to_ma10_pct.abs() < 5.0 {
                    20
                } else {
                    0
                }
            },
            _ => 0,
        };
        signal_strength += ma_score;
        
        // 3. 回调幅度评分（20分）
        let pullback_score = if pullback_from_high_pct > 15.0 {
            5  // 回调过深
        } else if pullback_from_high_pct > 10.0 {
            10
        } else if pullback_from_high_pct > 5.0 {
            20  // 理想回调
        } else if pullback_from_high_pct > 2.0 {
            15
        } else {
            10  // 回调不足
        };
        signal_strength += pullback_score;
        
        // 4. 均线多头排列（15分）
        if ma_bullish_aligned {
            signal_strength += 15;
            risk_level = risk_level.saturating_sub(1);
        }
        
        // 5. 缩量回调（10分）
        if is_volume_shrink {
            signal_strength += 10;
        } else if volume_ratio > 1.5 {
            // 放量回调，风险增加
            risk_level = risk_level.saturating_add(1);
        }
        
        // 根据距离均线的位置调整风险
        if distance_to_ma5_pct < -5.0 || distance_to_ma10_pct < -5.0 {
            // 跌破均线较多
            risk_level = risk_level.saturating_add(1);
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
        limit_up_count: usize,
        last_limit_up_date: &str,
        pullback_from_high_pct: f64,
        near_ma5: bool,
        near_ma10: bool,
        distance_to_ma5_pct: f64,
        distance_to_ma10_pct: f64,
        is_volume_shrink: bool,
    ) -> String {
        let mut desc = format!("近{}天{}次涨停，最近涨停日期{}。", 
                              self.config.lookback_days, limit_up_count, last_limit_up_date);
        
        desc.push_str(&format!("从高点回调{:.2}%。", pullback_from_high_pct));
        
        if near_ma5 && near_ma10 {
            desc.push_str("当前在5日线和10日线附近。");
        } else if near_ma5 {
            desc.push_str(&format!("当前在5日线附近（距离{:.2}%）。", distance_to_ma5_pct));
        } else if near_ma10 {
            desc.push_str(&format!("当前在10日线附近（距离{:.2}%）。", distance_to_ma10_pct));
        } else {
            desc.push_str(&format!("距5日线{:.2}%，距10日线{:.2}%。", 
                                  distance_to_ma5_pct, distance_to_ma10_pct));
        }
        
        if is_volume_shrink {
            desc.push_str("缩量回调，符合预期。");
        } else {
            desc.push_str("成交量未明显缩减。");
        }
        
        desc
    }
}

impl TradingStrategy for LimitUpPullbackStrategy {
    type Config = LimitUpPullbackConfig;
    
    fn name(&self) -> &str {
        "涨停回调策略"
    }
    
    fn description(&self) -> &str {
        "寻找近期有涨停且回调到均线附近的强势股"
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
        info!("开始分析股票 {} 的涨停回调信号", symbol);
        
        let result = self.analyze_internal(symbol, data)?;
        
        debug!(
            "股票 {} 分析完成：涨停{}次，信号强度={}",
            symbol, result.limit_up_count, result.signal_strength
        );
        
        Ok(StrategyResult::LimitUpPullback(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_data_with_limit_up() -> Vec<SecurityData> {
        let mut data = Vec::new();
        let base_date = 20240101;
        let mut price = 10.0;
        
        // 前20天正常交易
        for i in 0..20 {
            data.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("{}", base_date + i),
                open: price,
                close: price + 0.1,
                high: price + 0.2,
                low: price - 0.1,
                pre_close: Some(price),
                change: Some(0.1),
                volume: 1000000.0,
                amount: 10000000.0,
                pct_change: Some(1.0),
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
            });
            price += 0.1;
        }
        
        // 第21天涨停
        data.push(SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: format!("{}", base_date + 20),
            open: price,
            close: price * 1.1,
            high: price * 1.1,
            low: price,
            pre_close: Some(price),
            change: Some(price * 0.1),
            volume: 2000000.0,
            amount: 22000000.0,
            pct_change: Some(10.0),
            time_frame: TimeFrame::Daily,
            security_type: SecurityType::Stock,
            financial_data: None,
        });
        price = price * 1.1;
        
        // 涨停后回调到均线附近
        for i in 21..25 {
            price -= 0.2;
            data.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("{}", base_date + i),
                open: price + 0.1,
                close: price,
                high: price + 0.2,
                low: price - 0.1,
                pre_close: Some(price + 0.2),
                change: Some(-0.2),
                volume: 500000.0,  // 缩量
                amount: 5000000.0,
                pct_change: Some(-1.5),
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
            });
        }
        
        data
    }
    
    #[test]
    fn test_limit_up_pullback_strategy() {
        let config = LimitUpPullbackStrategy::standard();
        let mut strategy = LimitUpPullbackStrategy::new(config);
        
        let data = create_test_data_with_limit_up();
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::LimitUpPullback(r) = result {
            assert!(r.limit_up_count >= 1, "应该找到至少1次涨停");
            assert!(r.signal_strength > 0, "信号强度应该大于0");
            assert!(r.ma5 > 0.0, "MA5应该大于0");
        } else {
            panic!("Expected LimitUpPullback result");
        }
    }
}
