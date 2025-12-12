import PortfolioManager from '@/components/portfolio/PortfolioManager'

export default function PortfolioPage() {
  return (
    <div className="container mx-auto py-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">投资组合</h1>
        <p className="text-muted-foreground mt-2">
          创建和管理您的投资组合，跟踪持仓股票
        </p>
      </div>
      <PortfolioManager />
    </div>
  )
}
