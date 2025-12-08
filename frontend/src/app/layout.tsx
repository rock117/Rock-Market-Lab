import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { QueryProvider } from '@/providers/QueryProvider'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Rock Market Lab - 股票分析平台',
  description: '专业的股票数据分析和投资决策平台',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="zh-CN">
      <body className={inter.className}>
        <QueryProvider>
          <div className="min-h-screen bg-background">
            <header className="border-b bg-card">
              <div className="container mx-auto px-4 py-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                      <span className="text-primary-foreground font-bold text-sm">R</span>
                    </div>
                    <h1 className="text-xl font-bold">Rock Market Lab</h1>
                  </div>
                  <nav className="flex items-center gap-6">
                    <a href="#market" className="text-sm font-medium hover:text-primary">
                      A股大盘
                    </a>
                    <a href="#us-stocks" className="text-sm font-medium hover:text-primary">
                      美股市场
                    </a>
                    <a href="#analysis" className="text-sm font-medium hover:text-primary">
                      技术分析
                    </a>
                    <a href="#portfolio" className="text-sm font-medium hover:text-primary">
                      投资组合
                    </a>
                  </nav>
                </div>
              </div>
            </header>
            <main className="container mx-auto px-4 py-6">
              {children}
            </main>
            <footer className="border-t bg-card mt-12">
              <div className="container mx-auto px-4 py-6">
                <div className="text-center text-sm text-muted-foreground">
                  © 2024 Rock Market Lab. 专业股票分析平台
                </div>
              </div>
            </footer>
          </div>
        </QueryProvider>
      </body>
    </html>
  )
}
