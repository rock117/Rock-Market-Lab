//! Volume indicators module
//! 
//! This module contains indicators that analyze trading volume.

use super::{Indicator, IndicatorResult, IndicatorError};

/// On-Balance Volume (OBV)
///
/// A momentum indicator that uses volume flow to predict changes in stock price.
#[derive(Debug, Clone, Default)]
pub struct OBV {
    previous_close: Option<f64>,
    current_obv: f64,
}

impl OBV {
    /// Creates a new OBV indicator
    pub fn new() -> Self {
        Self {
            previous_close: None,
            current_obv: 0.0,
        }
    }
    
    /// Returns the current OBV value
    pub fn value(&self) -> f64 {
        self.current_obv
    }
}

impl Indicator for OBV {
    type Input = (f64, f64); // (close, volume)
    type Output = f64;
    
    fn update(&mut self, (close, volume): Self::Input) -> IndicatorResult<Self::Output> {
        if let Some(prev_close) = self.previous_close {
            if close > prev_close {
                // If closing price is higher than previous close, add volume to OBV
                self.current_obv += volume;
            } else if close < prev_close {
                // If closing price is lower than previous close, subtract volume from OBV
                self.current_obv -= volume;
            }
            // If close equals previous close, OBV remains the same
            
            self.previous_close = Some(close);
            Ok(self.current_obv)
        } else {
            // First data point - just store it, no OBV calculation yet
            self.previous_close = Some(close);
            Err(IndicatorError::NotEnoughData)
        }
    }
    
    fn reset(&mut self) {
        self.previous_close = None;
        self.current_obv = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_obv() {
        let mut obv = OBV::new();
        
        // First update - should return error as we need at least two data points
        assert!(obv.update((100.0, 1000.0)).is_err());
        
        // Price up - add volume
        assert_relative_eq!(obv.update((101.0, 1500.0)).unwrap(), 1500.0);
        
        // Price down - subtract volume
        assert_relative_eq!(obv.update((100.5, 2000.0)).unwrap(), -500.0);
        
        // Price same - no change
        assert_relative_eq!(obv.update((100.5, 1000.0)).unwrap(), -500.0);
        
        // Price up again - add volume
        assert_relative_eq!(obv.update((102.0, 800.0)).unwrap(), 300.0);
        
        // Reset and test again
        obv.reset();
        // After reset, first update should still return error
        assert!(obv.update((100.0, 1000.0)).is_err());
    }
}
