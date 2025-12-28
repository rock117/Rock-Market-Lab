'use client'

import React, { useState, useEffect } from 'react'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { stockDetailApi } from '@/services/api'
import { StockDetail as StockDetailType, StockHistoryResponse } from '@/types'
import KLineChart from '@/components/charts/KLineChart'
import { useToast } from '@/components/ui/toast'
import { 
  formatNumber, 
  formatLargeNumber, 
  formatPercent, 
  formatDate, 
  getTrendColorClass, 
  getStockTrend,
  debounce
} from '@/lib/utils'
import { 
  TrendingUp, 
  TrendingDown, 
  Building2, 
  Users, 
  DollarSign, 
  BarChart3, 
  PieChart,
  ArrowUpDown,
  Target,
  Briefcase,
  Search,
  X,
  Calendar,
  Clock,
  LineChart,
  TableIcon
} from 'lucide-react'

interface StockDetailProps {
  className?: string
}

interface StockSearchResult {
  ts_code: string
  name: string
}

export default function StockDetail({ className }: StockDetailProps) {
  const queryClient = useQueryClient()
  const { showToast } = useToast()

  const [selectedStock, setSelectedStock] = useState('000001.SZ') // 默认选择平安银行
  const [selectedStockName, setSelectedStockName] = useState<string>('')
  const [searchKeyword, setSearchKeyword] = useState('')
  const [searchResults, setSearchResults] = useState<StockSearchResult[]>([])
  const [showSearchResults, setShowSearchResults] = useState(false)

  const [timePeriod, setTimePeriod] = useState<string>('7d')
  
  // 时间选择状态
  const [startDate, setStartDate] = useState(() => {
    const date = new Date()
    date.setMonth(date.getMonth() - 1) // 默认显示近1个月
    return date.toISOString().split('T')[0]
  })
  const [endDate, setEndDate] = useState(() => {
    return new Date().toISOString().split('T')[0]
  })
  
  // 显示模式：table 或 chart
  const [displayMode, setDisplayMode] = useState<'table' | 'chart'>('table')

  // 表格分页（前端分页）
  const [page, setPage] = useState(1)
  const [pageSize] = useState(20)
  
  // 时间选择模式：custom 或 quick
  const [timeMode, setTimeMode] = useState<'custom' | 'quick'>('quick')
  
  // 获取股票详情
  const { data: stockDetail, isLoading, error } = useQuery({
    queryKey: ['stock-detail', selectedStock],
    queryFn: () => stockDetailApi.getStockDetail(selectedStock),
    staleTime: 5 * 60 * 1000, // 5分钟缓存
  })

  // 股票详情加载错误提示
  useEffect(() => {
    if (error) {
      showToast(`获取股票详情失败: ${error instanceof Error ? error.message : '未知错误'}`, 'error')
    }
  }, [error, showToast])

  // 搜索股票
  const { data: searchData, error: searchError } = useQuery({
    queryKey: ['search-stocks', searchKeyword],
    queryFn: () => stockDetailApi.searchStocks(searchKeyword),
    enabled: searchKeyword.length >= 1,
    staleTime: 2 * 60 * 1000,
  })

  // 搜索股票错误提示
  useEffect(() => {
    if (searchError) {
      showToast(`搜索股票失败: ${searchError instanceof Error ? searchError.message : '未知错误'}`, 'error')
    }
  }, [searchError, showToast])

  // 获取历史价格数据
  const { data: historyData, isLoading: historyLoading, error: historyError } = useQuery({
    queryKey: ['stock-history', selectedStock, timeMode, startDate, endDate, timePeriod],
    queryFn: () =>
      stockDetailApi.getStockHistory(selectedStock, {
        timeMode,
        startDate: timeMode === 'custom' ? startDate : undefined,
        endDate: timeMode === 'custom' ? endDate : undefined,
        timePeriod: timeMode === 'quick' ? timePeriod : undefined,
      }),
    staleTime: 5 * 60 * 1000,
  })

  // 历史数据加载错误提示
  useEffect(() => {
    if (historyError) {
      showToast(`获取历史数据失败: ${historyError instanceof Error ? historyError.message : '未知错误'}`, 'error')
    }
  }, [historyError, showToast])

  // 切换股票 / 时间条件后回到第一页
  useEffect(() => {
    setPage(1)
  }, [selectedStock, timeMode, startDate, endDate, timePeriod, displayMode])

  const totalRows = historyData?.data?.length ?? 0
  const totalPages = Math.max(1, Math.ceil(totalRows / pageSize))
  const safePage = Math.min(page, totalPages)
  const pagedHistoryData = historyData?.data?.slice((safePage - 1) * pageSize, safePage * pageSize) ?? []

  useEffect(() => {
    if (searchData?.stocks) {
      setSearchResults(searchData.stocks)
      setShowSearchResults(true)
    }
  }, [searchData])

  // 当选中的股票变化时，确保历史数据查询被触发
  useEffect(() => {
    // 查询会自动触发因为 selectedStock 在 queryKey 中
  }, [selectedStock])

  // 防抖搜索
  const debouncedSearch = debounce((keyword: string) => {
    setSearchKeyword(keyword)
  }, 300)

  // 选择股票
  const selectStock = (stock: StockSearchResult) => {
    setSelectedStock(stock.ts_code)
    setSelectedStockName(stock.name)
    setSearchKeyword('')
    setShowSearchResults(false)

    // 立即失效并重新获取股票详情和历史数据
    queryClient.invalidateQueries({ queryKey: ['stock-detail', stock.ts_code] })
    queryClient.invalidateQueries({ queryKey: ['stock-history', stock.ts_code] })
  }

  // 清空搜索
  const clearSearch = () => {
    setSearchKeyword('')
    setShowSearchResults(false)
  }

  // 快捷时间选择（仅用于 custom 模式下的日期计算）
  const setQuickTimeRange = (period: string) => {
    const end = new Date()
    const start = new Date()
    
    // 根据 period 格式计算日期范围
    const match = period.match(/^(\d+)([dwmy])$/i)
    if (match) {
      const value = parseInt(match[1])
      const unit = match[2].toLowerCase()
      
      switch (unit) {
        case 'd':
          start.setDate(end.getDate() - value)
          break
        case 'w':
          start.setDate(end.getDate() - value * 7)
          break
        case 'm':
          start.setMonth(end.getMonth() - value)
          break
        case 'y':
          start.setFullYear(end.getFullYear() - value)
          break
      }
    }
    
    setStartDate(start.toISOString().split('T')[0])
    setEndDate(end.toISOString().split('T')[0])
  }

  if (isLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            股票详情
          </CardTitle>
          <CardDescription>
            详细的股票基本面和交易数据分析
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-center h-64">
            <div className="text-muted-foreground">加载中...</div>
          </div>
        </CardContent>
      </Card>
    )
  }

  if (error || !stockDetail) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-destructive">
            <Building2 className="h-5 w-5" />
            股票详情 - 加载失败
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">
            <p className="text-muted-foreground">数据加载失败，请稍后重试</p>
          </div>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className={className}>
      {/* 股票选择器 */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Building2 className="h-5 w-5" />
            股票详情
          </CardTitle>
          <CardDescription>
            选择股票查看详细的基本面和交易数据
          </CardDescription>
        </CardHeader>
        <CardContent>
          {/* 股票搜索框 */}
          <div className="relative mb-6">
            <div className="flex items-center gap-2">
              <Search className="h-4 w-4 text-muted-foreground" />
              <div className="relative flex-1 max-w-md">
                <input
                  type="text"
                  placeholder="搜索股票代码或名称..."
                  className="w-full px-3 py-2 border rounded-md text-sm pr-8"
                  onChange={(e) => debouncedSearch(e.target.value)}
                  onFocus={() => searchResults.length > 0 && setShowSearchResults(true)}
                />
                {searchKeyword && (
                  <button
                    onClick={clearSearch}
                    className="absolute right-2 top-1/2 transform -translate-y-1/2 text-muted-foreground hover:text-foreground"
                  >
                    <X className="h-3 w-3" />
                  </button>
                )}
              </div>

              <select
                value={timeMode}
                onChange={(e) => setTimeMode(e.target.value as 'custom' | 'quick')}
                className="w-48 px-3 py-2 border rounded-md text-sm bg-background"
              >
                <option value="quick">快捷时间选择</option>
                <option value="custom">自定义时间范围</option>
              </select>

              {timeMode === 'custom' ? (
                <>
                  <input
                    type="date"
                    value={startDate}
                    onChange={(e) => setStartDate(e.target.value)}
                    className="w-40 px-3 py-2 border rounded-md text-sm"
                    max={endDate}
                  />
                  <input
                    type="date"
                    value={endDate}
                    onChange={(e) => setEndDate(e.target.value)}
                    className="w-40 px-3 py-2 border rounded-md text-sm"
                    min={startDate}
                  />
                </>
              ) : (
                <select
                  value={timePeriod}
                  onChange={(e) => {
                    const period = e.target.value
                    setTimePeriod(period)
                  }}
                  className="w-40 px-3 py-2 border rounded-md text-sm bg-background"
                >
                  <option value="">快捷区间</option>
                  <option value="7d">近7天</option>
                  <option value="1m">近1个月</option>
                  <option value="3m">近3个月</option>
                  <option value="6m">近6个月</option>
                  <option value="1y">近1年</option>
                </select>
              )}
            </div>
            
            {/* 搜索结果下拉框 */}
            {showSearchResults && searchResults.length > 0 && (
              <div className="absolute top-full left-6 right-0 mt-1 bg-white border rounded-md shadow-lg z-10 max-h-60 overflow-y-auto max-w-md">
                {searchResults.map((stock) => (
                  <div
                    key={stock.ts_code}
                    className="p-3 hover:bg-muted cursor-pointer border-b last:border-b-0"
                    onClick={() => selectStock(stock)}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <span className="font-medium">{stock.name}</span>
                        <span className="text-sm text-muted-foreground ml-2">{stock.ts_code}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* 当前选中股票信息 */}
          <div className="mb-4 p-3 bg-muted rounded-lg">
            <div className="flex items-center gap-2">
              <span className="text-sm text-muted-foreground">当前选中的股票:</span>
              <span className="font-medium">{selectedStockName || stockDetail.name || '--'}({stockDetail.ts_code || selectedStock})</span>
            </div>
          </div>
          
          {/* 股票基本信息 */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">当前价格</p>
              <p className="text-2xl font-bold">{formatNumber(stockDetail.current_price, 2)}</p>
              <div className="flex items-center gap-2 mt-1">
                <span className={`text-sm font-medium ${getTrendColorClass(getStockTrend(stockDetail.change))}`}>
                  {stockDetail.change > 0 ? '+' : ''}{formatNumber(stockDetail.change, 2)}
                </span>
                <span className={`text-sm ${getTrendColorClass(getStockTrend(stockDetail.pct_chg))}`}>
                  ({stockDetail.pct_chg > 0 ? '+' : ''}{formatPercent(stockDetail.pct_chg)})
                </span>
              </div>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">PE / PB</p>
              <div className="flex items-center gap-2">
                <span className="text-lg font-bold">{formatNumber(stockDetail.pe_ratio, 1)}</span>
                <span className="text-muted-foreground">/</span>
                <span className="text-lg font-bold">{formatNumber(stockDetail.pb_ratio, 2)}</span>
              </div>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">5日涨幅</p>
              <p className={`text-lg font-bold ${getTrendColorClass(getStockTrend(stockDetail.five_day_return))}`}>
                {stockDetail.five_day_return > 0 ? '+' : ''}{formatPercent(stockDetail.five_day_return)}
              </p>
            </div>
            
            <div className="p-4 border rounded-lg">
              <p className="text-sm text-muted-foreground">股东人数</p>
              <p className="text-lg font-bold">{formatNumber(stockDetail.shareholder_count.holder_count, 0)}</p>
              <p className="text-xs text-muted-foreground">
                变化: {stockDetail.shareholder_count.change_ratio > 0 ? '+' : ''}{formatPercent(stockDetail.shareholder_count.change_ratio)}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 概念板块 - 公共部分 */}
      <Card className="mt-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            概念板块
          </CardTitle>
          <CardDescription>
            股票所属的概念和行业板块
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <PieChart className="h-4 w-4" />
                概念
              </h4>
              <div className="flex flex-wrap gap-2">
                {stockDetail.concepts.map((concept, index) => (
                  <span 
                    key={index}
                    className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                  >
                    {concept}
                  </span>
                ))}
              </div>
            </div>
            <div>
              <h4 className="font-medium mb-3 flex items-center gap-2">
                <Briefcase className="h-4 w-4" />
                板块
              </h4>
              <div className="flex flex-wrap gap-2">
                {stockDetail.sectors.map((sector, index) => (
                  <span 
                    key={index}
                    className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm"
                  >
                    {sector}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* 标签页内容 */}
      <div className="mt-6 border rounded-lg bg-card">
        <Tabs defaultValue="history" className="w-full">
          <TabsList className="inline-flex h-12 items-center justify-start rounded-t-lg rounded-b-none bg-muted/50 p-1 text-muted-foreground w-auto border-b">
            <TabsTrigger value="history" className="flex items-center gap-2 data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-primary">
              <LineChart className="h-4 w-4" />
              历史数据
            </TabsTrigger>
            <TabsTrigger value="details" className="flex items-center gap-2 data-[state=active]:bg-background data-[state=active]:border-b-2 data-[state=active]:border-primary">
              <BarChart3 className="h-4 w-4" />
              基本面 & 交易数据
            </TabsTrigger>
          </TabsList>

          {/* 第一个标签页：历史数据 */}
          <TabsContent value="history" className="p-6 space-y-6 m-0">

          {/* 历史价格数据 */}
          <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            历史价格数据
          </CardTitle>
          <CardDescription>
            查看股票在指定时间范围内的历史价格走势
          </CardDescription>
        </CardHeader>
        <CardContent>
          {/* 时间选择和显示模式控制 */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
            {/* 显示模式选择 */}
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">显示模式</label>
                <div className="flex gap-2">
                  <button
                    onClick={() => setDisplayMode('table')}
                    className={`flex items-center gap-2 px-4 py-2 rounded-md text-sm ${
                      displayMode === 'table' 
                        ? 'bg-primary text-primary-foreground' 
                        : 'border hover:bg-muted'
                    }`}
                  >
                    <TableIcon className="h-4 w-4" />
                    表格模式
                  </button>
                  <button
                    onClick={() => setDisplayMode('chart')}
                    className={`flex items-center gap-2 px-4 py-2 rounded-md text-sm ${
                      displayMode === 'chart' 
                        ? 'bg-primary text-primary-foreground' 
                        : 'border hover:bg-muted'
                    }`}
                  >
                    <LineChart className="h-4 w-4" />
                    K线模式
                  </button>
                </div>
              </div>
              
              {/* 数据统计 */}
              {historyData && (
                <div className="text-sm text-muted-foreground">
                  时间范围：{startDate} 至 {endDate}<br />
                  数据条数：{historyData.total} 条
                </div>
              )}
            </div>
          </div>

          {/* 历史数据展示 */}
          {historyLoading && (
            <div className="flex items-center justify-center py-8">
              <div className="text-muted-foreground">加载历史数据中...</div>
            </div>
          )}

          {historyError && (
            <div className="text-center py-8">
              <p className="text-destructive mb-4">历史数据加载失败</p>
            </div>
          )}

          {historyData && displayMode === 'table' && (
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>交易日期</TableHead>
                    <TableHead className="text-right">收盘价</TableHead>
                    <TableHead className="text-right">涨跌幅</TableHead>
                    <TableHead className="text-right">成交量</TableHead>
                    <TableHead className="text-right">换手率</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {pagedHistoryData.map((item, index) => (
                    <TableRow key={index}>
                      <TableCell className="font-medium">{formatDate(item.trade_date)}</TableCell>
                      <TableCell className="text-right font-medium">{formatNumber(item.close, 2)}</TableCell>
                      <TableCell className={`text-right font-medium ${
                        item.pct_chg > 0 ? 'text-red-600' : 'text-green-600'
                      }`}>
                        {item.pct_chg > 0 ? '+' : ''}{formatNumber(item.pct_chg, 2)}%
                      </TableCell>
                      <TableCell className="text-right">{formatNumber(item.amount / 100000, 2)}亿</TableCell>
                      <TableCell className="text-right">{formatNumber(item.turnover_rate, 2)}%</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>

              <div className="flex items-center justify-between mt-4">
                <div className="text-sm text-muted-foreground">
                  第 {safePage} / {totalPages} 页（共 {totalRows} 条）
                </div>
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setPage(p => Math.max(1, p - 1))}
                    disabled={safePage <= 1}
                    className={`px-3 py-1 rounded-md text-sm border ${safePage <= 1 ? 'opacity-50 cursor-not-allowed' : 'hover:bg-muted'}`}
                  >
                    上一页
                  </button>
                  <button
                    onClick={() => setPage(p => Math.min(totalPages, p + 1))}
                    disabled={safePage >= totalPages}
                    className={`px-3 py-1 rounded-md text-sm border ${safePage >= totalPages ? 'opacity-50 cursor-not-allowed' : 'hover:bg-muted'}`}
                  >
                    下一页
                  </button>
                </div>
              </div>
            </div>
          )}

          {historyData && displayMode === 'chart' && (
            <KLineChart 
              data={historyData.data} 
              stockName={historyData.name}
              className="w-full"
            />
          )}
        </CardContent>
          </Card>
        </TabsContent>

        {/* 第二个标签页：基本面和交易数据 */}
        <TabsContent value="details" className="p-6 space-y-6 m-0">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* 基本面数据 */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <BarChart3 className="h-5 w-5" />
                  基本面数据
                </CardTitle>
                <CardDescription>
                  财务指标和盈利能力分析
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-2 gap-4">
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">ROE (净资产收益率)</p>
                    <p className="text-xl font-bold text-bull">{formatPercent(stockDetail.fundamentals.roe)}</p>
                  </div>
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">毛利率</p>
                    <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.gross_margin)}</p>
                  </div>
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">净利率</p>
                    <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.net_margin)}</p>
                  </div>
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">资产负债率</p>
                    <p className="text-xl font-bold">{formatPercent(stockDetail.fundamentals.debt_ratio)}</p>
                  </div>
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">营收增长率</p>
                    <p className={`text-xl font-bold ${getTrendColorClass(getStockTrend(stockDetail.fundamentals.revenue_growth))}`}>
                      {stockDetail.fundamentals.revenue_growth > 0 ? '+' : ''}{formatPercent(stockDetail.fundamentals.revenue_growth)}
                    </p>
                  </div>
                  <div className="p-3 bg-muted/30 rounded-lg">
                    <p className="text-sm text-muted-foreground">净利润增长率</p>
                    <p className={`text-xl font-bold ${getTrendColorClass(getStockTrend(stockDetail.fundamentals.net_profit_growth))}`}>
                      {stockDetail.fundamentals.net_profit_growth > 0 ? '+' : ''}{formatPercent(stockDetail.fundamentals.net_profit_growth)}
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* 融资融券数据 */}
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <DollarSign className="h-5 w-5" />
                  融资融券
                </CardTitle>
                <CardDescription>
                  融资融券交易数据
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                    <span className="text-sm text-muted-foreground">融资余额</span>
                    <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.margin_balance)}</span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                    <span className="text-sm text-muted-foreground">融资买入额</span>
                    <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.margin_buy)}</span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                    <span className="text-sm text-muted-foreground">融券余额</span>
                    <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.short_balance)}</span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-muted/30 rounded-lg">
                    <span className="text-sm text-muted-foreground">融券卖出量</span>
                    <span className="font-bold">{formatLargeNumber(stockDetail.margin_trading.short_sell)}</span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-bull/10 rounded-lg border border-bull/20">
                    <span className="text-sm text-muted-foreground">融资融券比例</span>
                    <span className="font-bold text-bull">{formatPercent(stockDetail.margin_trading.margin_ratio)}</span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* 大宗交易 */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <ArrowUpDown className="h-5 w-5" />
                大宗交易
              </CardTitle>
              <CardDescription>
                近期大宗交易记录
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="rounded-md border">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>交易日期</TableHead>
                      <TableHead className="text-right">成交价格</TableHead>
                      <TableHead className="text-right">成交量(万股)</TableHead>
                      <TableHead className="text-right">成交额(万元)</TableHead>
                      <TableHead>买方</TableHead>
                      <TableHead>卖方</TableHead>
                      <TableHead className="text-right">溢价率</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {stockDetail.block_trades.map((trade, index) => (
                      <TableRow key={index}>
                        <TableCell>{formatDate(trade.trade_date)}</TableCell>
                        <TableCell className="text-right font-medium">
                          {formatNumber(trade.price, 2)}
                        </TableCell>
                        <TableCell className="text-right">
                          {formatNumber(trade.volume / 10000, 0)}
                        </TableCell>
                        <TableCell className="text-right">
                          {formatNumber(trade.amount / 10000, 0)}
                        </TableCell>
                        <TableCell className="text-sm">{trade.buyer}</TableCell>
                        <TableCell className="text-sm">{trade.seller}</TableCell>
                        <TableCell className={`text-right font-medium ${getTrendColorClass(getStockTrend(trade.premium_rate))}`}>
                          {trade.premium_rate > 0 ? '+' : ''}{formatPercent(trade.premium_rate)}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            </CardContent>
          </Card>

          {/* 增减持数据 */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Users className="h-5 w-5" />
                股东增减持
              </CardTitle>
              <CardDescription>
                主要股东增减持变动情况
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="rounded-md border">
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>股东名称</TableHead>
                      <TableHead>变动类型</TableHead>
                      <TableHead className="text-right">变动股数(万股)</TableHead>
                      <TableHead className="text-right">变动比例</TableHead>
                      <TableHead>变动日期</TableHead>
                      <TableHead className="text-right">变动后持股比例</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {stockDetail.shareholding_changes.map((change, index) => (
                      <TableRow key={index}>
                        <TableCell className="font-medium max-w-[200px] truncate">
                          {change.holder_name}
                        </TableCell>
                        <TableCell>
                          <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                            change.change_type === 'increase' 
                              ? 'bg-bull/10 text-bull border border-bull/20' 
                              : 'bg-bear/10 text-bear border border-bear/20'
                          }`}>
                            {change.change_type === 'increase' ? '增持' : '减持'}
                          </span>
                        </TableCell>
                        <TableCell className="text-right">
                          {formatNumber(Math.abs(change.change_shares) / 10000, 0)}
                        </TableCell>
                        <TableCell className={`text-right font-medium ${getTrendColorClass(getStockTrend(change.change_ratio))}`}>
                          {change.change_ratio > 0 ? '+' : ''}{formatPercent(change.change_ratio)}
                        </TableCell>
                        <TableCell>{formatDate(change.change_date)}</TableCell>
                        <TableCell className="text-right font-medium">
                          {formatPercent(change.current_ratio)}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
