//! 性能优化模块
//!
//! 提供编译时缓存、增量编译和运行时性能优化功能

pub mod cache;
pub mod incremental;
pub mod metrics;
pub mod profiler;

// 重新导出主要类型
pub use cache::{CacheConfig, CacheEntry, CacheManager};
pub use incremental::{CompilationState, IncrementalCompiler};
pub use metrics::{MetricsCollector, PerformanceMetrics};
pub use profiler::{PerformanceProfiler, ProfilerConfig, ProfilingSession};

use std::time::{Duration, Instant};

/// 性能优化配置
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// 是否启用编译时缓存
    pub enable_compile_cache: bool,
    /// 是否启用增量编译
    pub enable_incremental: bool,
    /// 是否启用性能监控
    pub enable_metrics: bool,
    /// 缓存大小限制（MB）
    pub cache_size_limit: usize,
    /// 缓存过期时间（秒）
    pub cache_ttl: u64,
    /// 是否启用并行处理
    pub enable_parallel: bool,
    /// 工作线程数
    pub worker_threads: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_compile_cache: true,
            enable_incremental: true,
            enable_metrics: true,
            cache_size_limit: 100, // 100MB
            cache_ttl: 3600,       // 1小时
            enable_parallel: true,
            worker_threads: num_cpus::get(),
        }
    }
}

/// 性能优化管理器
pub struct PerformanceManager {
    config: PerformanceConfig,
    cache_manager: CacheManager,
    incremental_compiler: IncrementalCompiler,
    metrics_collector: MetricsCollector,
    profiler: PerformanceProfiler,
}

impl PerformanceManager {
    /// 创建新的性能管理器
    pub fn new(config: PerformanceConfig) -> Self {
        let cache_config = CacheConfig {
            max_size: config.cache_size_limit * 1024 * 1024, // 转换为字节
            ttl: Duration::from_secs(config.cache_ttl),
            enable_compression: true,
            enable_persistence: true,
            cache_dir: std::path::PathBuf::from("target/css-cache"),
        };

        Self {
            cache_manager: CacheManager::new(cache_config),
            incremental_compiler: IncrementalCompiler::new(),
            metrics_collector: MetricsCollector::new(),
            profiler: PerformanceProfiler::new(ProfilerConfig::default()),
            config,
        }
    }

    /// 使用默认配置创建性能管理器
    pub fn default() -> Self {
        Self::new(PerformanceConfig::default())
    }

    /// 开始性能分析
    pub fn start_profiling(&mut self, operation: &str) -> ProfilingSession {
        let sessionid = self.profiler.start_session(operation.to_string());
        let name = format!("{} ({})", operation, sessionid);

        ProfilingSession::new(sessionid, name)
    }

    /// 获取性能指标
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        self.metrics_collector.get_metrics()
    }

    /// 清理缓存
    pub fn cleanup_cache(&mut self) {
        self.cache_manager.cleanup();
    }

    /// 重置性能统计
    pub fn reset_metrics(&mut self) {
        self.metrics_collector.reset();
    }
}

// /// 性能分析会话
// pub struct ProfilingSession<'a> {
//     operation: String,
//     start_time: Instant,
//     manager: &'a mut PerformanceManager,
// }

// impl<'a> Drop for ProfilingSession<'a> {
//     fn drop(&mut self) {
//         let duration = self.start_time.elapsed();
//         self.manager
//             .metrics_collector
//             .record_operation(&self.operation, duration);
//     }
// }

/// 性能优化结果
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// 原始大小（字节）
    pub original_size: usize,
    /// 优化后大小（字节）
    pub optimized_size: usize,
    /// 节省的字节数
    pub bytes_saved: usize,
    /// 节省百分比
    pub savings_percentage: f64,
    /// 编译时间（毫秒）
    pub compile_time_ms: u64,
    /// 是否使用了缓存
    pub cache_hit: bool,
    /// 优化操作列表
    pub optimizations: Vec<String>,
}

impl OptimizationResult {
    /// 计算节省百分比
    pub fn calculate_savings_percentage(original: usize, optimized: usize) -> f64 {
        if original == 0 {
            return 0.0;
        }
        ((original - optimized) as f64 / original as f64) * 100.0
    }

    /// 创建新的优化结果
    pub fn new(
        original_size: usize,
        optimized_size: usize,
        compile_time: Duration,
        cache_hit: bool,
    ) -> Self {
        let bytes_saved = original_size.saturating_sub(optimized_size);
        let savings_percentage = Self::calculate_savings_percentage(original_size, optimized_size);

        Self {
            original_size,
            optimized_size,
            bytes_saved,
            savings_percentage,
            compile_time_ms: compile_time.as_millis() as u64,
            cache_hit,
            optimizations: Vec::new(),
        }
    }

    /// 添加优化操作
    pub fn add_optimization(&mut self, optimization: String) {
        self.optimizations.push(optimization);
    }
}
