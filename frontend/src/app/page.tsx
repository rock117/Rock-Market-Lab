'use client'

import MarketSummary from '@/components/market/MarketSummary'
import UsStockList from '@/components/us-stocks/UsStockList'
import StockDetail from '@/components/stock/StockDetail'
import KLineComparison from '@/components/kline/KLineComparison'
import { TrendingUp, Globe, Building2, BarChart3 } from 'lucide-react'

export default function HomePage() {
  return (
    <div className="space-y-8">
      {/* 页面标题 */}
      <div className="text-center">
        <h1 className="text-3xl font-bold tracking-tight mb-2">
          股票市场数据平台
        </h1>
        <p className="text-muted-foreground">
          A股大盘数据监控 & 美股市场分析 & 个股详情分析 & K线走势对比
        </p>
      </div>

      {/* 功能导航卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="p-6 border rounded-lg text-center hover:shadow-md transition-shadow">
          <TrendingUp className="h-12 w-12 text-bull mx-auto mb-4" />
          <h3 className="font-semibold mb-2">A股大盘</h3>
          <p className="text-sm text-muted-foreground">
            实时监控A股市场涨跌分布和成交数据
          </p>
        </div>

        <div className="p-6 border rounded-lg text-center hover:shadow-md transition-shadow">
          <Globe className="h-12 w-12 text-blue-500 mx-auto mb-4" />
          <h3 className="font-semibold mb-2">美股市场</h3>
          <p className="text-sm text-muted-foreground">
            美股公司基本面和财务指标分析
          </p>
        </div>

        <div className="p-6 border rounded-lg text-center hover:shadow-md transition-shadow">
          <Building2 className="h-12 w-12 text-purple-500 mx-auto mb-4" />
          <h3 className="font-semibold mb-2">个股详情</h3>
          <p className="text-sm text-muted-foreground">
            深度分析个股基本面和交易数据
          </p>
        </div>

        <div className="p-6 border rounded-lg text-center hover:shadow-md transition-shadow">
          <BarChart3 className="h-12 w-12 text-orange-500 mx-auto mb-4" />
          <h3 className="font-semibold mb-2">K线对比</h3>
          <p className="text-sm text-muted-foreground">
            多证券K线走势对比和趋势分析
          </p>
        </div>
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

      {/* 个股详情 */}
      <section id="stock-detail">
        <div className="mb-6">
          <h2 className="text-2xl font-bold mb-2">个股详情分析</h2>
          <p className="text-muted-foreground">
            深度分析个股的PE/PB、涨幅、基本面数据、大宗交易、概念板块、增减持、融资融券、股东人数等信息
          </p>
        </div>
        <StockDetail />
      </section>

      {/* K线对比分析 */}
      <section id="kline-comparison">
        <div className="mb-6">
          <h2 className="text-2xl font-bold mb-2">K线对比分析</h2>
          <p className="text-muted-foreground">
            多证券K线走势对比，支持股票、基金、指数，分析涨跌趋势一致性和相关性
          </p>
        </div>
        <KLineComparison />
      </section>
    </div>
  )
}
