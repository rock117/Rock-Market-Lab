<template>
  <div class="task-scheduler">
    <el-card class="header-card">
      <div class="header-content">
        <h2>定时任务管理</h2>
        <el-button type="primary" @click="showCreateDialog = true">
          <el-icon><Plus /></el-icon>
          创建任务
        </el-button>
      </div>
    </el-card>

    <!-- 筛选和搜索 -->
    <el-card class="filter-card">
      <el-form :model="filterForm" inline>
        <el-form-item label="任务类型">
          <el-select v-model="filterForm.taskType" placeholder="全部类型" clearable>
            <el-option label="全部" value="" />
            <el-option label="HTTP 请求" value="http_request" />
            <el-option label="Shell 命令" value="shell_command" />
            <el-option label="Rust 函数" value="rust_function" />
          </el-select>
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="filterForm.status" placeholder="全部状态" clearable>
            <el-option label="全部" value="" />
            <el-option label="启用" value="enabled" />
            <el-option label="暂停" value="paused" />
            <el-option label="禁用" value="disabled" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="loadTasks">查询</el-button>
          <el-button @click="resetFilter">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 任务列表 -->
    <el-card class="table-card">
      <el-table 
        :data="tasks" 
        v-loading="loading"
        stripe
        style="width: 100%"
      >
        <el-table-column prop="name" label="任务名称" min-width="150" />
        <el-table-column prop="task_type" label="类型" width="120">
          <template #default="{ row }">
            <el-tag :type="getTaskTypeColor(row.task_type)">
              {{ getTaskTypeName(row.task_type) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="schedule_type" label="调度类型" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ getScheduleTypeName(row.schedule_type) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusColor(row.status)">
              {{ getStatusName(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="next_run_time" label="下次执行" width="180">
          <template #default="{ row }">
            <span v-if="row.next_run_time">
              {{ formatDateTime(row.next_run_time) }}
            </span>
            <span v-else class="text-gray-400">-</span>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            {{ formatDateTime(row.created_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button-group size="small">
              <el-button @click="runTask(row)" :disabled="row.status !== 'enabled'">
                <el-icon><VideoPlay /></el-icon>
              </el-button>
              <el-button @click="editTask(row)">
                <el-icon><Edit /></el-icon>
              </el-button>
              <el-button 
                v-if="row.status === 'enabled'" 
                @click="pauseTask(row)"
                type="warning"
              >
                <el-icon><VideoPause /></el-icon>
              </el-button>
              <el-button 
                v-else-if="row.status === 'paused'" 
                @click="resumeTask(row)"
                type="success"
              >
                <el-icon><VideoPlay /></el-icon>
              </el-button>
              <el-button @click="deleteTask(row)" type="danger">
                <el-icon><Delete /></el-icon>
              </el-button>
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="pagination.page"
          v-model:page-size="pagination.pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="pagination.total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadTasks"
          @current-change="loadTasks"
        />
      </div>
    </el-card>

    <!-- 创建/编辑任务对话框 -->
    <TaskDialog 
      v-model="showCreateDialog"
      :task="currentTask"
      @saved="onTaskSaved"
    />

    <!-- 执行记录对话框 -->
    <ExecutionDialog 
      v-model="showExecutionDialog"
      :task-id="currentTaskId"
    />
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Edit, Delete, VideoPlay, VideoPause } from '@element-plus/icons-vue'
import TaskDialog from './components/TaskDialog.vue'
import ExecutionDialog from './components/ExecutionDialog.vue'
import { taskApi } from '@/api/task'

// 响应式数据
const loading = ref(false)
const tasks = ref([])
const showCreateDialog = ref(false)
const showExecutionDialog = ref(false)
const currentTask = ref(null)
const currentTaskId = ref(null)

// 筛选表单
const filterForm = reactive({
  taskType: '',
  status: ''
})

// 分页
const pagination = reactive({
  page: 1,
  pageSize: 20,
  total: 0
})

// 任务类型映射
const taskTypeMap = {
  'http_request': 'HTTP 请求',
  'shell_command': 'Shell 命令', 
  'rust_function': 'Rust 函数'
}

const taskTypeColorMap = {
  'http_request': 'primary',
  'shell_command': 'success',
  'rust_function': 'warning'
}

// 调度类型映射
const scheduleTypeMap = {
  'cron': 'Cron',
  'interval': '间隔',
  'once': '一次'
}

// 状态映射
const statusMap = {
  'enabled': '启用',
  'paused': '暂停',
  'disabled': '禁用'
}

const statusColorMap = {
  'enabled': 'success',
  'paused': 'warning',
  'disabled': 'danger'
}

// 方法
const getTaskTypeName = (type) => taskTypeMap[type] || type
const getTaskTypeColor = (type) => taskTypeColorMap[type] || 'info'
const getScheduleTypeName = (type) => scheduleTypeMap[type] || type
const getStatusName = (status) => statusMap[status] || status
const getStatusColor = (status) => statusColorMap[status] || 'info'

const formatDateTime = (dateTime) => {
  if (!dateTime) return '-'
  return new Date(dateTime).toLocaleString('zh-CN')
}

// 加载任务列表
const loadTasks = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page - 1, // API 使用 0 索引
      page_size: pagination.pageSize,
      task_type: filterForm.taskType || undefined,
      status: filterForm.status || undefined
    }
    
    const response = await taskApi.getTasks(params)
    tasks.value = response.tasks
    pagination.total = response.total
  } catch (error) {
    ElMessage.error('加载任务列表失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

// 重置筛选
const resetFilter = () => {
  filterForm.taskType = ''
  filterForm.status = ''
  pagination.page = 1
  loadTasks()
}

// 创建任务
const createTask = () => {
  currentTask.value = null
  showCreateDialog.value = true
}

// 编辑任务
const editTask = (task) => {
  currentTask.value = { ...task }
  showCreateDialog.value = true
}

// 运行任务
const runTask = async (task) => {
  try {
    await ElMessageBox.confirm(`确定要立即执行任务 "${task.name}" 吗？`, '确认执行', {
      type: 'warning'
    })
    
    await taskApi.runTask(task.id)
    ElMessage.success('任务已开始执行')
    
    // 显示执行记录
    currentTaskId.value = task.id
    showExecutionDialog.value = true
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('执行任务失败: ' + error.message)
    }
  }
}

// 暂停任务
const pauseTask = async (task) => {
  try {
    await taskApi.pauseTask(task.id)
    ElMessage.success('任务已暂停')
    loadTasks()
  } catch (error) {
    ElMessage.error('暂停任务失败: ' + error.message)
  }
}

// 恢复任务
const resumeTask = async (task) => {
  try {
    await taskApi.resumeTask(task.id)
    ElMessage.success('任务已恢复')
    loadTasks()
  } catch (error) {
    ElMessage.error('恢复任务失败: ' + error.message)
  }
}

// 删除任务
const deleteTask = async (task) => {
  try {
    await ElMessageBox.confirm(`确定要删除任务 "${task.name}" 吗？此操作不可恢复！`, '确认删除', {
      type: 'warning'
    })
    
    await taskApi.deleteTask(task.id)
    ElMessage.success('任务已删除')
    loadTasks()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除任务失败: ' + error.message)
    }
  }
}

// 任务保存回调
const onTaskSaved = () => {
  showCreateDialog.value = false
  loadTasks()
}

// 初始化
onMounted(() => {
  loadTasks()
})
</script>

<style scoped>
.task-scheduler {
  padding: 20px;
}

.header-card {
  margin-bottom: 20px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-content h2 {
  margin: 0;
  color: #303133;
}

.filter-card {
  margin-bottom: 20px;
}

.table-card {
  margin-bottom: 20px;
}

.pagination-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.text-gray-400 {
  color: #9ca3af;
}

:deep(.el-button-group .el-button) {
  margin: 0;
}
</style>
