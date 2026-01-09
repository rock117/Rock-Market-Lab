'use client'

import { useMemo, useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'

type XueqiuArticle = {
  id: string
  title: string
  author: string
  symbol?: string
  createdAt: string
  summary: string
}

const FAKE_ARTICLES: XueqiuArticle[] = Array.from({ length: 57 }).map((_, idx) => {
  const i = idx + 1
  const symbol = i % 3 === 0 ? 'AAPL' : i % 3 === 1 ? 'TSLA' : 'MSFT'
  return {
    id: String(i),
    title: `雪球文章标题 ${i}`,
    author: i % 2 === 0 ? 'rock' : 'alice',
    symbol,
    createdAt: `2026-01-${String((i % 28) + 1).padStart(2, '0')} 10:${String(i % 60).padStart(2, '0')}`,
    summary:
      '这里是文章摘要（假数据）。后续接入后端接口后，会展示真实的内容摘要、阅读数、评论数等信息。',
  }
})

export default function XueqiuArticles() {
  const [page, setPage] = useState(1)
  const [pageSize] = useState(10)

  const total = FAKE_ARTICLES.length
  const totalPages = Math.max(1, Math.ceil(total / pageSize))

  const items = useMemo(() => {
    const start = (page - 1) * pageSize
    return FAKE_ARTICLES.slice(start, start + pageSize)
  }, [page, pageSize])

  const canPrev = page > 1
  const canNext = page < totalPages

  return (
    <Card>
      <CardHeader>
        <CardTitle>雪球文章</CardTitle>
        <CardDescription>文章列表（假数据），支持分页</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-[90px]">ID</TableHead>
                <TableHead className="w-[240px]">标题</TableHead>
                <TableHead className="w-[140px]">作者</TableHead>
                <TableHead className="w-[120px]">股票代码</TableHead>
                <TableHead className="w-[180px]">发布时间</TableHead>
                <TableHead>摘要</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {items.map((a) => (
                <TableRow key={a.id}>
                  <TableCell>{a.id}</TableCell>
                  <TableCell className="font-medium">{a.title}</TableCell>
                  <TableCell>{a.author}</TableCell>
                  <TableCell>{a.symbol || 'N/A'}</TableCell>
                  <TableCell>{a.createdAt}</TableCell>
                  <TableCell className="whitespace-normal break-words">{a.summary}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        <div className="mt-4 flex items-center justify-between">
          <div className="text-sm text-muted-foreground">
            共 {total} 篇 | 第 {page} / {totalPages} 页
          </div>

          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" disabled={!canPrev} onClick={() => setPage((p) => Math.max(1, p - 1))}>
              上一页
            </Button>
            <Button variant="outline" size="sm" disabled={!canNext} onClick={() => setPage((p) => Math.min(totalPages, p + 1))}>
              下一页
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
