'use client'

import React, { useState, useEffect, useCallback, useRef, useMemo, useTransition } from 'react'
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

// 搜索框组件 - 带搜索按钮
const SearchBox = React.memo(({ 
  onSearch 
}: { 
  onSearch: (keyword: string) => void 
}) => {
  const [inputValue, setInputValue] = useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    onSearch(inputValue)
  }

  const handleClear = () => {
    setInputValue('')
    onSearch('')
  }

  return (
    <div className="mb-6">
      <form onSubmit={handleSubmit} className="flex items-center gap-3 max-w-2xl">
        <div className="flex-1 flex items-center gap-2 px-3 py-2 border rounded-md focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-transparent">
          <Search className="h-4 w-4 text-muted-foreground flex-shrink-0" />
          <input
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            placeholder="输入股票代码或名称，按Enter或点击搜索按钮..."
            className="flex-1 text-sm focus:outline-none bg-transparent"
            autoComplete="off"
          />
        </div>
        {inputValue && (
          <button
            type="button"
            onClick={handleClear}
            className="px-4 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
          >
            清空
          </button>
        )}
        <button
          type="submit"
          className="px-6 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:bg-primary/90 transition-colors flex items-center gap-2"
        >
          <Search className="h-4 w-4" />
          搜索
        </button>
      </form>
    </div>
  )
})

SearchBox.displayName = 'SearchBox'

// 独立的表格组件，只在数据变化时重新渲染
const StockTable = React.memo(({ 
  stockData, 
  page, 
  pageSize, 
  onPageChange, 
  onPageSizeChange 
}: {
  stockData: PagedResponse<UsStock>
  page: number
  pageSize: number
  onPageChange: (newPage: number) => void
  onPageSizeChange: (newSize: number) => void
}) => {
  return (
    <>
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
              <TableRow key={stock.tsCode || stock.symbol} className="hover:bg-muted/50">
                <TableCell className="font-mono font-medium">
                  {stock.tsCode || stock.symbol}
                </TableCell>
                <TableCell>
                  <div className="flex flex-col">
                    <span className="font-medium">{stock.name}</span>
                    <span className="text-xs text-muted-foreground">
                      {stock.sectorName || stock.sector}
                    </span>
                  </div>
                </TableCell>
                <TableCell>
                  <span className="inline-flex items-center px-2 py-1 rounded-full text-xs bg-blue-100 text-blue-800">
                    {stock.exchangeId || stock.exchange}
                  </span>
                </TableCell>
                <TableCell>
                  <span className="text-sm">{stock.industryName || stock.industry}</span>
                </TableCell>
                <TableCell className="text-right font-medium">
                  {stock.market_cap ? formatMarketCap(stock.market_cap) : 'N/A'}
                </TableCell>
                <TableCell className="text-right">
                  {stock.pe_ratio ? formatNumber(stock.pe_ratio, 1) : 'N/A'}
                </TableCell>
                <TableCell className="text-right">
                  {stock.roe ? (
                    <span className={getTrendColorClass(getStockTrend(stock.roe))}>
                      {formatPercent(stock.roe, 1)}
                    </span>
                  ) : 'N/A'}
                </TableCell>
                <TableCell>
                  {stock.list_date ? formatDate(stock.list_date) : 'N/A'}
                </TableCell>
                <TableCell className="text-right">
                  {stock.employee_count ? formatNumber(stock.employee_count, 0) : 'N/A'}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      {/* 分页控制 */}
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mt-6 pt-4 border-t">
        <div className="flex items-center gap-4">
          <div className="text-sm text-muted-foreground">
            显示 {((page - 1) * pageSize) + 1} - {Math.min(page * pageSize, stockData.total)} 条，共 {stockData.total} 条
          </div>
          <div className="flex items-center gap-2">
            <span className="text-sm text-muted-foreground">每页显示</span>
            <select
              value={pageSize}
              onChange={(e) => {
                onPageSizeChange(Number(e.target.value))
                onPageChange(1)
              }}
              className="px-2 py-1 border rounded text-sm"
            >
              <option value={5}>5</option>
              <option value={10}>10</option>
              <option value={20}>20</option>
              <option value={50}>50</option>
            </select>
          </div>
        </div>

        {stockData.total_pages > 1 && (
          <div className="flex items-center gap-2">
            <button
              onClick={() => onPageChange(1)}
              disabled={page === 1}
              className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
            >
              首页
            </button>
            <button
              onClick={() => onPageChange(page - 1)}
              disabled={page === 1}
              className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
            >
              上一页
            </button>
            <span className="text-sm text-muted-foreground">
              第 {page} / {stockData.total_pages} 页
            </span>
            <button
              onClick={() => onPageChange(page + 1)}
              disabled={page === stockData.total_pages}
              className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
            >
              下一页
            </button>
            <button
              onClick={() => onPageChange(stockData.total_pages)}
              disabled={page === stockData.total_pages}
              className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
            >
              末页
            </button>
          </div>
        )}
      </div>
    </>
  )
})

StockTable.displayName = 'StockTable'

function UsStockList({ className }: UsStockListProps) {
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [searchKeyword, setSearchKeyword] = useState('')

  // 搜索回调函数
  const handleSearch = useCallback((keyword: string) => {
    setSearchKeyword(keyword)
    setPage(1) // 搜索时重置到第一页
  }, [])

  // 分页回调函数，使用useCallback确保引用稳定
  const handlePageChange = useCallback((newPage: number) => {
    setPage(newPage)
  }, [])

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPageSize(newSize)
  }, [])

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ['us-stocks', page, pageSize, searchKeyword],
    queryFn: () => usStockApi.getUsStocks({
      page,
      page_size: pageSize,
      keyword: searchKeyword || undefined,
    }),
    staleTime: 5 * 60 * 1000, // 5分钟缓存
    keepPreviousData: true, // 保持之前的数据，避免闪烁
  })

  // 使用useMemo缓存模拟数据，避免重新渲染
  const mockData = useMemo(() => ({
    items: [
      {
        symbol: 'AAPL',
        name: 'Apple Inc.',
        exchange: 'NASDAQ',
        industry: 'Technology',
        sector: 'Consumer Electronics',
        market_cap: 30000000,
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
  }), [])

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
        {/* 搜索框 - 点击搜索按钮才触发查询 */}
        <SearchBox onSearch={handleSearch} />

        {/* 使用独立的表格组件 */}
        <StockTable
          stockData={stockData}
          page={page}
          pageSize={pageSize}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      </CardContent>
    </Card>
  )
}

export default UsStockList
