'use client'

import { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { stockDetailApi, stockSimilarityApi } from '@/services/api'
import type { StockSimilarityItem } from '@/types'
import { debounce, formatNumber } from '@/lib/utils'
import { Search } from 'lucide-react'

interface StockSearchResult {
  ts_code: string
  name: string
}

type SimilarityAlgoKey = 'zscore_cosine' | 'pearson' | 'best_lag_cosine'

export default function StockSimilarity() {
  const [selected, setSelected] = useState<StockSearchResult | null>(null)
  const [inputValue, setInputValue] = useState('')

  const [days, setDays] = useState(60)
  const [top, setTop] = useState(5)
  const [algo, setAlgo] = useState<SimilarityAlgoKey>('zscore_cosine')

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

  const debouncedSearch = useMemo(
    () =>
      debounce((keyword: string) => {
        setSearchKeyword(keyword)
      }, 300),
    [],
  )

  const safeDays = Math.max(5, Math.min(250, Number(days) || 60))
  const safeTop = Math.max(1, Math.min(200, Number(top) || 50))

  const { data: similarityList = [], isLoading, error } = useQuery<StockSimilarityItem[]>({
    queryKey: ['stock-similarity', selected?.ts_code || '', safeDays, safeTop, algo],
    queryFn: () =>
      stockSimilarityApi.getSimilarity({
        ts_code: selected?.ts_code || '',
        days: safeDays,
        top: safeTop,
        algo,
      }),
    enabled: !!selected?.ts_code,
    staleTime: 2 * 60 * 1000,
  })

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
                          if (selected) {
                            setSelected(null)
                          }
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
              <input
                type="number"
                value={days}
                min={5}
                max={250}
                onChange={(e) => setDays(Number(e.target.value))}
                className="w-full rounded-md border px-3 py-2 text-sm"
              />
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
            {selected?.ts_code ? `当前查询：${selected.ts_code}，近${safeDays}天，Top ${safeTop}` : '请选择股票'}
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
          {isLoading ? (
            <div className="py-8 text-center text-sm text-muted-foreground">加载中...</div>
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
                    <TableHead className="text-right">换手率</TableHead>
                    <TableHead className="text-right">涨跌幅</TableHead>
                    <TableHead className="text-right">5日涨跌幅</TableHead>
                    <TableHead className="text-right">10日涨跌幅</TableHead>
                    <TableHead className="text-right">20日涨跌幅</TableHead>
                    <TableHead className="text-right">60日涨跌幅</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {similarityList.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={11} className="text-center text-muted-foreground">
                        暂无数据
                      </TableCell>
                    </TableRow>
                  ) : (
                    similarityList.map((r, idx) => (
                      <TableRow key={r.ts_code}>
                        <TableCell>{idx + 1}</TableCell>
                        <TableCell>{r.ts_code}</TableCell>
                        <TableCell>{r.name || '-'}</TableCell>
                        <TableCell className="text-right">{formatNumber(r.similarity, 2)}</TableCell>
                        <TableCell className="text-right">{r.current_price == null ? '-' : formatNumber(r.current_price, 2)}</TableCell>
                        <TableCell className="text-right">{r.turnover_rate == null ? '-' : formatNumber(r.turnover_rate, 2)}</TableCell>
                        <TableCell className="text-right">{r.pct_chg == null ? '-' : formatNumber(r.pct_chg, 2)}</TableCell>
                        <TableCell className="text-right">{r.pct5 == null ? '-' : formatNumber(r.pct5, 2)}</TableCell>
                        <TableCell className="text-right">{r.pct10 == null ? '-' : formatNumber(r.pct10, 2)}</TableCell>
                        <TableCell className="text-right">{r.pct20 == null ? '-' : formatNumber(r.pct20, 2)}</TableCell>
                        <TableCell className="text-right">{r.pct60 == null ? '-' : formatNumber(r.pct60, 2)}</TableCell>
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
