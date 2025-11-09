//! Usage examples for technical indicators
//! 
//! This module provides practical examples of how to use the technical indicators.

#[cfg(test)]
mod examples {
    use super::super::*;
    
    #[test]
    fn example_basic_usage() {
        // Sample price data
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 46.08, 45.89, 46.03,
            46.83, 47.69, 46.49, 46.26, 47.09, 46.66, 46.80, 46.23, 46.08, 46.03,
        ];
        
        println!("=== Basic Technical Indicators Example ===");
        
        // 1. Simple Moving Average
        let sma_values = sma(&prices, 5).unwrap();
        println!("SMA(5): {:?}", sma_values);
        
        // 2. Exponential Moving Average
        let ema_values = ema(&prices, 5).unwrap();
        println!("EMA(5): {:?}", ema_values);
        
        // 3. RSI
        let rsi_values = rsi(&prices, 14).unwrap();
        println!("RSI(14): {:?}", rsi_values);
        
        // 4. MACD
        let macd_values = macd(&prices, 12, 26, 9).unwrap();
        println!("MACD: {:?}", macd_values);
    }
    
    #[test]
    fn example_real_time_processing() {
        println!("=== Real-time Processing Example ===");
        
        // Create indicators for real-time processing
        let mut sma20 = SMA::new(20).unwrap();
        let mut ema12 = EMA::new(12).unwrap();
        let mut rsi14 = RSI::new(14).unwrap();
        
        // Simulate real-time price updates
        let prices = vec![
            100.0, 101.0, 99.5, 102.0, 103.5, 101.8, 104.2, 102.1, 105.0, 103.7,
            106.2, 104.5, 107.1, 105.8, 108.3, 106.9, 109.5, 107.2, 110.1, 108.6,
        ];
        
        for (i, price) in prices.iter().enumerate() {
            println!("\n--- Update {} (Price: {:.2}) ---", i + 1, price);
            
            if let Ok(sma_val) = sma20.update(*price) {
                println!("SMA20: {:.2}", sma_val);
            }
            
            if let Ok(ema_val) = ema12.update(*price) {
                println!("EMA12: {:.2}", ema_val);
            }
            
            if let Ok(rsi_val) = rsi14.update(*price) {
                println!("RSI14: {:.2}", rsi_val);
            }
        }
    }
    
    #[test]
    fn example_indicator_builder() {
        println!("=== Indicator Builder Example ===");
        
        let mut builder = IndicatorBuilder::new();
        builder
            .add_sma(10)
            .add_sma(20)
            .add_ema(12)
            .add_rsi(14);
        
        let prices = vec![
            100.0, 101.0, 99.5, 102.0, 103.5, 101.8, 104.2, 102.1, 105.0, 103.7,
            106.2, 104.5, 107.1, 105.8, 108.3, 106.9, 109.5, 107.2, 110.1, 108.6,
            111.2, 109.8, 112.5, 110.3, 113.7, 111.9, 114.2, 112.6, 115.1, 113.4,
        ];
        
        for (i, price) in prices.iter().enumerate() {
            let results = builder.update(*price);
            
            if !results.is_empty() {
                println!("\nUpdate {}: Price = {:.2}", i + 1, price);
                for (name, value) in &results {
                    println!("  {}: {:.2}", name, value);
                }
            }
        }
    }
    
    #[test]
    fn example_bollinger_bands() {
        println!("=== Bollinger Bands Example ===");
        
        let prices = vec![
            20.0, 20.5, 21.0, 20.8, 21.2, 21.5, 21.8, 22.0, 21.7, 22.2,
            22.5, 22.8, 23.0, 22.7, 23.2, 23.5, 23.8, 24.0, 23.7, 24.2,
        ];
        
        let bb_values = bollinger_bands(&prices, 10, 2.0).unwrap();
        
        for (i, (middle, upper, lower, percent_b, bandwidth)) in bb_values.iter().enumerate() {
            println!(
                "Day {}: Price={:.2}, BB=[{:.2}, {:.2}, {:.2}], %B={:.2}, BW={:.3}",
                i + 10, // Starting from day 10 (period=10)
                prices[i + 9],
                upper, middle, lower, percent_b, bandwidth
            );
        }
    }
    
    #[test]
    fn example_trading_signals() {
        println!("=== Trading Signals Example ===");
        
        let prices = vec![
            100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0,
            111.0, 110.0, 112.0, 114.0, 113.0, 115.0, 117.0, 116.0, 118.0, 120.0,
            119.0, 121.0, 123.0, 122.0, 124.0, 126.0, 125.0, 127.0, 129.0, 128.0,
        ];
        
        // Calculate indicators
        let sma_short = sma(&prices, 5).unwrap();
        let sma_long = sma(&prices, 10).unwrap();
        let rsi_values = rsi(&prices, 14).unwrap();
        
        // Generate trading signals
        for i in 0..sma_short.len().min(sma_long.len()).min(rsi_values.len()) {
            let price = prices[i + 9]; // Adjust for the longest period (10)
            let short_ma = sma_short[i + 5]; // Adjust for SMA(5) offset
            let long_ma = sma_long[i];
            let rsi_val = if i + 5 < rsi_values.len() { rsi_values[i + 5] } else { continue; };
            
            let mut signals = Vec::new();
            
            // Golden Cross (bullish signal)
            if short_ma > long_ma && i > 0 {
                let prev_short = if i > 0 { sma_short[i + 4] } else { short_ma };
                let prev_long = sma_long[i - 1];
                if prev_short <= prev_long {
                    signals.push("GOLDEN_CROSS");
                }
            }
            
            // Death Cross (bearish signal)
            if short_ma < long_ma && i > 0 {
                let prev_short = if i > 0 { sma_short[i + 4] } else { short_ma };
                let prev_long = sma_long[i - 1];
                if prev_short >= prev_long {
                    signals.push("DEATH_CROSS");
                }
            }
            
            // RSI signals
            if rsi_val > 70.0 {
                signals.push("RSI_OVERBOUGHT");
            } else if rsi_val < 30.0 {
                signals.push("RSI_OVERSOLD");
            }
            
            if !signals.is_empty() {
                println!(
                    "Day {}: Price={:.2}, SMA5={:.2}, SMA10={:.2}, RSI={:.1} => {}",
                    i + 10,
                    price,
                    short_ma,
                    long_ma,
                    rsi_val,
                    signals.join(", ")
                );
            }
        }
    }
}
