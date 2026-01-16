use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaBreakoutConfig {
    pub ma_period: usize,
    pub direction: String,
    pub require_cross: bool,
}

impl Default for MaBreakoutConfig {
    fn default() -> Self {
        Self {
            ma_period: 20,
            direction: "up".to_string(),
            require_cross: true,
        }
    }
}

impl StrategyConfig for MaBreakoutConfig {
    fn strategy_name(&self) -> &str {
        "均线突破/跌破策略"
    }

    fn analysis_period(&self) -> usize {
        self.ma_period + 1
    }

    fn validate(&self) -> Result<()> {
        if self.ma_period == 0 {
            bail!("ma_period 不能为0");
        }
        if self.direction != "up" && self.direction != "down" {
            bail!("direction 只能为 up 或 down");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaBreakoutResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,

    pub ma_period: usize,
    pub direction: String,
    pub require_cross: bool,

    pub prev_close: f64,
    pub prev_ma: f64,
    pub current_ma: f64,

    pub crossed: bool,

    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
}

pub struct MaBreakoutStrategy {
    config: MaBreakoutConfig,
}

impl MaBreakoutStrategy {
    pub fn new(config: MaBreakoutConfig) -> Self {
        Self { config }
    }

    fn validate_data(&self, data: &[SecurityData]) -> Result<()> {
        if data.is_empty() {
            bail!("数据为空");
        }
        Ok(())
    }

    fn calculate_ma_at(&self, data: &[SecurityData], idx: usize, period: usize) -> Result<f64> {
        if period == 0 {
            bail!("period 不能为0");
        }
        if idx + 1 < period {
            bail!("数据不足以计算 {} 日均线", period);
        }
        let start = idx + 1 - period;
        let slice = &data[start..=idx];
        let sum: f64 = slice.iter().map(|d| d.close).sum();
        Ok(sum / period as f64)
    }

    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<MaBreakoutResult> {
        self.validate_data(data)?;
        self.config.validate()?;

        let period = self.config.ma_period;
        if data.len() < period + 1 {
            bail!(
                "数据不足：需要至少 {} 天数据(ma_period={} + 1)，实际只有 {} 天",
                period + 1,
                period,
                data.len()
            );
        }

        let idx_today = data.len() - 1;
        let idx_prev = data.len() - 2;

        let today = &data[idx_today];
        let prev = &data[idx_prev];

        let analysis_date = NaiveDate::parse_from_str(&today.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let prev_ma = self.calculate_ma_at(data, idx_prev, period)?;
        let current_ma = self.calculate_ma_at(data, idx_today, period)?;

        let prev_close = prev.close;
        let current_close = today.close;

        let crossed = match self.config.direction.as_str() {
            "up" => {
                if self.config.require_cross {
                    prev_close <= prev_ma && current_close > current_ma
                } else {
                    current_close > current_ma
                }
            }
            "down" => {
                if self.config.require_cross {
                    prev_close >= prev_ma && current_close < current_ma
                } else {
                    current_close < current_ma
                }
            }
            _ => false,
        };

        if !crossed {
            bail!(
                "{} 未满足 {}{} {}日均线条件",
                symbol,
                if self.config.direction == "up" { "突破" } else { "跌破" },
                if self.config.require_cross { "(穿越)" } else { "" },
                period
            );
        }

        let dist_pct = if current_ma.abs() < 1e-12 {
            0.0
        } else {
            (current_close - current_ma) / current_ma * 100.0
        };

        let mut signal_strength = 75u8;
        let dist_abs = dist_pct.abs();
        if dist_abs >= 2.0 {
            signal_strength = 85;
        }
        if dist_abs >= 5.0 {
            signal_strength = 95;
        }

        let strategy_signal = match self.config.direction.as_str() {
            "up" => {
                if signal_strength >= 90 {
                    StrategySignal::StrongBuy
                } else {
                    StrategySignal::Buy
                }
            }
            "down" => {
                if signal_strength >= 90 {
                    StrategySignal::StrongSell
                } else {
                    StrategySignal::Sell
                }
            }
            _ => StrategySignal::Hold,
        };

        let analysis_description = format!(
            "{}{} MA{}：昨日收盘{:.2}, 昨日MA{:.2}; 今日收盘{:.2}, 今日MA{:.2}; 偏离{:.2}%",
            if self.config.direction == "up" { "突破" } else { "跌破" },
            if self.config.require_cross { "(穿越)" } else { "" },
            period,
            prev_close,
            prev_ma,
            current_close,
            current_ma,
            dist_pct
        );

        Ok(MaBreakoutResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price: current_close,
            ma_period: period,
            direction: self.config.direction.clone(),
            require_cross: self.config.require_cross,
            prev_close,
            prev_ma,
            current_ma,
            crossed,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level: 2,
        })
    }
}

impl TradingStrategy for MaBreakoutStrategy {
    type Config = MaBreakoutConfig;

    fn name(&self) -> &str {
        "均线突破/跌破策略"
    }

    fn description(&self) -> &str {
        "支持突破/跌破 N 日均线，可选是否要求穿越(昨日在一侧，今日到另一侧)"
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
        Ok(StrategyResult::MaBreakout(result))
    }
}
