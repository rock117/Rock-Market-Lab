'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { etfApi, normalizeDate } from '@/services/api'
import type { EtfHolding, EtfItem } from '@/types'
import { formatLargeNumber, formatNumber, formatPercent } from '@/lib/utils'

export default function EtfModule() {
  const [listKeyword, setListKeyword] = useState('')

  const [activeTab, setActiveTab] = useState<'list' | 'holdings'>('list')

  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(20)
  const [sortKey, setSortKey] = useState<
    'tsCode' | 'cname' | 'listDate' | 'exchange' | 'etfType' |
    'close' | 'amount' | 'pct_chg' | 'pct5' | 'pct10' | 'pct20' | 'pct60' | 'vol'
  >('tsCode')
  const [sortDir, setSortDir] = useState<'asc' | 'desc'>('asc')

  const [holdingKeyword, setHoldingKeyword] = useState('')
  const [holdingSearchKeyword, setHoldingSearchKeyword] = useState('')
  const [selectedEtf, setSelectedEtf] = useState<EtfItem | null>(null)

  const { data: etfList = [], isLoading: listLoading, error: listError } = useQuery<EtfItem[]>({
    queryKey: ['etf-list'],
    queryFn: etfApi.getEtfList,
    staleTime: 5 * 60 * 1000,
  })

  const filteredEtfList = useMemo(() => {
    const kw = listKeyword.trim().toLowerCase()
    if (!kw) return etfList
    return etfList.filter((item: EtfItem) => {
      const name = (item.cname || '').toLowerCase()
      const code = (item.tsCode || '').toLowerCase()
      return code.includes(kw) || name.includes(kw)
    })
  }, [etfList, listKeyword])

  useEffect(() => {
    setPage(1)
  }, [listKeyword, pageSize, sortKey, sortDir])

  const sortedEtfList = useMemo(() => {
    const dir = sortDir === 'asc' ? 1 : -1
    const toNumber = (v: any) => {
      if (v === null || v === undefined) return NaN
      const n = typeof v === 'number' ? v : Number(v)
      return Number.isFinite(n) ? n : NaN
    }

    const getVal = (item: EtfItem): string | number => {
      switch (sortKey) {
        case 'tsCode':
          return item.tsCode || ''
        case 'cname':
          return item.cname || ''
        case 'listDate':
          return Date.parse(normalizeDate(String(item.listDate || ''))) || 0
        case 'exchange':
          return item.exchange || ''
        case 'etfType':
          return item.etfType || ''
        case 'close':
          return toNumber(item.close)
        case 'amount':
          return toNumber(item.amount)
        case 'pct_chg':
          return toNumber(item.pct_chg)
        case 'pct5':
          return toNumber(item.pct5)
        case 'pct10':
          return toNumber(item.pct10)
        case 'pct20':
          return toNumber(item.pct20)
        case 'pct60':
          return toNumber(item.pct60)
        case 'vol':
          return toNumber(item.vol)
        default:
          return ''
      }
    }

    return [...filteredEtfList].sort((a, b) => {
      const va = getVal(a)
      const vb = getVal(b)
      if (va === vb) return 0
      if (typeof va === 'number' && typeof vb === 'number') {
        const aValid = Number.isFinite(va)
        const bValid = Number.isFinite(vb)
        if (!aValid && !bValid) return 0
        if (!aValid) return 1
        if (!bValid) return -1
        return va > vb ? dir : -dir
      }
      const sa = String(va)
      const sb = String(vb)
      return sa.localeCompare(sb) * dir
    })
  }, [filteredEtfList, sortDir, sortKey])

  const totalPages = useMemo(() => {
    return Math.max(1, Math.ceil(sortedEtfList.length / pageSize))
  }, [pageSize, sortedEtfList.length])

  const safePage = useMemo(() => {
    return Math.min(Math.max(1, page), totalPages)
  }, [page, totalPages])

  useEffect(() => {
    if (safePage !== page) setPage(safePage)
  }, [page, safePage])

  const pagedEtfList = useMemo(() => {
    const start = (safePage - 1) * pageSize
    return sortedEtfList.slice(start, start + pageSize)
  }, [pageSize, safePage, sortedEtfList])

  const toggleSort = (key: typeof sortKey) => {
    if (sortKey !== key) {
      setSortKey(key)
      setSortDir('asc')
      return
    }
    setSortDir(prev => (prev === 'asc' ? 'desc' : 'asc'))
  }

  const renderSortIcon = (key: typeof sortKey) => {
    if (sortKey !== key) {
      return (
        <span className="ml-1 text-muted-foreground" aria-hidden>
          ↕
        </span>
      )
    }
    return (
      <span className="ml-1" aria-hidden>
        {sortDir === 'asc' ? '↑' : '↓'}
      </span>
    )
  }

  useEffect(() => {
    const kw = holdingKeyword.trim()
    if (!kw) {
      setHoldingSearchKeyword('')
      return
    }

    const t = setTimeout(() => {
      setHoldingSearchKeyword(kw)
    }, 250)

    return () => clearTimeout(t)
  }, [holdingKeyword])

  const {
    data: holdingCandidates = [],
    isLoading: holdingCandidatesLoading,
    error: holdingCandidatesError,
  } = useQuery<EtfItem[]>({
    queryKey: ['etf-search', holdingSearchKeyword],
    queryFn: () => etfApi.searchEtfs(holdingSearchKeyword),
    enabled: holdingSearchKeyword.length > 0,
    staleTime: 30 * 1000,
  })

  const {
    data: holdings = [],
    isLoading: holdingsLoading,
    error: holdingsError,
  } = useQuery<EtfHolding[]>({
    queryKey: ['etf-holdings', selectedEtf?.tsCode],
    queryFn: () => etfApi.getEtfHoldings(selectedEtf!.tsCode!),
    enabled: !!selectedEtf?.tsCode,
    staleTime: 2 * 60 * 1000,
  })

  const latestEndDate = holdings.length > 0 ? holdings[0].end_date : ''

  return (
    <div className="space-y-6">
      <Tabs value={activeTab} onValueChange={v => setActiveTab(v as 'list' | 'holdings')} className="w-full">
        <TabsList>
          <TabsTrigger value="list">ETF列表</TabsTrigger>
          <TabsTrigger value="holdings">ETF持仓</TabsTrigger>
        </TabsList>

        <TabsContent value="list" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>ETF列表</CardTitle>
              <CardDescription>后端返回全部 ETF，搜索在前端过滤（ts_code / 名称）</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Input
                value={listKeyword}
                onChange={e => setListKeyword(e.target.value)}
                placeholder="搜索 ETF代码 或 名称"
              />

              {listLoading && <div className="text-muted-foreground text-sm">加载中...</div>}
              {listError && <div className="text-destructive text-sm">加载ETF列表失败</div>}

              {!listLoading && !listError && (
                <div className="space-y-3">
                  <div className="rounded-md border overflow-x-auto">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead className="cursor-pointer" onClick={() => toggleSort('tsCode')}>
                          etf代码
                          {renderSortIcon('tsCode')}
                        </TableHead>
                        <TableHead className="cursor-pointer" onClick={() => toggleSort('cname')}>
                          名称
                          {renderSortIcon('cname')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('close')}>
                          当前价
                          {renderSortIcon('close')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('amount')}>
                          当前成交量
                          {renderSortIcon('amount')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('pct_chg')}>
                          涨跌幅
                          {renderSortIcon('pct_chg')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('pct5')}>
                          5日涨跌幅
                          {renderSortIcon('pct5')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('pct10')}>
                          10日涨跌幅
                          {renderSortIcon('pct10')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('pct20')}>
                          20日涨跌幅
                          {renderSortIcon('pct20')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('pct60')}>
                          60日涨跌幅
                          {renderSortIcon('pct60')}
                        </TableHead>
                        <TableHead className="cursor-pointer text-right" onClick={() => toggleSort('vol')}>
                          成交量
                          {renderSortIcon('vol')}
                        </TableHead>
                        <TableHead className="cursor-pointer" onClick={() => toggleSort('listDate')}>
                          发行日期
                          {renderSortIcon('listDate')}
                        </TableHead>
                        <TableHead className="cursor-pointer" onClick={() => toggleSort('exchange')}>
                          市场
                          {renderSortIcon('exchange')}
                        </TableHead>
                        <TableHead className="cursor-pointer" onClick={() => toggleSort('etfType')}>
                          境内/境外
                          {renderSortIcon('etfType')}
                        </TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {pagedEtfList.map(item => (
                        <TableRow key={item.tsCode || item.ts_code}>
                          <TableCell
                            className="font-medium cursor-pointer"
                            onClick={() => {
                              setSelectedEtf(item)
                              setHoldingKeyword('')
                              setHoldingSearchKeyword('')
                              setActiveTab('holdings')
                            }}
                          >
                            {item.tsCode || '-'}
                          </TableCell>
                          <TableCell>{item.cname || '-'}</TableCell>
                          <TableCell className="text-right">{item.close == null ? '-' : formatNumber(item.close, 4)}</TableCell>
                          <TableCell className="text-right">{item.amount == null ? '-' : formatLargeNumber(item.amount)}</TableCell>
                          <TableCell className="text-right">{item.pct_chg == null ? '-' : formatPercent(item.pct_chg, 2)}</TableCell>
                          <TableCell className="text-right">{item.pct5 == null ? '-' : formatPercent(item.pct5, 2)}</TableCell>
                          <TableCell className="text-right">{item.pct10 == null ? '-' : formatPercent(item.pct10, 2)}</TableCell>
                          <TableCell className="text-right">{item.pct20 == null ? '-' : formatPercent(item.pct20, 2)}</TableCell>
                          <TableCell className="text-right">{item.pct60 == null ? '-' : formatPercent(item.pct60, 2)}</TableCell>
                          <TableCell className="text-right">{item.vol == null ? '-' : formatLargeNumber(item.vol)}</TableCell>
                          <TableCell>{item.listDate ? normalizeDate(String(item.listDate)) : '-'}</TableCell>
                          <TableCell>{item.exchange || '-'}</TableCell>
                          <TableCell>{item.etfType || '-'}</TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>

                  <div className="flex flex-wrap items-center justify-between gap-2">
                    <div className="text-sm text-muted-foreground">
                      共 {sortedEtfList.length} 条，当前第 {page} / {totalPages} 页
                    </div>
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-muted-foreground">每页</span>
                      <select
                        className="h-9 rounded-md border bg-background px-2 text-sm"
                        value={pageSize}
                        onChange={e => setPageSize(Number(e.target.value))}
                      >
                        <option value={10}>10</option>
                        <option value={20}>20</option>
                        <option value={50}>50</option>
                        <option value={100}>100</option>
                      </select>
                      <button
                        className="h-9 rounded-md border px-3 text-sm disabled:opacity-50"
                        disabled={page <= 1}
                        onClick={() => setPage(1)}
                      >
                        首页
                      </button>
                      <button
                        className="h-9 rounded-md border px-3 text-sm disabled:opacity-50"
                        disabled={page <= 1}
                        onClick={() => setPage(p => Math.max(1, p - 1))}
                      >
                        上一页
                      </button>
                      <button
                        className="h-9 rounded-md border px-3 text-sm disabled:opacity-50"
                        disabled={page >= totalPages}
                        onClick={() => setPage(p => Math.min(totalPages, p + 1))}
                      >
                        下一页
                      </button>
                      <button
                        className="h-9 rounded-md border px-3 text-sm disabled:opacity-50"
                        disabled={page >= totalPages}
                        onClick={() => setPage(totalPages)}
                      >
                        末页
                      </button>
                    </div>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="holdings" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>ETF持仓</CardTitle>
              <CardDescription>选择一个 ETF，展示 fund_portfolio 最新 end_date 的持仓记录</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Input
                  value={holdingKeyword}
                  onChange={e => setHoldingKeyword(e.target.value)}
                  placeholder="搜索并选择 ETF（ETF代码 / 名称）"
                />

                {holdingCandidatesLoading && holdingSearchKeyword && (
                  <div className="text-muted-foreground text-sm">搜索中...</div>
                )}
                {holdingCandidatesError && holdingSearchKeyword && (
                  <div className="text-destructive text-sm">搜索ETF失败</div>
                )}

                {holdingSearchKeyword ? (
                  <div className="rounded-md border max-h-56 overflow-y-auto">
                    {(holdingCandidates as EtfItem[]).map(item => {
                      const isActive = selectedEtf?.tsCode === item.tsCode
                      return (
                        <button
                          key={item.tsCode}
                          onClick={() => {
                            setSelectedEtf(item)
                            setHoldingKeyword('')
                            setHoldingSearchKeyword('')
                          }}
                          className={`w-full text-left px-3 py-2 text-sm hover:bg-muted ${isActive ? 'bg-muted' : ''}`}
                        >
                          <span className="font-medium">{item.tsCode}</span>
                          <span className="ml-2 text-muted-foreground">{item.cname || ''}</span>
                        </button>
                      )
                    })}

                    {!holdingCandidatesLoading && (holdingCandidates as EtfItem[]).length === 0 && (
                      <div className="px-3 py-2 text-sm text-muted-foreground">无匹配结果</div>
                    )}
                  </div>
                ) : (
                  <div className="text-muted-foreground text-sm">输入关键词后显示搜索结果</div>
                )}
              </div>

              {selectedEtf && (
                <div className="text-sm text-muted-foreground">
                  当前选中：{selectedEtf.cname || '--'}({selectedEtf.tsCode || '--'})
                  {latestEndDate ? `，报告期：${latestEndDate}` : ''}
                </div>
              )}

              {selectedEtf && (
                <div className="rounded-md border overflow-x-auto">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>股票代码</TableHead>
                        <TableHead>股票名</TableHead>
                        <TableHead className="text-right">市值</TableHead>
                        <TableHead className="text-right">金额</TableHead>
                        <TableHead className="text-right">占净值比(%)</TableHead>
                        <TableHead className="text-right">占流通股比(%)</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {holdingsLoading ? (
                        <TableRow>
                          <TableCell colSpan={6} className="text-center text-muted-foreground">
                            加载中...
                          </TableCell>
                        </TableRow>
                      ) : holdingsError ? (
                        <TableRow>
                          <TableCell colSpan={6} className="text-center text-destructive">
                            加载持仓失败
                          </TableCell>
                        </TableRow>
                      ) : (holdings as EtfHolding[]).length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={6} className="text-center text-muted-foreground">
                            没数据
                          </TableCell>
                        </TableRow>
                      ) : (
                        (holdings as EtfHolding[]).map((h, idx) => (
                          <TableRow key={`${h.symbol}-${idx}`}>
                            <TableCell className="font-medium">{h.symbol}</TableCell>
                            <TableCell>{h.name || '-'}</TableCell>
                            <TableCell className="text-right">{formatNumber(h.mkv, 2)}</TableCell>
                            <TableCell className="text-right">{formatNumber(h.amount, 2)}</TableCell>
                            <TableCell className="text-right">
                              {h.stk_mkv_ratio == null ? '-' : formatNumber(h.stk_mkv_ratio, 4)}
                            </TableCell>
                            <TableCell className="text-right">
                              {h.stk_float_ratio == null ? '-' : formatNumber(h.stk_float_ratio, 4)}
                            </TableCell>
                          </TableRow>
                        ))
                      )}
                    </TableBody>
                  </Table>
                </div>
              )}

              {!selectedEtf && <div className="text-muted-foreground text-sm">请先选择一个 ETF</div>}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
