'use client'

import React, { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { portfolioApi } from '@/services/api'
import { Portfolio, PortfolioStock } from '@/types'
import { formatDate } from '@/lib/utils'
import { Plus, Trash2, Edit2, FolderOpen, X, Search, Save } from 'lucide-react'
import { useToast } from '@/components/ui/toast'

interface PortfolioManagerProps {
  className?: string
}

export default function PortfolioManager({ className }: PortfolioManagerProps) {
  const [selectedPortfolio, setSelectedPortfolio] = useState<Portfolio | null>(null)
  const [isCreating, setIsCreating] = useState(false)
  const [isEditing, setIsEditing] = useState(false)
  const [newPortfolioName, setNewPortfolioName] = useState('')
  const [newPortfolioDesc, setNewPortfolioDesc] = useState('')
  
  // 添加股票相关状态
  const [isAddingStock, setIsAddingStock] = useState(false)
  const [searchKeyword, setSearchKeyword] = useState('')
  const [newStock, setNewStock] = useState({
    symbol: '',
    name: '',
    exchange_id: '',
    desc: ''
  })

  const queryClient = useQueryClient()
  const { showToast } = useToast()

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
      setNewStock({ symbol: '', name: '', exchange_id: '', desc: '' })
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

  const handleDeletePortfolio = (id: string) => {
    if (confirm('确定要删除这个投资组合吗？')) {
      deleteMutation.mutate(id)
    }
  }

  const handleAddStock = () => {
    if (!selectedPortfolio) return
    if (!newStock.symbol.trim() || !newStock.name.trim()) {
      showToast('请输入股票代码和名称', 'warning')
      return
    }
    addStockMutation.mutate({
      portfolioId: selectedPortfolio.id,
      stock: {
        symbol: newStock.symbol,
        name: newStock.name,
        exchange_id: newStock.exchange_id || undefined,
        desc: newStock.desc || undefined
      }
    })
  }

  const handleRemoveStock = (stockId: string) => {
    if (!selectedPortfolio) return
    if (confirm('确定要从组合中移除这只股票吗？')) {
      removeStockMutation.mutate({
        portfolioId: selectedPortfolio.id,
        stockId
      })
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
                  onClick={() => setSelectedPortfolio(portfolio)}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1">
                      <div className="font-medium">{portfolio.name}</div>
                      <div className="text-xs text-muted-foreground mt-1">
                        {portfolio.stocks.length} 只股票
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
                    disabled={isEditing}
                  >
                    <Edit2 className="h-4 w-4 mr-1" />
                    编辑
                  </Button>
                  <Button
                    size="sm"
                    variant="destructive"
                    onClick={() => handleDeletePortfolio(selectedPortfolio.id)}
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
                  disabled={isAddingStock}
                >
                  <Plus className="h-4 w-4 mr-1" />
                  添加股票
                </Button>
              </div>

              {/* 添加股票表单 */}
              {isAddingStock && (
                <div className="mb-4 p-4 border rounded-lg bg-muted/50">
                  <div className="space-y-3">
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="text-sm font-medium">股票代码</label>
                        <Input
                          placeholder="例如：000001.SZ"
                          value={newStock.symbol}
                          onChange={(e) => setNewStock({ ...newStock, symbol: e.target.value })}
                        />
                      </div>
                      <div>
                        <label className="text-sm font-medium">股票名称</label>
                        <Input
                          placeholder="例如：平安银行"
                          value={newStock.name}
                          onChange={(e) => setNewStock({ ...newStock, name: e.target.value })}
                        />
                      </div>
                    </div>
                    <div>
                      <label className="text-sm font-medium">交易所ID（可选）</label>
                      <Input
                        placeholder="例如：SZ"
                        value={newStock.exchange_id}
                        onChange={(e) => setNewStock({ ...newStock, exchange_id: e.target.value })}
                      />
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
                        onClick={() => {
                          setIsAddingStock(false)
                          setNewStock({ symbol: '', name: '', exchange_id: '', desc: '' })
                        }}
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
                <div className="rounded-md border">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead className="w-[120px]">股票代码</TableHead>
                        <TableHead className="w-[150px]">股票名称</TableHead>
                        <TableHead className="w-[100px]">交易所ID</TableHead>
                        <TableHead className="w-[120px]">添加日期</TableHead>
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
                          <TableCell>
                            <span className="text-sm text-muted-foreground">
                              {stock.exchange_id || 'N/A'}
                            </span>
                          </TableCell>
                          <TableCell>
                            <span className="text-sm text-muted-foreground">
                              {formatDate(stock.added_date)}
                            </span>
                          </TableCell>
                          <TableCell>
                            <span className="text-sm text-muted-foreground">
                              {stock.desc || 'N/A'}
                            </span>
                          </TableCell>
                          <TableCell className="text-right">
                            <Button
                              size="sm"
                              variant="ghost"
                              onClick={() => handleRemoveStock(stock.id)}
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
