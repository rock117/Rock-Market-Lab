'use client'

import React, { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { strategyApi } from '@/services/api'
import { StrategyResult, StrategyType } from '@/types'
import { formatNumber, formatPercent, formatDate, getTrendColorClass, getStockTrend } from '@/lib/utils'
import { 
  Target, 
  Settings, 
  Play, 
  TrendingUp, 
  BarChart3, 
  AlertTriangle,
  CheckCircle,
  Clock,
  DollarSign
} from 'lucide-react'

interface StockSelectionStrategyProps {
  className?: string
}

// 策略类型映射
const STRATEGY_TYPES = [
  { value: 'price_volume_candlestick', label: '价量K线策略', description: '基于价格和成交量的K线形态分析' },
  { value: 'bottom_volume_surge', label: '底部放量上涨策略', description: '识别底部区域的放量上涨信号' },
  { value: 'long_term_bottom_reversal', label: '长期底部反转策略', description: '寻找长期底部的反转机会' },
  { value: 'yearly_high', label: '年内新高策略', description: '筛选创年内新高的强势股' },
  { value: 'price_strength', label: '价格强弱策略', description: '基于相对强弱指标的选股' },
  { value: 'distressed_reversal', label: '困境反转策略', description: '寻找困境中的反转机会' },
  { value: 'single_limit_up', label: '单次涨停策略', description: '识别单次涨停后的机会' },
  { value: 'fundamental', label: '基本面策略', description: '基于财务指标的价值投资策略' },
  { value: 'consecutive_strong', label: '连续强势股策略', description: '筛选连续强势表现的股票' },
  { value: 'turtle', label: '海龟交易策略', description: '经典的趋势跟踪策略' },
  { value: 'limit_up_pullback', label: '涨停回调策略', description: '涨停后回调的买入机会' },
  { value: 'strong_close', label: '强势收盘策略', description: '基于收盘强势的选股策略' },
  { value: 'quality_value', label: '优质价值策略', description: '寻找优质且被低估的股票' },
  { value: 'turnover_ma_bullish', label: '换手率均线多头策略', description: '基于换手率和均线的多头策略' },
  { value: 'low_shadow', label: '低位下影线策略', description: '识别低位长下影线的反转信号' }
]

// 默认参数示例
const DEFAULT_PARAMS: Record<string, any> = {
  price_volume_candlestick: {
    volume_threshold: 1.5,
    price_change_threshold: 0.03,
    lookback_days: 20
  },
  bottom_volume_surge: {
    volume_surge_ratio: 2.0,
    price_bottom_threshold: 0.9,
    surge_days: 3
  },
  fundamental: {
    min_roe: 0.15,
    max_pe: 25,
    min_revenue_growth: 0.1,
    max_debt_ratio: 0.6
  },
  turtle: {
    entry_period: 20,
    exit_period: 10,
    atr_period: 20,
    risk_per_trade: 0.02
  }
}

export default function StockSelectionStrategy({ className }: StockSelectionStrategyProps) {
  const [selectedStrategy, setSelectedStrategy] = useState<string>('')
  const [parameters, setParameters] = useState<string>('')
  const [isRunning, setIsRunning] = useState(false)

  const { data: strategyResult, isLoading, error, refetch } = useQuery({
    queryKey: ['strategy-result', selectedStrategy, parameters],
    queryFn: () => strategyApi.runStrategy(selectedStrategy as StrategyType, JSON.parse(parameters || '{}')),
    enabled: false, // 手动触发
    staleTime: 5 * 60 * 1000,
  })

  // 运行策略
  const runStrategy = async () => {
    if (!selectedStrategy) {
      alert('请选择策略类型')
      return
    }

    try {
      JSON.parse(parameters || '{}') // 验证JSON格式
      setIsRunning(true)
      await refetch() // 手动触发查询
      setIsRunning(false)
    } catch (error) {
      alert('参数格式错误，请输入有效的JSON格式')
      setIsRunning(false)
      return
    }
  }

  // 策略类型变化时更新默认参数
  const handleStrategyChange = (strategyType: string) => {
    setSelectedStrategy(strategyType)
    const defaultParams = DEFAULT_PARAMS[strategyType] || {}
    setParameters(JSON.stringify(defaultParams, null, 2))
  }

  // 获取策略状态颜色
  const getStrategyStatusColor = (signal: string) => {
    switch (signal) {
      case 'BUY': return 'text-green-600'
      case 'SELL': return 'text-red-600'
      case 'HOLD': return 'text-yellow-600'
      default: return 'text-gray-600'
    }
  }

  // 获取策略状态图标
  const getStrategyStatusIcon = (signal: string) => {
    switch (signal) {
      case 'BUY': return <TrendingUp className="h-4 w-4" />
      case 'SELL': return <AlertTriangle className="h-4 w-4" />
      case 'HOLD': return <Clock className="h-4 w-4" />
      default: return <CheckCircle className="h-4 w-4" />
    }
  }

  return (
    <div className={className}>
      {/* 策略配置 */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            选股策略配置
          </CardTitle>
          <CardDescription>
            选择策略类型并配置参数，运行策略获取选股结果
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* 策略选择 */}
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">策略类型</label>
                <select
                  value={selectedStrategy}
                  onChange={(e) => handleStrategyChange(e.target.value)}
                  className="w-full px-3 py-2 border rounded-md text-sm"
                >
                  <option value="">请选择策略类型</option>
                  {STRATEGY_TYPES.map((strategy) => (
                    <option key={strategy.value} value={strategy.value}>
                      {strategy.label}
                    </option>
                  ))}
                </select>
                {selectedStrategy && (
                  <p className="text-xs text-muted-foreground mt-1">
                    {STRATEGY_TYPES.find(s => s.value === selectedStrategy)?.description}
                  </p>
                )}
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={runStrategy}
                  disabled={!selectedStrategy || isRunning}
                  className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Play className="h-4 w-4" />
                  {isRunning ? '运行中...' : '运行策略'}
                </button>
                <button
                  onClick={() => {
                    setSelectedStrategy('')
                    setParameters('')
                  }}
                  className="px-4 py-2 border rounded-md text-sm hover:bg-muted"
                >
                  重置
                </button>
              </div>
            </div>

            {/* 参数配置 */}
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">策略参数 (JSON格式)</label>
                <textarea
                  value={parameters}
                  onChange={(e) => setParameters(e.target.value)}
                  placeholder="请输入JSON格式的策略参数"
                  className="w-full px-3 py-2 border rounded-md text-sm font-mono"
                  rows={8}
                />
                <p className="text-xs text-muted-foreground mt-1">
                  请输入有效的JSON格式参数，例如: {"{"}"volume_threshold": 1.5, "lookback_days": 20{"}"}
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 策略结果 */}
      {isLoading && (
        <Card>
          <CardContent className="py-8">
            <div className="flex items-center justify-center">
              <div className="text-muted-foreground">策略运行中，请稍候...</div>
            </div>
          </CardContent>
        </Card>
      )}

      {error && (
        <Card>
          <CardContent className="py-8">
            <div className="text-center">
              <AlertTriangle className="h-8 w-8 text-destructive mx-auto mb-2" />
              <p className="text-destructive mb-4">策略运行失败</p>
              <button 
                onClick={runStrategy}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                重新运行
              </button>
            </div>
          </CardContent>
        </Card>
      )}

      {strategyResult && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              策略运行结果
            </CardTitle>
            <CardDescription>
              {STRATEGY_TYPES.find(s => s.value === selectedStrategy)?.label} - 
              共找到 {strategyResult.stocks?.length || 0} 只符合条件的股票
            </CardDescription>
          </CardHeader>
          <CardContent>
            {/* 策略统计 */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
              <div className="p-4 border rounded-lg text-center">
                <div className="text-2xl font-bold text-primary">{strategyResult.stocks?.length || 0}</div>
                <div className="text-sm text-muted-foreground">选中股票</div>
              </div>
              <div className="p-4 border rounded-lg text-center">
                <div className="text-2xl font-bold text-green-600">
                  {strategyResult.performance?.success_rate ? formatPercent(strategyResult.performance.success_rate) : 'N/A'}
                </div>
                <div className="text-sm text-muted-foreground">成功率</div>
              </div>
              <div className="p-4 border rounded-lg text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {strategyResult.performance?.avg_return ? formatPercent(strategyResult.performance.avg_return) : 'N/A'}
                </div>
                <div className="text-sm text-muted-foreground">平均收益</div>
              </div>
              <div className="p-4 border rounded-lg text-center">
                <div className="text-2xl font-bold text-orange-600">
                  {strategyResult.risk_level || 'MEDIUM'}
                </div>
                <div className="text-sm text-muted-foreground">风险等级</div>
              </div>
            </div>

            {/* 股票列表 */}
            {strategyResult.stocks && strategyResult.stocks.length > 0 && (
              <div className="overflow-x-auto">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>股票代码</TableHead>
                      <TableHead>股票名称</TableHead>
                      <TableHead>当前价格</TableHead>
                      <TableHead>涨跌幅</TableHead>
                      <TableHead>策略信号</TableHead>
                      <TableHead>信号强度</TableHead>
                      <TableHead>推荐操作</TableHead>
                      <TableHead>更新时间</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {strategyResult.stocks.map((stock, index) => (
                      <TableRow key={stock.ts_code || index}>
                        <TableCell className="font-medium">{stock.ts_code}</TableCell>
                        <TableCell>{stock.name}</TableCell>
                        <TableCell className="text-right">
                          {formatNumber(stock.current_price, 2)}
                        </TableCell>
                        <TableCell className="text-right">
                          <span className={getTrendColorClass(getStockTrend(stock.change_percent))}>
                            {stock.change_percent > 0 ? '+' : ''}{formatPercent(stock.change_percent)}
                          </span>
                        </TableCell>
                        <TableCell>
                          <div className={`flex items-center gap-1 ${getStrategyStatusColor(stock.signal)}`}>
                            {getStrategyStatusIcon(stock.signal)}
                            <span className="text-sm font-medium">{stock.signal}</span>
                          </div>
                        </TableCell>
                        <TableCell className="text-right">
                          <div className="flex items-center gap-1">
                            <div className="w-16 bg-gray-200 rounded-full h-2">
                              <div 
                                className="bg-primary h-2 rounded-full" 
                                style={{ width: `${(stock.signal_strength || 0) * 100}%` }}
                              ></div>
                            </div>
                            <span className="text-xs text-muted-foreground">
                              {formatPercent(stock.signal_strength || 0)}
                            </span>
                          </div>
                        </TableCell>
                        <TableCell>
                          <span className={`px-2 py-1 rounded-full text-xs ${
                            stock.signal === 'BUY' ? 'bg-green-100 text-green-800' :
                            stock.signal === 'SELL' ? 'bg-red-100 text-red-800' :
                            'bg-yellow-100 text-yellow-800'
                          }`}>
                            {stock.signal === 'BUY' ? '买入' : 
                             stock.signal === 'SELL' ? '卖出' : '观望'}
                          </span>
                        </TableCell>
                        <TableCell className="text-sm text-muted-foreground">
                          {formatDate(stock.updated_at || new Date().toISOString())}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            )}

            {/* 无结果提示 */}
            {strategyResult.stocks && strategyResult.stocks.length === 0 && (
              <div className="text-center py-8">
                <Settings className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
                <p className="text-muted-foreground mb-2">当前策略参数下未找到符合条件的股票</p>
                <p className="text-sm text-muted-foreground">请尝试调整策略参数或选择其他策略类型</p>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  )
}
