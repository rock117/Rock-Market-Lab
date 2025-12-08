'use client'

import StockDetail from '@/components/stock/StockDetail'
import { Building2 } from 'lucide-react'

export default function StockDetailPage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <Building2 className="h-8 w-8 text-purple-500" />
          <h1 className="text-3xl font-bold tracking-tight">
            个股详情分析
          </h1>
        </div>
        <p className="text-lg text-muted-foreground">
          深度分析个股的PE/PB、涨幅、基本面数据、大宗交易、概念板块、增减持、融资融券、股东人数等信息
        </p>
      </div>

      {/* 个股详情组件 */}
      <StockDetail />
    </div>
  )
}
