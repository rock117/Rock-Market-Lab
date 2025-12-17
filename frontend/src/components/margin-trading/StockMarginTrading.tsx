'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import MarginBalanceKLineChart from '@/components/charts/MarginBalanceKLineChart'
import { marginTradingApi, stockDetailApi } from '@/services/api'
import { debounce, formatDate, formatNumber } from '@/lib/utils'
import { Building2, Calendar, Filter, Scale, Search, X } from 'lucide-react'

interface StockSearchResult {
  ts_code: string
  name: string
}

function toISODate(date: Date): string {
  return date.toISOString().split('T')[0]
}

export default function StockMarginTrading({ className }: { className?: string }) {
  const defaultEnd = useMemo(() => new Date(), [])
  const defaultStart = useMemo(() => {
    const d = new Date()
    d.setDate(d.getDate() - 10)
    return d
  }, [])

  const [startDate, setStartDate] = useState(() => toISODate(defaultStart))
  const [endDate, setEndDate] = useState(() => toISODate(defaultEnd))

  const [selectedStock, setSelectedStock] = useState('000001.SZ')
  const [selectedStockName, setSelectedStockName] = useState<string>('')

  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  const { data: searchData } = useQuery({
    queryKey: ['search-stocks', searchKeyword],
    queryFn: () => stockDetailApi.searchStocks(searchKeyword),
    enabled: searchKeyword.length >= 1,
    staleTime: 2 * 60 * 1000,
  })

  useEffect(() => {
    if (searchData?.stocks) {
      setSearchResults(searchData.stocks)
      setShowSearchResults(true)
    }
  }, [searchData])

  const debouncedSearch = debounce((keyword: string) => {
    setSearchKeyword(keyword)
  }, 300)

  const selectStock = (stock: StockSearchResult) => {
    setSelectedStock(stock.ts_code)
    setSelectedStockName(stock.name)
    setSearchKeyword('')
    setShowSearchResults(false)
  }

  const clearSearch = () => {
    setSearchKeyword('')
    setShowSearchResults(false)
  }

  const {
    data,
    isLoading,
    error,
    refetch,
    isFetching,
  } = useQuery({
    queryKey: ['stock-margin-trading-kline', selectedStock, startDate, endDate],
    queryFn: () => marginTradingApi.getStockMarginTradingKLine({ stock: selectedStock, startDate, endDate }),
    staleTime: 60 * 1000,
    enabled: Boolean(selectedStock),
  })

  const title = useMemo(() => {
    const namePart = selectedStockName ? `${selectedStockName} (${selectedStock})` : selectedStock
    return `个股融资融券 - ${namePart}`
  }, [selectedStock, selectedStockName])

  const summary = useMemo(() => {
    const rows = (data?.data || [])
      .filter(r => r?.trade_date && Number.isFinite(r.close))
      .map(r => ({ date: r.trade_date, balance: r.close }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

    if (!rows.length) return null

    const maxBalance = Math.max(...rows.map(r => r.balance))
    const minBalance = Math.min(...rows.map(r => r.balance))
    const avgBalance = rows.reduce((sum, r) => sum + r.balance, 0) / rows.length
    const latest = rows[rows.length - 1]

    return {
      maxBalance,
      minBalance,
      avgBalance,
      latestDate: latest.date,
      latestBalance: latest.balance,
    }
  }, [data?.data])

  return (
    <div className={className}>
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            股票选择
          </CardTitle>
          <CardDescription>
            搜索并选择股票后，按时间范围查询对应的融资余额（单位：万元）
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="relative">
            <div className="flex items-center gap-2">
              <Search className="h-4 w-4 text-muted-foreground" />
              <div className="relative flex-1 max-w-md">
                <input
                  type="text"
                  placeholder="搜索股票代码或名称..."
                  className="w-full px-3 py-2 border rounded-md text-sm pr-8"
                  onChange={(e) => debouncedSearch(e.target.value)}
                  onFocus={() => searchResults.length > 0 && setShowSearchResults(true)}
                />
                {searchKeyword && (
                  <button
                    onClick={clearSearch}
                    className="absolute right-2 top-1/2 transform -translate-y-1/2 text-muted-foreground hover:text-foreground"
                  >
                    <X className="h-3 w-3" />
                  </button>
                )}
              </div>
            </div>

            {showSearchResults && searchResults.length > 0 && (
              <div className="absolute top-full left-6 right-0 mt-1 bg-white border rounded-md shadow-lg z-10 max-h-60 overflow-y-auto max-w-md">
                {searchResults.map((stock) => (
                  <div
                    key={stock.ts_code}
                    className="p-3 hover:bg-muted cursor-pointer border-b last:border-b-0"
                    onClick={() => selectStock(stock)}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <span className="font-medium">{stock.name}</span>
                        <span className="text-sm text-muted-foreground ml-2">{stock.ts_code}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="mt-4 p-3 bg-muted rounded-lg">
            <div className="flex items-center gap-2">
              <span className="text-sm text-muted-foreground">当前股票:</span>
              <span className="font-medium">{selectedStockName || '--'}</span>
              <span className="text-sm text-muted-foreground">({selectedStock})</span>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Filter className="h-5 w-5" />
            筛选条件
          </CardTitle>
          <CardDescription>
            结束时间默认今天，开始时间默认10天前
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col lg:flex-row lg:items-end gap-4">
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
              <Calendar className="h-4 w-4" />
              <span>筛选</span>
            </div>

            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground h-4 leading-4">开始时间</span>
              <input
                type="date"
                className="h-10 px-3 py-2 border rounded-md text-sm"
                value={startDate}
                onChange={(e) => setStartDate(e.target.value)}
                max={endDate}
              />
            </div>

            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground h-4 leading-4">结束时间</span>
              <input
                type="date"
                className="h-10 px-3 py-2 border rounded-md text-sm"
                value={endDate}
                onChange={(e) => setEndDate(e.target.value)}
                min={startDate}
              />
            </div>

            <div className="flex flex-col gap-1">
              <span className="text-xs text-muted-foreground h-4 leading-4">&nbsp;</span>
              <button
                type="button"
                onClick={() => refetch()}
                className="h-10 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors text-sm whitespace-nowrap"
                disabled={isFetching}
              >
                {isFetching ? '查询中...' : '查询'}
              </button>
            </div>
          </div>

          {error && (
            <div className="mt-4 text-sm text-destructive">
              数据加载失败，请稍后重试
            </div>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Scale className="h-5 w-5" />
            {title}
          </CardTitle>
          <CardDescription>
            {startDate} 至 {endDate}
            {isLoading ? ' | 加载中...' : ''}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {summary && (
            <div className="mb-4 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">最大融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.maxBalance, 2)} 万元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">最小融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.minBalance, 2)} 万元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">平均融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.avgBalance, 2)} 万元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">当前融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.latestBalance, 2)} 万元</div>
                <div className="mt-1 text-xs text-muted-foreground">{formatDate(summary.latestDate)}</div>
              </div>
            </div>
          )}
          <MarginBalanceKLineChart data={data?.data || []} title={title} unitLabel="万元" />
        </CardContent>
      </Card>
    </div>
  )
}
