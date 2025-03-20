<template>
  <div class="security-compare">
    <div class="control-panel">
      <div class="date-section">
        <el-date-picker v-model="startDate" type="date" placeholder="开始日期" format="YYYY-MM-DD" value-format="YYYY-MM-DD"
          :disabled-date="disableFutureDate" style="width: 180px" />
        <el-date-picker v-model="endDate" type="date" placeholder="结束日期" format="YYYY-MM-DD" value-format="YYYY-MM-DD"
          :disabled-date="disableFutureDate" style="width: 180px" />
      </div>

      <div class="security-section">
        <el-select v-model="security1" filterable remote :remote-method="(query) => handleSearch(query, '1')"
          :loading="searching" placeholder="选择证券1" style="width: 250px">
          <el-option v-for="item in security1Options" :key="item.ts_code" :label="`${item.ts_code} - ${item.name}`"
            :value="item.ts_code" />
        </el-select>


      </div>

      <el-select v-model="selectedPeriod" placeholder="选择时间间隔" style="width: 120px">
        <el-option v-for="period in periodOptions" :key="period.value" :label="period.label" :value="period.value" />
      </el-select>
    </div>

    <!-- <div class="chart-container">
      <div v-if="loading" class="loading-overlay">
        <el-loading :fullscreen="false" />
      </div>
      <div ref="chartContainer" style="width: 100%; height: 600px"></div>
    </div> -->

    <el-table :data="tableData" stripe style="width: 100%" v-loading="loading">
      <el-table-column prop="trade_date" label="日期" width="100" />
      <el-table-column sortable prop="close" label="收盘价" width="160" />
      <el-table-column sortable prop="low" label="最低" width="160" />
      <el-table-column sortable prop="high" label="最高" width="160" />
      <el-table-column sortable prop="pct_chg" label="涨跌幅%" width="160" />
      <el-table-column sortable prop="mkv" label="换手率" width="180" />
      <el-table-column sortable prop="amount" label="成交量(亿元)" width="180" />
    </el-table>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import * as echarts from 'echarts'
import axios from 'axios'
import { ElMessage } from 'element-plus'
import { getSecurityPrice } from '@/service/index.js'

// 状态变量
const startDate = ref('')
const endDate = ref('')
const security1 = ref('')
const selectedPeriod = ref('Day')
const searching = ref(false)
const loading = ref(false)
const security1Options = ref([])
const chartContainer = ref(null)
const tableData = ref([])
let chart = null

// 时间间隔选项
const periodOptions = [
  { value: 'Day', label: '日线' },
  { value: 'Week', label: '周线' },
  { value: 'Month', label: '月线' }
]

// 禁用未来日期
const disableFutureDate = (date) => {
  return date > new Date()
}

// 搜索证券
const handleSearch = async (query, type) => {
  console.log("handleSearch", query, type)
  if (!query) {
    return
  }

  searching.value = true
  try {
    const response = await axios.get(`/api/securities/search?keyword=${encodeURIComponent(query)}`)
    if (type == '1') {
      security1Options.value = response.data.data
    } else if (type == '2') {
      security2Options.value = response.data.data
    }
  } catch (error) {
    console.error('搜索证券失败:', error)
    ElMessage.error('搜索证券失败，请重试')
  } finally {
    searching.value = false
  }
}

// 初始化图表
const initChart = () => {
  if (chart) {
    chart.dispose()
  }
  chart = echarts.init(chartContainer.value)
}

// 更新图表数据
const updateChart = (data) => {
  if (!chart) return

  const { dates, prices1, name1 } = data
  const option = {
    title: {
      text: `${name1}`
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross'
      }
    },
    legend: {
      data: [name1]
    },
    grid: {
      left: '3%',
      right: '10%',
      bottom: '3%',
      containLabel: true
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: dates
    },
    yAxis: [
      {
        type: 'value',
        name: name1,
        position: 'left',
        axisLine: {
          show: true,
          lineStyle: {
            color: '#5470c6'
          }
        },
        axisLabel: {
          formatter: '{value} 元'
        }
      }

    ],
    series: [
      {
        name: name1,
        type: 'line',
        data: prices1,
        yAxisIndex: 0,
        itemStyle: {
          color: '#5470c6'
        }
      }

    ]
  }

  chart.setOption(option, true)
}

// 获取证券数据
const fetchSecurityData = async () => {
  if (!startDate.value || !endDate.value || !security1.value || !selectedPeriod.value) {
    return
  }

  loading.value = true
  try {
    const sec1 = security1Options.value.find(s => s.ts_code === security1.value)

    const [response1] = await Promise.all([
      getSecurityPrice(sec1.type, security1.value, startDate.value, endDate.value),
    ])
    response1.data.sort((a, b) => a.trade_date.localeCompare(b.trade_date))
    const dates = response1.data.map(item => item.trade_date)
    const prices1 = response1.data.map(item => item.close)
    updateTable(response1.data)
    // updateChart({
    //   dates,
    //   prices1,
    //   name1: sec1.name,
    // })
  } catch (error) {
    console.error('获取证券数据失败:', error)
    ElMessage.error('获取证券数据失败，请重试')
  } finally {
    loading.value = false
  }
}

const updateTable = (datas) => {
  datas = datas.map(data => {
    return {
      ...data,
      amount: data.amount/100000
    }
  });
  datas.sort((a, b) => b.trade_date.localeCompare(a.trade_date))
  tableData.value = datas
}

// 监听输入变化
watch(
  [startDate, endDate, security1, selectedPeriod],
  ([newStartDate, newEndDate, newSecurity1, newPeriod]) => {
    if (newStartDate && newEndDate && newSecurity1 && newPeriod) {
      fetchSecurityData()
    }
  }
)

// 监听窗口大小变化
window.addEventListener('resize', () => {
  chart?.resize()
})

onMounted(() => {
  // initChart()
})
</script>

<style scoped>
.security-compare {
  padding: 20px;
}

.control-panel {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.date-section,
.security-section {
  display: flex;
  gap: 10px;
}

.chart-container {
  position: relative;
  background: #fff;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 20px;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1;
}
</style>
