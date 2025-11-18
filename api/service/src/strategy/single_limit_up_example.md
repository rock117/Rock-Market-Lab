# 单次涨停策略使用示例

## 策略概述

单次涨停策略用于筛选过去N天内**仅出现一次涨停**的股票，并分析涨停后的持续性表现。

### 核心逻辑

1. **单次涨停判断**：在指定周期内，股票仅出现一次涨停
   - 涨停阈值根据股票代码自动识别：
     - **688、300开头**：20%（科创板、创业板）
     - **920开头**：30%（北交所）
     - **其他**：10%（主板、中小板）
2. **上涨天数统计**：统计期间内上涨的天数，评估趋势持续性
3. **累计涨幅计算**：计算期间内的总涨幅，评估上涨力度

### 策略优势

- **避免游资炒作**：多次涨停往往是游资短期炒作，风险较高
- **关注持续性**：单次涨停后持续上涨，说明资金持续流入
- **趋势判断**：通过上涨天数和累计涨幅，判断上涨趋势的强度

## 配置参数

```rust
use service::strategy::{SingleLimitUpConfig, SingleLimitUpStrategy};

// 默认配置
let config = SingleLimitUpConfig {
    analysis_period: 20,        // 分析过去20个交易日
    limit_up_tolerance: 0.5,    // 容差0.5%
    min_up_days: 10,            // 至少10天上涨
    min_total_gain: 20.0,       // 累计涨幅至少20%
};

let mut strategy = SingleLimitUpStrategy::new(config);
```

### 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `analysis_period` | usize | 20 | 分析周期（交易日） |
| `limit_up_tolerance` | f64 | 0.5 | 涨停容差（%），允许略低于涨停价 |
| `min_up_days` | usize | 10 | 最小上涨天数要求 |
| `min_total_gain` | f64 | 20.0 | 最小累计涨幅要求（%） |

**注意**：涨停阈值会根据股票代码自动识别，无需手动配置

## 使用示例

### 基础用法

```rust
use service::strategy::{
    SingleLimitUpStrategy, 
    SingleLimitUpConfig,
    TradingStrategy,
    SecurityData,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建策略实例
    let config = SingleLimitUpConfig::default();
    let mut strategy = SingleLimitUpStrategy::new(config);
    
    // 2. 准备股票数据（从数据库或API获取）
    let stock_data: Vec<SecurityData> = fetch_stock_data("000001.SZ").await?;
    
    // 3. 执行分析
    let result = strategy.analyze("000001.SZ", &stock_data)?;
    
    // 4. 查看结果
    println!("股票代码: {}", result.stock_code());
    println!("策略信号: {:?}", result.strategy_signal());
    println!("信号强度: {}", result.signal_strength());
    println!("分析说明: {}", result.analysis_description());
    println!("风险等级: {}", result.risk_level());
    
    Ok(())
}
```

### 批量分析

```rust
use service::strategy::{SingleLimitUpStrategy, TradingStrategy};

async fn batch_analyze_stocks(stock_codes: Vec<String>) -> anyhow::Result<()> {
    let mut strategy = SingleLimitUpStrategy::new(SingleLimitUpConfig::default());
    
    // 准备数据
    let mut securities_data = Vec::new();
    for code in stock_codes {
        let data = fetch_stock_data(&code).await?;
        securities_data.push((code, data));
    }
    
    // 批量分析
    let results = strategy.batch_analyze(&securities_data);
    
    // 筛选强买入信号
    let strong_buy: Vec<_> = results.iter()
        .filter(|r| r.signal_strength() >= 80)
        .collect();
    
    println!("找到 {} 只强买入信号的股票", strong_buy.len());
    for result in strong_buy {
        println!("{}: {}", result.stock_code(), result.analysis_description());
    }
    
    Ok(())
}
```

### 自定义配置

```rust
// 创建自定义配置 - 适用于创业板（20%涨停）
let custom_config = SingleLimitUpConfig {
    analysis_period: 30,        // 分析过去30天
    limit_up_threshold: 19.9,   // 创业板涨停阈值
    limit_up_tolerance: 0.5,    // 容差
    min_up_days: 15,            // 至少15天上涨
    min_total_gain: 30.0,       // 累计涨幅至少30%
};

let mut strategy = SingleLimitUpStrategy::new(custom_config);
```

## 结果解读

### SingleLimitUpResult 字段

```rust
pub struct SingleLimitUpResult {
    pub stock_code: String,           // 股票代码
    pub analysis_date: NaiveDate,     // 分析日期
    pub current_price: f64,           // 当前价格
    pub strategy_signal: StrategySignal,  // 策略信号
    pub signal_strength: u8,          // 信号强度 (0-100)
    pub analysis_description: String, // 分析说明
    pub risk_level: u8,               // 风险等级 (1-5)
    
    // 策略特有字段
    pub limit_up_count: usize,        // 涨停次数
    pub limit_up_date: String,        // 涨停日期
    pub up_days: usize,               // 上涨天数
    pub total_gain_pct: f64,          // 累计涨幅（%）
    pub analysis_period: usize,       // 分析周期
}
```

### 信号强度评分规则

总分100分，由以下部分组成：

1. **涨停次数评分（40分）**
   - 仅一次涨停：40分
   - 无涨停：0分
   - 多次涨停：10分（风险较高）

2. **上涨天数评分（30分）**
   - ≥70%天数上涨：30分
   - 50%-70%天数上涨：20分
   - 达到最小要求：10分

3. **累计涨幅评分（30分）**
   - ≥2倍最小要求：30分
   - ≥1.5倍最小要求：20分
   - 达到最小要求：15分

### 信号类型

- **StrongBuy（强烈买入）**：信号强度 ≥ 80
- **Buy（买入）**：信号强度 ≥ 60
- **Hold（持有）**：信号强度 ≥ 40
- **Sell（卖出）**：信号强度 ≥ 20
- **StrongSell（强烈卖出）**：信号强度 < 20

### 风险等级

- **2（低风险）**：累计涨幅 < 10%
- **3（中等风险）**：累计涨幅 10%-50%
- **4（高风险）**：累计涨幅 > 50% 或多次涨停

## 实际应用场景

### 场景1：寻找稳健上涨股票

```rust
// 配置：关注稳健上涨，避免过度炒作
let config = SingleLimitUpConfig {
    analysis_period: 20,
    limit_up_threshold: 9.9,
    limit_up_tolerance: 0.5,
    min_up_days: 12,            // 要求更多上涨天数
    min_total_gain: 15.0,       // 降低涨幅要求
};
```

### 场景2：寻找强势突破股票

```rust
// 配置：关注强势突破
let config = SingleLimitUpConfig {
    analysis_period: 10,        // 短周期
    limit_up_tolerance: 0.5,
    min_up_days: 6,
    min_total_gain: 25.0,       // 要求更高涨幅
};
```

### 场景3：创业板/科创板股票筛选

```rust
// 配置：创业板(300)和科创板(688)会自动识别20%涨停
let config = SingleLimitUpConfig {
    analysis_period: 20,
    limit_up_tolerance: 0.5,    // 容差
    min_up_days: 10,
    min_total_gain: 30.0,       // 要求更高涨幅
};

// 分析创业板股票 - 自动使用20%涨停阈值
strategy.analyze("300001.SZ", &data)?;

// 分析科创板股票 - 自动使用20%涨停阈值
strategy.analyze("688001.SH", &data)?;
```

### 场景4：北交所股票筛选

```rust
// 配置：北交所(920)会自动识别30%涨停
let config = SingleLimitUpConfig {
    analysis_period: 20,
    limit_up_tolerance: 0.5,
    min_up_days: 10,
    min_total_gain: 40.0,       // 北交所波动大，要求更高涨幅
};

// 分析北交所股票 - 自动使用30%涨停阈值
strategy.analyze("920001.BJ", &data)?;
```

## 注意事项

1. **数据质量**：确保输入的股票数据完整且准确
2. **市场环境**：在不同市场环境下，参数需要相应调整
3. **风险控制**：即使信号强度高，也要结合其他指标和基本面分析
4. **涨停板制度**（已自动识别）：
   - **主板、中小板**：10%
   - **创业板(300)、科创板(688)**：20%
   - **北交所(920)**：30%
   - 注意：ST股票的5%涨停暂未支持
5. **回测验证**：使用历史数据回测，验证策略有效性

## 完整示例

```rust
use service::strategy::{
    SingleLimitUpStrategy,
    SingleLimitUpConfig,
    TradingStrategy,
    SecurityData,
    StrategySignal,
};
use chrono::NaiveDate;

async fn find_single_limit_up_stocks() -> anyhow::Result<()> {
    // 1. 配置策略
    let config = SingleLimitUpConfig {
        analysis_period: 20,
        limit_up_tolerance: 0.5,
        min_up_days: 10,
        min_total_gain: 20.0,
    };
    
    let mut strategy = SingleLimitUpStrategy::new(config);
    
    // 2. 获取股票列表（包含不同板块）
    let stock_codes = vec![
        "000001.SZ".to_string(),  // 主板 - 10%涨停
        "600000.SH".to_string(),  // 主板 - 10%涨停
        "300001.SZ".to_string(),  // 创业板 - 20%涨停
        "688001.SH".to_string(),  // 科创板 - 20%涨停
        "920001.BJ".to_string(),  // 北交所 - 30%涨停
    ];
    
    // 3. 批量分析
    let mut results = Vec::new();
    for code in stock_codes {
        let data = fetch_stock_data(&code).await?;
        match strategy.analyze(&code, &data) {
            Ok(result) => {
                // 只保留买入信号
                if matches!(
                    result.strategy_signal(),
                    StrategySignal::Buy | StrategySignal::StrongBuy
                ) {
                    results.push(result);
                }
            }
            Err(e) => {
                eprintln!("分析 {} 失败: {}", code, e);
            }
        }
    }
    
    // 4. 按信号强度排序
    results.sort_by(|a, b| b.signal_strength().cmp(&a.signal_strength()));
    
    // 5. 输出结果
    println!("\n=== 单次涨停策略分析结果 ===\n");
    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", i + 1, result.stock_code());
        println!("   信号: {:?} (强度: {})", 
            result.strategy_signal(), 
            result.signal_strength()
        );
        println!("   {}", result.analysis_description());
        println!("   风险等级: {}/5", result.risk_level());
        
        // 访问策略特有字段
        if let StrategyResult::SingleLimitUp(detail) = result {
            println!("   涨停日期: {}", detail.limit_up_date);
            println!("   上涨天数: {}/{}", 
                detail.up_days, 
                detail.analysis_period
            );
            println!("   累计涨幅: {:.2}%", detail.total_gain_pct);
        }
        println!();
    }
    
    Ok(())
}

// 模拟数据获取函数
async fn fetch_stock_data(code: &str) -> anyhow::Result<Vec<SecurityData>> {
    // 实际应用中，从数据库或API获取数据
    // 这里仅作示例
    Ok(vec![])
}
```

## 总结

单次涨停策略适合寻找：
- ✅ 有资金持续流入的股票
- ✅ 趋势稳健向上的股票
- ✅ 避免过度炒作的股票

通过合理配置参数和结合其他分析方法，可以有效提高选股成功率。
