'use client'

import React, { useState, useRef, useEffect } from 'react'

interface TooltipProps {
  content: string
  children: React.ReactNode
  className?: string
}

export function Tooltip({ content, children, className = '' }: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false)
  const [position, setPosition] = useState({ top: 0, left: 0 })
  const triggerRef = useRef<HTMLDivElement>(null)
  const tooltipRef = useRef<HTMLDivElement>(null)

  const updatePosition = () => {
    if (triggerRef.current && tooltipRef.current) {
      const triggerRect = triggerRef.current.getBoundingClientRect()
      const tooltipRect = tooltipRef.current.getBoundingClientRect()
      
      // 计算tooltip位置（默认显示在上方，左对齐）
      let top = triggerRect.top - tooltipRect.height - 8
      let left = triggerRect.left // 左对齐

      // 如果上方空间不足，显示在下方
      if (top < 10) {
        top = triggerRect.bottom + 8
      }

      // 确保不超出右边界
      if (left + tooltipRect.width > window.innerWidth - 10) {
        left = window.innerWidth - tooltipRect.width - 10
      }
      
      // 确保不超出左边界
      if (left < 10) {
        left = 10
      }

      setPosition({ top, left })
    }
  }

  useEffect(() => {
    if (isVisible) {
      updatePosition()
      window.addEventListener('scroll', updatePosition)
      window.addEventListener('resize', updatePosition)
      return () => {
        window.removeEventListener('scroll', updatePosition)
        window.removeEventListener('resize', updatePosition)
      }
    }
  }, [isVisible])

  return (
    <>
      <div
        ref={triggerRef}
        onMouseEnter={() => setIsVisible(true)}
        onMouseLeave={() => setIsVisible(false)}
        className={className}
      >
        {children}
      </div>

      {isVisible && content && (
        <div
          ref={tooltipRef}
          className="fixed z-50 px-4 py-3 text-sm text-gray-800 bg-white border border-gray-200 rounded-lg shadow-xl max-w-md animate-in fade-in-0 zoom-in-95 select-text cursor-text"
          style={{
            top: `${position.top}px`,
            left: `${position.left}px`,
          }}
          onMouseEnter={() => setIsVisible(true)}
          onMouseLeave={() => setIsVisible(false)}
        >
          <div className="whitespace-pre-wrap break-words">{content}</div>
          <div className="absolute w-2 h-2 bg-white border-l border-b border-gray-200 transform rotate-45 -bottom-1 left-6"></div>
        </div>
      )}
    </>
  )
}
