'use client'

import React, { createContext, useContext, useState, useCallback } from 'react'
import { AlertTriangle, CheckCircle, Info, X } from 'lucide-react'

type ToastType = 'success' | 'error' | 'info' | 'warning'

interface Toast {
  id: string
  message: string
  type: ToastType
  onConfirm?: () => void
  onCancel?: () => void
  confirmText?: string
  cancelText?: string
}

interface ToastContextType {
  showToast: (message: string, type?: ToastType) => void
  showConfirm: (message: string, onConfirm: () => void, onCancel?: () => void) => void
}

const ToastContext = createContext<ToastContextType | undefined>(undefined)

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([])

  const showToast = useCallback((message: string, type: ToastType = 'info') => {
    const id = Math.random().toString(36).substr(2, 9)
    setToasts((prev) => [...prev, { id, message, type }])

    // 自动移除
    setTimeout(() => {
      setToasts((prev) => prev.filter((toast) => toast.id !== id))
    }, 5000)
  }, [])

  const showConfirm = useCallback((message: string, onConfirm: () => void, onCancel?: () => void) => {
    const id = Math.random().toString(36).substr(2, 9)
    setToasts((prev) => [...prev, { 
      id, 
      message, 
      type: 'warning',
      onConfirm,
      onCancel,
      confirmText: '确定',
      cancelText: '取消'
    }])
  }, [])

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id))
  }, [])

  const getIcon = (type: ToastType) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="h-5 w-5 text-green-600" />
      case 'error':
        return <AlertTriangle className="h-5 w-5 text-red-600" />
      case 'warning':
        return <AlertTriangle className="h-5 w-5 text-orange-600" />
      default:
        return <Info className="h-5 w-5 text-blue-600" />
    }
  }

  const getStyles = (type: ToastType) => {
    switch (type) {
      case 'success':
        return 'bg-green-50 border-green-200'
      case 'error':
        return 'bg-red-50 border-red-200'
      case 'warning':
        return 'bg-orange-50 border-orange-200'
      default:
        return 'bg-blue-50 border-blue-200'
    }
  }

  return (
    <ToastContext.Provider value={{ showToast, showConfirm }}>
      {children}
      
      {/* Toast容器 */}
      <div className="fixed top-20 left-1/2 -translate-x-1/2 z-50 flex flex-col gap-2 max-w-md w-full px-4">
        {toasts.map((toast) => (
          <div
            key={toast.id}
            className={`flex items-start gap-3 p-4 rounded-lg border shadow-lg animate-in slide-in-from-top-5 ${getStyles(toast.type)}`}
          >
            <div className="flex-shrink-0 mt-0.5">
              {getIcon(toast.type)}
            </div>
            <div className="flex-1">
              <div className="text-sm text-gray-800 mb-3">
                {toast.message}
              </div>
              {toast.onConfirm && (
                <div className="flex gap-2">
                  <button
                    onClick={() => {
                      toast.onConfirm?.()
                      removeToast(toast.id)
                    }}
                    className="px-3 py-1.5 text-sm bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
                  >
                    {toast.confirmText || '确定'}
                  </button>
                  <button
                    onClick={() => {
                      toast.onCancel?.()
                      removeToast(toast.id)
                    }}
                    className="px-3 py-1.5 text-sm bg-gray-200 text-gray-800 rounded hover:bg-gray-300 transition-colors"
                  >
                    {toast.cancelText || '取消'}
                  </button>
                </div>
              )}
            </div>
            {!toast.onConfirm && (
              <button
                onClick={() => removeToast(toast.id)}
                className="flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors"
              >
                <X className="h-4 w-4" />
              </button>
            )}
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  )
}

export function useToast() {
  const context = useContext(ToastContext)
  if (!context) {
    throw new Error('useToast must be used within ToastProvider')
  }
  return context
}
