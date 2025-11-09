# Technical Indicators Module

这个模块提供了常用的技术指标实现，适用于金融市场分析。

## 功能特性

- **实时计算**：支持流式数据处理，适合实时交易系统
- **批量计算**：支持历史数据批量分析
- **类型安全**：使用 Rust 类型系统确保数据安全
- **高性能**：优化的算法实现，支持大量数据处理
- **易于使用**：提供简洁的 API 和便利函数

## 支持的指标

### 趋势指标 (Trend Indicators)
- **SMA** - Simple Moving Average (简单移动平均线)
- **EMA** - Exponential Moving Average (指数移动平均线)  
- **SAR** - Parabolic Stop and Reverse (抛物线转向指标)

### 动量指标 (Momentum Indicators)
- **RSI** - Relative Strength Index (相对强弱指数)
- **MACD** - Moving Average Convergence Divergence (指数平滑移动平均线)
- **KDJ** - Stochastic Oscillator (随机振荡器)

### 波动性指标 (Volatility Indicators)
- **ATR** - Average True Range (平均真实波幅)
- **BOLL** - Bollinger Bands (布林带)

### 成交量指标 (Volume Indicators)
- **OBV** - On-Balance Volume (能量潮)

## 快速开始

### 1. 基本用法 - 便利函数

```rust
use common::technical_indicators::*;

// 价格数据
let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 46.08];

// 计算 20 日简单移动平均线
let sma_values = sma(&prices, 5)?;
println!("SMA(5): {:?}", sma_values);

// 计算 RSI
let rsi_values = rsi(&prices, 6)?;
println!("RSI(6): {:?}", rsi_values);

// 计算 MACD
let macd_values = macd(&prices, 12, 26, 9)?;
for (macd_line, signal_line, histogram) in macd_values {
    println!("MACD: {:.2}, Signal: {:.2}, Histogram: {:.2}", 
             macd_line, signal_line, histogram);
}
```

### 2. 实时处理 - 使用指标对象

```rust
use common::technical_indicators::{SMA, EMA, RSI, Indicator};

// 创建指标实例
let mut sma20 = SMA::new(20)?;
let mut ema12 = EMA::new(12)?;
let mut rsi14 = RSI::new(14)?;

// 模拟实时价格更新
for price in price_stream {
    // 更新各个指标
    if let Ok(sma_val) = sma20.update(price) {
        println!("SMA20: {:.2}", sma_val);
    }
    
    if let Ok(ema_val) = ema12.update(price) {
        println!("EMA12: {:.2}", ema_val);
    }
    
    if let Ok(rsi_val) = rsi14.update(price) {
        println!("RSI14: {:.2}", rsi_val);
        
        // 基于 RSI 的交易信号
        if rsi_val > 70.0 {
            println!("⚠️  RSI 超买信号");
        } else if rsi_val < 30.0 {
            println!("📈 RSI 超卖信号");
        }
    }
}
```

### 3. 指标组合器 - IndicatorBuilder

```rust
use common::technical_indicators::IndicatorBuilder;

let mut builder = IndicatorBuilder::new();
builder
    .add_sma(10)    // 10日均线
    .add_sma(20)    // 20日均线
    .add_ema(12)    // 12日指数均线
    .add_rsi(14);   // 14日RSI

// 处理价格数据
for price in prices {
    let results = builder.update(price);
    
    // 获取所有指标值
    for (name, value) in &results {
        println!("{}: {:.2}", name, value);
    }
    
    // 交易策略示例
    if let (Some(sma10), Some(sma20), Some(rsi)) = (
        results.get("SMA_10"),
        results.get("SMA_20"), 
        results.get("RSI_14")
    ) {
        // 金叉 + RSI 不超买
        if sma10 > sma20 && rsi < &70.0 {
            println!("🚀 买入信号");
        }
        // 死叉 + RSI 不超卖
        else if sma10 < sma20 && rsi > &30.0 {
            println!("📉 卖出信号");
        }
    }
}
```

### 4. 高级用法 - 多种价格数据

```rust
// 布林带 (需要价格数据)
let bb_values = bollinger_bands(&prices, 20, 2.0)?;
for (middle, upper, lower, percent_b, bandwidth) in bb_values {
    println!("布林带: 上轨={:.2}, 中轨={:.2}, 下轨={:.2}", upper, middle, lower);
}

// ATR (需要最高价、最低价、收盘价)
let highs = vec![10.5, 11.0, 11.2, 10.8, 11.5];
let lows = vec![10.0, 10.3, 10.8, 10.2, 10.9];
let closes = vec![10.2, 10.8, 11.0, 10.5, 11.2];
let atr_values = atr(&highs, &lows, &closes, 14)?;

// KDJ (需要最高价、最低价、收盘价)
let kdj_values = kdj(&highs, &lows, &closes, 9, 3, 3)?;
for (k, d, j) in kdj_values {
    println!("KDJ: K={:.1}, D={:.1}, J={:.1}", k, d, j);
}

// OBV (需要收盘价和成交量)
let volumes = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
let obv_values = obv(&closes, &volumes)?;
```

## 错误处理

所有指标函数都返回 `IndicatorResult<T>`，可能的错误类型：

```rust
use common::technical_indicators::{IndicatorError, IndicatorResult};

match sma(&prices, 20) {
    Ok(values) => println!("SMA 计算成功: {:?}", values),
    Err(IndicatorError::NotEnoughData) => println!("数据不足，需要更多历史数据"),
    Err(IndicatorError::InvalidParameter(msg)) => println!("参数错误: {}", msg),
    Err(e) => println!("其他错误: {}", e),
}
```

## 性能考虑

- **内存效率**：指标只保存必要的历史数据
- **计算效率**：使用增量计算，避免重复计算
- **实时性**：支持 O(1) 时间复杂度的更新操作

## 常用参数建议

| 指标 | 常用参数 | 说明 |
|------|----------|------|
| SMA | 5, 10, 20, 50, 200 | 短期到长期趋势 |
| EMA | 12, 26 | MACD 默认参数 |
| RSI | 14 | 标准周期 |
| MACD | (12, 26, 9) | 快线、慢线、信号线 |
| ATR | 14 | 标准周期 |
| 布林带 | (20, 2.0) | 周期和标准差倍数 |
| KDJ | (9, 3, 3) | K、D、J 周期 |

## 测试

运行测试以验证指标计算的正确性：

```bash
cargo test --package common --lib indicators
```

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个模块。
