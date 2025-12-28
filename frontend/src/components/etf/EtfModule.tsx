'use client'

import React, { useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { etfApi } from '@/services/api'
import type { EtfHolding, EtfItem } from '@/types'
import { formatNumber } from '@/lib/utils'

export default function EtfModule() {
  const [listKeyword, setListKeyword] = useState('')

  const [holdingKeyword, setHoldingKeyword] = useState('')
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
      const name = (item.csname || item.cname || '').toLowerCase()
      return item.ts_code.toLowerCase().includes(kw) || name.includes(kw)
    })
  }, [etfList, listKeyword])

  const holdingCandidates = useMemo(() => {
    const kw = holdingKeyword.trim().toLowerCase()
    if (!kw) return etfList.slice(0, 50)
    return etfList
      .filter((item: EtfItem) => {
        const name = (item.csname || item.cname || '').toLowerCase()
        return item.ts_code.toLowerCase().includes(kw) || name.includes(kw)
      })
      .slice(0, 50)
  }, [etfList, holdingKeyword])

  const {
    data: holdings = [],
    isLoading: holdingsLoading,
    error: holdingsError,
  } = useQuery<EtfHolding[]>({
    queryKey: ['etf-holdings', selectedEtf?.ts_code],
    queryFn: () => etfApi.getEtfHoldings(selectedEtf!.ts_code),
    enabled: !!selectedEtf?.ts_code,
    staleTime: 2 * 60 * 1000,
  })

  const latestEndDate = holdings.length > 0 ? holdings[0].end_date : ''

  return (
    <div className="space-y-6">
      <Tabs defaultValue="list" className="w-full">
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
                placeholder="搜索 ts_code 或 名称"
              />

              {listLoading && <div className="text-muted-foreground text-sm">加载中...</div>}
              {listError && <div className="text-destructive text-sm">加载ETF列表失败</div>}

              {!listLoading && !listError && (
                <div className="rounded-md border overflow-x-auto">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>ts_code</TableHead>
                        <TableHead>名称</TableHead>
                        <TableHead>跟踪指数</TableHead>
                        <TableHead>市场</TableHead>
                        <TableHead>类型</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {filteredEtfList.map(item => (
                        <TableRow key={item.ts_code}>
                          <TableCell className="font-medium">{item.ts_code}</TableCell>
                          <TableCell>{item.csname || item.cname || '-'}</TableCell>
                          <TableCell>{item.index_name || item.index_code || '-'}</TableCell>
                          <TableCell>{item.exchange || '-'}</TableCell>
                          <TableCell>{item.etf_type || '-'}</TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
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
                  placeholder="搜索并选择 ETF（ts_code / 名称）"
                />

                <div className="rounded-md border max-h-56 overflow-y-auto">
                  {holdingCandidates.map(item => {
                    const isActive = selectedEtf?.ts_code === item.ts_code
                    return (
                      <button
                        key={item.ts_code}
                        onClick={() => setSelectedEtf(item)}
                        className={`w-full text-left px-3 py-2 text-sm hover:bg-muted ${isActive ? 'bg-muted' : ''}`}
                      >
                        <span className="font-medium">{item.ts_code}</span>
                        <span className="ml-2 text-muted-foreground">{item.csname || item.cname || ''}</span>
                      </button>
                    )
                  })}
                </div>
              </div>

              {selectedEtf && (
                <div className="text-sm text-muted-foreground">
                  当前选中：{selectedEtf.csname || selectedEtf.cname || '--'}({selectedEtf.ts_code})
                  {latestEndDate ? `，报告期：${latestEndDate}` : ''}
                </div>
              )}

              {holdingsLoading && selectedEtf && <div className="text-muted-foreground text-sm">加载持仓中...</div>}
              {holdingsError && selectedEtf && <div className="text-destructive text-sm">加载持仓失败</div>}

              {!holdingsLoading && !holdingsError && selectedEtf && (
                <div className="rounded-md border overflow-x-auto">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>成分股</TableHead>
                        <TableHead className="text-right">市值</TableHead>
                        <TableHead className="text-right">金额</TableHead>
                        <TableHead className="text-right">占净值比(%)</TableHead>
                        <TableHead className="text-right">占流通股比(%)</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {(holdings as EtfHolding[]).map((h, idx) => (
                        <TableRow key={`${h.symbol}-${idx}`}>
                          <TableCell className="font-medium">{h.symbol}</TableCell>
                          <TableCell className="text-right">{formatNumber(h.mkv, 2)}</TableCell>
                          <TableCell className="text-right">{formatNumber(h.amount, 2)}</TableCell>
                          <TableCell className="text-right">{h.stk_mkv_ratio == null ? '-' : formatNumber(h.stk_mkv_ratio, 4)}</TableCell>
                          <TableCell className="text-right">{h.stk_float_ratio == null ? '-' : formatNumber(h.stk_float_ratio, 4)}</TableCell>
                        </TableRow>
                      ))}
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
