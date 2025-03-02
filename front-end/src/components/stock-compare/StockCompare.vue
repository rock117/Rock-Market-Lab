<template>
  <div class="stock-price-chart">
    <!-- 控制区域 -->
    <div class="controls">
      <el-date-picker
        v-model="dateRange"
        type="daterange"
        range-separator="至"
        start-placeholder="开始日期"
        end-placeholder="结束日期"
        @change="handleDateChange"
        :size="size"
      />
      
      <el-select
        v-model="selectedStocks"
        multiple
        filterable
        clearable
        collapse-tags
        :max-collapse-tags="1"
        collapse-tags-tooltip
        placeholder="请选择股票"
        style="width: 300px; margin-left: 16px"
        :size="size"
      >
        <el-option
          v-for="stock in stockList"
          :key="stock.tsCode"
          :label="`${stock.name} (${stock.tsCode})`"
          :value="stock.tsCode"
        />
      </el-select>

      <el-radio-group v-model="displayMode" style="margin-left: 16px" :size="size">
        <el-radio-button label="table">表格模式</el-radio-button>
        <el-radio-button label="chart">图表模式</el-radio-button>
      </el-radio-group>
    </div>

    <!-- 表格视图 -->
    <div v-if="displayMode === 'table'" class="table-container">
      <el-table 
        :data="stockData" 
        style="width: 100%" 
        border
        :size="size"
      >
        <el-table-column prop="date" label="日期" width="120" sortable />
        <el-table-column prop="tsCode" label="股票代码" width="120" />
        <el-table-column prop="name" label="股票名称" width="120" />
        <el-table-column prop="price" label="价格" width="100" sortable>
          <template #default="scope">
            {{ scope.row.price.toFixed(2) }}
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 图表视图 -->
    <div v-else class="chart-container">
      <div ref="chartRef" style="width: 100%; height: 600px"></div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, nextTick, onUnmounted } from 'vue'
import * as echarts from 'echarts'

const dateRange = ref([])
const selectedStocks = ref([])
const displayMode = ref('table')
const chartRef = ref(null)
const size = ref('default')
let chart = null

// 模拟数据，实际使用时需要替换为真实的API调用
const stockList = ref([
  { tsCode: '600001.SH', name: '浦发银行', price: 14.3, date: '2020-05-06' },
  { tsCode: '600002.SH', name: '齐鲁石化', price: 18.5, date: '2020-05-06' },
  { tsCode: '600003.SH', name: '上海机电', price: 22.1, date: '2020-05-06' },
])

const stockData = ref([])

// 初始化图表
const initChart = () => {
  if (chart) {
    chart.dispose()
  }
  chart = echarts.init(chartRef.value)
}

// 更新图表数据
const updateChart = () => {
  if (!chart || !stockData.value.length) return

  const series = selectedStocks.value.map(tsCode => {
    const stockInfo = stockList.value.find(s => s.tsCode === tsCode)
    const data = stockData.value
      .filter(item => item.tsCode === tsCode)
      .map(item => [item.date, item.price])
    
    return {
      name: stockInfo.name,
      type: 'line',
      data: data,
      showSymbol: false,
    }
  })

  const option = {
    title: {
      text: '股票价格走势'
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: selectedStocks.value.map(tsCode => 
        stockList.value.find(s => s.tsCode === tsCode).name
      )
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'time',
      boundaryGap: false
    },
    yAxis: {
      type: 'value',
      scale: true,
      splitLine: {
        show: true
      }
    },
    series
  }

  chart.setOption(option)
}

// 处理日期变化
const handleDateChange = () => {
  if (dateRange.value && selectedStocks.value.length > 0) {
    loadStockData()
  }
}

// 加载股票数据
const loadStockData = async () => {
  if (!dateRange.value || !selectedStocks.value.length) {
    stockData.value = []
    return
  }

  // 这里需要实现从后端API获取数据的逻辑
  // 临时使用模拟数据
  const [startDate, endDate] = dateRange.value
  
  // 生成模拟数据
  const mockData = []
  const currentDate = new Date(startDate)
  const basePrice = {
    '600001.SH': 14.3,
    '600002.SH': 18.5,
    '600003.SH': 22.1
  }
  
  while (currentDate <= endDate) {
    selectedStocks.value.forEach(tsCode => {
      const stock = stockList.value.find(s => s.tsCode === tsCode)
      const baseStockPrice = basePrice[tsCode]
      // 生成一个基于基础价格上下10%范围内的随机价格
      const randomFactor = 0.9 + Math.random() * 0.2 // 0.9 到 1.1 之间的随机数
      const price = +(baseStockPrice * randomFactor).toFixed(2)
      
      mockData.push({
        tsCode,
        name: stock.name,
        price,
        date: currentDate.toISOString().split('T')[0]
      })
    })
    currentDate.setDate(currentDate.getDate() + 1)
  }
  
  stockData.value = mockData
  
  if (displayMode.value === 'chart') {
    nextTick(() => {
      updateChart()
    })
  }
}

// 监听选中股票和日期范围变化
watch([selectedStocks, dateRange], ([newStocks, newDateRange], [oldStocks, oldDateRange]) => {
  if (newStocks?.length > 0 && newDateRange) {
    loadStockData()
  }
}, { deep: true })

// 监听显示模式变化
watch(displayMode, (newMode) => {
  if (newMode === 'chart' && stockData.value.length > 0) {
    nextTick(() => {
      initChart()
      updateChart()
    })
  }
})

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
  if (chart) {
    chart.dispose()
    chart = null
  }
})

const handleResize = () => {
  if (chart) {
    chart.resize()
  }
}
</script>

<style scoped>
.stock-price-chart {
  padding: 20px;
}

.controls {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
}

.table-container {
  margin-top: 20px;
}

.chart-container {
  margin-top: 20px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 20px;
}
</style>
