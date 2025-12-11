use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{
    SecurityData, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    TradingStrategy,
};

/// 价格强弱策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PriceStrengthConfig {
    /// 分析周期（天数）- 默认 20 天
    pub analysis_period: usize,
    
    /// 最小平均强度分数（0-100）- 默认 60
    pub min_avg_strength: f64,
    
    /// 最小强势天数占比（0-1）- 默认 0.6（60%的天数为强势）
    pub min_strong_days_ratio: f64,
    
    /// 要求最近N天连续强势 - 默认 3 天
    pub require_recent_strong_days: usize,
}

impl Default for PriceStrengthConfig {
    fn default() -> Self {
        Self {
            analysis_period: 20,
            min_avg_strength: 60.0,
            min_strong_days_ratio: 0.6,
            require_recent_strong_days: 3,
        }
    }
}

impl StrategyConfigTrait for PriceStrengthConfig {
    fn strategy_name(&self) -> &str {
        "价格强弱策略"
    }

    fn validate(&self) -> Result<()> {
        if self.analysis_period < 5 {
            bail!("分析周期至少需要5天");
        }
        if self.min_avg_strength < 0.0 || self.min_avg_strength > 100.0 {
            bail!("最小平均强度必须在0-100之间");
        }
        if self.min_strong_days_ratio < 0.0 || self.min_strong_days_ratio > 1.0 {
            bail!("强势天数占比必须在0-1之间");
        }
        if self.require_recent_strong_days > self.analysis_period {
            bail!("要求的连续强势天数不能超过分析周期");
        }
        Ok(())
    }

    fn analysis_period(&self) -> usize {
        self.analysis_period
    }
}

/// 价格强弱策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceStrengthResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 股票名称（可选）
    pub stock_name: Option<String>,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 当日涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// 平均强度分数（0-100）
    pub avg_strength_score: f64,
    
    /// 强势天数
    pub strong_days_count: usize,
    
    /// 强势天数占比
    pub strong_days_ratio: f64,
    
    /// 最近N天连续强势天数
    pub recent_strong_days: usize,
    
    /// 每日强度分数列表（最近N天）
    pub daily_strength_scores: Vec<f64>,
    
    /// 强度趋势（上升/下降/平稳）
    pub strength_trend: String,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}

/// 价格强弱策略
pub struct PriceStrengthStrategy {
    config: PriceStrengthConfig,
}

impl PriceStrengthStrategy {
    /// 创建新的价格强弱策略实例
    pub fn new(config: PriceStrengthConfig) -> Self {
        Self { config }
    }
    
    /// 创建保守配置的策略实例
    pub fn conservative() -> Self {
        Self {
            config: PriceStrengthConfig {
                min_avg_strength: 70.0,
                min_strong_days_ratio: 0.7,
                require_recent_strong_days: 5,
                ..Default::default()
            },
        }
    }
    
    /// 创建激进配置的策略实例
    pub fn aggressive() -> Self {
        Self {
            config: PriceStrengthConfig {
                min_avg_strength: 50.0,
                min_strong_days_ratio: 0.5,
                require_recent_strong_days: 2,
                ..Default::default()
            },
        }
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<PriceStrengthResult> {
        if data.len() < self.config.analysis_period {
            bail!("数据不足，需要至少{}天数据", self.config.analysis_period);
        }
        
        // 获取分析窗口数据
        let analysis_data = &data[data.len() - self.config.analysis_period..];
        let latest = data.last().unwrap();
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 计算每日强度分数
        let daily_strength_scores: Vec<f64> = analysis_data
            .iter()
            .map(|d| self.calculate_daily_strength(d))
            .collect();
        
        // 计算平均强度
        let avg_strength_score = daily_strength_scores.iter().sum::<f64>() / daily_strength_scores.len() as f64;
        
        // 统计强势天数（强度 >= 60）
        let strong_days_count = daily_strength_scores.iter().filter(|&&s| s >= 60.0).count();
        let strong_days_ratio = strong_days_count as f64 / daily_strength_scores.len() as f64;
        
        // 计算最近连续强势天数
        let recent_strong_days = self.count_recent_strong_days(&daily_strength_scores);
        
        // 分析强度趋势
        let strength_trend = self.analyze_strength_trend(&daily_strength_scores);
        
        // 判断是否满足条件
        let meets_criteria = avg_strength_score >= self.config.min_avg_strength
            && strong_days_ratio >= self.config.min_strong_days_ratio
            && recent_strong_days >= self.config.require_recent_strong_days;
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = if meets_criteria {
            self.generate_signal(avg_strength_score, strong_days_ratio, recent_strong_days, &strength_trend)
        } else {
            (StrategySignal::Sell, 0, 3)
        };
        
        let analysis_description = self.generate_description(
            avg_strength_score,
            strong_days_ratio,
            recent_strong_days,
            &strength_trend,
            meets_criteria,
        );
        
        Ok(PriceStrengthResult {
            stock_code: symbol.to_string(),
            stock_name: None,
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            avg_strength_score,
            strong_days_count,
            strong_days_ratio,
            recent_strong_days,
            daily_strength_scores,
            strength_trend,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
    
    /// 计算单日强度评分（0-100分）
    /// 
    /// # 评分逻辑依据
    /// 
    /// 该方法通过分析K线形态的多个维度来量化当日的多空力量对比，总分100分。
    /// 评分越高表示多方力量越强，股价上涨动能越足。
    /// 
    /// ## 评分维度（总分110分，最终限制在0-100）
    /// 
    /// ### 1. 实体强度（30分）- 反映多空博弈的激烈程度
    /// - **计算方式**: `实体长度 / 全天振幅 × 30`
    /// - **理论依据**: 实体越大说明开盘到收盘的价格变化越显著，表示当日主导方（多方或空方）控盘力度强
    /// - **实例**:
    ///   - 实体占振幅100%（一字涨停）→ 30分
    ///   - 实体占振幅50% → 15分
    ///   - 实体占振幅10%（十字星）→ 3分
    /// 
    /// ### 2. 上影线评分（20分）- 反映上方抛压
    /// - **计算方式**: `(1 - 上影线长度/振幅) × 20`
    /// - **理论依据**: 上影线短说明股价冲高后没有明显回落，表示上方抛压轻，多方控盘强
    /// - **实例**:
    ///   - 无上影线 → 20分（最强）
    ///   - 上影线占50%振幅 → 10分
    ///   - 上影线占100%振幅（T字板）→ 0分
    /// 
    /// ### 3. 下影线评分（20分）- 反映下方支撑与探底力度
    /// - **计算方式**: 下影线在10%-30%区间最佳得20分，偏离则递减
    /// - **理论依据**: 
    ///   - 适度下影线（10%-30%）表示盘中有回调但获得支撑，说明下方买盘强劲
    ///   - 下影线过短（<10%）可能表示没有充分换手，上涨基础不稳
    ///   - 下影线过长（>30%）说明盘中大幅下探，多方力量不足
    /// - **实例**:
    ///   - 下影线20%振幅 → 20分（最佳）
    ///   - 下影线5%振幅 → 10分
    ///   - 下影线50%振幅 → 约6分
    /// 
    /// ### 4. 收盘位置（30分）- 反映当日收盘强弱
    /// - **计算方式**: `(收盘价 - 最低价) / 振幅 × 30`
    /// - **理论依据**: 收盘价越接近最高价，说明多方在尾盘仍保持强势，次日延续上涨概率大
    /// - **实例**:
    ///   - 收盘在最高价（涨停板）→ 30分
    ///   - 收盘在中间位置 → 15分
    ///   - 收盘在最低价 → 0分
    /// 
    /// ### 5. 阴阳线调整（±10分）- 反映当日涨跌方向
    /// - **计算方式**: 阳线+10分，阴线-10分
    /// - **理论依据**: 阳线表示多方胜出，阴线表示空方胜出，直接影响市场情绪
    /// 
    /// ## 综合评分标准
    /// - **80-100分**: 极强势，大阳线且上影线短、收盘价高
    /// - **60-80分**: 强势，阳线且形态较好
    /// - **40-60分**: 中性，震荡或小幅波动
    /// - **20-40分**: 弱势，阴线或形态较差
    /// - **0-20分**: 极弱势，大阴线且下影线长
    fn calculate_daily_strength(&self, data: &SecurityData) -> f64 {
        let open = data.open;
        let close = data.close;
        let high = data.high;
        let low = data.low;
        
        // 避免除零：一字板（涨停或跌停）返回中性分数
        if high == low {
            return 50.0;
        }
        
        let range = high - low;
        let body = (close - open).abs();
        let is_bullish = close > open;
        
        let mut score = 0.0;
        
        // 1. 实体强度（30分）：实体占全天振幅的比例
        let body_ratio = body / range;
        let body_score = body_ratio * 30.0;
        score += body_score;
        
        // 2. 上影线评分（20分）：上影线越短越强
        let upper_shadow = if is_bullish {
            high - close
        } else {
            high - open
        };
        let upper_shadow_ratio = upper_shadow / range;
        let upper_shadow_score = (1.0 - upper_shadow_ratio) * 20.0;
        score += upper_shadow_score;
        
        // 3. 下影线评分（20分）：下影线适中最好（表示有支撑但不是大幅下探）
        let lower_shadow = if is_bullish {
            open - low
        } else {
            close - low
        };
        let lower_shadow_ratio = lower_shadow / range;
        // 下影线在10%-30%之间最佳
        let lower_shadow_score = if lower_shadow_ratio >= 0.1 && lower_shadow_ratio <= 0.3 {
            20.0
        } else if lower_shadow_ratio < 0.1 {
            lower_shadow_ratio * 200.0 // 0-10%: 0-20分
        } else {
            20.0 - (lower_shadow_ratio - 0.3) * 28.57 // 30%以上递减
        };
        score += lower_shadow_score.max(0.0).min(20.0);
        
        // 4. 收盘位置（30分）：收盘价在全天区间的位置
        let close_position = (close - low) / range;
        let close_position_score = close_position * 30.0;
        score += close_position_score;
        
        // 5. 涨跌调整：阳线加分，阴线减分
        if is_bullish {
            score += 10.0;
        } else {
            score -= 10.0;
        }
        
        // 确保分数在0-100之间
        score.max(0.0).min(100.0)
    }
    
    /// 计算最近连续强势天数
    fn count_recent_strong_days(&self, scores: &[f64]) -> usize {
        let mut count = 0;
        for &score in scores.iter().rev() {
            if score >= 60.0 {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
    
    /// 分析强度趋势
    fn analyze_strength_trend(&self, scores: &[f64]) -> String {
        if scores.len() < 3 {
            return "数据不足".to_string();
        }
        
        // 将数据分为前半段和后半段
        let mid = scores.len() / 2;
        let first_half_avg = scores[..mid].iter().sum::<f64>() / mid as f64;
        let second_half_avg = scores[mid..].iter().sum::<f64>() / (scores.len() - mid) as f64;
        
        let diff = second_half_avg - first_half_avg;
        
        if diff > 5.0 {
            "上升".to_string()
        } else if diff < -5.0 {
            "下降".to_string()
        } else {
            "平稳".to_string()
        }
    }
    
    /// 生成策略信号
    fn generate_signal(
        &self,
        avg_strength: f64,
        strong_ratio: f64,
        recent_strong: usize,
        trend: &str,
    ) -> (StrategySignal, u8, u8) {
        let mut signal_strength = 0u8;
        let mut risk_level = 3u8;
        
        // 平均强度评分（40分）
        let strength_score = ((avg_strength / 100.0) * 40.0) as u8;
        signal_strength += strength_score;
        
        // 强势天数占比评分（30分）
        let ratio_score = (strong_ratio * 30.0) as u8;
        signal_strength += ratio_score;
        
        // 最近连续强势天数评分（20分）
        let recent_score = ((recent_strong as f64 / self.config.require_recent_strong_days as f64).min(1.0) * 20.0) as u8;
        signal_strength += recent_score;
        
        // 趋势评分（10分）
        let trend_score = match trend {
            "上升" => 10,
            "平稳" => 5,
            _ => 0,
        };
        signal_strength += trend_score;
        
        // 根据趋势调整风险等级
        if trend == "上升" {
            risk_level = risk_level.saturating_sub(1);
        } else if trend == "下降" {
            risk_level = (risk_level + 1).min(5);
        }
        
        // 根据平均强度调整风险
        if avg_strength >= 80.0 {
            risk_level = risk_level.saturating_sub(1);
        } else if avg_strength < 50.0 {
            risk_level = (risk_level + 1).min(5);
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
        avg_strength: f64,
        strong_ratio: f64,
        recent_strong: usize,
        trend: &str,
        meets_criteria: bool,
    ) -> String {
        if !meets_criteria {
            return format!(
                "价格强度不足：平均强度{:.1}分，强势天数占比{:.1}%，最近{}天连续强势",
                avg_strength,
                strong_ratio * 100.0,
                recent_strong
            );
        }
        
        format!(
            "价格强势：平均强度{:.1}分，{:.0}%天数强势，最近{}天连续强势，趋势{}",
            avg_strength,
            strong_ratio * 100.0,
            recent_strong,
            trend
        )
    }
}

impl Default for PriceStrengthStrategy {
    fn default() -> Self {
        Self::new(PriceStrengthConfig::default())
    }
}

impl TradingStrategy for PriceStrengthStrategy {
    type Config = PriceStrengthConfig;

    fn name(&self) -> &str {
        "价格强弱策略"
    }

    fn description(&self) -> &str {
        "基于K线形态（开高收低）计算每日价格强度，综合评估股票强弱"
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
        Ok(StrategyResult::PriceStrength(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data(open: f64, high: f64, low: f64, close: f64) -> SecurityData {
        SecurityData {
            trade_date: "20240101".to_string(),
            open,
            high,
            low,
            close,
            volume: 1000000.0,
            amount: 10000000.0,
        }
    }

    #[test]
    fn test_daily_strength_calculation() {
        let strategy = PriceStrengthStrategy::default();

        // 强势阳线：低开高走，上影线短，下影线短
        let strong_bull = create_test_data(10.0, 11.0, 9.9, 10.9);
        let score1 = strategy.calculate_daily_strength(&strong_bull);
        assert!(score1 > 70.0, "强势阳线分数应该>70，实际: {}", score1);

        // 弱势阴线：高开低走
        let weak_bear = create_test_data(10.0, 10.1, 9.0, 9.1);
        let score2 = strategy.calculate_daily_strength(&weak_bear);
        assert!(score2 < 50.0, "弱势阴线分数应该<50，实际: {}", score2);

        // 十字星
        let doji = create_test_data(10.0, 10.0, 10.0, 10.0);
        let score3 = strategy.calculate_daily_strength(&doji);
        assert_eq!(score3, 50.0, "十字星应该是50分");
    }

    #[test]
    fn test_config_validation() {
        let config = PriceStrengthConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = PriceStrengthConfig {
            analysis_period: 2,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
