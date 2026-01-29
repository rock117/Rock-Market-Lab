-- ============================================================================
-- 定时任务系统完整数据库初始化脚本
-- 包含所有表结构、索引、示例数据和常用查询
-- ============================================================================

-- 设置字符集
SET NAMES utf8mb4;
SET CHARACTER SET utf8mb4;

-- ============================================================================
-- 1. 创建定时任务表
-- ============================================================================
CREATE TABLE scheduled_task (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL COMMENT '任务名称',
    description TEXT COMMENT '任务描述',
    task_type VARCHAR(50) NOT NULL COMMENT '任务类型: http_request, shell_command, rust_function',
    schedule_type VARCHAR(20) NOT NULL COMMENT '调度类型: cron, interval, once',
    schedule_config TEXT NOT NULL COMMENT '调度配置 JSON',
    task_config TEXT NOT NULL COMMENT '任务配置 JSON',
    status VARCHAR(20) NOT NULL DEFAULT 'enabled' COMMENT '任务状态: enabled, paused, disabled, deleted',
    max_concurrent INT NOT NULL DEFAULT 1 COMMENT '最大并发数',
    timeout_seconds INT NOT NULL DEFAULT 300 COMMENT '超时时间(秒)',
    retry_count INT NOT NULL DEFAULT 0 COMMENT '重试次数',
    retry_interval_seconds INT NOT NULL DEFAULT 60 COMMENT '重试间隔(秒)',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    created_by VARCHAR(100) COMMENT '创建者',
    next_run_time TIMESTAMP NULL COMMENT '下次执行时间',
    
    INDEX idx_status (status),
    INDEX idx_task_type (task_type),
    INDEX idx_next_run_time (next_run_time),
    INDEX idx_created_at (created_at),
    INDEX idx_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='定时任务表';

-- ============================================================================
-- 2. 创建任务执行记录表
-- ============================================================================
CREATE TABLE task_execution (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    task_id BIGINT NOT NULL COMMENT '关联的任务ID',
    execution_id VARCHAR(100) NOT NULL UNIQUE COMMENT '执行唯一标识符',
    status VARCHAR(20) NOT NULL DEFAULT 'running' COMMENT '执行状态: running, success, failed, timeout',
    started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '开始执行时间',
    finished_at TIMESTAMP NULL COMMENT '完成时间',
    duration_ms INT NULL COMMENT '执行耗时(毫秒)',
    error_message TEXT COMMENT '错误信息',
    output_summary TEXT COMMENT '输出摘要',
    retry_attempt INT NOT NULL DEFAULT 0 COMMENT '重试次数',
    
    FOREIGN KEY (task_id) REFERENCES scheduled_task(id) ON DELETE CASCADE,
    INDEX idx_task_id (task_id),
    INDEX idx_execution_id (execution_id),
    INDEX idx_status (status),
    INDEX idx_started_at (started_at),
    INDEX idx_finished_at (finished_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='任务执行记录表';

-- ============================================================================
-- 3. 创建任务执行日志表
-- ============================================================================
CREATE TABLE task_execution_log (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    execution_id VARCHAR(100) NOT NULL COMMENT '关联的执行ID',
    log_level VARCHAR(10) NOT NULL COMMENT '日志级别: INFO, WARN, ERROR, DEBUG',
    message TEXT NOT NULL COMMENT '日志消息',
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '日志时间',
    
    FOREIGN KEY (execution_id) REFERENCES task_execution(execution_id) ON DELETE CASCADE,
    INDEX idx_execution_id (execution_id),
    INDEX idx_log_level (log_level),
    INDEX idx_timestamp (timestamp)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='任务执行日志表';

-- ============================================================================
-- 4. 创建性能优化索引
-- ============================================================================

-- scheduled_task 表的复合索引
CREATE INDEX idx_task_status_type ON scheduled_task(status, task_type);
CREATE INDEX idx_task_next_run_status ON scheduled_task(next_run_time, status);
CREATE INDEX idx_task_created_by_status ON scheduled_task(created_by, status);

-- task_execution 表的复合索引
CREATE INDEX idx_execution_task_status ON task_execution(task_id, status);
CREATE INDEX idx_execution_started_status ON task_execution(started_at, status);
CREATE INDEX idx_execution_duration ON task_execution(duration_ms);

-- task_execution_log 表的复合索引
CREATE INDEX idx_log_execution_level ON task_execution_log(execution_id, log_level);
CREATE INDEX idx_log_timestamp_level ON task_execution_log(timestamp, log_level);

-- ============================================================================
-- 5. 数据清理脚本（手动执行）
-- ============================================================================

-- 清理超过30天的执行记录
-- DELETE FROM task_execution WHERE started_at < DATE_SUB(NOW(), INTERVAL 30 DAY);

-- 清理孤立的日志记录
-- DELETE l FROM task_execution_log l
-- LEFT JOIN task_execution e ON l.execution_id = e.execution_id
-- WHERE e.execution_id IS NULL;

-- 重置卡住的任务（运行超过2小时）
-- UPDATE task_execution 
-- SET status = 'timeout',
--     finished_at = NOW(),
--     duration_ms = TIMESTAMPDIFF(MICROSECOND, started_at, NOW()) / 1000,
--     error_message = 'Task timeout - automatically reset'
-- WHERE status = 'running' 
--     AND started_at < DATE_SUB(NOW(), INTERVAL 2 HOUR);

