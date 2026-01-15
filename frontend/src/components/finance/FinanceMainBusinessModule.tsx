'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { financeApi, FinanceMainBusinessType } from '@/services/api/finance'
import { normalizeDate } from '@/services/api'
import { ArrowDown, ArrowUp, ArrowUpDown } from 'lucide-react'

type SortKey = 'end_date' | 'ts_code' | 'bz_item' | 'bz_sales' | 'bz_profit' | 'bz_cost'

const SORT_LABELS: Record<SortKey, string> = {
  ts_code: '股票代码',
  end_date: '日期',
  bz_item: '主营业务来源',
  bz_sales: '主营业务收入(亿)',
  bz_profit: '主营业务利润(亿)',
  bz_cost: '主营业务成本(亿)'
}

function toYi(value?: string | null): string {
  if (value === null || value === undefined) return '-'
  const n = Number(value)
  if (!Number.isFinite(n)) return '-'
  return (n / 1e8).toFixed(4)
}

export default function FinanceMainBusinessModule() {
  const [type, setType] = useState<FinanceMainBusinessType>('P')
  const [keyword, setKeyword] = useState('')

  const [selectedEndDates, setSelectedEndDates] = useState<string[]>([])
  const [selectedBzItems, setSelectedBzItems] = useState<string[]>([])

  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)

  const [sortKey, setSortKey] = useState<SortKey>('end_date')
  const [sortDir, setSortDir] = useState<'asc' | 'desc'>('desc')

  useEffect(() => {
    setPage(1)
    setSelectedEndDates([])
    setSelectedBzItems([])
  }, [type, pageSize, sortKey, sortDir])

  const endDatesQuery = useQuery({
    queryKey: ['finance-main-business-end-dates'],
    queryFn: () => financeApi.getMainBusinessEndDates(),
    staleTime: 5 * 60 * 1000,
  })

  const bzItemsQuery = useQuery({
    queryKey: ['finance-main-business-bz-items', type],
    queryFn: () => financeApi.getMainBusinessBzItems(type),
    staleTime: 5 * 60 * 1000,
  })

  const hasLoadedEndDates = endDatesQuery.isSuccess || endDatesQuery.isError

  const { data, isLoading, error } = useQuery({
    queryKey: ['finance-main-business', type, page, pageSize, sortKey, sortDir, selectedEndDates],
    queryFn: () => financeApi.getMainBusinessList({
      type,
      page,
      pageSize,
      sortBy: sortKey,
      sortDir,
      endDates: selectedEndDates.length > 0 ? selectedEndDates : undefined,
    }),
    enabled: hasLoadedEndDates,
    staleTime: 60 * 1000,
  })

  const filtered = useMemo(() => {
    const list = data?.data || []
    const kw = keyword.trim().toLowerCase()
    const dateSet = new Set(selectedEndDates)
    const bzSet = new Set(selectedBzItems)
    return list.filter((r) => {
      const endDate = normalizeDate(r.endDate)
      if (selectedEndDates.length > 0 && !dateSet.has(endDate)) return false
      if (selectedBzItems.length > 0 && !bzSet.has(r.bzItem)) return false
      if (!kw) return true
      const code = (r.tsCode || '').toLowerCase()
      const name = String(r.stockName || '').toLowerCase()
      return code.includes(kw) || name.includes(kw)
    })
  }, [data?.data, keyword, selectedEndDates, selectedBzItems])

  const endDateOptions = useMemo(() => {
    const list = endDatesQuery.data || []
    const uniq = Array.from(new Set(list.map((x) => normalizeDate(x))))
    uniq.sort((a, b) => b.localeCompare(a))
    return uniq
  }, [endDatesQuery.data])

  useEffect(() => {
    if (endDateOptions.length === 0) {
      setSelectedEndDates([])
      return
    }

    setSelectedEndDates((prev) => {
      if (prev.length === 0) {
        return [endDateOptions[0]]
      }

      const prevSet = new Set(prev)
      const next = endDateOptions.filter((d) => prevSet.has(d))
      if (next.length > 0) return next
      return [endDateOptions[0]]
    })
  }, [endDateOptions])

  const bzItemOptions = useMemo(() => {
    const list = bzItemsQuery.data || []
    const uniq = Array.from(new Set(list.map((x) => String(x || '').trim()).filter(Boolean)))
    uniq.sort((a, b) => a.localeCompare(b))
    return uniq
  }, [bzItemsQuery.data])

  useEffect(() => {
    if (bzItemOptions.length === 0) {
      setSelectedBzItems([])
      return
    }

    setSelectedBzItems((prev) => {
      if (prev.length === 0) return bzItemOptions
      const prevSet = new Set(prev)
      const next = bzItemOptions.filter((x) => prevSet.has(x))
      return next.length > 0 ? next : bzItemOptions
    })
  }, [bzItemOptions])

  const totalPages = data?.totalPages || 1

  const toggleSort = (k: SortKey) => {
    if (sortKey !== k) {
      setSortKey(k)
      setSortDir('asc')
      return
    }
    setSortDir(prev => (prev === 'asc' ? 'desc' : 'asc'))
  }

  const sortIcon = (k: SortKey) => {
    if (sortKey !== k) return <ArrowUpDown className="h-4 w-4" />
    return sortDir === 'asc' ? <ArrowUp className="h-4 w-4" /> : <ArrowDown className="h-4 w-4" />
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>主营业务</CardTitle>
        <CardDescription>数据来源：finance_main_business（按类型查询，前端本地搜索）</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex flex-col md:flex-row gap-3 md:items-center md:justify-between">
          <div className="flex flex-col md:flex-row gap-2 md:items-center">
            <div className="flex gap-2 items-center">
              <label className="text-sm text-muted-foreground whitespace-nowrap min-w-[3rem]">类型</label>
              <select
                className="border rounded-md px-2 py-2 text-sm bg-background"
                value={type}
                onChange={(e) => setType(e.target.value as FinanceMainBusinessType)}
              >
                <option value="P">P - 按产品</option>
                <option value="D">D - 按地区</option>
                <option value="I">I - 按行业</option>
              </select>
            </div>

            <div className="flex gap-2 items-center">
              <label className="text-sm text-muted-foreground whitespace-nowrap">日期</label>
              <details className="relative">
                <summary className="list-none border rounded-md px-3 py-2 text-sm cursor-pointer bg-background select-none min-w-[14rem]">
                  {selectedEndDates.length === 0
                    ? '全部'
                    : selectedEndDates.length === endDateOptions.length
                      ? `全部（${endDateOptions.length}）`
                      : `已选 ${selectedEndDates.length}/${endDateOptions.length}`}
                </summary>
                <div className="absolute right-0 mt-2 w-96 max-h-80 overflow-auto border rounded-md bg-background shadow-lg p-3 z-20">
                  <div className="flex items-center justify-between gap-2 mb-2">
                    <button
                      type="button"
                      className="text-sm border rounded-md px-2 py-1"
                      onClick={(e) => {
                        e.preventDefault()
                        setSelectedEndDates(endDateOptions)
                      }}
                      disabled={endDateOptions.length === 0}
                    >
                      全选
                    </button>
                    <button
                      type="button"
                      className="text-sm border rounded-md px-2 py-1"
                      onClick={(e) => {
                        e.preventDefault()
                        const set = new Set(selectedEndDates)
                        setSelectedEndDates(endDateOptions.filter((d) => !set.has(d)))
                      }}
                      disabled={endDateOptions.length === 0}
                    >
                      反选
                    </button>
                  </div>

                  <div className="flex flex-col gap-2">
                    {endDateOptions.map((d) => {
                      const checked = selectedEndDates.includes(d)
                      return (
                        <label key={d} className="flex items-center gap-2 text-sm cursor-pointer">
                          <input
                            type="checkbox"
                            checked={checked}
                            onChange={() => {
                              setSelectedEndDates((prev) => {
                                if (prev.includes(d)) return prev.filter((x) => x !== d)
                                return [...prev, d]
                              })
                            }}
                          />
                          <span className="font-mono">{d}</span>
                        </label>
                      )
                    })}
                    {endDateOptions.length === 0 && (
                      <div className="text-sm text-muted-foreground">暂无日期</div>
                    )}
                  </div>
                </div>
              </details>
            </div>

            <div className="flex gap-2 items-center">
              <label className="text-sm text-muted-foreground whitespace-nowrap">来源</label>
              <details className="relative">
                <summary className="list-none border rounded-md px-3 py-2 text-sm cursor-pointer bg-background select-none min-w-[14rem]">
                  {selectedBzItems.length === 0
                    ? '全部'
                    : selectedBzItems.length === bzItemOptions.length
                      ? `全部（${bzItemOptions.length}）`
                      : `已选 ${selectedBzItems.length}/${bzItemOptions.length}`}
                </summary>
                <div className="absolute right-0 mt-2 w-96 max-h-80 overflow-auto border rounded-md bg-background shadow-lg p-3 z-20">
                  <div className="flex items-center justify-between gap-2 mb-2">
                    <button
                      type="button"
                      className="text-sm border rounded-md px-2 py-1"
                      onClick={(e) => {
                        e.preventDefault()
                        setSelectedBzItems(bzItemOptions)
                      }}
                      disabled={bzItemOptions.length === 0}
                    >
                      全选
                    </button>
                    <button
                      type="button"
                      className="text-sm border rounded-md px-2 py-1"
                      onClick={(e) => {
                        e.preventDefault()
                        const set = new Set(selectedBzItems)
                        setSelectedBzItems(bzItemOptions.filter((x) => !set.has(x)))
                      }}
                      disabled={bzItemOptions.length === 0}
                    >
                      反选
                    </button>
                  </div>

                  <div className="flex flex-col gap-2">
                    {bzItemOptions.map((x) => {
                      const checked = selectedBzItems.includes(x)
                      return (
                        <label key={x} className="flex items-center gap-2 text-sm cursor-pointer">
                          <input
                            type="checkbox"
                            checked={checked}
                            onChange={() => {
                              setSelectedBzItems((prev) => {
                                if (prev.includes(x)) return prev.filter((v) => v !== x)
                                return [...prev, x]
                              })
                            }}
                          />
                          <span className="truncate" title={x}>{x}</span>
                        </label>
                      )
                    })}
                    {bzItemOptions.length === 0 && (
                      <div className="text-sm text-muted-foreground">暂无来源</div>
                    )}
                  </div>
                </div>
              </details>
            </div>

            <Input
              placeholder="搜索股票代码/名称（仅前端过滤）"
              value={keyword}
              onChange={(e) => setKeyword(e.target.value)}
              className="w-72"
            />
          </div>
        </div>

        {error && (
          <div className="text-sm text-red-500">加载失败：{String((error as any)?.message || error)}</div>
        )}

        <div className="border rounded-md overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('ts_code')} title="点击排序">
                    {SORT_LABELS.ts_code}
                    {sortIcon('ts_code')}
                  </button>
                </TableHead>
                <TableHead>股票名称</TableHead>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('end_date')} title="点击排序">
                    {SORT_LABELS.end_date}
                    {sortIcon('end_date')}
                  </button>
                </TableHead>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('bz_item')} title="点击排序">
                    {SORT_LABELS.bz_item}
                    {sortIcon('bz_item')}
                  </button>
                </TableHead>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('bz_sales')} title="点击排序">
                    {SORT_LABELS.bz_sales}
                    {sortIcon('bz_sales')}
                  </button>
                </TableHead>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('bz_profit')} title="点击排序">
                    {SORT_LABELS.bz_profit}
                    {sortIcon('bz_profit')}
                  </button>
                </TableHead>
                <TableHead>
                  <button className="flex items-center gap-1" onClick={() => toggleSort('bz_cost')} title="点击排序">
                    {SORT_LABELS.bz_cost}
                    {sortIcon('bz_cost')}
                  </button>
                </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading && (
                <TableRow>
                  <TableCell colSpan={7} className="text-center text-muted-foreground">加载中...</TableCell>
                </TableRow>
              )}
              {!isLoading && filtered.length === 0 && (
                <TableRow>
                  <TableCell colSpan={7} className="text-center text-muted-foreground">暂无数据</TableCell>
                </TableRow>
              )}
              {filtered.map((r, idx) => (
                <TableRow key={`${r.tsCode}-${r.endDate}-${idx}`}>
                  <TableCell>{r.tsCode}</TableCell>
                  <TableCell>{r.stockName || '-'}</TableCell>
                  <TableCell>{normalizeDate(r.endDate)}</TableCell>
                  <TableCell className="max-w-[240px] truncate" title={r.bzItem}>{r.bzItem}</TableCell>
                  <TableCell>{toYi(r.bzSales)}</TableCell>
                  <TableCell>{toYi(r.bzProfit)}</TableCell>
                  <TableCell>{toYi(r.bzCost)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        <div className="flex flex-col md:flex-row gap-3 md:items-center md:justify-between">
          <div className="flex flex-wrap gap-3 items-center">
            <div className="text-sm text-muted-foreground">
              共 {data?.total ?? 0} 条，当前页 {page}/{totalPages}
            </div>

            <div className="flex flex-wrap gap-2 items-center">
              <label className="text-sm text-muted-foreground">每页</label>
              <select
                className="border rounded-md px-2 py-2 text-sm bg-background"
                value={pageSize}
                onChange={(e) => setPageSize(Number(e.target.value))}
              >
                {[10, 20, 50, 100].map((n) => (
                  <option key={n} value={n}>{n}</option>
                ))}
              </select>
            </div>
          </div>

          <div className="flex flex-wrap gap-2 items-center">
            <button
              className="border rounded-md px-3 py-2 text-sm disabled:opacity-50"
              disabled={page <= 1}
              onClick={() => setPage(1)}
            >
              首页
            </button>
            <button
              className="border rounded-md px-3 py-2 text-sm disabled:opacity-50"
              disabled={page <= 1}
              onClick={() => setPage(p => Math.max(1, p - 1))}
            >
              上一页
            </button>
            <button
              className="border rounded-md px-3 py-2 text-sm disabled:opacity-50"
              disabled={page >= totalPages}
              onClick={() => setPage(p => Math.min(totalPages, p + 1))}
            >
              下一页
            </button>
            <button
              className="border rounded-md px-3 py-2 text-sm disabled:opacity-50"
              disabled={page >= totalPages}
              onClick={() => setPage(totalPages)}
            >
              末页
            </button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
