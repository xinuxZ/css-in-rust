//! 性能分析器模块
//!
//! 提供详细的性能分析和调试功能

use crate::performance::metrics::{MetricsCollector, OperationMetric, OperationType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// 性能分析器配置
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    /// 是否启用详细分析
    pub enable_detailed_profiling: bool,
    /// 是否启用内存分析
    pub enable_memory_profiling: bool,
    /// 是否启用CPU分析
    pub enable_cpu_profiling: bool,
    /// 采样间隔（毫秒）
    pub sampling_interval_ms: u64,
    /// 最大分析会话数
    pub max_sessions: usize,
    /// 是否自动生成报告
    pub auto_generate_reports: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enable_detailed_profiling: true,
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
            sampling_interval_ms: 100,
            max_sessions: 100,
            auto_generate_reports: false,
        }
    }
}

/// 性能分析会话
#[derive(Debug, Clone)]
pub struct ProfilingSession {
    /// 会话ID
    pub session_id: String,
    /// 会话名称
    pub name: String,
    /// 开始时间
    pub start_time: Instant,
    /// 结束时间
    pub end_time: Option<Instant>,
    /// 子操作列表
    pub operations: Vec<ProfiledOperation>,
    /// 内存使用情况
    pub memory_usage: Vec<MemorySnapshot>,
    /// CPU使用情况
    pub cpu_usage: Vec<CpuSnapshot>,
    /// 自定义标签
    pub tags: HashMap<String, String>,
}

// impl<'a> Drop for ProfilingSession<'a> {
//     fn drop(&mut self) {
//         let duration = self.start_time.elapsed();
//         self.manager
//             .metrics_collector
//             .record_operation(&self.operation, duration);
//     }
// }

impl ProfilingSession {
    /// 创建新的分析会话
    pub fn new(session_id: String, name: String) -> Self {
        Self {
            session_id,
            name,
            start_time: Instant::now(),
            end_time: None,
            operations: Vec::new(),
            memory_usage: Vec::new(),
            cpu_usage: Vec::new(),
            tags: HashMap::new(),
        }
    }

    /// 添加操作
    pub fn add_operation(&mut self, operation: ProfiledOperation) {
        self.operations.push(operation);
    }

    /// 添加内存快照
    pub fn add_memory_snapshot(&mut self, snapshot: MemorySnapshot) {
        self.memory_usage.push(snapshot);
    }

    /// 添加CPU快照
    pub fn add_cpu_snapshot(&mut self, snapshot: CpuSnapshot) {
        self.cpu_usage.push(snapshot);
    }

    /// 添加标签
    pub fn add_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }

    /// 结束会话
    pub fn finish(&mut self) {
        self.end_time = Some(Instant::now());
    }

    /// 获取会话持续时间
    pub fn duration(&self) -> Duration {
        match self.end_time {
            Some(end) => end.duration_since(self.start_time),
            None => Instant::now().duration_since(self.start_time),
        }
    }

    /// 获取总操作数
    pub fn total_operations(&self) -> usize {
        self.operations.len()
    }

    /// 获取平均操作时间
    pub fn average_operation_time(&self) -> Duration {
        if self.operations.is_empty() {
            return Duration::from_millis(0);
        }

        let total_duration: Duration = self.operations.iter().map(|op| op.duration).sum();

        total_duration / self.operations.len() as u32
    }

    /// 获取最慢的操作
    pub fn slowest_operation(&self) -> Option<&ProfiledOperation> {
        self.operations.iter().max_by_key(|op| op.duration)
    }

    /// 获取内存使用峰值
    pub fn peak_memory_usage(&self) -> Option<usize> {
        self.memory_usage
            .iter()
            .map(|snapshot| snapshot.used_bytes)
            .max()
    }
}

/// 分析的操作
#[derive(Debug, Clone)]
pub struct ProfiledOperation {
    /// 操作名称
    pub name: String,
    /// 操作类型
    pub operation_type: OperationType,
    /// 开始时间
    pub start_time: Instant,
    /// 持续时间
    pub duration: Duration,
    /// 处理的数据大小
    pub data_size: usize,
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error: Option<String>,
    /// 子操作
    pub sub_operations: Vec<ProfiledOperation>,
    /// 自定义属性
    pub attributes: HashMap<String, String>,
}

impl ProfiledOperation {
    /// 创建新的分析操作
    pub fn new(name: String, operation_type: OperationType) -> Self {
        Self {
            name,
            operation_type,
            start_time: Instant::now(),
            duration: Duration::from_millis(0),
            data_size: 0,
            success: true,
            error: None,
            sub_operations: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    /// 完成操作
    pub fn finish(&mut self) {
        self.duration = Instant::now().duration_since(self.start_time);
    }

    /// 设置错误
    pub fn set_error(&mut self, error: String) {
        self.success = false;
        self.error = Some(error);
    }

    /// 添加子操作
    pub fn add_sub_operation(&mut self, sub_op: ProfiledOperation) {
        self.sub_operations.push(sub_op);
    }

    /// 设置数据大小
    pub fn set_data_size(&mut self, size: usize) {
        self.data_size = size;
    }

    /// 添加属性
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    /// 计算吞吐量
    pub fn throughput(&self) -> f64 {
        if self.duration.as_secs_f64() == 0.0 {
            return 0.0;
        }
        self.data_size as f64 / self.duration.as_secs_f64()
    }
}

/// 内存快照
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    /// 时间戳
    pub timestamp: Instant,
    /// 已使用内存（字节）
    pub used_bytes: usize,
    /// 总分配内存（字节）
    pub allocated_bytes: usize,
    /// 堆内存（字节）
    pub heap_bytes: usize,
    /// 栈内存（字节）
    pub stack_bytes: usize,
}

impl MemorySnapshot {
    /// 创建当前内存快照
    pub fn current() -> Self {
        // 这里应该使用实际的内存监控API
        // 为了演示，使用模拟数据
        Self {
            timestamp: Instant::now(),
            used_bytes: Self::get_current_memory_usage(),
            allocated_bytes: Self::get_allocated_memory(),
            heap_bytes: Self::get_heap_memory(),
            stack_bytes: Self::get_stack_memory(),
        }
    }

    /// 获取当前内存使用量（模拟）
    fn get_current_memory_usage() -> usize {
        // 在实际实现中，这里应该调用系统API
        // 例如在Linux上可以读取/proc/self/status
        1024 * 1024 * 10 // 10MB 模拟值
    }

    /// 获取分配内存（模拟）
    fn get_allocated_memory() -> usize {
        1024 * 1024 * 15 // 15MB 模拟值
    }

    /// 获取堆内存（模拟）
    fn get_heap_memory() -> usize {
        1024 * 1024 * 8 // 8MB 模拟值
    }

    /// 获取栈内存（模拟）
    fn get_stack_memory() -> usize {
        1024 * 1024 * 2 // 2MB 模拟值
    }
}

/// CPU快照
#[derive(Debug, Clone)]
pub struct CpuSnapshot {
    /// 时间戳
    pub timestamp: Instant,
    /// CPU使用率（百分比）
    pub cpu_usage_percent: f64,
    /// 用户态时间（微秒）
    pub user_time_micros: u64,
    /// 系统态时间（微秒）
    pub system_time_micros: u64,
    /// 线程数
    pub thread_count: usize,
}

impl CpuSnapshot {
    /// 创建当前CPU快照
    pub fn current() -> Self {
        Self {
            timestamp: Instant::now(),
            cpu_usage_percent: Self::get_cpu_usage(),
            user_time_micros: Self::get_user_time(),
            system_time_micros: Self::get_system_time(),
            thread_count: Self::get_thread_count(),
        }
    }

    /// 获取CPU使用率（模拟）
    fn get_cpu_usage() -> f64 {
        // 在实际实现中，这里应该调用系统API
        25.5 // 模拟值
    }

    /// 获取用户态时间（模拟）
    fn get_user_time() -> u64 {
        1000000 // 1秒 模拟值
    }

    /// 获取系统态时间（模拟）
    fn get_system_time() -> u64 {
        500000 // 0.5秒 模拟值
    }

    /// 获取线程数（模拟）
    fn get_thread_count() -> usize {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    }
}

/// 性能分析器
pub struct PerformanceProfiler {
    config: ProfilerConfig,
    sessions: Arc<Mutex<HashMap<String, ProfilingSession>>>,
    metrics_collector: Arc<Mutex<MetricsCollector>>,
    active_session: Arc<Mutex<Option<String>>>,
}

impl PerformanceProfiler {
    /// 创建新的性能分析器
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            metrics_collector: Arc::new(Mutex::new(MetricsCollector::new())),
            active_session: Arc::new(Mutex::new(None)),
        }
    }

    /// 开始新的分析会话
    pub fn start_session(&self, name: String) -> String {
        let session_id = format!("session_{}", chrono::Utc::now().timestamp_millis());
        let session = ProfilingSession::new(session_id.clone(), name);

        {
            let mut sessions = self.sessions.lock().unwrap();

            // 限制会话数量
            if sessions.len() >= self.config.max_sessions {
                // 移除最旧的会话
                if let Some(oldest_id) = sessions.keys().next().cloned() {
                    sessions.remove(&oldest_id);
                }
            }

            sessions.insert(session_id.clone(), session);
        }

        // 设置为活动会话
        *self.active_session.lock().unwrap() = Some(session_id.clone());

        session_id
    }

    /// 结束分析会话
    pub fn end_session(&self, session_id: &str) {
        if let Some(session) = self.sessions.lock().unwrap().get_mut(session_id) {
            session.finish();
        }

        // 如果是活动会话，清除活动状态
        let mut active = self.active_session.lock().unwrap();
        if active.as_ref() == Some(&session_id.to_string()) {
            *active = None;
        }
    }

    /// 开始操作分析
    pub fn start_operation(
        &self,
        name: String,
        operation_type: OperationType,
    ) -> OperationProfiler {
        OperationProfiler::new(
            name,
            operation_type,
            self.sessions.clone(),
            self.active_session.clone(),
            self.metrics_collector.clone(),
        )
    }

    /// 添加内存快照到活动会话
    pub fn add_memory_snapshot(&self) {
        if let Some(session_id) = self.active_session.lock().unwrap().clone() {
            if let Some(session) = self.sessions.lock().unwrap().get_mut(&session_id) {
                session.add_memory_snapshot(MemorySnapshot::current());
            }
        }
    }

    /// 添加CPU快照到活动会话
    pub fn add_cpu_snapshot(&self) {
        if let Some(session_id) = self.active_session.lock().unwrap().clone() {
            if let Some(session) = self.sessions.lock().unwrap().get_mut(&session_id) {
                session.add_cpu_snapshot(CpuSnapshot::current());
            }
        }
    }

    /// 获取会话
    pub fn get_session(&self, session_id: &str) -> Option<ProfilingSession> {
        self.sessions.lock().unwrap().get(session_id).cloned()
    }

    /// 获取所有会话
    pub fn get_all_sessions(&self) -> Vec<ProfilingSession> {
        self.sessions.lock().unwrap().values().cloned().collect()
    }

    /// 生成性能报告
    pub fn generate_report(&self, session_id: &str) -> Option<String> {
        let session = self.get_session(session_id)?;

        let mut report = String::new();

        report.push_str(&format!("=== 性能分析报告: {} ===\n\n", session.name));
        report.push_str(&format!("会话ID: {}\n", session.session_id));
        report.push_str(&format!(
            "持续时间: {:.2}ms\n",
            session.duration().as_millis()
        ));
        report.push_str(&format!("总操作数: {}\n", session.total_operations()));

        if let Some(avg_time) = session.average_operation_time().as_millis().checked_sub(0) {
            report.push_str(&format!("平均操作时间: {:.2}ms\n", avg_time));
        }

        if let Some(slowest) = session.slowest_operation() {
            report.push_str(&format!(
                "最慢操作: {} ({:.2}ms)\n",
                slowest.name,
                slowest.duration.as_millis()
            ));
        }

        if let Some(peak_memory) = session.peak_memory_usage() {
            report.push_str(&format!(
                "内存使用峰值: {:.2} MB\n",
                peak_memory as f64 / 1024.0 / 1024.0
            ));
        }

        // 操作详情
        report.push_str("\n=== 操作详情 ===\n");
        for (i, operation) in session.operations.iter().enumerate() {
            report.push_str(&format!(
                "{}. {} ({:?}) - {:.2}ms\n",
                i + 1,
                operation.name,
                operation.operation_type,
                operation.duration.as_millis()
            ));

            if !operation.success {
                if let Some(error) = &operation.error {
                    report.push_str(&format!("   错误: {}\n", error));
                }
            }
        }

        // 标签
        if !session.tags.is_empty() {
            report.push_str("\n=== 标签 ===\n");
            for (key, value) in &session.tags {
                report.push_str(&format!("{}: {}\n", key, value));
            }
        }

        Some(report)
    }

    /// 清理旧会话
    pub fn cleanup_old_sessions(&self, max_age: Duration) {
        let cutoff_time = Instant::now() - max_age;

        let mut sessions = self.sessions.lock().unwrap();
        sessions.retain(|_, session| session.start_time >= cutoff_time);
    }
}

/// 操作分析器
pub struct OperationProfiler {
    operation: ProfiledOperation,
    sessions: Arc<Mutex<HashMap<String, ProfilingSession>>>,
    active_session: Arc<Mutex<Option<String>>>,
    metrics_collector: Arc<Mutex<MetricsCollector>>,
}

impl OperationProfiler {
    /// 创建新的操作分析器
    fn new(
        name: String,
        operation_type: OperationType,
        sessions: Arc<Mutex<HashMap<String, ProfilingSession>>>,
        active_session: Arc<Mutex<Option<String>>>,
        metrics_collector: Arc<Mutex<MetricsCollector>>,
    ) -> Self {
        Self {
            operation: ProfiledOperation::new(name, operation_type),
            sessions,
            active_session,
            metrics_collector,
        }
    }

    /// 设置数据大小
    pub fn set_data_size(&mut self, size: usize) {
        self.operation.set_data_size(size);
    }

    /// 添加属性
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.operation.add_attribute(key, value);
    }

    /// 设置错误
    pub fn set_error(&mut self, error: String) {
        self.operation.set_error(error);
    }
}

impl Drop for OperationProfiler {
    fn drop(&mut self) {
        // 完成操作
        self.operation.finish();

        // 添加到活动会话
        if let Some(session_id) = self.active_session.lock().unwrap().clone() {
            if let Some(session) = self.sessions.lock().unwrap().get_mut(&session_id) {
                session.add_operation(self.operation.clone());
            }
        }

        // 添加到指标收集器
        let metric = OperationMetric::new(
            self.operation.operation_type.clone(),
            self.operation.start_time,
            self.operation.duration,
        )
        .with_data_size(self.operation.data_size);

        let metric = if self.operation.success {
            metric
        } else {
            metric.with_error(self.operation.error.clone().unwrap_or_default())
        };

        self.metrics_collector.lock().unwrap().add_metric(metric);
    }
}
