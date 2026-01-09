'use client'

import React, { useCallback, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Tooltip } from '@/components/ui/tooltip'
import { stockApi } from '@/services/api'
import type { AStockOverview } from '@/types'
import { formatNumber, formatPercent, formatMarketCap, formatYmd } from '@/lib/utils'
import { TrendingUp } from 'lucide-react'

interface AStockListProps {
  className?: string
}

export default function AStockList({ className }: AStockListProps) {
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)

  const [orderBy, setOrderBy] = useState<string>('pct_chg')
  const [order, setOrder] = useState<'ascending' | 'descending'>('descending')

  const [keyword, setKeyword] = useState('')

  const getUpDownClass = useCallback((value: number | string | null | undefined) => {
    const n = value == null ? NaN : Number(value)
    if (!Number.isFinite(n) || n === 0) return 'text-neutral'
    return n > 0 ? 'text-red-500' : 'text-green-500'
  }, [])

  const handlePageChange = useCallback((newPage: number) => {
    setPage(newPage)
  }, [])

  const handlePageSizeChange = useCallback((newSize: number) => {
    setPageSize(newSize)
  }, [])

  const toggleSort = useCallback(
    (key: string) => {
      setPage(1)
      if (orderBy !== key) {
        setOrderBy(key)
        setOrder('descending')
        return
      }
      setOrder(prev => (prev === 'descending' ? 'ascending' : 'descending'))
    },
    [orderBy]
  )

  const getSortLabel = useCallback(
    (key: string, label: string) => {
      if (orderBy !== key) return label
      return order === 'descending' ? `${label} ↓` : `${label} ↑`
    },
    [orderBy, order]
  )

  // 只请求一次全量数据，排序/分页均在前端完成，不再触发网络请求
  const { data: allItems, isLoading, error, refetch } = useQuery<AStockOverview[]>({
    queryKey: ['a-stocks-all'],
    queryFn: () => stockApi.getAllAStocks(),
    staleTime: 2 * 60 * 1000,
  })

  const data = useMemo(() => {
    const all = allItems || []
    const kw = keyword.trim().toLowerCase()
    const filtered = kw
      ? all.filter(s => {
          const ts = (s.ts_code || '').toLowerCase()
          const name = (s.name || '').toLowerCase()
          const py = (s.name_py || '').toLowerCase()
          return ts.includes(kw) || name.includes(kw) || py.includes(kw)
        })
      : all

    const toNumber = (v: any) => {
      if (v === null || v === undefined) return NaN
      const n = typeof v === 'number' ? v : Number(v)
      return Number.isFinite(n) ? n : NaN
    }

    const toDateKey = (v: any) => {
      if (v === null || v === undefined) return NaN
      const s = String(v).trim()
      if (/^\d{8}$/.test(s)) return Number(s)
      if (/^\d{4}-\d{2}-\d{2}$/.test(s)) return Number(s.replace(/-/g, ''))
      return NaN
    }

    const sortedAll = [...filtered]
    if (orderBy && orderBy !== 'none') {
      const desc = order === 'descending'
      sortedAll.sort((a: any, b: any) => {
        const av = orderBy === 'list_date' ? toDateKey(a?.[orderBy]) : toNumber(a?.[orderBy])
        const bv = orderBy === 'list_date' ? toDateKey(b?.[orderBy]) : toNumber(b?.[orderBy])
        const aValid = Number.isFinite(av)
        const bValid = Number.isFinite(bv)
        if (!aValid && !bValid) return 0
        if (!aValid) return 1
        if (!bValid) return -1
        return desc ? bv - av : av - bv
      })
    }

    const total = sortedAll.length
    const pageSizeSafe = Math.max(1, pageSize)
    const totalPages = Math.max(1, Math.ceil(total / pageSizeSafe))
    const pageSafe = Math.min(Math.max(1, page), totalPages)
    const start = (pageSafe - 1) * pageSizeSafe
    const end = start + pageSizeSafe
    const items = sortedAll.slice(start, end)

    return {
      items,
      total,
      page: pageSafe,
      page_size: pageSizeSafe,
      total_pages: totalPages,
    }
  }, [allItems, keyword, orderBy, order, page, pageSize])

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
        <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 mb-4">
          <div className="w-full sm:max-w-sm">
            <input
              value={keyword}
              onChange={e => {
                setKeyword(e.target.value)
                setPage(1)
              }}
              placeholder="搜索：代码 / 名称 / 首字母"
              className="w-full px-3 py-2 border rounded-md text-sm bg-background"
            />
          </div>
        </div>

        <div className="rounded-md border overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[120px]">股票代码</TableHead>
                <TableHead className="w-[160px]">股票名称</TableHead>
                <TableHead className="w-[100px] text-right cursor-pointer select-none" onClick={() => toggleSort('close')}>
                  {getSortLabel('close', '当前价')}
                </TableHead>
                <TableHead className="w-[100px] text-right cursor-pointer select-none" onClick={() => toggleSort('pct_chg')}>
                  {getSortLabel('pct_chg', '涨跌幅')}
                </TableHead>
                <TableHead className="w-[110px] text-right cursor-pointer select-none" onClick={() => toggleSort('pct5')}>
                  {getSortLabel('pct5', '5日涨跌幅')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('pct10')}>
                  {getSortLabel('pct10', '10日涨跌幅')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('pct20')}>
                  {getSortLabel('pct20', '20日涨跌幅')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('pct60')}>
                  {getSortLabel('pct60', '60日涨跌幅')}
                </TableHead>
                <TableHead className="w-[240px]">概念</TableHead>
                <TableHead className="w-[90px] text-right cursor-pointer select-none" onClick={() => toggleSort('pe')}>
                  {getSortLabel('pe', 'PE')}
                </TableHead>
                <TableHead className="w-[90px] text-right cursor-pointer select-none" onClick={() => toggleSort('dv_ratio')}>
                  {getSortLabel('dv_ratio', '股息率')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('total_mv')}>
                  {getSortLabel('total_mv', '市值')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('list_date')}>
                  {getSortLabel('list_date', '上市日期')}
                </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.items.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={13} className="text-center text-muted-foreground py-10">
                    {keyword.trim() ? '没有匹配结果' : '暂无数据'}
                  </TableCell>
                </TableRow>
              ) : (
                data.items.map(stock => {
                  return (
                    <TableRow key={stock.ts_code} className="hover:bg-muted/50">
                      <TableCell className="font-mono font-medium">{stock.ts_code}</TableCell>
                      <TableCell className="font-medium">{stock.name || '-'}</TableCell>
                      <TableCell className="text-right">{stock.close == null ? '-' : formatNumber(stock.close, 2)}</TableCell>
                      <TableCell className={`text-right ${getUpDownClass(stock.pct_chg)}`}>
                        {stock.pct_chg == null ? '-' : formatPercent(stock.pct_chg, 2)}
                      </TableCell>
                      <TableCell className={`text-right ${getUpDownClass(stock.pct5)}`}>
                        {stock.pct5 == null ? '-' : formatPercent(stock.pct5, 2)}
                      </TableCell>
                      <TableCell className={`text-right ${getUpDownClass(stock.pct10)}`}>
                        {stock.pct10 == null ? '-' : formatPercent(stock.pct10, 2)}
                      </TableCell>
                      <TableCell className={`text-right ${getUpDownClass(stock.pct20)}`}>
                        {stock.pct20 == null ? '-' : formatPercent(stock.pct20, 2)}
                      </TableCell>
                      <TableCell className={`text-right ${getUpDownClass(stock.pct60)}`}>
                        {stock.pct60 == null ? '-' : formatPercent(stock.pct60, 2)}
                      </TableCell>
                      <TableCell className="max-w-[240px]">
                        {stock.concepts ? (
                          <Tooltip content={stock.concepts}>
                            <div className="truncate cursor-help">{stock.concepts}</div>
                          </Tooltip>
                        ) : (
                          '-'
                        )}
                      </TableCell>
                      <TableCell className="text-right">{stock.pe == null ? '-' : formatNumber(stock.pe, 2)}</TableCell>
                      <TableCell className="text-right">{stock.dv_ratio == null ? '-' : formatPercent(stock.dv_ratio, 2)}
                      </TableCell>
                      <TableCell className="text-right font-medium">
                        {stock.total_mv == null ? '-' : formatMarketCap(stock.total_mv)}
                      </TableCell>
                      <TableCell className="text-right">{formatYmd(stock.list_date ?? null)}</TableCell>
                    </TableRow>
                  )
                })
              )}
            </TableBody>
          </Table>
        </div>

        {data.total > 0 && (
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
        )}
      </CardContent>
    </Card>
  )
}
