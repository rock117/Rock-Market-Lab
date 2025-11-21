# 连续强势股策略 (Consecutive Strong Strategy)

## 策略概述

连续强势股策略用于筛选出近n天**每天都是强势股**的股票。

### 强势股定义

一个交易日被认为是"强势"，当且仅当满足以下两个条件：
1. **收盘价 > 最低价**（表示没有收在最低点）
2. **收盘价 > 开盘价**（表示当日上涨，收阳线）

这是一个简单直观的强势定义，反映了当日多方力量占优的情况。

## 配置参数

### ConsecutiveStrongConfig

```rust
pub struct ConsecutiveStrongConfig {
    /// 分析周期（天数）- 默认 5 天
    pub analysis_period: usize,
    
    /// 要求的最少连续强势天数 - 默认等于分析周期（即全部天数都强势）
    pub min_consecutive_days: usize,
}
```

### 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `analysis_period` | usize | 5 | 分析的天数窗口 |
| `min_consecutive_days` | usize | 5 | 要求的最少连续强势天数（从最近一天往前数） |

### 注意事项

- `min_consecutive_days` 不能大于 `analysis_period`
- 当 `min_consecutive_days` = `analysis_period` 时，要求全部天数都强势
- 当 `min_consecutive_days` < `analysis_period` 时，允许部分天数不强势，但最近的 `min_consecutive_days` 天必须连续强势

## 使用示例

### 快速开始 - 使用预设配置

策略提供了几个预设配置函数，可以快速创建常用场景的配置：

```rust
use service::strategy::{ConsecutiveStrongStrategy, ConsecutiveStrongConfig, TradingStrategy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 方式1：3天连续强势
    let config = ConsecutiveStrongStrategy::three_days();
    let mut strategy = ConsecutiveStrongStrategy::new(config);
    
    // 方式2：5天连续强势（默认）
    let config = ConsecutiveStrongStrategy::five_days();
    let mut strategy = ConsecutiveStrongStrategy::new(config);
    
    // 方式3：10天连续强势
    let config = ConsecutiveStrongStrategy::ten_days();
    let mut strategy = ConsecutiveStrongStrategy::new(config);
    
    // 方式4：宽松配置（10天中至少8天强势）
    let config = ConsecutiveStrongStrategy::relaxed();
    let mut strategy = ConsecutiveStrongStrategy::new(config);
    
    // 准备数据并分析
    let data = fetch_stock_data("000001.SZ").await?;
    let result = strategy.analyze("000001.SZ", &data)?;
    
    println!("股票代码: {}", result.stock_code());
    println!("策略信号: {:?}", result.strategy_signal());
    println!("信号强度: {}", result.signal_strength());
    println!("分析说明: {}", result.analysis_description());
    
    Ok(())
}
```

### 预设配置对比

| 配置函数 | 分析周期 | 最少连续天数 | 适用场景 |
|---------|---------|-------------|---------|
| `three_days()` | 3天 | 3天 | 短期强势，快速筛选 |
| `five_days()` | 5天 | 5天 | 中短期强势（默认） |
| `ten_days()` | 10天 | 10天 | 中期强势，要求严格 |
| `relaxed()` | 10天 | 8天 | 宽松筛选，允许少量回调 |

### 基础用法

```rust
use service::strategy::{ConsecutiveStrongStrategy, ConsecutiveStrongConfig, TradingStrategy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建配置
    let config = ConsecutiveStrongConfig {
        analysis_period: 5,
        min_consecutive_days: 5,
    };
    
    // 2. 创建策略实例
    let mut strategy = ConsecutiveStrongStrategy::new(config);
    
    // 3. 准备数据（需要至少5天的数据）
    let data = fetch_stock_data("000001.SZ").await?;
    
    // 4. 执行分析
    let result = strategy.analyze("000001.SZ", &data)?;
    
    // 5. 处理结果
    if let StrategyResult::ConsecutiveStrong(r) = result {
        println!("股票代码: {}", r.stock_code);
        println!("连续强势天数: {}", r.consecutive_strong_days);
        println!("强势占比: {:.1}%", r.strong_days_ratio * 100.0);
        println!("累计涨幅: {:.2}%", r.total_gain_pct);
        println!("平均日涨幅: {:.2}%", r.avg_daily_gain);
        println!("策略信号: {:?}", r.strategy_signal);
        println!("信号强度: {}", r.signal_strength);
    }
    
    Ok(())
}
```

## 结果结构

### ConsecutiveStrongResult

```rust
pub struct ConsecutiveStrongResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 分析周期
    pub analysis_period: usize,
    
    /// 实际连续强势天数（从最近一天往前数）
    pub consecutive_strong_days: usize,
    
    /// 强势天数占比（0-1）
    pub strong_days_ratio: f64,
    
    /// 每日是否强势（true=强势，false=弱势）
    pub daily_strong_flags: Vec<bool>,
    
    /// 每日涨幅（%）
    pub daily_changes: Vec<f64>,
    
    /// 累计涨幅（%）
    pub total_gain_pct: f64,
    
    /// 平均日涨幅（%）
    pub avg_daily_gain: f64,
    
    /// 策略信号
    pub strategy_signal: StrategySignal,
    
    /// 信号强度 (0-100)
    pub signal_strength: u8,
    
    /// 分析说明
    pub analysis_description: String,
    
    /// 风险等级 (1-5)
    pub risk_level: u8,
}
```

### 字段说明

- **consecutive_strong_days**: 从最近一天往前数的连续强势天数
- **strong_days_ratio**: 强势天数占总分析天数的比例
- **daily_strong_flags**: 每日是否强势的布尔数组，可用于详细分析
- **daily_changes**: 每日涨跌幅，可用于计算累计收益
- **total_gain_pct**: 分析周期内的累计涨幅
- **avg_daily_gain**: 平均每日涨幅

## 配置示例

### 场景1：严格筛选（5天全部强势）

```rust
let config = ConsecutiveStrongConfig {
    analysis_period: 5,
    min_consecutive_days: 5,
};
```

**适用场景**：寻找短期内持续强势的股票，适合短线交易。

### 场景2：中期强势（10天全部强势）

```rust
let config = ConsecutiveStrongConfig {
    analysis_period: 10,
    min_consecutive_days: 10,
};
```

**适用场景**：寻找中期趋势明确的强势股，适合波段操作。

### 场景3：宽松筛选（10天中至少8天强势）

```rust
let config = ConsecutiveStrongConfig {
    analysis_period: 10,
    min_consecutive_days: 8,
};
```

**适用场景**：允许少量回调，但整体保持强势，适合寻找稳健上涨的股票。

### 场景4：超短线（3天连续强势）

```rust
let config = ConsecutiveStrongConfig {
    analysis_period: 3,
    min_consecutive_days: 3,
};
```

**适用场景**：快速筛选近期爆发的股票，适合超短线操作。

## 信号强度评分

策略根据以下维度计算信号强度（0-100分）：

1. **连续天数评分（40分）**
   - 连续强势天数占分析周期的比例
   - 连续天数越多，得分越高

2. **强势占比评分（30分）**
   - 强势天数占总天数的比例
   - 占比越高，得分越高

3. **累计涨幅评分（20分）**
   - >20%: 20分
   - >10%: 15分
   - >5%: 10分
   - >0%: 5分
   - ≤0%: 0分

4. **平均日涨幅评分（10分）**
   - >3%: 10分
   - >2%: 8分
   - >1%: 5分
   - >0%: 3分
   - ≤0%: 0分

### 信号等级

- **80-100分**: StrongBuy（强烈买入）
- **65-79分**: Buy（买入）
- **50-64分**: Hold（持有）
- **<50分**: Sell（卖出）

## 风险等级

策略根据涨幅和连续天数评估风险：

- **风险等级1-2**: 涨幅大（>15%），连续天数多，风险较低
- **风险等级3**: 中等涨幅（8-15%），中等风险
- **风险等级4-5**: 涨幅小（<8%），风险较高

## 与 StockPickerService 集成

### 使用动态策略筛选

```rust
use service::stock_picker_service::StockPickerService;
use chrono::NaiveDate;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = get_database_connection().await?;
    let service = StockPickerService::new(db);
    
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 20).unwrap();
    
    // 使用默认配置
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "consecutive_strong",
        None
    ).await?;
    
    // 使用自定义配置
    let config = json!({
        "analysis_period": 10,
        "min_consecutive_days": 8
    });
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "consecutive_strong",
        Some(config)
    ).await?;
    
    // 处理结果
    for stock in results {
        println!("股票: {} - {}", stock.ts_code, stock.name);
        println!("信号: {:?} (强度: {})", stock.signal, stock.signal_strength);
        println!("描述: {}", stock.description);
        println!("---");
    }
    
    Ok(())
}
```

### 批量分析示例

```rust
// 筛选近5天连续强势的科技股
let config = ConsecutiveStrongConfig {
    analysis_period: 5,
    min_consecutive_days: 5,
};

let mut strategy = ConsecutiveStrongStrategy::new(config);

// 准备多只股票的数据
let securities_data = vec![
    ("000001.SZ".to_string(), fetch_stock_data("000001.SZ").await?),
    ("600000.SH".to_string(), fetch_stock_data("600000.SH").await?),
    ("300750.SZ".to_string(), fetch_stock_data("300750.SZ").await?),
];

// 批量分析
let results = strategy.batch_analyze(&securities_data);

// 筛选出强烈买入信号的股票
let strong_buys: Vec<_> = results.iter()
    .filter(|r| r.signal_strength() >= 80)
    .collect();

println!("找到 {} 只连续强势股", strong_buys.len());
```

## 实战建议

### 1. 短线操作（3-5天）

```rust
let config = ConsecutiveStrongStrategy::five_days();
```

- **优点**: 快速捕捉短期强势股
- **风险**: 可能追高，需要及时止损
- **建议**: 配合成交量和技术指标使用

### 2. 波段操作（10天）

```rust
let config = ConsecutiveStrongStrategy::ten_days();
```

- **优点**: 趋势更明确，假信号较少
- **风险**: 可能错过早期机会
- **建议**: 适合趋势交易者

### 3. 稳健操作（宽松配置）

```rust
let config = ConsecutiveStrongStrategy::relaxed();
```

- **优点**: 允许正常回调，减少假信号
- **风险**: 信号较少
- **建议**: 适合稳健型投资者

### 4. 组合使用

```rust
// 先用宽松配置筛选候选股
let relaxed_config = ConsecutiveStrongStrategy::relaxed();
let candidates = screen_stocks(relaxed_config).await?;

// 再用严格配置精选
let strict_config = ConsecutiveStrongStrategy::ten_days();
let final_picks = filter_candidates(candidates, strict_config).await?;
```

## 注意事项

1. **数据质量**: 确保输入数据完整且准确
2. **市场环境**: 在震荡市中，连续强势股较少；在牛市中，信号较多
3. **止损策略**: 即使是连续强势股，也要设置止损位
4. **结合其他指标**: 建议配合成交量、技术指标等综合判断
5. **回测验证**: 在实盘使用前，建议先进行历史回测

## 常见问题

### Q1: 为什么有些股票连续强势但信号强度不高？

A: 信号强度不仅看连续天数，还看涨幅。如果连续强势但涨幅很小（如每天只涨0.1%），信号强度会较低。

### Q2: 宽松配置和严格配置如何选择？

A: 
- **严格配置**（min_consecutive_days = analysis_period）：适合寻找最强势的股票，信号少但质量高
- **宽松配置**（min_consecutive_days < analysis_period）：适合寻找稳健上涨的股票，信号多但需要进一步筛选

### Q3: 如何处理涨停板？

A: 涨停板当天通常满足强势定义（收盘>开盘且收盘>最低），会被计入强势天数。但连续涨停的股票风险较高，建议谨慎对待。

### Q4: 策略是否适用于下跌行情？

A: 该策略专注于筛选强势股，在下跌行情中信号会很少。如果需要筛选弱势股（做空），可以反向使用逻辑（收盘<开盘且收盘<最高）。

## 总结

连续强势股策略是一个简单直观的趋势跟踪策略，通过识别持续上涨的股票来捕捉市场机会。它的核心优势是：

✅ **逻辑简单**: 强势定义清晰，易于理解和验证  
✅ **趋势明确**: 连续强势表示趋势稳定  
✅ **灵活配置**: 可根据交易风格调整参数  
✅ **量化评分**: 提供客观的信号强度评分  

但也要注意：

⚠️ **追高风险**: 可能在高位买入  
⚠️ **滞后性**: 趋势已经形成后才产生信号  
⚠️ **市场依赖**: 在震荡市中效果较差  

建议将此策略作为选股工具之一，结合其他分析方法综合判断。
