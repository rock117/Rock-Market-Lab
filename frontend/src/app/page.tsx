'use client'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import MarketSummary from '@/components/market/MarketSummary'
import UsStockList from '@/components/us-stocks/UsStockList'
import { TrendingUp, Globe, BarChart3, PieChart } from 'lucide-react'

export default function HomePage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <h1 className="text-3xl font-bold tracking-tight mb-2">
          股票市场数据分析平台
        </h1>
        <p className="text-muted-foreground">
          实时监控A股和美股市场动态，提供专业的投资分析工具
        </p>
      </div>

      {/* 功能导航卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="hover:shadow-md transition-shadow cursor-pointer">
          <CardContent className="p-6 text-center">
            <TrendingUp className="h-12 w-12 text-bull mx-auto mb-4" />
            <h3 className="font-semibold mb-2">A股大盘</h3>
            <p className="text-sm text-muted-foreground">
              实时监控A股市场涨跌分布和成交数据
            </p>
          </CardContent>
        </Card>

        <Card className="hover:shadow-md transition-shadow cursor-pointer">
          <CardContent className="p-6 text-center">
            <Globe className="h-12 w-12 text-blue-500 mx-auto mb-4" />
            <h3 className="font-semibold mb-2">美股市场</h3>
            <p className="text-sm text-muted-foreground">
              美股公司基本面和财务指标分析
            </p>
          </CardContent>
        </Card>

        <Card className="hover:shadow-md transition-shadow cursor-pointer">
          <CardContent className="p-6 text-center">
            <BarChart3 className="h-12 w-12 text-orange-500 mx-auto mb-4" />
            <h3 className="font-semibold mb-2">技术分析</h3>
            <p className="text-sm text-muted-foreground">
              K线图表和技术指标分析工具
            </p>
          </CardContent>
        </Card>

        <Card className="hover:shadow-md transition-shadow cursor-pointer">
          <CardContent className="p-6 text-center">
            <PieChart className="h-12 w-12 text-purple-500 mx-auto mb-4" />
            <h3 className="font-semibold mb-2">投资组合</h3>
            <p className="text-sm text-muted-foreground">
              个人投资组合管理和收益分析
            </p>
          </CardContent>
        </Card>
      </div>

      {/* A股大盘数据汇总 */}
      <section id="market">
        <div className="mb-6">
          <h2 className="text-2xl font-bold mb-2">A股大盘数据</h2>
          <p className="text-muted-foreground">
            实时A股市场概览，包含涨跌家数、成交量、涨跌分布等关键指标
          </p>
        </div>
        <MarketSummary />
      </section>

      {/* 美股列表 */}
      <section id="us-stocks">
        <div className="mb-6">
          <h2 className="text-2xl font-bold mb-2">美股市场</h2>
          <p className="text-muted-foreground">
            美股主要公司的基本信息、财务指标和投资价值分析
          </p>
        </div>
        <UsStockList />
      </section>

      {/* 快速统计 */}
      <section className="bg-muted/30 rounded-lg p-6">
        <h3 className="text-lg font-semibold mb-4">今日市场快览</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
          <div>
            <p className="text-2xl font-bold text-bull">2,156</p>
            <p className="text-sm text-muted-foreground">上涨股票</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-bear">2,834</p>
            <p className="text-sm text-muted-foreground">下跌股票</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-orange-500">45</p>
            <p className="text-sm text-muted-foreground">涨停股票</p>
          </div>
          <div>
            <p className="text-2xl font-bold text-blue-500">9,876亿</p>
            <p className="text-sm text-muted-foreground">总成交额</p>
          </div>
        </div>
      </section>
    </div>
  )
}
