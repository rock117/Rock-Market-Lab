//! Technical indicators module for financial analysis
//!
//! This module provides various technical indicators commonly used in financial analysis.
//! The indicators are organized into categories:
//! - Trend indicators (MA, EMA, SAR)
//! - Momentum indicators (RSI, MACD, KDJ, WR, CCI, STOCH)
//! - Volatility indicators (ATR, BOLL)
//! - Volume indicators (OBV)

pub mod trend;
pub mod momentum;
pub mod volatility;
pub mod volume;
pub mod examples;

/// Common error type for technical indicators
#[derive(Debug, thiserror::Error)]
pub enum IndicatorError {
    #[error("Not enough data points")]
    NotEnoughData,
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Common result type for technical indicators
pub type IndicatorResult<T> = Result<T, IndicatorError>;

/// Common trait for all technical indicators
pub trait Indicator {
    /// The input type for the indicator
    type Input;
    
    /// The output type of the indicator
    type Output;
    
    /// Updates the indicator with a new data point
    fn update(&mut self, input: Self::Input) -> IndicatorResult<Self::Output>;
    
    /// Resets the indicator to its initial state
    fn reset(&mut self);
}

// Re-export commonly used types for convenience
pub use trend::{SMA, EMA, SAR};
pub use momentum::{RSI, MACD, KDJ};
pub use volatility::{ATR, BollingerBands};
pub use volume::OBV;

/// Convenience functions for quick indicator calculations
/// These functions provide a simple API for common use cases

/// Calculate Simple Moving Average for a price series
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - Moving average period
/// 
/// # Returns
/// Vector of SMA values (starts from index `period-1`)
/// 
/// # Example
/// ```
/// use common::indicators::sma;
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let sma_values = sma(&prices, 3).unwrap();
/// assert_eq!(sma_values, vec![2.0, 3.0, 4.0]);
/// ```
pub fn sma(prices: &[f64], period: usize) -> IndicatorResult<Vec<f64>> {
    SMA::calculate_batch(period, prices)
}

/// Calculate Moving Average (alias for sma)
/// 
/// MA (Moving Average) 通常指简单移动平均线 (Simple Moving Average)
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - Moving average period
/// 
/// # Example
/// ```
/// use common::indicators::ma;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let ma_values = ma(&prices, 3).unwrap();
/// // 计算结果: [(10+11+12)/3, (11+12+13)/3, (12+13+14)/3] = [11.0, 12.0, 13.0]
/// ```
pub fn ma(prices: &[f64], period: usize) -> IndicatorResult<Vec<f64>> {
    sma(prices, period)
}

/// Calculate Exponential Moving Average for a price series
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - EMA period
/// 
/// # Example
/// ```
/// use common::indicators::ema;
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let ema_values = ema(&prices, 3).unwrap();
/// ```
pub fn ema(prices: &[f64], period: usize) -> IndicatorResult<Vec<f64>> {
    if period < 2 {
        return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
    }
    
    let mut ema_indicator = EMA::new(period)?;
    let mut results = Vec::new();
    
    for &price in prices {
        match ema_indicator.update(price) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate RSI (Relative Strength Index) for a price series
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - RSI period (typically 14)
/// 
/// # Example
/// ```
/// use common::indicators::rsi;
/// let prices = vec![44.0, 44.25, 44.5, 43.75, 44.5, 45.0, 45.25, 45.5];
/// let rsi_values = rsi(&prices, 6).unwrap();
/// ```
pub fn rsi(prices: &[f64], period: usize) -> IndicatorResult<Vec<f64>> {
    if period < 2 {
        return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
    }
    
    let mut rsi_indicator = RSI::new(period)?;
    let mut results = Vec::new();
    
    for &price in prices {
        match rsi_indicator.update(price) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate MACD (Moving Average Convergence Divergence)
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `fast_period` - Fast EMA period (typically 12)
/// * `slow_period` - Slow EMA period (typically 26)
/// * `signal_period` - Signal line EMA period (typically 9)
/// 
/// # Returns
/// Vector of (MACD line, Signal line, Histogram) tuples
/// 
/// # Example
/// ```
/// use common::indicators::macd;
/// let prices = vec![/* price data */];
/// let macd_values = macd(&prices, 12, 26, 9).unwrap();
/// for (macd_line, signal_line, histogram) in macd_values {
///     println!("MACD: {:.2}, Signal: {:.2}, Histogram: {:.2}", 
///              macd_line, signal_line, histogram);
/// }
/// ```
pub fn macd(prices: &[f64], fast_period: usize, slow_period: usize, signal_period: usize) 
    -> IndicatorResult<Vec<(f64, f64, f64)>> {
    let mut macd_indicator = MACD::new(fast_period, slow_period, signal_period)?;
    let mut results = Vec::new();
    
    for &price in prices {
        match macd_indicator.update(price) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate Parabolic SAR
/// 
/// # Arguments
/// * `highs` - High price data slice
/// * `lows` - Low price data slice
/// * `acceleration` - Initial acceleration factor (typically 0.02)
/// * `max_acceleration` - Maximum acceleration factor (typically 0.2)
/// 
/// # Example
/// ```
/// use common::indicators::sar;
/// let highs = vec![10.5, 11.0, 11.2, 10.8, 11.5];
/// let lows = vec![10.0, 10.3, 10.8, 10.2, 10.9];
/// let sar_values = sar(&highs, &lows, 0.02, 0.2).unwrap();
/// ```
pub fn sar(highs: &[f64], lows: &[f64], acceleration: f64, max_acceleration: f64) 
    -> IndicatorResult<Vec<f64>> {
    if highs.len() != lows.len() {
        return Err(IndicatorError::InvalidParameter("Highs and lows must have same length".to_string()));
    }
    
    let mut sar_indicator = SAR::new(acceleration, max_acceleration, 0.02)?;
    let mut results = Vec::new();
    
    for (&high, &low) in highs.iter().zip(lows.iter()) {
        match sar_indicator.update((high, low)) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate ATR (Average True Range)
/// 
/// # Arguments
/// * `highs` - High price data slice
/// * `lows` - Low price data slice
/// * `closes` - Close price data slice
/// * `period` - ATR period (typically 14)
/// 
/// # Example
/// ```
/// use common::indicators::atr;
/// let highs = vec![10.5, 11.0, 11.2, 10.8, 11.5];
/// let lows = vec![10.0, 10.3, 10.8, 10.2, 10.9];
/// let closes = vec![10.2, 10.8, 11.0, 10.5, 11.2];
/// let atr_values = atr(&highs, &lows, &closes, 3).unwrap();
/// ```
pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) 
    -> IndicatorResult<Vec<f64>> {
    if highs.len() != lows.len() || highs.len() != closes.len() {
        return Err(IndicatorError::InvalidParameter("All price arrays must have same length".to_string()));
    }
    
    let mut atr_indicator = ATR::new(period)?;
    let mut results = Vec::new();
    
    for ((&high, &low), &close) in highs.iter().zip(lows.iter()).zip(closes.iter()) {
        match atr_indicator.update((high, low, close)) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate Bollinger Bands
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - Moving average period (typically 20)
/// * `std_dev` - Number of standard deviations (typically 2.0)
/// 
/// # Returns
/// Vector of (middle_band, upper_band, lower_band, %b, bandwidth) tuples
/// 
/// # Example
/// ```
/// use common::indicators::bollinger_bands;
/// let prices = vec![/* price data */];
/// let bb_values = bollinger_bands(&prices, 20, 2.0).unwrap();
/// for (middle, upper, lower, percent_b, bandwidth) in bb_values {
///     println!("BB: {:.2}/{:.2}/{:.2}, %b: {:.2}", upper, middle, lower, percent_b);
/// }
/// ```
pub fn bollinger_bands(prices: &[f64], period: usize, std_dev: f64) 
    -> IndicatorResult<Vec<(f64, f64, f64, f64, f64)>> {
    let mut bb_indicator = BollingerBands::new(period, std_dev)?;
    let mut results = Vec::new();
    
    for &price in prices {
        match bb_indicator.update(price) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate Bollinger Bands (alias for bollinger_bands)
/// 
/// # Arguments
/// * `prices` - Price data slice
/// * `period` - Moving average period (typically 20)
/// * `std_dev` - Number of standard deviations (typically 2.0)
/// 
/// # Example
/// ```
/// use common::indicators::boll;
/// let prices = vec![20.0, 21.0, 22.0, 21.5, 23.0, 22.8, 24.0];
/// let boll_values = boll(&prices, 5, 2.0).unwrap();
/// for (middle, upper, lower, percent_b, bandwidth) in boll_values {
///     println!("BOLL: 上轨={:.2}, 中轨={:.2}, 下轨={:.2}", upper, middle, lower);
/// }
/// ```
pub fn boll(prices: &[f64], period: usize, std_dev: f64) 
    -> IndicatorResult<Vec<(f64, f64, f64, f64, f64)>> {
    bollinger_bands(prices, period, std_dev)
}

/// Calculate KDJ indicator
/// 
/// # Arguments
/// * `highs` - High price data slice
/// * `lows` - Low price data slice
/// * `closes` - Close price data slice
/// * `k_period` - K period (typically 9)
/// * `d_period` - D period (typically 3)
/// * `j_period` - J period (typically 3)
/// 
/// # Returns
/// Vector of (K, D, J) tuples
/// 
/// # Example
/// ```
/// use common::indicators::kdj;
/// let highs = vec![/* high data */];
/// let lows = vec![/* low data */];
/// let closes = vec![/* close data */];
/// let kdj_values = kdj(&highs, &lows, &closes, 9, 3, 3).unwrap();
/// ```
pub fn kdj(highs: &[f64], lows: &[f64], closes: &[f64], k_period: usize, d_period: usize, j_period: usize) 
    -> IndicatorResult<Vec<(f64, f64, f64)>> {
    if highs.len() != lows.len() || highs.len() != closes.len() {
        return Err(IndicatorError::InvalidParameter("All price arrays must have same length".to_string()));
    }
    
    let mut kdj_indicator = KDJ::new(k_period, d_period, j_period)?;
    let mut results = Vec::new();
    
    for ((&high, &low), &close) in highs.iter().zip(lows.iter()).zip(closes.iter()) {
        match kdj_indicator.update((high, low, close)) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Calculate OBV (On-Balance Volume)
/// 
/// # Arguments
/// * `closes` - Close price data slice
/// * `volumes` - Volume data slice
/// 
/// # Example
/// ```
/// use common::indicators::obv;
/// let closes = vec![10.0, 10.5, 10.2, 10.8, 11.0];
/// let volumes = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
/// let obv_values = obv(&closes, &volumes).unwrap();
/// ```
pub fn obv(closes: &[f64], volumes: &[f64]) -> IndicatorResult<Vec<f64>> {
    if closes.len() != volumes.len() {
        return Err(IndicatorError::InvalidParameter("Closes and volumes must have same length".to_string()));
    }
    
    let mut obv_indicator = OBV::new();
    let mut results = Vec::new();
    
    for (&close, &volume) in closes.iter().zip(volumes.iter()) {
        match obv_indicator.update((close, volume)) {
            Ok(value) => results.push(value),
            Err(IndicatorError::NotEnoughData) => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}

/// Builder pattern for creating indicator combinations
/// 
/// # Example
/// ```
/// use common::indicators::IndicatorBuilder;
///
/// let mut builder = IndicatorBuilder::new();
/// builder.add_sma(20)
///        .add_rsi(14)
///        .add_macd(12, 26, 9);
///
/// // Process data
/// for price in prices {
///     let results = builder.update(price);
///     // Use results...
/// }
/// ```
pub struct IndicatorBuilder {
    sma_indicators: Vec<(String, SMA)>,
    ema_indicators: Vec<(String, EMA)>,
    rsi_indicators: Vec<(String, RSI)>,
    // Add more as needed
}

impl IndicatorBuilder {
    pub fn new() -> Self {
        Self {
            sma_indicators: Vec::new(),
            ema_indicators: Vec::new(),
            rsi_indicators: Vec::new(),
        }
    }
    
    pub fn add_sma(&mut self, period: usize) -> &mut Self {
        let name = format!("SMA_{}", period);
        if let Ok(sma) = SMA::new(period) {
            self.sma_indicators.push((name, sma));
        }
        self
    }
    
    pub fn add_ema(&mut self, period: usize) -> &mut Self {
        let name = format!("EMA_{}", period);
        if let Ok(ema) = EMA::new(period) {
            self.ema_indicators.push((name, ema));
        }
        self
    }
    
    pub fn add_rsi(&mut self, period: usize) -> &mut Self {
        let name = format!("RSI_{}", period);
        if let Ok(rsi) = RSI::new(period) {
            self.rsi_indicators.push((name, rsi));
        }
        self
    }
    
    /// Update all indicators with new price data
    pub fn update(&mut self, price: f64) -> std::collections::HashMap<String, f64> {
        let mut results = std::collections::HashMap::new();
        
        for (name, indicator) in &mut self.sma_indicators {
            if let Ok(value) = indicator.update(price) {
                results.insert(name.clone(), value);
            }
        }
        
        for (name, indicator) in &mut self.ema_indicators {
            if let Ok(value) = indicator.update(price) {
                results.insert(name.clone(), value);
            }
        }
        
        for (name, indicator) in &mut self.rsi_indicators {
            if let Ok(value) = indicator.update(price) {
                results.insert(name.clone(), value);
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_convenience_functions() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        
        // Test SMA
        let sma_values = sma(&prices, 3).unwrap();
        assert_eq!(sma_values.len(), 8); // 10 - 3 + 1
        assert_relative_eq!(sma_values[0], 2.0); // (1+2+3)/3
        
        // Test EMA
        let ema_values = ema(&prices, 3).unwrap();
        assert_eq!(ema_values.len(), 10);
        
        // Test RSI
        let rsi_values = rsi(&prices, 3).unwrap();
        assert!(!rsi_values.is_empty());
    }
    
    #[test]
    fn test_indicator_builder() {
        let mut builder = IndicatorBuilder::new();
        builder.add_sma(3).add_ema(3).add_rsi(3);
        
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mut all_results = Vec::new();
        
        for price in prices {
            let results = builder.update(price);
            all_results.push(results);
        }
        
        // Should have results for the last few updates
        assert!(!all_results.is_empty());
    }
}
