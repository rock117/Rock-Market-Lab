# 困境反转策略使用示例

## 策略概述

困境反转策略（Distressed Reversal Strategy）专注于识别那些经历困境但正在反转的公司。这类公司通常具有以下特征：

- **曾经历困境**：连续亏损、ROE下降、业绩不佳
- **出现反转信号**：盈利改善、现金流好转、资产质量提升
- **估值合理**：PE、PB处于历史低位
- **技术面配合**：价格企稳、成交量放大

## 策略逻辑

### 1. 困境识别
- ROE低于10%或曾连续亏损
- 负债率曾经较高
- 股价处于历史低位

### 2. 反转信号
- **盈利改善**：ROE环比改善≥3个百分点
- **增长恢复**：净利润增长率≥50%
- **现金流健康**：经营现金流/净利润≥80%
- **负债可控**：资产负债率≤70%

### 3. 估值评估
- PE处于历史30%分位以下
- PB处于历史30%分位以下

### 4. 技术确认
- 价格连续5天企稳（上涨或横盘）
- 成交量放大1.5倍以上

## 配置参数

```rust
pub struct DistressedReversalConfig {
    /// 分析周期（天数）- 用于技术面分析
    pub analysis_period: usize,  // 默认: 60
    
    /// 财务数据季度数（用于趋势分析）
    pub financial_quarters: usize,  // 默认: 8 (2年)
    
    /// ROE改善阈值（百分点）
    pub roe_improvement_threshold: f64,  // 默认: 3.0
    
    /// 净利润增长率阈值（百分比）
    pub profit_growth_threshold: f64,  // 默认: 50.0
    
    /// 经营现金流/净利润比率阈值
    pub cashflow_ratio_threshold: f64,  // 默认: 0.8
    
    /// 资产负债率上限（百分比）
    pub max_debt_ratio: f64,  // 默认: 70.0
    
    /// PE百分位阈值（历史低位）
    pub pe_percentile_threshold: f64,  // 默认: 30.0
    
    /// PB百分位阈值（历史低位）
    pub pb_percentile_threshold: f64,  // 默认: 30.0
    
    /// 价格企稳天数（连续上涨或横盘）
    pub price_stabilization_days: usize,  // 默认: 5
    
    /// 成交量放大倍数
    pub volume_surge_ratio: f64,  // 默认: 1.5
}
```

## 使用示例

### 1. 使用默认配置

```rust
use service::strategy::DistressedReversalStrategy;

// 创建默认策略
let mut strategy = DistressedReversalStrategy::default_strategy();

// 分析股票
let result = strategy.analyze("000001.SZ", &stock_data)?;
```

### 2. 使用激进配置

激进配置适合风险承受能力较高的投资者，条件更宽松：

```rust
// 创建激进策略
let mut strategy = DistressedReversalStrategy::aggressive();

// 激进配置特点：
// - ROE改善阈值: 2个百分点（更低）
// - 净利润增长: 30%（更低）
// - 现金流比率: 60%（更低）
// - 负债率上限: 80%（更高）
// - PE/PB百分位: 40%（更宽松）
// - 企稳天数: 3天（更短）
// - 成交量放大: 1.3倍（更低）
```

### 3. 使用保守配置

保守配置适合风险厌恶型投资者，条件更严格：

```rust
// 创建保守策略
let mut strategy = DistressedReversalStrategy::conservative();

// 保守配置特点：
// - ROE改善阈值: 5个百分点（更高）
// - 净利润增长: 80%（更高）
// - 现金流比率: 100%（更高）
// - 负债率上限: 60%（更低）
// - PE/PB百分位: 20%（更严格）
// - 企稳天数: 10天（更长）
// - 成交量放大: 2.0倍（更高）
```

### 4. 自定义配置

```rust
use service::strategy::{DistressedReversalStrategy, DistressedReversalConfig};

let config = DistressedReversalConfig {
    analysis_period: 90,
    financial_quarters: 12,  // 3年财报数据
    roe_improvement_threshold: 4.0,
    profit_growth_threshold: 60.0,
    cashflow_ratio_threshold: 0.9,
    max_debt_ratio: 65.0,
    pe_percentile_threshold: 25.0,
    pb_percentile_threshold: 25.0,
    price_stabilization_days: 7,
    volume_surge_ratio: 1.8,
};

let mut strategy = DistressedReversalStrategy::new(config);
```

### 5. 通过 API 调用

```bash
# 使用默认配置
curl -X POST http://localhost:8000/api/stocks/pick \
  -H "Content-Type: application/json" \
  -d '{
    "strategy": "distressed_reversal"
  }'

# 使用自定义配置
curl -X POST http://localhost:8000/api/stocks/pick \
  -H "Content-Type: application/json" \
  -d '{
    "strategy": "distressed_reversal",
    "settings": {
      "analysis_period": 90,
      "roe_improvement_threshold": 4.0,
      "profit_growth_threshold": 60.0,
      "cashflow_ratio_threshold": 0.9,
      "max_debt_ratio": 65.0,
      "pe_percentile_threshold": 25.0,
      "pb_percentile_threshold": 25.0,
      "price_stabilization_days": 7,
      "volume_surge_ratio": 1.8
    }
  }'
```

## 结果字段说明

```rust
pub struct DistressedReversalResult {
    // 基础信息
    pub stock_code: String,           // 股票代码
    pub analysis_date: NaiveDate,     // 分析日期
    pub current_price: f64,           // 当前价格
    pub strategy_signal: StrategySignal,  // 策略信号
    pub signal_strength: u8,          // 信号强度 (0-100)
    pub analysis_description: String, // 分析说明
    pub risk_level: u8,               // 风险等级 (1-5)
    
    // 困境反转标识
    pub is_distressed: bool,          // 是否处于困境
    pub has_reversal_signal: bool,    // 是否有反转迹象
    
    // 财务指标
    pub latest_roe: f64,              // 最新ROE（%）
    pub roe_change: f64,              // ROE环比变化（百分点）
    pub profit_growth_rate: f64,      // 净利润增长率（%）
    pub cashflow_ratio: f64,          // 经营现金流/净利润
    pub debt_ratio: f64,              // 资产负债率（%）
    
    // 估值指标
    pub current_pe: f64,              // 当前PE
    pub pe_percentile: f64,           // PE历史百分位
    pub current_pb: f64,              // 当前PB
    pub pb_percentile: f64,           // PB历史百分位
    
    // 技术指标
    pub price_stable_days: usize,     // 价格企稳天数
    pub volume_ratio: f64,            // 成交量放大倍数
    pub recent_low: f64,              // 近期最低价
    pub recent_high: f64,             // 近期最高价
}
```

## 信号强度评分

策略采用100分制评分系统：

### 财务改善（40分）
- ROE改善达标：10分
- 净利润增长达标：15分
- 现金流健康：10分
- 负债率可控：5分

### 估值水平（30分）
- PE处于低位：15分
- PB处于低位：15分

### 技术面（30分）
- 价格企稳：15分
- 成交量放大：15分

### 信号等级
- **80-100分**：强烈买入（StrongBuy）
- **60-79分**：买入（Buy）
- **40-59分**：持有（Hold）
- **0-39分**：卖出（Sell）

## 风险等级

困境反转策略本身风险较高（基础风险等级5），但会根据财务状况调整：

- **现金流良好**：风险-1
- **低负债率**：风险-1
- **最终风险等级**：1-5

## 适用场景

### 适合使用的情况
1. **周期性行业底部**：钢铁、化工、航运等周期性行业处于底部时
2. **行业整顿后**：教育、房地产等行业经历整顿后的龙头企业
3. **经营困难改善**：管理层更换、业务重组后的公司
4. **市场过度悲观**：因短期利空导致股价超跌的优质公司

### 不适合使用的情况
1. **持续恶化的公司**：基本面持续恶化，无反转迹象
2. **财务造假风险**：财务数据可疑的公司
3. **夕阳行业**：行业整体衰退，无复苏可能
4. **高负债高风险**：资产负债率过高，有破产风险

## 注意事项

1. **财务数据质量**：
   - 需要真实可靠的财务数据
   - 注意识别财务造假风险
   - 关注审计意见

2. **行业周期判断**：
   - 区分周期性困境和结构性困境
   - 周期性困境更容易反转
   - 结构性困境需谨慎

3. **反转持续性**：
   - 单季度改善不足以确认反转
   - 需要连续多个季度的数据支持
   - 关注改善的可持续性

4. **风险控制**：
   - 困境反转策略风险较高
   - 建议分散投资
   - 设置止损位

5. **时间成本**：
   - 困境反转需要时间
   - 可能需要持有1-2年
   - 需要耐心等待

## 最佳实践

1. **结合行业分析**：
   ```rust
   // 优先选择周期性行业
   if industry.is_cyclical() && industry.is_at_bottom() {
       // 使用困境反转策略
   }
   ```

2. **多维度验证**：
   ```rust
   // 同时满足财务、估值、技术三个维度
   if result.has_reversal_signal 
      && result.pe_percentile < 30.0 
      && result.price_stable_days >= 5 {
       // 信号更可靠
   }
   ```

3. **动态调整仓位**：
   ```rust
   let position_size = match result.signal_strength {
       80..=100 => 0.15,  // 15%仓位
       60..=79 => 0.10,   // 10%仓位
       40..=59 => 0.05,   // 5%仓位
       _ => 0.0,          // 不建仓
   };
   ```

4. **设置止损止盈**：
   ```rust
   let stop_loss = result.current_price * 0.85;   // -15%止损
   let take_profit = result.current_price * 1.50;  // +50%止盈
   ```

## 实战案例

### 案例1：周期性行业反转

某钢铁股在行业底部时：
- ROE从-2%改善至5%（改善7个百分点）✓
- 净利润扭亏为盈，增长200%✓
- 经营现金流/净利润 = 1.2✓
- 资产负债率65%✓
- PE处于历史10%分位✓
- PB处于历史15%分位✓
- 价格连续8天企稳✓
- 成交量放大2.5倍✓

**信号强度：95分（强烈买入）**
**风险等级：3（中等风险）**

### 案例2：经营改善但估值偏高

某消费股管理层更换后：
- ROE从8%改善至12%（改善4个百分点）✓
- 净利润增长65%✓
- 经营现金流/净利润 = 0.95✓
- 资产负债率45%✓
- PE处于历史60%分位✗
- PB处于历史55%分位✗
- 价格连续6天企稳✓
- 成交量放大1.8倍✓

**信号强度：55分（持有）**
**风险等级：2（较低风险）**

## 总结

困境反转策略是一种高风险高回报的投资策略，适合：
- 有一定风险承受能力的投资者
- 能够深入研究基本面的投资者
- 有耐心等待反转的长期投资者

关键成功因素：
1. 准确识别真正的困境反转机会
2. 区分周期性困境和结构性困境
3. 严格控制风险，分散投资
4. 耐心持有，等待反转兑现
