'use client'

import React from 'react'
import { 
  ComposedChart, 
  Line, 
  Bar, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  Legend, 
  ResponsiveContainer,
  ReferenceLine
} from 'recharts'
import { StockHistoryData } from '@/types'
import { formatNumber, formatPercent, formatDate } from '@/lib/utils'

interface KLineChartProps {
  data: StockHistoryData[]
  stockName: string
  className?: string
}

// 自定义K线图形组件
const CandlestickBar = (props: any) => {
  const { payload, x, y, width, height } = props
  if (!payload) return null
  
  const { open, close, high, low } = payload
  const isUp = close >= open
  const color = isUp ? '#ef4444' : '#22c55e' // 红涨绿跌
  
  const bodyHeight = Math.abs(close - open)
  const bodyY = Math.min(close, open)
  const wickTop = high
  const wickBottom = low
  
  // 计算实际的像素位置
  const scale = height / (high - low)
  const wickTopY = y + (high - wickTop) * scale
  const wickBottomY = y + (high - wickBottom) * scale
  const bodyTopY = y + (high - Math.max(open, close)) * scale
  const actualBodyHeight = bodyHeight * scale
  
  return (
    <g>
      {/* 上影线 */}
      <line
        x1={x + width / 2}
        y1={wickTopY}
        x2={x + width / 2}
        y2={bodyTopY}
        stroke={color}
        strokeWidth={1}
      />
      {/* 下影线 */}
      <line
        x1={x + width / 2}
        y1={bodyTopY + actualBodyHeight}
        x2={x + width / 2}
        y2={wickBottomY}
        stroke={color}
        strokeWidth={1}
      />
      {/* K线实体 */}
      <rect
        x={x + width * 0.2}
        y={bodyTopY}
        width={width * 0.6}
        height={Math.max(actualBodyHeight, 1)}
        fill={isUp ? color : color}
        stroke={color}
        strokeWidth={1}
      />
    </g>
  )
}

// 自定义Tooltip
const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    const data = payload[0].payload
    return (
      <div className="bg-white p-3 border rounded-lg shadow-lg">
        <p className="font-medium">{formatDate(label)}</p>
        <div className="space-y-1 text-sm">
          <p>开盘: <span className="font-medium">{formatNumber(data.open, 2)}</span></p>
          <p>最高: <span className="font-medium text-red-600">{formatNumber(data.high, 2)}</span></p>
          <p>最低: <span className="font-medium text-green-600">{formatNumber(data.low, 2)}</span></p>
          <p>收盘: <span className="font-medium">{formatNumber(data.close, 2)}</span></p>
          <p>涨跌幅: <span className={`font-medium ${data.pct_chg > 0 ? 'text-red-600' : 'text-green-600'}`}>
            {data.pct_chg > 0 ? '+' : ''}{formatPercent(data.pct_chg / 100)}
          </span></p>
          <p>成交量: <span className="font-medium">{formatNumber(data.volume / 10000, 0)}万</span></p>
          <p>换手率: <span className="font-medium">{formatPercent(data.turnover_rate / 100)}</span></p>
        </div>
      </div>
    )
  }
  return null
}

export default function KLineChart({ data, stockName, className }: KLineChartProps) {
  // 转换数据格式，按时间排序
  const chartData = data
    .map(item => ({
      date: item.trade_date,
      open: item.open,
      high: item.high,
      low: item.low,
      close: item.close,
      volume: item.volume,
      pct_chg: item.pct_chg,
      turnover_rate: item.turnover_rate,
      // 用于显示的格式化日期
      displayDate: formatDate(item.trade_date).slice(5) // 只显示月-日
    }))
    .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

  if (!data.length) {
    return (
      <div className={`flex items-center justify-center h-96 ${className}`}>
        <div className="text-center">
          <div className="text-muted-foreground mb-2">暂无数据</div>
          <div className="text-sm text-muted-foreground">请选择时间范围查看K线图</div>
        </div>
      </div>
    )
  }

  return (
    <div className={className}>
      <div className="mb-4">
        <h3 className="text-lg font-semibold">{stockName} - K线图</h3>
        <div className="text-sm text-muted-foreground">
          时间范围：{chartData[0]?.date} 至 {chartData[chartData.length - 1]?.date} | 
          数据点数：{chartData.length} 个交易日
        </div>
      </div>
      
      {/* 价格K线图 */}
      <div className="mb-6">
        <h4 className="text-sm font-medium mb-2">价格走势</h4>
        <div className="h-80 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <ComposedChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#e1e5e9" />
              <XAxis 
                dataKey="displayDate" 
                tick={{ fontSize: 12 }}
                stroke="#666"
              />
              <YAxis 
                domain={['dataMin - 5', 'dataMax + 5']}
                tick={{ fontSize: 12 }}
                stroke="#666"
              />
              <Tooltip content={<CustomTooltip />} />
              
              {/* 收盘价线 */}
              <Line 
                type="monotone" 
                dataKey="close" 
                stroke="#2563eb" 
                strokeWidth={2}
                dot={false}
                name="收盘价"
              />
              
              {/* 最高价线 */}
              <Line 
                type="monotone" 
                dataKey="high" 
                stroke="#ef4444" 
                strokeWidth={1}
                strokeDasharray="2 2"
                dot={false}
                name="最高价"
              />
              
              {/* 最低价线 */}
              <Line 
                type="monotone" 
                dataKey="low" 
                stroke="#22c55e" 
                strokeWidth={1}
                strokeDasharray="2 2"
                dot={false}
                name="最低价"
              />
            </ComposedChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* 成交量图 */}
      <div className="mb-4">
        <h4 className="text-sm font-medium mb-2">成交量</h4>
        <div className="h-32 w-full">
          <ResponsiveContainer width="100%" height="100%">
            <ComposedChart data={chartData} margin={{ top: 10, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#e1e5e9" />
              <XAxis 
                dataKey="displayDate" 
                tick={{ fontSize: 12 }}
                stroke="#666"
              />
              <YAxis 
                tick={{ fontSize: 12 }}
                stroke="#666"
              />
              <Tooltip 
                formatter={(value: any) => [formatNumber(value / 10000, 0) + '万', '成交量']}
                labelFormatter={(label) => formatDate(label)}
              />
              
              <Bar 
                dataKey="volume" 
                fill="#26a69a"
                opacity={0.7}
                name="成交量"
              />
            </ComposedChart>
          </ResponsiveContainer>
        </div>
      </div>
      
      <div className="flex flex-wrap gap-4 text-sm text-muted-foreground">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-blue-600 rounded"></div>
          <span>收盘价</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-red-500 rounded"></div>
          <span>最高价</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-green-500 rounded"></div>
          <span>最低价</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 bg-teal-500 rounded opacity-70"></div>
          <span>成交量</span>
        </div>
      </div>
    </div>
  )
}
