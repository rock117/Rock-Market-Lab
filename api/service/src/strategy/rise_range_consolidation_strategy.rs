use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{FinancialData, SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 区间涨幅 + 前置横盘策略配置
///
/// 核心规则：
/// - 前置窗口（pre_lookback_days=M，紧邻 lookback_days 之前）：
///   - 以收盘价为基准，振幅 = (max_close - min_close) / min_close * 100
///   - 振幅必须 <= pre_amplitude_max_pct
/// - 过去 lookback_days=N 天：
///   - 每天涨跌幅 pct_change 必须在 [daily_pct_change_min, daily_pct_change_max] 范围内（允许下跌）
///   - 同一窗口内累计涨幅必须在 [total_rise_min_pct, total_rise_max_pct] 范围内
/// - 排名（在 stock_picker_service 中实现）：综合考虑 市值/ROE/dv_ttm 的加权得分（缺失值整体靠后）
pub struct RiseRangeConsolidationConfig {
    pub lookback_days: usize,
    pub daily_pct_change_min: f64,
    pub daily_pct_change_max: f64,
    pub total_rise_min_pct: f64,
    pub total_rise_max_pct: f64,

    pub pre_lookback_days: usize,
    pub pre_amplitude_max_pct: f64,

    pub weight_market_cap: f64,
    pub weight_roe: f64,
    pub weight_dv_ttm: f64,
}

impl Default for RiseRangeConsolidationConfig {
    fn default() -> Self {
        Self {
            lookback_days: 20,
            daily_pct_change_min: -2.0,
            daily_pct_change_max: 3.0,
            total_rise_min_pct: 3.0,
            total_rise_max_pct: 15.0,
            pre_lookback_days: 20,
            pre_amplitude_max_pct: 5.0,
            weight_market_cap: 0.4,
            weight_roe: 0.3,
            weight_dv_ttm: 0.3,
        }
    }
}

impl StrategyConfig for RiseRangeConsolidationConfig {
    fn strategy_name(&self) -> &str {
        "区间涨幅+前置横盘策略"
    }

    fn analysis_period(&self) -> usize {
        self.lookback_days + self.pre_lookback_days
    }

    fn validate(&self) -> Result<()> {
        if self.lookback_days == 0 {
            bail!("lookback_days 不能为0");
        }
        if self.pre_lookback_days == 0 {
            bail!("pre_lookback_days 不能为0");
        }
        if self.daily_pct_change_min > self.daily_pct_change_max {
            bail!("daily_pct_change_min 不能大于 daily_pct_change_max");
        }
        if self.total_rise_min_pct > self.total_rise_max_pct {
            bail!("total_rise_min_pct 不能大于 total_rise_max_pct");
        }
        if self.pre_amplitude_max_pct < 0.0 {
            bail!("pre_amplitude_max_pct 不能为负数");
        }

        if self.weight_market_cap < 0.0 || self.weight_roe < 0.0 || self.weight_dv_ttm < 0.0 {
            bail!("权重不能为负数");
        }
        let sum = self.weight_market_cap + self.weight_roe + self.weight_dv_ttm;
        if sum <= 0.0 {
            bail!("权重之和必须大于0");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiseRangeConsolidationResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,

    pub lookback_days: usize,
    pub pre_lookback_days: usize,

    pub pre_amplitude_pct: f64,
    pub pre_amplitude_max_required: f64,

    pub daily_pct_change_min_required: f64,
    pub daily_pct_change_max_required: f64,
    pub daily_pct_change_min_observed: f64,
    pub daily_pct_change_max_observed: f64,

    pub total_rise_pct: f64,
    pub total_rise_min_required: f64,
    pub total_rise_max_required: f64,

    pub market_cap_yi: Option<f64>,
    pub roe: Option<f64>,
    pub dv_ttm: Option<f64>,

    pub weight_market_cap: f64,
    pub weight_roe: f64,
    pub weight_dv_ttm: f64,

    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
}

pub struct RiseRangeConsolidationStrategy {
    config: RiseRangeConsolidationConfig,
}

impl RiseRangeConsolidationStrategy {
    pub fn new(config: RiseRangeConsolidationConfig) -> Self {
        Self { config }
    }

    fn get_latest_financial<'a>(&self, window: &'a [SecurityData]) -> Option<&'a FinancialData> {
        window
            .iter()
            .rev()
            .find_map(|d| d.financial_data.as_ref())
    }

    fn validate_data(&self, data: &[SecurityData]) -> Result<()> {
        if data.is_empty() {
            bail!("数据为空");
        }
        Ok(())
    }

    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<RiseRangeConsolidationResult> {
        self.validate_data(data)?;
        let n = self.config.lookback_days;
        let m = self.config.pre_lookback_days;
        if data.len() < n + m {
            bail!(
                "数据不足：需要至少 {} 天数据(lookback_days={} + pre_lookback_days={})，实际只有 {} 天",
                n + m,
                n,
                m,
                data.len()
            );
        }

        let pre_window_end = data.len() - n;
        let pre_window_start = pre_window_end - m;
        let pre_window = &data[pre_window_start..pre_window_end];

        let mut pre_min_close = f64::INFINITY;
        let mut pre_max_close = f64::NEG_INFINITY;
        for d in pre_window {
            pre_min_close = pre_min_close.min(d.close);
            pre_max_close = pre_max_close.max(d.close);
        }
        if pre_min_close <= 0.0 {
            bail!("前置窗口最小收盘价非法: {}", pre_min_close);
        }
        let pre_amplitude_pct = (pre_max_close - pre_min_close) / pre_min_close * 100.0;
        if pre_amplitude_pct > self.config.pre_amplitude_max_pct {
            bail!(
                "{} 前置{}天横盘振幅 {:.2}% 超过阈值 {:.2}%",
                symbol,
                m,
                pre_amplitude_pct,
                self.config.pre_amplitude_max_pct
            );
        }

        let window = &data[data.len() - n..];
        let start = window.first().ok_or_else(|| anyhow::anyhow!("窗口数据为空"))?;
        let latest = window.last().ok_or_else(|| anyhow::anyhow!("窗口数据为空"))?;

        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let mut pct_min_observed = f64::INFINITY;
        let mut pct_max_observed = f64::NEG_INFINITY;
        for d in window {
            let pct = d.pct_change.ok_or_else(|| {
                anyhow::anyhow!("缺少涨跌幅数据: {} {}", d.symbol, d.trade_date)
            })?;

            if pct < self.config.daily_pct_change_min || pct > self.config.daily_pct_change_max {
                bail!(
                    "{} 在 {} 涨跌幅 {:.2}% 不在范围 [{:.2}%, {:.2}%]",
                    symbol,
                    d.trade_date,
                    pct,
                    self.config.daily_pct_change_min,
                    self.config.daily_pct_change_max
                );
            }

            pct_min_observed = pct_min_observed.min(pct);
            pct_max_observed = pct_max_observed.max(pct);
        }

        let start_price = start.close;
        let current_price = latest.close;
        if start_price <= 0.0 {
            bail!("起始价格非法: {}", start_price);
        }
        let total_rise_pct = (current_price - start_price) / start_price * 100.0;
        if total_rise_pct < self.config.total_rise_min_pct || total_rise_pct > self.config.total_rise_max_pct {
            bail!(
                "{} 过去{}天区间涨幅 {:.2}% 不在范围 [{:.2}%, {:.2}%]",
                symbol,
                n,
                total_rise_pct,
                self.config.total_rise_min_pct,
                self.config.total_rise_max_pct
            );
        }

        let latest_fin = self.get_latest_financial(window);
        let market_cap_yi = latest_fin
            .and_then(|f| f.market_cap)
            .map(|v_yuan| v_yuan / 100_000_000.0);
        let roe = latest_fin.and_then(|f| f.roe);
        let dv_ttm = latest_fin.and_then(|f| f.dv_ttm);

        let mut signal_strength = 75u8;
        if total_rise_pct >= self.config.total_rise_min_pct + (self.config.total_rise_max_pct - self.config.total_rise_min_pct) * 0.6 {
            signal_strength = 85;
        }

        let strategy_signal = if signal_strength >= 85 {
            StrategySignal::StrongBuy
        } else {
            StrategySignal::Buy
        };

        let analysis_description = format!(
            "前置{}天振幅 {:.2}%（阈值<= {:.2}%），过去{}天日涨跌幅范围[{:.2}%,{:.2}%]（要求[{:.2}%,{:.2}%]），区间涨幅 {:.2}%（要求[{:.2}%,{:.2}%]）。市值(亿)={:?}，ROE={:?}%，股息率TTM={:?}%",
            m,
            pre_amplitude_pct,
            self.config.pre_amplitude_max_pct,
            n,
            pct_min_observed,
            pct_max_observed,
            self.config.daily_pct_change_min,
            self.config.daily_pct_change_max,
            total_rise_pct,
            self.config.total_rise_min_pct,
            self.config.total_rise_max_pct,
            market_cap_yi,
            roe,
            dv_ttm
        );

        Ok(RiseRangeConsolidationResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            lookback_days: n,
            pre_lookback_days: m,
            pre_amplitude_pct,
            pre_amplitude_max_required: self.config.pre_amplitude_max_pct,
            daily_pct_change_min_required: self.config.daily_pct_change_min,
            daily_pct_change_max_required: self.config.daily_pct_change_max,
            daily_pct_change_min_observed: pct_min_observed,
            daily_pct_change_max_observed: pct_max_observed,
            total_rise_pct,
            total_rise_min_required: self.config.total_rise_min_pct,
            total_rise_max_required: self.config.total_rise_max_pct,
            market_cap_yi,
            roe,
            dv_ttm,

            weight_market_cap: self.config.weight_market_cap,
            weight_roe: self.config.weight_roe,
            weight_dv_ttm: self.config.weight_dv_ttm,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level: 2,
        })
    }
}

impl TradingStrategy for RiseRangeConsolidationStrategy {
    type Config = RiseRangeConsolidationConfig;

    fn name(&self) -> &str {
        "区间涨幅+前置横盘策略"
    }

    fn description(&self) -> &str {
        "前置M天横盘(振幅约束)，过去N天每日涨跌幅在区间内且累计涨幅在区间内"
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
        Ok(StrategyResult::RiseRangeConsolidation(result))
    }
}
