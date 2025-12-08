'use client'

import React, { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { stockDetailApi } from '@/services/api'
import { StockDetail as StockDetailType } from '@/types'
import { 
  formatNumber, 
  formatLargeNumber, 
  formatPercent, 
  formatDate, 
  getTrendColorClass, 
  getStockTrend 
} from '@/lib/utils'
import { 
  TrendingUp, 
  TrendingDown, 
  Building2, 
  Users, 
  DollarSign, 
  BarChart3, 
  PieChart,
  ArrowUpDown,
  Target,
  Briefcase
} from 'lucide-react'

interface StockDetailProps {
  className?: string
}

export default function StockDetail({ className }: StockDetailProps) {
  const [selectedStock, setSelectedStock] = useState('000001.SZ')
  
  const { data: stockDetail, isLoading, error } = useQuery({
    queryKey: ['stock-detail', selectedStock],
    queryFn: () => stockDetailApi.getStockDetail(selectedStock),
    staleTime: 5 * 60 * 1000, // 5分钟缓存
  })

  const stockOptions = [
    { value: '000001.SZ', label: '平安银行 (000001.SZ)' },
    { value: '000002.SZ', label: '万科A (000002.SZ)' },
    { value: '600036.SH', label: '招商银行 (600036.SH)' }
  ]

  if (isLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            股票详情
          </CardTitle>
          <CardDescription>
            详细的股票基本面和交易数据分析
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-center h-64">
            <div className="text-muted-foreground">加载中...</div>
          </div>
        </CardContent>
      </Card>
    )
  }

  if (error || !stockDetail) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-destructive">
            <Building2 className="h-5 w-5" />
            股票详情 - 加载失败
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">
            <p className="text-muted-foreground">数据加载失败，请稍后重试</p>
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className={className}>
      {/* 股票选择器 */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            股票详情
          </CardTitle>
          <CardDescription>
            选择股票查看详细的基本面和交易数据
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-4 mb-4">
            <label className="text-sm font-medium">选择股票:</label>
            <select
              value={selectedStock}
              onChange={(e) => setSelectedStock(e.target.value)}
              className="px-3 py-2 border rounded-md text-sm min-w-[200px]"
            >
              {stockOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>
          
          {/* 股票基本信息 */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">当前价格</p>
              <p className="text-2xl font-bold">{formatNumber(stockDetail.current_price, 2)}</p>
              <div className="flex items-center gap-2 mt-1">
                <span className={`text-sm font-medium ${getTrendColorClass(getStockTrend(stockDetail.change))}`}>
                  {stockDetail.change > 0 ? '+' : ''}{formatNumber(stockDetail.change, 2)}
                </span>
                <span className={`text-sm ${getTrendColorClass(getStockTrend(stockDetail.pct_chg))}`}>
                  ({stockDetail.pct_chg > 0 ? '+' : ''}{formatPercent(stockDetail.pct_chg)})
                </span>
              </div>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">PE / PB</p>
              <div className="flex items-center gap-2">
                <span className="text-lg font-bold">{formatNumber(stockDetail.pe_ratio, 1)}</span>
                <span className="text-muted-foreground">/</span>
                <span className="text-lg font-bold">{formatNumber(stockDetail.pb_ratio, 2)}</span>
              </div>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">5日涨幅</p>
              <p className={`text-lg font-bold ${getTrendColorClass(getStockTrend(stockDetail.five_day_return))}`}>
                {stockDetail.five_day_return > 0 ? '+' : ''}{formatPercent(stockDetail.five_day_return)}
              </p>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">股东人数</p>
              <p className="text-lg font-bold">{formatNumber(stockDetail.shareholder_count.holder_count, 0)}</p>
              <p className="text-xs text-muted-foreground">
                变化: {stockDetail.shareholder_count.change_ratio > 0 ? '+' : ''}{formatPercent(stockDetail.shareholder_count.change_ratio)}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* 基本面数据 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              基本面数据
            </CardTitle>
            <CardDescription>
              财务指标和盈利能力分析
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-2 gap-4">
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">ROE (净资产收益率)</p>
                <p className="text-xl font-bold text-bull">{formatPercent(stockDetail.fundamentals.roe)}</p>
              </div>
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">毛利率</p>
                <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.gross_margin)}</p>
              </div>
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">净利率</p>
                <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.net_margin)}</p>
              </div>
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">资产负债率</p>
                <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.debt_ratio)}</p>
              </div>
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">营收增长率</p>
                <p className={`text-xl font-bold ${getTrendColorClass(getStockTrend(stockDetail.fundamentals.revenue_growth))}`}>
                  {stockDetail.fundamentals.revenue_growth > 0 ? '+' : ''}{formatPercent(stockDetail.fundamentals.revenue_growth)}
                </p>
              </div>
              <div className="p-3 bg-muted/30 rounded-lg">
                <p className="text-sm text-muted-foreground">净利润增长率</p>
                <p className={`text-xl font-bold ${getTrendColorClass(getStockTrend(stockDetail.fundamentals.net_profit_growth))}`}>
                  {stockDetail.fundamentals.net_profit_growth > 0 ? '+' : ''}{formatPercent(stockDetail.fundamentals.net_profit_growth)}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 融资融券数据 */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <DollarSign className="h-5 w-5" />
              融资融券
            </CardTitle>
            <CardDescription>
              融资融券交易数据
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                <span className="text-sm text-muted-foreground">融资余额</span>
                <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.margin_balance)}</span>
              </div>
              <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                <span className="text-sm text-muted-foreground">融资买入额</span>
                <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.margin_buy)}</span>
              </div>
              <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                <span className="text-sm text-muted-foreground">融券余额</span>
                <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.short_balance)}</span>
              </div>
              <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                <span className="text-sm text-muted-foreground">融券卖出量</span>
                <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.short_sell)}</span>
              </div>
              <div className="flex justify-between items-center p-3 bg-bull/10 rounded-lg border border-bull/20">
                <span className="text-sm text-muted-foreground">融资融券比例</span>
                <span className="font-bold text-bull">{formatPercent(stockDetail.margin_trading.margin_ratio)}</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* 概念板块 */}
      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            概念板块
          </CardTitle>
          <CardDescription>
            股票所属的概念和行业板块
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <PieChart className="h-4 w-4" />
                概念
              </h4>
              <div className="flex flex-wrap gap-2">
                {stockDetail.concepts.map((concept, index) => (
                  <span 
                    key={index}
                    className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                  >
                    {concept}
                  </span>
                ))}
              </div>
            </div>
            <div>
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <Briefcase className="h-4 w-4" />
                板块
              </h4>
              <div className="flex flex-wrap gap-2">
                {stockDetail.sectors.map((sector, index) => (
                  <span 
                    key={index}
                    className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm"
                  >
                    {sector}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 大宗交易 */}
      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <ArrowUpDown className="h-5 w-5" />
            大宗交易
          </CardTitle>
          <CardDescription>
            近期大宗交易记录
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>交易日期</TableHead>
                  <TableHead className="text-right">成交价格</TableHead>
                  <TableHead className="text-right">成交量(万股)</TableHead>
                  <TableHead className="text-right">成交额(万元)</TableHead>
                  <TableHead>买方</TableHead>
                  <TableHead>卖方</TableHead>
                  <TableHead className="text-right">溢价率</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {stockDetail.block_trades.map((trade, index) => (
                  <TableRow key={index}>
                    <TableCell>{formatDate(trade.trade_date)}</TableCell>
                    <TableCell className="text-right font-medium">
                      {formatNumber(trade.price, 2)}
                    </TableCell>
                    <TableCell className="text-right">
                      {formatNumber(trade.volume / 10000, 0)}
                    </TableCell>
                    <TableCell className="text-right">
                      {formatNumber(trade.amount / 10000, 0)}
                    </TableCell>
                    <TableCell className="text-sm">{trade.buyer}</TableCell>
                    <TableCell className="text-sm">{trade.seller}</TableCell>
                    <TableCell className={`text-right font-medium ${getTrendColorClass(getStockTrend(trade.premium_rate))}`}>
                      {trade.premium_rate > 0 ? '+' : ''}{formatPercent(trade.premium_rate)}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>

      {/* 增减持数据 */}
      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Users className="h-5 w-5" />
            股东增减持
          </CardTitle>
          <CardDescription>
            主要股东增减持变动情况
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>股东名称</TableHead>
                  <TableHead>变动类型</TableHead>
                  <TableHead className="text-right">变动股数(万股)</TableHead>
                  <TableHead className="text-right">变动比例</TableHead>
                  <TableHead>变动日期</TableHead>
                  <TableHead className="text-right">变动后持股比例</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {stockDetail.shareholding_changes.map((change, index) => (
                  <TableRow key={index}>
                    <TableCell className="font-medium max-w-[200px] truncate">
                      {change.holder_name}
                    </TableCell>
                    <TableCell>
                      <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                        change.change_type === 'increase' 
                          ? 'bg-bull/10 text-bull border border-bull/20' 
                          : 'bg-bear/10 text-bear border border-bear/20'
                      }`}>
                        {change.change_type === 'increase' ? '增持' : '减持'}
                      </span>
                    </TableCell>
                    <TableCell className="text-right">
                      {formatNumber(Math.abs(change.change_shares) / 10000, 0)}
                    </TableCell>
                    <TableCell className={`text-right font-medium ${getTrendColorClass(getStockTrend(change.change_ratio))}`}>
                      {change.change_ratio > 0 ? '+' : ''}{formatPercent(change.change_ratio)}
                    </TableCell>
                    <TableCell>{formatDate(change.change_date)}</TableCell>
                    <TableCell className="text-right font-medium">
                      {formatPercent(change.current_ratio)}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
