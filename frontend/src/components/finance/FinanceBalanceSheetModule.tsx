'use client'

import React from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'

const mockRows = [
  { tsCode: '000001.SZ', name: '平安银行', endDate: '2024-12-31', totalAssets: 1000000000000, totalLiabilities: 950000000000 },
  { tsCode: '600519.SH', name: '贵州茅台', endDate: '2024-12-31', totalAssets: 320000000000, totalLiabilities: 60000000000 },
]

function toYi(n: number): string {
  return (n / 1e8).toFixed(4)
}

export default function FinanceBalanceSheetModule() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>资产负债表</CardTitle>
        <CardDescription>占位数据（后续接入真实数据源）</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="border rounded-md overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>股票代码</TableHead>
                <TableHead>股票名称</TableHead>
                <TableHead>日期</TableHead>
                <TableHead>总资产(亿)</TableHead>
                <TableHead>总负债(亿)</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {mockRows.map((r) => (
                <TableRow key={r.tsCode}>
                  <TableCell>{r.tsCode}</TableCell>
                  <TableCell>{r.name}</TableCell>
                  <TableCell>{r.endDate}</TableCell>
                  <TableCell>{toYi(r.totalAssets)}</TableCell>
                  <TableCell>{toYi(r.totalLiabilities)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  )
}
