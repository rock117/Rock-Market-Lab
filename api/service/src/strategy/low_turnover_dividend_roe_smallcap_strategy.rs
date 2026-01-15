use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowTurnoverDividendRoeSmallCapConfig {
    pub lookback_days: usize,

    pub max_avg_turnover_rate: f64,

    pub total_rise_min_pct: f64,

    pub total_rise_max_pct: f64,

    pub min_dv_ttm: f64,

    pub min_roe: f64,

    pub max_market_cap_yi: f64,
}

impl Default for LowTurnoverDividendRoeSmallCapConfig {
    fn default() -> Self {
        Self {
            lookback_days: 20,
            max_avg_turnover_rate: 2.0,
            total_rise_min_pct: 2.0,
            total_rise_max_pct: 15.0,
            min_dv_ttm: 3.0,
            min_roe: 12.0,
            max_market_cap_yi: 100.0,
        }
    }
}

impl StrategyConfig for LowTurnoverDividendRoeSmallCapConfig {
    fn strategy_name(&self) -> &str {
        "低换手高股息高ROE小市值策略"
    }

    fn analysis_period(&self) -> usize {
        self.lookback_days
    }

    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("lookback_days 不能为0");
        }
        if self.max_avg_turnover_rate < 0.0 {
            bail!("max_avg_turnover_rate 不能为负数");
        }
        if self.total_rise_min_pct > self.total_rise_max_pct {
            bail!("total_rise_min_pct 不能大于 total_rise_max_pct");
        }
        if self.min_dv_ttm < 0.0 {
            bail!("min_dv_ttm 不能为负数");
        }
        if self.min_roe < 0.0 {
            bail!("min_roe 不能为负数");
        }
        if self.max_market_cap_yi <= 0.0 {
            bail!("max_market_cap_yi 必须大于0");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowTurnoverDividendRoeSmallCapResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,

    pub p80_turnover_rate: f64,
    pub total_rise_pct: f64,

    pub dv_ttm: Option<f64>,
    pub roe: Option<f64>,
    pub market_cap_yi: f64,

    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
}

pub struct LowTurnoverDividendRoeSmallCapStrategy {
    config: LowTurnoverDividendRoeSmallCapConfig,
}

impl LowTurnoverDividendRoeSmallCapStrategy {
    pub fn new(config: LowTurnoverDividendRoeSmallCapConfig) -> Self {
        Self { config }
    }
}

impl TradingStrategy for LowTurnoverDividendRoeSmallCapStrategy {
    type Config = LowTurnoverDividendRoeSmallCapConfig;

    fn name(&self) -> &str {
        "低换手高股息高ROE小市值策略"
    }

    fn description(&self) -> &str {
        "过去N天换手率偏低且股价温和上涨，同时要求高股息率(dv_ttm)、高ROE、小市值"
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
        self.validate_data(data)?;

        let n = self.config.lookback_days;
        let window = if data.len() > n { &data[data.len() - n..] } else { data };
        if window.len() < 2 {
            bail!("数据不足: 需要至少 2 个数据点");
        }

        let first = &window[0];
        let last = window.last().unwrap();

        let p80_turnover_rate = {
            let mut values: Vec<f64> = window.iter().filter_map(|d| d.turnover_rate).collect();
            if values.is_empty() {
                bail!("缺少换手率数据");
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let n = values.len();
            let idx = ((0.8 * n as f64).ceil() as usize).saturating_sub(1).min(n - 1);
            values[idx]
        };

        if p80_turnover_rate > self.config.max_avg_turnover_rate {
            bail!(
                "换手率P80 {:.2}% 高于上限 {:.2}%",
                p80_turnover_rate,
                self.config.max_avg_turnover_rate
            );
        }

        let total_rise_pct = if first.close <= 0.0 {
            bail!("首日收盘价异常");
        } else {
            (last.close - first.close) / first.close * 100.0
        };

        if total_rise_pct < self.config.total_rise_min_pct {
            bail!(
                "区间涨幅 {:.2}% 低于下限 {:.2}%",
                total_rise_pct,
                self.config.total_rise_min_pct
            );
        }

        if total_rise_pct > self.config.total_rise_max_pct {
            bail!(
                "区间涨幅 {:.2}% 高于上限 {:.2}%",
                total_rise_pct,
                self.config.total_rise_max_pct
            );
        }

        let fd = last
            .financial_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("缺少 financial_data"))?;

        let dv_ttm = fd.dv_ttm;
        if let Some(v) = dv_ttm {
            if v < self.config.min_dv_ttm {
                bail!("股息率 {:.2}% 低于下限 {:.2}%", v, self.config.min_dv_ttm);
            }
        }

        let roe = fd.roe;
        if let Some(v) = roe {
            if v < self.config.min_roe {
                bail!("ROE {:.2}% 低于下限 {:.2}%", v, self.config.min_roe);
            }
        }

        let market_cap_yi = fd
            .market_cap
            .ok_or_else(|| anyhow::anyhow!("缺少市值 market_cap"))?
            / 100_000_000.0;

        if market_cap_yi > self.config.max_market_cap_yi {
            bail!(
                "市值 {:.2}亿 高于上限 {:.2}亿",
                market_cap_yi,
                self.config.max_market_cap_yi
            );
        }

        let analysis_date = NaiveDate::parse_from_str(&last.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let signal_strength = {
            let mut s = 60u8;
            if dv_ttm.is_some_and(|v| v >= self.config.min_dv_ttm + 2.0) {
                s = s.saturating_add(10);
            }
            if roe.is_some_and(|v| v >= self.config.min_roe + 5.0) {
                s = s.saturating_add(10);
            }
            if market_cap_yi <= self.config.max_market_cap_yi * 0.5 {
                s = s.saturating_add(10);
            }
            s.min(100)
        };

        let dv_desc = dv_ttm
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "N/A".to_string());
        let roe_desc = roe
            .map(|v| format!("{:.2}%", v))
            .unwrap_or_else(|| "N/A".to_string());

        let analysis_description = format!(
            "满足条件：近{}天换手率P80{:.2}%，区间涨幅{:.2}%，股息率TTM{}，ROE{}，市值{:.2}亿",
            window.len(),
            p80_turnover_rate,
            total_rise_pct,
            dv_desc,
            roe_desc,
            market_cap_yi
        );

        Ok(StrategyResult::LowTurnoverDividendRoeSmallCap(
            LowTurnoverDividendRoeSmallCapResult {
                stock_code: symbol.to_string(),
                analysis_date,
                current_price: last.close,
                p80_turnover_rate,
                total_rise_pct,
                dv_ttm,
                roe,
                market_cap_yi,
                strategy_signal: StrategySignal::Buy,
                signal_strength,
                analysis_description,
                risk_level: 2,
            },
        ))
    }
}
