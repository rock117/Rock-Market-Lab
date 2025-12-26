'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { stockDetailApi } from '@/services/api'
import { debounce, formatDate } from '@/lib/utils'
import { ArrowUpDown, Search, X } from 'lucide-react'

interface StockSearchResult {
  ts_code: string
  name: string
}

type TimeMode = 'custom' | 'quick'

type Trend = 'up' | 'down'

type StreakResult = {
  currentUp: number
  currentDown: number
  maxUp: number
  maxUpStart?: string
  maxUpEnd?: string
  maxDown: number
  maxDownStart?: string
  maxDownEnd?: string
  latestDate?: string
}

function toISODate(date: Date): string {
  return date.toISOString().split('T')[0]
}

function computeStreaks(rows: Array<{ trade_date: string; pct_chg: number }>): StreakResult {
  const sorted = rows
    .filter(r => r?.trade_date)
    .map(r => ({ ...r, pct_chg: Number(r.pct_chg) }))
    .filter(r => Number.isFinite(r.pct_chg))
    .sort((a, b) => new Date(a.trade_date).getTime() - new Date(b.trade_date).getTime())

  const nonZero = sorted.filter(r => r.pct_chg !== 0)
  const latest = sorted.length ? sorted[sorted.length - 1] : undefined

  let currentUp = 0
  let currentDown = 0

  if (nonZero.length) {
    const last = nonZero[nonZero.length - 1]
    const lastTrend: Trend = last.pct_chg > 0 ? 'up' : 'down'

    for (let i = nonZero.length - 1; i >= 0; i--) {
      const cur = nonZero[i]
      const curTrend: Trend = cur.pct_chg > 0 ? 'up' : 'down'
      if (curTrend !== lastTrend) break
      if (lastTrend === 'up') currentUp++
      else currentDown++
    }
  }

  let maxUp = 0
  let maxUpStart: string | undefined
  let maxUpEnd: string | undefined

  let maxDown = 0
  let maxDownStart: string | undefined
  let maxDownEnd: string | undefined

  let run = 0
  let runTrend: Trend | null = null
  let runStart: string | undefined

  const finalize = (endDate?: string) => {
    if (!runTrend || !runStart || !endDate) return

    if (runTrend === 'up') {
      if (run > maxUp) {
        maxUp = run
        maxUpStart = runStart
        maxUpEnd = endDate
      }
    } else {
      if (run > maxDown) {
        maxDown = run
        maxDownStart = runStart
        maxDownEnd = endDate
      }
    }
  }

  for (const row of nonZero) {
    const t: Trend = row.pct_chg > 0 ? 'up' : 'down'
    if (!runTrend) {
      runTrend = t
      run = 1
      runStart = row.trade_date
      continue
    }

    if (t === runTrend) {
      run++
      continue
    }

    finalize(row.trade_date)
    runTrend = t
    run = 1
    runStart = row.trade_date
  }

  if (nonZero.length) {
    finalize(nonZero[nonZero.length - 1].trade_date)
  }

  return {
    currentUp,
    currentDown,
    maxUp,
    maxUpStart,
    maxUpEnd,
    maxDown,
    maxDownStart,
    maxDownEnd,
    latestDate: latest?.trade_date,
  }
}

export default function StreakStats({ className }: { className?: string }) {
  const defaultEnd = useMemo(() => new Date(), [])
  const defaultStart = useMemo(() => {
    const d = new Date()
    d.setDate(d.getDate() - 90)
    return d
  }, [])

  const [selectedStock, setSelectedStock] = useState('000001.SZ')
  const [selectedStockName, setSelectedStockName] = useState<string>('')

  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  const [timeMode, setTimeMode] = useState<TimeMode>('quick')
  const [quickDays, setQuickDays] = useState<number>(30)

  const [startDate, setStartDate] = useState(() => toISODate(defaultStart))
  const [endDate, setEndDate] = useState(() => toISODate(defaultEnd))

  const setQuickTimeRange = (days: number) => {
    const end = new Date()
    const start = new Date()
    start.setDate(end.getDate() - days)
    setStartDate(toISODate(start))
    setEndDate(toISODate(end))
    setQuickDays(days)
  }

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

  const { data, isLoading, error, isFetching } = useQuery({
    queryKey: ['streak-stats', selectedStock, timeMode, startDate, endDate, quickDays],
    queryFn: () =>
      stockDetailApi.getStockHistory(selectedStock, {
        timeMode,
        startDate: timeMode === 'custom' ? startDate : undefined,
        endDate: timeMode === 'custom' ? endDate : undefined,
        timePeriod: timeMode === 'quick' ? quickDays : undefined,
      }),
    enabled: Boolean(selectedStock),
    staleTime: 60 * 1000,
  })

  const streak = useMemo(() => {
    const rows = (data?.data || []).map(r => ({ trade_date: r.trade_date, pct_chg: r.pct_chg }))
    return computeStreaks(rows)
  }, [data?.data])

  const title = useMemo(() => {
    const namePart = selectedStockName ? `${selectedStockName} (${selectedStock})` : selectedStock
    return `连涨/连跌统计 - ${namePart}`
  }, [selectedStock, selectedStockName])

  return (
    <div className={className}>
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <ArrowUpDown className="h-5 w-5" />
            {title}
          </CardTitle>
          <CardDescription>基于涨跌幅（pct_chg）统计当前连涨/连跌天数与区间最大连涨/连跌</CardDescription>
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

              <select
                value={timeMode}
                onChange={(e) => setTimeMode(e.target.value as TimeMode)}
                className="w-48 px-3 py-2 border rounded-md text-sm bg-background"
              >
                <option value="quick">快捷时间选择</option>
                <option value="custom">自定义时间范围</option>
              </select>

              {timeMode === 'custom' ? (
                <>
                  <input
                    type="date"
                    value={startDate}
                    onChange={(e) => setStartDate(e.target.value)}
                    className="w-40 px-3 py-2 border rounded-md text-sm"
                    max={endDate}
                  />
                  <input
                    type="date"
                    value={endDate}
                    onChange={(e) => setEndDate(e.target.value)}
                    className="w-40 px-3 py-2 border rounded-md text-sm"
                    min={startDate}
                  />
                </>
              ) : (
                <select
                  value={`${quickDays}`}
                  onChange={(e) => {
                    const days = Number(e.target.value)
                    if (Number.isFinite(days) && days > 0) {
                      setQuickTimeRange(days)
                    }
                  }}
                  className="w-40 px-3 py-2 border rounded-md text-sm bg-background"
                >
                  <option value="7">近7天</option>
                  <option value="30">近1个月</option>
                  <option value="90">近3个月</option>
                  <option value="180">近6个月</option>
                </select>
              )}
            </div>

            {showSearchResults && searchResults.length > 0 && (
              <div className="absolute top-full left-6 right-0 mt-1 bg-white border rounded-md shadow-lg z-10 max-h-60 overflow-y-auto max-w-md">
                {searchResults.map((stock) => (
                  <div
                    key={stock.ts_code}
                    className="p-3 hover:bg-muted cursor-pointer border-b last:border-b-0"
                    onClick={() => selectStock(stock)}
                  >
                    <div>
                      <span className="font-medium">{stock.name}</span>
                      <span className="text-sm text-muted-foreground ml-2">{stock.ts_code}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {isLoading && (
        <Card>
          <CardContent className="py-8 text-center text-muted-foreground">加载统计数据中...</CardContent>
        </Card>
      )}

      {error && (
        <Card>
          <CardContent className="py-8 text-center text-destructive">数据加载失败</CardContent>
        </Card>
      )}

      {!isLoading && !error && (
        <div className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <Card>
              <CardContent className="p-4">
                <div className="text-sm text-muted-foreground">当前连涨</div>
                <div className="text-2xl font-bold text-red-600">{streak.currentUp} 天</div>
                {streak.latestDate && (
                  <div className="text-xs text-muted-foreground mt-1">截至 {formatDate(streak.latestDate)}</div>
                )}
              </CardContent>
            </Card>

            <Card>
              <CardContent className="p-4">
                <div className="text-sm text-muted-foreground">当前连跌</div>
                <div className="text-2xl font-bold text-green-600">{streak.currentDown} 天</div>
                {streak.latestDate && (
                  <div className="text-xs text-muted-foreground mt-1">截至 {formatDate(streak.latestDate)}</div>
                )}
              </CardContent>
            </Card>

            <Card>
              <CardContent className="p-4">
                <div className="text-sm text-muted-foreground">区间最大连涨</div>
                <div className="text-2xl font-bold">{streak.maxUp} 天</div>
                {streak.maxUpStart && streak.maxUpEnd && (
                  <div className="text-xs text-muted-foreground mt-1">
                    {formatDate(streak.maxUpStart)} - {formatDate(streak.maxUpEnd)}
                  </div>
                )}
              </CardContent>
            </Card>

            <Card>
              <CardContent className="p-4">
                <div className="text-sm text-muted-foreground">区间最大连跌</div>
                <div className="text-2xl font-bold">{streak.maxDown} 天</div>
                {streak.maxDownStart && streak.maxDownEnd && (
                  <div className="text-xs text-muted-foreground mt-1">
                    {formatDate(streak.maxDownStart)} - {formatDate(streak.maxDownEnd)}
                  </div>
                )}
              </CardContent>
            </Card>
          </div>

          {isFetching && <div className="text-sm text-muted-foreground">更新中...</div>}
        </div>
      )}
    </div>
  )
}
