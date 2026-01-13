//! 日/周/月连阳策略
//! 
//! 识别连续阳线形态，捕捉上升趋势的持续信号
//! 
//! ## 策略原理
//! 
//! 连续阳线是技术分析中的重要形态，表示买方力量持续占优，趋势向上。
//! 
//! ### 1. 连阳的含义
//! - **阳线定义**: 收盘价高于开盘价的K线（在A股中，涨跌幅>0）
//! - **上升趋势**: 连续阳线表明买方力量持续
//! - **动能强劲**: 连阳越多，上涨动能越强
//! 
//! ### 2. 连阳判断标准
//! - **连续天数/周数/月数**: 连续阳线的数量
//! - **时间周期**: 日线/周线/月线三种周期
//! - **强度要求**: 可选择要求阳线的涨幅
//! 
//! ### 3. 策略核心逻辑
//! - 按时间周期（日/周/月）判断连续阳线
//! - 统计连续满足条件的阳线数量
//! - 可选检查成交量配合
//! - 可选要求最小涨幅
//! 
//! ### 4. 应用场景
//! - **趋势确认**: 确认上升趋势的持续性
//! - **买入信号**: 连阳可能预示继续上涨
//! - **强度评估**: 通过连阳数量评估趋势强度
//! 
//! ## 风险提示
//! - 连阳越多，回调风险越大
//! - 在弱势市场，连阳后可能大幅回调
//! - 需要关注后续的阴线确认趋势反转

use anyhow::Result;
use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::traits::{
    TradingStrategy, StrategyConfig as StrategyConfigTrait, StrategyResult, StrategySignal,
    SecurityData,
};

/// 时间周期类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimePeriod {
    Daily,   // 日线
    Weekly,  // 周线
    Monthly, // 月线
}

impl TimePeriod {
    pub fn name(&self) -> &str {
        match self {
            TimePeriod::Daily => "日线",
            TimePeriod::Weekly => "周线",
            TimePeriod::Monthly => "月线",
        }
    }
    
    pub fn is_same_period(&self, date1: &NaiveDate, date2: &NaiveDate) -> bool {
        match self {
            TimePeriod::Daily => {
                // 日线：每天都是不同周期
                date1 == date2
            }
            TimePeriod::Weekly => {
                // 周线：判断是否同一周
                let iso_week1 = date1.iso_week();
                let iso_week2 = date2.iso_week();
                let week1 = iso_week1.week();
                let week2 = iso_week2.week();
                let year1 = iso_week1.year();
                let year2 = iso_week2.year();
                week1 == week2 && year1 == year2
            }
            TimePeriod::Monthly => {
                // 月线：判断是否同一月
                date1.year() == date2.year() && date1.month() == date2.month()
            }
        }
    }
}

/// 日/周/月连阳策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConsecutiveBullishConfig {
    /// 时间周期（日线/周线/月线）
    pub time_period: String,
    
    /// 最小连阳数量
    pub min_consecutive_days: usize,
    
    /// 要求单根阳线最小涨幅（百分比，0表示不限制）
    pub min_rise_pct: f64,
    
    /// 是否要求成交量配合（成交量放大）
    pub require_volume_surge: bool,
    
    /// 成交量放大倍数
    pub volume_surge_ratio: f64,
    
    /// 分析周期（用于计算平均成交量）
    pub analysis_period: usize,
}

impl Default for ConsecutiveBullishConfig {
    fn default() -> Self {
        Self {
            time_period: "daily".to_string(),
            min_consecutive_days: 3,     // 至少3连阳
            min_rise_pct: 0.0,           // 不限制涨幅
            require_volume_surge: false,  // 不要求成交量
            volume_surge_ratio: 1.2,     // 成交量放大1.2倍
            analysis_period: 20,          // 过去20天用于计算平均成交量
        }
    }
}

impl StrategyConfigTrait for ConsecutiveBullishConfig {
    fn strategy_name(&self) -> &str {
        "日/周/月连阳策略"
    }
    
    fn analysis_period(&self) -> usize {
        self.min_consecutive_days + self.analysis_period
    }
    
    fn validate(&self) -> Result<()> {
        if !["daily", "weekly", "week", "monthly", "month"].contains(&self.time_period.as_str()) {
            anyhow::bail!("时间周期必须是 'daily', 'weekly' 或 'monthly'");
        }
        
        if self.min_consecutive_days < 2 || self.min_consecutive_days > 10 {
            anyhow::bail!("最小连阳数量应在2-10之间");
        }
        
        if self.min_rise_pct < 0.0 || self.min_rise_pct > 20.0 {
            anyhow::bail!("单根阳线最小涨幅应在0%-20%之间");
        }
        
        if self.volume_surge_ratio < 0.5 || self.volume_surge_ratio > 3.0 {
            anyhow::bail!("成交量放大倍数应在0.5-3.0之间");
        }
        
        if self.analysis_period < 5 || self.analysis_period > 60 {
            anyhow::bail!("分析周期应在5-60天之间");
        }
        
        Ok(())
    }
}

/// 日/周/月连阳策略结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveBullishResult {
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
    /// 时间周期
    pub time_period: String,
    
    /// 连续阳线数量
    pub consecutive_days: usize,
    
    /// 是否满足最小连阳要求
    pub meets_min_consecutive: bool,
    
    /// 各根阳线的数据
    pub bullish_candles: Vec<BullishCandle>,
    
    /// 平均涨幅（百分比）
    pub avg_rise_pct: f64,
    
    /// 总涨幅（从第一根到当前）
    pub total_rise_pct: f64,
    
    /// 最大单根涨幅（百分比）
    pub max_rise_pct: f64,
}

/// 阳线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BullishCandle {
    /// 日期
    pub date: NaiveDate,
    
    /// 开盘价
    pub open: f64,
    
    /// 最高价
    pub high: f64,
    
    /// 最低价
    pub low: f64,
    
    /// 收盘价
    pub close: f64,
    
    /// 涨跌幅（百分比）
    pub pct_chg: f64,
    
    /// 成交量
    pub volume: f64,
    
    /// 成交量比率（相对于平均）
    pub volume_ratio: f64,
}

/// 日/周/月连阳策略
pub struct ConsecutiveBullishStrategy {
    config: ConsecutiveBullishConfig,
}

impl ConsecutiveBullishStrategy {
    pub fn new(config: ConsecutiveBullishConfig) -> Self {
        Self { config }
    }
    
    /// 创建日线标准配置
    pub fn daily_standard() -> Self {
        Self {
            config: ConsecutiveBullishConfig {
                time_period: "daily".to_string(),
                min_consecutive_days: 3,
                min_rise_pct: 0.0,
                require_volume_surge: false,
                volume_surge_ratio: 1.2,
                analysis_period: 20,
                ..Default::default()
            },
        }
    }
    
    /// 创建日线激进配置（更多连阳）
    pub fn daily_aggressive() -> Self {
        Self {
            config: ConsecutiveBullishConfig {
                time_period: "daily".to_string(),
                min_consecutive_days: 5,
                min_rise_pct: 1.0,
                require_volume_surge: true,
                volume_surge_ratio: 1.3,
                analysis_period: 20,
                ..Default::default()
            },
        }
    }
    
    /// 创建周线标准配置
    pub fn weekly_standard() -> Self {
        Self {
            config: ConsecutiveBullishConfig {
                time_period: "weekly".to_string(),
                min_consecutive_days: 3,
                min_rise_pct: 0.0,
                require_volume_surge: false,
                volume_surge_ratio: 1.2,
                analysis_period: 10,
                ..Default::default()
            },
        }
    }
    
    /// 创建月线标准配置
    pub fn monthly_standard() -> Self {
        Self {
            config: ConsecutiveBullishConfig {
                time_period: "monthly".to_string(),
                min_consecutive_days: 3,
                min_rise_pct: 0.0,
                require_volume_surge: false,
                volume_surge_ratio: 1.2,
                analysis_period: 6,
                ..Default::default()
            },
        }
    }
    
    /// 判断是否为阳线
    fn is_bullish(&self, data: &SecurityData) -> bool {
        // A股：涨跌幅>0即为阳线
        data.pct_change.map_or(false, |pct| pct > 0.0)
    }
    
    /// 判断是否满足涨幅要求
    fn meets_min_rise(&self, data: &SecurityData) -> bool {
        if self.config.min_rise_pct <= 0.0 {
            return true;
        }
        data.pct_change.map_or(false, |pct| pct >= self.config.min_rise_pct)
    }
    
    /// 获取平均成交量
    fn calculate_avg_volume(&self, data: &[SecurityData], end_index: usize) -> f64 {
        if data.is_empty() || self.config.analysis_period == 0 {
            return 0.0;
        }

        // 计算均量时不包含当前K线，避免把“放量”稀释掉
        if end_index == 0 {
            return 0.0;
        }

        let end = end_index - 1;
        let start = end.saturating_sub(self.config.analysis_period.saturating_sub(1));
        let sum: f64 = data[start..=end].iter().map(|d| d.volume).sum();
        let count = end - start + 1;

        sum / count as f64
    }
    
    /// 聚合数据到指定周期（周线/月线）
    fn aggregate_to_period(&self, data: &[SecurityData]) -> Vec<SecurityData> {
        if self.config.time_period == "daily" {
            return data.to_vec();
        }
        
        // 按日期排序
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        let mut aggregated = Vec::new();
        let mut current_period_data: Vec<&SecurityData> = Vec::new();
        let mut current_period_key: Option<(i32, u32)> = None;
        let mut prev_period_close: Option<f64> = None;
        let mut invalid_trade_date_count: usize = 0;
        
        for item in &sorted_data {
            let date = match NaiveDate::parse_from_str(&item.trade_date, "%Y%m%d") {
                Ok(d) => d,
                Err(_) => {
                    invalid_trade_date_count += 1;
                    continue;
                }
            };
            
            let period_key = if self.config.time_period == "weekly" || self.config.time_period == "week" {
                let iso_week = date.iso_week();
                (iso_week.year(), iso_week.week())
            } else { // monthly
                (date.year(), date.month())
            };
            
            if current_period_key.is_none() {
                current_period_key = Some(period_key);
                current_period_data.push(item);
            } else if current_period_key == Some(period_key) {
                current_period_data.push(item);
            } else {
                // 新周期，聚合上一周期数据
                if let Some(agg) = self.aggregate_period_data(&current_period_data, prev_period_close) {
                    prev_period_close = Some(agg.close);
                    aggregated.push(agg);
                }
                current_period_key = Some(period_key);
                current_period_data.clear();
                current_period_data.push(item);
            }
        }
        
        // 聚合最后一个周期
        if let Some(agg) = self.aggregate_period_data(&current_period_data, prev_period_close) {
            aggregated.push(agg);
        }

        if invalid_trade_date_count > 0 {
            info!(
                "[consecutive_bullish] aggregate_to_period skipped invalid trade_date rows={} time_period={}",
                invalid_trade_date_count,
                self.config.time_period
            );
        }

        aggregated
    }
    
    /// 聚合一个周期的数据（周线/月线）
    fn aggregate_period_data(
        &self,
        data: &[&SecurityData],
        prev_period_close: Option<f64>,
    ) -> Option<SecurityData> {
        if data.is_empty() {
            return None;
        }
        
        // 按日期排序
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        let first = sorted.first()?;
        let last = sorted.last()?;
        
        // 周线/月线：open=第一个交易日开盘，close=最后一个交易日收盘
        let open = first.open;
        let close = last.close;
        let high = sorted.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);
        let low = sorted.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
        let volume = sorted.iter().map(|d| d.volume).sum();
        let amount = sorted.iter().map(|d| d.amount).sum();

        // 周/月涨跌幅按“上一周期收盘 -> 本周期收盘”计算（更贴近日线 pct_chg）
        // 若缺少上一周期收盘，则回退到该周期第一天的 pre_close（如果有）。
        let period_pre_close = prev_period_close.or_else(|| first.pre_close);
        let (change, pct_change) = if let Some(pre_close) = period_pre_close {
            if pre_close > 0.0 {
                let chg = close - pre_close;
                let pct = chg / pre_close * 100.0;
                (Some(chg), Some(pct))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        
        Some(SecurityData {
            trade_date: last.trade_date.clone(),
            symbol: last.symbol.clone(),
            open,
            high,
            low,
            close,
            pre_close: period_pre_close,
            change,
            volume,
            amount,
            turnover_rate: last.turnover_rate,
            pct_change,
            time_frame: last.time_frame.clone(),
            security_type: last.security_type.clone(),
            financial_data: last.financial_data.clone(),
            target: last.target.clone(),
        })
    }
    
    /// 计算连续阳线
    fn calculate_consecutive_bullish(&self, data: &[SecurityData], latest_index: usize) -> Vec<BullishCandle> {
        let mut consecutive_bullish = Vec::new();
        
        // 从最新一天往前回溯，找连续阳线
        for i in (0..=latest_index).rev() {
            let item = &data[i];
            
            // 判断是否为阳线
            if !self.is_bullish(item) {
                break;
            }
            
            // 判断是否满足最小涨幅要求
            if !self.meets_min_rise(item) {
                break;
            }
            
            // 计算成交量比率
            let avg_volume = self.calculate_avg_volume(data, i);
            let volume_ratio = if avg_volume > 0.0 {
                item.volume / avg_volume
            } else {
                1.0
            };
            
            // 如果要求成交量配合，检查是否满足
            if self.config.require_volume_surge && volume_ratio < self.config.volume_surge_ratio {
                break;
            }
            
            // 解析日期
            let date = match NaiveDate::parse_from_str(&item.trade_date, "%Y%m%d") {
                Ok(d) => d,
                Err(_) => continue,
            };
            
            let candle = BullishCandle {
                date,
                open: item.open,
                high: item.high,
                low: item.low,
                close: item.close,
                pct_chg: item.pct_change.unwrap_or(0.0),
                volume: item.volume,
                volume_ratio,
            };
            
            consecutive_bullish.insert(0, candle);
        }
        
        consecutive_bullish
    }
    
    /// 计算信号强度
    fn calculate_signal_strength(
        &self,
        consecutive_days: usize,
        avg_rise_pct: f64,
        volume_avg_ratio: f64,
    ) -> u8 {
        let mut score = 0.0;
        
        // 连阳数量评分（50分）
        let consecutive_score = ((consecutive_days as f64) / 10.0).min(1.0) * 50.0;
        score += consecutive_score;
        
        // 平均涨幅评分（30分）
        let rise_score = (avg_rise_pct / 10.0).min(1.0) * 30.0;
        score += rise_score;
        
        // 成交量评分（20分）
        let volume_score = if self.config.require_volume_surge {
            ((volume_avg_ratio - 1.0) / 1.0).min(1.0) * 20.0
        } else {
            10.0
        };
        score += volume_score;
        
        score.min(100.0) as u8
    }
    
    /// 内部分析方法
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<ConsecutiveBullishResult> {
        let required_period = self.config.analysis_period();
        
        if data.len() < required_period {
            anyhow::bail!(
                "数据不足: 需要 {} 个数据点，实际 {} 个",
                required_period,
                data.len()
            );
        }
        
        // 聚合数据到指定周期
        let aggregated_data = self.aggregate_to_period(data);
        
        // 按日期排序
        let mut sorted_data = aggregated_data.to_vec();
        sorted_data.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
        
        // 取最新的数据点作为分析基准
        let latest_index = sorted_data.len() - 1;
        let latest = &sorted_data[latest_index];
        
        let current_price = latest.close;
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")?;
        
        // 计算连续阳线
        let bullish_candles = self.calculate_consecutive_bullish(&sorted_data, latest_index);
        let consecutive_days = bullish_candles.len();
        
        // 计算统计信息
        let avg_rise_pct = if consecutive_days > 0 {
            bullish_candles.iter().map(|c| c.pct_chg).sum::<f64>() / consecutive_days as f64
        } else {
            0.0
        };
        
        let total_rise_pct = if consecutive_days >= 2 {
            let first_close = bullish_candles.first().unwrap().close;
            let last_close = bullish_candles.last().unwrap().close;
            if first_close > 0.0 {
                (last_close - first_close) / first_close * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        let max_rise_pct = if consecutive_days > 0 {
            bullish_candles.iter().map(|c| c.pct_chg).fold(f64::NEG_INFINITY, f64::max)
        } else {
            0.0
        };
        
        let volume_avg_ratio = if consecutive_days > 0 {
            bullish_candles.iter().map(|c| c.volume_ratio).sum::<f64>() / consecutive_days as f64
        } else {
            1.0
        };
        
        // 判断是否满足最小连阳要求
        let meets_min_consecutive = consecutive_days >= self.config.min_consecutive_days;

        if meets_min_consecutive
            && (self.config.time_period == "weekly"
                || self.config.time_period == "week"
                || self.config.time_period == "monthly"
                || self.config.time_period == "month")
        {
            let closes = bullish_candles
                .iter()
                .map(|c| format!("{}:{:.2}", c.date.format("%Y-%m-%d"), c.close))
                .collect::<Vec<_>>()
                .join(", ");

            let details = bullish_candles
                .iter()
                .map(|c| {
                    format!(
                        "{} c={:.2} pct={:.2}%",
                        c.date.format("%Y-%m-%d"),
                        c.close,
                        c.pct_chg
                    )
                })
                .collect::<Vec<_>>()
                .join(" | ");
            info!(
                "[consecutive_bullish] ts_code={} {} {}连阳 closes=[{}] details=[{}]",
                symbol,
                self.config.time_period,
                consecutive_days,
                closes,
                details
            );
        }
        
        // 生成策略信号
        let (strategy_signal, signal_strength, risk_level) = if meets_min_consecutive {
            let strength = self.calculate_signal_strength(
                consecutive_days,
                avg_rise_pct,
                volume_avg_ratio,
            );
            
            let risk = if consecutive_days >= 7 {
                4 // 连阳过多，风险较高
            } else if consecutive_days >= 5 {
                3
            } else {
                2
            };
            
            let signal = if strength >= 80 {
                StrategySignal::StrongBuy
            } else if strength >= 65 {
                StrategySignal::Buy
            } else if strength >= 50 {
                StrategySignal::Hold
            } else {
                StrategySignal::Sell
            };
            
            (signal, strength, risk)
        } else {
            (StrategySignal::Hold, 0, 3)
        };
        
        // 生成分析说明
        let analysis_description = if meets_min_consecutive {
            let period_name = match self.config.time_period.as_str() {
                "daily" => "日",
                "weekly" | "week" => "周",
                "monthly" | "month" => "月",
                _ => "",
            };
            
            format!(
                "{}线连阳{}{}，总涨幅{:.1}%",
                period_name,
                consecutive_days,
                if self.config.min_rise_pct > 0.0 {
                    format!("（单根涨幅≥{:.1}%）", self.config.min_rise_pct)
                } else {
                    String::new()
                },
                total_rise_pct
            )
        } else {
            format!(
                "不符合条件：连阳{}天（要求≥{}天）",
                consecutive_days,
                self.config.min_consecutive_days
            )
        };
        
        debug!(
            "股票 {}: {}线连阳{}, 总涨幅{:.1}%, 信号={:?}",
            symbol,
            self.config.time_period,
            consecutive_days,
            total_rise_pct,
            strategy_signal
        );
        
        Ok(ConsecutiveBullishResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            pct_chg: latest.pct_change.unwrap_or(0.0),
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
            time_period: match self.config.time_period.as_str() {
                "daily" => "日线".to_string(),
                "weekly" | "week" => "周线".to_string(),
                "monthly" | "month" => "月线".to_string(),
                _ => self.config.time_period.clone(),
            },
            consecutive_days,
            meets_min_consecutive,
            bullish_candles,
            avg_rise_pct,
            total_rise_pct,
            max_rise_pct,
        })
    }
}

impl Default for ConsecutiveBullishStrategy {
    fn default() -> Self {
        Self::new(ConsecutiveBullishConfig::default())
    }
}

impl TradingStrategy for ConsecutiveBullishStrategy {
    type Config = ConsecutiveBullishConfig;
    
    fn name(&self) -> &str {
        "日/周/月连阳策略"
    }
    
    fn description(&self) -> &str {
        "识别连续阳线形态，捕捉上升趋势的持续信号"
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
        Ok(StrategyResult::ConsecutiveBullish(result))
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
        pct_change: f64,
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
            pct_change: Some(pct_change),
            time_frame: crate::strategy::traits::TimeFrame::Daily,
            security_type: crate::strategy::traits::SecurityType::Stock,
            financial_data: None,
            target: None,
        }
    }
    
    #[test]
    fn test_daily_consecutive_bullish() {
        let mut strategy = ConsecutiveBullishStrategy::daily_standard();
        
        let mut data = Vec::new();
        // 创建连续5天阳线
        for i in 0..23 {
            let close = 100.0 + (i as f64 * 0.5);
            data.push(create_test_data(
                close - 0.2, close * 1.01, close - 0.5, close,
                1000000.0, 0.5 + (i as f64 * 0.05),
                &format!("202401{:02}", i + 1)
            ));
        }
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::ConsecutiveBullish(r) = result {
            assert!(r.consecutive_days >= 3, "应该至少3连阳");
            assert!(r.meets_min_consecutive, "应该满足最小连阳要求");
            assert!(r.signal_strength > 0, "应该有信号强度");
        }
    }
    
    #[test]
    fn test_weekly_aggregation() {
        let mut strategy = ConsecutiveBullishStrategy::weekly_standard();
        
        let mut data = Vec::new();
        // 创建4周的数据
        for week in 0..4 {
            for day in 0..5 {
                let close = 100.0 + (week as f64 * 2.0) + (day as f64 * 0.3);
                data.push(create_test_data(
                    close - 0.2, close * 1.01, close - 0.5, close,
                    1000000.0, 0.5,
                    &format!("202401{:02}", week * 7 + day + 1)
                ));
            }
        }
        
        let result = strategy.analyze("TEST001", &data).unwrap();
        
        if let StrategyResult::ConsecutiveBullish(r) = result {
            assert!(r.time_period == "周线");
            assert!(r.consecutive_days >= 2, "应该至少2周连阳");
        }
    }
    
    #[test]
    fn test_config_validation() {
        let config = ConsecutiveBullishConfig::default();
        assert!(config.validate().is_ok());
        
        let invalid_config = ConsecutiveBullishConfig {
            time_period: "hourly".to_string(), // 无效周期
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
        
        let invalid_config2 = ConsecutiveBullishConfig {
            min_consecutive_days: 1, // 太少
            ..Default::default()
        };
        assert!(invalid_config2.validate().is_err());
    }
}
