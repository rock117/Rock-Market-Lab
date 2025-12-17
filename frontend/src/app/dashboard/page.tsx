'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { 
  TrendingUp, 
  Globe, 
  Building2, 
  BarChart3,
  PieChart,
  LineChart,
  Activity,
  DollarSign,
  Users,
  Target,
  Briefcase,
  Calculator,
  Search,
  Filter,
  Grid3X3,
  List,
  Clock,
  Star
} from 'lucide-react'

// 模块分类定义
const moduleCategories = [
  {
    id: 'market',
    name: '市场数据',
    description: '实时市场行情和统计数据',
    icon: TrendingUp,
    color: 'text-bull',
    modules: [
      { id: 'market-summary', name: 'A股大盘', description: '市场概览、涨跌分布', status: 'active', path: '/market' },
      { id: 'us-stocks', name: '美股市场', description: '美股列表、财务指标', status: 'active', path: '/us-stocks' },
      { id: 'hk-stocks', name: '港股市场', description: '港股行情、沪深港通', status: 'coming', path: '/hk-stocks' },
      { id: 'futures', name: '期货市场', description: '商品期货、股指期货', status: 'coming', path: '/futures' },
      { id: 'bonds', name: '债券市场', description: '国债、企业债、可转债', status: 'coming', path: '/bonds' },
      { id: 'funds', name: '基金市场', description: '公募基金、ETF、私募', status: 'coming', path: '/funds' }
    ]
  },
  {
    id: 'analysis',
    name: '分析工具',
    description: '专业的技术分析和基本面分析工具',
    icon: BarChart3,
    color: 'text-blue-500',
    modules: [
      { id: 'stock-detail', name: '个股详情', description: 'PE/PB、基本面分析', status: 'active', path: '/stock-detail' },
      { id: 'kline-comparison', name: 'K线对比', description: '多证券走势对比', status: 'active', path: '/kline-comparison' },
      { id: 'margin-trading', name: '融资融券', description: '融资融券信息K线展示', status: 'active', path: '/margin-trading' },
      { id: 'technical-analysis', name: '技术分析', description: 'MACD、RSI、布林带', status: 'coming', path: '/technical' },
      { id: 'fundamental-analysis', name: '基本面分析', description: 'ROE、ROA、财务比率', status: 'coming', path: '/fundamental' },
      { id: 'sector-analysis', name: '行业分析', description: '行业对比、轮动分析', status: 'coming', path: '/sector' },
      { id: 'risk-analysis', name: '风险分析', description: 'VaR、夏普比率、最大回撤', status: 'coming', path: '/risk' }
    ]
  },
  {
    id: 'portfolio',
    name: '投资组合',
    description: '投资组合管理和绩效分析',
    icon: Briefcase,
    color: 'text-green-500',
    modules: [
      { id: 'portfolio-manager', name: '组合管理', description: '持仓管理、收益统计', status: 'coming', path: '/portfolio' },
      { id: 'performance-analysis', name: '绩效分析', description: '收益归因、风险调整收益', status: 'coming', path: '/performance' },
      { id: 'asset-allocation', name: '资产配置', description: '大类资产配置建议', status: 'coming', path: '/allocation' },
      { id: 'rebalancing', name: '再平衡', description: '投资组合再平衡策略', status: 'coming', path: '/rebalance' }
    ]
  },
  {
    id: 'trading',
    name: '交易工具',
    description: '交易执行和风险控制工具',
    icon: Activity,
    color: 'text-orange-500',
    modules: [
      { id: 'order-management', name: '订单管理', description: '智能下单、止盈止损', status: 'coming', path: '/orders' },
      { id: 'backtesting', name: '策略回测', description: '历史数据回测验证', status: 'coming', path: '/backtest' },
      { id: 'algo-trading', name: '算法交易', description: '量化策略、自动交易', status: 'coming', path: '/algo' },
      { id: 'risk-control', name: '风险控制', description: '实时风控、预警系统', status: 'coming', path: '/risk-control' }
    ]
  },
  {
    id: 'research',
    name: '研究报告',
    description: '投资研究和市场洞察',
    icon: Search,
    color: 'text-purple-500',
    modules: [
      { id: 'market-research', name: '市场研究', description: '宏观经济、政策解读', status: 'coming', path: '/market-research' },
      { id: 'company-research', name: '公司研究', description: '深度调研、估值模型', status: 'coming', path: '/company-research' },
      { id: 'industry-research', name: '行业研究', description: '产业链分析、竞争格局', status: 'coming', path: '/industry-research' },
      { id: 'investment-ideas', name: '投资观点', description: '投资主题、选股逻辑', status: 'coming', path: '/ideas' }
    ]
  }
]

export default function DashboardPage() {
  const router = useRouter()
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')
  const [searchTerm, setSearchTerm] = useState('')

  // 获取所有模块
  const allModules = moduleCategories.flatMap(cat => 
    cat.modules.map(mod => ({ ...mod, category: cat.name, categoryId: cat.id }))
  )

  // 筛选模块
  const filteredModules = allModules.filter(module => {
    const matchesSearch = module.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         module.description.toLowerCase().includes(searchTerm.toLowerCase())
    const matchesCategory = !selectedCategory || module.categoryId === selectedCategory
    return matchesSearch && matchesCategory
  })

  const activeModules = allModules.filter(m => m.status === 'active')
  const comingSoonModules = allModules.filter(m => m.status === 'coming')

  return (
    <div className="space-y-8">
      {/* 页面标题和统计 */}
      <div className="text-center">
        <h1 className="text-3xl font-bold tracking-tight mb-2">
          股票市场数据平台
        </h1>
        <p className="text-lg text-muted-foreground mb-4">
          专业的金融数据分析和投资决策平台
        </p>
        <div className="flex justify-center gap-6 text-sm">
          <div className="flex items-center gap-2">
            <Badge variant="default" className="bg-bull text-white">
              {activeModules.length}
            </Badge>
            <span className="text-muted-foreground">已上线</span>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="secondary">
              {comingSoonModules.length}
            </Badge>
            <span className="text-muted-foreground">即将上线</span>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="outline">
              {moduleCategories.length}
            </Badge>
            <span className="text-muted-foreground">功能分类</span>
          </div>
        </div>
      </div>

      {/* 搜索和筛选 */}
      <div className="flex flex-col sm:flex-row gap-4 items-center justify-between">
        <div className="flex items-center gap-2 flex-1 max-w-md">
          <Search className="h-4 w-4 text-muted-foreground" />
          <input
            type="text"
            placeholder="搜索功能模块..."
            className="flex-1 px-3 py-2 border rounded-md text-sm"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
        
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1 border rounded-md p-1">
            <button
              onClick={() => setViewMode('grid')}
              className={`p-1 rounded ${viewMode === 'grid' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              <Grid3X3 className="h-4 w-4" />
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`p-1 rounded ${viewMode === 'list' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              <List className="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>

      {/* 分类筛选 */}
      <div className="flex flex-wrap gap-2">
        <button
          onClick={() => setSelectedCategory(null)}
          className={`px-3 py-1 rounded-full text-sm transition-colors ${
            !selectedCategory 
              ? 'bg-primary text-primary-foreground' 
              : 'bg-muted hover:bg-muted/80'
          }`}
        >
          全部
        </button>
        {moduleCategories.map((category) => {
          const Icon = category.icon
          return (
            <button
              key={category.id}
              onClick={() => setSelectedCategory(category.id)}
              className={`flex items-center gap-1 px-3 py-1 rounded-full text-sm transition-colors ${
                selectedCategory === category.id 
                  ? 'bg-primary text-primary-foreground' 
                  : 'bg-muted hover:bg-muted/80'
              }`}
            >
              <Icon className="h-3 w-3" />
              {category.name}
            </button>
          )
        })}
      </div>

      {/* 模块展示 */}
      {viewMode === 'grid' ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {filteredModules.map((module) => {
            const category = moduleCategories.find(cat => cat.id === module.categoryId)
            const Icon = category?.icon || Activity
            
            return (
              <Card 
                key={module.id} 
                className={`cursor-pointer transition-all hover:shadow-md ${
                  module.status === 'active' ? 'hover:border-primary' : 'opacity-75'
                }`}
                onClick={() => {
                  if (module.status === 'active') {
                    router.push(module.path)
                  }
                }}
              >
                <CardHeader className="pb-3">
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      <Icon className={`h-5 w-5 ${category?.color || 'text-muted-foreground'}`} />
                      <CardTitle className="text-base">{module.name}</CardTitle>
                    </div>
                    <Badge 
                      variant={module.status === 'active' ? 'default' : 'secondary'}
                      className={module.status === 'active' ? 'bg-bull text-white' : ''}
                    >
                      {module.status === 'active' ? '已上线' : '即将上线'}
                    </Badge>
                  </div>
                  <CardDescription className="text-sm">
                    {module.description}
                  </CardDescription>
                </CardHeader>
                <CardContent className="pt-0">
                  <div className="text-xs text-muted-foreground">
                    {module.category}
                  </div>
                </CardContent>
              </Card>
            )
          })}
        </div>
      ) : (
        <div className="space-y-2">
          {filteredModules.map((module) => {
            const category = moduleCategories.find(cat => cat.id === module.categoryId)
            const Icon = category?.icon || Activity
            
            return (
              <div 
                key={module.id}
                className={`flex items-center gap-4 p-4 border rounded-lg cursor-pointer transition-all hover:shadow-sm ${
                  module.status === 'active' ? 'hover:border-primary' : 'opacity-75'
                }`}
                onClick={() => {
                  if (module.status === 'active') {
                    router.push(module.path)
                  }
                }}
              >
                <Icon className={`h-5 w-5 ${category?.color || 'text-muted-foreground'}`} />
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="font-medium">{module.name}</h3>
                    <Badge 
                      variant={module.status === 'active' ? 'default' : 'secondary'}
                      className={module.status === 'active' ? 'bg-bull text-white' : ''}
                    >
                      {module.status === 'active' ? '已上线' : '即将上线'}
                    </Badge>
                  </div>
                  <p className="text-sm text-muted-foreground">{module.description}</p>
                </div>
                <div className="text-xs text-muted-foreground">
                  {module.category}
                </div>
              </div>
            )
          })}
        </div>
      )}

      {/* 空状态 */}
      {filteredModules.length === 0 && (
        <div className="text-center py-12">
          <Search className="h-12 w-12 mx-auto mb-4 text-muted-foreground" />
          <h3 className="text-lg font-medium mb-2">未找到匹配的模块</h3>
          <p className="text-muted-foreground">
            尝试调整搜索条件或选择不同的分类
          </p>
        </div>
      )}
    </div>
  )
}
