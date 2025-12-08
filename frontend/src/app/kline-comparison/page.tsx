'use client'

import KLineComparison from '@/components/kline/KLineComparison'
import { BarChart3 } from 'lucide-react'

export default function KLineComparisonPage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <BarChart3 className="h-8 w-8 text-orange-500" />
          <h1 className="text-3xl font-bold tracking-tight">
            K线对比分析
          </h1>
        </div>
        <p className="text-lg text-muted-foreground">
          多证券K线走势对比，支持股票、基金、指数，分析涨跌趋势一致性和相关性
        </p>
      </div>

      {/* K线对比组件 */}
      <KLineComparison />
    </div>
  )
}
