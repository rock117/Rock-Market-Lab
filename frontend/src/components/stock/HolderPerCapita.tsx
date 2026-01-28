'use client'

import React, { useCallback, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { holderPerCapitaApi, type HolderPerCapitaItem } from '@/services/api/holder-per-capita'
import { formatNumber, formatMarketCap } from '@/lib/utils'

function formatEndDate(dateStr: string): string {
  if (dateStr.length === 8) {
    return `${dateStr.slice(0, 4)}-${dateStr.slice(4, 6)}-${dateStr.slice(6, 8)}`
  }
  return dateStr
}
import { Users } from 'lucide-react'

interface HolderPerCapitaProps {
  className?: string
}

export default function HolderPerCapita({ className }: HolderPerCapitaProps) {
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)

  const [orderBy, setOrderBy] = useState<string>('per_capita_mv')
  const [order, setOrder] = useState<'ascending' | 'descending'>('descending')

  const [keyword, setKeyword] = useState('')

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

  const { data: allItems, isLoading, error, refetch } = useQuery<HolderPerCapitaItem[]>({
    queryKey: ['holder-per-capita-all'],
    queryFn: () => holderPerCapitaApi.getAll(),
    staleTime: 2 * 60 * 1000,
  })

  const data = useMemo(() => {
    const all = allItems || []
    const kw = keyword.trim().toLowerCase()
    const filtered = kw
      ? all.filter(s => {
          const ts = (s.ts_code || '').toLowerCase()
          const name = (s.name || '').toLowerCase()
          return ts.includes(kw) || name.includes(kw)
        })
      : all

    const toNumber = (v: any) => {
      if (v === null || v === undefined) return NaN
      const n = typeof v === 'number' ? v : Number(v)
      return Number.isFinite(n) ? n : NaN
    }

    const sortedAll = [...filtered]
    if (orderBy && orderBy !== 'none') {
      const desc = order === 'descending'
      sortedAll.sort((a: any, b: any) => {
        const av = toNumber(a?.[orderBy])
        const bv = toNumber(b?.[orderBy])
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
            <Users className="h-5 w-5 text-blue-500" />
            人均持股
          </CardTitle>
          <CardDescription>展示A股市场股票人均持股数据</CardDescription>
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
            <Users className="h-5 w-5" />
            人均持股 - 加载失败
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
          <Users className="h-5 w-5 text-blue-500" />
          人均持股
        </CardTitle>
        <CardDescription>展示A股市场股票人均持股数据（共 {data.total} 条）</CardDescription>
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
              placeholder="搜索：代码 / 名称"
              className="w-full px-3 py-2 border rounded-md text-sm bg-background"
            />
          </div>
        </div>

        <div className="rounded-md border overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[120px]">股票代码</TableHead>
                <TableHead className="w-[140px]">股票名称</TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('holder_num')}>
                  {getSortLabel('holder_num', '股东数')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('total_mv')}>
                  {getSortLabel('total_mv', '总市值')}
                </TableHead>
                <TableHead className="w-[120px] text-right cursor-pointer select-none" onClick={() => toggleSort('circ_mv')}>
                  {getSortLabel('circ_mv', '流通市值')}
                </TableHead>
                <TableHead className="w-[140px] text-right cursor-pointer select-none" onClick={() => toggleSort('per_capita_mv')}>
                  {getSortLabel('per_capita_mv', '人均持股市值')}
                </TableHead>
                <TableHead className="w-[140px] text-right cursor-pointer select-none" onClick={() => toggleSort('per_capita_share')}>
                  {getSortLabel('per_capita_share', '人均持股数')}
                </TableHead>
                <TableHead className="w-[100px] text-right cursor-pointer select-none" onClick={() => toggleSort('close')}>
                  {getSortLabel('close', '收盘价')}
                </TableHead>
                <TableHead className="w-[110px]">股东数日期</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {data.items.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={9} className="text-center text-muted-foreground py-10">
                    {keyword.trim() ? '没有匹配结果' : '暂无数据'}
                  </TableCell>
                </TableRow>
              ) : (
                data.items.map(item => {
                  return (
                    <TableRow key={item.ts_code} className="hover:bg-muted/50">
                      <TableCell className="font-mono font-medium">{item.ts_code}</TableCell>
                      <TableCell className="font-medium">{item.name || '-'}</TableCell>
                      <TableCell className="text-right">
                        {item.holder_num == null ? '-' : formatNumber(item.holder_num, 0)}
                      </TableCell>
                      <TableCell className="text-right font-medium">
                        {item.total_mv == null ? '-' : formatMarketCap(item.total_mv)}
                      </TableCell>
                      <TableCell className="text-right font-medium">
                        {item.circ_mv == null ? '-' : formatMarketCap(item.circ_mv)}
                      </TableCell>
                      <TableCell className="text-right font-medium text-blue-600">
                        {item.per_capita_mv == null ? '-' : `${formatNumber(item.per_capita_mv / 10000, 2)}万`}
                      </TableCell>
                      <TableCell className="text-right">
                        {item.per_capita_share == null ? '-' : formatNumber(item.per_capita_share, 0)}
                      </TableCell>
                      <TableCell className="text-right">
                        {item.close == null ? '-' : formatNumber(item.close, 2)}
                      </TableCell>
                      <TableCell className="text-right text-muted-foreground">
                        {item.end_date ? formatEndDate(item.end_date) : '-'}
                      </TableCell>
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
