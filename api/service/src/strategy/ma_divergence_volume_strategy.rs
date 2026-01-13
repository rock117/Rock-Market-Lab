use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::traits::{SecurityData, StrategyConfig, StrategyResult, StrategySignal, TradingStrategy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaDivergenceVolumeConfig {
    /// MA5 周期（短期均线）
    pub ma5_period: usize,
    /// MA10 周期（中期均线）
    pub ma10_period: usize,
    /// MA20 周期（中长期均线）
    pub ma20_period: usize,

    /// 均线“发散”对比用回看天数：比较当前与 N 天前的 MA 间距是否扩大
    pub gap_lookback_days: usize,

    /// 价格连续站上 MA5 的最小天数（要求 >2 天，一般配置为 3）
    pub min_above_ma5_days: usize,
    /// 价格连续站上 MA5 的最大天数（要求 <5 天，一般配置为 4）
    pub max_above_ma5_days: usize,

    /// 成交量均线周期（用于计算量比 / 放量判断）
    pub volume_ma_period: usize,
    /// 放量阈值：当日成交量 / 成交量均线 >= 该值，则视为放量
    pub volume_surge_ratio: f64,
    /// 连续放量的最小天数（至少 2 天）
    pub min_volume_surge_days: usize,
}

impl Default for MaDivergenceVolumeConfig {
    fn default() -> Self {
        Self {
            ma5_period: 5,
            ma10_period: 10,
            ma20_period: 20,
            gap_lookback_days: 3,
            min_above_ma5_days: 3,
            max_above_ma5_days: 4,
            volume_ma_period: 20,
            volume_surge_ratio: 1.5,
            min_volume_surge_days: 2,
        }
    }
}

impl StrategyConfig for MaDivergenceVolumeConfig {
    fn strategy_name(&self) -> &str {
        "均线向上发散放量策略"
    }

    fn analysis_period(&self) -> usize {
        let need_for_gap = self.ma20_period + self.gap_lookback_days;
        let need_for_above = self.ma5_period + self.max_above_ma5_days;
        let need_for_volume = self.volume_ma_period + self.min_volume_surge_days;
        need_for_gap.max(need_for_above).max(need_for_volume) + 5
    }

    fn validate(&self) -> Result<()> {
        if self.ma5_period == 0 || self.ma10_period == 0 || self.ma20_period == 0 {
            bail!("均线周期不能为0");
        }
        if !(self.ma5_period < self.ma10_period && self.ma10_period < self.ma20_period) {
            bail!("均线周期必须满足 ma5_period < ma10_period < ma20_period");
        }
        if self.gap_lookback_days == 0 {
            bail!("gap_lookback_days 不能为0");
        }
        if self.min_above_ma5_days < 3 {
            bail!("min_above_ma5_days 建议至少为3（>2天）");
        }
        if self.max_above_ma5_days >= 5 {
            bail!("max_above_ma5_days 必须小于5（<5天）");
        }
        if self.min_above_ma5_days > self.max_above_ma5_days {
            bail!("min_above_ma5_days 不能大于 max_above_ma5_days");
        }
        if self.volume_ma_period == 0 {
            bail!("volume_ma_period 不能为0");
        }
        if self.volume_surge_ratio <= 1.0 {
            bail!("volume_surge_ratio 必须大于1.0");
        }
        if self.min_volume_surge_days < 2 {
            bail!("min_volume_surge_days 至少为2");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaDivergenceVolumeResult {
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,
    pub pct_chg: f64,

    pub ma5: f64,
    pub ma10: f64,
    pub ma20: f64,

    pub gap_5_10_now: f64,
    pub gap_10_20_now: f64,
    pub gap_5_10_prev: f64,
    pub gap_10_20_prev: f64,

    pub above_ma5_days: usize,
    pub volume_surge_days: usize,
    pub latest_volume_ratio: f64,

    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
}

pub struct MaDivergenceVolumeStrategy {
    config: MaDivergenceVolumeConfig,
}

impl MaDivergenceVolumeStrategy {
    pub fn new(config: MaDivergenceVolumeConfig) -> Self {
        Self { config }
    }

    pub fn standard() -> MaDivergenceVolumeConfig {
        MaDivergenceVolumeConfig::default()
    }

    pub fn aggressive() -> MaDivergenceVolumeConfig {
        MaDivergenceVolumeConfig {
            gap_lookback_days: 2,
            min_above_ma5_days: 3,
            max_above_ma5_days: 4,
            volume_surge_ratio: 1.3,
            min_volume_surge_days: 2,
            ..Default::default()
        }
    }

    pub fn conservative() -> MaDivergenceVolumeConfig {
        MaDivergenceVolumeConfig {
            gap_lookback_days: 5,
            min_above_ma5_days: 3,
            max_above_ma5_days: 4,
            volume_surge_ratio: 1.8,
            min_volume_surge_days: 3,
            ..Default::default()
        }
    }

    fn calculate_ma_at(&self, data: &[SecurityData], idx: usize, period: usize) -> Result<f64> {
        if idx + 1 < period {
            bail!("数据不足以计算 {} 日均线", period);
        }
        let start = idx + 1 - period;
        let slice = &data[start..=idx];
        let sum: f64 = slice.iter().map(|d| d.close).sum();
        Ok(sum / period as f64)
    }

    fn calculate_volume_ma_at(&self, data: &[SecurityData], idx: usize, period: usize) -> Result<f64> {
        if idx + 1 < period {
            bail!("数据不足以计算 {} 日成交量均线", period);
        }
        let start = idx + 1 - period;
        let slice = &data[start..=idx];
        let sum: f64 = slice.iter().map(|d| d.volume).sum();
        Ok(sum / period as f64)
    }

    fn count_consecutive_above_ma5(&self, data: &[SecurityData], latest_idx: usize) -> Result<usize> {
        let mut count = 0usize;
        for i in (0..=latest_idx).rev() {
            let ma5 = self.calculate_ma_at(data, i, self.config.ma5_period)?;
            if data[i].close > ma5 {
                count += 1;
            } else {
                break;
            }
            if count > self.config.max_above_ma5_days {
                break;
            }
        }
        Ok(count)
    }

    fn count_consecutive_volume_surge(&self, data: &[SecurityData], latest_idx: usize) -> Result<(usize, f64)> {
        let mut count = 0usize;
        let mut latest_ratio = 0.0;

        for i in (0..=latest_idx).rev() {
            let vma = self.calculate_volume_ma_at(data, i, self.config.volume_ma_period)?;
            if vma <= 0.0 {
                break;
            }
            let ratio = data[i].volume / vma;
            if i == latest_idx {
                latest_ratio = ratio;
            }
            if ratio >= self.config.volume_surge_ratio {
                count += 1;
            } else {
                break;
            }
        }

        Ok((count, latest_ratio))
    }

    fn analyze_internal(&self, symbol: &str, data: &[SecurityData]) -> Result<MaDivergenceVolumeResult> {
        self.validate_data(data)?;

        let latest_idx = data.len().saturating_sub(1);
        if data.len() < self.config.analysis_period() {
            bail!("数据不足：需要至少 {} 天数据，实际只有 {} 天", self.config.analysis_period(), data.len());
        }

        let latest = &data[latest_idx];
        let analysis_date = NaiveDate::parse_from_str(&latest.trade_date, "%Y%m%d")
            .map_err(|e| anyhow::anyhow!("日期解析失败: {}", e))?;

        let ma5 = self.calculate_ma_at(data, latest_idx, self.config.ma5_period)?;
        let ma10 = self.calculate_ma_at(data, latest_idx, self.config.ma10_period)?;
        let ma20 = self.calculate_ma_at(data, latest_idx, self.config.ma20_period)?;

        if !(ma5 > ma10 && ma10 > ma20) {
            bail!("均线未形成多头排列：MA5 {:.2} MA10 {:.2} MA20 {:.2}", ma5, ma10, ma20);
        }

        let prev_idx = latest_idx
            .checked_sub(self.config.gap_lookback_days)
            .ok_or_else(|| anyhow::anyhow!("gap_lookback_days 超出数据范围"))?;

        let ma5_prev = self.calculate_ma_at(data, prev_idx, self.config.ma5_period)?;
        let ma10_prev = self.calculate_ma_at(data, prev_idx, self.config.ma10_period)?;
        let ma20_prev = self.calculate_ma_at(data, prev_idx, self.config.ma20_period)?;

        let gap_5_10_now = (ma5 - ma10) / ma10 * 100.0;
        let gap_10_20_now = (ma10 - ma20) / ma20 * 100.0;
        let gap_5_10_prev = (ma5_prev - ma10_prev) / ma10_prev * 100.0;
        let gap_10_20_prev = (ma10_prev - ma20_prev) / ma20_prev * 100.0;

        if !(gap_5_10_now > gap_5_10_prev && gap_10_20_now > gap_10_20_prev) {
            bail!(
                "均线未向上发散：gap5-10 {:.2}% -> {:.2}%，gap10-20 {:.2}% -> {:.2}%",
                gap_5_10_prev,
                gap_5_10_now,
                gap_10_20_prev,
                gap_10_20_now
            );
        }

        let above_ma5_days = self.count_consecutive_above_ma5(data, latest_idx)?;
        if above_ma5_days < self.config.min_above_ma5_days || above_ma5_days > self.config.max_above_ma5_days {
            bail!(
                "站上MA5天数不符合：{}（要求 {}-{}）",
                above_ma5_days,
                self.config.min_above_ma5_days,
                self.config.max_above_ma5_days
            );
        }

        let (volume_surge_days, latest_volume_ratio) = self.count_consecutive_volume_surge(data, latest_idx)?;
        if volume_surge_days < self.config.min_volume_surge_days {
            bail!(
                "放量天数不足：{}（要求 >= {}）",
                volume_surge_days,
                self.config.min_volume_surge_days
            );
        }

        let pct_chg = latest.pct_change.unwrap_or(0.0);

        let mut signal_strength = 70u8;
        if gap_5_10_now >= 1.0 && gap_10_20_now >= 1.0 {
            signal_strength = signal_strength.saturating_add(10);
        }
        if latest_volume_ratio >= self.config.volume_surge_ratio * 1.3 {
            signal_strength = signal_strength.saturating_add(10);
        }
        if volume_surge_days >= self.config.min_volume_surge_days + 1 {
            signal_strength = signal_strength.saturating_add(5);
        }
        if pct_chg >= 3.0 {
            signal_strength = signal_strength.saturating_add(5);
        }
        if signal_strength > 100 {
            signal_strength = 100;
        }

        let strategy_signal = if signal_strength >= 85 {
            StrategySignal::StrongBuy
        } else {
            StrategySignal::Buy
        };

        let risk_level = if latest_volume_ratio >= 3.0 { 4 } else { 3 };

        let analysis_description = format!(
            "MA5>{} MA10>{} MA20>{}，发散(gap5-10 {:.2}% -> {:.2}%，gap10-20 {:.2}% -> {:.2}%)，站上MA5 {}天，放量 {}天(量比{:.2})",
            format!("{:.2}", ma5),
            format!("{:.2}", ma10),
            format!("{:.2}", ma20),
            gap_5_10_prev,
            gap_5_10_now,
            gap_10_20_prev,
            gap_10_20_now,
            above_ma5_days,
            volume_surge_days,
            latest_volume_ratio
        );

        Ok(MaDivergenceVolumeResult {
            stock_code: symbol.to_string(),
            analysis_date,
            current_price: latest.close,
            pct_chg,
            ma5,
            ma10,
            ma20,
            gap_5_10_now,
            gap_10_20_now,
            gap_5_10_prev,
            gap_10_20_prev,
            above_ma5_days,
            volume_surge_days,
            latest_volume_ratio,
            strategy_signal,
            signal_strength,
            analysis_description,
            risk_level,
        })
    }
}

impl TradingStrategy for MaDivergenceVolumeStrategy {
    type Config = MaDivergenceVolumeConfig;

    fn name(&self) -> &str {
        "均线向上发散放量策略"
    }

    fn description(&self) -> &str {
        "均线多头且向上发散，K线连续站上MA5(>2<5天)，并出现连续放量"
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
        Ok(StrategyResult::MaDivergenceVolume(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::traits::{SecurityType, TimeFrame};

    #[test]
    fn test_ma_divergence_volume_ok() {
        let mut data = Vec::new();
        let base_date = 20240101u32;

        for i in 0..40 {
            let close = 10.0 + i as f64 * 0.05;
            let volume = if i >= 38 { 3000.0 } else { 1000.0 };
            data.push(SecurityData {
                symbol: "TEST".to_string(),
                trade_date: format!("{}", base_date + i),
                open: close,
                high: close,
                low: close,
                close,
                pre_close: None,
                change: None,
                pct_change: None,
                volume,
                amount: 0.0,
                turnover_rate: None,
                security_type: SecurityType::Stock,
                time_frame: TimeFrame::Daily,
                financial_data: None,
                target: None,
            });
        }

        let mut s = MaDivergenceVolumeStrategy::new(MaDivergenceVolumeConfig {
            gap_lookback_days: 3,
            min_above_ma5_days: 3,
            max_above_ma5_days: 4,
            volume_ma_period: 10,
            volume_surge_ratio: 1.5,
            min_volume_surge_days: 2,
            ..Default::default()
        });

        let r = s.analyze("TEST", &data);
        assert!(r.is_ok());
    }
}
