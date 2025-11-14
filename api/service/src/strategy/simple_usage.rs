//! 简化的策略使用示例
//! 
//! 展示去掉 Factory 模式后的简洁用法

use anyhow::Result;
use super::{
    TradingStrategy, 
    PriceVolumeCandlestickStrategy, 
    PriceVolumeStrategyConfig,
    SecurityData,
    SecurityType,
    TimeFrame
};
use rust_decimal::Decimal;

/// 基本使用示例
pub fn basic_usage_example() -> Result<()> {
    // 1. 使用默认配置创建策略
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 2. 使用自定义配置创建策略
    let config = PriceVolumeStrategyConfig {
        analysis_period: 20,
        volume_ma_period: 5,
        price_volatility_threshold: 3.0,
        volume_amplification_threshold: 1.5,
        candlestick_body_threshold: 2.0,
    };
    let mut custom_strategy = PriceVolumeCandlestickStrategy::new(config);
    
    // 3. 使用预设配置创建策略
    let conservative_strategy = PriceVolumeCandlestickStrategy::conservative();
    let aggressive_strategy = PriceVolumeCandlestickStrategy::aggressive();
    
    // 4. 获取策略信息
    let info = PriceVolumeCandlestickStrategy::info();
    println!("策略名称: {}", info.name);
    println!("策略描述: {}", info.description);
    println!("风险等级: {:?}", info.risk_level);
    
    // 5. 分析股票数据
    let test_data = create_sample_data();
    let result = strategy.analyze("000001.SZ", &test_data)?;
    
    println!("分析结果:");
    println!("  股票代码: {}", result.stock_code());
    println!("  策略信号: {:?}", result.strategy_signal());
    println!("  信号强度: {}%", result.signal_strength());
    println!("  风险等级: {}/5", result.risk_level());
    
    Ok(())
}

/// 批量分析示例
pub fn batch_analysis_example() -> Result<()> {
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 准备多只证券数据
    let securities_data = vec![
        ("000001.SZ".to_string(), create_sample_data()),
        ("000002.SZ".to_string(), create_sample_data()),
        ("600000.SH".to_string(), create_sample_data()),
    ];
    
    // 批量分析
    let results = strategy.batch_analyze(&securities_data);
    
    println!("批量分析结果 (按信号强度排序):");
    for result in results.iter().take(5) {
        println!("  {}: {:?} ({}%)", 
            result.stock_code(), 
            result.strategy_signal(), 
            result.signal_strength()
        );
    }
    
    Ok(())
}

/// 策略比较示例
pub fn strategy_comparison_example() -> Result<()> {
    let test_data = create_sample_data();
    
    // 创建不同配置的策略
    let strategies = vec![
        ("保守策略", PriceVolumeCandlestickStrategy::conservative()),
        ("激进策略", PriceVolumeCandlestickStrategy::aggressive()),
        ("默认策略", PriceVolumeCandlestickStrategy::default()),
    ];
    
    println!("策略比较结果:");
    for (name, mut strategy) in strategies {
        let result = strategy.analyze("TEST", &test_data)?;
        println!("  {}: {:?} (信号强度: {}%, 风险: {}/5)", 
            name,
            result.strategy_signal(),
            result.signal_strength(),
            result.risk_level()
        );
    }
    
    Ok(())
}

/// 创建示例数据
fn create_sample_data() -> Vec<SecurityData> {
    vec![
        SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20240101".to_string(),
            open: 10.00,
            high: 10.50,
            low: 9.80,
            close: 10.20,
            pre_close: Some(10.00),
            change: Some(0.20),
            pct_change: Some(2.00),
            volume: 1000000.0,
            amount: 10200000.0,
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
            financial_data: None,
        },
        SecurityData {
            symbol: "000001.SZ".to_string(),
            trade_date: "20240102".to_string(),
            open: 10.20,
            high: 10.80,
            low: 10.00,
            close: 10.50,
            pre_close: Some(10.20),
            change: Some(0.30),
            pct_change: Some(2.94),
            volume: 1200000.0,
            amount: 12600000.0,
            security_type: SecurityType::Stock,
            time_frame: TimeFrame::Daily,
            financial_data: None,
        },
        // 更多测试数据...
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_usage() {
        let result = basic_usage_example();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_batch_analysis() {
        let result = batch_analysis_example();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_strategy_comparison() {
        let result = strategy_comparison_example();
        assert!(result.is_ok());
    }
}
