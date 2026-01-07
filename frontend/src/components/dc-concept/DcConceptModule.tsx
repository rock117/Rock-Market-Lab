'use client'

import React, { useEffect, useMemo, useState } from 'react'
import { useQuery } from '@tanstack/react-query'
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

export default function DcConceptModule() {
  const [selected, setSelected] = useState<SelectedConcept | null>(null)
  const [keyword, setKeyword] = useState('')
  const [conceptPage, setConceptPage] = useState(1)
  const [memberPage, setMemberPage] = useState(1)
  const [conceptPageSize, setConceptPageSize] = useState(20)
  const [memberPageSize, setMemberPageSize] = useState(20)
  const pageSizeOptions = [10, 20, 50, 100]

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

  const conceptTotalPages = useMemo(() => {
    return Math.max(1, Math.ceil(filteredConcepts.length / conceptPageSize))
  }, [filteredConcepts.length, conceptPageSize])

  const memberTotalPages = useMemo(() => {
    return Math.max(1, Math.ceil(members.length / memberPageSize))
  }, [members.length, memberPageSize])

  const pagedConcepts = useMemo(() => {
    const start = (conceptPage - 1) * conceptPageSize
    return filteredConcepts.slice(start, start + conceptPageSize)
  }, [conceptPage, conceptPageSize, filteredConcepts])

  const pagedMembers = useMemo(() => {
    const start = (memberPage - 1) * memberPageSize
    return members.slice(start, start + memberPageSize)
  }, [memberPage, memberPageSize, members])

  useEffect(() => {
    setConceptPage(1)
  }, [keyword])

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
                        <TableHead>概念名称</TableHead>
                        <TableHead>ts_code</TableHead>
                        <TableHead>trade_date</TableHead>
                        <TableHead className="text-right">操作</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {conceptsQuery.isLoading ? (
                        <TableRow>
                          <TableCell colSpan={4} className="text-center py-8 text-muted-foreground">
                            加载中...
                          </TableCell>
                        </TableRow>
                      ) : filteredConcepts.length === 0 ? (
                        <TableRow>
                          <TableCell colSpan={4} className="text-center py-8 text-muted-foreground">
                            无匹配概念
                          </TableCell>
                        </TableRow>
                      ) : (
                        pagedConcepts.map((c) => (
                          <TableRow key={`${c.ts_code}_${c.trade_date}`}>
                            <TableCell className="font-medium">{c.name || '--'}</TableCell>
                            <TableCell className="font-mono text-xs">{c.ts_code}</TableCell>
                            <TableCell className="font-mono text-xs">{c.trade_date}</TableCell>
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
                      共 {filteredConcepts.length} 条
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
