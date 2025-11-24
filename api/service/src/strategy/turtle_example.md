# 海龟交易策略 (Turtle Trading Strategy)

## 策略概述

海龟交易策略是由传奇交易员理查德·丹尼斯（Richard Dennis）在1983年创立的经典趋势跟踪策略。该策略通过价格突破来识别趋势，使用ATR（平均真实波动幅度）来管理风险和仓位。

### 核心理念

1. **趋势跟踪**：顺势而为，不预测市场
2. **突破入场**：价格突破N日高点时买入
3. **ATR止损**：使用波动率来设置止损位
4. **金字塔加仓**：趋势延续时逐步加仓
5. **纪律执行**：严格遵守规则，不凭感觉交易

## 策略原理

### 入场规则

当价格突破过去N天的最高价时，产生买入信号：
```
当前价格 > N日最高价 → 买入
```

### 出场规则

当价格跌破过去M天的最低价时，产生卖出信号：
```
当前价格 < M日最低价 → 卖出
```

### ATR止损

止损位 = 入场价 - (ATR × 止损倍数)

ATR（Average True Range）衡量市场波动性，使止损适应不同市场环境。

### 金字塔加仓

每当价格上涨0.5个ATR，可以加仓一次，最多加仓4次。

## 配置参数

### TurtleConfig

```rust
pub struct TurtleConfig {
    /// 入场突破周期（天数）- 默认 20 天
    pub entry_breakout_period: usize,
    
    /// 出场突破周期（天数）- 默认 10 天
    pub exit_breakout_period: usize,
    
    /// ATR周期（天数）- 默认 20 天
    pub atr_period: usize,
    
    /// 止损ATR倍数 - 默认 2.0
    pub stop_loss_atr_multiple: f64,
    
    /// 加仓ATR倍数 - 默认 0.5
    pub pyramid_atr_multiple: f64,
    
    /// 最大加仓次数 - 默认 4 次
    pub max_pyramid_units: usize,
    
    /// 是否使用系统2 - 默认 false
    pub use_system2: bool,
}
```

### 参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `entry_breakout_period` | usize | 20 | 入场突破周期，价格突破N日最高价时买入 |
| `exit_breakout_period` | usize | 10 | 出场突破周期，价格跌破N日最低价时卖出 |
| `atr_period` | usize | 20 | 计算ATR的周期 |
| `stop_loss_atr_multiple` | f64 | 2.0 | 止损距离 = ATR × 此倍数 |
| `pyramid_atr_multiple` | f64 | 0.5 | 加仓间隔 = ATR × 此倍数 |
| `max_pyramid_units` | usize | 4 | 最大加仓次数（包括首次建仓） |
| `use_system2` | bool | false | 是否使用系统2（55天突破） |

### 系统1 vs 系统2

原始海龟策略包含两个系统：

- **系统1**（20天突破）：更激进，信号更频繁，适合短期趋势
- **系统2**（55天突破）：更保守，信号更可靠，适合长期趋势

## 使用示例

### 快速开始 - 使用预设配置

```rust
use service::strategy::{TurtleStrategy, TurtleConfig, TradingStrategy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 方式1：系统1（20天突破，更激进）
    let config = TurtleStrategy::system1();
    let mut strategy = TurtleStrategy::new(config);
    
    // 方式2：系统2（55天突破，更保守）
    let config = TurtleStrategy::system2();
    let mut strategy = TurtleStrategy::new(config);
    
    // 方式3：保守配置（更大的止损空间）
    let config = TurtleStrategy::conservative();
    let mut strategy = TurtleStrategy::new(config);
    
    // 方式4：激进配置（更小的止损，更多加仓）
    let config = TurtleStrategy::aggressive();
    let mut strategy = TurtleStrategy::new(config);
    
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

| 配置函数 | 入场周期 | 出场周期 | 止损倍数 | 加仓倍数 | 最大加仓 | 适用场景 |
|---------|---------|---------|---------|---------|---------|---------|
| `system1()` | 20天 | 10天 | 2.0 | 0.5 | 4次 | 标准配置，平衡 |
| `system2()` | 55天 | 20天 | 2.0 | 0.5 | 4次 | 长期趋势，保守 |
| `conservative()` | 30天 | 15天 | 3.0 | 1.0 | 3次 | 更大止损，稳健 |
| `aggressive()` | 10天 | 5天 | 1.5 | 0.3 | 5次 | 短期趋势，激进 |

### 基础用法

```rust
use service::strategy::{TurtleStrategy, TurtleConfig, TradingStrategy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建配置
    let config = TurtleConfig {
        entry_breakout_period: 20,
        exit_breakout_period: 10,
        atr_period: 20,
        stop_loss_atr_multiple: 2.0,
        pyramid_atr_multiple: 0.5,
        max_pyramid_units: 4,
        use_system2: false,
    };
    
    // 2. 创建策略实例
    let mut strategy = TurtleStrategy::new(config);
    
    // 3. 准备数据（需要至少30天的数据）
    let data = fetch_stock_data("000001.SZ").await?;
    
    // 4. 执行分析
    let result = strategy.analyze("000001.SZ", &data)?;
    
    // 5. 处理结果
    if let StrategyResult::Turtle(r) = result {
        println!("股票代码: {}", r.stock_code);
        println!("当前价格: {:.2}", r.current_price);
        println!("入场突破价: {:.2}", r.entry_breakout_price);
        println!("出场突破价: {:.2}", r.exit_breakout_price);
        println!("是否突破入场: {}", r.is_entry_breakout);
        println!("ATR: {:.2}", r.atr);
        println!("建议止损价: {:.2}", r.stop_loss_price);
        println!("加仓价格: {:?}", r.pyramid_prices);
        println!("策略信号: {:?}", r.strategy_signal);
        println!("信号强度: {}", r.signal_strength);
    }
    
    Ok(())
}
```

## 结果结构

### TurtleResult

```rust
pub struct TurtleResult {
    /// 股票代码
    pub stock_code: String,
    
    /// 分析日期
    pub analysis_date: NaiveDate,
    
    /// 当前价格
    pub current_price: f64,
    
    /// 入场突破价（N日最高价）
    pub entry_breakout_price: f64,
    
    /// 出场突破价（N日最低价）
    pub exit_breakout_price: f64,
    
    /// 是否突破入场
    pub is_entry_breakout: bool,
    
    /// 是否跌破出场
    pub is_exit_breakout: bool,
    
    /// ATR值（平均真实波动幅度）
    pub atr: f64,
    
    /// 建议止损价
    pub stop_loss_price: f64,
    
    /// 建议加仓价格列表
    pub pyramid_prices: Vec<f64>,
    
    /// 当前可加仓次数
    pub available_pyramid_units: usize,
    
    /// 距离入场突破的百分比（负数表示未突破）
    pub distance_to_entry_pct: f64,
    
    /// 距离出场突破的百分比（正数表示未跌破）
    pub distance_to_exit_pct: f64,
    
    /// 趋势强度（0-100）
    pub trend_strength: u8,
    
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

### 关键字段说明

- **entry_breakout_price**: 过去N天的最高价，突破此价格产生买入信号
- **exit_breakout_price**: 过去M天的最低价，跌破此价格产生卖出信号
- **atr**: 平均真实波动幅度，用于计算止损和加仓位置
- **stop_loss_price**: 建议的止损价格，基于ATR计算
- **pyramid_prices**: 建议的加仓价格列表，每个价格相隔0.5个ATR
- **distance_to_entry_pct**: 当前价格距离入场线的百分比，正数表示已突破
- **distance_to_exit_pct**: 当前价格距离出场线的百分比，正数表示安全

## 配置示例

### 场景1：标准海龟系统1

```rust
let config = TurtleConfig {
    entry_breakout_period: 20,
    exit_breakout_period: 10,
    atr_period: 20,
    stop_loss_atr_multiple: 2.0,
    pyramid_atr_multiple: 0.5,
    max_pyramid_units: 4,
    use_system2: false,
};
```

**适用场景**：经典配置，适合大多数市场环境。

### 场景2：长期趋势跟踪

```rust
let config = TurtleConfig {
    entry_breakout_period: 55,
    exit_breakout_period: 20,
    atr_period: 20,
    stop_loss_atr_multiple: 2.0,
    pyramid_atr_multiple: 0.5,
    max_pyramid_units: 4,
    use_system2: true,
};
```

**适用场景**：捕捉大级别趋势，减少假突破，适合长线投资者。

### 场景3：短期趋势交易

```rust
let config = TurtleConfig {
    entry_breakout_period: 10,
    exit_breakout_period: 5,
    atr_period: 10,
    stop_loss_atr_multiple: 1.5,
    pyramid_atr_multiple: 0.3,
    max_pyramid_units: 5,
    use_system2: false,
};
```

**适用场景**：快速捕捉短期趋势，适合活跃交易者。

### 场景4：稳健保守型

```rust
let config = TurtleConfig {
    entry_breakout_period: 30,
    exit_breakout_period: 15,
    atr_period: 20,
    stop_loss_atr_multiple: 3.0,
    pyramid_atr_multiple: 1.0,
    max_pyramid_units: 3,
    use_system2: false,
};
```

**适用场景**：更大的止损空间，更少的加仓次数，适合稳健型投资者。

## 信号强度评分

策略根据以下维度计算信号强度（0-100分）：

### 突破入场时（满足条件）

1. **突破幅度评分（30分）**
   - >5%: 30分
   - >3%: 25分
   - >1%: 20分
   - ≤1%: 15分

2. **趋势强度评分（40分）**
   - 基于价格位置、上涨天数占比等综合计算
   - 趋势越强，得分越高

3. **波动率评分（20分）**
   - ATR占价格比例 <2%: 20分（低波动）
   - ATR占价格比例 2-4%: 15分（中等波动）
   - ATR占价格比例 4-6%: 10分（高波动）
   - ATR占价格比例 >6%: 5分（极高波动）

4. **安全边际评分（10分）**
   - 距离出场线 >20%: 10分
   - 距离出场线 10-20%: 7分
   - 距离出场线 5-10%: 4分
   - 距离出场线 <5%: 0分

### 信号等级

- **80-100分**: StrongBuy（强烈买入）
- **65-79分**: Buy（买入）
- **50-64分**: Hold（持有）
- **<50分**: Sell（卖出）

### 特殊情况

- **跌破出场线**: 立即返回 StrongSell，信号强度0分

## 风险等级

策略根据波动率评估风险：

- **风险等级2**: ATR占价格比例 <3%（低波动）
- **风险等级3**: ATR占价格比例 3-5%（中等波动）
- **风险等级4**: ATR占价格比例 >5%（高波动）
- **风险等级5**: 跌破出场线（极高风险）

## 实战应用

### 1. 完整交易流程

```rust
// 1. 初始化策略
let config = TurtleStrategy::system1();
let mut strategy = TurtleStrategy::new(config);

// 2. 获取数据
let data = fetch_stock_data("000001.SZ").await?;

// 3. 分析信号
let result = strategy.analyze("000001.SZ", &data)?;

if let StrategyResult::Turtle(r) = result {
    // 4. 判断入场
    if r.is_entry_breakout && r.signal_strength >= 65 {
        println!("买入信号！");
        println!("建议入场价: {:.2}", r.current_price);
        println!("止损价: {:.2}", r.stop_loss_price);
        println!("风险: {:.2}%", 
                 (r.current_price - r.stop_loss_price) / r.current_price * 100.0);
        
        // 5. 加仓计划
        println!("加仓价格:");
        for (i, price) in r.pyramid_prices.iter().enumerate() {
            println!("  第{}次加仓: {:.2}", i + 2, price);
        }
    }
    
    // 6. 判断出场
    if r.is_exit_breakout {
        println!("卖出信号！立即止损");
    }
}
```

### 2. 仓位管理

海龟策略的核心是仓位管理：

```rust
// 假设账户总资金
let account_value = 1000000.0;

// 单位风险（账户的1%）
let unit_risk = account_value * 0.01;

// 单位规模 = 单位风险 / ATR
let unit_size = unit_risk / r.atr;

// 首次建仓（1个单位）
let initial_shares = (unit_size / r.current_price).floor();
println!("首次建仓: {} 股", initial_shares);

// 后续加仓（每次1个单位）
for (i, price) in r.pyramid_prices.iter().enumerate() {
    let add_shares = (unit_size / price).floor();
    println!("第{}次加仓: {} 股 @ {:.2}", i + 2, add_shares, price);
}
```

### 3. 批量筛选

```rust
// 筛选突破入场的股票
let config = TurtleStrategy::system1();
let mut strategy = TurtleStrategy::new(config);

let mut breakout_stocks = Vec::new();

for stock_code in stock_list {
    let data = fetch_stock_data(&stock_code).await?;
    let result = strategy.analyze(&stock_code, &data)?;
    
    if let StrategyResult::Turtle(r) = result {
        if r.is_entry_breakout && r.signal_strength >= 70 {
            breakout_stocks.push((stock_code, r));
        }
    }
}

// 按信号强度排序
breakout_stocks.sort_by(|a, b| b.1.signal_strength.cmp(&a.1.signal_strength));

println!("找到 {} 只突破股票", breakout_stocks.len());
for (code, result) in breakout_stocks.iter().take(10) {
    println!("{}: 信号强度{}, ATR={:.2}", 
             code, result.signal_strength, result.atr);
}
```

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
    let end_date = NaiveDate::from_ymd_opt(2024, 11, 24).unwrap();
    
    // 使用默认配置（系统1）
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "turtle",
        None
    ).await?;
    
    // 使用自定义配置
    let config = json!({
        "entry_breakout_period": 30,
        "exit_breakout_period": 15,
        "atr_period": 20,
        "stop_loss_atr_multiple": 2.5,
        "pyramid_atr_multiple": 0.5,
        "max_pyramid_units": 4,
        "use_system2": false
    });
    
    let results = service.pick_stocks(
        &start_date,
        &end_date,
        "turtle",
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

## 策略优势

### ✅ 优点

1. **趋势跟踪**: 能够捕捉大级别趋势，获取丰厚利润
2. **风险管理**: 使用ATR止损，适应市场波动
3. **纪律性强**: 规则明确，易于执行
4. **久经考验**: 历史业绩优秀，经过实战检验
5. **仓位管理**: 金字塔加仓，让利润奔跑
6. **适应性强**: 适用于各种市场和品种

### ⚠️ 注意事项

1. **震荡市不利**: 在横盘震荡市场中容易产生假突破
2. **回撤较大**: 趋势跟踪策略通常有较大回撤
3. **胜率不高**: 胜率可能只有40-50%，但盈亏比高
4. **需要耐心**: 等待趋势形成需要时间
5. **资金管理**: 严格的仓位管理是成功的关键

## 实战建议

### 1. 市场选择

- **适合**: 趋势性强的市场（牛市、熊市）
- **不适合**: 长期横盘震荡的市场

### 2. 品种选择

- **优选**: 流动性好、波动适中的股票
- **避免**: 流动性差、波动极大的股票

### 3. 资金管理

```rust
// 单次风险不超过账户的1-2%
let max_risk_per_trade = account_value * 0.01;

// 根据ATR计算仓位
let position_size = max_risk_per_trade / (atr * stop_loss_multiple);
```

### 4. 组合管理

- 同时持有4-6个不相关的品种
- 分散风险，提高稳定性
- 单个品种最多4个单位

### 5. 心理准备

- 接受连续亏损（可能连续亏损5-10次）
- 坚持执行规则，不凭感觉交易
- 让利润奔跑，及时止损

## 常见问题

### Q1: 为什么会有这么多假突破？

A: 假突破是趋势跟踪策略的固有特性。海龟策略通过以下方式应对：
- 严格的止损控制单次损失
- 金字塔加仓让真正的趋势带来大利润
- 整体盈亏比高于1:2或1:3

### Q2: 如何选择系统1还是系统2？

A:
- **系统1（20天）**: 信号更频繁，适合活跃交易，但假突破更多
- **系统2（55天）**: 信号更可靠，适合长线持有，但可能错过早期机会
- **建议**: 可以同时使用，系统1用于主要交易，系统2用于备份

### Q3: ATR止损会不会太宽？

A: ATR止损的优势在于：
- 适应市场波动，波动大时止损宽，波动小时止损窄
- 避免被正常波动扫出
- 配合仓位管理，实际风险可控

### Q4: 如何处理跳空缺口？

A: 跳空缺口的处理：
- 向上跳空突破：视为有效突破信号
- 向下跳空破位：立即止损，不等回补
- 建议使用限价单而非市价单

### Q5: 可以用于A股吗？

A: 可以，但需要注意：
- A股有涨跌停限制，可能影响止损执行
- T+1交易制度，当日买入不能卖出
- 建议适当调整参数，如加大止损空间

## 历史背景

海龟交易策略源于1983年理查德·丹尼斯与威廉·埃克哈特的一场著名赌约：优秀的交易员是天生的还是可以培养的？

丹尼斯招募了23名"海龟"（Turtles），用两周时间教授他们这套交易系统，然后给他们真金白银去交易。结果证明，严格遵守规则的海龟们在4年内平均年化收益率达到80%。

这个实验证明了：
1. 交易可以通过系统化的方法学习
2. 纪律和风险管理比预测市场更重要
3. 简单的规则加上严格执行可以战胜市场

## 总结

海龟交易策略是一个经过实战检验的趋势跟踪系统，其核心价值在于：

✅ **系统化**: 规则明确，易于执行  
✅ **风险管理**: ATR止损，仓位控制  
✅ **趋势跟踪**: 捕捉大趋势，让利润奔跑  
✅ **久经考验**: 40年历史，业绩优秀  

但也要注意：

⚠️ **需要耐心**: 等待趋势形成  
⚠️ **回撤较大**: 接受连续亏损  
⚠️ **纪律要求高**: 严格执行规则  

成功使用海龟策略的关键是：**理解原理，严格执行，坚持到底**。
