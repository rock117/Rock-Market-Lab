#!/bin/bash

# 成交量分布分析 API 测试脚本

BASE_URL="http://localhost:8000"

echo "=========================================="
echo "成交量分布分析 API 测试"
echo "=========================================="
echo ""

# 测试 1: 基本查询
echo "测试 1: 查询某个交易日的成交量分布（默认 Top 50）"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=20240101"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=20240101" \
  -H "Content-Type: application/json" | jq '.'
echo ""
echo ""

# 测试 2: 指定 Top N
echo "测试 2: 查询并返回 Top 100 股票详情"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=20240101&top_n=100"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=20240101&top_n=100" \
  -H "Content-Type: application/json" | jq '.'
echo ""
echo ""

# 测试 3: 只查看集中度指标
echo "测试 3: 只查看集中度指标"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=20240101"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=20240101" \
  -H "Content-Type: application/json" | jq '.data.concentration_metrics'
echo ""
echo ""

# 测试 4: 只查看 Top N 占比
echo "测试 4: 只查看 Top N 占比"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=20240101"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=20240101" \
  -H "Content-Type: application/json" | jq '.data.top_concentrations'
echo ""
echo ""

# 测试 5: 查看 Top 10 股票
echo "测试 5: 查看 Top 10 股票详情"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=20240101&top_n=10"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=20240101&top_n=10" \
  -H "Content-Type: application/json" | jq '.data.top_stocks'
echo ""
echo ""

# 测试 6: 错误处理 - 日期格式错误
echo "测试 6: 错误处理 - 日期格式错误"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=2024-01-01"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=2024-01-01" \
  -H "Content-Type: application/json" | jq '.'
echo ""
echo ""

# 测试 7: 错误处理 - 无数据
echo "测试 7: 错误处理 - 无数据的交易日"
echo "请求: GET ${BASE_URL}/api/volume-distribution?trade_date=19900101"
echo ""
curl -X GET "${BASE_URL}/api/volume-distribution?trade_date=19900101" \
  -H "Content-Type: application/json" | jq '.'
echo ""
echo ""

echo "=========================================="
echo "测试完成"
echo "=========================================="
