'use client'

import { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import MarketSummary from '@/components/market/MarketSummary'
import UsStockList from '@/components/us-stocks/UsStockList'
import StockDetail from '@/components/stock/StockDetail'
import KLineComparison from '@/components/kline/KLineComparison'
import { 
  TrendingUp, 
  Globe, 
  Building2, 
  BarChart3, 
  Menu,
  X,
  Search,
  Settings,
  Home,
  PieChart,
  LineChart,
  Activity,
  DollarSign,
  Users,
  Target,
  Briefcase,
  Calculator,
  ChevronDown,
  ChevronRight
} from 'lucide-react'

// 模块定义
const modules = [
  {
    id: 'home',
    name: '首页概览',
    icon: Home,
    color: 'text-gray-600',
    category: 'main'
  },
  {
    id: 'market-summary',
    name: 'A股大盘',
    description: '市场概览、涨跌分布',
    icon: TrendingUp,
    color: 'text-bull',
    category: 'market',
    status: 'active'
  },
  {
    id: 'us-stocks',
    name: '美股市场',
    description: '美股列表、财务指标',
    icon: Globe,
    color: 'text-blue-500',
    category: 'market',
    status: 'active'
  },
  {
    id: 'stock-detail',
    name: '个股详情',
    description: 'PE/PB、基本面分析',
    icon: Building2,
    color: 'text-purple-500',
    category: 'analysis',
    status: 'active'
  },
  {
    id: 'kline-comparison',
    name: 'K线对比',
    description: '多证券走势对比',
    icon: BarChart3,
    color: 'text-orange-500',
    category: 'analysis',
    status: 'active'
  },
  // 即将上线的模块
  {
    id: 'hk-stocks',
    name: '港股市场',
    description: '港股行情、沪深港通',
    icon: Globe,
    color: 'text-green-500',
    category: 'market',
    status: 'coming'
  },
  {
    id: 'technical-analysis',
    name: '技术分析',
    description: 'MACD、RSI、布林带',
    icon: LineChart,
    color: 'text-indigo-500',
    category: 'analysis',
    status: 'coming'
  },
  {
    id: 'portfolio-manager',
    name: '组合管理',
    description: '持仓管理、收益统计',
    icon: Briefcase,
    color: 'text-emerald-500',
    category: 'portfolio',
    status: 'coming'
  },
  {
    id: 'risk-analysis',
    name: '风险分析',
    description: 'VaR、最大回撤',
    icon: Target,
    color: 'text-red-500',
    category: 'analysis',
    status: 'coming'
  }
]

// 分类定义
const categories = [
  { id: 'main', name: '主页', icon: Home },
  { id: 'market', name: '市场数据', icon: TrendingUp },
  { id: 'analysis', name: '分析工具', icon: BarChart3 },
  { id: 'portfolio', name: '投资组合', icon: Briefcase },
  { id: 'trading', name: '交易工具', icon: Activity },
  { id: 'research', name: '研究报告', icon: Search }
]

export default function HomePage() {
  const [activeModule, setActiveModule] = useState('home')
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false)
  const [expandedCategories, setExpandedCategories] = useState<string[]>(['main', 'market', 'analysis'])

  // 切换分类展开状态
  const toggleCategory = (categoryId: string) => {
    setExpandedCategories(prev => 
      prev.includes(categoryId) 
        ? prev.filter(id => id !== categoryId)
        : [...prev, categoryId]
    )
  }

  // 渲染主内容区域
  const renderMainContent = () => {
    switch (activeModule) {
      case 'home':
        return <HomeOverview />
      case 'market-summary':
        return <MarketSummary />
      case 'us-stocks':
        return <UsStockList />
      case 'stock-detail':
        return <StockDetail />
      case 'kline-comparison':
        return <KLineComparison />
      default:
        return <ComingSoonModule moduleName={modules.find(m => m.id === activeModule)?.name || ''} />
    }
  }

  return (
    <div className="flex h-screen bg-background">
      {/* 侧边栏 */}
      <div className={`${sidebarCollapsed ? 'w-16' : 'w-64'} bg-card border-r transition-all duration-300 flex flex-col`}>
        {/* 侧边栏头部 */}
        <div className="p-4 border-b flex items-center justify-between">
          {!sidebarCollapsed && (
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                <span className="text-primary-foreground font-bold text-sm">R</span>
              </div>
              <span className="font-bold">Rock Market</span>
            </div>
          )}
          <button
            onClick={() => setSidebarCollapsed(!sidebarCollapsed)}
            className="p-1 hover:bg-muted rounded"
          >
            {sidebarCollapsed ? <Menu className="h-4 w-4" /> : <X className="h-4 w-4" />}
          </button>
        </div>

        {/* 导航菜单 */}
        <div className="flex-1 overflow-y-auto p-2">
          {categories.map((category) => {
            const categoryModules = modules.filter(m => m.category === category.id)
            if (categoryModules.length === 0) return null
            
            const CategoryIcon = category.icon
            const isExpanded = expandedCategories.includes(category.id)
            
            return (
              <div key={category.id} className="mb-2">
                {/* 分类标题 */}
                {!sidebarCollapsed && (
                  <button
                    onClick={() => toggleCategory(category.id)}
                    className="w-full flex items-center gap-2 px-2 py-1 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                  >
                    <CategoryIcon className="h-4 w-4" />
                    <span className="flex-1 text-left">{category.name}</span>
                    {isExpanded ? <ChevronDown className="h-3 w-3" /> : <ChevronRight className="h-3 w-3" />}
                  </button>
                )}
                
                {/* 模块列表 */}
                {(isExpanded || sidebarCollapsed) && (
                  <div className={sidebarCollapsed ? 'space-y-1' : 'ml-4 space-y-1'}>
                    {categoryModules.map((module) => {
                      const ModuleIcon = module.icon
                      const isActive = activeModule === module.id
                      const isDisabled = module.status === 'coming'
                      
                      return (
                        <button
                          key={module.id}
                          onClick={() => !isDisabled && setActiveModule(module.id)}
                          disabled={isDisabled}
                          className={`w-full flex items-center gap-2 px-2 py-2 rounded text-sm transition-colors ${
                            isActive 
                              ? 'bg-primary text-primary-foreground' 
                              : isDisabled
                                ? 'text-muted-foreground cursor-not-allowed opacity-50'
                                : 'hover:bg-muted'
                          }`}
                          title={sidebarCollapsed ? module.name : undefined}
                        >
                          <ModuleIcon className={`h-4 w-4 ${module.color}`} />
                          {!sidebarCollapsed && (
                            <div className="flex-1 text-left">
                              <div className="font-medium">{module.name}</div>
                              {module.description && (
                                <div className="text-xs opacity-70">{module.description}</div>
                              )}
                            </div>
                          )}
                          {!sidebarCollapsed && module.status === 'coming' && (
                            <Badge variant="secondary" className="text-xs">即将上线</Badge>
                          )}
                        </button>
                      )
                    })}
                  </div>
                )}
              </div>
            )
          })}
        </div>

        {/* 侧边栏底部 */}
        {!sidebarCollapsed && (
          <div className="p-4 border-t">
            <div className="text-xs text-muted-foreground space-y-1">
              <div>已上线: {modules.filter(m => m.status === 'active').length} 个</div>
              <div>即将上线: {modules.filter(m => m.status === 'coming').length} 个</div>
            </div>
          </div>
        )}
      </div>

      {/* 主内容区域 */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* 顶部工具栏 */}
        <div className="bg-card border-b p-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-xl font-bold">
                {modules.find(m => m.id === activeModule)?.name || '首页概览'}
              </h1>
              <p className="text-sm text-muted-foreground">
                {modules.find(m => m.id === activeModule)?.description || '欢迎使用股票市场数据平台'}
              </p>
            </div>
            <div className="flex items-center gap-2">
              <button className="p-2 hover:bg-muted rounded">
                <Search className="h-4 w-4" />
              </button>
              <button className="p-2 hover:bg-muted rounded">
                <Settings className="h-4 w-4" />
              </button>
            </div>
          </div>
        </div>

        {/* 主内容 */}
        <div className="flex-1 overflow-y-auto p-6">
          {renderMainContent()}
        </div>
      </div>
    </div>
  )
}

// 首页概览组件
function HomeOverview() {
  const activeModules = modules.filter(m => m.status === 'active')
  const comingModules = modules.filter(m => m.status === 'coming')
  
  return (
    <div className="space-y-8">
      {/* 统计卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card>
          <CardContent className="p-6 text-center">
            <div className="text-2xl font-bold text-bull mb-2">{activeModules.length}</div>
            <div className="text-sm text-muted-foreground">已上线模块</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-6 text-center">
            <div className="text-2xl font-bold text-blue-500 mb-2">{comingModules.length}</div>
            <div className="text-sm text-muted-foreground">即将上线</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-6 text-center">
            <div className="text-2xl font-bold text-green-500 mb-2">{categories.length}</div>
            <div className="text-sm text-muted-foreground">功能分类</div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-6 text-center">
            <div className="text-2xl font-bold text-purple-500 mb-2">100%</div>
            <div className="text-sm text-muted-foreground">模拟数据</div>
          </CardContent>
        </Card>
      </div>

      {/* 核心功能 */}
      <section>
        <h2 className="text-2xl font-bold mb-6">核心功能模块</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {activeModules.slice(0, 4).map((module) => {
            const Icon = module.icon
            return (
              <Card key={module.id} className="cursor-pointer hover:shadow-md transition-shadow">
                <CardHeader className="pb-3">
                  <div className="flex items-center gap-3 mb-2">
                    <Icon className={`h-6 w-6 ${module.color}`} />
                    <CardTitle className="text-lg">{module.name}</CardTitle>
                  </div>
                  <CardDescription>{module.description}</CardDescription>
                </CardHeader>
                <CardContent className="pt-0">
                  <Badge className="bg-bull text-white">已上线</Badge>
                </CardContent>
              </Card>
            )
          })}
        </div>
      </section>

      {/* 快速开始 */}
      <section>
        <h2 className="text-2xl font-bold mb-6">快速开始</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Card>
            <CardHeader>
              <CardTitle>推荐使用路径</CardTitle>
              <CardDescription>适合新手用户的功能使用顺序</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {activeModules.slice(0, 3).map((module, index) => {
                  const Icon = module.icon
                  return (
                    <div key={module.id} className="flex items-center gap-3 p-3 border rounded-lg">
                      <div className="w-6 h-6 rounded-full bg-primary text-primary-foreground text-xs flex items-center justify-center font-bold">
                        {index + 1}
                      </div>
                      <Icon className={`h-4 w-4 ${module.color}`} />
                      <div className="flex-1">
                        <div className="font-medium">{module.name}</div>
                        <div className="text-sm text-muted-foreground">{module.description}</div>
                      </div>
                    </div>
                  )
                })}
              </div>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle>即将上线</CardTitle>
              <CardDescription>正在开发中的新功能</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {comingModules.slice(0, 3).map((module) => {
                  const Icon = module.icon
                  return (
                    <div key={module.id} className="flex items-center gap-3 p-3 border rounded-lg opacity-75">
                      <Icon className={`h-4 w-4 ${module.color}`} />
                      <div className="flex-1">
                        <div className="font-medium">{module.name}</div>
                        <div className="text-sm text-muted-foreground">{module.description}</div>
                      </div>
                      <Badge variant="secondary">即将上线</Badge>
                    </div>
                  )
                })}
              </div>
            </CardContent>
          </Card>
        </div>
      </section>
    </div>
  )
}

// 即将上线模块占位组件
function ComingSoonModule({ moduleName }: { moduleName: string }) {
  return (
    <div className="flex items-center justify-center h-96">
      <div className="text-center">
        <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mx-auto mb-4">
          <Settings className="h-8 w-8 text-muted-foreground" />
        </div>
        <h3 className="text-xl font-medium mb-2">{moduleName}</h3>
        <p className="text-muted-foreground mb-4">该功能正在开发中，敬请期待</p>
        <Badge variant="secondary">即将上线</Badge>
      </div>
    </div>
  )
}
