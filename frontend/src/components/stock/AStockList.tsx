'use client'

import React, { useCallback, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { stockApi } from '@/services/api'
import type { AStockOverview, PagedResponse } from '@/types'
import { formatNumber, formatPercent, getStockTrend, getTrendColorClass, formatMarketCap } from '@/lib/utils'
import { TrendingUp } from 'lucide-react'

interface AStockListProps {
  className?: string
}

export default function AStockList({ className }: AStockListProps) {
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)

  const handlePageChange = useCallback((newPage: number) => {
    setPage(newPage)
  }, [])

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPageSize(newSize)
  }, [])

  const { data, isLoading, error, refetch } = useQuery<PagedResponse<AStockOverview>>({
    queryKey: ['a-stocks', page, pageSize],
    queryFn: () =>
      stockApi.getAStockOverviews({
        page,
        page_size: pageSize,
        order_by: 'pct_chg',
        order: 'descending',
        market: 'All',
        area: 'All',
        industry: 'All',
      }),
    staleTime: 2 * 60 * 1000,
    placeholderData: prev => prev,
  })

  if (isLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5 text-bull" />
            A股列表
          </CardTitle>
          <CardDescription>展示A股市场股票列表及核心指标</CardDescription>
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
            <TrendingUp className="h-5 w-5" />
            A股列表 - 加载失败
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

  if (!data || data.items.length === 0) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5 text-bull" />
            A股列表
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
          <TrendingUp className="h-5 w-5 text-bull" />
          A股列表
        </CardTitle>
        <CardDescription>展示A股市场股票列表及核心指标（共 {data.total} 条）</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="rounded-md border overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[120px]">股票代码</TableHead>
                <TableHead className="w-[160px]">股票名称</TableHead>
                <TableHead className="w-[100px] text-right">当前价</TableHead>
                <TableHead className="w-[100px] text-right">涨跌幅</TableHead>
                <TableHead className="w-[90px] text-right">MA5</TableHead>
                <TableHead className="w-[90px] text-right">MA10</TableHead>
                <TableHead className="w-[90px] text-right">MA20</TableHead>
                <TableHead className="w-[90px] text-right">MA60</TableHead>
                <TableHead className="w-[90px] text-right">PE</TableHead>
                <TableHead className="w-[90px] text-right">股息率</TableHead>
                <TableHead className="w-[120px] text-right">市值</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.items.map(stock => {
                const trend = getStockTrend(stock.pct_chg ?? 0)
                return (
                  <TableRow key={stock.ts_code} className="hover:bg-muted/50">
                    <TableCell className="font-mono font-medium">{stock.ts_code}</TableCell>
                    <TableCell className="font-medium">{stock.name || '-'}</TableCell>
                    <TableCell className="text-right">{stock.close == null ? '-' : formatNumber(stock.close, 2)}</TableCell>
                    <TableCell className={`text-right ${getTrendColorClass(trend)}`}>
                      {stock.pct_chg == null ? '-' : formatPercent(stock.pct_chg, 2)}
                    </TableCell>
                    <TableCell className="text-right">{stock.ma5 == null ? '-' : formatNumber(stock.ma5, 2)}</TableCell>
                    <TableCell className="text-right">{stock.ma10 == null ? '-' : formatNumber(stock.ma10, 2)}</TableCell>
                    <TableCell className="text-right">{stock.ma20 == null ? '-' : formatNumber(stock.ma20, 2)}</TableCell>
                    <TableCell className="text-right">{stock.ma60 == null ? '-' : formatNumber(stock.ma60, 2)}</TableCell>
                    <TableCell className="text-right">{stock.pe == null ? '-' : formatNumber(stock.pe, 2)}</TableCell>
                    <TableCell className="text-right">{stock.dv_ratio == null ? '-' : formatPercent(stock.dv_ratio, 2)}</TableCell>
                    <TableCell className="text-right font-medium">
                      {stock.total_mv == null ? '-' : formatMarketCap(stock.total_mv)}
                    </TableCell>
                  </TableRow>
                )
              })}
            </TableBody>
          </Table>
        </div>

        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mt-6 pt-4 border-t">
          <div className="flex items-center gap-4">
            <div className="text-sm text-muted-foreground">
              显示 {((page - 1) * pageSize) + 1} - {Math.min(page * pageSize, data.total)} 条，共 {data.total} 条
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm text-muted-foreground">每页</span>
              <select
                value={pageSize}
                onChange={e => {
                  handlePageSizeChange(Number(e.target.value))
                  handlePageChange(1)
                }}
                className="px-2 py-1 border rounded text-sm"
              >
                <option value={10}>10</option>
                <option value={20}>20</option>
                <option value={50}>50</option>
                <option value={100}>100</option>
              </select>
            </div>
          </div>

          {data.total_pages > 1 && (
            <div className="flex items-center gap-2">
              <button
                onClick={() => handlePageChange(1)}
                disabled={page === 1}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                首页
              </button>
              <button
                onClick={() => handlePageChange(page - 1)}
                disabled={page === 1}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                上一页
              </button>
              <span className="text-sm text-muted-foreground">
                第 {page} / {data.total_pages} 页
              </span>
              <button
                onClick={() => handlePageChange(page + 1)}
                disabled={page === data.total_pages}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                下一页
              </button>
              <button
                onClick={() => handlePageChange(data.total_pages)}
                disabled={page === data.total_pages}
                className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
              >
                末页
              </button>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
