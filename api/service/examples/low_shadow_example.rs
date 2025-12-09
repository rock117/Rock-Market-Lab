//! 低位下影线策略使用示例
//! 
//! 展示如何使用低位下影线策略来识别股价在相对低位出现长下影线的反转信号

use service::strategy::{LowShadowStrategy, LowShadowConfig};
use service::strategy::traits::{TradingStrategy, SecurityData, TimeFrame, SecurityType};

fn main() {
    println!("=== 低位下影线策略示例 ===\n");

    // 1. 创建不同配置的策略
    let standard_strategy = LowShadowStrategy::default();
    let conservative_strategy = LowShadowStrategy::conservative();
    let aggressive_strategy = LowShadowStrategy::aggressive();

    println!("📊 策略配置对比:");
    println!("标准配置: {:?}", standard_strategy.config());
    println!("保守配置: {:?}", conservative_strategy.config());
    println!("激进配置: {:?}", aggressive_strategy.config());
    println!();

    // 2. 创建测试数据 - 模拟股价在低位出现长下影线
    let test_data = create_test_scenario();
    
    println!("📈 测试数据:");
    for (i, data) in test_data.iter().enumerate() {
        println!("第{}天: 开盘{:.2}, 最高{:.2}, 最低{:.2}, 收盘{:.2}, 成交量{:.0}", 
            i + 1, data.open, data.high, data.low, data.close, data.volume);
    }
    println!();

    // 3. 使用不同配置分析同一组数据
    let mut standard = LowShadowStrategy::default();
    let mut conservative = LowShadowStrategy::conservative();
    let mut aggressive = LowShadowStrategy::aggressive();

    println!("🔍 分析结果:");
    
    if let Ok(result) = standard.analyze("TEST001", &test_data) {
        println!("标准配置: 信号={:?}, 强度={}, 描述={}", 
            result.strategy_signal(), result.signal_strength(), result.analysis_description());
    }
    
    if let Ok(result) = conservative.analyze("TEST001", &test_data) {
        println!("保守配置: 信号={:?}, 强度={}, 描述={}", 
            result.strategy_signal(), result.signal_strength(), result.analysis_description());
    }
    
    if let Ok(result) = aggressive.analyze("TEST001", &test_data) {
        println!("激进配置: 信号={:?}, 强度={}, 描述={}", 
            result.strategy_signal(), result.signal_strength(), result.analysis_description());
    }

    println!("\n✅ 低位下影线策略示例运行完成!");
}

/// 创建测试场景数据
/// 模拟股价从高位回调到低位，最后一天出现长下影线
fn create_test_scenario() -> Vec<SecurityData> {
    let mut data = Vec::new();
    
    // 前19天：价格从120逐渐下跌到102区间
    for i in 0..19 {
        let base_price = 120.0 - (i as f64 * 0.8);
        let volume = 1000000.0 + (i as f64 * 10000.0); // 成交量逐渐放大
        
        data.push(SecurityData {
            trade_date: format!("2024010{:02}", i + 1),
            symbol: "TEST001".to_string(),
            open: base_price,
            high: base_price + 1.0,
            low: base_price - 0.5,
            close: base_price - 0.2,
            pre_close: Some(base_price + 0.2),
            change: Some(-0.2),
            volume,
            amount: volume * base_price,
            turnover_rate: Some(1.0 + (i as f64 * 0.1)),
            pct_change: Some(-0.2),
            time_frame: TimeFrame::Daily,
            security_type: SecurityType::Stock,
            financial_data: None,
        });
    }
    
    // 第20天：在低位出现长下影线阳线，成交量放大
    data.push(SecurityData {
        trade_date: "20240120".to_string(),
        symbol: "TEST001".to_string(),
        open: 102.0,        // 开盘价
        high: 104.0,        // 最高价
        low: 98.0,          // 最低价（长下影线）
        close: 103.5,       // 收盘价（阳线）
        pre_close: Some(101.8),
        change: Some(1.7),
        volume: 1800000.0,  // 成交量放大1.8倍
        amount: 1800000.0 * 103.5,
        turnover_rate: Some(2.5),
        pct_change: Some(1.67),
        time_frame: TimeFrame::Daily,
        security_type: SecurityType::Stock,
        financial_data: None,
    });
    
    data
}
