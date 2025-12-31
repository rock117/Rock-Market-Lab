'use client'

import { useEffect, useMemo, useRef, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { stockDetailApi, stockSimilarityApi } from '@/services/api'
import type { StockSimilarityResponse } from '@/types'
import { debounce, formatNumber } from '@/lib/utils'
import { Check, ChevronDown, Search } from 'lucide-react'
import {
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts'

interface StockSearchResult {
  ts_code: string
  name: string
}

type SimilarityAlgoKey = 'zscore_cosine' | 'pearson' | 'best_lag_cosine'
type SimilarityFreqKey = 'day' | 'week' | 'month'

export default function StockSimilarity() {
  const [selected, setSelected] = useState<StockSearchResult | null>(null)
  const [inputValue, setInputValue] = useState('')

  const [visibleCodes, setVisibleCodes] = useState<string[] | null>(null)
  const [codesPickerOpen, setCodesPickerOpen] = useState(false)
  const codesPickerRef = useRef<HTMLDivElement | null>(null)

  const [days, setDays] = useState(60)
  const [top, setTop] = useState(20)
  const [algo, setAlgo] = useState<SimilarityAlgoKey>('zscore_cosine')
  const [freq, setFreq] = useState<SimilarityFreqKey>('day')

  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  const [viewMode, setViewMode] = useState<'table' | 'chart'>('table')

  const { data: searchData } = useQuery({
    queryKey: ['search-stocks', searchKeyword],
    queryFn: () => stockDetailApi.searchStocks(searchKeyword),
    enabled: searchKeyword.length >= 1,
    staleTime: 2 * 60 * 1000,
  })

  const renderPctCell = (value?: number | null) => {
    if (value == null || !Number.isFinite(value)) {
      return <span>-</span>
    }
    const cls = value > 0 ? 'text-red-600' : 'text-green-600'
    return <span className={cls}>{`${formatNumber(value, 2)}%`}</span>
  }

  const renderPctCellNoColor = (value?: number | null) => {
    if (value == null || !Number.isFinite(value)) {
      return <span>-</span>
    }
    return <span>{`${formatNumber(value, 2)}%`}</span>
  }

  useEffect(() => {
    if (searchData?.stocks) {
      setSearchResults(searchData.stocks)
      setShowSearchResults(true)
    }
  }, [searchData])

  const debouncedSearch = useMemo(
    () =>
      debounce((keyword: string) => {
        setSearchKeyword(keyword)
      }, 300),
    [],
  )

  const safeDays = Math.max(5, Math.min(250, Number(days) || 60))
  const safeTop = Math.max(1, Math.min(200, Number(top) || 50))

  const freqLabel = freq === 'week' ? '周' : freq === 'month' ? '月' : '天'

  const { data: similarityResp, isLoading, error } = useQuery<StockSimilarityResponse>({
    queryKey: ['stock-similarity', selected?.ts_code || '', safeDays, safeTop, algo, freq],
    queryFn: () =>
      stockSimilarityApi.getSimilarity({
        ts_code: selected?.ts_code || '',
        days: safeDays,
        top: safeTop,
        algo,
        freq,
      }),
    enabled: !!selected?.ts_code,
    staleTime: 2 * 60 * 1000,
  })

  const similarityList = similarityResp?.items ?? []
  const klineMap = similarityResp?.kline ?? {}

  useEffect(() => {
    if (!codesPickerOpen) return

    const onMouseDown = (e: MouseEvent) => {
      const el = codesPickerRef.current
      if (!el) return
      if (el.contains(e.target as Node)) return
      setCodesPickerOpen(false)
    }

    document.addEventListener('mousedown', onMouseDown)
    return () => document.removeEventListener('mousedown', onMouseDown)
  }, [codesPickerOpen])

  useEffect(() => {
    const codes = similarityList
      .slice()
      .sort((a, b) => (b.similarity ?? 0) - (a.similarity ?? 0))
      .map((x) => x.ts_code)
    setVisibleCodes((prev) => {
      if (codes.length === 0) return null
      if (prev == null) return null
      const prevSet = new Set(prev)
      const next = codes.filter((c) => prevSet.has(c))
      return next
    })
  }, [similarityList])

  const onSelect = (s: StockSearchResult) => {
    setSelected(s)
    setInputValue(s.name)
    setSearchKeyword('')
    setShowSearchResults(false)
  }

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>股票走势相似度</CardTitle>
          <CardDescription>选择一只股票，按近 N 天走势相似度返回相似股票列表</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
            <div className="md:col-span-1">
              <div className="flex items-end gap-3">
                <div className="flex-1">
                  <label className="mb-2 block text-sm font-medium">选择股票</label>
                  <div className="relative">
                    <div className="flex items-center gap-2 rounded-md border px-3 py-2">
                      <Search className="h-4 w-4 text-muted-foreground" />
                      <input
                        type="text"
                        value={inputValue}
                        placeholder={'搜索股票代码或名称...'}
                        className="w-full bg-transparent text-sm outline-none"
                        onChange={(e) => {
                          const v = e.target.value
                          setInputValue(v)
                          debouncedSearch(v)
                        }}
                        onFocus={() => searchResults.length > 0 && setShowSearchResults(true)}
                      />
                    </div>

                    {showSearchResults && searchResults.length > 0 && (
                      <div className="absolute top-full left-0 right-0 z-10 mt-1 max-h-60 overflow-y-auto rounded-md border bg-white shadow-lg">
                        {searchResults.map((r) => (
                          <div
                            key={r.ts_code}
                            className="cursor-pointer border-b p-3 hover:bg-muted last:border-b-0"
                            onClick={() => onSelect(r)}
                          >
                            <div className="flex items-center justify-between">
                              <div>
                                <span className="font-medium">{r.name}</span>
                                <span className="ml-2 text-sm text-muted-foreground">{r.ts_code}</span>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                </div>

                <div className="w-56">
                  <label className="mb-2 block text-sm font-medium">算法</label>
                  <select
                    value={algo}
                    onChange={(e) => setAlgo(e.target.value as SimilarityAlgoKey)}
                    className="w-full rounded-md border px-3 py-2 text-sm"
                  >
                    <option value={'zscore_cosine'}>z-score收益率 + 余弦</option>
                    <option value={'pearson'}>收益率 Pearson</option>
                    <option value={'best_lag_cosine'}>best-lag(±5) 余弦</option>
                  </select>
                </div>
              </div>
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium">过去 N 天</label>
              <div className="flex items-center gap-2">
                <input
                  type="number"
                  value={days}
                  min={5}
                  max={250}
                  onChange={(e) => setDays(Number(e.target.value))}
                  className="w-full rounded-md border px-3 py-2 text-sm"
                />
                <select
                  value={freq}
                  onChange={(e) => setFreq(e.target.value as SimilarityFreqKey)}
                  className="w-24 rounded-md border px-3 py-2 text-sm"
                >
                  <option value={'day'}>日</option>
                  <option value={'week'}>周</option>
                  <option value={'month'}>月</option>
                </select>
              </div>
            </div>

            <div>
              <label className="mb-2 block text-sm font-medium">返回条数 Top</label>
              <input
                type="number"
                value={top}
                min={1}
                max={200}
                onChange={(e) => setTop(Number(e.target.value))}
                className="w-full rounded-md border px-3 py-2 text-sm"
              />
            </div>

          </div>

          <div className="mt-4 text-sm text-muted-foreground">
            {selected?.ts_code ? `当前查询：${selected.ts_code}，近${safeDays}${freqLabel}，Top ${safeTop}` : '请选择股票'}
          </div>

          {error ? <div className="mt-4 text-sm text-destructive">{String((error as any)?.message || error)}</div> : null}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>相似股票列表</CardTitle>
          <CardDescription>相似度范围 [-1, 1]，越接近 1 越相似</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="mb-4 flex items-center gap-2">
            <button
              className={`rounded-md border px-3 py-2 text-sm ${viewMode === 'table' ? 'bg-muted font-medium' : ''}`}
              onClick={() => setViewMode('table')}
              type="button"
            >
              表格
            </button>
            <button
              className={`rounded-md border px-3 py-2 text-sm ${viewMode === 'chart' ? 'bg-muted font-medium' : ''}`}
              onClick={() => setViewMode('chart')}
              type="button"
            >
              图表(走势对比)
            </button>
          </div>

          {viewMode === 'chart' ? (
            <div className="space-y-6">
              {isLoading ? (
                <div className="text-center text-muted-foreground">加载中...</div>
              ) : !selected?.ts_code ? (
                <div className="text-center text-muted-foreground">请选择股票</div>
              ) : similarityList.length === 0 ? (
                <div className="text-center text-muted-foreground">暂无数据</div>
              ) : (
                (() => {
                  const codes = similarityList
                    .slice()
                    .sort((a, b) => (b.similarity ?? 0) - (a.similarity ?? 0))
                    .map((x) => x.ts_code)
                  const visibleSet = new Set(visibleCodes == null ? codes : visibleCodes)
                  const dateSet = new Set<string>()
                  for (const code of codes) {
                    const pts = klineMap[code] ?? []
                    for (const p of pts) {
                      if (p?.date) dateSet.add(p.date)
                    }
                  }
                  const dates = Array.from(dateSet).sort((a, b) => new Date(a).getTime() - new Date(b).getTime())

                  const closeByCode: Record<string, Record<string, number>> = {}
                  const pctByCode: Record<string, Record<string, number>> = {}
                  const baseByCode: Record<string, number> = {}
                  for (const code of codes) {
                    const pts = (klineMap[code] ?? []).slice().sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
                    if (pts.length > 0) {
                      const base = Number(pts[0]?.close)
                      baseByCode[code] = Number.isFinite(base) && base !== 0 ? base : 1
                    } else {
                      baseByCode[code] = 1
                    }

                    const map: Record<string, number> = {}
                    const pctMap: Record<string, number> = {}
                    for (const p of pts) {
                      const c = Number(p.close)
                      const pct = Number(p.pct_chg)
                      if (p?.date && Number.isFinite(c)) map[p.date] = c
                      if (p?.date && Number.isFinite(pct)) pctMap[p.date] = pct
                    }
                    closeByCode[code] = map
                    pctByCode[code] = pctMap
                  }

                  const chartData = dates.map(date => {
                    const row: any = { date }
                    for (const code of codes) {
                      const c = closeByCode[code]?.[date]
                      if (c == null) {
                        row[code] = null
                      } else {
                        row[code] = Number(((c / baseByCode[code]) * 100).toFixed(2))
                      }
                    }
                    return row
                  })

                  const nameByCode: Record<string, string> = {}
                  const similarityByCode: Record<string, number> = {}
                  for (const it of similarityList) {
                    nameByCode[it.ts_code] = it.name || it.ts_code
                    if (typeof it.similarity === 'number' && Number.isFinite(it.similarity)) {
                      similarityByCode[it.ts_code] = it.similarity
                    }
                  }

                  const colors = ['#2563eb', '#ef4444', '#22c55e', '#f59e0b', '#8b5cf6', '#14b8a6', '#ec4899', '#64748b']

                  const CustomTooltip = ({ active, label, payload }: any) => {
                    if (!active || !payload || payload.length === 0) return null
                    const date = label
                    return (
                      <div className="bg-white p-3 border rounded-lg shadow-lg">
                        <div className="font-medium mb-2">{date}</div>
                        <div className="space-y-1 text-sm">
                          {payload
                            .filter((p: any) => p && p.dataKey)
                            .map((p: any) => {
                              const code = String(p.dataKey)
                              const price = closeByCode[code]?.[date]
                              const pct = pctByCode[code]?.[date]
                              const pctCls = pct == null ? '' : pct > 0 ? 'text-red-600' : pct < 0 ? 'text-green-600' : ''
                              return (
                                <div key={code} className="flex items-center justify-between gap-6">
                                  <div className="flex items-center gap-2">
                                    <span className="inline-block h-2 w-2 rounded-full" style={{ backgroundColor: p.color || '#999' }}></span>
                                    <span>{nameByCode[code] || code}</span>
                                  </div>
                                  <div className="flex items-center gap-3 whitespace-nowrap">
                                    <span>{price == null ? '-' : formatNumber(Number(price), 2)}</span>
                                    <span className={pctCls}>{pct == null ? '-' : `${formatNumber(pct, 2)}%`}</span>
                                  </div>
                                </div>
                              )
                            })}
                        </div>
                      </div>
                    )
                  }

                  return (
                    <div>
                      <div className="mb-3 text-sm text-muted-foreground">
                        归一化规则：各股票首日收盘价 = 100
                      </div>
                      <div className="mb-3 flex items-center justify-between gap-3">
                        <label className="text-sm font-medium">显示股票</label>
                        {(() => {
                          const currentVisible = visibleCodes == null ? codes : visibleCodes
                          const visibleSetLocal = new Set(currentVisible)
                          const total = codes.length
                          const selectedCount = codes.filter((c) => visibleSetLocal.has(c)).length

                          const setVisibleSafe = (nextCodes: string[]) => {
                            setVisibleCodes(nextCodes)
                          }

                          const onToggleCode = (code: string) => {
                            const next = new Set(currentVisible)
                            if (next.has(code)) {
                              next.delete(code)
                            } else {
                              next.add(code)
                            }
                            setVisibleSafe(Array.from(next))
                          }

                          const onToggleAll = () => {
                            setVisibleCodes((prev) => (prev == null ? [] : null))
                          }

                          return (
                            <div className="relative" ref={codesPickerRef}>
                              <button
                                type="button"
                                className="flex h-10 w-72 items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm"
                                onClick={() => setCodesPickerOpen((v) => !v)}
                              >
                                <span className="truncate">
                                  {visibleCodes == null
                                    ? `全部显示（${total}）`
                                    : selectedCount === 0
                                      ? '已隐藏全部'
                                      : `已选 ${selectedCount}/${total}`}
                                </span>
                                <ChevronDown className={`h-4 w-4 opacity-60 transition-transform ${codesPickerOpen ? 'rotate-180' : ''}`} />
                              </button>

                              {codesPickerOpen && (
                                <div className="absolute left-0 right-0 z-20 mt-1 rounded-md border bg-white shadow-lg">
                                  <div className="flex items-center justify-between gap-2 border-b p-2">
                                    <button
                                      type="button"
                                      className={`rounded-md border px-2 py-1 text-xs hover:bg-muted ${visibleCodes == null ? 'bg-muted font-medium' : ''}`}
                                      onClick={onToggleAll}
                                    >
                                      全选
                                    </button>
                                  </div>

                                  <div className="max-h-64 overflow-y-auto p-1">
                                    {codes.map((code, idx) => {
                                      const isChecked = visibleSetLocal.has(code)
                                      const isSel = code === selected?.ts_code
                                      const sim = similarityByCode[code]
                                      return (
                                        <button
                                          key={code}
                                          type="button"
                                          className={`flex w-full items-center gap-2 rounded-sm px-2 py-2 text-left text-sm hover:bg-muted ${isSel ? 'font-medium' : ''}`}
                                          onClick={() => onToggleCode(code)}
                                        >
                                          <span
                                            className={`flex h-4 w-4 items-center justify-center rounded border ${isChecked ? 'bg-primary text-primary-foreground border-primary' : 'bg-white'}`}
                                            style={{ borderColor: isChecked ? undefined : colors[idx % colors.length] }}
                                          >
                                            {isChecked ? <Check className="h-3 w-3" /> : null}
                                          </span>
                                          <span className="flex-1 truncate">{nameByCode[code] || code}</span>
                                          <span className="text-xs text-muted-foreground">{code}</span>
                                          <span className="text-xs tabular-nums text-muted-foreground">
                                            {sim == null ? '-' : formatNumber(sim, 2)}
                                          </span>
                                        </button>
                                      )
                                    })}
                                  </div>
                                </div>
                              )}
                            </div>
                          )
                        })()}
                      </div>
                      <div className="h-[520px] w-full">
                        <ResponsiveContainer width="100%" height="100%">
                          <LineChart data={chartData} margin={{ top: 20, right: 30, left: 10, bottom: 5 }}>
                            <CartesianGrid strokeDasharray="3 3" stroke="#e1e5e9" />
                            <XAxis
                              dataKey="date"
                              tick={{ fontSize: 12 }}
                              stroke="#666"
                              tickFormatter={(v) => String(v).slice(5)}
                            />
                            <YAxis tick={{ fontSize: 12 }} stroke="#666" />
                            <Tooltip content={<CustomTooltip />} />
                            <Legend />
                            {codes.map((code, idx) => (
                              !visibleSet.has(code) ? null :
                              <Line
                                key={code}
                                type="monotone"
                                dataKey={code}
                                name={nameByCode[code]}
                                stroke={colors[idx % colors.length]}
                                strokeWidth={code === selected.ts_code ? 3 : 2}
                                strokeOpacity={code === selected.ts_code ? 1 : 0.55}
                                dot={false}
                                connectNulls={false}
                                isAnimationActive={false}
                              />
                            ))}
                          </LineChart>
                        </ResponsiveContainer>
                      </div>
                    </div>
                  )
                })()
              )}
            </div>
          ) : (
          <div className="overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>排名</TableHead>
                  <TableHead>股票代码</TableHead>
                  <TableHead>名称</TableHead>
                  <TableHead className="text-right">相似度</TableHead>
                  <TableHead className="text-right">当前价</TableHead>
                  <TableHead className="text-right">涨跌幅</TableHead>
                  <TableHead className="text-right">5日涨跌幅</TableHead>
                  <TableHead className="text-right">10日涨跌幅</TableHead>
                  <TableHead className="text-right">20日涨跌幅</TableHead>
                  <TableHead className="text-right">60日涨跌幅</TableHead>
                  <TableHead className="text-right">换手率</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {isLoading ? (
                  <TableRow>
                    <TableCell colSpan={11} className="text-center text-muted-foreground">
                      加载中...
                    </TableCell>
                  </TableRow>
                ) : similarityList.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={11} className="text-center text-muted-foreground">
                      暂无数据
                    </TableCell>
                  </TableRow>
                ) : (
                  similarityList.map((r, idx) => (
                    <TableRow
                      key={r.ts_code}
                      className={
                        r.ts_code === selected?.ts_code
                          ? 'border-l-4 border-l-primary bg-muted/50 font-medium'
                          : undefined
                      }
                    >
                      <TableCell>{idx + 1}</TableCell>
                      <TableCell>{r.ts_code}</TableCell>
                      <TableCell>{r.name || '-'}</TableCell>
                      <TableCell className="text-right">{formatNumber(r.similarity, 2)}</TableCell>
                      <TableCell className="text-right">{r.current_price == null ? '-' : formatNumber(r.current_price, 2)}</TableCell>
                      <TableCell className="text-right">{renderPctCell(r.pct_chg)}</TableCell>
                      <TableCell className="text-right">{renderPctCell(r.pct5)}</TableCell>
                      <TableCell className="text-right">{renderPctCell(r.pct10)}</TableCell>
                      <TableCell className="text-right">{renderPctCell(r.pct20)}</TableCell>
                      <TableCell className="text-right">{renderPctCell(r.pct60)}</TableCell>
                      <TableCell className="text-right">{renderPctCellNoColor(r.turnover_rate)}</TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
