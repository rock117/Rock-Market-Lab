use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 换手率区间涨幅策略配置
///
/// 核心规则：
/// - 过去 `lookback_days` 天内：
///   - 每天换手率 `turnover_rate` 必须在 [turnover_rate_min, turnover_rate_max] 范围内
///   - 每天涨跌幅 `pct_change` 必须在 [daily_pct_change_min, daily_pct_change_max] 范围内
/// - 同一窗口内，从起始收盘价到最新收盘价的累计涨幅必须在 [total_rise_min_pct, total_rise_max_pct] 范围内
pub struct TurnoverRiseConfig {
    /// 回溯天数（窗口大小）
    pub lookback_days: usize,

    /// 前置窗口天数（发生在 lookback_days 窗口之前）
    ///
    /// 例如：lookback_days=5, pre_lookback_days=3
    /// 则前置窗口为 d8-d6（紧邻 lookback 窗口之前的 3 天）
    pub pre_lookback_days: usize,

    /// 前置窗口累计涨跌幅范围（%）
    pub pre_total_rise_min_pct: f64,
    pub pre_total_rise_max_pct: f64,

    /// 每日换手率范围（%）
    pub turnover_rate_min: f64,
    pub turnover_rate_max: f64,

    /// 每日涨跌幅范围（%）
    pub daily_pct_change_min: f64,
    pub daily_pct_change_max: f64,

    /// 区间累计涨幅范围（%）
    pub total_rise_min_pct: f64,
    pub total_rise_max_pct: f64,
}

impl Default for TurnoverRiseConfig {
    fn default() -> Self {
        Self {
            lookback_days: 5,
            pre_lookback_days: 3,
            pre_total_rise_min_pct: 3.0,
            pre_total_rise_max_pct: 5.0,
            turnover_rate_min: 3.0,
            turnover_rate_max: 100.0,
            daily_pct_change_min: 3.0,
            daily_pct_change_max: 20.0,
            total_rise_min_pct: 15.0,
            total_rise_max_pct: 5000.0,
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

        if self.pre_total_rise_min_pct > self.pre_total_rise_max_pct {
            bail!("pre_total_rise_min_pct 不能大于 pre_total_rise_max_pct");
        }

        if self.turnover_rate_min < 0.0 || self.turnover_rate_max < 0.0 {
            bail!("turnover_rate_min/turnover_rate_max 不能为负数");
        }
        if self.turnover_rate_min > self.turnover_rate_max {
            bail!("turnover_rate_min 不能大于 turnover_rate_max");
        }

        if self.daily_pct_change_min > self.daily_pct_change_max {
            bail!("daily_pct_change_min 不能大于 daily_pct_change_max");
        }

        if self.total_rise_min_pct > self.total_rise_max_pct {
            bail!("total_rise_min_pct 不能大于 total_rise_max_pct");
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

    /// 前置窗口天数
    pub pre_lookback_days: usize,
    /// 前置窗口累计涨幅（%）
    pub pre_total_rise_pct: Option<f64>,
    /// 前置窗口累计涨幅范围要求（%）
    pub pre_total_rise_min_required: f64,
    pub pre_total_rise_max_required: f64,

    /// 窗口起始收盘价
    pub start_price: f64,
    /// 窗口累计涨幅（%）
    pub total_rise_pct: f64,

    /// 换手率范围要求（%）
    pub turnover_rate_min_required: f64,
    pub turnover_rate_max_required: f64,
    /// 窗口内观测到的换手率范围（%）
    pub turnover_rate_min_observed: f64,
    pub turnover_rate_max_observed: f64,

    /// 每日涨跌幅范围要求（%）
    pub daily_pct_change_min_required: f64,
    pub daily_pct_change_max_required: f64,
    /// 窗口内观测到的每日涨跌幅范围（%）
    pub daily_pct_change_min_observed: f64,
    pub daily_pct_change_max_observed: f64,

    /// 区间累计涨幅范围要求（%）
    pub total_rise_min_required: f64,
    pub total_rise_max_required: f64,

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

}

impl TradingStrategy for TurnoverRiseStrategy {
    type Config = TurnoverRiseConfig;

    fn name(&self) -> &str {
        "换手率区间涨幅策略"
    }

    fn description(&self) -> &str {
        "过去N天每日换手率/涨跌幅在指定范围内，且区间累计涨幅在指定范围内"
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
        let pre_n = self.config.pre_lookback_days;
        if data.len() < n + pre_n {
            bail!(
                "数据不足：需要至少 {} 天数据(lookback_days={} + pre_lookback_days={})，实际只有 {} 天",
                n + pre_n,
                n,
                pre_n,
                data.len()
            );
        }

        let mut pre_total_rise_pct: Option<f64> = None;
        if pre_n > 0 {
            let pre_window_end = data.len() - n;
            let pre_window_start = pre_window_end - pre_n;
            let pre_window = &data[pre_window_start..pre_window_end];

            let pre_start = pre_window
                .first()
                .ok_or_else(|| anyhow::anyhow!("前置窗口数据为空"))?;
            let pre_latest = pre_window
                .last()
                .ok_or_else(|| anyhow::anyhow!("前置窗口数据为空"))?;

            let pre_start_price = pre_start.close;
            let pre_latest_price = pre_latest.close;
            if pre_start_price <= 0.0 {
                bail!("前置窗口起始价格非法: {}", pre_start_price);
            }

            let pct = (pre_latest_price - pre_start_price) / pre_start_price * 100.0;
            pre_total_rise_pct = Some(pct);

            if pct < self.config.pre_total_rise_min_pct || pct > self.config.pre_total_rise_max_pct {
                bail!(
                    "{} 前置窗口{}天(紧邻lookback之前)累计涨幅 {:.2}% 不在范围 [{:.2}%, {:.2}%]",
                    symbol,
                    pre_n,
                    pct,
                    self.config.pre_total_rise_min_pct,
                    self.config.pre_total_rise_max_pct
                );
            }
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

        let mut turnover_min_observed = f64::INFINITY;
        let mut turnover_max_observed = f64::NEG_INFINITY;

        let mut pct_min_observed = f64::INFINITY;
        let mut pct_max_observed = f64::NEG_INFINITY;
        for d in window {
            let turnover = d.turnover_rate.ok_or_else(|| anyhow::anyhow!(
                "缺少换手率数据: {} {}",
                d.symbol,
                d.trade_date
            ))?;

            if turnover < self.config.turnover_rate_min || turnover > self.config.turnover_rate_max {
                bail!(
                    "{} 在 {} 换手率 {:.2}% 不在范围 [{:.2}%, {:.2}%]",
                    symbol,
                    d.trade_date,
                    turnover,
                    self.config.turnover_rate_min,
                    self.config.turnover_rate_max
                );
            }

            turnover_min_observed = turnover_min_observed.min(turnover);
            turnover_max_observed = turnover_max_observed.max(turnover);

            let pct = d.pct_change.ok_or_else(|| anyhow::anyhow!(
                "缺少涨跌幅数据: {} {}",
                d.symbol,
                d.trade_date
            ))?;

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

        let mut signal_strength = 70u8;
        if total_rise_pct >= self.config.total_rise_min_pct + (self.config.total_rise_max_pct - self.config.total_rise_min_pct) * 0.6 {
            signal_strength = 85;
        }

        let strategy_signal = if signal_strength >= 85 {
            StrategySignal::StrongBuy
        } else {
            StrategySignal::Buy
        };

        let risk_level = if self.config.turnover_rate_max >= 30.0 {
            4
        } else {
            3
        };

        let analysis_description = format!(
            "前置{}天累计涨幅{}（要求[{:.2}%,{:.2}%]），过去{}天换手率范围[{:.2}%,{:.2}%]（要求[{:.2}%,{:.2}%]），日涨跌幅范围[{:.2}%,{:.2}%]（要求[{:.2}%,{:.2}%]），区间涨幅 {:.2}%（要求[{:.2}%,{:.2}%]）",
            pre_n,
            pre_total_rise_pct
                .map(|v| format!("{:.2}%", v))
                .unwrap_or_else(|| "N/A".to_string()),
            self.config.pre_total_rise_min_pct,
            self.config.pre_total_rise_max_pct,
            n,
            turnover_min_observed,
            turnover_max_observed,
            self.config.turnover_rate_min,
            self.config.turnover_rate_max,
            pct_min_observed,
            pct_max_observed,
            self.config.daily_pct_change_min,
            self.config.daily_pct_change_max,
            total_rise_pct,
            self.config.total_rise_min_pct,
            self.config.total_rise_max_pct
        );

        Ok(TurnoverRiseResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price,
            lookback_days: n,
            pre_lookback_days: pre_n,
            pre_total_rise_pct,
            pre_total_rise_min_required: self.config.pre_total_rise_min_pct,
            pre_total_rise_max_required: self.config.pre_total_rise_max_pct,
            start_price,
            total_rise_pct,
            turnover_rate_min_required: self.config.turnover_rate_min,
            turnover_rate_max_required: self.config.turnover_rate_max,
            turnover_rate_min_observed: turnover_min_observed,
            turnover_rate_max_observed: turnover_max_observed,
            daily_pct_change_min_required: self.config.daily_pct_change_min,
            daily_pct_change_max_required: self.config.daily_pct_change_max,
            daily_pct_change_min_observed: pct_min_observed,
            daily_pct_change_max_observed: pct_max_observed,
            total_rise_min_required: self.config.total_rise_min_pct,
            total_rise_max_required: self.config.total_rise_max_pct,
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
                pre_close: Some(close),
                change: Some(0.0),
                pct_change: Some(1.0),
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
            pre_lookback_days: 3,
            pre_total_rise_min_pct: -100.0,
            pre_total_rise_max_pct: 200.0,
            turnover_rate_min: 3.0,
            turnover_rate_max: 10.0,
            daily_pct_change_min: -100.0,
            daily_pct_change_max: 200.0,
            total_rise_min_pct: 1.0,
            total_rise_max_pct: 50.0,
        });

        let data = make_data(10, 10.0, 0.1, 5.0);
        let r = s.analyze("000001.SZ", &data);
        assert!(r.is_ok());
    }

    #[test]
    fn test_turnover_rise_strategy_pre_window_fail() {
        let mut s = TurnoverRiseStrategy::new(TurnoverRiseConfig {
            lookback_days: 5,
            pre_lookback_days: 3,
            pre_total_rise_min_pct: 10.0,
            pre_total_rise_max_pct: 20.0,
            turnover_rate_min: 3.0,
            turnover_rate_max: 10.0,
            daily_pct_change_min: -100.0,
            daily_pct_change_max: 200.0,
            total_rise_min_pct: 1.0,
            total_rise_max_pct: 50.0,
        });

        // make_data 生成的是线性上升(close 每天 +0.1)，前置3天累计涨幅很小，无法达到 [10%, 20%]
        let data = make_data(10, 10.0, 0.1, 5.0);
        let r = s.analyze("000001.SZ", &data);
        assert!(r.is_err());
    }
}
