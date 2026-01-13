//! 换手率均线多头策略 (Turnover MA Bullish Strategy)
//! 
//! # 策略思想
//! 
//! **核心理念：活跃资金 + 趋势向上 = 强势股**
//! 
//! 这是一个结合成交活跃度和趋势强度的技术分析策略，通过两个维度筛选强势股票：
//! 
//! ## 1. 换手率 - 衡量成交活跃度
//! 
//! **换手率 = 成交量 / 流通股本 × 100%**
//! 
//! **为什么重要：**
//! - 反映资金关注度和参与度
//! - 高换手率说明有资金在积极博弈
//! - 适度换手率（3%-10%）最健康
//! - 过高换手率可能是炒作，风险大
//! 
//! **换手率分级：**
//! - < 1%：冷门股，无人问津
//! - 1% - 3%：正常水平
//! - 3% - 7%：活跃，有资金关注
//! - 7% - 15%：高度活跃，强势股特征
//! - > 15%：异常活跃，可能炒作
//! 
//! ## 2. 均线多头排列 - 确认趋势向上
//! 
//! **多头排列定义：**
//! - 短期均线 > 中期均线 > 长期均线
//! - 价格 > 所有均线
//! - 均线向上发散
//! 
//! **为什么重要：**
//! - 多头排列是上涨趋势的标志
//! - 均线系统提供支撑
//! - 趋势一旦形成，惯性较强
//! 
//! **常用均线组合：**
//! - 短期：MA5（周线）
//! - 中期：MA20（月线）
//! - 长期：MA60（季线）
//! 
//! ## 策略逻辑
//! 
//! 1. **换手率筛选**：剔除过冷和过热的股票
//! 2. **均线排列检查**：确认趋势向上
//! 3. **价格位置**：价格在均线之上
//! 4. **均线斜率**：均线向上倾斜
//! 
//! ## 适用场景
//! 
//! - **牛市**：效果最好，趋势明确
//! - **震荡市**：需要提高换手率要求
//! - **熊市**：不适用，假突破多
//! 
//! ## 风险提示
//! 
//! - 均线是滞后指标，可能错过最佳买点
//! - 高换手率不一定是好事，要结合基本面
//! - 注意止损，趋势反转时及时退出

use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

/// 换手率均线多头策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnoverMaBullishConfig {
    /// 最小换手率（百分比）
    pub min_turnover_rate: f64,
    
    /// 最大换手率（百分比，避免过度炒作）
    pub max_turnover_rate: f64,
    
    /// 短期均线周期
    pub short_ma_period: usize,
    
    /// 中期均线周期
    pub medium_ma_period: usize,
    
    /// 长期均线周期
    pub long_ma_period: usize,
    
    /// 是否要求价格在所有均线之上
    pub require_price_above_all_ma: bool,
    
    /// 均线向上最小斜率（可选，百分比）
    pub min_ma_slope: Option<f64>,
}

impl Default for TurnoverMaBullishConfig {
    fn default() -> Self {
        Self {
            min_turnover_rate: 5.0,   // 至少5%换手率
            max_turnover_rate: 15.0,  // 最多15%，避免过度炒作
            short_ma_period: 5,
            medium_ma_period: 20,
            long_ma_period: 60,
            require_price_above_all_ma: true,
            min_ma_slope: Some(0.5),  // 均线至少向上0.5%
        }
    }
}

impl StrategyConfig for TurnoverMaBullishConfig {
    fn strategy_name(&self) -> &str {
        "换手率均线多头策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.long_ma_period + 10  // 需要足够数据计算长期均线和斜率
    }
    
    fn validate(&self) -> Result<()> {
        if self.min_turnover_rate < 0.0 {
            bail!("min_turnover_rate 不能为负数");
        }
        
        if self.max_turnover_rate <= self.min_turnover_rate {
            bail!("max_turnover_rate 必须大于 min_turnover_rate");
        }
        
        if self.short_ma_period >= self.medium_ma_period {
            bail!("short_ma_period 必须小于 medium_ma_period");
        }
        
        if self.medium_ma_period >= self.long_ma_period {
            bail!("medium_ma_period 必须小于 long_ma_period");
        }
        
        Ok(())
    }
}

/// 换手率均线多头策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnoverMaBullishResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// 当前换手率（百分比）
    pub turnover_rate: f64,
    
    /// 平均换手率（最近N天）
    pub avg_turnover_rate: f64,
    
    /// 换手率评级
    pub turnover_rating: String,
    
    /// 短期均线值
    pub short_ma: f64,
    
    /// 中期均线值
    pub medium_ma: f64,
    
    /// 长期均线值
    pub long_ma: f64,
    
    /// 是否多头排列
    pub is_bullish_alignment: bool,
    
    /// 价格相对短期均线的位置（百分比）
    pub price_above_short_ma_pct: f64,
    
    /// 短期均线斜率（百分比）
    pub short_ma_slope: f64,
    
    /// 中期均线斜率（百分比）
    pub medium_ma_slope: f64,
    
    /// 趋势强度评分 (0-100)
    pub trend_strength: u8,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 换手率均线多头策略
pub struct TurnoverMaBullishStrategy {
    config: TurnoverMaBullishConfig,
}

impl TurnoverMaBullishStrategy {
    pub fn new(config: TurnoverMaBullishConfig) -> Self {
        Self { config }
    }
    
    /// 标准配置（3%-15%换手率，5/20/60均线）
    pub fn standard() -> TurnoverMaBullishConfig {
        TurnoverMaBullishConfig::default()
    }
    
    /// 活跃配置（5%-20%换手率，更高要求）
    pub fn active() -> TurnoverMaBullishConfig {
        TurnoverMaBullishConfig {
            min_turnover_rate: 5.0,
            max_turnover_rate: 20.0,
            short_ma_period: 5,
            medium_ma_period: 20,
            long_ma_period: 60,
            require_price_above_all_ma: true,
            min_ma_slope: Some(1.0),
        }
    }
    
    /// 稳健配置（2%-10%换手率，更长周期均线）
    pub fn conservative() -> TurnoverMaBullishConfig {
        TurnoverMaBullishConfig {
            min_turnover_rate: 2.0,
            max_turnover_rate: 10.0,
            short_ma_period: 10,
            medium_ma_period: 30,
            long_ma_period: 60,
            require_price_above_all_ma: true,
            min_ma_slope: Some(0.3),
        }
    }
    
    /// 短线配置（7%-25%换手率，短周期均线）
    pub fn short_term() -> TurnoverMaBullishConfig {
        TurnoverMaBullishConfig {
            min_turnover_rate: 7.0,
            max_turnover_rate: 25.0,
            short_ma_period: 3,
            medium_ma_period: 10,
            long_ma_period: 20,
            require_price_above_all_ma: true,
            min_ma_slope: Some(1.5),
        }
    }
}

impl TradingStrategy for TurnoverMaBullishStrategy {
    type Config = TurnoverMaBullishConfig;
    
    fn name(&self) -> &str {
        "换手率均线多头策略"
    }
    
    fn description(&self) -> &str {
        "结合换手率和均线多头排列的技术分析策略，筛选活跃且趋势向上的强势股"
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
        Ok(StrategyResult::TurnoverMaBullish(result))
    }
}

impl TurnoverMaBullishStrategy {
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<TurnoverMaBullishResult> {
        if data.len() < self.config.analysis_period() {
            bail!("数据不足，需要至少 {} 天数据，当前只有 {} 天", 
                  self.config.analysis_period(), data.len());
        }
        
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;
        
        // 获取换手率（从数据中直接读取）
        let turnover_rate = latest.turnover_rate
            .ok_or_else(|| anyhow::anyhow!("缺少换手率数据"))?;
        
        // 检查换手率范围
        if turnover_rate < self.config.min_turnover_rate {
            bail!("换手率 {:.2}% 低于最小要求 {:.2}%", 
                  turnover_rate, self.config.min_turnover_rate);
        }
        
        if turnover_rate > self.config.max_turnover_rate {
            bail!("换手率 {:.2}% 高于最大限制 {:.2}%（可能过度炒作）", 
                  turnover_rate, self.config.max_turnover_rate);
        }
        
        // 计算平均换手率
        let avg_turnover_rate = self.calculate_avg_turnover_rate(data, 10);
        
        // 计算均线
        let short_ma = self.calculate_ma(data, self.config.short_ma_period)?;
        let medium_ma = self.calculate_ma(data, self.config.medium_ma_period)?;
        let long_ma = self.calculate_ma(data, self.config.long_ma_period)?;
        
        // 检查多头排列
        let is_bullish_alignment = self.check_bullish_alignment(
            current_price, short_ma, medium_ma, long_ma
        );
        
        if !is_bullish_alignment {
            bail!("不满足多头排列条件");
        }
        
        // 检查价格位置
        if self.config.require_price_above_all_ma {
            if current_price <= short_ma || current_price <= medium_ma || current_price <= long_ma {
                bail!("价格未在所有均线之上");
            }
        }
        
        // 计算均线斜率
        let short_ma_slope = self.calculate_ma_slope(data, self.config.short_ma_period)?;
        let medium_ma_slope = self.calculate_ma_slope(data, self.config.medium_ma_period)?;
        
        // 检查均线斜率
        if let Some(min_slope) = self.config.min_ma_slope {
            if short_ma_slope < min_slope || medium_ma_slope < min_slope {
                bail!("均线斜率不足，短期MA斜率: {:.2}%, 中期MA斜率: {:.2}%", 
                      short_ma_slope, medium_ma_slope);
            }
        }
        
        // 计算价格相对短期均线的位置
        let price_above_short_ma_pct = (current_price - short_ma) / short_ma * 100.0;
        
        // 生成评级和得分
        let turnover_rating = self.rate_turnover(turnover_rate);
        let trend_strength = self.calculate_trend_strength(
            current_price,
            short_ma,
            medium_ma,
            long_ma,
            short_ma_slope,
            medium_ma_slope,
        );
        
        let (strategy_signal, signal_strength, risk_level) = self.generate_signal(
            turnover_rate,
            trend_strength,
            price_above_short_ma_pct,
        );
        
        let analysis_description = self.generate_description(
            turnover_rate,
            &turnover_rating,
            short_ma_slope,
            medium_ma_slope,
            price_above_short_ma_pct,
        );
        
        Ok(TurnoverMaBullishResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            turnover_rate,
            avg_turnover_rate,
            turnover_rating,
            short_ma,
            medium_ma,
            long_ma,
            is_bullish_alignment,
            price_above_short_ma_pct,
            short_ma_slope,
            medium_ma_slope,
            trend_strength,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 计算平均换手率
    fn calculate_avg_turnover_rate(&self, data: &[SecurityData], period: usize) -> f64 {
        let start = data.len().saturating_sub(period);
        let recent_data = &data[start..];
        
        let sum: f64 = recent_data.iter()
            .filter_map(|d| d.turnover_rate)
            .sum();
        
        let count = recent_data.iter()
            .filter(|d| d.turnover_rate.is_some())
            .count();
        
        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }
    
    /// 计算移动平均线
    fn calculate_ma(&self, data: &[SecurityData], period: usize) -> Result<f64> {
        if data.len() < period {
            bail!("数据不足以计算 {} 日均线", period);
        }
        
        let start = data.len() - period;
        let sum: f64 = data[start..].iter().map(|d| d.close).sum();
        Ok(sum / period as f64)
    }
    
    /// 检查多头排列
    fn check_bullish_alignment(
        &self,
        price: f64,
        short_ma: f64,
        medium_ma: f64,
        long_ma: f64,
    ) -> bool {
        // 多头排列：价格 > 短期MA > 中期MA > 长期MA
        price > short_ma && short_ma > medium_ma && medium_ma > long_ma
    }
    
    /// 计算均线斜率（百分比）
    fn calculate_ma_slope(&self, data: &[SecurityData], period: usize) -> Result<f64> {
        if data.len() < period + 5 {
            bail!("数据不足以计算均线斜率");
        }
        
        // 计算当前均线值
        let current_ma = self.calculate_ma(data, period)?;
        
        // 计算5天前的均线值
        let prev_data = &data[..data.len() - 5];
        let prev_ma = self.calculate_ma(prev_data, period)?;
        
        // 计算斜率（5天涨幅的百分比）
        Ok((current_ma - prev_ma) / prev_ma * 100.0)
    }
    
    /// 换手率评级
    fn rate_turnover(&self, turnover_rate: f64) -> String {
        if turnover_rate >= 15.0 {
            "极度活跃".to_string()
        } else if turnover_rate >= 10.0 {
            "高度活跃".to_string()
        } else if turnover_rate >= 7.0 {
            "活跃".to_string()
        } else if turnover_rate >= 3.0 {
            "正常活跃".to_string()
        } else {
            "低活跃".to_string()
        }
    }
    
    /// 计算趋势强度
    fn calculate_trend_strength(
        &self,
        price: f64,
        short_ma: f64,
        medium_ma: f64,
        long_ma: f64,
        short_ma_slope: f64,
        medium_ma_slope: f64,
    ) -> u8 {
        let mut score = 0u8;
        
        // 多头排列得分（40分）
        if price > short_ma && short_ma > medium_ma && medium_ma > long_ma {
            score += 40;
        }
        
        // 价格位置得分（30分）
        let price_above_short_pct = (price - short_ma) / short_ma * 100.0;
        if price_above_short_pct >= 5.0 {
            score += 30;
        } else if price_above_short_pct >= 3.0 {
            score += 25;
        } else if price_above_short_pct >= 1.0 {
            score += 20;
        } else if price_above_short_pct > 0.0 {
            score += 15;
        }
        
        // 均线斜率得分（30分）
        if short_ma_slope >= 2.0 && medium_ma_slope >= 1.0 {
            score += 30;
        } else if short_ma_slope >= 1.0 && medium_ma_slope >= 0.5 {
            score += 25;
        } else if short_ma_slope >= 0.5 && medium_ma_slope >= 0.0 {
            score += 20;
        } else if short_ma_slope > 0.0 {
            score += 15;
        }
        
        score.min(100)
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        turnover_rate: f64,
        trend_strength: u8,
        price_above_short_ma_pct: f64,
    ) -> (StrategySignal, u8, u8) {
        // 综合评分
        let mut signal_strength = trend_strength;
        
        // 换手率加成
        if turnover_rate >= 7.0 && turnover_rate <= 15.0 {
            signal_strength = signal_strength.saturating_add(10);
        }
        
        signal_strength = signal_strength.min(100);
        
        // 根据综合得分确定信号
        let strategy_signal = if signal_strength >= 85 && price_above_short_ma_pct < 5.0 {
            StrategySignal::StrongBuy  // 强势且未大幅偏离均线
        } else if signal_strength >= 70 {
            StrategySignal::Buy
        } else {
            StrategySignal::Hold
        };
        
        // 风险等级
        let risk_level = if turnover_rate > 15.0 {
            4  // 换手率过高，风险较大
        } else if trend_strength >= 80 && turnover_rate >= 5.0 {
            2  // 趋势强且换手率适中，风险较低
        } else if trend_strength >= 60 {
            3  // 中等风险
        } else {
            4  // 较高风险
        };
        
        (strategy_signal, signal_strength, risk_level)
    }
    
    /// 生成分析说明
    fn generate_description(
        &self,
        turnover_rate: f64,
        turnover_rating: &str,
        short_ma_slope: f64,
        medium_ma_slope: f64,
        price_above_short_ma_pct: f64,
    ) -> String {
        format!(
            "换手率 {:.2}%（{}），短期MA斜率 {:.2}%，中期MA斜率 {:.2}%，价格高于短期MA {:.2}%。{}",
            turnover_rate,
            turnover_rating,
            short_ma_slope,
            medium_ma_slope,
            price_above_short_ma_pct,
            if turnover_rate >= 7.0 && short_ma_slope >= 1.0 {
                "资金活跃且趋势强劲，强势股特征明显"
            } else if turnover_rate >= 3.0 && short_ma_slope >= 0.5 {
                "成交活跃，趋势向上，值得关注"
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
    
    fn create_bullish_ma_data() -> Vec<SecurityData> {
        let mut data = Vec::new();
        let base_price = 10.0;
        
        // 创建70天数据，模拟上涨趋势
        for i in 0..70 {
            let price = base_price + (i as f64 * 0.1);  // 逐步上涨
            let volume = 5_000_000.0 + (i as f64 * 50_000.0);  // 成交量逐步放大
            
            data.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("202411{:02}", i % 30 + 1),
                open: price - 0.1,
                close: price,
                high: price + 0.2,
                low: price - 0.2,
                pre_close: Some(price - 0.1),
                change: Some(0.1),
                pct_change: Some(1.0),
                volume,
                amount: volume * price,
                turnover_rate: Some(5.0 + (i as f64 * 0.05)),  // 模拟换手率逐步增加
                time_frame: TimeFrame::Daily,
                security_type: SecurityType::Stock,
                financial_data: None,
                target: None,
            });
        }
        
        data
    }
    
    #[test]
    fn test_turnover_ma_bullish_strategy() {
        let config = TurnoverMaBullishStrategy::standard();
        let mut strategy = TurnoverMaBullishStrategy::new(config);
        
        let data = create_bullish_ma_data();
        let result = strategy.analyze("000001.SZ", &data);
        
        assert!(result.is_ok());
        
        if let Ok(StrategyResult::TurnoverMaBullish(r)) = result {
            assert!(r.is_bullish_alignment);
            assert!(r.short_ma < r.current_price);
            assert!(r.medium_ma < r.short_ma);
            assert!(r.long_ma < r.medium_ma);
            assert!(r.trend_strength > 0);
        } else {
            panic!("Expected TurnoverMaBullish result");
        }
    }
}
