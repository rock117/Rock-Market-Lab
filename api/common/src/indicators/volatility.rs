//! Volatility indicators module
//! 
//! This module contains indicators that measure the rate of price movements.

use super::{Indicator, IndicatorResult, IndicatorError};
use std::collections::VecDeque;

/// Average True Range (ATR)
///
/// A measure of market volatility that shows the average range of price movement.
#[derive(Debug, Clone)]
pub struct ATR {
    period: usize,
    previous_close: Option<f64>,
    true_ranges: VecDeque<f64>,
    sum_true_ranges: f64,
}

impl ATR {
    /// Creates a new ATR indicator with the given period
    pub fn new(period: usize) -> IndicatorResult<Self> {
        if period < 2 {
            return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
        }
        
        Ok(Self {
            period,
            previous_close: None,
            true_ranges: VecDeque::with_capacity(period + 1),
            sum_true_ranges: 0.0,
        })
    }
    
    fn calculate_true_range(&self, high: f64, low: f64, previous_close: f64) -> f64 {
        let range1 = high - low;
        let range2 = (high - previous_close).abs();
        let range3 = (low - previous_close).abs();
        
        range1.max(range2).max(range3)
    }
}

impl Indicator for ATR {
    type Input = (f64, f64, f64); // (high, low, close)
    type Output = f64;
    
    fn update(&mut self, (high, low, close): Self::Input) -> IndicatorResult<Self::Output> {
        if let Some(prev_close) = self.previous_close {
            let true_range = self.calculate_true_range(high, low, prev_close);
            
            self.true_ranges.push_back(true_range);
            self.sum_true_ranges += true_range;
            
            if self.true_ranges.len() > self.period {
                if let Some(oldest) = self.true_ranges.pop_front() {
                    self.sum_true_ranges -= oldest;
                }
            }
        }
        
        self.previous_close = Some(close);
        
        if self.true_ranges.len() < self.period {
            return Err(IndicatorError::NotEnoughData);
        }
        
        Ok(self.sum_true_ranges / self.true_ranges.len() as f64)
    }
    
    fn reset(&mut self) {
        self.previous_close = None;
        self.true_ranges.clear();
        self.sum_true_ranges = 0.0;
    }
}

/// Bollinger Bands
///
/// A volatility indicator that consists of a middle band (SMA) with two outer bands.
#[derive(Debug, Clone)]
pub struct BollingerBands {
    period: usize,
    num_std_dev: f64,
    prices: VecDeque<f64>,
}

impl BollingerBands {
    /// Creates a new Bollinger Bands indicator with the given parameters
    pub fn new(period: usize, num_std_dev: f64) -> IndicatorResult<Self> {
        if period < 2 {
            return Err(IndicatorError::InvalidParameter("Period must be at least 2".to_string()));
        }
        
        if num_std_dev <= 0.0 {
            return Err(IndicatorError::InvalidParameter(
                "Number of standard deviations must be greater than 0".to_string(),
            ));
        }
        
        Ok(Self {
            period,
            num_std_dev,
            prices: VecDeque::with_capacity(period + 1),
        })
    }
    
    /// Returns (middle_band, upper_band, lower_band, %b, bandwidth)
    pub fn value(&self, price: f64) -> Option<(f64, f64, f64, f64, f64)> {
        if self.prices.len() < self.period {
            return None;
        }
        
        // Calculate middle band (SMA)
        let middle_band = self.prices.iter().sum::<f64>() / self.prices.len() as f64;
        
        // Calculate standard deviation
        let variance = self.prices
            .iter()
            .map(|&p| (p - middle_band).powi(2))
            .sum::<f64>() / self.prices.len() as f64;
        let std_dev = variance.sqrt();
        
        let upper_band = middle_band + (std_dev * self.num_std_dev);
        let lower_band = (middle_band - (std_dev * self.num_std_dev)).max(0.0);
        
        // %b (percent b) - where price is in relation to the bands
        let percent_b = if (upper_band - lower_band).abs() < f64::EPSILON {
            0.5 // Neutral value when bands are too close
        } else {
            (price - lower_band) / (upper_band - lower_band)
        };
        
        // Bandwidth - the width of the bands relative to the middle band
        let bandwidth = if middle_band.abs() > f64::EPSILON {
            (upper_band - lower_band) / middle_band
        } else {
            0.0
        };
        
        Some((middle_band, upper_band, lower_band, percent_b, bandwidth))
    }
}

impl Indicator for BollingerBands {
    type Input = f64;
    type Output = (f64, f64, f64, f64, f64); // (middle, upper, lower, %b, bandwidth)
    
    fn update(&mut self, price: Self::Input) -> IndicatorResult<Self::Output> {
        self.prices.push_back(price);
        
        if self.prices.len() > self.period {
            self.prices.pop_front();
        }
        
        if self.prices.len() < self.period {
            return Err(IndicatorError::NotEnoughData);
        }
        
        self.value(price).ok_or(IndicatorError::NotEnoughData)
    }
    
    fn reset(&mut self) {
        self.prices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_atr() {
        let mut atr = ATR::new(14).unwrap();
        
        // Test with increasing prices
        let mut result = None;
        for i in 1..30 {
            let high = i as f64 + 1.0;
            let low = i as f64 - 1.0;
            let close = i as f64;
            
            match atr.update((high, low, close)) {
                Ok(val) => result = Some(val),
                Err(_) => ()
            }
        }
        
        // After enough data, we should get a value
        assert!(result.is_some());
        let atr_value = result.unwrap();
        assert!(atr_value > 0.0);
    }
    
    #[test]
    fn test_bollinger_bands() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap(); // Use smaller period for testing
        
        // Test with increasing prices
        let prices = vec![20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0];
        
        for (i, &price) in prices.iter().enumerate() {
            let result = bb.update(price);
            
            if i >= 4 { // Need at least period (5) to get valid values
                let (middle, upper, lower, percent_b, _) = result.unwrap();
                
                assert!(upper > middle);
                assert!(middle > lower);
                assert!(lower >= 0.0); // Lower can be 0 or positive
                
                // %b should be a valid number
                assert!(!percent_b.is_nan());
                assert!(!percent_b.is_infinite());
            } else {
                assert!(result.is_err());
            }
        }
    }
}


