'use client'

import React, { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { klineApi } from '@/services/api'
import { Security, SecurityKLineData, TrendAnalysis } from '@/types'
import { formatNumber, formatPercent, debounce } from '@/lib/utils'
import { 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  Legend, 
  ResponsiveContainer,
  ReferenceLine
} from 'recharts'
import { 
  Search, 
  Plus, 
  X, 
  TrendingUp, 
  BarChart3, 
  Activity,
  Target,
  Zap
} from 'lucide-react'

interface KLineComparisonProps {
  className?: string
}

export default function KLineComparison({ className }: KLineComparisonProps) {
  const [selectedSecurities, setSelectedSecurities] = useState<Security[]>([
    { code: '000001.SH', name: '上证指数', type: 'index', market: 'SH' },
    { code: '399006.SZ', name: '创业板指', type: 'index', market: 'SZ' }
  ])
  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<Security[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  // 获取K线数据
  const { data: klineData, isLoading: klineLoading } = useQuery({
    queryKey: ['kline-data', selectedSecurities],
    queryFn: () => klineApi.getSecurityKLineData(selectedSecurities),
    enabled: selectedSecurities.length > 0,
    staleTime: 5 * 60 * 1000,
  })

  // 获取趋势分析
  const { data: trendAnalysis, isLoading: analysisLoading } = useQuery({
    queryKey: ['trend-analysis', selectedSecurities],
    queryFn: () => klineApi.analyzeTrendCorrelation(selectedSecurities),
    enabled: selectedSecurities.length >= 2,
    staleTime: 5 * 60 * 1000,
  })

  // 搜索证券
  const { data: searchData } = useQuery({
    queryKey: ['search-securities', searchKeyword],
    queryFn: () => klineApi.searchSecurities(searchKeyword),
    enabled: searchKeyword.length >= 2,
    staleTime: 2 * 60 * 1000,
  })

  useEffect(() => {
    if (searchData?.securities) {
      setSearchResults(searchData.securities)
      setShowSearchResults(true)
    }
  }, [searchData])

  // 防抖搜索
  const debouncedSearch = debounce((keyword: string) => {
    setSearchKeyword(keyword)
  }, 300)

  // 添加证券
  const addSecurity = (security: Security) => {
    if (selectedSecurities.length >= 8) {
      alert('最多只能比较8个证券')
      return
    }
    
    const exists = selectedSecurities.some(s => s.code === security.code)
    if (!exists) {
      setSelectedSecurities([...selectedSecurities, security])
    }
    setShowSearchResults(false)
    setSearchKeyword('')
  }

  // 移除证券
  const removeSecurity = (code: string) => {
    setSelectedSecurities(selectedSecurities.filter(s => s.code !== code))
  }

  // 获取证券类型标签颜色
  const getSecurityTypeColor = (type: string) => {
    switch (type) {
      case 'stock': return 'bg-blue-100 text-blue-800'
      case 'fund': return 'bg-green-100 text-green-800'
      case 'index': return 'bg-purple-100 text-purple-800'
      default: return 'bg-gray-100 text-gray-800'
    }
  }

  // 获取趋势一致性颜色
  const getTrendConsistencyColor = (consistency: string) => {
    switch (consistency) {
      case 'high': return 'text-bull'
      case 'medium': return 'text-orange-500'
      case 'low': return 'text-bear'
      default: return 'text-muted-foreground'
    }
  }

  // 准备图表数据
  const prepareChartData = () => {
    if (!klineData || klineData.length === 0) return []

    // 获取所有日期
    const allDates = klineData[0]?.data.map(item => item.date) || []
    
    return allDates.map(date => {
      const dataPoint: any = { date }
      
      klineData.forEach((securityData) => {
        const dayData = securityData.data.find(d => d.date === date)
        if (dayData) {
          // 使用收盘价，并进行归一化处理（以第一个数据点为基准）
          const firstPrice = securityData.data[0]?.close || 1
          const normalizedPrice = (dayData.close / firstPrice) * 100
          dataPoint[securityData.security.code] = Number(normalizedPrice.toFixed(2))
        }
      })
      
      return dataPoint
    })
  }

  const chartData = prepareChartData()

  return (
    <div className={className}>
      {/* 证券选择器 */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            K线比较分析
          </CardTitle>
          <CardDescription>
            选择多个证券进行K线走势对比分析，支持股票、基金、指数
          </CardDescription>
        </CardHeader>
        <CardContent>
          {/* 搜索框 */}
          <div className="relative mb-4">
            <div className="flex items-center gap-2">
              <Search className="h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                placeholder="搜索证券代码或名称..."
                className="flex-1 px-3 py-2 border rounded-md text-sm"
                onChange={(e) => debouncedSearch(e.target.value)}
                onFocus={() => searchResults.length > 0 && setShowSearchResults(true)}
              />
              <Plus className="h-4 w-4 text-muted-foreground" />
            </div>
            
            {/* 搜索结果 */}
            {showSearchResults && searchResults.length > 0 && (
              <div className="absolute top-full left-0 right-0 mt-1 bg-white border rounded-md shadow-lg z-10 max-h-60 overflow-y-auto">
                {searchResults.map((security) => (
                  <div
                    key={security.code}
                    className="p-3 hover:bg-muted cursor-pointer border-b last:border-b-0"
                    onClick={() => addSecurity(security)}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <span className="font-medium">{security.name}</span>
                        <span className="text-sm text-muted-foreground ml-2">{security.code}</span>
                      </div>
                      <span className={`px-2 py-1 rounded-full text-xs ${getSecurityTypeColor(security.type)}`}>
                        {security.type === 'stock' ? '股票' : 
                         security.type === 'fund' ? '基金' : '指数'}
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* 已选择的证券 */}
          <div className="space-y-2">
            <h4 className="text-sm font-medium">已选择证券 ({selectedSecurities.length}/8)</h4>
            <div className="flex flex-wrap gap-2">
              {selectedSecurities.map((security, index) => (
                <div
                  key={security.code}
                  className="flex items-center gap-2 px-3 py-2 bg-muted rounded-lg"
                >
                  <div 
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: klineData?.[index]?.color || '#3b82f6' }}
                  ></div>
                  <span className="text-sm font-medium">{security.name}</span>
                  <span className="text-xs text-muted-foreground">{security.code}</span>
                  <span className={`px-2 py-1 rounded-full text-xs ${getSecurityTypeColor(security.type)}`}>
                    {security.type === 'stock' ? '股票' : 
                     security.type === 'fund' ? '基金' : '指数'}
                  </span>
                  <button
                    onClick={() => removeSecurity(security.code)}
                    className="text-muted-foreground hover:text-destructive"
                  >
                    <X className="h-3 w-3" />
                  </button>
                </div>
              ))}
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 趋势分析结果 */}
      {selectedSecurities.length >= 2 && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Activity className="h-5 w-5" />
              趋势相关性分析
            </CardTitle>
            <CardDescription>
              分析所选证券的涨跌趋势一致性
            </CardDescription>
          </CardHeader>
          <CardContent>
            {analysisLoading ? (
              <div className="flex items-center justify-center h-20">
                <div className="text-muted-foreground">分析中...</div>
              </div>
            ) : trendAnalysis ? (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="p-4 border rounded-lg text-center">
                  <Target className="h-8 w-8 mx-auto mb-2 text-blue-500" />
                  <p className="text-sm text-muted-foreground">相关系数</p>
                  <p className="text-2xl font-bold">{trendAnalysis.correlation}</p>
                  <p className="text-xs text-muted-foreground mt-1">
                    {trendAnalysis.correlation > 0 ? '正相关' : 
                     trendAnalysis.correlation < 0 ? '负相关' : '无相关'}
                  </p>
                </div>
                
                <div className="p-4 border rounded-lg text-center">
                  <Zap className="h-8 w-8 mx-auto mb-2 text-orange-500" />
                  <p className="text-sm text-muted-foreground">趋势一致性</p>
                  <p className={`text-2xl font-bold ${getTrendConsistencyColor(trendAnalysis.trend_consistency)}`}>
                    {trendAnalysis.trend_consistency === 'high' ? '高' :
                     trendAnalysis.trend_consistency === 'medium' ? '中' : '低'}
                  </p>
                  <p className="text-xs text-muted-foreground mt-1">
                    同步率: {formatPercent(trendAnalysis.sync_rate)}
                  </p>
                </div>
                
                <div className="p-4 border rounded-lg text-center">
                  <TrendingUp className="h-8 w-8 mx-auto mb-2 text-green-500" />
                  <p className="text-sm text-muted-foreground">分析周期</p>
                  <p className="text-lg font-bold">{trendAnalysis.analysis_period}</p>
                  <p className="text-xs text-muted-foreground mt-1">
                    {selectedSecurities.length}个证券对比
                  </p>
                </div>
              </div>
            ) : null}
          </CardContent>
        </Card>
      )}

      {/* K线图表 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            K线走势对比图
          </CardTitle>
          <CardDescription>
            归一化价格走势对比（以首个交易日为基准100%）
          </CardDescription>
        </CardHeader>
        <CardContent>
          {klineLoading ? (
            <div className="flex items-center justify-center h-96">
              <div className="text-muted-foreground">加载K线数据中...</div>
            </div>
          ) : chartData.length > 0 ? (
            <div className="h-96">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={chartData}>
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis 
                    dataKey="date" 
                    tick={{ fontSize: 10 }}
                    tickFormatter={(value) => {
                      const date = new Date(value)
                      return `${date.getMonth() + 1}/${date.getDate()}`
                    }}
                  />
                  <YAxis 
                    tick={{ fontSize: 10 }}
                    tickFormatter={(value) => `${value}%`}
                  />
                  <Tooltip 
                    formatter={(value: number, name: string) => [
                      `${formatNumber(value, 2)}%`,
                      selectedSecurities.find(s => s.code === name)?.name || name
                    ]}
                    labelFormatter={(label) => `日期: ${label}`}
                  />
                  <Legend 
                    formatter={(value) => selectedSecurities.find(s => s.code === value)?.name || value}
                  />
                  <ReferenceLine y={100} stroke="#666" strokeDasharray="2 2" />
                  
                  {klineData?.map((securityData) => (
                    <Line
                      key={securityData.security.code}
                      type="monotone"
                      dataKey={securityData.security.code}
                      stroke={securityData.color}
                      strokeWidth={2}
                      dot={false}
                      connectNulls={false}
                    />
                  ))}
                </LineChart>
              </ResponsiveContainer>
            </div>
          ) : (
            <div className="flex items-center justify-center h-96">
              <div className="text-center">
                <BarChart3 className="h-12 w-12 mx-auto mb-4 text-muted-foreground" />
                <p className="text-muted-foreground">请选择证券查看K线对比图</p>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* 数据说明 */}
      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="text-lg">使用说明</CardTitle>
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground space-y-2">
          <p>• <strong>归一化处理</strong>：所有证券价格都以首个交易日为基准（100%），便于比较不同价格水平的证券</p>
          <p>• <strong>相关系数</strong>：范围-1到1，越接近1表示正相关性越强，越接近-1表示负相关性越强</p>
          <p>• <strong>趋势一致性</strong>：高（&gt;70%）、中（40-70%）、低（&lt;40%）</p>
          <p>• <strong>支持类型</strong>：股票、基金、指数，最多同时比较8个证券</p>
        </CardContent>
      </Card>
    </div>
  )
}
