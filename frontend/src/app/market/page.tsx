'use client'

import MarketSummary from '@/components/market/MarketSummary'
import { TrendingUp } from 'lucide-react'

export default function MarketPage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <TrendingUp className="h-8 w-8 text-bull" />
          <h1 className="text-3xl font-bold tracking-tight">
            A股大盘数据
          </h1>
        </div>
        <p className="text-lg text-muted-foreground">
          实时A股市场概览，包含涨跌家数、成交量、涨跌分布等关键指标
        </p>
      </div>

      {/* 市场数据组件 */}
      <MarketSummary />
    </div>
  )
}
