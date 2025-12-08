'use client'

import UsStockList from '@/components/us-stocks/UsStockList'
import { Globe } from 'lucide-react'

export default function UsStocksPage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <Globe className="h-8 w-8 text-blue-500" />
          <h1 className="text-3xl font-bold tracking-tight">
            美股市场
          </h1>
        </div>
        <p className="text-lg text-muted-foreground">
          美股主要公司的基本信息、财务指标和投资价值分析
        </p>
      </div>

      {/* 美股列表组件 */}
      <UsStockList />
    </div>
  )
}
