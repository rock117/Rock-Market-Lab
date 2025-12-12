# 投资组合管理 API 文档

## 概述

投资组合管理 API 提供了完整的投资组合（Portfolio）和持仓（Holding）管理功能，包括创建、查询、修改和删除操作。

## 数据模型

### Portfolio（投资组合）
```json
{
  "id": 1,
  "name": "我的投资组合",
  "holdings": [...]
}
```

### Holding（持仓）
```json
{
  "id": 1,
  "exchange_id": "XNAS",
  "symbol": "AAPL",
  "portfolio_id": 1
}
```

## API 端点

### 1. 创建投资组合

**请求**
```http
POST /api/portfolios
Content-Type: application/json

{
  "name": "我的美股组合"
}
```

**响应**
```json
{
  "data": {
    "id": 1,
    "name": "我的美股组合",
    "holdings": []
  },
  "success": true
}
```

**示例（PowerShell）**
```powershell
$body = @{
    name = "我的美股组合"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios" `
    -Method Post `
    -Body $body `
    -ContentType "application/json"
```

**示例（curl）**
```bash
curl -X POST http://localhost:8000/api/portfolios \
  -H "Content-Type: application/json" \
  -d '{"name":"我的美股组合"}'
```

---

### 2. 获取所有投资组合

**请求**
```http
GET /api/portfolios
```

**响应**
```json
{
  "data": [
    {
      "id": 1,
      "name": "我的美股组合",
      "holdings": [
        {
          "id": 1,
          "exchange_id": "XNAS",
          "symbol": "AAPL",
          "portfolio_id": 1
        }
      ]
    }
  ],
  "success": true
}
```

**示例（PowerShell）**
```powershell
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios" -Method Get
```

**示例（curl）**
```bash
curl http://localhost:8000/api/portfolios
```

---

### 3. 获取单个投资组合详情

**请求**
```http
GET /api/portfolios/{portfolio_id}
```

**路径参数**
- `portfolio_id`: 投资组合ID

**响应**
```json
{
  "data": {
    "id": 1,
    "name": "我的美股组合",
    "holdings": [
      {
        "id": 1,
        "exchange_id": "XNAS",
        "symbol": "AAPL",
        "portfolio_id": 1
      }
    ]
  },
  "success": true
}
```

**示例（PowerShell）**
```powershell
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/1" -Method Get
```

**示例（curl）**
```bash
curl http://localhost:8000/api/portfolios/1
```

---

### 4. 删除投资组合

删除投资组合时会自动删除该组合下的所有持仓（级联删除，在事务中执行）。

**请求**
```http
DELETE /api/portfolios/{portfolio_id}
```

**路径参数**
- `portfolio_id`: 投资组合ID

**响应**
```json
{
  "data": "Portfolio 1 deleted successfully",
  "success": true
}
```

**示例（PowerShell）**
```powershell
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/1" -Method Delete
```

**示例（curl）**
```bash
curl -X DELETE http://localhost:8000/api/portfolios/1
```

---

### 5. 添加持仓到投资组合

**请求**
```http
POST /api/portfolios/{portfolio_id}/holdings
Content-Type: application/json

{
  "exchange_id": "XNAS",
  "symbol": "AAPL"
}
```

**路径参数**
- `portfolio_id`: 投资组合ID

**请求体字段**
- `exchange_id`: 交易所ID（可选）
- `symbol`: 股票代码（可选）

**响应**
```json
{
  "data": {
    "id": 1,
    "exchange_id": "XNAS",
    "symbol": "AAPL",
    "portfolio_id": 1
  },
  "success": true
}
```

**示例（PowerShell）**
```powershell
$body = @{
    exchange_id = "XNAS"
    symbol = "AAPL"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/1/holdings" `
    -Method Post `
    -Body $body `
    -ContentType "application/json"
```

**示例（curl）**
```bash
curl -X POST http://localhost:8000/api/portfolios/1/holdings \
  -H "Content-Type: application/json" \
  -d '{"exchange_id":"XNAS","symbol":"AAPL"}'
```

---

### 6. 从投资组合删除持仓

**请求**
```http
DELETE /api/portfolios/{portfolio_id}/holdings/{holding_id}
```

**路径参数**
- `portfolio_id`: 投资组合ID
- `holding_id`: 持仓ID

**响应**
```json
{
  "data": "Holding 1 removed successfully",
  "success": true
}
```

**示例（PowerShell）**
```powershell
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/1/holdings/1" -Method Delete
```

**示例（curl）**
```bash
curl -X DELETE http://localhost:8000/api/portfolios/1/holdings/1
```

---

## 完整使用流程示例

### PowerShell 脚本示例

```powershell
# 1. 创建投资组合
$portfolio = Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios" `
    -Method Post `
    -Body (@{name = "科技股组合"} | ConvertTo-Json) `
    -ContentType "application/json"

$portfolioId = $portfolio.data.id
Write-Host "创建投资组合成功，ID: $portfolioId"

# 2. 添加多个持仓
$stocks = @(
    @{exchange_id = "XNAS"; symbol = "AAPL"},
    @{exchange_id = "XNAS"; symbol = "MSFT"},
    @{exchange_id = "XNAS"; symbol = "GOOGL"}
)

foreach ($stock in $stocks) {
    $holding = Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/$portfolioId/holdings" `
        -Method Post `
        -Body ($stock | ConvertTo-Json) `
        -ContentType "application/json"
    Write-Host "添加持仓: $($stock.symbol)"
}

# 3. 查看投资组合详情
$details = Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/$portfolioId" -Method Get
Write-Host "投资组合详情:"
$details.data | ConvertTo-Json -Depth 10

# 4. 删除某个持仓
$holdingId = $details.data.holdings[0].id
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/$portfolioId/holdings/$holdingId" -Method Delete
Write-Host "删除持仓 ID: $holdingId"

# 5. 查看所有投资组合
$allPortfolios = Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios" -Method Get
Write-Host "所有投资组合:"
$allPortfolios.data | ConvertTo-Json -Depth 10

# 6. 删除投资组合（会自动删除所有持仓）
Invoke-RestMethod -Uri "http://localhost:8000/api/portfolios/$portfolioId" -Method Delete
Write-Host "删除投资组合 ID: $portfolioId"
```

### Bash 脚本示例

```bash
#!/bin/bash

BASE_URL="http://localhost:8000"

# 1. 创建投资组合
echo "创建投资组合..."
PORTFOLIO=$(curl -s -X POST "$BASE_URL/api/portfolios" \
  -H "Content-Type: application/json" \
  -d '{"name":"科技股组合"}')

PORTFOLIO_ID=$(echo $PORTFOLIO | jq -r '.data.id')
echo "创建投资组合成功，ID: $PORTFOLIO_ID"

# 2. 添加多个持仓
echo "添加持仓..."
curl -s -X POST "$BASE_URL/api/portfolios/$PORTFOLIO_ID/holdings" \
  -H "Content-Type: application/json" \
  -d '{"exchange_id":"XNAS","symbol":"AAPL"}' | jq

curl -s -X POST "$BASE_URL/api/portfolios/$PORTFOLIO_ID/holdings" \
  -H "Content-Type: application/json" \
  -d '{"exchange_id":"XNAS","symbol":"MSFT"}' | jq

curl -s -X POST "$BASE_URL/api/portfolios/$PORTFOLIO_ID/holdings" \
  -H "Content-Type: application/json" \
  -d '{"exchange_id":"XNAS","symbol":"GOOGL"}' | jq

# 3. 查看投资组合详情
echo "查看投资组合详情..."
DETAILS=$(curl -s "$BASE_URL/api/portfolios/$PORTFOLIO_ID")
echo $DETAILS | jq

# 4. 删除某个持仓
HOLDING_ID=$(echo $DETAILS | jq -r '.data.holdings[0].id')
echo "删除持仓 ID: $HOLDING_ID"
curl -s -X DELETE "$BASE_URL/api/portfolios/$PORTFOLIO_ID/holdings/$HOLDING_ID" | jq

# 5. 查看所有投资组合
echo "查看所有投资组合..."
curl -s "$BASE_URL/api/portfolios" | jq

# 6. 删除投资组合
echo "删除投资组合 ID: $PORTFOLIO_ID"
curl -s -X DELETE "$BASE_URL/api/portfolios/$PORTFOLIO_ID" | jq
```

---

## 错误处理

所有 API 在发生错误时会返回：

```json
{
  "data": "错误信息描述",
  "success": false
}
```

常见错误：
- **404**: 投资组合或持仓不存在
- **400**: 请求参数错误
- **500**: 服务器内部错误

---

## 技术实现特点

### 事务保证
- 删除投资组合时，使用数据库事务确保持仓和组合同时删除
- 删除失败时自动回滚，保证数据一致性

### 数据校验
- 删除持仓时会验证该持仓是否属于指定的投资组合
- 防止跨组合误删除

### 日志记录
- 所有操作都有详细的日志记录
- 便于问题排查和审计

---

## 数据库表结构

### portfolio 表
```sql
CREATE TABLE portfolio (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) NOT NULL
);
```

### holding 表
```sql
CREATE TABLE holding (
    id INT PRIMARY KEY AUTO_INCREMENT,
    exchange_id VARCHAR(20),
    symbol VARCHAR(20),
    portfolio_id INT NOT NULL,
    FOREIGN KEY (portfolio_id) REFERENCES portfolio(id)
);
```

---

## 后续扩展建议

1. **添加用户权限**: 为投资组合添加用户所有权，实现多用户隔离
2. **持仓数量**: 在 holding 表中添加 `quantity`（数量）和 `cost_basis`（成本价）字段
3. **组合统计**: 添加投资组合的总价值、收益率等统计信息
4. **批量操作**: 支持批量添加/删除持仓
5. **组合复制**: 支持复制现有投资组合
6. **历史记录**: 记录持仓的买入卖出历史

---

## 联系与支持

如有问题或建议，请通过以下方式联系：
- GitHub Issues
- Email: support@example.com
