'use client'

import { useMemo, useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Select, SelectItem } from '@/components/ui/select'

type CrawlMode = 'username' | 'symbol'

type CrawlResult = {
  ok: boolean
  message: string
  fetchedCount: number
  previewTitles: string[]
}

const FAKE_SYMBOLS = [
  'AAPL',
  'TSLA',
  'MSFT',
  'NVDA',
  'AMZN',
  'GOOG',
  'META',
  'NFLX',
  'BABA',
  'AMD',
]

export default function XueqiuCrawler() {
  const [mode, setMode] = useState<CrawlMode>('username')
  const [username, setUsername] = useState('')
  const [symbol, setSymbol] = useState('')
  const [isRunning, setIsRunning] = useState(false)
  const [result, setResult] = useState<CrawlResult | null>(null)

  const canRun = useMemo(() => {
    if (mode === 'username') {
      return username.trim().length > 0
    }
    return symbol.trim().length > 0
  }, [mode, username, symbol])

  const run = async () => {
    setIsRunning(true)
    setResult(null)

    await new Promise((r) => setTimeout(r, 600))

    const key = mode === 'username' ? username.trim() : symbol.trim()
    const fetchedCount = Math.max(1, Math.min(20, key.length * 2))

    setResult({
      ok: true,
      message: mode === 'username' ? `已按用户名抓取：${key}` : `已按股票代码抓取：${key}`,
      fetchedCount,
      previewTitles: Array.from({ length: Math.min(5, fetchedCount) }).map(
        (_, i) => `抓取到的文章标题 ${i + 1}（假数据）`
      ),
    })
    setIsRunning(false)
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>雪球文章爬取</CardTitle>
        <CardDescription>根据用户名或股票代码抓取文章（两者二选一，假数据）</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="flex items-center gap-3 flex-wrap">
            <div className="flex items-center gap-2">
              <div className="text-sm text-muted-foreground whitespace-nowrap">抓取方式:</div>
              <Select value={mode} onValueChange={(v) => setMode(v as CrawlMode)} className="w-40">
                <SelectItem value="username">用户名</SelectItem>
                <SelectItem value="symbol">股票代码</SelectItem>
              </Select>
            </div>

            {mode === 'username' ? (
              <input
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="请输入雪球用户名..."
                className="h-10 w-72 rounded-md border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                autoComplete="off"
              />
            ) : (
              <Select
                value={symbol}
                onValueChange={(v) => setSymbol(v)}
                placeholder="选择股票代码"
                searchable
                searchPlaceholder="搜索股票代码..."
                className="w-56"
              >
                {FAKE_SYMBOLS.map((s) => (
                  <SelectItem key={s} value={s}>
                    {s}
                  </SelectItem>
                ))}
              </Select>
            )}

            <div className="flex items-center gap-2">
              <Button disabled={!canRun || isRunning} onClick={run}>
                {isRunning ? '抓取中...' : '开始抓取'}
              </Button>
              <Button
                variant="outline"
                disabled={isRunning}
                onClick={() => {
                  setUsername('')
                  setSymbol('')
                  setResult(null)
                }}
              >
                清空
              </Button>
            </div>
          </div>

          {result && (
            <div className="rounded-md border p-4">
              <div className="text-sm font-medium">{result.message}</div>
              <div className="mt-1 text-sm text-muted-foreground">抓取数量（假数据）: {result.fetchedCount}</div>
              <div className="mt-3 text-sm font-medium">预览</div>
              <div className="mt-1 space-y-1 text-sm text-muted-foreground">
                {result.previewTitles.map((t) => (
                  <div key={t}>{t}</div>
                ))}
              </div>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
