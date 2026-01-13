use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRiseTurnoverConfig {
    pub lookback_days: usize,
    pub min_daily_rise_pct: f64,
    pub min_turnover_rate: f64,
}

impl Default for DailyRiseTurnoverConfig {
    fn default() -> Self {
        Self {
            lookback_days: 5,
            min_daily_rise_pct: 3.0,
            min_turnover_rate: 10.0,
        }
    }
}

impl StrategyConfig for DailyRiseTurnoverConfig {
    fn strategy_name(&self) -> &str {
        "连续上涨且换手率达标策略"
    }

    fn analysis_period(&self) -> usize {
        self.lookback_days
    }

    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("lookback_days 不能为0");
        }
        if self.min_daily_rise_pct < 0.0 {
            bail!("min_daily_rise_pct 不能为负数");
        }
        if self.min_turnover_rate < 0.0 {
            bail!("min_turnover_rate 不能为负数");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRiseTurnoverResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,

    pub lookback_days: usize,
    pub min_daily_rise_pct_required: f64,
    pub min_turnover_rate_required: f64,

    pub min_daily_rise_pct_observed: f64,
    pub min_turnover_rate_observed: f64,

    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
}

pub struct DailyRiseTurnoverStrategy {
    config: DailyRiseTurnoverConfig,
}

impl DailyRiseTurnoverStrategy {
    pub fn new(config: DailyRiseTurnoverConfig) -> Self {
        Self { config }
    }
}

impl TradingStrategy for DailyRiseTurnoverStrategy {
    type Config = DailyRiseTurnoverConfig;

    fn name(&self) -> &str {
        "连续上涨且换手率达标策略"
    }

    fn description(&self) -> &str {
        "过去N天每天涨幅不低于阈值，且每天换手率不低于阈值"
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
        Ok(StrategyResult::DailyRiseTurnover(result))
    }
}

impl DailyRiseTurnoverStrategy {
    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<DailyRiseTurnoverResult> {
        self.validate_data(data)?;

        let n = self.config.lookback_days;
        if data.len() < n {
            bail!("数据不足：需要至少 {} 天数据，实际只有 {} 天", n, data.len());
        }

        let window = &data[data.len() - n..];
        let latest = window
            .last()
            .ok_or_else(|| anyhow::anyhow!("窗口数据为空"))?;

        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let mut min_rise_observed = f64::INFINITY;
        let mut min_turnover_observed = f64::INFINITY;

        for d in window {
            let rise_pct = d.pct_change.ok_or_else(|| {
                anyhow::anyhow!("缺少涨跌幅数据: {} {}", d.symbol, d.trade_date)
            })?;
            let turnover = d.turnover_rate.ok_or_else(|| {
                anyhow::anyhow!("缺少换手率数据: {} {}", d.symbol, d.trade_date)
            })?;

            if rise_pct < self.config.min_daily_rise_pct {
                bail!(
                    "{} 在 {} 涨幅 {:.2}% 低于阈值 {:.2}%",
                    symbol,
                    d.trade_date,
                    rise_pct,
                    self.config.min_daily_rise_pct
                );
            }
            if turnover < self.config.min_turnover_rate {
                bail!(
                    "{} 在 {} 换手率 {:.2}% 低于阈值 {:.2}%",
                    symbol,
                    d.trade_date,
                    turnover,
                    self.config.min_turnover_rate
                );
            }

            min_rise_observed = min_rise_observed.min(rise_pct);
            min_turnover_observed = min_turnover_observed.min(turnover);
        }

        let mut signal_strength = 75u8;
        if min_rise_observed >= self.config.min_daily_rise_pct * 1.5 {
            signal_strength = 85;
        }
        if min_turnover_observed >= self.config.min_turnover_rate * 1.5 {
            signal_strength = signal_strength.saturating_add(5).min(95);
        }

        let strategy_signal = if signal_strength >= 90 {
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
            "过去{}天最小日涨幅 {:.2}%（阈值 {:.2}%），最小换手率 {:.2}%（阈值 {:.2}%）",
            n,
            min_rise_observed,
            self.config.min_daily_rise_pct,
            min_turnover_observed,
            self.config.min_turnover_rate
        );

        Ok(DailyRiseTurnoverResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price: latest.close,
            lookback_days: n,
            min_daily_rise_pct_required: self.config.min_daily_rise_pct,
            min_turnover_rate_required: self.config.min_turnover_rate,
            min_daily_rise_pct_observed: min_rise_observed,
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

    fn make_data(days: usize, pct: f64, turnover: f64) -> Vec<SecurityData> {
        let mut v = Vec::new();
        for i in 0..days {
            v.push(SecurityData {
                symbol: "000001.SZ".to_string(),
                trade_date: format!("202411{:02}", (i % 30) + 1),
                open: 10.0,
                high: 10.0,
                low: 10.0,
                close: 10.0,
                pre_close: None,
                change: None,
                pct_change: Some(pct),
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
    fn test_daily_rise_turnover_ok() {
        let mut s = DailyRiseTurnoverStrategy::new(DailyRiseTurnoverConfig {
            lookback_days: 5,
            min_daily_rise_pct: 3.0,
            min_turnover_rate: 10.0,
        });

        let data = make_data(10, 3.5, 12.0);
        let r = s.analyze("000001.SZ", &data);
        assert!(r.is_ok());
    }
}
