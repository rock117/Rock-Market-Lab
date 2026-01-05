'use client'

import React, { useState, useRef, useEffect } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { portfolioApi, stockDetailApi, usStockApi } from '@/services/api'
import { Portfolio, PortfolioStock } from '@/types'
import { formatDate } from '@/lib/utils'
import { Plus, Trash2, Edit2, FolderOpen, X, Search, Save, Tag } from 'lucide-react'
import { useToast } from '@/components/ui/toast'

function renderPctCell(v: number | null | undefined) {
  if (v === null || v === undefined || Number.isNaN(Number(v))) {
    return <span className="text-sm text-muted-foreground">--</span>
  }
  const n = Number(v)
  const cls = n > 0 ? 'text-bear' : 'text-bull'
  return <span className={`text-sm font-medium ${cls}`}>{n.toFixed(2)}%</span>
}

function renderPriceCell(v: number | null | undefined) {
  if (v === null || v === undefined || Number.isNaN(Number(v))) {
    return <span className="text-sm text-muted-foreground">--</span>
  }
  const n = Number(v)
  return <span className="text-sm font-medium">{n.toFixed(2)}</span>
}

// 假数据：标签列表
const MOCK_TAGS = [
  { id: '1', name: '蓝筹股', color: 'bg-blue-100 text-blue-800' },
  { id: '2', name: '成长股', color: 'bg-green-100 text-green-800' },
  { id: '3', name: '价值股', color: 'bg-purple-100 text-purple-800' },
  { id: '4', name: '周期股', color: 'bg-orange-100 text-orange-800' },
  { id: '5', name: '防守股', color: 'bg-red-100 text-red-800' },
  { id: '6', name: '科技股', color: 'bg-indigo-100 text-indigo-800' },
  { id: '7', name: '消费股', color: 'bg-pink-100 text-pink-800' },
  { id: '8', name: '金融股', color: 'bg-yellow-100 text-yellow-800' },
]

interface PortfolioManagerProps {
  className?: string
}

export default function PortfolioManager({ className }: PortfolioManagerProps) {
  const [selectedPortfolio, setSelectedPortfolio] = useState<Portfolio | null>(null)
  const [isLoadingPortfolio, setIsLoadingPortfolio] = useState(false)
  const [isCreating, setIsCreating] = useState(false)
  const [isEditing, setIsEditing] = useState(false)
  const [newPortfolioName, setNewPortfolioName] = useState('')
  const [newPortfolioDesc, setNewPortfolioDesc] = useState('')
  
  // 添加股票相关状态
  const [isAddingStock, setIsAddingStock] = useState(false)
  const [searchKeyword, setSearchKeyword] = useState('')
  const [showSearchResults, setShowSearchResults] = useState(false)
  const [searchResults, setSearchResults] = useState<
    Array<{ market: 'cn' | 'us'; symbol: string; name: string; exchange_id?: string }>
  >([])
  const [selectedTags, setSelectedTags] = useState<string[]>([])
  const [showTagDropdown, setShowTagDropdown] = useState(false)
  const tagDropdownRef = useRef<HTMLDivElement>(null)
  const searchDropdownRef = useRef<HTMLDivElement>(null)
  const [newStock, setNewStock] = useState({
    symbol: '',
    exchange_id: '',
    desc: ''
  })

  // 点击外部关闭标签下拉框
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (tagDropdownRef.current && !tagDropdownRef.current.contains(event.target as Node)) {
        setShowTagDropdown(false)
      }
      if (searchDropdownRef.current && !searchDropdownRef.current.contains(event.target as Node)) {
        setShowSearchResults(false)
      }
    }

    if (showTagDropdown) {
      document.addEventListener('mousedown', handleClickOutside)
      return () => document.removeEventListener('mousedown', handleClickOutside)
    }
    if (showSearchResults) {
      document.addEventListener('mousedown', handleClickOutside)
      return () => document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [showTagDropdown, showSearchResults])

  const searchStocks = async (keyword: string) => {
    const q = keyword.trim()
    if (!q) {
      setSearchResults([])
      setShowSearchResults(false)
      return
    }

    try {
      const [cn, us] = await Promise.all([
        stockDetailApi.searchStocks(q),
        usStockApi.getUsStocks({ page: 1, page_size: 20, keyword: q })
      ])

      const cnRows = (cn?.stocks ?? []).map((r: any) => ({
        market: 'cn' as const,
        symbol: r.ts_code,
        name: r.name,
      }))

      const usRows = (us?.items ?? []).map((r: any) => ({
        market: 'us' as const,
        symbol: r.tsCode || r.symbol,
        name: r.name,
        exchange_id: r.exchangeId || r.exchange,
      }))

      const merged = [...cnRows, ...usRows]
        .filter(r => r.symbol && r.name)
        .slice(0, 30)

      setSearchResults(merged)
      setShowSearchResults(true)
    } catch (e) {
      console.error('searchStocks failed', e)
      setSearchResults([])
      setShowSearchResults(false)
      showToast('搜索股票失败', 'error')
    }
  }

  const handlePickStock = (item: { market: 'cn' | 'us'; symbol: string; name: string; exchange_id?: string }) => {
    setNewStock(prev => ({
      ...prev,
      symbol: item.symbol,
      exchange_id: item.market === 'us' ? (item.exchange_id || '') : ''
    }))
    setSearchKeyword(`${item.symbol} ${item.name}`)
    setShowSearchResults(false)
  }
  
  // 编辑股票描述相关状态
  const [editingStockId, setEditingStockId] = useState<string | null>(null)
  const [editingDesc, setEditingDesc] = useState('')
  
  const queryClient = useQueryClient()
  const { showToast, showConfirm } = useToast()

  // 获取所有投资组合
  const { data: portfolios = [], isLoading, error } = useQuery({
    queryKey: ['portfolios'],
    queryFn: () => portfolioApi.getPortfolios(),
  })

  // 调试日志
  console.log('📋 投资组合状态:', { 
    portfolios, 
    isLoading, 
    error,
    count: portfolios.length 
  })

  // 创建投资组合
  const createMutation = useMutation({
    mutationFn: (data: { name: string; description?: string }) =>
      portfolioApi.createPortfolio(data.name, data.description),
    onSuccess: (newPortfolio) => {
      console.log('✅ 创建投资组合成功:', newPortfolio)
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setIsCreating(false)
      setNewPortfolioName('')
      setNewPortfolioDesc('')
      setSelectedPortfolio(newPortfolio)
      showToast('投资组合已创建', 'success')
    },
    onError: (error: Error) => {
      console.error('❌ 创建投资组合失败:', error)
      showToast(error.message, 'error')
    }
  })

  // 更新投资组合
  const updateMutation = useMutation({
    mutationFn: (data: { id: string; name: string; description?: string }) =>
      portfolioApi.updatePortfolio(data.id, { name: data.name, description: data.description }),
    onSuccess: (updatedPortfolio) => {
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setIsEditing(false)
      setSelectedPortfolio(updatedPortfolio)
      showToast('投资组合已更新', 'success')
    },
    onError: (error: Error) => {
      showToast(error.message, 'error')
    }
  })

  // 删除投资组合
  const deleteMutation = useMutation({
    mutationFn: (id: string) => portfolioApi.deletePortfolio(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setSelectedPortfolio(null)
      showToast('投资组合已删除', 'success')
    },
    onError: (error: Error) => {
      showToast(error.message, 'error')
    }
  })

  // 添加股票
  const addStockMutation = useMutation({
    mutationFn: (data: { portfolioId: string; stock: Omit<PortfolioStock, 'id' | 'added_date'> }) =>
      portfolioApi.addStock(data.portfolioId, data.stock),
    onSuccess: (updatedPortfolio) => {
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setSelectedPortfolio(updatedPortfolio)
      setIsAddingStock(false)
      setNewStock({ symbol: '', exchange_id: '', desc: '' })
      setSearchKeyword('')
      showToast('股票已添加到组合', 'success')
    },
    onError: (error: Error) => {
      showToast(error.message, 'error')
    }
  })

  // 删除股票
  const removeStockMutation = useMutation({
    mutationFn: (data: { portfolioId: string; stockId: string }) =>
      portfolioApi.removeStock(data.portfolioId, data.stockId),
    onSuccess: (updatedPortfolio) => {
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setSelectedPortfolio(updatedPortfolio)
      showToast('股票已从组合中移除', 'success')
    },
    onError: (error: Error) => {
      showToast(error.message, 'error')
    }
  })

  // 更新股票描述
  const updateStockMutation = useMutation({
    mutationFn: (data: { portfolioId: string; stockId: string; desc: string }) =>
      portfolioApi.updateStock(data.portfolioId, data.stockId, data.desc),
    onSuccess: (updatedPortfolio) => {
      queryClient.invalidateQueries({ queryKey: ['portfolios'] })
      setSelectedPortfolio(updatedPortfolio)
      setEditingStockId(null)
      setEditingDesc('')
      showToast('描述已更新', 'success')
    },
    onError: (error: Error) => {
      showToast(error.message, 'error')
    }
  })

  const handleCreatePortfolio = () => {
    if (!newPortfolioName.trim()) {
      showToast('请输入组合名称', 'warning')
      return
    }
    createMutation.mutate({
      name: newPortfolioName,
      description: newPortfolioDesc || undefined
    })
  }

  const handleUpdatePortfolio = () => {
    if (!selectedPortfolio || !newPortfolioName.trim()) {
      showToast('请输入组合名称', 'warning')
      return
    }
    updateMutation.mutate({
      id: selectedPortfolio.id,
      name: newPortfolioName,
      description: newPortfolioDesc || undefined
    })
  }

  const handleDeletePortfolio = (id: string, name: string) => {
    showConfirm(
      `确定要删除投资组合 "${name}" 吗？此操作不可恢复！`,
      () => {
        deleteMutation.mutate(id)
      }
    )
  }

  const handleAddStock = () => {
    if (!selectedPortfolio) return
    if (!newStock.symbol.trim()) {
      showToast('请输入股票代码', 'warning')
      return
    }
    addStockMutation.mutate({
      portfolioId: selectedPortfolio.id,
      stock: {
        symbol: newStock.symbol,
        name: '',
        exchange_id: newStock.exchange_id || undefined,
        portfolio_id: selectedPortfolio.id,
        desc: newStock.desc || undefined,
        tags: selectedTags.length > 0 ? selectedTags : undefined
      } as any
    })
  }

  const toggleTag = (tagId: string) => {
    setSelectedTags(prev =>
      prev.includes(tagId)
        ? prev.filter(id => id !== tagId)
        : [...prev, tagId]
    )
  }

  const handleCancelAddStock = () => {
    setIsAddingStock(false)
    setNewStock({ symbol: '', exchange_id: '', desc: '' })
    setSelectedTags([])
    setShowTagDropdown(false)
  }

  const handleRemoveStock = (stockId: string, stockName: string) => {
    if (!selectedPortfolio) return
    
    showConfirm(
      `确定要从组合中移除 "${stockName}" 吗？`,
      () => {
        removeStockMutation.mutate({
          portfolioId: selectedPortfolio.id,
          stockId
        })
      }
    )
  }

  // 开始编辑股票描述
  const startEditingStock = (stock: PortfolioStock) => {
    setEditingStockId(stock.id)
    setEditingDesc(stock.desc || '')
  }

  // 取消编辑
  const cancelEditingStock = () => {
    setEditingStockId(null)
    setEditingDesc('')
  }

  // 保存股票描述
  const handleUpdateStockDesc = () => {
    if (!selectedPortfolio || !editingStockId) return
    updateStockMutation.mutate({
      portfolioId: selectedPortfolio.id,
      stockId: editingStockId,
      desc: editingDesc
    })
  }

  // 加载完整的投资组合数据（包含持仓列表）
  const handleSelectPortfolio = async (portfolio: Portfolio) => {
    console.log('🔍 点击投资组合:', portfolio.id)
    setIsLoadingPortfolio(true)
    try {
      const fullPortfolio = await portfolioApi.getPortfolio(portfolio.id)
      if (fullPortfolio) {
        setSelectedPortfolio(fullPortfolio)
      } else {
        showToast('获取投资组合详情失败', 'error')
      }
    } catch (error) {
      console.error('❌ 加载投资组合失败:', error)
      showToast('获取投资组合详情失败', 'error')
    } finally {
      setIsLoadingPortfolio(false)
    }
  }

  const startEditingPortfolio = () => {
    if (selectedPortfolio) {
      setNewPortfolioName(selectedPortfolio.name)
      setNewPortfolioDesc(selectedPortfolio.description || '')
      setIsEditing(true)
    }
  }

  const cancelEditingPortfolio = () => {
    setIsEditing(false)
    setNewPortfolioName('')
    setNewPortfolioDesc('')
  }

  if (isLoading) {
    return (
      <Card className={className}>
        <CardContent className="py-8">
          <div className="text-center text-muted-foreground">加载中...</div>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className={`grid grid-cols-1 lg:grid-cols-3 gap-6 ${className}`}>
      {/* 左侧：投资组合列表 */}
      <Card className="lg:col-span-1">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="flex items-center gap-2">
              <FolderOpen className="h-5 w-5" />
              投资组合
            </CardTitle>
            <Button
              size="sm"
              onClick={() => setIsCreating(true)}
              disabled={isCreating}
            >
              <Plus className="h-4 w-4 mr-1" />
              新建
            </Button>
          </div>
          <CardDescription>管理您的投资组合</CardDescription>
        </CardHeader>
        <CardContent>
          {/* 创建新组合表单 */}
          {isCreating && (
            <div className="mb-4 p-4 border rounded-lg bg-muted/50">
              <div className="space-y-3">
                <div>
                  <label className="text-sm font-medium">组合名称</label>
                  <Input
                    placeholder="例如：价值投资组合"
                    value={newPortfolioName}
                    onChange={(e) => setNewPortfolioName(e.target.value)}
                  />
                </div>
                <div>
                  <label className="text-sm font-medium">描述（可选）</label>
                  <Textarea
                    placeholder="描述这个投资组合的策略..."
                    value={newPortfolioDesc}
                    onChange={(e) => setNewPortfolioDesc(e.target.value)}
                    rows={3}
                  />
                </div>
                <div className="flex gap-2">
                  <Button
                    size="sm"
                    onClick={handleCreatePortfolio}
                    disabled={createMutation.isPending}
                  >
                    <Save className="h-4 w-4 mr-1" />
                    保存
                  </Button>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => {
                      setIsCreating(false)
                      setNewPortfolioName('')
                      setNewPortfolioDesc('')
                    }}
                  >
                    <X className="h-4 w-4 mr-1" />
                    取消
                  </Button>
                </div>
              </div>
            </div>
          )}

          {/* 组合列表 */}
          <div className="space-y-2">
            {portfolios.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground text-sm">
                暂无投资组合，点击"新建"创建第一个组合
              </div>
            ) : (
              portfolios.map((portfolio) => (
                <div
                  key={portfolio.id}
                  className={`p-3 border rounded-lg cursor-pointer transition-colors hover:bg-muted/50 ${
                    selectedPortfolio?.id === portfolio.id ? 'bg-muted border-primary' : ''
                  }`}
                  onClick={() => handleSelectPortfolio(portfolio)}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="font-medium">{portfolio.name}</div>
                      <div className="text-xs text-muted-foreground mt-1">
                        {portfolio.holdings_num ?? portfolio.stocks.length} 只股票
                      </div>
                      {portfolio.description && (
                        <div className="text-xs text-muted-foreground mt-1 line-clamp-2">
                          {portfolio.description}
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </CardContent>
      </Card>

      {/* 右侧：组合详情 */}
      <Card className="lg:col-span-2">
        <CardHeader>
          {selectedPortfolio ? (
            <>
              <div className="flex items-center justify-between">
                <CardTitle>{selectedPortfolio.name}</CardTitle>
                <div className="flex gap-2">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={startEditingPortfolio}
                    disabled={isEditing || isLoadingPortfolio}
                  >
                    <Edit2 className="h-4 w-4 mr-1" />
                    编辑
                  </Button>
                  <Button
                    size="sm"
                    variant="destructive"
                    onClick={() => handleDeletePortfolio(selectedPortfolio.id, selectedPortfolio.name)}
                    disabled={isLoadingPortfolio}
                  >
                    <Trash2 className="h-4 w-4 mr-1" />
                    删除
                  </Button>
                </div>
              </div>
              <CardDescription>
                创建于 {formatDate(selectedPortfolio.created_date)} | 
                最后更新 {formatDate(selectedPortfolio.updated_date)}
              </CardDescription>
              {selectedPortfolio.description && (
                <p className="text-sm text-muted-foreground mt-2">
                  {selectedPortfolio.description}
                </p>
              )}
            </>
          ) : (
            <>
              <CardTitle>投资组合详情</CardTitle>
              <CardDescription>请从左侧选择一个投资组合</CardDescription>
            </>
          )}
        </CardHeader>
        <CardContent>
          {!selectedPortfolio ? (
            <div className="text-center py-12 text-muted-foreground">
              <FolderOpen className="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p>请从左侧选择或创建一个投资组合</p>
            </div>
          ) : isLoadingPortfolio ? (
            <div className="py-12 text-center text-muted-foreground">
              加载中...
            </div>
          ) : isEditing ? (
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium">组合名称</label>
                <Input
                  value={newPortfolioName}
                  onChange={(e) => setNewPortfolioName(e.target.value)}
                />
              </div>
              <div>
                <label className="text-sm font-medium">描述</label>
                <Textarea
                  value={newPortfolioDesc}
                  onChange={(e) => setNewPortfolioDesc(e.target.value)}
                  rows={3}
                />
              </div>
              <div className="flex gap-2">
                <Button
                  onClick={handleUpdatePortfolio}
                  disabled={updateMutation.isPending}
                >
                  <Save className="h-4 w-4 mr-1" />
                  保存
                </Button>
                <Button
                  variant="outline"
                  onClick={cancelEditingPortfolio}
                >
                  <X className="h-4 w-4 mr-1" />
                  取消
                </Button>
              </div>
            </div>
          ) : (
            <>
              {/* 添加股票按钮 */}
              <div className="mb-4">
                <Button
                  onClick={() => setIsAddingStock(true)}
                  disabled={isAddingStock || isLoadingPortfolio}
                >
                  <Plus className="h-4 w-4 mr-1" />
                  添加股票
                </Button>
              </div>

              {/* 添加股票表单 */}
              {isAddingStock && (
                <div className="mb-4 p-4 border rounded-lg bg-muted/50">
                  <div className="space-y-3">
                    <div>
                      <label className="text-sm font-medium">股票搜索</label>
                      <div className="relative" ref={searchDropdownRef}>
                        <div className="flex items-center gap-2">
                          <div className="relative flex-1">
                            <Search className="h-4 w-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
                            <Input
                              placeholder="搜索A股/美股：代码/名称"
                              value={searchKeyword}
                              onChange={(e) => {
                                const v = e.target.value
                                setSearchKeyword(v)
                                searchStocks(v)
                              }}
                              onFocus={() => {
                                if (searchResults.length > 0) setShowSearchResults(true)
                              }}
                              className="pl-9"
                              autoComplete="off"
                            />
                          </div>
                          {searchKeyword && (
                            <Button
                              type="button"
                              size="sm"
                              variant="outline"
                              onClick={() => {
                                setSearchKeyword('')
                                setSearchResults([])
                                setShowSearchResults(false)
                                setNewStock(prev => ({ ...prev, symbol: '', exchange_id: '' }))
                              }}
                            >
                              <X className="h-4 w-4" />
                            </Button>
                          )}
                        </div>

                        {showSearchResults && searchResults.length > 0 && (
                          <div className="absolute top-full left-0 right-0 mt-1 border rounded-md bg-background shadow-lg z-10 max-h-64 overflow-y-auto">
                            {searchResults.map((r) => (
                              <button
                                key={`${r.market}-${r.symbol}-${r.exchange_id || ''}`}
                                type="button"
                                onClick={() => handlePickStock(r)}
                                className="w-full px-3 py-2 text-left hover:bg-muted transition-colors flex items-center justify-between gap-3"
                              >
                                <div className="min-w-0">
                                  <div className="text-sm font-medium truncate">{r.symbol} {r.name}</div>
                                  <div className="text-xs text-muted-foreground">
                                    {r.market === 'us' ? `美股 ${r.exchange_id || ''}` : 'A股'}
                                  </div>
                                </div>
                                <div className="text-xs text-muted-foreground whitespace-nowrap">
                                  选择
                                </div>
                              </button>
                            ))}
                          </div>
                        )}
                      </div>
                      <p className="text-xs text-muted-foreground mt-1">
                        股票名称将自动从后端获取
                      </p>
                    </div>
                    <div>
                      <label className="text-sm font-medium flex items-center gap-2">
                        <Tag className="h-4 w-4" />
                        标签（可选）
                      </label>
                      <div className="relative" ref={tagDropdownRef}>
                        <button
                          type="button"
                          onClick={() => setShowTagDropdown(!showTagDropdown)}
                          className="w-full px-3 py-2 border rounded-md text-sm text-left bg-background hover:bg-muted transition-colors"
                        >
                          {selectedTags.length === 0 ? (
                            <span className="text-muted-foreground">选择标签...</span>
                          ) : (
                            <div className="flex flex-wrap gap-1">
                              {selectedTags.map(tagId => {
                                const tag = MOCK_TAGS.find(t => t.id === tagId)
                                return tag ? (
                                  <span key={tagId} className={`px-2 py-1 rounded text-xs ${tag.color}`}>
                                    {tag.name}
                                  </span>
                                ) : null
                              })}
                            </div>
                          )}
                        </button>
                        {showTagDropdown && (
                          <div className="absolute top-full left-0 right-0 mt-1 border rounded-md bg-background shadow-lg z-10 p-2 max-h-48 overflow-y-auto">
                            <div className="space-y-2">
                              {MOCK_TAGS.map(tag => (
                                <label key={tag.id} className="flex items-center gap-2 cursor-pointer p-2 hover:bg-muted rounded transition-colors">
                                  <input
                                    type="checkbox"
                                    checked={selectedTags.includes(tag.id)}
                                    onChange={() => toggleTag(tag.id)}
                                    className="w-4 h-4 rounded"
                                  />
                                  <span className={`px-2 py-1 rounded text-xs ${tag.color}`}>
                                    {tag.name}
                                  </span>
                                </label>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>
                    </div>
                    <div>
                      <label className="text-sm font-medium">描述（可选）</label>
                      <Textarea
                        placeholder="添加描述信息..."
                        value={newStock.desc}
                        onChange={(e) => setNewStock({ ...newStock, desc: e.target.value })}
                        rows={2}
                      />
                    </div>
                    <div className="flex gap-2">
                      <Button
                        size="sm"
                        onClick={handleAddStock}
                        disabled={addStockMutation.isPending}
                      >
                        <Save className="h-4 w-4 mr-1" />
                        添加
                      </Button>
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={handleCancelAddStock}
                      >
                        <X className="h-4 w-4 mr-1" />
                        取消
                      </Button>
                    </div>
                  </div>
                </div>
              )}

              {/* 成分股列表 */}
              {selectedPortfolio.stocks.length === 0 ? (
                <div className="text-center py-8 text-muted-foreground">
                  <p>暂无成分股，点击"添加股票"开始构建组合</p>
                </div>
              ) : (
                <div className="rounded-md border overflow-x-auto">
                  <Table className="min-w-[1200px]">
                    <TableHeader>
                      <TableRow>
                        <TableHead className="w-[120px]">股票代码</TableHead>
                        <TableHead className="w-[300px]">股票名称</TableHead>
                        <TableHead className="w-[110px] text-right">当前价</TableHead>
                        <TableHead className="w-[110px] text-right">涨跌幅</TableHead>
                        <TableHead className="w-[110px] text-right">5日涨幅</TableHead>
                        <TableHead className="w-[110px] text-right">10日涨幅</TableHead>
                        <TableHead className="w-[110px] text-right">20日涨幅</TableHead>
                        <TableHead className="w-[110px] text-right">60日涨幅</TableHead>
                        <TableHead className="w-[120px]">添加日期</TableHead>
                        <TableHead className="w-[200px]">标签</TableHead>
                        <TableHead>描述</TableHead>
                        <TableHead className="w-[100px] text-right">操作</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {selectedPortfolio.stocks.map((stock) => (
                        <TableRow key={stock.id}>
                          <TableCell className="font-mono font-medium">
                            {stock.symbol}
                          </TableCell>
                          <TableCell>{stock.name}</TableCell>
                          <TableCell className="text-right">{renderPriceCell((stock as any).current_price)}</TableCell>
                          <TableCell className="text-right">{renderPctCell((stock as any).pct_chg)}</TableCell>
                          <TableCell className="text-right">{renderPctCell((stock as any).pct5)}</TableCell>
                          <TableCell className="text-right">{renderPctCell((stock as any).pct10)}</TableCell>
                          <TableCell className="text-right">{renderPctCell((stock as any).pct20)}</TableCell>
                          <TableCell className="text-right">{renderPctCell((stock as any).pct60)}</TableCell>
                          <TableCell>
                            <span className="text-sm text-muted-foreground">
                              {formatDate(stock.added_date)}
                            </span>
                          </TableCell>
                          <TableCell>
                            <div className="flex flex-wrap gap-1">
                              {(stock as any).tags && (stock as any).tags.length > 0 ? (
                                (stock as any).tags.map((tagId: string) => {
                                  const tag = MOCK_TAGS.find(t => t.id === tagId)
                                  return tag ? (
                                    <span key={tagId} className={`px-2 py-1 rounded text-xs ${tag.color}`}>
                                      {tag.name}
                                    </span>
                                  ) : null
                                })
                              ) : (
                                <span className="text-xs text-muted-foreground">暂无标签</span>
                              )}
                            </div>
                          </TableCell>
                          <TableCell>
                            {editingStockId === stock.id ? (
                              <div className="flex items-center gap-2">
                                <Input
                                  value={editingDesc}
                                  onChange={(e) => setEditingDesc(e.target.value)}
                                  className="h-8 text-sm"
                                  placeholder="输入描述..."
                                  autoFocus
                                />
                                <Button
                                  size="sm"
                                  variant="ghost"
                                  onClick={handleUpdateStockDesc}
                                  disabled={updateStockMutation.isPending}
                                >
                                  <Save className="h-4 w-4 text-green-600" />
                                </Button>
                                <Button
                                  size="sm"
                                  variant="ghost"
                                  onClick={cancelEditingStock}
                                >
                                  <X className="h-4 w-4" />
                                </Button>
                              </div>
                            ) : (
                              <div 
                                className="text-sm text-muted-foreground cursor-pointer hover:text-foreground transition-colors"
                                onClick={() => startEditingStock(stock)}
                              >
                                {stock.desc || '点击添加描述...'}
                              </div>
                            )}
                          </TableCell>
                          <TableCell className="text-right">
                            <Button
                              size="sm"
                              variant="ghost"
                              onClick={() => handleRemoveStock(stock.id, stock.name)}
                            >
                              <Trash2 className="h-4 w-4 text-destructive" />
                            </Button>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
