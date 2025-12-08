'use client'

import React from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { stockApi } from '@/services/api'
import { MarketSummary as MarketSummaryType, IndexData, PieChartData } from '@/types'
import { formatNumber, formatLargeNumber, formatPercent, getTrendColorClass, getStockTrend, getTrendColor } from '@/lib/utils'
import { TrendingUp, TrendingDown, Activity, BarChart3, PieChart, Users } from 'lucide-react'
import { PieChart as RechartsPieChart, Pie, Cell, ResponsiveContainer, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip } from 'recharts'

interface MarketSummaryProps {
  className?: string
}

export default function MarketSummary({ className }: MarketSummaryProps) {
  // 获取市场概览数据
  const { data: marketData, isLoading: marketLoading } = useQuery({
    queryKey: ['market-summary'],
    queryFn: () => stockApi.getMarketSummary(),
    staleTime: 2 * 60 * 1000, // 2分钟缓存
  })

  // 获取主要指数数据
  const { data: indexData, isLoading: indexLoading } = useQuery({
    queryKey: ['index-data'],
    queryFn: () => stockApi.getIndexData({
      ts_codes: ['000001.SH', '399001.SZ', '399006.SZ'] // 上证指数、深证成指、创业板指
    }),
    staleTime: 2 * 60 * 1000,
  })

  // 获取涨跌分布数据
  const { data: distributionData, isLoading: distributionLoading } = useQuery({
    queryKey: ['price-distribution'],
    queryFn: () => stockApi.getPriceDistribution(),
    staleTime: 2 * 60 * 1000,
  })

  // 模拟数据
  const mockMarketData: MarketSummaryType = {
    trade_date: '2024-12-08',
    total_stocks: 5234,
    up_count: 2156,
    down_count: 2834,
    flat_count: 244,
    limit_up_count: 45,
    limit_down_count: 12,
    total_volume: 1234567890,
    total_amount: 987654321000,
    avg_pct_chg: -0.85
  }

  const mockIndexData: IndexData[] = [
    {
      ts_code: '000001.SH',
      name: '上证指数',
      trade_date: '2024-12-08',
      close: 3245.67,
      open: 3258.12,
      high: 3268.45,
      low: 3238.90,
      pre_close: 3258.12,
      change: -12.45,
      pct_chg: -0.38,
      vol: 234567890,
      amount: 345678901234
    },
    {
      ts_code: '399001.SZ',
      name: '深证成指',
      trade_date: '2024-12-08',
      close: 10456.78,
      open: 10523.45,
      high: 10567.89,
      low: 10398.12,
      pre_close: 10523.45,
      change: -66.67,
      pct_chg: -0.63,
      vol: 345678901,
      amount: 456789012345
    },
    {
      ts_code: '399006.SZ',
      name: '创业板指',
      trade_date: '2024-12-08',
      close: 2134.56,
      open: 2156.78,
      high: 2167.89,
      low: 2123.45,
      pre_close: 2156.78,
      change: -22.22,
      pct_chg: -1.03,
      vol: 456789012,
      amount: 567890123456
    }
  ]

  const mockDistributionData = [
    { range: '>9%', count: 45, percentage: 0.86 },
    { range: '7-9%', count: 123, percentage: 2.35 },
    { range: '5-7%', count: 234, percentage: 4.47 },
    { range: '3-5%', count: 456, percentage: 8.71 },
    { range: '1-3%', count: 789, percentage: 15.08 },
    { range: '0-1%', count: 509, percentage: 9.73 },
    { range: '0%', count: 244, percentage: 4.66 },
    { range: '0~-1%', count: 567, percentage: 10.84 },
    { range: '-1~-3%', count: 890, percentage: 17.01 },
    { range: '-3~-5%', count: 678, percentage: 12.95 },
    { range: '-5~-7%', count: 345, percentage: 6.59 },
    { range: '-7~-9%', count: 234, percentage: 4.47 },
    { range: '<-9%', count: 120, percentage: 2.29 }
  ]

  const currentMarketData = marketData || mockMarketData
  const currentIndexData = indexData || mockIndexData
  const currentDistributionData = distributionData || mockDistributionData

  // 准备饼图数据
  const pieData: PieChartData[] = [
    { name: '上涨', value: currentMarketData.up_count, color: '#10b981' },
    { name: '下跌', value: currentMarketData.down_count, color: '#ef4444' },
    { name: '平盘', value: currentMarketData.flat_count, color: '#6b7280' }
  ]

  const isLoading = marketLoading || indexLoading || distributionLoading

  if (isLoading) {
    return (
      <div className={className}>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          {[...Array(4)].map((_, i) => (
            <Card key={i}>
              <CardContent className="p-6">
                <div className="animate-pulse">
                  <div className="h-4 bg-muted rounded w-3/4 mb-2"></div>
                  <div className="h-8 bg-muted rounded w-1/2"></div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    )
  }

  return (
    <div className={className}>
      {/* 市场概览指标卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {/* 涨跌家数 */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">涨跌家数</p>
                <div className="flex items-center gap-2 mt-1">
                  <span className="text-2xl font-bold text-bull">{currentMarketData.up_count}</span>
                  <span className="text-sm text-muted-foreground">/</span>
                  <span className="text-2xl font-bold text-bear">{currentMarketData.down_count}</span>
                </div>
                <p className="text-xs text-muted-foreground mt-1">
                  平盘: {currentMarketData.flat_count}
                </p>
              </div>
              <div className="flex flex-col items-center">
                <TrendingUp className="h-8 w-8 text-bull" />
                <span className="text-xs text-muted-foreground mt-1">
                  {formatPercent((currentMarketData.up_count / currentMarketData.total_stocks) * 100, 1)}
                </span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 成交量 */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">成交量</p>
                <p className="text-2xl font-bold">{formatLargeNumber(currentMarketData.total_volume)}</p>
                <p className="text-xs text-muted-foreground mt-1">
                  成交额: {formatLargeNumber(currentMarketData.total_amount)}
                </p>
              </div>
              <Activity className="h-8 w-8 text-primary" />
            </div>
          </CardContent>
        </Card>

        {/* 涨停跌停 */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">涨停/跌停</p>
                <div className="flex items-center gap-2 mt-1">
                  <span className="text-2xl font-bold text-bull">{currentMarketData.limit_up_count}</span>
                  <span className="text-sm text-muted-foreground">/</span>
                  <span className="text-2xl font-bold text-bear">{currentMarketData.limit_down_count}</span>
                </div>
              </div>
              <BarChart3 className="h-8 w-8 text-orange-500" />
            </div>
          </CardContent>
        </Card>

        {/* 平均涨跌幅 */}
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">平均涨跌幅</p>
                <p className={`text-2xl font-bold ${getTrendColorClass(getStockTrend(currentMarketData.avg_pct_chg))}`}>
                  {formatPercent(currentMarketData.avg_pct_chg)}
                </p>
              </div>
              {currentMarketData.avg_pct_chg >= 0 ? (
                <TrendingUp className="h-8 w-8 text-bull" />
              ) : (
                <TrendingDown className="h-8 w-8 text-bear" />
              )}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* 主要指数 */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            主要指数
          </CardTitle>
          <CardDescription>
            上证指数、深证成指、创业板指实时行情
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {currentIndexData.map((index) => (
              <div key={index.ts_code} className="p-4 border rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-medium">{index.name}</h3>
                  <span className="text-xs text-muted-foreground">{index.ts_code}</span>
                </div>
                <div className="flex items-center gap-4">
                  <div>
                    <p className="text-2xl font-bold">{formatNumber(index.close, 2)}</p>
                    <div className="flex items-center gap-2 mt-1">
                      <span className={`text-sm font-medium ${getTrendColorClass(getStockTrend(index.change))}`}>
                        {index.change > 0 ? '+' : ''}{formatNumber(index.change, 2)}
                      </span>
                      <span className={`text-sm ${getTrendColorClass(getStockTrend(index.pct_chg))}`}>
                        ({index.pct_chg > 0 ? '+' : ''}{formatPercent(index.pct_chg)})
                      </span>
                    </div>
                  </div>
                  <div className="text-right text-xs text-muted-foreground">
                    <p>最高: {formatNumber(index.high, 2)}</p>
                    <p>最低: {formatNumber(index.low, 2)}</p>
                    <p>成交量: {formatLargeNumber(index.vol)}</p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* 涨跌分布和图表 */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* 涨跌家数饼图 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <PieChart className="h-5 w-5" />
              涨跌分布
            </CardTitle>
            <CardDescription>
              今日股票涨跌家数分布情况
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <RechartsPieChart>
                  <Pie
                    data={pieData}
                    cx="50%"
                    cy="50%"
                    innerRadius={60}
                    outerRadius={100}
                    paddingAngle={2}
                    dataKey="value"
                  >
                    {pieData.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={entry.color} />
                    ))}
                  </Pie>
                  <Tooltip 
                    formatter={(value: number, name: string) => [
                      `${value} 只 (${formatPercent((value / currentMarketData.total_stocks) * 100, 1)})`,
                      name
                    ]}
                  />
                </RechartsPieChart>
              </ResponsiveContainer>
            </div>
            <div className="flex justify-center gap-6 mt-4">
              {pieData.map((item) => (
                <div key={item.name} className="flex items-center gap-2">
                  <div 
                    className="w-3 h-3 rounded-full" 
                    style={{ backgroundColor: item.color }}
                  ></div>
                  <span className="text-sm">{item.name}: {item.value}</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* 涨跌幅分布柱状图 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              涨跌幅分布
            </CardTitle>
            <CardDescription>
              不同涨跌幅区间的股票数量分布
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="h-64">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={currentDistributionData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis 
                    dataKey="range" 
                    tick={{ fontSize: 10 }}
                    angle={-45}
                    textAnchor="end"
                    height={60}
                  />
                  <YAxis tick={{ fontSize: 10 }} />
                  <Tooltip 
                    formatter={(value: number, name: string) => [
                      `${value} 只`,
                      '股票数量'
                    ]}
                    labelFormatter={(label: string) => `涨跌幅: ${label}`}
                  />
                  <Bar 
                    dataKey="count" 
                    fill="#3b82f6"
                    radius={[2, 2, 0, 0]}
                  />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
