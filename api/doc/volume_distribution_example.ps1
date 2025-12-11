# 成交量分布分析 API 测试脚本 (PowerShell)

$BaseUrl = "http://localhost:8000"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "成交量分布分析 API 测试" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# 测试 1: 基本查询
Write-Host "测试 1: 查询某个交易日的成交量分布（默认 Top 50）" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101" -Method Get
$response | ConvertTo-Json -Depth 10
Write-Host ""
Write-Host ""

# 测试 2: 指定 Top N
Write-Host "测试 2: 查询并返回 Top 100 股票详情" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101&top_n=100" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101&top_n=100" -Method Get
$response | ConvertTo-Json -Depth 10
Write-Host ""
Write-Host ""

# 测试 3: 只查看集中度指标
Write-Host "测试 3: 只查看集中度指标" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101" -Method Get
Write-Host "HHI (赫芬达尔指数): $($response.data.concentration_metrics.hhi) - $($response.data.concentration_metrics.hhi_level)" -ForegroundColor Green
Write-Host "Gini (基尼系数): $($response.data.concentration_metrics.gini_coefficient) - $($response.data.concentration_metrics.gini_level)" -ForegroundColor Green
Write-Host "CR4 (前4名集中度): $($response.data.concentration_metrics.cr4)%" -ForegroundColor Green
Write-Host "CR8 (前8名集中度): $($response.data.concentration_metrics.cr8)%" -ForegroundColor Green
Write-Host "Entropy (熵指数): $($response.data.concentration_metrics.entropy)" -ForegroundColor Green
Write-Host ""
Write-Host ""

# 测试 4: 只查看 Top N 占比
Write-Host "测试 4: 只查看 Top N 占比" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101" -Method Get
Write-Host "Top 10 占比: $([math]::Round($response.data.top_concentrations.top10_pct * 100, 2))%" -ForegroundColor Magenta
Write-Host "Top 30 占比: $([math]::Round($response.data.top_concentrations.top30_pct * 100, 2))%" -ForegroundColor Magenta
Write-Host "Top 50 占比: $([math]::Round($response.data.top_concentrations.top50_pct * 100, 2))%" -ForegroundColor Magenta
Write-Host "Top 100 占比: $([math]::Round($response.data.top_concentrations.top100_pct * 100, 2))%" -ForegroundColor Magenta
Write-Host ""
Write-Host ""

# 测试 5: 查看 Top 10 股票
Write-Host "测试 5: 查看 Top 10 股票详情" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101&top_n=10" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101&top_n=10" -Method Get
Write-Host "排名 | 股票代码 | 成交量占比 | 成交额占比 | 涨跌幅" -ForegroundColor Cyan
Write-Host "------|----------|------------|------------|--------" -ForegroundColor Cyan
foreach ($stock in $response.data.top_stocks) {
    $pctChg = if ($stock.pct_chg) { "$($stock.pct_chg)%" } else { "N/A" }
    Write-Host "$($stock.rank) | $($stock.ts_code) | $($stock.volume_pct)% | $($stock.amount_pct)% | $pctChg"
}
Write-Host ""
Write-Host ""

# 测试 6: 市场分析示例
Write-Host "测试 6: 市场分析示例" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=20240101" -ForegroundColor Gray
Write-Host ""
$response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=20240101" -Method Get

Write-Host "=== 市场概况 ===" -ForegroundColor Cyan
Write-Host "交易日期: $($response.data.trade_date)"
Write-Host "股票总数: $($response.data.total_stocks)"
Write-Host "总成交量: $([math]::Round($response.data.total_volume / 100000000, 2)) 亿手"
Write-Host "总成交额: $([math]::Round($response.data.total_amount / 100000000, 2)) 亿元"
Write-Host ""

Write-Host "=== 集中度分析 ===" -ForegroundColor Cyan
$hhi = $response.data.concentration_metrics.hhi
$gini = $response.data.concentration_metrics.gini_coefficient

if ($hhi -lt 1500) {
    Write-Host "市场集中度: 低（HHI=$hhi）- 市场竞争充分" -ForegroundColor Green
} elseif ($hhi -lt 2500) {
    Write-Host "市场集中度: 中（HHI=$hhi）- 中等集中" -ForegroundColor Yellow
} else {
    Write-Host "市场集中度: 高（HHI=$hhi）- 高度集中" -ForegroundColor Red
}

if ($gini -lt 0.3) {
    Write-Host "分布均衡度: 均衡（Gini=$gini）- 成交量分布均匀" -ForegroundColor Green
} elseif ($gini -lt 0.5) {
    Write-Host "分布均衡度: 中等（Gini=$gini）- 中等不均" -ForegroundColor Yellow
} else {
    Write-Host "分布均衡度: 不均（Gini=$gini）- 高度不均衡" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== 资金流向 ===" -ForegroundColor Cyan
$top10 = [math]::Round($response.data.top_concentrations.top10_pct * 100, 2)
$top100 = [math]::Round($response.data.top_concentrations.top100_pct * 100, 2)

if ($top10 -gt 30 -and $top100 -lt 70) {
    Write-Host "市场特征: 结构性行情 - 资金集中在少数龙头股" -ForegroundColor Yellow
} elseif ($top10 -lt 20 -and $top100 -gt 70) {
    Write-Host "市场特征: 全面行情 - 普涨格局，市场活跃" -ForegroundColor Green
} else {
    Write-Host "市场特征: 正常交易 - 资金分布相对均衡" -ForegroundColor Cyan
}

Write-Host ""
Write-Host ""

# 测试 7: 错误处理
Write-Host "测试 7: 错误处理 - 日期格式错误" -ForegroundColor Yellow
Write-Host "请求: GET $BaseUrl/api/volume-distribution?trade_date=2024-01-01" -ForegroundColor Gray
Write-Host ""
try {
    $response = Invoke-RestMethod -Uri "$BaseUrl/api/volume-distribution?trade_date=2024-01-01" -Method Get
    $response | ConvertTo-Json
} catch {
    Write-Host "错误: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""
Write-Host ""

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "测试完成" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
