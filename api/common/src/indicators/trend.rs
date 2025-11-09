//! Trend indicators module
//! 
//! This module contains trend-following indicators that help identify the direction of the market.

use super::{Indicator, IndicatorResult, IndicatorError};
use std::collections::VecDeque;

/// Simple Moving Average (SMA)
///
/// The average of the last n values in a data series.
#[derive(Debug, Clone)]
pub struct SMA {
    period: usize,
    values: VecDeque<f64>,
    sum: f64,
}

impl SMA {
    /// Creates a new SMA indicator with the given period
    pub fn new(period: usize) -> IndicatorResult<Self> {
        if period == 0 {
            return Err(IndicatorError::InvalidParameter("Period must be greater than 0".to_string()));
        }
        
        Ok(Self {
            period,
            values: VecDeque::with_capacity(period + 1),
            sum: 0.0,
        })
    }
    
    /// Batch calculation for historical data analysis
    pub fn calculate_batch(period: usize, prices: &[f64]) -> IndicatorResult<Vec<f64>> {
        if period == 0 || period > prices.len() {
            return Err(IndicatorError::InvalidParameter("Invalid period or insufficient data".to_string()));
        }
        
        let mut results = Vec::new();
        let mut sma = Self::new(period)?;
        
        for &price in prices {
            match sma.update(price) {
                Ok(value) => results.push(value),
                Err(IndicatorError::NotEnoughData) => continue,
                Err(e) => return Err(e),
            }
        }
        
        Ok(results)
    }
    
    /// Get current SMA value without updating
    pub fn current_value(&self) -> Option<f64> {
        if self.values.len() >= self.period {
            Some(self.sum / self.period as f64)
        } else {
            None
        }
    }
}

impl Indicator for SMA {
    type Input = f64;
    type Output = f64;
    
    fn update(&mut self, input: Self::Input) -> IndicatorResult<Self::Output> {
        self.values.push_back(input);
        self.sum += input;
        
        if self.values.len() > self.period {
            if let Some(removed) = self.values.pop_front() {
                self.sum -= removed;
            }
        }
        
        if self.values.len() < self.period {
            return Err(IndicatorError::NotEnoughData);
        }
        
        Ok(self.sum / self.period as f64)
    }
    
    fn reset(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }
}

/// Exponential Moving Average (EMA)
///
/// A type of moving average that places a greater weight on recent data points.
#[derive(Debug, Clone)]
pub struct EMA {
    period: usize,
    multiplier: f64,
    current: Option<f64>,
    initialized: bool,
}

impl EMA {
    /// Creates a new EMA indicator with the given period
    pub fn new(period: usize) -> IndicatorResult<Self> {
        if period < 2 {
            return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
        }
        
        Ok(Self {
            period,
            multiplier: 2.0 / (period as f64 + 1.0),
            current: None,
            initialized: false,
        })
    }
    
    /// Get current EMA value without updating
    pub fn current(&self) -> Option<f64> {
        self.current
    }
}

impl Indicator for EMA {
    type Input = f64;
    type Output = f64;
    
    fn update(&mut self, input: Self::Input) -> IndicatorResult<Self::Output> {
        if !self.initialized {
            self.current = Some(input);
            self.initialized = true;
        } else {
            let prev = self.current.unwrap_or(input);
            self.current = Some((input - prev) * self.multiplier + prev);
        }
        
        self.current.ok_or(IndicatorError::NotEnoughData)
    }
    
    fn reset(&mut self) {
        self.current = None;
        self.initialized = false;
    }
}

/// Parabolic SAR (Stop and Reverse)
///
/// A technical analysis indicator used to determine the price direction and potential reversals.
#[derive(Debug, Clone)]
pub struct SAR {
    acceleration: f64,
    max_acceleration: f64,
    step: f64,
    high: f64,
    low: f64,
    trend: i8, // 1 for uptrend, -1 for downtrend
    sar: Option<f64>,
    extreme_point: f64,
}

impl SAR {
    /// Creates a new SAR indicator with the given parameters
    pub fn new(acceleration: f64, max_acceleration: f64, step: f64) -> IndicatorResult<Self> {
        if acceleration <= 0.0 || max_acceleration <= 0.0 || step <= 0.0 {
            return Err(IndicatorError::InvalidParameter(
                "Acceleration, max_acceleration, and step must be greater than 0".to_string(),
            ));
        }
        
        Ok(Self {
            acceleration,
            max_acceleration,
            step,
            high: 0.0,
            low: 0.0,
            trend: 1, // Start with uptrend by default
            sar: None,
            extreme_point: 0.0,
        })
    }
}

impl Indicator for SAR {
    type Input = (f64, f64); // (high, low)
    type Output = f64;
    
    fn update(&mut self, (high, low): Self::Input) -> IndicatorResult<Self::Output> {
        if self.sar.is_none() {
            // Initialize on first update
            self.high = high;
            self.low = low;
            self.extreme_point = self.high;
            self.sar = Some(self.low - self.acceleration * (self.high - self.low));
            return self.sar.ok_or(IndicatorError::NotEnoughData);
        }
        
        let mut sar = self.sar.unwrap();
        
        // Update SAR
        sar = sar + self.acceleration * (self.extreme_point - sar);
        
        // Check for trend reversal
        if (self.trend > 0 && low < sar) || (self.trend < 0 && high > sar) {
            self.trend *= -1;
            sar = self.extreme_point;
            self.acceleration = 0.02;
            self.extreme_point = if self.trend > 0 { high } else { low };
        }
        
        // Update extreme point and acceleration
        if self.trend > 0 {
            if high > self.extreme_point {
                self.extreme_point = high;
                self.acceleration = (self.acceleration + self.step).min(self.max_acceleration);
            }
            // Ensure SAR is below the previous two lows
            if let Some(prev_low) = Some(low) {
                sar = sar.min(prev_low).min(low);
            }
        } else {
            if low < self.extreme_point {
                self.extreme_point = low;
                self.acceleration = (self.acceleration + self.step).min(self.max_acceleration);
            }
            // Ensure SAR is above the previous two highs
            if let Some(prev_high) = Some(high) {
                sar = sar.max(prev_high).max(high);
            }
        }
        
        self.sar = Some(sar);
        self.high = high;
        self.low = low;
        
        self.sar.ok_or(IndicatorError::NotEnoughData)
    }
    
    fn reset(&mut self) {
        self.high = 0.0;
        self.low = 0.0;
        self.trend = 1;
        self.sar = None;
        self.extreme_point = 0.0;
        self.acceleration = 0.02;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_sma() {
        let mut sma = SMA::new(3).unwrap();
        assert!(sma.update(1.0).is_err()); // Not enough data
        assert!(sma.update(2.0).is_err()); // Still not enough data
        assert_relative_eq!(sma.update(3.0).unwrap(), 2.0); // (1+2+3)/3
        assert_relative_eq!(sma.update(4.0).unwrap(), 3.0); // (2+3+4)/3
        sma.reset();
        assert!(sma.update(1.0).is_err());
    }
    
    #[test]
    fn test_ema() {
        let mut ema = EMA::new(3).unwrap();
        let multiplier = 2.0 / (3.0 + 1.0);
        
        let first = 1.0;
        assert_relative_eq!(ema.update(first).unwrap(), first);
        
        let second = 2.0;
        let expected = second * multiplier + first * (1.0 - multiplier);
        assert_relative_eq!(ema.update(second).unwrap(), expected);
        
        let third = 3.0;
        let expected = third * multiplier + expected * (1.0 - multiplier);
        assert_relative_eq!(ema.update(third).unwrap(), expected);
    }
}
