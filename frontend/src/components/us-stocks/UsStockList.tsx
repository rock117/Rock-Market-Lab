'use client'

import React, { useState, useEffect, useCallback, useRef, useMemo, useTransition } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Tooltip } from '@/components/ui/tooltip'
import { usStockApi } from '@/services/api'
import { UsStock } from '@/types'
import { formatNumber, formatMarketCap, formatPercent, formatDate, getTrendColorClass, getStockTrend } from '@/lib/utils'
import { Search, Filter, TrendingUp, TrendingDown, Building2, Globe } from 'lucide-react'

interface UsStockListProps {
  className?: string
}

// 搜索框组件 - 完全独立的状态管理，避免父组件渲染影响
const SearchBox = React.memo(({ 
  onSearch
}: { 
  onSearch: (keyword: string) => void
}) => {
  console.log('🎨 SearchBox 渲染')
  
  // 搜索框内部管理自己的状态
  const [inputValue, setInputValue] = useState('')
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    console.log('📝 SearchBox handleSubmit 调用')
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
}, (prevProps, nextProps) => {
  // 自定义比较函数：只要onSearch引用相同就不重新渲染
  const shouldNotRerender = prevProps.onSearch === nextProps.onSearch
  console.log('🔍 SearchBox props比较:', shouldNotRerender ? '相同，不渲染' : '不同，需要渲染')
  return shouldNotRerender
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
  console.log('📊 StockTable 渲染')
  
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
              <TableHead className="w-[80px]">官网</TableHead>
              <TableHead className="min-w-[200px]">主营业务</TableHead>
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
                <TableCell>
                  {stock.webAddress ? (
                    <a 
                      href={stock.webAddress} 
                      target="_blank" 
                      rel="noopener noreferrer"
                      className="text-blue-600 hover:text-blue-800 hover:underline text-sm inline-flex items-center gap-1"
                    >
                      查看
                      <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                      </svg>
                    </a>
                  ) : 'N/A'}
                </TableCell>
                <TableCell>
                  {stock.businessDescription ? (
                    <Tooltip content={stock.businessDescription}>
                      <div className="text-sm text-muted-foreground max-w-[300px] truncate cursor-help">
                        {stock.businessDescription}
                      </div>
                    </Tooltip>
                  ) : (
                    <span className="text-sm text-muted-foreground">N/A</span>
                  )}
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
  console.log('🔄 UsStockList 渲染')
  
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [searchKeyword, setSearchKeyword] = useState('')
  const [isPending, startTransition] = useTransition()

  // 搜索回调函数 - 使用 useTransition 降低优先级，减少渲染次数
  const handleSearch = useCallback((keyword: string) => {
    console.log('🔍 执行搜索:', keyword)
    startTransition(() => {
      // 批量更新状态，减少渲染次数
      setSearchKeyword(keyword)
      setPage(1)
    })
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
    refetchOnWindowFocus: false, // 避免窗口聚焦时重新获取
    refetchOnMount: false, // 避免组件挂载时重新获取
    notifyOnChangeProps: ['data', 'error'], // 只在关键属性变化时通知
  })

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

  if (!data) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="h-5 w-5" />
            美股列表
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-center h-64">
            <div className="text-muted-foreground">暂无数据</div>
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
          展示美股市场主要公司的基本信息和财务指标 (共 {data.total} 只股票)
        </CardDescription>
      </CardHeader>
      <CardContent>
        {/* 搜索框 - 完全独立，只在点击搜索时通知父组件 */}
        <SearchBox onSearch={handleSearch} />

        {/* 搜索加载状态指示器 */}
        {isPending && (
          <div className="mb-4 px-4 py-2 bg-blue-50 border border-blue-200 rounded-md text-sm text-blue-700">
            正在搜索中...
          </div>
        )}

        {/* 使用独立的表格组件 */}
        <StockTable
          stockData={data}
          page={page}
          pageSize={pageSize}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      </CardContent>
    </Card>
  )
}

// 使用 React.memo 包装主组件，减少不必要的重新渲染
export default React.memo(UsStockList)
