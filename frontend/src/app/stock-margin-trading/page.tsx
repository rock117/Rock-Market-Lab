'use client'

import StockMarginTrading from '@/components/margin-trading/StockMarginTrading'
import { Scale } from 'lucide-react'

export default function StockMarginTradingPage() {
  return (
    <div className="space-y-8">
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <Scale className="h-8 w-8 text-orange-500" />
          <h1 className="text-3xl font-bold tracking-tight">个股融资融券</h1>
        </div>
        <p className="text-lg text-muted-foreground">
          选择股票并按时间范围筛选融资余额数据，以折线形式展示趋势
        </p>
      </div>

      <StockMarginTrading />
    </div>
  )
}
