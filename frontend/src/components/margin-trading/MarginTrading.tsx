'use client'

import React, { useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Select, SelectItem } from '@/components/ui/select'
import MarginBalanceKLineChart from '@/components/charts/MarginBalanceKLineChart'
import { marginTradingApi } from '@/services/api'
import { ExchangeCode } from '@/types'
import { formatDate, formatNumber } from '@/lib/utils'
import { Calendar, Filter, Scale } from 'lucide-react'

function toISODate(date: Date): string {
  return date.toISOString().split('T')[0]
}

export default function MarginTrading({ className }: { className?: string }) {
  const defaultEnd = useMemo(() => new Date(), [])
  const defaultStart = useMemo(() => {
    const d = new Date()
    d.setDate(d.getDate() - 10)
    return d
  }, [])

  const [startDate, setStartDate] = useState(() => toISODate(defaultStart))
  const [endDate, setEndDate] = useState(() => toISODate(defaultEnd))
  const [exchange, setExchange] = useState<ExchangeCode>('ALL')

  const { data, isLoading, error, refetch, isFetching } = useQuery({
    queryKey: ['margin-trading-kline', startDate, endDate, exchange],
    queryFn: () => marginTradingApi.getMarginTradingKLine({ startDate, endDate, exchange }),
    staleTime: 60 * 1000,
  })

  const title = useMemo(() => {
    const exchangeText: Record<ExchangeCode, string> = {
      ALL: '全部交易所',
      SSE: '上交所(SSE)',
      SZSE: '深交所(SZSE)',
      BSE: '北交所(BSE)',
    }

    return `融资融券 - ${exchangeText[exchange]}`
  }, [exchange])

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
            <Filter className="h-5 w-5" />
            筛选条件
          </CardTitle>
          <CardDescription>
            结束时间默认今天，开始时间默认10天前；支持交易所维度筛选
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

            <div className="flex flex-col gap-1 min-w-44">
              <span className="text-xs text-muted-foreground h-4 leading-4">交易所</span>
              <Select
                value={exchange}
                onValueChange={(value) => setExchange(value as ExchangeCode)}
                placeholder="选择交易所"
                className="w-full"
              >
                <SelectItem value="ALL">ALL 所有交易所</SelectItem>
                <SelectItem value="SSE">SSE 上交所</SelectItem>
                <SelectItem value="SZSE">SZSE 深交所</SelectItem>
                <SelectItem value="BSE">BSE 北交所</SelectItem>
              </Select>
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
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.maxBalance, 2)} 亿元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">最小融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.minBalance, 2)} 亿元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">平均融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.avgBalance, 2)} 亿元</div>
              </div>
              <div className="rounded-lg border p-3">
                <div className="text-xs text-muted-foreground">当前融资余额</div>
                <div className="mt-1 text-lg font-semibold">{formatNumber(summary.latestBalance, 2)} 亿元</div>
                <div className="mt-1 text-xs text-muted-foreground">{formatDate(summary.latestDate)}</div>
              </div>
            </div>
          )}
          <MarginBalanceKLineChart data={data?.data || []} title={title} />
        </CardContent>
      </Card>
    </div>
  )
}
