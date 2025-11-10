# 交易策略模块

这个模块包含各种股票交易策略的实现，目前主要实现了价量K线策略。

## 模块结构

```
strategy/
├── price_volume_candlestick_strategy.rs  # 价量K线策略核心实现
├── examples.rs                          # 使用示例
├── mod.rs                              # 模块定义
└── README.md                           # 说明文档
```

## 价量K线策略 (PriceVolumeCandlestickStrategy)

### 功能特点

这是一个综合性的交易策略，结合了：

1. **K线形态识别** - 识别经典的K线形态
2. **成交量分析** - 分析成交量与价格的配合关系
3. **趋势判断** - 基于价格走势判断短期趋势
4. **信号生成** - 综合各项指标生成买卖信号

### K线形态识别

支持识别以下经典K线形态：

- **锤子线 (Hammer)** - 看涨反转信号
- **倒锤子线 (InvertedHammer)** - 可能的看涨信号
- **上吊线 (HangingMan)** - 看跌反转信号
- **流星线 (ShootingStar)** - 看跌信号
- **十字星 (Doji)** - 变盘信号
- **长阳线 (LongBullish)** - 强烈看涨
- **长阴线 (LongBearish)** - 强烈看跌
- **小阳线/小阴线** - 温和信号

### 成交量信号分析

- **放量上涨 (VolumeUptrend)** - 买盘活跃，看涨信号
- **放量下跌 (VolumeDowntrend)** - 卖盘汹涌，看跌信号
- **缩量上涨 (LowVolumeUptrend)** - 上涨乏力
- **缩量下跌 (LowVolumeDowntrend)** - 抛压减轻
- **异常放量 (AbnormalVolume)** - 需要谨慎

### 策略信号

- **强烈买入 (StrongBuy)** - 多重看涨信号确认
- **买入 (Buy)** - 看涨信号
- **持有 (Hold)** - 观望
- **卖出 (Sell)** - 看跌信号
- **强烈卖出 (StrongSell)** - 多重看跌信号确认

## 快速开始

### 基础用法

```rust
use service::strategy::{PriceVolumeCandlestickStrategy, StrategyConfig};

// 使用默认配置创建策略
let mut strategy = PriceVolumeCandlestickStrategy::default();

// 分析单只股票
let result = strategy.analyze("000001.SZ", &daily_data)?;

println!("策略信号: {:?}", result.strategy_signal);
println!("信号强度: {}%", result.signal_strength);
println!("分析说明: {}", result.analysis_description);
```

### 自定义配置

```rust
let config = StrategyConfig {
    analysis_period: 30,                    // 30天分析周期
    volume_ma_period: 10,                   // 10日成交量均线
    price_volatility_threshold: 5.0,       // 5%价格波动阈值
    volume_amplification_threshold: 2.0,    // 2倍成交量放大阈值
    candlestick_body_threshold: 3.0,       // 3%K线实体阈值
};

let mut strategy = PriceVolumeCandlestickStrategy::new(config);
```

### 批量分析

```rust
// 准备多只股票数据
let stocks_data = vec![
    ("000001.SZ".to_string(), stock1_data),
    ("000002.SZ".to_string(), stock2_data),
    ("600000.SH".to_string(), stock3_data),
];

// 批量分析，结果按信号强度排序
let results = strategy.batch_analyze(&stocks_data);

for result in results {
    if matches!(result.strategy_signal, StrategySignal::StrongBuy | StrategySignal::Buy) {
        println!("买入机会: {} - {}", result.stock_code, result.analysis_description);
    }
}
```

## 策略配置参数

### StrategyConfig

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `analysis_period` | usize | 20 | 分析周期天数 |
| `volume_ma_period` | usize | 5 | 成交量均线周期 |
| `price_volatility_threshold` | f64 | 3.0 | 价格波动阈值(%) |
| `volume_amplification_threshold` | f64 | 1.5 | 成交量放大倍数阈值 |
| `candlestick_body_threshold` | f64 | 2.0 | K线实体大小阈值(%) |

## 分析结果

### StrategyResult

```rust
pub struct StrategyResult {
    pub stock_code: String,              // 股票代码
    pub analysis_date: NaiveDate,        // 分析日期
    pub current_price: f64,              // 当前价格
    pub candlestick_pattern: CandlestickPattern,  // K线形态
    pub volume_signal: VolumeSignal,     // 成交量信号
    pub strategy_signal: StrategySignal, // 策略信号
    pub signal_strength: u8,             // 信号强度(0-100)
    pub analysis_description: String,    // 分析说明
    pub risk_level: u8,                  // 风险等级(1-5)
}
```

## 使用场景

### 1. 日内交易

适合短线交易者，关注：
- K线形态的即时信号
- 成交量的异常变化
- 快速的信号响应

### 2. 波段交易

适合中短期持有，关注：
- 趋势的确认信号
- 成交量与价格的配合
- 风险控制

### 3. 选股筛选

结合技术分析选股模块使用：
- 批量分析大量股票
- 筛选出强信号股票
- 排序和优先级设定

## 策略优化建议

### 1. 参数调优

根据不同市场环境调整参数：
- 牛市：降低买入阈值，提高卖出阈值
- 熊市：提高买入阈值，降低卖出阈值
- 震荡市：增加持有信号的权重

### 2. 多时间框架

结合不同时间周期：
- 日线：主要信号
- 周线：趋势确认
- 分钟线：精确入场

### 3. 风险管理

- 设置止损位
- 控制仓位大小
- 分散投资组合

## 扩展功能

可以基于现有框架扩展：

1. **更多K线形态**
   - 多K线组合形态
   - 缺口分析
   - 支撑阻力位

2. **高级成交量分析**
   - 成交量价格趋势(VPT)
   - 资金流向指标
   - 大单分析

3. **机器学习增强**
   - 特征工程
   - 模型训练
   - 预测准确率优化

## 使用示例

完整的使用示例请参考 `examples.rs` 文件，包含：

- 基础策略分析
- 自定义配置使用
- 批量分析处理
- K线形态识别
- 成交量信号分析
- 策略信号筛选

运行示例：

```rust
use service::strategy::examples;

// 运行所有示例
examples::run_all_examples().await?;
```

## 性能考虑

1. **数据缓存** - 避免重复计算
2. **并行处理** - 批量分析时使用多线程
3. **增量更新** - 只处理新增数据
4. **内存管理** - 及时清理历史数据

## 注意事项

1. **数据质量** - 确保输入数据的准确性
2. **市场环境** - 策略效果受市场环境影响
3. **风险控制** - 策略信号仅供参考，需结合风险管理
4. **回测验证** - 建议进行充分的历史回测

这个策略模块为量化交易提供了坚实的基础，可以根据具体需求进行定制和扩展。
