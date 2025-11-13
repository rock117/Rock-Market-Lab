//! 价量K线策略使用示例

use anyhow::Result;
use tracing::info;
use super::traits::{TradingStrategy, StrategySignal, SecurityData, SecurityType, TimeFrame};
use super::price_volume_candlestick_strategy::{
    PriceVolumeCandlestickStrategy, PriceVolumeStrategyConfig, PriceVolumeAnalysisResult,
    CandlestickPattern, VolumeSignal
};

/// 基础策略分析示例
pub async fn basic_strategy_example() -> Result<()> {
    info!("=== 价量K线策略基础示例 ===");
    
    // 创建策略实例
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 创建测试数据
    let test_data = create_sample_data();
    
    // 分析单只股票
    let result = strategy.analyze("000001.SZ", &test_data)?;
    
    info!("股票分析结果:");
    info!("  股票代码: {}", result.stock_code());
    info!("  当前价格: {:.2}", result.current_price());
    
    // 根据策略类型提取特定信息
    match &result {
        super::StrategyResult::PriceVolumeCandlestick(r) => {
            info!("  K线形态: {}", r.candlestick_pattern);
            info!("  成交量信号: {}", r.volume_signal);
            info!("  价格波动率: {:.2}%", r.price_volatility);
            info!("  成交量比率: {:.2}", r.volume_ratio);
        },
        _ => {}
    }
    
    info!("  策略信号: {:?}", result.strategy_signal());
    info!("  信号强度: {}%", result.signal_strength());
    info!("  风险等级: {}/5", result.risk_level());
    info!("  分析说明: {}", result.analysis_description());
    
    Ok(())
}

/// 自定义配置示例
pub async fn custom_config_example() -> Result<()> {
    info!("=== 自定义配置示例 ===");
    
    // 自定义策略配置
    let config = PriceVolumeStrategyConfig {
        analysis_period: 30,                    // 30天分析周期
        volume_ma_period: 10,                   // 10日成交量均线
        price_volatility_threshold: 5.0,       // 5%价格波动阈值
        volume_amplification_threshold: 2.0,    // 2倍成交量放大阈值
        candlestick_body_threshold: 3.0,       // 3%K线实体阈值
    };
    
    let mut strategy = PriceVolumeCandlestickStrategy::new(config);
    
    // 分析数据
    let test_data = create_volatile_data();
    let result = strategy.analyze("000002.SZ", &test_data)?;
    
    info!("自定义配置分析结果:");
    info!("  策略信号: {:?} (强度: {}%)", result.strategy_signal(), result.signal_strength());
    info!("  分析说明: {}", result.analysis_description());
    
    Ok(())
}

/// 批量分析示例
pub async fn batch_analysis_example() -> Result<()> {
    info!("=== 批量分析示例 ===");
    
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 准备多只股票数据
    let stocks_data = vec![
        ("000001.SZ".to_string(), create_sample_data()),
        ("000002.SZ".to_string(), create_volatile_data()),
        ("600000.SH".to_string(), create_trending_data()),
    ];
    
    let results = strategy.batch_analyze(&stocks_data);
    
    info!("批量分析结果 (按信号强度排序):");
    for (i, result) in results.iter().enumerate() {
        info!("  {}. {} - {:?} ({}%): {}", 
            i + 1, 
            result.stock_code(), 
            result.strategy_signal(), 
            result.signal_strength(),
            result.analysis_description()
        );
    }
    
    Ok(())
}

/// 不同K线形态识别示例
pub async fn candlestick_patterns_example() -> Result<()> {
    info!("=== K线形态识别示例 ===");
    
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 测试不同的K线形态
    let patterns_data = vec![
        ("锤子线", create_hammer_data()),
        ("十字星", create_doji_data()),
        ("长阳线", create_long_bullish_data()),
        ("流星线", create_shooting_star_data()),
    ];
    
    for (pattern_name, data) in patterns_data {
        let result = strategy.analyze("TEST", &data)?;
        
        let pattern_info = match &result {
            super::StrategyResult::PriceVolumeCandlestick(r) => r.candlestick_pattern.clone(),
            _ => "未知".to_string(),
        };
            
        info!("  {}: {} - {}", 
            pattern_name, 
            pattern_info,
            result.analysis_description()
        );
    }
    
    Ok(())
}

/// 成交量信号分析示例
pub async fn volume_signals_example() -> Result<()> {
    info!("=== 成交量信号分析示例 ===");
    
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    let volume_scenarios = vec![
        ("放量上涨", create_volume_uptrend_data()),
        ("放量下跌", create_volume_downtrend_data()),
        ("缩量上涨", create_low_volume_uptrend_data()),
        ("异常放量", create_abnormal_volume_data()),
    ];
    
    for (scenario_name, data) in volume_scenarios {
        let result = strategy.analyze("TEST", &data)?;
        
        let volume_info = match &result {
            super::StrategyResult::PriceVolumeCandlestick(r) => r.volume_signal.clone(),
            _ => "未知".to_string(),
        };
            
        info!("  {}: {} - 信号强度{}%", 
            scenario_name, 
            volume_info,
            result.signal_strength()
        );
    }
    
    Ok(())
}

/// 策略信号筛选示例
pub async fn signal_filtering_example() -> Result<()> {
    info!("=== 策略信号筛选示例 ===");
    
    let mut strategy = PriceVolumeCandlestickStrategy::default();
    
    // 模拟多只股票数据
    let all_stocks = vec![
        ("STRONG_BUY".to_string(), create_strong_buy_data()),
        ("BUY".to_string(), create_buy_data()),
        ("HOLD".to_string(), create_hold_data()),
        ("SELL".to_string(), create_sell_data()),
        ("STRONG_SELL".to_string(), create_strong_sell_data()),
    ];
    
    let results = strategy.batch_analyze(&all_stocks);
    
    // 按信号类型分类
    let strong_buy: Vec<_> = results.iter()
        .filter(|r| r.strategy_signal() == StrategySignal::StrongBuy)
        .collect();
    let buy: Vec<_> = results.iter()
        .filter(|r| r.strategy_signal() == StrategySignal::Buy)
        .collect();
    let sell: Vec<_> = results.iter()
        .filter(|r| matches!(r.strategy_signal(), StrategySignal::Sell | StrategySignal::StrongSell))
        .collect();
    
    info!("强烈买入信号: {} 只", strong_buy.len());
    info!("买入信号: {} 只", buy.len());
    info!("卖出信号: {} 只", sell.len());
    
    Ok(())
}

// 辅助函数：创建各种测试数据

fn create_sample_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1050, 980, 1020, 1000000),
        create_security_data("20240102", 1020, 1080, 1000, 1050, 1200000),
        create_security_data("20240103", 1050, 1100, 1030, 1080, 1500000),
    ]
}

fn create_volatile_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1100, 900, 950, 2000000),
        create_security_data("20240102", 950, 1050, 850, 1000, 2500000),
        create_security_data("20240103", 1000, 1150, 950, 1100, 3000000),
    ]
}

fn create_trending_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1010, 800000),
        create_security_data("20240102", 1010, 1030, 1000, 1020, 850000),
        create_security_data("20240103", 1020, 1040, 1010, 1030, 900000),
    ]
}

fn create_hammer_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1010, 950, 1005, 1000000), // 锤子线
    ]
}

fn create_doji_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 980, 1000, 1000000), // 十字星
    ]
}

fn create_long_bullish_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1100, 990, 1090, 1500000), // 长阳线
    ]
}

fn create_shooting_star_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1050, 995, 1005, 1000000), // 流星线
    ]
}

fn create_volume_uptrend_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1010, 800000),
        create_security_data("20240102", 1010, 1030, 1000, 1020, 850000),
        create_security_data("20240103", 1020, 1050, 1015, 1040, 2000000), // 放量上涨
    ]
}

fn create_volume_downtrend_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1010, 800000),
        create_security_data("20240102", 1010, 1030, 1000, 1020, 850000),
        create_security_data("20240103", 1020, 1025, 980, 990, 2000000), // 放量下跌
    ]
}

fn create_low_volume_uptrend_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1010, 1000000),
        create_security_data("20240102", 1010, 1030, 1000, 1020, 1000000),
        create_security_data("20240103", 1020, 1040, 1015, 1030, 500000), // 缩量上涨
    ]
}

fn create_abnormal_volume_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1010, 800000),
        create_security_data("20240102", 1010, 1030, 1000, 1020, 850000),
        create_security_data("20240103", 1020, 1025, 1015, 1022, 5000000), // 异常放量
    ]
}

fn create_strong_buy_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1010, 950, 1005, 1000000), // 锤子线
        create_security_data("20240102", 1005, 1040, 1000, 1035, 2000000), // 放量上涨
    ]
}

fn create_buy_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 990, 1015, 1200000), // 小阳线 + 适度放量
    ]
}

fn create_hold_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1020, 980, 1000, 1000000), // 十字星
    ]
}

fn create_sell_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1010, 980, 985, 1200000), // 小阴线 + 放量
    ]
}

fn create_strong_sell_data() -> Vec<SecurityData> {
    vec![
        create_security_data("20240101", 1000, 1005, 950, 960, 2500000), // 长阴线 + 放量
    ]
}

fn create_security_data(date: &str, open: i64, high: i64, low: i64, close: i64, vol: i64) -> SecurityData {
    SecurityData {
        symbol: "TEST".to_string(),
        trade_date: date.to_string(),
        open: (open as f64) / 100.0,
        high: (high as f64) / 100.0,
        low: (low as f64) / 100.0,
        close: (close as f64) / 100.0,
        pre_close: Some((open as f64) / 100.0),
        change: Some(((close - open) as f64) / 100.0),
        pct_change: Some(((close - open) as f64) * 100.0 / (open as f64)),
        volume: vol as f64,
        amount: (vol * close / 100) as f64,
        security_type: SecurityType::Stock,
        time_frame: TimeFrame::Daily,
    }
}

/// 运行所有示例
pub async fn run_all_examples() -> Result<()> {
    info!("开始运行价量K线策略示例...");
    
    basic_strategy_example().await?;
    custom_config_example().await?;
    batch_analysis_example().await?;
    candlestick_patterns_example().await?;
    volume_signals_example().await?;
    signal_filtering_example().await?;
    
    info!("所有示例运行完成！");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_examples() {
        use crate::strategy::traits::TradingStrategy;
        
        let result = basic_strategy_example().await;
        assert!(result.is_ok());
        
        let result = batch_analysis_example().await;
        assert!(result.is_ok());
    }
}
