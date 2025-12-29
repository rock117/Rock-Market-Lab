'use client'

import { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { stockDetailApi, stockSimilarityApi } from '@/services/api'
import type { StockSimilarityItem } from '@/types'
import { debounce, formatNumber, formatPercent } from '@/lib/utils'
import { Search } from 'lucide-react'

interface StockSearchResult {
  ts_code: string
  name: string
}

export default function StockSimilarity() {
  const [selected, setSelected] = useState<StockSearchResult | null>(null)

  const [days, setDays] = useState(60)
  const [top, setTop] = useState(50)

  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  const [submittedTsCode, setSubmittedTsCode] = useState<string>('')
  const [submittedDays, setSubmittedDays] = useState<number>(60)
  const [submittedTop, setSubmittedTop] = useState<number>(50)

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

  const { data: similarityList = [], isLoading, error } = useQuery<StockSimilarityItem[]>({
    queryKey: ['stock-similarity', submittedTsCode, submittedDays, submittedTop],
    queryFn: () =>
      stockSimilarityApi.getSimilarity({
        ts_code: submittedTsCode,
        days: submittedDays,
        top: submittedTop,
      }),
    enabled: !!submittedTsCode,
    staleTime: 2 * 60 * 1000,
  })

  const onSelect = (s: StockSearchResult) => {
    setSelected(s)
    setSearchKeyword('')
    setShowSearchResults(false)
  }

  const onSubmit = () => {
    if (!selected?.ts_code) return
    const safeDays = Math.max(5, Math.min(250, Number(days) || 60))
    const safeTop = Math.max(1, Math.min(200, Number(top) || 50))

    setDays(safeDays)
    setTop(safeTop)

    setSubmittedTsCode(selected.ts_code)
    setSubmittedDays(safeDays)
    setSubmittedTop(safeTop)
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
              <label className="mb-2 block text-sm font-medium">选择股票</label>
              <div className="relative">
                <div className="flex items-center gap-2 rounded-md border px-3 py-2">
                  <Search className="h-4 w-4 text-muted-foreground" />
                  <input
                    type="text"
                    placeholder={selected ? `${selected.name} (${selected.ts_code})` : '搜索股票代码或名称...'}
                    className="w-full bg-transparent text-sm outline-none"
                    onChange={(e) => debouncedSearch(e.target.value)}
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

          <div className="mt-4 flex items-center gap-3">
            <button
              onClick={onSubmit}
              disabled={!selected?.ts_code}
              className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
            >
              查询
            </button>
            <div className="text-sm text-muted-foreground">
              {submittedTsCode ? `当前查询：${submittedTsCode}，近${submittedDays}天，Top ${submittedTop}` : '请选择股票并点击查询'}
            </div>
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
                    <TableHead className="text-right">相似度(%)</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {similarityList.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={5} className="text-center text-muted-foreground">
                        暂无数据
                      </TableCell>
                    </TableRow>
                  ) : (
                    similarityList.map((r, idx) => (
                      <TableRow key={r.ts_code}>
                        <TableCell>{idx + 1}</TableCell>
                        <TableCell>{r.ts_code}</TableCell>
                        <TableCell>{r.name || '-'}</TableCell>
                        <TableCell className="text-right">{formatNumber(r.similarity, 6)}</TableCell>
                        <TableCell className="text-right">{formatPercent(r.similarity * 100, 2)}</TableCell>
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
