# 价格强弱策略使用示例

## 策略说明

价格强弱策略通过分析K线形态（开盘价、最高价、最低价、收盘价的关系）来计算每天的价格强度分数，综合评估股票的强弱趋势。

## 核心特点

**每日强度评分维度（0-100分）**：

1. **实体强度（30分）**：实体占全天振幅的比例，实体越大越强
2. **上影线（20分）**：上影线越短越强，表示买盘强劲
3. **下影线（20分）**：下影线适中最佳（10%-30%），表示有支撑
4. **收盘位置（30分）**：收盘价越接近最高价越强
5. **涨跌调整（±10分）**：阳线加分，阴线减分

**综合评估**：
- 平均强度分数
- 强势天数占比
- 最近连续强势天数
- 强度趋势（上升/下降/平稳）

## 使用示例

```rust
use service::strategy::{
    PriceStrengthStrategy, PriceStrengthConfig,
    TradingStrategy, SecurityData, StrategyResult
};

// 1. 使用默认配置
let mut strategy = PriceStrengthStrategy::default();

// 2. 使用保守配置（更严格的筛选）
let mut conservative = PriceStrengthStrategy::conservative();

// 3. 使用激进配置（更宽松的筛选）
let mut aggressive = PriceStrengthStrategy::aggressive();

// 4. 自定义配置
let config = PriceStrengthConfig {
    analysis_period: 20,           // 分析最近20天
    min_avg_strength: 60.0,        // 平均强度至少60分
    min_strong_days_ratio: 0.6,    // 60%的天数为强势
    require_recent_strong_days: 3, // 最近3天连续强势
};
let mut custom = PriceStrengthStrategy::new(config);

// 5. 分析单只股票
let stock_data: Vec<SecurityData> = load_stock_data("000001.SZ");
match strategy.analyze("000001.SZ", &stock_data) {
    Ok(result) => {
        if let StrategyResult::PriceStrength(r) = result {
            println!("股票代码: {}", r.stock_code);
            println!("当前价格: {:.2}", r.current_price);
            println!("平均强度: {:.1}分", r.avg_strength_score);
            println!("强势天数: {}/{} ({:.1}%)", 
                r.strong_days_count, 
                r.daily_strength_scores.len(),
                r.strong_days_ratio * 100.0
            );
            println!("最近连续强势: {}天", r.recent_strong_days);
            println!("强度趋势: {}", r.strength_trend);
            println!("信号: {:?} ({}分)", r.strategy_signal, r.signal_strength);
            println!("描述: {}", r.analysis_description);
            
            // 查看每日强度分数
            println!("\n每日强度分数:");
            for (i, score) in r.daily_strength_scores.iter().enumerate() {
                println!("  第{}天: {:.1}分", i + 1, score);
            }
        }
    }
    Err(e) => eprintln!("分析失败: {}", e),
}

// 6. 批量筛选强势股票
let stocks = vec![
    ("000001.SZ".to_string(), load_stock_data("000001.SZ")),
    ("000002.SZ".to_string(), load_stock_data("000002.SZ")),
    ("600000.SH".to_string(), load_stock_data("600000.SH")),
];

let results = strategy.batch_analyze(&stocks);

// 筛选强势股票
let strong_stocks: Vec<_> = results.iter()
    .filter_map(|r| {
        if let StrategyResult::PriceStrength(ps) = r {
            if ps.signal_strength >= 70 {
                Some(ps)
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect();

println!("找到 {} 只强势股票", strong_stocks.len());
for stock in strong_stocks {
    println!("{}: 平均强度{:.1}分，{}天连续强势，趋势{}", 
        stock.stock_code, 
        stock.avg_strength_score,
        stock.recent_strong_days,
        stock.strength_trend
    );
}
```

## 配置参数说明

### 默认配置
```rust
PriceStrengthConfig {
    analysis_period: 20,           // 分析最近20天
    min_avg_strength: 60.0,        // 平均强度至少60分
    min_strong_days_ratio: 0.6,    // 60%的天数为强势
    require_recent_strong_days: 3, // 最近3天连续强势
}
```

### 保守配置
```rust
PriceStrengthConfig {
    min_avg_strength: 70.0,        // 平均强度至少70分
    min_strong_days_ratio: 0.7,    // 70%的天数为强势
    require_recent_strong_days: 5, // 最近5天连续强势
    ..Default::default()
}
```

### 激进配置
```rust
PriceStrengthConfig {
    min_avg_strength: 50.0,        // 平均强度至少50分
    min_strong_days_ratio: 0.5,    // 50%的天数为强势
    require_recent_strong_days: 2, // 最近2天连续强势
    ..Default::default()
}
```

## 单日强度计算示例

### 强势阳线（分数 > 80）
```
开盘: 10.00
最高: 11.00
最低: 9.90
收盘: 10.90

特征：
- 实体大（0.90），占振幅（1.10）的82%
- 上影线短（0.10），仅占9%
- 下影线短（0.10），仅占9%
- 收盘位置高，在全天区间的91%位置
- 阳线加分

强度分数: ~85分
```

### 弱势阴线（分数 < 40）
```
开盘: 10.00
最高: 10.10
最低: 9.00
收盘: 9.10

特征：
- 实体大（0.90），但是阴线
- 上影线短（0.10）
- 下影线短（0.10）
- 收盘位置低，仅在全天区间的9%位置
- 阴线减分

强度分数: ~30分
```

### 十字星（分数 = 50）
```
开盘: 10.00
最高: 10.00
最低: 10.00
收盘: 10.00

特征：一字板，无振幅
强度分数: 50分（中性）
```

## 信号评分规则

总分100分，评分维度：

1. **平均强度（40分）**：平均强度分数 / 100 * 40
2. **强势天数占比（30分）**：强势天数占比 * 30
3. **最近连续强势（20分）**：实际天数 / 要求天数 * 20
4. **强度趋势（10分）**：
   - 上升：10分
   - 平稳：5分
   - 下降：0分

### 信号等级

- **≥80分**：强烈买入 - 价格持续强势，趋势向上
- **≥65分**：买入 - 价格较强，值得关注
- **≥50分**：持有 - 价格中性，观望
- **<50分**：卖出 - 价格弱势，不建议买入

## 风险等级

基础风险等级为3级（中等），根据以下因素调整：

- **降低风险**：
  - 强度趋势上升：-1级
  - 平均强度 ≥ 80分：-1级
  
- **提高风险**：
  - 强度趋势下降：+1级
  - 平均强度 < 50分：+1级

最终风险等级范围：1-5级

## 结果字段说明

```rust
pub struct PriceStrengthResult {
    pub stock_code: String,              // 股票代码
    pub stock_name: Option<String>,      // 股票名称（可选）
    pub analysis_date: NaiveDate,        // 分析日期
    pub current_price: f64,              // 当前价格
    pub avg_strength_score: f64,         // 平均强度分数（0-100）
    pub strong_days_count: usize,        // 强势天数（≥60分）
    pub strong_days_ratio: f64,          // 强势天数占比（0-1）
    pub recent_strong_days: usize,       // 最近连续强势天数
    pub daily_strength_scores: Vec<f64>, // 每日强度分数列表
    pub strength_trend: String,          // 强度趋势（上升/下降/平稳）
    pub strategy_signal: StrategySignal, // 策略信号
    pub signal_strength: u8,             // 信号强度(0-100)
    pub analysis_description: String,    // 分析说明
    pub risk_level: u8,                  // 风险等级(1-5)
}
```

## 使用场景

### 1. 筛选持续强势股票
```rust
let mut strategy = PriceStrengthStrategy::conservative();
let results = strategy.batch_analyze(&all_stocks);

// 筛选平均强度高且趋势向上的股票
let strong_uptrend: Vec<_> = results.iter()
    .filter_map(|r| {
        if let StrategyResult::PriceStrength(ps) = r {
            if ps.avg_strength_score >= 70.0 
                && ps.strength_trend == "上升" 
                && ps.recent_strong_days >= 5 {
                Some(ps)
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect();
```

### 2. 监控价格强度变化
```rust
// 每天分析，跟踪强度变化
let mut strategy = PriceStrengthStrategy::default();
for (symbol, data) in stocks {
    if let Ok(StrategyResult::PriceStrength(r)) = strategy.analyze(&symbol, &data) {
        if r.strength_trend == "上升" && r.recent_strong_days >= 3 {
            println!("{}: 强度上升，最近{}天连续强势", symbol, r.recent_strong_days);
        }
    }
}
```

### 3. 结合其他策略
```rust
// 先用价格强度筛选，再用其他策略确认
let mut strength_strategy = PriceStrengthStrategy::default();
let mut volume_strategy = BottomVolumeSurgeStrategy::default();

// 筛选价格强势的股票
let strength_results = strength_strategy.batch_analyze(&stocks);
let strong_stocks: Vec<_> = strength_results.iter()
    .filter(|r| r.signal_strength() >= 70)
    .map(|r| r.stock_code())
    .collect();

// 在强势股票中找放量突破的
for code in strong_stocks {
    if let Ok(volume_result) = volume_strategy.analyze(code, &data) {
        if volume_result.signal_strength() >= 70 {
            println!("{}: 价格强势 + 放量突破", code);
        }
    }
}
```

## 注意事项

1. **K线质量**：确保数据准确，开高收低四个价格都正确
2. **分析周期**：建议至少20天，太短可能不准确
3. **市场环境**：牛市中强势股更有效，熊市中需谨慎
4. **结合成交量**：价格强势最好配合成交量放大
5. **趋势延续**：强度上升趋势的股票更值得关注

## 最佳实践

1. **保守配置选股**：使用保守配置筛选高质量强势股
2. **关注趋势**：优先选择强度趋势上升的股票
3. **连续强势**：最近连续强势天数越多越好
4. **分批建仓**：即使强势也要分批买入，控制风险
5. **设置止损**：根据风险等级设置合理止损位

## K线形态强度参考

| K线形态 | 典型特征 | 强度分数范围 |
|--------|---------|------------|
| 光头光脚阳线 | 无上下影线，实体大 | 85-95 |
| 大阳线 | 上下影线短，实体大 | 75-85 |
| 小阳线 | 实体小，收盘位置高 | 60-70 |
| 十字星 | 无实体或实体极小 | 45-55 |
| 小阴线 | 实体小，收盘位置低 | 30-40 |
| 大阴线 | 上下影线短，实体大 | 15-25 |
| 光头光脚阴线 | 无上下影线，实体大 | 5-15 |

**注意**：具体分数还会受上下影线比例、收盘位置等因素影响。
