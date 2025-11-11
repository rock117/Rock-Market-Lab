# 选股 API 接口文档

## 1. 简单选股接口

### 接口信息
- **路径**: `POST /api/stocks/pick`
- **描述**: 使用默认配置进行选股（最近3个月数据，激进策略）

### 请求参数
无需参数

### 响应示例
```json
{
  "code": 200,
  "data": [
    {
      "ts_code": "000001.SZ",
      "stock_name": "平安银行",
      "strategy_result": {
        "stock_code": "000001.SZ",
        "analysis_date": "2024-11-11",
        "current_price": 12.50,
        "strategy_signal": "Buy",
        "signal_strength": 75,
        "risk_level": 2,
        "analysis_description": "放量上涨，长阳线形态"
      }
    }
  ]
}
```

---

## 2. 高级选股接口（动态配置）

### 接口信息
- **路径**: `POST /api/stocks/pick/advanced`
- **描述**: 支持自定义策略类型和动态配置参数

### 请求参数

#### 参数结构
```json
{
  "type": "策略类型（字符串）",
  "settings": {
    // 动态字段，根据 type 不同而不同
  }
}
```

#### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| type | string | 是 | 策略类型，如 "price_volume", "conservative", "aggressive" |
| settings | object | 是 | 策略配置，字段根据 type 动态变化 |

---

### 策略类型及对应的 settings 字段

#### 1. 价量K线策略 (`type: "price_volume"`)

```json
{
  "type": "price_volume",
  "settings": {
    "analysis_period": 20,              // 分析周期（天数），默认20
    "volume_ma_period": 5,              // 成交量均线周期，默认5
    "price_volatility_threshold": 3.0,  // 价格波动阈值（%），默认3.0
    "volume_amplification_threshold": 1.5, // 成交量放大倍数，默认1.5
    "candlestick_body_threshold": 2.0   // K线实体大小阈值（%），默认2.0
  }
}
```

**字段说明**：
- `analysis_period`: 分析的历史数据天数
- `volume_ma_period`: 计算成交量移动平均的周期
- `price_volatility_threshold`: 价格波动超过此阈值视为显著变化
- `volume_amplification_threshold`: 成交量超过均值的倍数
- `candlestick_body_threshold`: K线实体占整体的比例阈值

#### 2. 保守策略 (`type: "conservative"`)

```json
{
  "type": "conservative",
  "settings": {
    "risk_level": "low",           // 风险等级: "low", "medium", "high"
    "min_market_cap": 10000000000, // 最小市值（元）
    "max_volatility": 2.0,         // 最大波动率（%）
    "min_pe_ratio": 5.0,           // 最小市盈率
    "max_pe_ratio": 30.0           // 最大市盈率
  }
}
```

#### 3. 激进策略 (`type: "aggressive"`)

```json
{
  "type": "aggressive",
  "settings": {
    "min_price_change": 5.0,       // 最小涨幅（%）
    "min_volume_ratio": 2.0,       // 最小成交量比率
    "allow_new_high": true,        // 是否允许创新高的股票
    "momentum_period": 10          // 动量周期（天）
  }
}
```

#### 4. 自定义策略 (`type: "custom"`)

```json
{
  "type": "custom",
  "settings": {
    // 任意自定义字段
    "field1": "value1",
    "field2": 123,
    "nested": {
      "subfield": true
    }
  }
}
```

---

### 响应示例

```json
{
  "code": 200,
  "data": {
    "stocks": [
      {
        "ts_code": "000001.SZ",
        "stock_name": "平安银行",
        "strategy_result": {
          "stock_code": "000001.SZ",
          "analysis_date": "2024-11-11",
          "current_price": 12.50,
          "strategy_signal": "Buy",
          "signal_strength": 75,
          "risk_level": 2,
          "analysis_description": "放量上涨，长阳线形态"
        }
      },
      {
        "ts_code": "600000.SH",
        "stock_name": "浦发银行",
        "strategy_result": {
          "stock_code": "600000.SH",
          "analysis_date": "2024-11-11",
          "current_price": 8.20,
          "strategy_signal": "StrongBuy",
          "signal_strength": 85,
          "risk_level": 1,
          "analysis_description": "强势突破，成交量显著放大"
        }
      }
    ],
    "total": 2,
    "strategy_type": "price_volume"
  }
}
```

---

## 使用示例

### cURL 示例

#### 1. 简单选股
```bash
curl -X POST http://localhost:8000/api/stocks/pick
```

#### 2. 价量K线策略选股
```bash
curl -X POST http://localhost:8000/api/stocks/pick/advanced \
  -H "Content-Type: application/json" \
  -d '{
    "type": "price_volume",
    "settings": {
      "analysis_period": 30,
      "volume_ma_period": 10,
      "price_volatility_threshold": 5.0
    }
  }'
```

#### 3. 保守策略选股
```bash
curl -X POST http://localhost:8000/api/stocks/pick/advanced \
  -H "Content-Type: application/json" \
  -d '{
    "type": "conservative",
    "settings": {
      "risk_level": "low",
      "min_market_cap": 50000000000,
      "max_volatility": 1.5
    }
  }'
```

### JavaScript 示例

```javascript
// 价量K线策略
const response = await fetch('http://localhost:8000/api/stocks/pick/advanced', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    type: 'price_volume',
    settings: {
      analysis_period: 20,
      volume_ma_period: 5,
      price_volatility_threshold: 3.0,
      volume_amplification_threshold: 1.5,
      candlestick_body_threshold: 2.0
    }
  })
});

const data = await response.json();
console.log(data);
```

### Python 示例

```python
import requests

# 激进策略
response = requests.post(
    'http://localhost:8000/api/stocks/pick/advanced',
    json={
        'type': 'aggressive',
        'settings': {
            'min_price_change': 5.0,
            'min_volume_ratio': 2.0,
            'allow_new_high': True,
            'momentum_period': 10
        }
    }
)

data = response.json()
print(data)
```

---

## 错误响应

### 参数错误
```json
{
  "code": 400,
  "message": "Invalid request parameters"
}
```

### 服务器错误
```json
{
  "code": 500,
  "message": "Internal server error"
}
```

---

## 注意事项

1. **settings 字段的灵活性**：
   - `settings` 使用 `JsonValue` 类型，可以接收任意 JSON 对象
   - 不同的 `type` 对应不同的 `settings` 字段结构
   - 后端会根据 `type` 动态解析 `settings` 中的字段

2. **扩展性**：
   - 添加新策略时，只需定义新的 `type` 值和对应的 `settings` 结构
   - 无需修改 API 接口定义

3. **性能考虑**：
   - 全市场扫描可能需要较长时间
   - 建议在后台任务中执行或添加超时限制

4. **数据要求**：
   - 确保数据库中有足够的历史数据
   - 不同策略可能需要不同的最小数据量
