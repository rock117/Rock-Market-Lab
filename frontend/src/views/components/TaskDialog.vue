<template>
  <el-dialog
    v-model="dialogVisible"
    :title="isEdit ? '编辑任务' : '创建任务'"
    width="800px"
    :before-close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="120px"
      v-loading="loading"
    >
      <!-- 基本信息 -->
      <el-card class="form-section">
        <template #header>
          <span>基本信息</span>
        </template>
        
        <el-form-item label="任务名称" prop="name">
          <el-input v-model="form.name" placeholder="请输入任务名称" />
        </el-form-item>
        
        <el-form-item label="描述" prop="description">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="2"
            placeholder="请输入任务描述（可选）"
          />
        </el-form-item>
        
        <el-form-item label="任务类型" prop="task_type">
          <el-select 
            v-model="form.task_type" 
            placeholder="请选择任务类型"
            @change="onTaskTypeChange"
            style="width: 100%"
          >
            <el-option
              v-for="type in taskTypes"
              :key="type.name"
              :label="type.description"
              :value="type.name"
            />
          </el-select>
        </el-form-item>
      </el-card>

      <!-- 调度配置 -->
      <el-card class="form-section">
        <template #header>
          <span>调度配置</span>
        </template>
        
        <el-form-item label="调度类型" prop="schedule_type">
          <el-radio-group v-model="form.schedule_type" @change="onScheduleTypeChange">
            <el-radio label="cron">Cron 表达式</el-radio>
            <el-radio label="interval">固定间隔</el-radio>
            <el-radio label="once">执行一次</el-radio>
          </el-radio-group>
        </el-form-item>
        
        <!-- Cron 配置 -->
        <el-form-item 
          v-if="form.schedule_type === 'cron'" 
          label="Cron 表达式" 
          prop="cron_expression"
        >
          <el-input 
            v-model="form.schedule_config.expression" 
            placeholder="例如: 0 0 12 * * ? (每天中午12点)"
          />
          <div class="form-help">
            <small>格式: 秒 分 时 日 月 周 年(可选)</small>
          </div>
        </el-form-item>
        
        <!-- 间隔配置 -->
        <el-form-item 
          v-if="form.schedule_type === 'interval'" 
          label="间隔时间" 
          prop="interval_seconds"
        >
          <el-input-number 
            v-model="form.schedule_config.seconds" 
            :min="1"
            :max="86400"
            placeholder="秒"
            style="width: 200px"
          />
          <span style="margin-left: 10px">秒</span>
        </el-form-item>
        
        <!-- 一次性执行配置 -->
        <el-form-item 
          v-if="form.schedule_type === 'once'" 
          label="执行时间" 
          prop="run_at"
        >
          <el-date-picker
            v-model="form.schedule_config.run_at"
            type="datetime"
            placeholder="选择执行时间"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DDTHH:mm:ss.000Z"
          />
        </el-form-item>
      </el-card>

      <!-- 任务配置 -->
      <el-card class="form-section" v-if="currentTaskTypeSchema">
        <template #header>
          <span>任务配置</span>
        </template>
        
        <div v-for="field in currentTaskTypeSchema.fields" :key="field.name">
          <!-- 字符串输入 -->
          <el-form-item 
            v-if="field.field_type === 'string'"
            :label="field.description"
            :prop="`task_config.${field.name}`"
            :rules="field.required ? [{ required: true, message: `请输入${field.description}` }] : []"
          >
            <el-input 
              v-model="form.task_config[field.name]" 
              :placeholder="field.description"
            />
          </el-form-item>
          
          <!-- 文本域 -->
          <el-form-item 
            v-else-if="field.field_type === 'text'"
            :label="field.description"
            :prop="`task_config.${field.name}`"
            :rules="field.required ? [{ required: true, message: `请输入${field.description}` }] : []"
          >
            <el-input 
              v-model="form.task_config[field.name]" 
              type="textarea"
              :rows="3"
              :placeholder="field.description"
            />
          </el-form-item>
          
          <!-- 选择框 -->
          <el-form-item 
            v-else-if="field.field_type === 'select'"
            :label="field.description"
            :prop="`task_config.${field.name}`"
            :rules="field.required ? [{ required: true, message: `请选择${field.description}` }] : []"
          >
            <el-select 
              v-model="form.task_config[field.name]" 
              :placeholder="`请选择${field.description}`"
              style="width: 100%"
            >
              <el-option
                v-for="option in field.options"
                :key="option"
                :label="option"
                :value="option"
              />
            </el-select>
          </el-form-item>
          
          <!-- 数字输入 -->
          <el-form-item 
            v-else-if="field.field_type === 'number'"
            :label="field.description"
            :prop="`task_config.${field.name}`"
            :rules="field.required ? [{ required: true, message: `请输入${field.description}` }] : []"
          >
            <el-input-number 
              v-model="form.task_config[field.name]" 
              :min="0"
              style="width: 200px"
            />
          </el-form-item>
          
          <!-- 布尔值 -->
          <el-form-item 
            v-else-if="field.field_type === 'boolean'"
            :label="field.description"
          >
            <el-switch v-model="form.task_config[field.name]" />
          </el-form-item>
          
          <!-- 对象/JSON -->
          <el-form-item 
            v-else-if="field.field_type === 'object'"
            :label="field.description"
          >
            <el-input 
              v-model="objectFields[field.name]" 
              type="textarea"
              :rows="3"
              :placeholder="`JSON 格式的${field.description}`"
              @blur="parseObjectField(field.name)"
            />
            <div class="form-help">
              <small>请输入有效的 JSON 格式</small>
            </div>
          </el-form-item>
        </div>
      </el-card>

      <!-- 高级配置 -->
      <el-card class="form-section">
        <template #header>
          <span>高级配置</span>
        </template>
        
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="最大并发" prop="max_concurrent">
              <el-input-number 
                v-model="form.max_concurrent" 
                :min="1"
                :max="10"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="超时时间" prop="timeout_seconds">
              <el-input-number 
                v-model="form.timeout_seconds" 
                :min="1"
                :max="3600"
                style="width: 100%"
              />
              <span style="margin-left: 5px">秒</span>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="重试次数" prop="retry_count">
              <el-input-number 
                v-model="form.retry_count" 
                :min="0"
                :max="5"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>
      </el-card>
    </el-form>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">取消</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="loading">
          {{ isEdit ? '更新' : '创建' }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup>
import { ref, reactive, computed, watch, nextTick } from 'vue'
import { ElMessage } from 'element-plus'
import { taskApi } from '@/api/task'

// Props & Emits
const props = defineProps({
  modelValue: Boolean,
  task: Object
})

const emit = defineEmits(['update:modelValue', 'saved'])

// 响应式数据
const loading = ref(false)
const formRef = ref()
const taskTypes = ref([])
const objectFields = reactive({})

// 计算属性
const dialogVisible = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const isEdit = computed(() => !!props.task)

const currentTaskTypeSchema = computed(() => {
  if (!form.task_type || !taskTypes.value.length) return null
  return taskTypes.value.find(type => type.name === form.task_type)
})

// 表单数据
const form = reactive({
  name: '',
  description: '',
  task_type: '',
  schedule_type: 'cron',
  schedule_config: {},
  task_config: {},
  max_concurrent: 1,
  timeout_seconds: 300,
  retry_count: 0
})

// 表单验证规则
const rules = {
  name: [
    { required: true, message: '请输入任务名称', trigger: 'blur' },
    { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  task_type: [
    { required: true, message: '请选择任务类型', trigger: 'change' }
  ],
  schedule_type: [
    { required: true, message: '请选择调度类型', trigger: 'change' }
  ]
}

// 方法
const loadTaskTypes = async () => {
  try {
    const response = await taskApi.getTaskTypes()
    taskTypes.value = response.map(type => ({
      name: type.name.toLowerCase().replace(/\s+/g, '_'),
      description: type.description,
      fields: type.fields
    }))
  } catch (error) {
    ElMessage.error('加载任务类型失败: ' + error.message)
  }
}

const onTaskTypeChange = () => {
  // 重置任务配置
  form.task_config = {}
  
  // 设置默认值
  if (currentTaskTypeSchema.value) {
    currentTaskTypeSchema.value.fields.forEach(field => {
      if (field.default_value !== undefined) {
        form.task_config[field.name] = field.default_value
      }
    })
  }
}

const onScheduleTypeChange = () => {
  // 重置调度配置
  form.schedule_config = {}
  
  // 设置默认值
  switch (form.schedule_type) {
    case 'cron':
      form.schedule_config = { expression: '0 0 12 * * ?' }
      break
    case 'interval':
      form.schedule_config = { seconds: 3600 }
      break
    case 'once':
      form.schedule_config = { run_at: null }
      break
  }
}

const parseObjectField = (fieldName) => {
  const value = objectFields[fieldName]
  if (!value) {
    form.task_config[fieldName] = {}
    return
  }
  
  try {
    form.task_config[fieldName] = JSON.parse(value)
  } catch (error) {
    ElMessage.warning(`${fieldName} 格式不正确，请检查 JSON 语法`)
  }
}

const resetForm = () => {
  Object.assign(form, {
    name: '',
    description: '',
    task_type: '',
    schedule_type: 'cron',
    schedule_config: {},
    task_config: {},
    max_concurrent: 1,
    timeout_seconds: 300,
    retry_count: 0
  })
  
  // 清空对象字段
  Object.keys(objectFields).forEach(key => {
    delete objectFields[key]
  })
}

const loadTaskData = () => {
  if (props.task) {
    Object.assign(form, {
      name: props.task.name,
      description: props.task.description || '',
      task_type: props.task.task_type,
      schedule_type: props.task.schedule_type,
      schedule_config: props.task.schedule_config || {},
      task_config: props.task.task_config || {},
      max_concurrent: props.task.max_concurrent || 1,
      timeout_seconds: props.task.timeout_seconds || 300,
      retry_count: props.task.retry_count || 0
    })
    
    // 处理对象字段
    if (currentTaskTypeSchema.value) {
      currentTaskTypeSchema.value.fields.forEach(field => {
        if (field.field_type === 'object' && form.task_config[field.name]) {
          objectFields[field.name] = JSON.stringify(form.task_config[field.name], null, 2)
        }
      })
    }
  }
}

const handleSubmit = async () => {
  try {
    await formRef.value.validate()
    
    loading.value = true
    
    const submitData = {
      name: form.name,
      description: form.description || null,
      task_type: form.task_type,
      schedule_type: form.schedule_type,
      schedule_config: form.schedule_config,
      task_config: form.task_config,
      max_concurrent: form.max_concurrent,
      timeout_seconds: form.timeout_seconds,
      retry_count: form.retry_count
    }
    
    if (isEdit.value) {
      await taskApi.updateTask(props.task.id, submitData)
      ElMessage.success('任务更新成功')
    } else {
      await taskApi.createTask(submitData)
      ElMessage.success('任务创建成功')
    }
    
    emit('saved')
  } catch (error) {
    ElMessage.error('保存失败: ' + error.message)
  } finally {
    loading.value = false
  }
}

const handleClose = () => {
  formRef.value?.resetFields()
  resetForm()
  dialogVisible.value = false
}

// 监听器
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    loadTaskTypes().then(() => {
      nextTick(() => {
        loadTaskData()
      })
    })
  }
})

watch(() => form.task_type, () => {
  if (currentTaskTypeSchema.value) {
    // 初始化对象字段
    currentTaskTypeSchema.value.fields.forEach(field => {
      if (field.field_type === 'object') {
        if (!objectFields[field.name]) {
          objectFields[field.name] = JSON.stringify(field.default_value || {}, null, 2)
        }
      }
    })
  }
})
</script>

<style scoped>
.form-section {
  margin-bottom: 20px;
}

.form-help {
  margin-top: 5px;
  color: #909399;
}

.dialog-footer {
  text-align: right;
}

:deep(.el-card__header) {
  padding: 10px 20px;
  background-color: #f5f7fa;
}

:deep(.el-card__body) {
  padding: 20px;
}
</style>
