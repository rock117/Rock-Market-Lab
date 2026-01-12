use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 换手率区间涨幅策略配置
///
/// 核心规则：
/// - 过去 `lookback_days` 天内，每天换手率 `turnover_rate` 都必须 >= `min_turnover_rate`
/// - 同一窗口内，从起始收盘价到最新收盘价的累计涨幅必须 >= `min_price_rise_pct`
pub struct TurnoverRiseConfig {
    /// 回溯天数（窗口大小）
    pub lookback_days: usize,
    /// 最小换手率阈值（百分比，单位：%）
    pub min_turnover_rate: f64,
    /// 最小区间涨幅阈值（百分比，单位：%）
    pub min_price_rise_pct: f64,
}

impl Default for TurnoverRiseConfig {
    fn default() -> Self {
        Self {
            lookback_days: 5,
            min_turnover_rate: 3.0,
            min_price_rise_pct: 5.0,
        }
    }
}

impl StrategyConfig for TurnoverRiseConfig {
    fn strategy_name(&self) -> &str {
        "换手率区间涨幅策略"
    }

    fn analysis_period(&self) -> usize {
        self.lookback_days
    }

    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("lookback_days 不能为0");
        }
        if self.min_turnover_rate < 0.0 {
            bail!("min_turnover_rate 不能为负数");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 换手率区间涨幅策略输出结果
pub struct TurnoverRiseResult {
    /// 股票代码
    pub stock_code: String,
    /// 分析日期（窗口内最后一个交易日）
    pub analysis_date: NaiveDate,
    /// 当前价格（窗口内最后一个收盘价）
    pub current_price: f64,

    /// 回溯天数（窗口大小）
    pub lookback_days: usize,

    /// 窗口起始收盘价
    pub start_price: f64,
    /// 窗口累计涨幅（%）
    pub total_rise_pct: f64,

    /// 要求的最小换手率阈值（%）
    pub min_turnover_rate_required: f64,
    /// 窗口内观测到的最小换手率（%）
    pub min_turnover_rate_observed: f64,

    /// 策略信号
    pub strategy_signal: StrategySignal,
    /// 信号强度（0-100）
    pub signal_strength: u8,
    /// 分析描述（可用于前端展示）
    pub analysis_description: String,
    /// 风险等级（1-5）
    pub risk_level: u8,
}

/// 换手率区间涨幅策略
pub struct TurnoverRiseStrategy {
    config: TurnoverRiseConfig,
}

impl TurnoverRiseStrategy {
    /// 创建策略实例
    pub fn new(config: TurnoverRiseConfig) -> Self {
        Self { config }
    }

    /// 预设：标准
    pub fn standard() -> TurnoverRiseConfig {
        TurnoverRiseConfig::default()
    }

    /// 预设：激进
    pub fn aggressive() -> TurnoverRiseConfig {
        TurnoverRiseConfig {
            lookback_days: 5,
            min_turnover_rate: 5.0,
            min_price_rise_pct: 10.0,
        }
    }

    /// 预设：保守
    pub fn conservative() -> TurnoverRiseConfig {
        TurnoverRiseConfig {
            lookback_days: 10,
            min_turnover_rate: 2.0,
            min_price_rise_pct: 5.0,
        }
    }
}

impl TradingStrategy for TurnoverRiseStrategy {
    type Config = TurnoverRiseConfig;

    fn name(&self) -> &str {
        "换手率区间涨幅策略"
    }

    fn description(&self) -> &str {
        "过去N天每日换手率均高于阈值，且区间累计涨幅高于阈值"
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
        Ok(StrategyResult::TurnoverRise(result))
    }
}

impl TurnoverRiseStrategy {
    /// 核心分析逻辑：
    /// - 校验数据点数量
    /// - 校验窗口内每天换手率不低于阈值
    /// - 计算窗口累计涨幅并校验阈值
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<TurnoverRiseResult> {
        self.validate_data(data)?;

        let n = self.config.lookback_days;
        if data.len() < n {
            bail!("数据不足：需要至少 {} 天数据，实际只有 {} 天", n, data.len());
        }

        let window = &data[data.len() - n..];
        let start = window
            .first()
            .ok_or_else(|| anyhow::anyhow!("窗口数据为空"))?;
        let latest = window
            .last()
            .ok_or_else(|| anyhow::anyhow!("窗口数据为空"))?;

        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let mut min_turnover_observed = f64::INFINITY;
        for d in window {
            let turnover = d.turnover_rate.ok_or_else(|| anyhow::anyhow!(
                "缺少换手率数据: {} {}",
                d.symbol,
                d.trade_date
            ))?;

            if turnover < self.config.min_turnover_rate {
                bail!(
                    "{} 在 {} 换手率 {:.2}% 低于阈值 {:.2}%",
                    symbol,
                    d.trade_date,
                    turnover,
                    self.config.min_turnover_rate
                );
            }

            min_turnover_observed = min_turnover_observed.min(turnover);
        }

        let start_price = start.close;
        let current_price = latest.close;
        if start_price <= 0.0 {
            bail!("起始价格非法: {}", start_price);
        }

        let total_rise_pct = (current_price - start_price) / start_price * 100.0;
        if total_rise_pct < self.config.min_price_rise_pct {
            bail!(
                "{} 过去{}天区间涨幅 {:.2}% 低于阈值 {:.2}%",
                symbol,
                n,
                total_rise_pct,
                self.config.min_price_rise_pct
            );
        }

        let mut signal_strength = 70u8;
        if total_rise_pct >= self.config.min_price_rise_pct * 2.0 {
            signal_strength = 85;
        }

        let strategy_signal = if signal_strength >= 85 {
            StrategySignal::StrongBuy
        } else {
            StrategySignal::Buy
        };

        let risk_level = if self.config.min_turnover_rate >= 10.0 {
            4
        } else {
            3
        };

        let analysis_description = format!(
            "过去{}天最小换手率 {:.2}%（阈值 {:.2}%），区间涨幅 {:.2}%（阈值 {:.2}%）",
            n,
            min_turnover_observed,
            self.config.min_turnover_rate,
            total_rise_pct,
            self.config.min_price_rise_pct
        );

        Ok(TurnoverRiseResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            lookback_days: n,
            start_price,
            total_rise_pct,
            min_turnover_rate_required: self.config.min_turnover_rate,
            min_turnover_rate_observed: min_turnover_observed,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::traits::{SecurityType, TimeFrame};

    fn make_data(days: usize, start_price: f64, daily_rise: f64, turnover: f64) -> Vec<SecurityData> {
        let mut v = Vec::new();
        for i in 0..days {
            let close = start_price + (i as f64) * daily_rise;
            v.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("202411{:02}", (i % 30) + 1),
                open: close,
                high: close,
                low: close,
                close,
                pre_close: None,
                change: None,
                pct_change: None,
                volume: 0.0,
                amount: 0.0,
                turnover_rate: Some(turnover),
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
                financial_data: None,
                target: None,
            });
        }
        v
    }

    #[test]
    fn test_turnover_rise_strategy_ok() {
        let mut s = TurnoverRiseStrategy::new(TurnoverRiseConfig {
            lookback_days: 5,
            min_turnover_rate: 3.0,
            min_price_rise_pct: 2.0,
        });

        let data = make_data(10, 10.0, 0.1, 5.0);
        let r = s.analyze("000001.SZ", &data);
        assert!(r.is_ok());
    }
}
