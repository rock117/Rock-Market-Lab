'use client'

import EtfModule from '@/components/etf/EtfModule'
import { PieChart } from 'lucide-react'

export default function EtfPage() {
  return (
    <div className="space-y-8">
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 mb-4">
          <PieChart className="h-8 w-8 text-emerald-500" />
          <h1 className="text-3xl font-bold tracking-tight">ETF</h1>
        </div>
        <p className="text-lg text-muted-foreground">ETF 列表与 ETF 持仓（最新报告期）</p>
      </div>

      <EtfModule />
    </div>
  )
}
