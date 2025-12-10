//! 技术指标计算模块

use crate::strategy::traits::SecurityData;
use anyhow::Result;

/// 技术指标计算器
pub struct TechnicalIndicators;

impl TechnicalIndicators {
    /// 计算简单移动平均线
    pub fn sma(data: &[f64], period: usize) -> Result<Vec<f64>> {
        if data.len() < period {
            return Err(anyhow::anyhow!("数据不足，需要至少 {} 个数据点", period));
        }

        let mut sma_values = Vec::new();
        for i in (period - 1)..data.len() {
            let sum: f64 = data[(i + 1 - period)..=i].iter().sum();
            sma_values.push(sum / period as f64);
        }
        Ok(sma_values)
    }

    /// 计算指数移动平均线
    pub fn ema(data: &[f64], period: usize) -> Result<Vec<f64>> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("数据为空"));
        }

        let alpha = 2.0 / (period as f64 + 1.0);
        let mut ema_values = Vec::new();
        ema_values.push(data[0]);

        for i in 1..data.len() {
            let ema = alpha * data[i] + (1.0 - alpha) * ema_values[i - 1];
            ema_values.push(ema);
        }
        Ok(ema_values)
    }

    /// 计算MACD指标
    pub fn macd(prices: &[f64], fast_period: usize, slow_period: usize, signal_period: usize) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>)> {
        if prices.len() < slow_period {
            return Err(anyhow::anyhow!("数据不足，需要至少 {} 个数据点", slow_period));
        }

        let ema_fast = Self::ema(prices, fast_period)?;
        let ema_slow = Self::ema(prices, slow_period)?;

        // 计算MACD线
        let mut macd_line = Vec::new();
        let start_idx = slow_period - fast_period;
        for i in start_idx..ema_fast.len() {
            macd_line.push(ema_fast[i] - ema_slow[i - start_idx]);
        }

        // 计算信号线
        let signal_line = Self::ema(&macd_line, signal_period)?;

        // 计算柱状图
        let mut histogram = Vec::new();
        let hist_start = signal_period - 1;
        for i in hist_start..macd_line.len() {
            histogram.push(macd_line[i] - signal_line[i - hist_start]);
        }

        Ok((macd_line, signal_line, histogram))
    }

    /// 计算RSI指标
    pub fn rsi(prices: &[f64], period: usize) -> Result<Vec<f64>> {
        if prices.len() < period + 1 {
            return Err(anyhow::anyhow!("数据不足，需要至少 {} 个数据点", period + 1));
        }

        let mut gains = Vec::new();
        let mut losses = Vec::new();

        // 计算价格变化
        for i in 1..prices.len() {
            let change = prices[i] - prices[i - 1];
            if change > 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }

        let mut rsi_values = Vec::new();
        
        // 计算第一个RSI值
        let initial_avg_gain: f64 = gains[..period].iter().sum::<f64>() / period as f64;
        let initial_avg_loss: f64 = losses[..period].iter().sum::<f64>() / period as f64;
        
        let mut avg_gain = initial_avg_gain;
        let mut avg_loss = initial_avg_loss;
        
        if avg_loss == 0.0 {
            rsi_values.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi_values.push(100.0 - (100.0 / (1.0 + rs)));
        }

        // 计算后续RSI值
        for i in period..gains.len() {
            avg_gain = (avg_gain * (period - 1) as f64 + gains[i]) / period as f64;
            avg_loss = (avg_loss * (period - 1) as f64 + losses[i]) / period as f64;
            
            if avg_loss == 0.0 {
                rsi_values.push(100.0);
            } else {
                let rs = avg_gain / avg_loss;
                rsi_values.push(100.0 - (100.0 / (1.0 + rs)));
            }
        }

        Ok(rsi_values)
    }

    /// 计算KDJ指标
    pub fn kdj(data: &[SecurityData], period: usize, k_period: usize, d_period: usize) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>)> {
        if data.len() < period {
            return Err(anyhow::anyhow!("数据不足，需要至少 {} 个数据点", period));
        }

        let mut rsv_values = Vec::new();
        
        // 计算RSV值
        for i in (period - 1)..data.len() {
            let window = &data[(i + 1 - period)..=i];
            let highest = window.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);
            let lowest = window.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
            let close = data[i].close;
            
            let rsv = if highest == lowest {
                50.0 // 避免除零
            } else {
                (close - lowest) / (highest - lowest) * 100.0
            };
            rsv_values.push(rsv);
        }

        // 计算K值
        let mut k_values = Vec::new();
        k_values.push(50.0); // 初始K值
        
        for i in 0..rsv_values.len() {
            let k = if i == 0 {
                (rsv_values[i] + 2.0 * k_values[0]) / 3.0
            } else {
                (rsv_values[i] + 2.0 * k_values[i]) / 3.0
            };
            if i > 0 {
                k_values.push(k);
            } else {
                k_values[0] = k;
            }
        }

        // 计算D值
        let mut d_values = Vec::new();
        d_values.push(50.0); // 初始D值
        
        for i in 0..k_values.len() {
            let d = if i == 0 {
                (k_values[i] + 2.0 * d_values[0]) / 3.0
            } else {
                (k_values[i] + 2.0 * d_values[i]) / 3.0
            };
            if i > 0 {
                d_values.push(d);
            } else {
                d_values[0] = d;
            }
        }

        // 计算J值
        let mut j_values = Vec::new();
        for i in 0..k_values.len() {
            let j = 3.0 * k_values[i] - 2.0 * d_values[i];
            j_values.push(j);
        }

        Ok((k_values, d_values, j_values))
    }

    /// 计算成交量移动平均线
    pub fn volume_ma(volumes: &[f64], period: usize) -> Result<Vec<f64>> {
        Self::sma(volumes, period)
    }

    /// 计算换手率平均值
    pub fn turnover_rate_avg(turnover_rates: &[Option<f64>], period: usize) -> Result<f64> {
        let valid_rates: Vec<f64> = turnover_rates
            .iter()
            .filter_map(|&rate| rate)
            .collect();
            
        if valid_rates.len() < period {
            return Err(anyhow::anyhow!("有效换手率数据不足"));
        }

        let recent_rates = &valid_rates[valid_rates.len().saturating_sub(period)..];
        Ok(recent_rates.iter().sum::<f64>() / recent_rates.len() as f64)
    }

    /// 计算价格波动率
    pub fn price_volatility(prices: &[f64], period: usize) -> Result<f64> {
        if prices.len() < period {
            return Err(anyhow::anyhow!("数据不足"));
        }

        let recent_prices = &prices[prices.len() - period..];
        let mean = recent_prices.iter().sum::<f64>() / recent_prices.len() as f64;
        
        let variance = recent_prices
            .iter()
            .map(|&price| (price - mean).powi(2))
            .sum::<f64>() / recent_prices.len() as f64;
            
        Ok(variance.sqrt())
    }
}
