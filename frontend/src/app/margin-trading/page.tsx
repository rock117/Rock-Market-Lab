'use client'

import MarginTrading from '@/components/margin-trading/MarginTrading'
import { Scale } from 'lucide-react'

export default function MarginTradingPage() {
  return (
    <div className="space-y-8">
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <Scale className="h-8 w-8 text-orange-500" />
          <h1 className="text-3xl font-bold tracking-tight">融资融券</h1>
        </div>
        <p className="text-lg text-muted-foreground">
          按时间与交易所筛选融资融券数据，并以K线形式展示趋势
        </p>
      </div>

      <MarginTrading />
    </div>
  )
}
