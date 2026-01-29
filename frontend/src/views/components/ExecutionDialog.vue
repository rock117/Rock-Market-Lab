<template>
  <el-dialog
    v-model="dialogVisible"
    title="执行记录"
    width="1000px"
    :before-close="handleClose"
  >
    <!-- 筛选器 -->
    <div class="filter-section">
      <el-form :model="filterForm" inline>
        <el-form-item label="状态">
          <el-select v-model="filterForm.status" placeholder="全部状态" clearable>
            <el-option label="全部" value="" />
            <el-option label="运行中" value="running" />
            <el-option label="成功" value="success" />
            <el-option label="失败" value="failed" />
            <el-option label="超时" value="timeout" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="loadExecutions">查询</el-button>
          <el-button @click="resetFilter">重置</el-button>
        </el-form-item>
      </el-form>
    </div>

    <!-- 执行记录表格 -->
    <el-table 
      :data="executions" 
      v-loading="loading"
      stripe
      style="width: 100%"
      max-height="400px"
    >
      <el-table-column prop="execution_id" label="执行ID" width="120">
        <template #default="{ row }">
          <el-text class="execution-id">{{ row.execution_id.substring(0, 8) }}...</el-text>
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="getStatusColor(row.status)" :icon="getStatusIcon(row.status)">
            {{ getStatusName(row.status) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="started_at" label="开始时间" width="180">
        <template #default="{ row }">
          {{ formatDateTime(row.started_at) }}
        </template>
      </el-table-column>
      <el-table-column prop="finished_at" label="结束时间" width="180">
        <template #default="{ row }">
          <span v-if="row.finished_at">{{ formatDateTime(row.finished_at) }}</span>
          <span v-else class="text-gray-400">-</span>
        </template>
      </el-table-column>
      <el-table-column prop="duration_ms" label="耗时" width="100">
        <template #default="{ row }">
          <span v-if="row.duration_ms">{{ formatDuration(row.duration_ms) }}</span>
          <span v-else class="text-gray-400">-</span>
        </template>
      </el-table-column>
      <el-table-column prop="retry_attempt" label="重试次数" width="100" />
      <el-table-column label="操作" width="120">
        <template #default="{ row }">
          <el-button size="small" @click="viewDetails(row)">
            <el-icon><View /></el-icon>
            详情
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 分页 -->
    <div class="pagination-container">
      <el-pagination
        v-model:current-page="pagination.page"
        v-model:page-size="pagination.pageSize"
        :page-sizes="[10, 20, 50]"
        :total="pagination.total"
        layout="total, sizes, prev, pager, next"
        @size-change="loadExecutions"
        @current-change="loadExecutions"
      />
    </div>

    <!-- 执行详情对话框 -->
    <el-dialog
      v-model="showDetailDialog"
      title="执行详情"
      width="800px"
      append-to-body
    >
      <div v-if="currentExecution">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="执行ID">
            {{ currentExecution.execution_id }}
          </el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="getStatusColor(currentExecution.status)">
              {{ getStatusName(currentExecution.status) }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="开始时间">
            {{ formatDateTime(currentExecution.started_at) }}
          </el-descriptions-item>
          <el-descriptions-item label="结束时间">
            <span v-if="currentExecution.finished_at">
              {{ formatDateTime(currentExecution.finished_at) }}
            </span>
            <span v-else>-</span>
          </el-descriptions-item>
          <el-descriptions-item label="执行耗时">
            <span v-if="currentExecution.duration_ms">
              {{ formatDuration(currentExecution.duration_ms) }}
            </span>
            <span v-else>-</span>
          </el-descriptions-item>
          <el-descriptions-item label="重试次数">
            {{ currentExecution.retry_attempt }}
          </el-descriptions-item>
        </el-descriptions>

        <!-- 错误信息 -->
        <div v-if="currentExecution.error_message" class="detail-section">
          <h4>错误信息</h4>
          <el-alert
            type="error"
            :title="currentExecution.error_message"
            show-icon
            :closable="false"
          />
        </div>

        <!-- 输出信息 -->
        <div v-if="currentExecution.output_summary" class="detail-section">
          <h4>输出信息</h4>
          <el-input
            v-model="currentExecution.output_summary"
            type="textarea"
            :rows="8"
            readonly
            class="output-text"
          />
        </div>
      </div>
    </el-dialog>
  </el-dialog>
</template>

<script setup>
import { ref, reactive, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { View, Loading, SuccessFilled, CircleCloseFilled, WarningFilled } from '@element-plus/icons-vue'
import { taskApi } from '@/api/task'

// Props & Emits
const props = defineProps({
  modelValue: Boolean,
  taskId: [Number, String]
})

const emit = defineEmits(['update:modelValue'])

// 响应式数据
const loading = ref(false)
const executions = ref([])
const showDetailDialog = ref(false)
const currentExecution = ref(null)

// 计算属性
const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

// 筛选表单
const filterForm = reactive({
  status: ''
})

// 分页
const pagination = reactive({
  page: 1,
  pageSize: 20,
  total: 0
})

// 状态映射
const statusMap = {
  'running': '运行中',
  'success': '成功',
  'failed': '失败',
  'timeout': '超时'
}

const statusColorMap = {
  'running': 'primary',
  'success': 'success',
  'failed': 'danger',
  'timeout': 'warning'
}

const statusIconMap = {
  'running': Loading,
  'success': SuccessFilled,
  'failed': CircleCloseFilled,
  'timeout': WarningFilled
}

// 方法
const getStatusName = (status) => statusMap[status] || status
const getStatusColor = (status) => statusColorMap[status] || 'info'
const getStatusIcon = (status) => statusIconMap[status]

const formatDateTime = (dateTime) => {
  if (!dateTime) return '-'
  return new Date(dateTime).toLocaleString('zh-CN')
}

const formatDuration = (ms) => {
  if (!ms) return '-'
  
  if (ms < 1000) {
    return `${ms}ms`
  } else if (ms < 60000) {
    return `${(ms / 1000).toFixed(1)}s`
  } else {
    const minutes = Math.floor(ms / 60000)
    const seconds = Math.floor((ms % 60000) / 1000)
    return `${minutes}m ${seconds}s`
  }
}

// 加载执行记录
const loadExecutions = async () => {
  if (!props.taskId) return
  
  loading.value = true
  try {
    const params = {
      page: pagination.page - 1,
      page_size: pagination.pageSize,
      task_id: props.taskId,
      status: filterForm.status || undefined
    }
    
    const response = await taskApi.getExecutions(params)
    executions.value = response.executions
    pagination.total = response.total
  } catch (error) {
    ElMessage.error('加载执行记录失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

// 重置筛选
const resetFilter = () => {
  filterForm.status = ''
  pagination.page = 1
  loadExecutions()
}

// 查看详情
const viewDetails = (execution) => {
  currentExecution.value = execution
  showDetailDialog.value = true
}

// 关闭对话框
const handleClose = () => {
  dialogVisible.value = false
  executions.value = []
  pagination.page = 1
  pagination.total = 0
}

// 监听器
watch(() => props.modelValue, (newVal) => {
  if (newVal && props.taskId) {
    loadExecutions()
    
    // 设置定时刷新（每5秒刷新一次运行中的记录）
    const refreshInterval = setInterval(() => {
      if (dialogVisible.value && executions.value.some(exec => exec.status === 'running')) {
        loadExecutions()
      } else {
        clearInterval(refreshInterval)
      }
    }, 5000)
  }
})
</script>

<style scoped>
.filter-section {
  margin-bottom: 20px;
  padding: 15px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.execution-id {
  font-family: 'Courier New', monospace;
  font-size: 12px;
}

.text-gray-400 {
  color: #9ca3af;
}

.detail-section {
  margin-top: 20px;
}

.detail-section h4 {
  margin: 0 0 10px 0;
  color: #303133;
  font-size: 14px;
  font-weight: 600;
}

.output-text {
  font-family: 'Courier New', monospace;
  font-size: 12px;
}

:deep(.output-text .el-textarea__inner) {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.4;
}
</style>
