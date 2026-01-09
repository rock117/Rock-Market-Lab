'use client'

import React, { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { strategyApi } from '@/services/api'
import { StrategyResult, StrategyType } from '@/types'
import { formatNumber } from '@/lib/utils'
import { 
  Target, 
  Settings, 
  Play, 
  BarChart3, 
  AlertTriangle
} from 'lucide-react'
import { useToast } from '@/components/ui/toast'

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
  { value: 'low_shadow', label: '低位下影线策略', description: '识别低位长下影线的反转信号' },
  { value: 'similarity', label: '股价走势相似策略', description: '股价走势相似策略' },
  { value: 'ma_convergence', label: '均线粘合策略', description: '识别均线粘合形态，筛选下跌后的粘合机会' },
  { value: 'consecutive_bullish', label: '日/周/月连阳策略', description: '识别连续阳线形态，捕捉上升趋势的持续信号' }
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
  },
  ma_convergence: {
    ma_types: ["MA5", "MA10", "MA20"],
    convergence_threshold: 0.05,
    min_convergence_days: 3,
    decline_check_period: 20,
    min_decline_pct: 0.10,
    time_frame: "daily",
    max_convergence_days: 20,
    recent_turnover_rate_min: 5,
    recent_turnover_rate_max: 100
  },
  consecutive_bullish: {
    time_period: "daily",
    min_consecutive_days: 3,
    min_rise_pct: 0.0,
    require_volume_surge: false,
    volume_surge_ratio: 1.2,
    analysis_period: 20
  }
}

export default function StockSelectionStrategy({ className }: StockSelectionStrategyProps) {
  const { showToast } = useToast()
  const [selectedStrategy, setSelectedStrategy] = useState<string>('')
  const [parameters, setParameters] = useState<string>('')
  const [isRunning, setIsRunning] = useState(false)
  const [executionTime, setExecutionTime] = useState<number>(0)
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  const { data: apiResponse, isLoading, error, refetch } = useQuery({
    queryKey: ['strategy-result', selectedStrategy, parameters],
    queryFn: () => strategyApi.runStrategy(selectedStrategy as StrategyType, JSON.parse(parameters || '{}')),
    enabled: false,
    staleTime: 5 * 60 * 1000,
  })

  // 从API响应中提取数据
  const allResults = Array.isArray(apiResponse?.data) ? apiResponse.data : []
  
  // 调试：打印第一条数据查看结构
  if (allResults.length > 0) {
    console.log('📊 策略结果第一条数据:', allResults[0])
    console.log('📊 concepts字段:', allResults[0].concepts)
  }
  
  // 计算分页数据
  const totalItems = allResults.length
  const totalPages = Math.ceil(totalItems / pageSize)
  const startIndex = (page - 1) * pageSize
  const endIndex = startIndex + pageSize
  const strategyResult = allResults.slice(startIndex, endIndex)

  // 运行策略
  const runStrategy = async () => {
    if (!selectedStrategy) {
      showToast('请选择策略类型', 'warning')
      return
    }

    try {
      JSON.parse(parameters || '{}') // 验证JSON格式
    } catch (error) {
      showToast('参数格式错误，请输入有效的JSON格式', 'error')
      return
    }

    try {
      setIsRunning(true)
      setPage(1) // 重置到第一页
      const startTime = Date.now()
      const result = await refetch() // 手动触发查询
      const endTime = Date.now()
      setExecutionTime(endTime - startTime)
      setIsRunning(false)
      
      // 检查是否有错误
      if (result.error) {
        showToast(`策略运行失败：${result.error.message}`, 'error')
      } else {
        showToast(`策略运行成功，找到 ${allResults.length} 只股票`, 'success')
      }
    } catch (error: any) {
      setIsRunning(false)
      showToast(`策略运行失败：${error.message || '未知错误'}`, 'error')
    }
  }

  // 策略类型变化时更新默认参数
  const handleStrategyChange = (strategyType: string) => {
    setSelectedStrategy(strategyType)
    const defaultParams = DEFAULT_PARAMS[strategyType] || {}
    setParameters(JSON.stringify(defaultParams, null, 2))
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

      {allResults.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BarChart3 className="h-5 w-5" />
              策略运行结果
            </CardTitle>
            <CardDescription>
              {STRATEGY_TYPES.find(s => s.value === selectedStrategy)?.label} - 
              共找到 {totalItems} 只符合条件的股票
              {executionTime > 0 && ` · 运行耗时 ${(executionTime / 1000).toFixed(2)}s`}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {/* 股票列表 */}
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="whitespace-nowrap min-w-[100px]">股票代码</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">股票名称</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">当前价格</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[100px]">涨跌幅</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[200px]">核心概念</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[120px]">信号强度</TableHead>
                    <TableHead className="whitespace-nowrap min-w-[350px]">分析结果</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {strategyResult.map((item: any, index: number) => (
                    <TableRow key={item.ts_code || index}>
                      <TableCell className="font-medium font-mono whitespace-nowrap min-w-[100px]">{item.ts_code}</TableCell>
                      <TableCell className="whitespace-nowrap min-w-[120px]">{item.stock_name}</TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[120px]">
                        ¥{formatNumber(item.strategy_result?.current_price || 0, 2)}
                      </TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[100px]">
                        <span className={`font-medium ${
                          (item.strategy_result?.pct_chg || 0) > 0 ? 'text-red-600' :
                          (item.strategy_result?.pct_chg || 0) < 0 ? 'text-green-600' :
                          'text-gray-600'
                        }`}>
                          {(item.strategy_result?.pct_chg || 0) > 0 ? '+' : ''}
                          {formatNumber(item.strategy_result?.pct_chg || 0, 2)}%
                        </span>
                      </TableCell>
                      <TableCell className="text-sm whitespace-nowrap min-w-[200px]">
                        {item.concepts || 'N/A'}
                      </TableCell>
                      <TableCell className="text-right whitespace-nowrap min-w-[120px]">
                        <div className="flex items-center gap-2">
                          <div className="flex-1 bg-gray-200 rounded-full h-2 min-w-[60px]">
                            <div
                              className={`h-2 rounded-full ${
                                item.strategy_result?.signal_strength >= 100 ? 'bg-green-600' :
                                item.strategy_result?.signal_strength >= 80 ? 'bg-blue-600' :
                                'bg-yellow-600'
                              }`}
                              style={{ width: `${Math.min((item.strategy_result?.signal_strength || 0), 100)}%` }}
                            ></div>
                          </div>
                          <span className="text-sm font-medium min-w-[40px]">
                            {item.strategy_result?.signal_strength || 0}
                          </span>
                        </div>
                      </TableCell>
                      <TableCell className="text-sm whitespace-nowrap min-w-[350px]">
                        {item.strategy_result?.analysis_description || 'N/A'}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>

            {/* 分页控件 */}
            {totalPages > 1 && (
              <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mt-6 pt-4 border-t">
                <div className="flex items-center gap-4">
                  <div className="text-sm text-muted-foreground">
                    显示 {startIndex + 1} - {Math.min(endIndex, totalItems)} 条，共 {totalItems} 条
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-sm text-muted-foreground">每页显示</span>
                    <select
                      value={pageSize}
                      onChange={(e) => {
                        setPageSize(Number(e.target.value))
                        setPage(1)
                      }}
                      className="px-2 py-1 border rounded text-sm"
                    >
                      <option value={10}>10</option>
                      <option value={20}>20</option>
                      <option value={50}>50</option>
                      <option value={100}>100</option>
                    </select>
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setPage(1)}
                    disabled={page === 1}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    首页
                  </button>
                  <button
                    onClick={() => setPage(page - 1)}
                    disabled={page === 1}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    上一页
                  </button>
                  <span className="text-sm text-muted-foreground">
                    第 {page} / {totalPages} 页
                  </span>
                  <button
                    onClick={() => setPage(page + 1)}
                    disabled={page === totalPages}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    下一页
                  </button>
                  <button
                    onClick={() => setPage(totalPages)}
                    disabled={page === totalPages}
                    className="px-3 py-1 border rounded text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-muted"
                  >
                    末页
                  </button>
                </div>
              </div>
            )}

          </CardContent>
        </Card>
      )}

      {/* 无结果提示 */}
      {apiResponse && allResults.length === 0 && (
        <Card>
          <CardContent className="py-8">
            <div className="text-center">
              <Settings className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
              <p className="text-muted-foreground mb-2">当前策略参数下未找到符合条件的股票</p>
              <p className="text-sm text-muted-foreground">请尝试调整策略参数或选择其他策略类型</p>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
