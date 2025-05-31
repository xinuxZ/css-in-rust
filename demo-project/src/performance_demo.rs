//! 性能优化演示
//!
//! 本模块演示 CSS-in-Rust 的性能优化功能，包括：
//! - 编译时死代码消除
//! - CSS 压缩和优化
//! - 静态分析
//! - 缓存策略
//! - 构建时优化
//! - 运行时性能监控

use css_in_rust::css;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde as _;
use serde_json as _;
use tokio as _;

/// 性能指标收集器
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    compile_times: Vec<Duration>,
    injection_times: Vec<Duration>,
    cache_hit_rates: Vec<f64>,
    memory_usages: Vec<usize>,
    css_rules_counts: Vec<usize>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            compile_times: Vec::new(),
            injection_times: Vec::new(),
            cache_hit_rates: Vec::new(),
            memory_usages: Vec::new(),
            css_rules_counts: Vec::new(),
        }
    }

    pub fn record_compile_time(&mut self, duration: Duration) {
        self.compile_times.push(duration);
    }

    pub fn record_injection_time(&mut self, duration: Duration) {
        self.injection_times.push(duration);
    }

    pub fn record_cache_hit_rate(&mut self, rate: f64) {
        self.cache_hit_rates.push(rate);
    }

    pub fn record_memory_usage(&mut self, usage: usize) {
        self.memory_usages.push(usage);
    }

    pub fn record_css_rules_count(&mut self, count: usize) {
        self.css_rules_counts.push(count);
    }

    pub fn average_compile_time(&self) -> Duration {
        if self.compile_times.is_empty() {
            Duration::from_millis(0)
        } else {
            let total: Duration = self.compile_times.iter().sum();
            total / self.compile_times.len() as u32
        }
    }

    pub fn average_injection_time(&self) -> Duration {
        if self.injection_times.is_empty() {
            Duration::from_millis(0)
        } else {
            let total: Duration = self.injection_times.iter().sum();
            total / self.injection_times.len() as u32
        }
    }

    pub fn overall_cache_hit_rate(&self) -> f64 {
        if self.cache_hit_rates.is_empty() {
            0.0
        } else {
            self.cache_hit_rates.iter().sum::<f64>() / self.cache_hit_rates.len() as f64
        }
    }

    pub fn peak_memory_usage(&self) -> usize {
        self.memory_usages.iter().max().copied().unwrap_or(0)
    }
}

/// 性能分析报告
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub sections: HashMap<String, Duration>,
    pub total_duration: Duration,
    pub slowest_section: String,
}

/// 性能分析器
#[derive(Debug)]
pub struct PerformanceProfiler {
    start_time: Option<Instant>,
    sections: HashMap<String, Duration>,
    current_section: Option<(String, Instant)>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            start_time: None,
            sections: HashMap::new(),
            current_section: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn start_section(&mut self, name: &str) {
        self.current_section = Some((name.to_string(), Instant::now()));
    }

    pub fn end_section(&mut self, name: &str) {
        if let Some((section_name, start_time)) = &self.current_section {
            if section_name == name {
                let duration = start_time.elapsed();
                self.sections.insert(name.to_string(), duration);
                self.current_section = None;
            }
        }
    }

    pub fn stop(&self) -> PerformanceReport {
        let total_duration = self.start_time.map(|t| t.elapsed()).unwrap_or_default();
        let slowest_section = self
            .sections
            .iter()
            .max_by_key(|(_, duration)| *duration)
            .map(|(name, _)| name.clone())
            .unwrap_or_default();

        PerformanceReport {
            sections: self.sections.clone(),
            total_duration,
            slowest_section,
        }
    }

    pub fn get_optimization_suggestions(&self, _report: &PerformanceReport) -> Vec<String> {
        vec![
            "考虑使用CSS变量减少重复计算".to_string(),
            "启用样式缓存以提高性能".to_string(),
            "使用增量编译减少构建时间".to_string(),
        ]
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub css_entries: usize,
    pub compilation_entries: usize,
    pub hits: usize,
    pub misses: usize,
    pub size_bytes: usize,
}

impl CacheStatistics {
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

/// 缓存管理器
#[derive(Debug)]
pub struct CacheManager {
    css_cache: HashMap<String, String>,
    compilation_cache: HashMap<String, String>,
    stats: CacheStatistics,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            css_cache: HashMap::new(),
            compilation_cache: HashMap::new(),
            stats: CacheStatistics {
                css_entries: 0,
                compilation_entries: 0,
                hits: 0,
                misses: 0,
                size_bytes: 0,
            },
        }
    }

    pub fn store_css(&mut self, key: &str, value: &str) {
        self.css_cache.insert(key.to_string(), value.to_string());
        self.stats.css_entries = self.css_cache.len();
        self.update_size();
    }

    pub fn get_css(&mut self, key: &str) -> Option<String> {
        if let Some(value) = self.css_cache.get(key) {
            self.stats.hits += 1;
            Some(value.clone())
        } else {
            self.stats.misses += 1;
            None
        }
    }

    pub fn store_compilation_result(&mut self, source: &str, result: &str) {
        self.compilation_cache
            .insert(source.to_string(), result.to_string());
        self.stats.compilation_entries = self.compilation_cache.len();
        self.update_size();
    }

    pub fn get_compilation_result(&mut self, source: &str) -> Option<String> {
        if let Some(result) = self.compilation_cache.get(source) {
            self.stats.hits += 1;
            Some(result.clone())
        } else {
            self.stats.misses += 1;
            None
        }
    }

    pub fn get_statistics(&self) -> CacheStatistics {
        self.stats.clone()
    }

    pub fn cleanup_expired(&mut self) -> usize {
        // 模拟清理过期条目
        let cleaned = 2;
        self.update_size();
        cleaned
    }

    pub fn optimize(&mut self) {
        // 模拟缓存优化
        self.update_size();
    }

    fn update_size(&mut self) {
        self.stats.size_bytes = (self.css_cache.len() + self.compilation_cache.len()) * 100;
    }
}

/// 构建优化器
#[derive(Debug)]
pub struct BuildOptimizer {
    config: BuildOptimizerConfig,
}

#[derive(Debug, Clone)]
pub struct BuildOptimizerConfig {
    pub enable_dead_code_elimination: bool,
    pub enable_css_minification: bool,
    pub enable_tree_shaking: bool,
}

impl Default for BuildOptimizerConfig {
    fn default() -> Self {
        Self {
            enable_dead_code_elimination: true,
            enable_css_minification: true,
            enable_tree_shaking: true,
        }
    }
}

impl BuildOptimizer {
    pub fn new() -> Self {
        Self {
            config: BuildOptimizerConfig::default(),
        }
    }

    pub fn with_config(config: BuildOptimizerConfig) -> Self {
        Self { config }
    }

    pub async fn optimize_css(&self, css: &str) -> String {
        sleep(Duration::from_millis(10)).await;
        if self.config.enable_css_minification {
            css.replace("  ", "").replace("\n", "")
        } else {
            css.to_string()
        }
    }

    pub async fn eliminate_dead_code(&self, css: &str) -> String {
        sleep(Duration::from_millis(5)).await;
        if self.config.enable_dead_code_elimination {
            // 模拟死代码消除
            css.lines()
                .filter(|line| !line.trim().starts_with("/* unused */"))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            css.to_string()
        }
    }

    pub async fn tree_shake(&self, css: &str) -> String {
        sleep(Duration::from_millis(8)).await;
        if self.config.enable_tree_shaking {
            // 模拟树摇优化
            css.to_string()
        } else {
            css.to_string()
        }
    }

    pub fn get_optimization_report(&self) -> OptimizationReport {
        OptimizationReport {
            original_size: 1024,
            optimized_size: 768,
            compression_ratio: 0.75,
            eliminated_rules: 15,
            processing_time: Duration::from_millis(50),
        }
    }

    pub fn deduplicate_styles(&self, css: &str) -> String {
        // 模拟样式去重
        css.to_string()
    }

    pub fn compress_css(&self, original_size: usize) -> usize {
        // 模拟CSS压缩，返回压缩后的大小
        (original_size as f64 * 0.7) as usize
    }

    pub fn extract_critical_css(&self, css: &str) -> String {
        // 模拟提取关键CSS
        css.lines().take(5).collect::<Vec<_>>().join("\n")
    }

    pub fn chunk_css(&self, css: &str, chunk_count: usize) -> Vec<String> {
        // 模拟CSS分块
        let lines: Vec<&str> = css.lines().collect();
        let chunk_size = lines.len() / chunk_count;
        (0..chunk_count)
            .map(|i| {
                let start = i * chunk_size;
                let end = if i == chunk_count - 1 {
                    lines.len()
                } else {
                    (i + 1) * chunk_size
                };
                lines[start..end].join("\n")
            })
            .collect()
    }

    pub fn get_build_statistics(&self) -> BuildStatistics {
        BuildStatistics {
            total_files: 25,
            processed_files: 25,
            total_css_rules: 156,
            eliminated_rules: 23,
            compression_ratio: 0.72,
            build_time: Duration::from_millis(1200),
            output_size_bytes: 1024 * 768, // 768KB
            files_processed: 25,
            css_rules_generated: 156,
            dead_code_eliminated: 23,
            duplicates_merged: 12,
            total_build_time: Duration::from_millis(1200),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildStatistics {
    pub total_files: usize,
    pub processed_files: usize,
    pub total_css_rules: usize,
    pub eliminated_rules: usize,
    pub compression_ratio: f64,
    pub build_time: Duration,
    pub output_size_bytes: usize,
    pub files_processed: usize,
    pub css_rules_generated: usize,
    pub dead_code_eliminated: usize,
    pub duplicates_merged: usize,
    pub total_build_time: Duration,
}

#[derive(Debug, Clone)]
pub struct OptimizationReport {
    pub original_size: usize,
    pub optimized_size: usize,
    pub compression_ratio: f64,
    pub eliminated_rules: usize,
    pub processing_time: Duration,
}

/// 运行时监控器
#[derive(Debug)]
pub struct RuntimeMonitor {
    metrics: Vec<RuntimeMetric>,
}

#[derive(Debug, Clone)]
pub struct RuntimeMetric {
    pub timestamp: Instant,
    pub metric_type: String,
    pub value: f64,
}

impl RuntimeMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    pub fn record_metric(&mut self, metric_type: &str, value: f64) {
        self.metrics.push(RuntimeMetric {
            timestamp: Instant::now(),
            metric_type: metric_type.to_string(),
            value,
        });
    }

    pub fn get_metrics(&self) -> &[RuntimeMetric] {
        &self.metrics
    }

    pub fn get_average(&self, metric_type: &str) -> f64 {
        let values: Vec<f64> = self
            .metrics
            .iter()
            .filter(|m| m.metric_type == metric_type)
            .map(|m| m.value)
            .collect();

        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    }

    pub fn generate_report(&self) -> RuntimeReport {
        RuntimeReport {
            total_metrics: self.metrics.len(),
            avg_render_time: self.get_average("render_time"),
            avg_style_calculation: self.get_average("style_calculation"),
            avg_layout_time: self.get_average("layout_time"),
            slowest_application: Some("样式应用".to_string()),
            current_memory_mb: self.get_average("memory_usage") / (1024.0 * 1024.0),
            peak_memory_mb: 15.2,
            average_memory_mb: 12.8,
            style_applications: self
                .metrics
                .iter()
                .filter(|m| m.metric_type == "style_application")
                .count(),
            average_application_time: self.get_average("style_application"),
        }
    }

    pub fn start(&mut self) {
        // 模拟开始监控
        self.record_metric("monitor_start", 1.0);
    }

    pub fn stop(&mut self) {
        // 模拟停止监控
        self.record_metric("monitor_stop", 1.0);
    }

    pub fn record_style_application(&mut self, time_ms: f64) {
        self.record_metric("style_application", time_ms);
    }

    pub fn record_memory_usage(&mut self, memory_bytes: usize) {
        self.record_metric("memory_usage", memory_bytes as f64);
    }

    pub fn get_statistics(&self) -> RuntimeReport {
        self.generate_report()
    }

    pub fn get_performance_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        let avg_render = self.get_average("render_time");
        if avg_render > 16.0 {
            warnings.push(format!("渲染时间过长: {:.2}ms (建议 < 16ms)", avg_render));
        }

        let avg_style = self.get_average("style_calculation");
        if avg_style > 5.0 {
            warnings.push(format!("样式计算时间过长: {:.2}ms (建议 < 5ms)", avg_style));
        }

        warnings
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeReport {
    pub total_metrics: usize,
    pub avg_render_time: f64,
    pub avg_style_calculation: f64,
    pub avg_layout_time: f64,
    pub slowest_application: Option<String>,
    pub current_memory_mb: f64,
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub style_applications: usize,
    pub average_application_time: f64,
}

/// 内存优化器
#[derive(Debug)]
pub struct MemoryOptimizer {
    allocated_memory: usize,
    peak_memory: usize,
}

impl MemoryOptimizer {
    pub fn new() -> Self {
        Self {
            allocated_memory: 0,
            peak_memory: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) {
        self.allocated_memory += size;
        if self.allocated_memory > self.peak_memory {
            self.peak_memory = self.allocated_memory;
        }
    }

    pub fn deallocate(&mut self, size: usize) {
        if self.allocated_memory >= size {
            self.allocated_memory -= size;
        }
    }

    pub fn get_current_usage(&self) -> usize {
        self.allocated_memory
    }

    pub fn get_peak_usage(&self) -> usize {
        self.peak_memory
    }

    pub async fn optimize(&mut self) {
        sleep(Duration::from_millis(20)).await;
        // 模拟内存优化
        self.allocated_memory = (self.allocated_memory as f64 * 0.8) as usize;
    }

    pub fn get_memory_report(&self) -> MemoryReport {
        MemoryReport {
            current_usage: self.allocated_memory,
            peak_usage: self.peak_memory,
            fragmentation_ratio: 0.15,
            gc_cycles: 5,
        }
    }

    pub fn cleanup_cache(&mut self) -> usize {
        // 模拟缓存清理
        let freed = 1024 * 512; // 512KB
        self.deallocate(freed);
        freed
    }

    pub fn cleanup_unused_styles(&mut self) -> usize {
        // 模拟清理未使用样式
        let freed = 1024 * 256; // 256KB
        self.deallocate(freed);
        freed
    }

    pub fn optimize_string_pool(&mut self) -> usize {
        // 模拟字符串池优化
        let freed = 1024 * 128; // 128KB
        self.deallocate(freed);
        freed
    }

    pub fn garbage_collect(&mut self) -> usize {
        // 模拟垃圾回收
        let freed = 1024 * 64; // 64KB
        self.deallocate(freed);
        freed
    }

    pub fn get_memory_suggestions(&self) -> Vec<String> {
        vec![
            "考虑启用样式缓存以减少内存使用".to_string(),
            "定期清理未使用的CSS规则".to_string(),
            "使用字符串池优化重复字符串".to_string(),
            "启用增量垃圾回收".to_string(),
        ]
    }

    pub fn set_memory_threshold(&mut self, _threshold: usize) {
        // 模拟设置内存阈值
    }

    pub fn check_memory_threshold(&self, memory_usage: usize) -> bool {
        // 模拟检查内存阈值，假设阈值为10MB
        memory_usage > 10 * 1024 * 1024
    }

    pub fn optimize_style_cache(&mut self) -> usize {
        // 模拟样式缓存优化
        let freed = 1024 * 1024; // 1MB
        self.deallocate(freed);
        freed
    }
}

#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub current_usage: usize,
    pub peak_usage: usize,
    pub fragmentation_ratio: f64,
    pub gc_cycles: usize,
}

/// 性能演示主函数
pub fn run_performance_demo() {
    println!("⚡ 性能优化系统演示");
    println!("=====================");
    println!();

    // 演示编译时优化
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_performance_metrics());

    // 演示死代码消除
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_performance_profiler());

    // 演示 CSS 压缩
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_cache_manager());

    // 演示静态分析
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_build_optimizer());

    // 演示缓存策略
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_runtime_monitor());

    // 演示运行时性能
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_memory_optimizer());

    // 演示构建优化
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_comprehensive_analysis());

    // 演示性能监控
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(test_performance_metrics());

    println!("✅ 性能优化系统演示完成！");
    println!();
}

fn main() {
    run_performance_demo();
}

/// 测试性能指标收集
async fn test_performance_metrics() {
    println!("\n--- 测试性能指标收集 ---");

    // 创建性能指标收集器
    let mut metrics = PerformanceMetrics::new();

    println!("开始收集性能指标...");

    // 模拟CSS编译性能测试
    let start_time = Instant::now();

    // 生成一些测试样式来模拟编译过程
    let styles = vec![
        css!("color: red; font-size: 14px; padding: 8px;"),
        css!("background: blue; margin: 16px; border-radius: 4px;"),
        css!("display: flex; justify-content: center; align-items: center;"),
        css!("position: absolute; top: 0; left: 0; width: 100%; height: 100%;"),
        css!("transform: translateX(-50%) translateY(-50%); opacity: 0.8;"),
    ];

    let compile_time = start_time.elapsed();
    metrics.record_compile_time(compile_time);

    println!("CSS编译时间: {:?}", compile_time);
    println!("生成的样式数量: {}", styles.len());

    // 模拟样式注入性能
    let injection_start = Instant::now();

    // 模拟DOM操作延迟
    sleep(Duration::from_millis(10)).await;

    let injection_time = injection_start.elapsed();
    metrics.record_injection_time(injection_time);

    println!("样式注入时间: {:?}", injection_time);

    // 模拟缓存命中率
    let cache_hits = 85;
    let cache_misses = 15;
    let cache_hit_rate = cache_hits as f64 / (cache_hits + cache_misses) as f64;

    metrics.record_cache_hit_rate(cache_hit_rate);
    println!("缓存命中率: {:.1}%", cache_hit_rate * 100.0);

    // 模拟内存使用
    let memory_usage = 1024 * 1024 * 2; // 2MB
    metrics.record_memory_usage(memory_usage);
    println!("内存使用: {:.2}MB", memory_usage as f64 / (1024.0 * 1024.0));

    // 模拟CSS规则数量
    let css_rules_count = 156;
    metrics.record_css_rules_count(css_rules_count);
    println!("CSS规则数量: {}", css_rules_count);

    // 显示汇总指标
    println!("\n性能指标汇总:");
    println!("  平均编译时间: {:?}", metrics.average_compile_time());
    println!("  平均注入时间: {:?}", metrics.average_injection_time());
    println!(
        "  总体缓存命中率: {:.1}%",
        metrics.overall_cache_hit_rate() * 100.0
    );
    println!(
        "  峰值内存使用: {:.2}MB",
        metrics.peak_memory_usage() as f64 / (1024.0 * 1024.0)
    );
}

/// 测试性能分析器
async fn test_performance_profiler() {
    println!("\n--- 测试性能分析器 ---");

    // 创建性能分析器
    let mut profiler = PerformanceProfiler::new();

    println!("启动性能分析器...");
    profiler.start();

    // 模拟不同的操作并进行性能分析

    // 1. CSS解析性能
    profiler.start_section("css_parsing");

    let complex_css = css!(
        r#"
        .complex-component {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 1rem;
            padding: 2rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border-radius: 12px;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        }

        .complex-component:hover {
            transform: translateY(-5px);
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
        }

        @media (max-width: 768px) {
            .complex-component {
                grid-template-columns: 1fr;
                padding: 1rem;
            }
        }
    "#
    );

    profiler.end_section("css_parsing");
    println!("复杂CSS解析完成，类名: {}", complex_css);

    // 2. 样式计算性能
    profiler.start_section("style_calculation");

    // 模拟样式计算过程
    let mut calculated_styles = HashMap::new();
    calculated_styles.insert("width", "100%");
    calculated_styles.insert("height", "auto");
    calculated_styles.insert("margin", "0 auto");
    calculated_styles.insert("padding", "16px");

    sleep(Duration::from_millis(5)).await;

    profiler.end_section("style_calculation");
    println!("样式计算完成，计算属性数量: {}", calculated_styles.len());

    // 3. DOM更新性能
    profiler.start_section("dom_update");

    // 模拟DOM更新延迟
    sleep(Duration::from_millis(8)).await;

    profiler.end_section("dom_update");
    println!("DOM更新完成");

    // 停止分析器并获取报告
    let report = profiler.stop();

    println!("\n性能分析报告:");
    for (section, duration) in &report.sections {
        println!("  {}: {:?}", section, duration);
    }

    println!("  总执行时间: {:?}", report.total_duration);
    println!("  最慢的操作: {}", report.slowest_section);

    // 性能建议
    let suggestions = profiler.get_optimization_suggestions(&report);
    if !suggestions.is_empty() {
        println!("\n优化建议:");
        for suggestion in suggestions {
            println!("  💡 {}", suggestion);
        }
    }
}

/// 测试缓存管理
async fn test_cache_manager() {
    println!("\n--- 测试缓存管理 ---");

    // 创建缓存管理器
    let mut cache_manager = CacheManager::new();

    println!("初始化缓存管理器...");

    // 测试CSS缓存
    let css_key = "button-primary";
    let css_value = css!(
        "background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px;"
    );

    // 存储到缓存
    cache_manager.store_css(css_key, &css_value);
    println!("存储CSS到缓存: {} -> {}", css_key, css_value);

    // 从缓存读取
    if let Some(cached_css) = cache_manager.get_css(css_key) {
        println!("从缓存读取CSS: {}", cached_css);
    }

    // 测试编译结果缓存
    let source_code = r#"
        let button_style = css!("background: red; color: white;");
    "#;

    let compiled_result = "/* 编译后的CSS */\n.css-abc123 { background: red; color: white; }";

    cache_manager.store_compilation_result(source_code, compiled_result);
    println!("\n存储编译结果到缓存");

    if let Some(cached_result) = cache_manager.get_compilation_result(source_code) {
        println!("从缓存读取编译结果: {}", cached_result);
    }

    // 测试缓存统计
    let stats = cache_manager.get_statistics();
    println!("\n缓存统计信息:");
    println!("  CSS缓存条目: {}", stats.css_entries);
    println!("  编译缓存条目: {}", stats.compilation_entries);
    println!("  缓存命中次数: {}", stats.hits);
    println!("  缓存未命中次数: {}", stats.misses);
    println!("  缓存命中率: {:.1}%", stats.hit_rate() * 100.0);
    println!("  缓存大小: {:.2}KB", stats.size_bytes as f64 / 1024.0);

    // 测试缓存清理
    println!("\n执行缓存清理...");
    let cleaned_entries = cache_manager.cleanup_expired();
    println!("清理了 {} 个过期缓存条目", cleaned_entries);

    // 测试缓存优化
    cache_manager.optimize();
    println!("缓存优化完成");

    let optimized_stats = cache_manager.get_statistics();
    println!(
        "优化后缓存大小: {:.2}KB",
        optimized_stats.size_bytes as f64 / 1024.0
    );
}

/// 测试构建优化
async fn test_build_optimizer() {
    println!("\n--- 测试构建优化 ---");

    // 创建构建优化器
    let optimizer = BuildOptimizer::new();

    println!("初始化构建优化器...");

    // 模拟CSS代码
    let css_code = vec![
        css!("color: red; font-size: 14px; margin: 8px;"),
        css!("background: blue; padding: 16px; border-radius: 4px;"),
        css!("display: flex; justify-content: center; align-items: center;"),
        css!("position: relative; width: 100%; height: auto;"),
        css!("color: red; font-size: 14px; margin: 8px;"), // 重复样式
    ];

    println!("原始CSS样式数量: {}", css_code.len());

    // 1. 死代码消除
    let css_string = css_code.join("\n");
    let after_dce = optimizer.eliminate_dead_code(&css_string).await;
    println!("\n死代码消除后样式长度: {}", after_dce.len());

    // 2. 重复样式合并
    let after_dedup = optimizer.deduplicate_styles(&after_dce);
    println!("重复样式合并后数量: {}", after_dedup.len());

    // 3. CSS压缩
    let original_size = 1024; // 模拟原始大小
    let compressed_size = optimizer.compress_css(original_size);
    let compression_ratio = (original_size - compressed_size) as f64 / original_size as f64;

    println!("\nCSS压缩结果:");
    println!("  原始大小: {}B", original_size);
    println!("  压缩后大小: {}B", compressed_size);
    println!("  压缩率: {:.1}%", compression_ratio * 100.0);

    // 4. 关键CSS提取
    let critical_css = optimizer.extract_critical_css(&after_dedup);
    println!("\n关键CSS提取:");
    println!("  关键样式数量: {}", critical_css.len());

    // 5. CSS分块
    let chunks = optimizer.chunk_css(&after_dedup, 2);
    println!("\nCSS分块结果:");
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  块 {}: {} 个样式", i + 1, chunk.len());
    }

    // 6. 构建统计
    let build_stats = optimizer.get_build_statistics();
    println!("\n构建统计:");
    println!("  处理的文件数: {}", build_stats.files_processed);
    println!("  生成的CSS规则: {}", build_stats.css_rules_generated);
    println!("  消除的死代码: {}", build_stats.dead_code_eliminated);
    println!("  合并的重复样式: {}", build_stats.duplicates_merged);
    println!("  总构建时间: {:?}", build_stats.total_build_time);
    println!(
        "  输出文件大小: {:.2}KB",
        build_stats.output_size_bytes as f64 / 1024.0
    );
}

/// 测试运行时监控
async fn test_runtime_monitor() {
    println!("\n--- 测试运行时监控 ---");

    // 创建运行时监控器
    let mut monitor = RuntimeMonitor::new();

    println!("启动运行时监控...");
    monitor.start();

    // 模拟运行时操作
    for i in 0..10 {
        // 模拟样式应用
        let start = Instant::now();

        let dynamic_style = css!(format!(
            "color: hsl({}, 70%, 50%); transform: rotate({}deg);",
            i * 36,
            i * 10
        )
        .as_str());

        let apply_time = start.elapsed();
        monitor.record_style_application(apply_time.as_millis() as f64);

        println!(
            "应用动态样式 {}: {} (耗时: {:?})",
            i + 1,
            dynamic_style,
            apply_time
        );

        // 模拟DOM更新延迟
        sleep(Duration::from_millis(2)).await;
    }

    // 模拟内存使用监控
    let memory_samples = vec![1.2, 1.5, 1.8, 2.1, 1.9, 1.6, 1.4, 1.3, 1.1, 1.0];
    for (i, &memory_mb) in memory_samples.iter().enumerate() {
        let memory_bytes = (memory_mb * 1024.0 * 1024.0) as usize;
        monitor.record_memory_usage(memory_bytes);
        println!("内存使用样本 {}: {:.1}MB", i + 1, memory_mb);
    }

    // 获取运行时统计
    let runtime_stats = monitor.get_statistics();

    println!("\n运行时统计:");
    println!("  样式应用次数: {}", runtime_stats.style_applications);
    println!(
        "  平均应用时间: {:?}",
        runtime_stats.average_application_time
    );
    println!("  最慢的应用: {:?}", runtime_stats.slowest_application);
    println!("  当前内存使用: {:.2}MB", runtime_stats.current_memory_mb);
    println!("  峰值内存使用: {:.2}MB", runtime_stats.peak_memory_mb);
    println!("  平均内存使用: {:.2}MB", runtime_stats.average_memory_mb);

    // 性能警告
    let warnings = monitor.get_performance_warnings();
    if !warnings.is_empty() {
        println!("\n性能警告:");
        for warning in warnings {
            println!("  ⚠️  {}", warning);
        }
    }

    monitor.stop();
    println!("\n运行时监控已停止");
}

/// 测试内存优化
async fn test_memory_optimizer() {
    println!("\n--- 测试内存优化 ---");

    // 创建内存优化器
    let mut optimizer = MemoryOptimizer::new();

    println!("初始化内存优化器...");

    // 模拟内存使用情况
    let initial_memory = 5 * 1024 * 1024; // 5MB
    println!(
        "初始内存使用: {:.2}MB",
        initial_memory as f64 / (1024.0 * 1024.0)
    );

    // 1. 样式缓存优化
    println!("\n执行样式缓存优化...");
    let cache_freed = optimizer.optimize_style_cache();
    println!("释放缓存内存: {:.2}KB", cache_freed as f64 / 1024.0);

    // 2. 未使用样式清理
    println!("\n清理未使用样式...");
    let unused_freed = optimizer.cleanup_unused_styles();
    println!("释放未使用样式内存: {:.2}KB", unused_freed as f64 / 1024.0);

    // 3. 字符串池优化
    println!("\n优化字符串池...");
    let string_pool_freed = optimizer.optimize_string_pool();
    println!(
        "释放字符串池内存: {:.2}KB",
        string_pool_freed as f64 / 1024.0
    );

    // 4. 垃圾回收
    println!("\n执行垃圾回收...");
    let gc_freed = optimizer.garbage_collect();
    println!("垃圾回收释放内存: {:.2}KB", gc_freed as f64 / 1024.0);

    // 计算总优化效果
    let total_freed = cache_freed + unused_freed + string_pool_freed + gc_freed;
    let final_memory = initial_memory - total_freed;
    let optimization_ratio = total_freed as f64 / initial_memory as f64;

    println!("\n内存优化结果:");
    println!(
        "  优化前内存: {:.2}MB",
        initial_memory as f64 / (1024.0 * 1024.0)
    );
    println!(
        "  优化后内存: {:.2}MB",
        final_memory as f64 / (1024.0 * 1024.0)
    );
    println!("  释放内存总量: {:.2}KB", total_freed as f64 / 1024.0);
    println!("  优化率: {:.1}%", optimization_ratio * 100.0);

    // 内存使用建议
    let suggestions = optimizer.get_memory_suggestions();
    if !suggestions.is_empty() {
        println!("\n内存优化建议:");
        for suggestion in suggestions {
            println!("  💡 {}", suggestion);
        }
    }

    // 设置内存监控阈值
    optimizer.set_memory_threshold(10 * 1024 * 1024); // 10MB
    println!("\n设置内存监控阈值: 10MB");

    // 模拟内存压力测试
    println!("\n执行内存压力测试...");
    for i in 1..=5 {
        let test_memory = initial_memory + (i * 2 * 1024 * 1024);
        let is_over_threshold = optimizer.check_memory_threshold(test_memory);

        println!(
            "  测试 {}: {:.2}MB - {}",
            i,
            test_memory as f64 / (1024.0 * 1024.0),
            if is_over_threshold {
                "超出阈值"
            } else {
                "正常"
            }
        );

        if is_over_threshold {
            println!("    触发自动内存优化...");
        }
    }
}

/// 测试综合性能分析
async fn test_comprehensive_analysis() {
    println!("\n--- 测试综合性能分析 ---");

    // 创建综合分析器
    let mut analyzer = PerformanceProfiler::new();

    println!("开始综合性能分析...");
    analyzer.start();

    // 模拟完整的CSS-in-Rust工作流程

    // 1. 项目初始化
    analyzer.start_section("project_init");
    sleep(Duration::from_millis(50)).await;
    analyzer.end_section("project_init");

    // 2. CSS编译
    analyzer.start_section("css_compilation");

    let test_styles = vec![
        css!("display: flex; flex-direction: column; gap: 1rem;"),
        css!("background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px;"),
        css!("padding: 1rem; margin: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1);"),
        css!("font-family: 'Inter', sans-serif; font-size: 1rem; line-height: 1.5;"),
        css!("transition: all 0.3s ease; transform: scale(1); opacity: 1;"),
    ];

    sleep(Duration::from_millis(80)).await;
    analyzer.end_section("css_compilation");

    // 3. 样式注入
    analyzer.start_section("style_injection");
    sleep(Duration::from_millis(20)).await;
    analyzer.end_section("style_injection");

    // 4. 缓存操作
    analyzer.start_section("cache_operations");
    sleep(Duration::from_millis(15)).await;
    analyzer.end_section("cache_operations");

    // 5. DOM更新
    analyzer.start_section("dom_updates");
    sleep(Duration::from_millis(25)).await;
    analyzer.end_section("dom_updates");

    // 获取综合分析报告
    let comprehensive_report = analyzer.stop();

    println!("\n=== 综合性能分析报告 ===");

    // 各阶段性能
    println!("\n各阶段性能:");
    let mut total_time = Duration::new(0, 0);
    for (section, duration) in &comprehensive_report.sections {
        println!("  {}: {:?}", section, duration);
        total_time += *duration;
    }

    // 性能分布
    println!("\n性能分布:");
    for (section, duration) in &comprehensive_report.sections {
        let percentage = duration.as_millis() as f64 / total_time.as_millis() as f64 * 100.0;
        println!("  {}: {:.1}%", section, percentage);
    }

    // 性能等级评估
    let performance_grade = if total_time.as_millis() < 100 {
        "A+ (优秀)"
    } else if total_time.as_millis() < 200 {
        "A (良好)"
    } else if total_time.as_millis() < 300 {
        "B (一般)"
    } else {
        "C (需要优化)"
    };

    println!("\n性能评估:");
    println!("  总执行时间: {:?}", total_time);
    println!("  性能等级: {}", performance_grade);
    println!("  生成样式数量: {}", test_styles.len());
    println!(
        "  平均每个样式耗时: {:?}",
        total_time / test_styles.len() as u32
    );

    // 性能建议
    println!("\n性能优化建议:");

    if comprehensive_report
        .sections
        .get("css_compilation")
        .unwrap()
        .as_millis()
        > 50
    {
        println!("  🔧 CSS编译时间较长，建议启用编译缓存");
    }

    if comprehensive_report
        .sections
        .get("dom_updates")
        .unwrap()
        .as_millis()
        > 20
    {
        println!("  ⚡ DOM更新频繁，建议使用批量更新");
    }

    if total_time.as_millis() > 200 {
        println!("  📊 总体性能有待提升，建议进行性能分析");
    } else {
        println!("  ✅ 性能表现良好，继续保持");
    }

    // 资源使用情况
    println!("\n资源使用情况:");
    println!(
        "  估计内存使用: {:.2}MB",
        (test_styles.len() * 1024) as f64 / (1024.0 * 1024.0)
    );
    println!(
        "  估计CPU使用: {:.1}%",
        total_time.as_millis() as f64 / 10.0
    );
    println!("  缓存效率: 85.2%");

    println!("\n综合性能分析完成");
}
