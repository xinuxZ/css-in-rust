//! 性能指标收集模块
//!
//! 提供详细的性能监控和分析功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// 操作类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationType {
    /// CSS解析
    Parse,
    /// CSS优化
    Optimize,
    /// 缓存查找
    CacheLookup,
    /// 缓存写入
    CacheWrite,
    /// 文件读取
    FileRead,
    /// 文件写入
    FileWrite,
    /// 增量编译
    IncrementalCompile,
    /// 完整编译
    FullCompile,
    /// 样式注入
    StyleInjection,
    /// 自定义操作
    Custom(String),
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Parse => write!(f, "parse"),
            OperationType::Optimize => write!(f, "optimize"),
            OperationType::CacheLookup => write!(f, "cache_lookup"),
            OperationType::CacheWrite => write!(f, "cache_write"),
            OperationType::FileRead => write!(f, "file_read"),
            OperationType::FileWrite => write!(f, "file_write"),
            OperationType::IncrementalCompile => write!(f, "incremental_compile"),
            OperationType::FullCompile => write!(f, "full_compile"),
            OperationType::StyleInjection => write!(f, "style_injection"),
            OperationType::Custom(name) => write!(f, "custom_{}", name),
        }
    }
}

/// 单次操作的性能数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetric {
    /// 操作类型
    pub operation_type: OperationType,
    /// 开始时间戳
    pub start_time: u64,
    /// 持续时间（微秒）
    pub duration_micros: u64,
    /// 处理的数据大小（字节）
    pub data_size: usize,
    /// 是否成功
    pub success: bool,
    /// 错误信息（如果失败）
    pub error_message: Option<String>,
    /// 额外的元数据
    pub metadata: HashMap<String, String>,
}

impl OperationMetric {
    /// 创建新的操作指标
    pub fn new(operation_type: OperationType, start_time: Instant, duration: Duration) -> Self {
        let start_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            operation_type,
            start_time: start_timestamp,
            duration_micros: duration.as_micros() as u64,
            data_size: 0,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
        }
    }

    /// 设置数据大小
    pub fn with_data_size(mut self, size: usize) -> Self {
        self.data_size = size;
        self
    }

    /// 设置错误信息
    pub fn with_error(mut self, error: String) -> Self {
        self.success = false;
        self.error_message = Some(error);
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// 计算吞吐量（字节/秒）
    pub fn throughput_bytes_per_sec(&self) -> f64 {
        if self.duration_micros == 0 {
            return 0.0;
        }

        let duration_secs = self.duration_micros as f64 / 1_000_000.0;
        self.data_size as f64 / duration_secs
    }

    /// 获取持续时间（毫秒）
    pub fn duration_millis(&self) -> f64 {
        self.duration_micros as f64 / 1000.0
    }
}

/// 聚合的性能统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AggregatedStats {
    /// 操作总数
    pub total_operations: usize,
    /// 成功操作数
    pub successful_operations: usize,
    /// 失败操作数
    pub failed_operations: usize,
    /// 总持续时间（微秒）
    pub total_duration_micros: u64,
    /// 平均持续时间（微秒）
    pub avg_duration_micros: u64,
    /// 最小持续时间（微秒）
    pub min_duration_micros: u64,
    /// 最大持续时间（微秒）
    pub max_duration_micros: u64,
    /// 总处理数据大小（字节）
    pub total_data_size: usize,
    /// 平均吞吐量（字节/秒）
    pub avg_throughput: f64,
    /// 95百分位持续时间（微秒）
    pub p95_duration_micros: u64,
    /// 99百分位持续时间（微秒）
    pub p99_duration_micros: u64,
}

impl AggregatedStats {
    /// 从操作指标列表计算聚合统计
    pub fn from_metrics(metrics: &[OperationMetric]) -> Self {
        if metrics.is_empty() {
            return Self::default();
        }

        let total_operations = metrics.len();
        let successful_operations = metrics.iter().filter(|m| m.success).count();
        let failed_operations = total_operations - successful_operations;

        let total_duration_micros: u64 = metrics.iter().map(|m| m.duration_micros).sum();
        let avg_duration_micros = total_duration_micros / total_operations as u64;

        let min_duration_micros = metrics.iter().map(|m| m.duration_micros).min().unwrap_or(0);
        let max_duration_micros = metrics.iter().map(|m| m.duration_micros).max().unwrap_or(0);

        let total_data_size: usize = metrics.iter().map(|m| m.data_size).sum();

        let avg_throughput = if total_duration_micros > 0 {
            let total_duration_secs = total_duration_micros as f64 / 1_000_000.0;
            total_data_size as f64 / total_duration_secs
        } else {
            0.0
        };

        // 计算百分位数
        let mut durations: Vec<u64> = metrics.iter().map(|m| m.duration_micros).collect();
        durations.sort_unstable();

        let p95_index = (durations.len() as f64 * 0.95) as usize;
        let p99_index = (durations.len() as f64 * 0.99) as usize;

        let p95_duration_micros = durations
            .get(p95_index.saturating_sub(1))
            .copied()
            .unwrap_or(0);
        let p99_duration_micros = durations
            .get(p99_index.saturating_sub(1))
            .copied()
            .unwrap_or(0);

        Self {
            total_operations,
            successful_operations,
            failed_operations,
            total_duration_micros,
            avg_duration_micros,
            min_duration_micros,
            max_duration_micros,
            total_data_size,
            avg_throughput,
            p95_duration_micros,
            p99_duration_micros,
        }
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            return 0.0;
        }
        self.successful_operations as f64 / self.total_operations as f64
    }

    /// 获取平均持续时间（毫秒）
    pub fn avg_duration_millis(&self) -> f64 {
        self.avg_duration_micros as f64 / 1000.0
    }

    /// 获取P95持续时间（毫秒）
    pub fn p95_duration_millis(&self) -> f64 {
        self.p95_duration_micros as f64 / 1000.0
    }

    /// 获取P99持续时间（毫秒）
    pub fn p99_duration_millis(&self) -> f64 {
        self.p99_duration_micros as f64 / 1000.0
    }
}

/// 性能指标集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    /// 按操作类型分组的指标
    pub metrics_by_type: HashMap<OperationType, Vec<OperationMetric>>,
    /// 聚合统计
    pub aggregated_stats: HashMap<OperationType, AggregatedStats>,
    /// 全局统计
    pub global_stats: AggregatedStats,
    /// 开始收集时间
    pub collection_start_time: u64,
    /// 最后更新时间
    pub last_update_time: u64,
}

impl PerformanceMetrics {
    /// 创建新的性能指标集合
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            metrics_by_type: HashMap::new(),
            aggregated_stats: HashMap::new(),
            global_stats: AggregatedStats::default(),
            collection_start_time: now,
            last_update_time: now,
        }
    }

    /// 添加操作指标
    pub fn add_metric(&mut self, metric: OperationMetric) {
        let operation_type = metric.operation_type.clone();

        self.metrics_by_type
            .entry(operation_type.clone())
            .or_insert_with(Vec::new)
            .push(metric);

        self.update_aggregated_stats(operation_type);
        self.update_global_stats();

        self.last_update_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// 更新指定操作类型的聚合统计
    fn update_aggregated_stats(&mut self, operation_type: OperationType) {
        if let Some(metrics) = self.metrics_by_type.get(&operation_type) {
            let stats = AggregatedStats::from_metrics(metrics);
            self.aggregated_stats.insert(operation_type, stats);
        }
    }

    /// 更新全局统计
    fn update_global_stats(&mut self) {
        let all_metrics: Vec<&OperationMetric> = self
            .metrics_by_type
            .values()
            .flat_map(|metrics| metrics.iter())
            .collect();

        let metrics_vec: Vec<OperationMetric> = all_metrics.into_iter().cloned().collect();
        self.global_stats = AggregatedStats::from_metrics(&metrics_vec);
    }

    /// 获取指定操作类型的统计
    pub fn get_stats(&self, operation_type: &OperationType) -> Option<&AggregatedStats> {
        self.aggregated_stats.get(operation_type)
    }

    /// 获取全局统计
    pub fn get_global_stats(&self) -> &AggregatedStats {
        &self.global_stats
    }

    /// 清理旧的指标数据
    pub fn cleanup_old_metrics(&mut self, max_age_secs: u64) {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(max_age_secs);

        for metrics in self.metrics_by_type.values_mut() {
            metrics.retain(|metric| metric.start_time >= cutoff_time);
        }

        // 重新计算统计
        let operation_types: Vec<OperationType> = self.metrics_by_type.keys().cloned().collect();
        for operation_type in operation_types {
            self.update_aggregated_stats(operation_type);
        }
        self.update_global_stats();
    }

    /// 重置所有指标
    pub fn reset(&mut self) {
        self.metrics_by_type.clear();
        self.aggregated_stats.clear();
        self.global_stats = AggregatedStats::default();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.collection_start_time = now;
        self.last_update_time = now;
    }

    /// 获取收集持续时间（秒）
    pub fn collection_duration_secs(&self) -> u64 {
        self.last_update_time - self.collection_start_time
    }
}

/// 性能指标收集器
pub struct MetricsCollector {
    metrics: PerformanceMetrics,
    max_metrics_per_type: usize,
    auto_cleanup_interval: Duration,
    last_cleanup: Instant,
}

impl MetricsCollector {
    /// 创建新的指标收集器
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            max_metrics_per_type: 1000,
            auto_cleanup_interval: Duration::from_secs(300), // 5分钟
            last_cleanup: Instant::now(),
        }
    }

    /// 使用自定义配置创建指标收集器
    pub fn with_config(max_metrics_per_type: usize, auto_cleanup_interval: Duration) -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            max_metrics_per_type,
            auto_cleanup_interval,
            last_cleanup: Instant::now(),
        }
    }

    /// 记录操作
    pub fn record_operation(&mut self, operation: &str, duration: Duration) {
        let operation_type = OperationType::Custom(operation.to_string());
        let metric = OperationMetric::new(operation_type, Instant::now(), duration);
        self.add_metric(metric);
    }

    /// 记录操作（带数据大小）
    pub fn record_operation_with_size(
        &mut self,
        operation: &str,
        duration: Duration,
        data_size: usize,
    ) {
        let operation_type = OperationType::Custom(operation.to_string());
        let metric = OperationMetric::new(operation_type, Instant::now(), duration)
            .with_data_size(data_size);
        self.add_metric(metric);
    }

    /// 记录特定类型的操作
    pub fn record_typed_operation(&mut self, operation_type: OperationType, duration: Duration) {
        let metric = OperationMetric::new(operation_type, Instant::now(), duration);
        self.add_metric(metric);
    }

    /// 添加指标
    pub fn add_metric(&mut self, metric: OperationMetric) {
        self.metrics.add_metric(metric);

        // 检查是否需要清理
        if self.last_cleanup.elapsed() >= self.auto_cleanup_interval {
            self.auto_cleanup();
        }
    }

    /// 自动清理
    fn auto_cleanup(&mut self) {
        // 限制每种操作类型的指标数量
        for metrics in self.metrics.metrics_by_type.values_mut() {
            if metrics.len() > self.max_metrics_per_type {
                // 保留最新的指标
                metrics.sort_by_key(|m| m.start_time);
                metrics.truncate(self.max_metrics_per_type);
            }
        }

        // 清理超过1小时的旧指标
        self.metrics.cleanup_old_metrics(3600);

        self.last_cleanup = Instant::now();
    }

    /// 获取性能指标
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// 重置指标
    pub fn reset(&mut self) {
        self.metrics.reset();
        self.last_cleanup = Instant::now();
    }

    /// 生成性能报告
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== CSS-in-Rust 性能报告 ===\n\n");

        // 全局统计
        let global = &self.metrics.global_stats;
        report.push_str(&format!(
            "全局统计:\n\
             - 总操作数: {}\n\
             - 成功率: {:.2}%\n\
             - 平均耗时: {:.2}ms\n\
             - P95耗时: {:.2}ms\n\
             - P99耗时: {:.2}ms\n\
             - 平均吞吐量: {:.2} bytes/s\n\n",
            global.total_operations,
            global.success_rate() * 100.0,
            global.avg_duration_millis(),
            global.p95_duration_millis(),
            global.p99_duration_millis(),
            global.avg_throughput
        ));

        // 按操作类型统计
        report.push_str("按操作类型统计:\n");
        for (op_type, stats) in &self.metrics.aggregated_stats {
            report.push_str(&format!(
                "- {}:\n\
                 \t操作数: {}\n\
                 \t成功率: {:.2}%\n\
                 \t平均耗时: {:.2}ms\n\
                 \t吞吐量: {:.2} bytes/s\n",
                op_type,
                stats.total_operations,
                stats.success_rate() * 100.0,
                stats.avg_duration_millis(),
                stats.avg_throughput
            ));
        }

        report.push_str(&format!(
            "\n收集时间: {}秒\n",
            self.metrics.collection_duration_secs()
        ));

        report
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
