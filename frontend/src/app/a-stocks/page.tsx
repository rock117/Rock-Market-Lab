'use client'

import AStockList from '@/components/stock/AStockList'
import { TrendingUp } from 'lucide-react'

export default function AStocksPage() {
  return (
    <div className="space-y-8">
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <TrendingUp className="h-8 w-8 text-bull" />
          <h1 className="text-3xl font-bold tracking-tight">A股列表</h1>
        </div>
        <p className="text-lg text-muted-foreground">展示全部A股的核心行情与估值指标</p>
      </div>

      <AStockList />
    </div>
  )
}
