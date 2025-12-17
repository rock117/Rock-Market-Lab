'use client'

import React, { useMemo } from 'react'
import {
  ComposedChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts'
import { StockHistoryData } from '@/types'
import { formatDate, formatNumber, formatPercent } from '@/lib/utils'

interface MarginBalanceKLineChartProps {
  data: StockHistoryData[]
  title: string
  className?: string
}

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null
  const d = payload[0].payload
  const dateValue = d?.date ?? label

  return (
    <div className="bg-white p-3 border rounded-lg shadow-lg">
      <p className="font-medium">日期：{dateValue ? formatDate(dateValue) : '-'}</p>
      <p className="text-sm">融资余额：{formatNumber(d.balance, 2)} 亿元</p>
      <p className={`text-sm ${d.pct_chg >= 0 ? 'text-red-500' : 'text-green-500'}`}>
        涨跌幅：{formatPercent(d.pct_chg)}
      </p>
    </div>
  )
}

export default function MarginBalanceKLineChart({ data, title, className }: MarginBalanceKLineChartProps) {
  const chartData = useMemo(() => {
    return data
      .map((item) => ({
        date: item.trade_date,
        balance: item.close,
        pct_chg: item.pct_chg,
      }))
      .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
  }, [data])

  if (!data.length) {
    return (
      <div className={`flex items-center justify-center h-96 ${className || ''}`}>
        <div className="text-center">
          <div className="text-muted-foreground mb-2">暂无数据</div>
          <div className="text-sm text-muted-foreground">请选择时间范围查看融资余额K线</div>
        </div>
      </div>
    )
  }

  return (
    <div className={className}>
      <div className="mb-4">
        <h3 className="text-lg font-semibold">{title} - 融资余额K线</h3>
        <div className="text-sm text-muted-foreground">
          时间范围：{chartData[0]?.date} 至 {chartData[chartData.length - 1]?.date} | 数据点数：{chartData.length}
        </div>
      </div>

      <div className="h-96 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <ComposedChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#e1e5e9" />
            <XAxis
              dataKey="date"
              tick={{ fontSize: 12 }}
              stroke="#666"
              tickFormatter={(v) => formatDate(v, 'MM-DD')}
              interval={0}
              minTickGap={0}
              angle={-45}
              textAnchor="end"
              height={60}
              tickMargin={10}
            />
            <YAxis
              domain={['dataMin - 5', 'dataMax + 5']}
              tick={{ fontSize: 12 }}
              stroke="#666"
              label={{ value: '亿元', angle: -90, position: 'insideLeft' }}
            />
            <Tooltip content={<CustomTooltip />} />

            <Line type="monotone" dataKey="balance" stroke="#2563eb" strokeWidth={2} dot={false} name="融资余额" />
          </ComposedChart>
        </ResponsiveContainer>
      </div>
    </div>
  )
}
