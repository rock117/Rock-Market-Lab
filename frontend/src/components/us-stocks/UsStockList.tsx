'use client'

import React, { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { usStockApi } from '@/services/api'
import { UsStock } from '@/types'
import { formatNumber, formatMarketCap, formatPercent, formatDate, getTrendColorClass, getStockTrend } from '@/lib/utils'
import { Search, Filter, TrendingUp, TrendingDown, Building2, Globe } from 'lucide-react'

interface UsStockListProps {
  className?: string
}

export default function UsStockList({ className }: UsStockListProps) {
  const [page, setPage] = useState(1)
  const [pageSize] = useState(50)
  const [filters, setFilters] = useState({
    exchange: '',
    industry: '',
    search: ''
  })

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ['us-stocks', page, pageSize, filters],
    queryFn: () => usStockApi.getUsStocks({
      page,
      page_size: pageSize,
      exchange: filters.exchange || undefined,
      industry: filters.industry || undefined,
    }),
    staleTime: 5 * 60 * 1000, // 5分钟缓存
  })

  // 模拟数据（在实际API未完成时使用）
  const mockData = {
    items: [
      {
        symbol: 'AAPL',
        name: 'Apple Inc.',
        exchange: 'NASDAQ',
        industry: 'Technology',
        sector: 'Consumer Electronics',
        market_cap: 30000000, // 3万亿美元，以万元为单位
        pe_ratio: 28.5,
        roe: 15.6,
        list_date: '1980-12-12',
        description: 'Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.',
        website: 'https://www.apple.com',
        employee_count: 164000,
        founded_date: '1976-04-01',
        address: 'One Apple Park Way, Cupertino, CA 95014'
      },
      {
        symbol: 'MSFT',
        name: 'Microsoft Corporation',
        exchange: 'NASDAQ',
        industry: 'Technology',
        sector: 'Software',
        market_cap: 28000000,
        pe_ratio: 32.1,
        roe: 18.2,
        list_date: '1986-03-13',
        description: 'Microsoft Corporation develops, licenses, and supports software, services, devices, and solutions worldwide.',
        website: 'https://www.microsoft.com',
        employee_count: 221000,
        founded_date: '1975-04-04',
        address: 'One Microsoft Way, Redmond, WA 98052'
      },
      {
        symbol: 'GOOGL',
        name: 'Alphabet Inc.',
        exchange: 'NASDAQ',
        industry: 'Technology',
        sector: 'Internet Services',
        market_cap: 17000000,
        pe_ratio: 25.8,
        roe: 14.3,
        list_date: '2004-08-19',
        description: 'Alphabet Inc. provides online advertising services in the United States, Europe, the Middle East, Africa, the Asia-Pacific, Canada, and Latin America.',
        website: 'https://www.alphabet.com',
        employee_count: 190000,
        founded_date: '1998-09-04',
        address: '1600 Amphitheatre Parkway, Mountain View, CA 94043'
      },
      {
        symbol: 'AMZN',
        name: 'Amazon.com Inc.',
        exchange: 'NASDAQ',
        industry: 'Technology',
        sector: 'E-commerce',
        market_cap: 15000000,
        pe_ratio: 45.2,
        roe: 12.8,
        list_date: '1997-05-15',
        description: 'Amazon.com, Inc. engages in the retail sale of consumer products and subscriptions in North America and internationally.',
        website: 'https://www.amazon.com',
        employee_count: 1540000,
        founded_date: '1994-07-05',
        address: '410 Terry Avenue North, Seattle, WA 98109'
      },
      {
        symbol: 'TSLA',
        name: 'Tesla Inc.',
        exchange: 'NASDAQ',
        industry: 'Automotive',
        sector: 'Electric Vehicles',
        market_cap: 8000000,
        pe_ratio: 65.4,
        roe: 19.3,
        list_date: '2010-06-29',
        description: 'Tesla, Inc. designs, develops, manufactures, leases, and sells electric vehicles, and energy generation and storage systems.',
        website: 'https://www.tesla.com',
        employee_count: 140000,
        founded_date: '2003-07-01',
        address: '1 Tesla Road, Austin, TX 78725'
      }
    ] as UsStock[],
    total: 5,
    page: 1,
    page_size: 50,
    total_pages: 1
  }

  const stockData = data || mockData

  if (isLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="h-5 w-5" />
            美股列表
          </CardTitle>
          <CardDescription>
            展示美股市场主要公司的基本信息和财务指标
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

  if (error) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-destructive">
            <Globe className="h-5 w-5" />
            美股列表 - 加载失败
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">
            <p className="text-muted-foreground mb-4">数据加载失败，请稍后重试</p>
            <button 
              onClick={() => refetch()}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
            >
              重新加载
            </button>
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Globe className="h-5 w-5" />
          美股列表
        </CardTitle>
        <CardDescription>
          展示美股市场主要公司的基本信息和财务指标 (共 {stockData.total} 只股票)
        </CardDescription>
      </CardHeader>
      <CardContent>
        {/* 筛选器 */}
        <div className="mb-6 flex flex-wrap gap-4">
          <div className="flex items-center gap-2">
            <Search className="h-4 w-4 text-muted-foreground" />
            <input
              type="text"
              placeholder="搜索股票代码或名称..."
              className="px-3 py-2 border rounded-md text-sm w-64"
              value={filters.search}
              onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
            />
          </div>
          <select
            className="px-3 py-2 border rounded-md text-sm"
            value={filters.exchange}
            onChange={(e) => setFilters(prev => ({ ...prev, exchange: e.target.value }))}
          >
            <option value="">所有交易所</option>
            <option value="NASDAQ">NASDAQ</option>
            <option value="NYSE">NYSE</option>
          </select>
          <select
            className="px-3 py-2 border rounded-md text-sm"
            value={filters.industry}
            onChange={(e) => setFilters(prev => ({ ...prev, industry: e.target.value }))}
          >
            <option value="">所有行业</option>
            <option value="Technology">科技</option>
            <option value="Healthcare">医疗保健</option>
            <option value="Financial">金融</option>
            <option value="Automotive">汽车</option>
          </select>
        </div>

        {/* 股票列表表格 */}
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[100px]">代码</TableHead>
                <TableHead className="w-[200px]">公司名称</TableHead>
                <TableHead className="w-[80px]">交易所</TableHead>
                <TableHead className="w-[120px]">行业</TableHead>
                <TableHead className="w-[120px] text-right">市值</TableHead>
                <TableHead className="w-[80px] text-right">PE</TableHead>
                <TableHead className="w-[80px] text-right">ROE</TableHead>
                <TableHead className="w-[100px]">上市时间</TableHead>
                <TableHead className="w-[100px] text-right">员工数</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {stockData.items.map((stock) => (
                <TableRow key={stock.symbol} className="hover:bg-muted/50">
                  <TableCell className="font-mono font-medium">
                    {stock.symbol}
                  </TableCell>
                  <TableCell>
                    <div className="flex flex-col">
                      <span className="font-medium">{stock.name}</span>
                      <span className="text-xs text-muted-foreground">{stock.sector}</span>
                    </div>
                  </TableCell>
                  <TableCell>
                    <span className="inline-flex items-center px-2 py-1 rounded-full text-xs bg-blue-100 text-blue-800">
                      {stock.exchange}
                    </span>
                  </TableCell>
                  <TableCell>
                    <span className="text-sm">{stock.industry}</span>
                  </TableCell>
                  <TableCell className="text-right font-medium">
                    {formatMarketCap(stock.market_cap)}
                  </TableCell>
                  <TableCell className="text-right">
                    {formatNumber(stock.pe_ratio, 1)}
                  </TableCell>
                  <TableCell className="text-right">
                    <span className={getTrendColorClass(getStockTrend(stock.roe))}>
                      {formatPercent(stock.roe, 1)}
                    </span>
                  </TableCell>
                  <TableCell>
                    {formatDate(stock.list_date)}
                  </TableCell>
                  <TableCell className="text-right">
                    {formatNumber(stock.employee_count, 0)}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        {/* 分页 */}
        {stockData.total_pages > 1 && (
          <div className="flex items-center justify-between mt-4">
            <div className="text-sm text-muted-foreground">
              显示 {((page - 1) * pageSize) + 1} - {Math.min(page * pageSize, stockData.total)} 条，共 {stockData.total} 条
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setPage(p => Math.max(1, p - 1))}
                disabled={page === 1}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                上一页
              </button>
              <span className="px-3 py-1 text-sm">
                {page} / {stockData.total_pages}
              </span>
              <button
                onClick={() => setPage(p => Math.min(stockData.total_pages, p + 1))}
                disabled={page === stockData.total_pages}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                下一页
              </button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
