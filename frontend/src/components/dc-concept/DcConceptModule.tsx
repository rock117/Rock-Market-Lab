'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { ArrowDown, ArrowUp } from 'lucide-react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Select, SelectItem } from '@/components/ui/select'
import { dcConceptApi } from '@/services/api/dc-concept'
import type { ApiDcIndex, ApiDcMember } from '@/types'

type SelectedConcept = {
  ts_code: string
  trade_date: string
  name?: string | null
}

function normalizeText(v: any) {
  return String(v ?? '').trim()
}

function fmt(v: any, digits = 2) {
  if (v === null || v === undefined || v === '') return '--'
  if (typeof v === 'number') return v.toFixed(digits)
  return String(v)
}

function toNumber(v: any): number | null {
  if (v === null || v === undefined || v === '') return null
  if (typeof v === 'number') return Number.isFinite(v) ? v : null
  const n = Number.parseFloat(String(v))
  return Number.isFinite(n) ? n : null
}

function formatTradeDate(v: any) {
  const s = String(v ?? '').trim()
  if (!s) return '--'
  if (/^\d{8}$/.test(s)) return `${s.slice(0, 4)}-${s.slice(4, 6)}-${s.slice(6, 8)}`
  if (/^\d{4}-\d{2}-\d{2}/.test(s)) return s.slice(0, 10)
  return s
}

function formatPercent(v: any) {
  const n = toNumber(v)
  if (n === null) return '--'
  return `${n.toFixed(2)}%`
}

export default function DcConceptModule() {
  const [selected, setSelected] = useState<SelectedConcept | null>(null)
  const [keyword, setKeyword] = useState('')
  const [conceptPage, setConceptPage] = useState(1)
  const [memberPage, setMemberPage] = useState(1)
  const [conceptPageSize, setConceptPageSize] = useState(20)
  const [memberPageSize, setMemberPageSize] = useState(20)
  const pageSizeOptions = [10, 20, 50, 100]
  const [conceptSortKey, setConceptSortKey] = useState<
    'name' | 'ts_code' | 'trade_date' | 'pct_change' | 'leading' | 'leading_pct' | 'holdings_num' | 'up_num' | 'down_num'
  >('trade_date')
  const [conceptSortDir, setConceptSortDir] = useState<'asc' | 'desc'>('desc')

  const conceptsQuery = useQuery({
    queryKey: ['dc_index_latest'],
    queryFn: () => dcConceptApi.listConcepts(),
    enabled: selected === null,
  })

  const membersQuery = useQuery({
    queryKey: ['dc_members', selected?.ts_code, selected?.trade_date],
    queryFn: () => dcConceptApi.listMembers(selected!.ts_code, selected!.trade_date),
    enabled: selected !== null,
  })

  const concepts = (conceptsQuery.data || []) as ApiDcIndex[]
  const members = (membersQuery.data || []) as ApiDcMember[]

  const filteredConcepts = useMemo(() => {
    const q = normalizeText(keyword).toLowerCase()
    if (!q) return concepts
    return concepts.filter((c) => {
      const name = normalizeText(c.name).toLowerCase()
      const code = normalizeText(c.ts_code).toLowerCase()
      return name.includes(q) || code.includes(q)
    })
  }, [concepts, keyword])

  const sortedConcepts = useMemo(() => {
    const dir = conceptSortDir === 'asc' ? 1 : -1
    const getValue = (c: ApiDcIndex) => {
      switch (conceptSortKey) {
        case 'name':
          return normalizeText(c.name)
        case 'ts_code':
          return normalizeText(c.ts_code)
        case 'trade_date':
          return normalizeText(c.trade_date)
        case 'pct_change':
          return toNumber(c.pct_change)
        case 'leading':
          return normalizeText(c.leading)
        case 'leading_pct':
          return toNumber(c.leading_pct)
        case 'holdings_num': {
          const up = toNumber(c.up_num) ?? 0
          const down = toNumber(c.down_num) ?? 0
          return up + down
        }
        case 'up_num':
          return toNumber(c.up_num)
        case 'down_num':
          return toNumber(c.down_num)
        default:
          return null
      }
    }

    const cmp = (a: ApiDcIndex, b: ApiDcIndex) => {
      const av = getValue(a)
      const bv = getValue(b)

      const aNull = av === null || av === undefined || av === ''
      const bNull = bv === null || bv === undefined || bv === ''
      if (aNull && bNull) return 0
      if (aNull) return 1
      if (bNull) return -1

      if (typeof av === 'number' && typeof bv === 'number') {
        if (av === bv) return 0
        return av > bv ? dir : -dir
      }

      const as = String(av)
      const bs = String(bv)
      return as.localeCompare(bs) * dir
    }

    return [...filteredConcepts].sort(cmp)
  }, [conceptSortDir, conceptSortKey, filteredConcepts])

  const conceptTotalPages = useMemo(() => {
    return Math.max(1, Math.ceil(sortedConcepts.length / conceptPageSize))
  }, [sortedConcepts.length, conceptPageSize])

  const memberTotalPages = useMemo(() => {
    return Math.max(1, Math.ceil(members.length / memberPageSize))
  }, [members.length, memberPageSize])

  const pagedConcepts = useMemo(() => {
    const start = (conceptPage - 1) * conceptPageSize
    return sortedConcepts.slice(start, start + conceptPageSize)
  }, [conceptPage, conceptPageSize, sortedConcepts])

  const pagedMembers = useMemo(() => {
    const start = (memberPage - 1) * memberPageSize
    return members.slice(start, start + memberPageSize)
  }, [memberPage, memberPageSize, members])

  useEffect(() => {
    setConceptPage(1)
  }, [keyword])

  useEffect(() => {
    setConceptPage(1)
  }, [conceptSortDir, conceptSortKey])

  useEffect(() => {
    setConceptPage(1)
  }, [conceptPageSize])

  useEffect(() => {
    setMemberPage(1)
  }, [memberPageSize])

  useEffect(() => {
    setConceptPage((p) => Math.min(Math.max(1, p), conceptTotalPages))
  }, [conceptTotalPages])

  useEffect(() => {
    setMemberPage((p) => Math.min(Math.max(1, p), memberTotalPages))
  }, [memberTotalPages])

  const breadcrumb = selected ? (
    <div className="flex items-center gap-2 text-sm text-muted-foreground">
      <button
        className="hover:underline"
        onClick={() => {
          setSelected(null)
          setKeyword('')
        }}
      >
        概念列表
      </button>
      <span>/</span>
      <span className="text-foreground">{selected.name || selected.ts_code}</span>
    </div>
  ) : (
    <div className="text-sm text-muted-foreground">概念列表</div>
  )

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>概念板块</CardTitle>
          <CardDescription>
            {breadcrumb}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {selected === null ? (
            <>
              <div className="flex items-center gap-2 mb-3">
                <Input
                  placeholder="搜索概念（名称/ts_code）"
                  value={keyword}
                  onChange={(e) => setKeyword(e.target.value)}
                />
                <Button variant="outline" onClick={() => conceptsQuery.refetch()} disabled={conceptsQuery.isFetching}>
                  刷新
                </Button>
              </div>

              {conceptsQuery.error ? (
                <div className="text-sm text-destructive">{(conceptsQuery.error as any)?.message || '加载失败'}</div>
              ) : (
                <>
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'name') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('name')
                                setConceptSortDir('asc')
                              }
                            }}
                          >
                            概念名称
                            {conceptSortKey === 'name' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'ts_code') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('ts_code')
                                setConceptSortDir('asc')
                              }
                            }}
                          >
                            概念代码
                            {conceptSortKey === 'ts_code' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'trade_date') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('trade_date')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            日期
                            {conceptSortKey === 'trade_date' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'pct_change') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('pct_change')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            涨跌幅
                            {conceptSortKey === 'pct_change' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'leading') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('leading')
                                setConceptSortDir('asc')
                              }
                            }}
                          >
                            领涨股
                            {conceptSortKey === 'leading' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'leading_pct') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('leading_pct')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            领涨股/涨跌幅
                            {conceptSortKey === 'leading_pct' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'holdings_num') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('holdings_num')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            持仓数
                            {conceptSortKey === 'holdings_num' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'up_num') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('up_num')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            上涨家数
                            {conceptSortKey === 'up_num' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead>
                          <button
                            className="flex items-center gap-1"
                            onClick={() => {
                              if (conceptSortKey === 'down_num') {
                                setConceptSortDir((d) => (d === 'asc' ? 'desc' : 'asc'))
                              } else {
                                setConceptSortKey('down_num')
                                setConceptSortDir('desc')
                              }
                            }}
                          >
                            下跌家数
                            {conceptSortKey === 'down_num' ? (
                              conceptSortDir === 'asc' ? (
                                <ArrowUp className="h-3.5 w-3.5" />
                              ) : (
                                <ArrowDown className="h-3.5 w-3.5" />
                              )
                            ) : null}
                          </button>
                        </TableHead>
                        <TableHead className="text-right">操作</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {conceptsQuery.isLoading ? (
                        <TableRow>
                          <TableCell colSpan={10} className="text-center py-8 text-muted-foreground">
                            加载中...
                          </TableCell>
                        </TableRow>
                      ) : sortedConcepts.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={10} className="text-center py-8 text-muted-foreground">
                            无匹配概念
                          </TableCell>
                        </TableRow>
                      ) : (
                        pagedConcepts.map((c) => (
                          <TableRow key={`${c.ts_code}_${c.trade_date}`}>
                            <TableCell className="font-medium">{c.name || '--'}</TableCell>
                            <TableCell className="font-mono text-xs">{c.ts_code}</TableCell>
                            <TableCell className="font-mono text-xs">{formatTradeDate(c.trade_date)}</TableCell>
                            <TableCell className={toNumber(c.pct_change) && toNumber(c.pct_change)! > 0 ? 'text-red-600 font-mono text-xs' : 'text-green-600 font-mono text-xs'}>
                              {formatPercent(c.pct_change)}
                            </TableCell>
                            <TableCell>{c.leading || <span className="text-muted-foreground">--</span>}</TableCell>
                            <TableCell className={toNumber(c.leading_pct) && toNumber(c.leading_pct)! > 0 ? 'text-red-600 font-mono text-xs' : 'text-green-600 font-mono text-xs'}>
                              {formatPercent(c.leading_pct)}
                            </TableCell>
                            <TableCell className="font-mono text-xs">{fmt((toNumber(c.up_num) ?? 0) + (toNumber(c.down_num) ?? 0), 0)}</TableCell>
                            <TableCell className="font-mono text-xs">{fmt(c.up_num, 0)}</TableCell>
                            <TableCell className="font-mono text-xs">{fmt(c.down_num, 0)}</TableCell>
                            <TableCell className="text-right">
                              <Button
                                size="sm"
                                onClick={() => {
                                  setSelected({ ts_code: c.ts_code, trade_date: c.trade_date, name: c.name })
                                  setMemberPage(1)
                                }}
                              >
                                查看成分股
                              </Button>
                            </TableCell>
                          </TableRow>
                        ))
                      )}
                    </TableBody>
                  </Table>

                  <div className="mt-3 flex items-center justify-between gap-2 text-sm text-muted-foreground">
                    <div>
                      共 {sortedConcepts.length} 条
                      <span className="mx-2">|</span>
                      第 {conceptPage} / {conceptTotalPages} 页
                    </div>
                    <div className="flex items-center gap-2">
                      <div className="flex items-center gap-2">
                        <span>每页</span>
                        <Select
                          value={String(conceptPageSize)}
                          onValueChange={(v) => setConceptPageSize(Number(v) || 20)}
                          className="w-[110px]"
                        >
                          {pageSizeOptions.map((n) => (
                            <SelectItem key={n} value={String(n)}>
                              {n}
                            </SelectItem>
                          ))}
                        </Select>
                      </div>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={conceptPage <= 1}
                        onClick={() => setConceptPage(1)}
                      >
                        首页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={conceptPage <= 1}
                        onClick={() => setConceptPage((p) => Math.max(1, p - 1))}
                      >
                        上一页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={conceptPage >= conceptTotalPages}
                        onClick={() => setConceptPage((p) => Math.min(conceptTotalPages, p + 1))}
                      >
                        下一页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={conceptPage >= conceptTotalPages}
                        onClick={() => setConceptPage(conceptTotalPages)}
                      >
                        末页
                      </Button>
                    </div>
                  </div>
                </>
              )}
            </>
          ) : (
            <>
              <div className="flex items-center justify-between gap-2 mb-3">
                <div className="text-sm text-muted-foreground">
                  ts_code: <span className="font-mono">{selected.ts_code}</span>
                  <span className="mx-2">|</span>
                  trade_date: <span className="font-mono">{selected.trade_date}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Button variant="outline" onClick={() => membersQuery.refetch()} disabled={membersQuery.isFetching}>
                    刷新
                  </Button>
                  <Button
                    variant="outline"
                    onClick={() => {
                      setSelected(null)
                      setKeyword('')
                      setConceptPage(1)
                    }}
                  >
                    返回概念列表
                  </Button>
                </div>
              </div>

              {membersQuery.error ? (
                <div className="text-sm text-destructive">{(membersQuery.error as any)?.message || '加载失败'}</div>
              ) : (
                <>
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>股票名称</TableHead>
                        <TableHead>con_code</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {membersQuery.isLoading ? (
                        <TableRow>
                          <TableCell colSpan={2} className="text-center py-8 text-muted-foreground">
                            加载中...
                          </TableCell>
                        </TableRow>
                      ) : members.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={2} className="text-center py-8 text-muted-foreground">
                            暂无成分股
                          </TableCell>
                        </TableRow>
                      ) : (
                        pagedMembers.map((m) => (
                          <TableRow key={`${m.ts_code}_${m.con_code}_${m.trade_date}`}>
                            <TableCell>{m.name || <span className="text-muted-foreground">--</span>}</TableCell>
                            <TableCell>
                              <Badge variant="secondary" className="font-mono text-xs">{m.con_code}</Badge>
                            </TableCell>
                          </TableRow>
                        ))
                      )}
                    </TableBody>
                  </Table>
                  <div className="mt-3 flex items-center justify-between gap-2 text-sm text-muted-foreground">
                    <div>
                      共 {members.length} 条
                      <span className="mx-2">|</span>
                      第 {memberPage} / {memberTotalPages} 页
                    </div>
                    <div className="flex items-center gap-2">
                      <div className="flex items-center gap-2">
                        <span>每页</span>
                        <Select
                          value={String(memberPageSize)}
                          onValueChange={(v) => setMemberPageSize(Number(v) || 20)}
                          className="w-[110px]"
                        >
                          {pageSizeOptions.map((n) => (
                            <SelectItem key={n} value={String(n)}>
                              {n}
                            </SelectItem>
                          ))}
                        </Select>
                      </div>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={memberPage <= 1}
                        onClick={() => setMemberPage(1)}
                      >
                        首页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={memberPage <= 1}
                        onClick={() => setMemberPage((p) => Math.max(1, p - 1))}
                      >
                        上一页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={memberPage >= memberTotalPages}
                        onClick={() => setMemberPage((p) => Math.min(memberTotalPages, p + 1))}
                      >
                        下一页
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        disabled={memberPage >= memberTotalPages}
                        onClick={() => setMemberPage(memberTotalPages)}
                      >
                        末页
                      </Button>
                    </div>
                  </div>
                </>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
