//! Momentum indicators module
//! 
//! This module contains momentum indicators that help identify the speed of price movements.

use super::{Indicator, IndicatorResult, IndicatorError};
use std::collections::VecDeque;

/// Relative Strength Index (RSI)
///
/// A momentum oscillator that measures the speed and change of price movements.
#[derive(Debug, Clone)]
pub struct RSI {
    period: usize,
    prices: VecDeque<f64>,
    avg_gain: f64,
    avg_loss: f64,
    prev_price: Option<f64>,
}

impl RSI {
    /// Creates a new RSI indicator with the given period
    pub fn new(period: usize) -> IndicatorResult<Self> {
        if period < 2 {
            return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
        }
        
        Ok(Self {
            period,
            prices: VecDeque::with_capacity(period + 1),
            avg_gain: 0.0,
            avg_loss: 0.0,
            prev_price: None,
        })
    }
}

impl Indicator for RSI {
    type Input = f64;
    type Output = f64;
    
    fn update(&mut self, price: Self::Input) -> IndicatorResult<Self::Output> {
        if let Some(prev) = self.prev_price {
            let change = price - prev;
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };
            
            if self.prices.len() < self.period {
                self.avg_gain = (self.avg_gain * (self.prices.len() as f64) + gain) / (self.prices.len() as f64 + 1.0);
                self.avg_loss = (self.avg_loss * (self.prices.len() as f64) + loss) / (self.prices.len() as f64 + 1.0);
            } else {
                self.avg_gain = (self.avg_gain * (self.period - 1) as f64 + gain) / self.period as f64;
                self.avg_loss = (self.avg_loss * (self.period - 1) as f64 + loss) / self.period as f64;
            }
            
            self.prices.push_back(price);
            if self.prices.len() > self.period {
                self.prices.pop_front();
            }
        }
        
        self.prev_price = Some(price);
        
        if self.prices.len() < self.period {
            return Err(IndicatorError::NotEnoughData);
        }
        
        if self.avg_loss.abs() < f64::EPSILON {
            return Ok(100.0);
        }
        
        let rs = self.avg_gain / self.avg_loss;
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        
        Ok(rsi)
    }
    
    fn reset(&mut self) {
        self.prices.clear();
        self.avg_gain = 0.0;
        self.avg_loss = 0.0;
        self.prev_price = None;
    }
}

/// Moving Average Convergence Divergence (MACD)
///
/// A trend-following momentum indicator that shows the relationship between two moving averages.
#[derive(Debug, Clone)]
pub struct MACD {
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
    initialized: bool,
}

impl MACD {
    /// Creates a new MACD indicator with the given parameters
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> IndicatorResult<Self> {
        if fast_period >= slow_period {
            return Err(IndicatorError::InvalidParameter(
                "Fast period must be less than slow period".to_string(),
            ));
        }
        
        Ok(Self {
            fast_ema: EMA::new(fast_period)?,
            slow_ema: EMA::new(slow_period)?,
            signal_ema: EMA::new(signal_period)?,
            initialized: false,
        })
    }
    
    /// Returns the current MACD line, signal line, and histogram values
    pub fn value(&self) -> Option<(f64, f64, f64)> {
        if !self.initialized {
            return None;
        }
        
        let macd_line = self.fast_ema.current()? - self.slow_ema.current()?;
        let signal_line = self.signal_ema.current()?;
        let histogram = macd_line - signal_line;
        
        Some((macd_line, signal_line, histogram))
    }
}

impl Indicator for MACD {
    type Input = f64;
    type Output = (f64, f64, f64); // (MACD line, Signal line, Histogram)
    
    fn update(&mut self, price: Self::Input) -> IndicatorResult<Self::Output> {
        let fast_ema = self.fast_ema.update(price)?;
        let slow_ema = self.slow_ema.update(price)?;
        
        let macd_line = fast_ema - slow_ema;
        let signal_line = self.signal_ema.update(macd_line)?;
        
        self.initialized = true;
        
        Ok((macd_line, signal_line, macd_line - signal_line))
    }
    
    fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
        self.initialized = false;
    }
}

/// KDJ Indicator (Stochastic Oscillator)
/// 
/// A momentum indicator that uses support and resistance levels.
#[derive(Debug, Clone)]
pub struct KDJ {
    k_period: usize,
    d_period: usize,
    j_period: usize,
    high_prices: VecDeque<f64>,
    low_prices: VecDeque<f64>,
    close_prices: VecDeque<f64>,
    k_values: VecDeque<f64>,
    d_values: VecDeque<f64>,
}

impl KDJ {
    /// Creates a new KDJ indicator with the given periods
    pub fn new(k_period: usize, d_period: usize, j_period: usize) -> IndicatorResult<Self> {
        if k_period == 0 || d_period == 0 || j_period == 0 {
            return Err(IndicatorError::InvalidParameter(
                "Periods must be greater than 0".to_string(),
            ));
        }
        
        Ok(Self {
            k_period,
            d_period,
            j_period,
            high_prices: VecDeque::with_capacity(k_period + 1),
            low_prices: VecDeque::with_capacity(k_period + 1),
            close_prices: VecDeque::with_capacity(k_period + 1),
            k_values: VecDeque::with_capacity(d_period + 1),
            d_values: VecDeque::with_capacity(j_period + 1),
        })
    }
    
    fn calculate_k(&self) -> Option<f64> {
        if self.high_prices.len() < self.k_period {
            return None;
        }
        
        let highest_high = self.high_prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest_low = self.low_prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        if (highest_high - lowest_low).abs() < f64::EPSILON {
            return Some(50.0); // Neutral value when no range
        }
        
        let last_close = *self.close_prices.back()?;
        let k = 100.0 * (last_close - lowest_low) / (highest_high - lowest_low);
        
        Some(k)
    }
    
    fn calculate_d(&self) -> Option<f64> {
        if self.k_values.len() < self.d_period {
            return None;
        }
        
        let sum: f64 = self.k_values.iter().sum();
        Some(sum / self.k_values.len() as f64)
    }
    
    fn calculate_j(&self, k: f64, d: f64) -> f64 {
        3.0 * k - 2.0 * d
    }
}

impl Indicator for KDJ {
    type Input = (f64, f64, f64); // (high, low, close)
    type Output = (f64, f64, f64); // (K, D, J)
    
    fn update(&mut self, (high, low, close): Self::Input) -> IndicatorResult<Self::Output> {
        self.high_prices.push_back(high);
        self.low_prices.push_back(low);
        self.close_prices.push_back(close);
        
        if self.high_prices.len() > self.k_period {
            self.high_prices.pop_front();
            self.low_prices.pop_front();
            self.close_prices.pop_front();
        }
        
        let k = match self.calculate_k() {
            Some(k) => k,
            None => return Err(IndicatorError::NotEnoughData),
        };
        
        self.k_values.push_back(k);
        if self.k_values.len() > self.d_period {
            self.k_values.pop_front();
        }
        
        let d = match self.calculate_d() {
            Some(d) => d,
            None => return Err(IndicatorError::NotEnoughData),
        };
        
        self.d_values.push_back(d);
        if self.d_values.len() > self.j_period {
            self.d_values.pop_front();
        }
        
        let j = self.calculate_j(k, d);
        
        Ok((k, d, j))
    }
    
    fn reset(&mut self) {
        self.high_prices.clear();
        self.low_prices.clear();
        self.close_prices.clear();
        self.k_values.clear();
        self.d_values.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_rsi() {
        let mut rsi = RSI::new(14).unwrap();
        
        // Test with increasing prices (should approach 100)
        for i in 1..30 {
            let price = i as f64;
            let result = rsi.update(price);
            
            if i > 14 { // Need more than 14 data points for RSI calculation
                let rsi_value = result.unwrap();
                assert!(rsi_value > 50.0 && rsi_value <= 100.0);
            } else {
                assert!(result.is_err());
            }
        }
        
        rsi.reset();
        
        // Test with decreasing prices (should approach 0)
        for i in (1..30).rev() {
            let price = i as f64;
            let result = rsi.update(price);
            
            if (30 - i) > 14 { // Need more than 14 data points
                let rsi_value = result.unwrap();
                assert!(rsi_value >= 0.0 && rsi_value < 50.0);
            } else {
                assert!(result.is_err());
            }
        }
    }
    
    #[test]
    fn test_macd() {
        let mut macd = MACD::new(12, 26, 9).unwrap();
        
        // Test with increasing prices
        for i in 1..50 {
            let price = i as f64;
            let (macd_line, signal_line, _) = macd.update(price).unwrap();
            
            if i >= 26 { // Need at least slow period to get valid values
                assert!(macd_line > signal_line); // In an uptrend, MACD should be above signal
            }
        }
    }
    
    #[test]
    fn test_kdj() {
        let mut kdj = KDJ::new(9, 3, 3).unwrap();
        
        // Test with increasing prices
        for i in 1..20 {
            let high = i as f64 + 1.0;
            let low = i as f64 - 1.0;
            let close = i as f64;
            
            let result = kdj.update((high, low, close));
            
            // Need K period (9) data points to calculate K, then D period (3) K values to calculate D
            // So total needed is 9 + 3 - 1 = 11 data points
            if i >= 11 { 
                let (k, d, _j) = result.unwrap();
                assert!(k >= 0.0 && k <= 100.0);
                assert!(d >= 0.0 && d <= 100.0);
                // J can go above 100 or below 0
            } else {
                assert!(result.is_err());
            }
        }
    }
}

// Import EMA from the trend module
use super::trend::EMA;

