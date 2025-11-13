# 底部放量上涨策略使用文档

## 策略概述

**底部放量上涨策略（Bottom Volume Surge Strategy）** 用于识别证券在底部区域出现放量上涨的信号，这通常是底部反转的重要标志。

### 核心逻辑

1. **底部判断**：证券在近N日内价格波动范围较小，处于横盘整理状态
2. **放量确认**：当前成交量显著超过近期平均成交量
3. **价格上涨**：价格相对底部出现明显上涨

### 适用场景

- 捕捉底部反转机会
- 识别主力资金进场信号
- 适合中短期交易

---

## 策略配置

### BottomVolumeSurgeConfig 参数说明

```rust
pub struct BottomVolumeSurgeConfig {
    /// 分析周期（天数）
    pub analysis_period: usize,
    
    /// 成交量均线周期
    pub volume_ma_period: usize,
    
    /// 成交量放大倍数阈值
    pub volume_surge_threshold: f64,
    
    /// 价格上涨阈值（百分比）
    pub price_rise_threshold: f64,
    
    /// 底部判断周期（天数）
    pub bottom_period: usize,
    
    /// 底部价格波动范围（百分比）
    pub bottom_price_range: f64,
}
```

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `analysis_period` | 20 | 需要的历史数据天数 |
| `volume_ma_period` | 5 | 计算成交量均值的周期 |
| `volume_surge_threshold` | 1.5 | 成交量需要达到均值的倍数 |
| `price_rise_threshold` | 2.0 | 价格需要上涨的百分比 |
| `bottom_period` | 10 | 判断底部的回溯天数 |
| `bottom_price_range` | 5.0 | 底部价格波动不超过此百分比 |

---

## 使用示例

### 1. 使用默认配置

```rust
use service::strategy::{
    BottomVolumeSurgeStrategy,
    TradingStrategy,
    SecurityData,
};

// 创建策略实例
let mut strategy = BottomVolumeSurgeStrategy::default_config();

// 准备数据
let data: Vec<SecurityData> = get_stock_data("000001.SZ").await?;

// 执行分析
let result = strategy.analyze("000001.SZ", &data)?;

println!("信号: {:?}", result.strategy_signal);
println!("信号强度: {}", result.signal_strength);
println!("分析说明: {}", result.analysis_description);
```

### 2. 使用保守配置

```rust
// 保守策略：要求更严格的条件
let mut strategy = BottomVolumeSurgeStrategy::conservative();

let result = strategy.analyze("000001.SZ", &data)?;

// 保守策略配置：
// - 分析周期: 30天
// - 成交量放大: 2.0倍
// - 价格上涨: 3.0%
// - 底部波动: 3.0%
```

### 3. 使用激进配置

```rust
// 激进策略：更容易触发信号
let mut strategy = BottomVolumeSurgeStrategy::aggressive();

let result = strategy.analyze("000001.SZ", &data)?;

// 激进策略配置：
// - 分析周期: 15天
// - 成交量放大: 1.2倍
// - 价格上涨: 1.0%
// - 底部波动: 8.0%
```

### 4. 自定义配置

```rust
use service::strategy::BottomVolumeSurgeConfig;

// 创建自定义配置
let config = BottomVolumeSurgeConfig {
    analysis_period: 25,
    volume_ma_period: 7,
    volume_surge_threshold: 1.8,
    price_rise_threshold: 2.5,
    bottom_period: 12,
    bottom_price_range: 4.0,
};

let mut strategy = BottomVolumeSurgeStrategy::new(config);
let result = strategy.analyze("000001.SZ", &data)?;
```

### 5. 批量分析多只股票

```rust
use service::strategy::TradingStrategy;

let mut strategy = BottomVolumeSurgeStrategy::default_config();

// 准备多只股票的数据
let securities_data = vec![
    ("000001.SZ".to_string(), stock1_data),
    ("600000.SH".to_string(), stock2_data),
    ("000002.SZ".to_string(), stock3_data),
];

// 批量分析
let results = strategy.batch_analyze(&securities_data);

// 结果已按信号强度排序
for result in results.iter().take(10) {
    println!("{}: {:?} (强度: {})", 
        result.stock_code, 
        result.strategy_signal, 
        result.signal_strength
    );
}
```

---

## 策略结果解读

### BottomVolumeSurgeResult 字段说明

```rust
pub struct BottomVolumeSurgeResult {
    // 基础字段
    pub stock_code: String,
    pub analysis_date: NaiveDate,
    pub current_price: f64,
    pub strategy_signal: StrategySignal,
    pub signal_strength: u8,
    pub analysis_description: String,
    pub risk_level: u8,
    
    // 策略特定字段
    pub is_at_bottom: bool,           // 是否处于底部
    pub bottom_price: f64,            // 底部价格
    pub bottom_date: String,          // 底部日期
    pub current_volume: f64,          // 当前成交量
    pub volume_ma: f64,               // 成交量均值
    pub volume_surge_ratio: f64,      // 成交量放大倍数
    pub price_rise_pct: f64,          // 价格涨幅
    pub recent_low: f64,              // 近期最低价
    pub recent_high: f64,             // 近期最高价
}
```

### 信号等级

| 信号 | 信号强度 | 说明 |
|------|----------|------|
| `StrongBuy` | 80-100 | 强烈买入：底部明确，放量显著，价格上涨明显 |
| `Buy` | 60-79 | 买入：符合大部分条件 |
| `Hold` | 40-59 | 持有：部分条件满足 |
| `Sell` | 20-39 | 卖出：条件不满足 |
| `StrongSell` | 0-19 | 强烈卖出：完全不符合条件 |

### 信号强度计算规则

- **底部判断** (40分)：是否处于底部区域
- **成交量放大** (30分)：
  - ≥ 2.0倍阈值：30分
  - ≥ 1.5倍阈值：20分
  - ≥ 1.0倍阈值：15分
- **价格上涨** (30分)：
  - ≥ 2.0倍阈值：30分
  - ≥ 1.5倍阈值：20分
  - ≥ 1.0倍阈值：15分

### 风险等级

| 等级 | 说明 |
|------|------|
| 1-2 | 低风险：价格处于底部区域 |
| 3 | 中等风险：价格处于中间位置 |
| 4-5 | 高风险：价格已经较高 |

---

## 实战案例

### 案例1：典型的底部放量上涨

```rust
// 场景：股票在9.8-10.2之间横盘10天后，突然放量上涨至11.2

let result = strategy.analyze("000001.SZ", &data)?;

// 结果：
// - is_at_bottom: true
// - bottom_price: 9.8
// - volume_surge_ratio: 2.5 (2.5倍放量)
// - price_rise_pct: 14.3% (相对底部上涨14.3%)
// - strategy_signal: StrongBuy
// - signal_strength: 95
// - analysis_description: "处于底部区域（底部价格: 9.80），成交量放大 2.50 倍，价格上涨 14.30%，当前价格: 11.20"
```

### 案例2：假突破（不满足条件）

```rust
// 场景：价格上涨但成交量未放大

let result = strategy.analyze("600000.SH", &data)?;

// 结果：
// - is_at_bottom: true
// - volume_surge_ratio: 0.8 (成交量反而缩小)
// - price_rise_pct: 3.0%
// - strategy_signal: Hold
// - signal_strength: 55
// - analysis_description: "处于底部区域（底部价格: 8.50），成交量未明显放大（0.80倍），价格上涨 3.00%，当前价格: 8.76"
```

---

## 与选股服务集成

```rust
use service::stock_picker_service::StockPickerService;
use service::strategy::BottomVolumeSurgeStrategy;

// 创建选股服务
let picker = StockPickerService::new(db_conn);

// 创建策略
let mut strategy = BottomVolumeSurgeStrategy::aggressive();

// 执行选股
let start_date = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
let end_date = NaiveDate::from_ymd_opt(2024, 11, 11).unwrap();

let results = picker.pick_stocks(
    &mut strategy,
    &start_date,
    &end_date,
    Some(StrategySignal::Buy)  // 只返回买入及以上信号
).await?;

println!("找到 {} 只符合条件的股票", results.len());
```

---

## 策略优化建议

### 1. 结合其他指标

```rust
// 可以在分析后添加额外的过滤条件
let result = strategy.analyze("000001.SZ", &data)?;

if result.strategy_signal >= StrategySignal::Buy {
    // 进一步检查：
    // - RSI是否超卖后回升
    // - MACD是否金叉
    // - 是否突破关键阻力位
}
```

### 2. 动态调整参数

```rust
// 根据市场环境调整参数
let config = if is_bull_market {
    // 牛市使用激进配置
    BottomVolumeSurgeConfig {
        volume_surge_threshold: 1.3,
        price_rise_threshold: 1.5,
        ..Default::default()
    }
} else {
    // 熊市使用保守配置
    BottomVolumeSurgeConfig {
        volume_surge_threshold: 2.0,
        price_rise_threshold: 3.0,
        ..Default::default()
    }
};
```

### 3. 回测验证

```rust
// 在历史数据上验证策略效果
let backtest_results = backtest_strategy(
    &mut strategy,
    historical_data,
    initial_capital,
)?;

println!("胜率: {:.2}%", backtest_results.win_rate);
println!("收益率: {:.2}%", backtest_results.total_return);
```

---

## 注意事项

1. **数据质量**：确保输入数据按时间升序排列
2. **数据量要求**：至少需要 `analysis_period` 天的数据
3. **市场环境**：策略在震荡市和底部反转时效果最好
4. **风险控制**：建议结合止损策略使用
5. **参数调优**：根据不同市场和个股特性调整参数

---

## 常见问题

### Q1: 为什么信号强度很低？

**A**: 可能原因：
- 未处于底部区域（价格波动过大）
- 成交量未达到放大阈值
- 价格上涨幅度不足

### Q2: 如何提高信号的准确性？

**A**: 建议：
- 使用保守配置，提高信号质量
- 结合其他技术指标确认
- 关注成交量放大的持续性

### Q3: 策略适合什么类型的股票？

**A**: 最适合：
- 中小盘股票（流动性好）
- 有明显底部特征的股票
- 基本面良好但短期超跌的股票

---

## 扩展阅读

- [TradingStrategy Trait 文档](./traits.rs)
- [价量K线策略](./price_volume_candlestick_strategy.rs)
- [选股服务使用指南](../stock_picker_service.rs)
