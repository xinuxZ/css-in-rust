# 性能优化指南

本指南将帮助您最大化 CSS-in-Rust 的性能，涵盖编译时优化、运行时优化和最佳实践。

## 📊 性能概览

CSS-in-Rust 在多个层面提供性能优化：

- **编译时优化**: 死代码消除、CSS 压缩、静态分析
- **运行时优化**: 智能缓存、懒加载、样式去重
- **构建优化**: 增量编译、并行处理、缓存策略
- **网络优化**: CSS 分块、压缩传输、CDN 支持

## ⚡ 编译时优化

### 1. 死代码消除

自动移除未使用的 CSS 规则，显著减少最终包大小。

```rust
// build.rs
use css_in_rust::build_tools::{CssBuildProcessor, BuildConfig};

fn main() {
    let config = BuildConfig {
        project_root: std::env::var("CARGO_MANIFEST_DIR").unwrap().into(),
        output_dir: "dist".into(),

        // 启用死代码消除
        enable_dead_code_elimination: true,

        // 设置使用阈值 (0.0 = 移除所有未使用的样式)
        usage_threshold: 0.0,

        // 激进模式 (更彻底的优化)
        aggressive_elimination: true,

        enable_minification: true,
        generate_report: true,
    };

    let processor = CssBuildProcessor::new(config);

    match processor.run() {
        Ok(result) => {
            println!("✅ 优化完成");
            println!("📦 原始大小: {} KB", result.total_size_before / 1024);
            println!("📦 优化后大小: {} KB", result.total_size_after / 1024);
            println!("🗜️ 压缩率: {:.1}%", result.compression_ratio * 100.0);
        }
        Err(e) => {
            eprintln!("❌ 优化失败: {}", e);
            std::process::exit(1);
        }
    }

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=styles/");
}
```

### 2. CSS 压缩和优化

```rust
use css_in_rust::css;

// 编译时会自动优化这些样式
let optimized_style = css! {
    // 冗余属性会被合并
    margin-top: 10px;
    margin-right: 10px;
    margin-bottom: 10px;
    margin-left: 10px;
    // 优化后: margin: 10px;

    // 颜色值会被优化
    color: #ffffff; // 优化后: color: #fff;
    background-color: rgba(255, 255, 255, 1.0); // 优化后: background-color: #fff;

    // 无效或重复的规则会被移除
    display: block;
    display: flex; // 只保留最后一个

    // 厂商前缀会被自动添加
    transform: translateX(10px);
    // 自动添加: -webkit-transform: translateX(10px);
};
```

### 3. 静态分析优化

```rust
use css_in_rust::build_tools::StaticAnalyzer;
use std::path::PathBuf;

// 分析代码中的 CSS 使用情况
let analyzer = StaticAnalyzer::new(PathBuf::from("./"))
    .with_include_patterns(vec![
        "src/**/*.rs".to_string(),
        "components/**/*.rs".to_string(),
    ])
    .with_exclude_patterns(vec![
        "target/**".to_string(),
        "tests/**".to_string(),
    ])
    .with_analyze_dependencies(true);

let report = analyzer.analyze().expect("分析失败");

println!("📊 分析报告:");
println!("  - 分析文件数: {}", report.analyzed_files.len());
println!("  - CSS 宏调用: {}", report.css_macro_calls.len());
println!("  - 使用的类: {}", report.used_classes.len());
println!("  - 使用的 ID: {}", report.used_ids.len());

// 基于分析结果优化
if report.used_classes.len() < 100 {
    println!("💡 建议: 考虑使用内联样式减少类生成开销");
}

if report.css_macro_calls.len() > 1000 {
    println!("⚠️ 警告: CSS 宏调用过多，考虑使用变体系统");
}
```

## 🏃 运行时优化

### 1. 智能缓存策略

```rust
use css_in_rust::runtime::{StyleManager, StyleManagerConfig};

// 配置高性能缓存
let config = StyleManagerConfig {
    enable_caching: true,
    cache_size: 5000,  // 增加缓存大小
    enable_deduplication: true,  // 启用样式去重
    enable_compression: true,    // 启用压缩
    lazy_loading: true,          // 启用懒加载
};

let style_manager = StyleManager::with_config(config);

// 预加载关键样式
style_manager.preload_critical_styles(&[
    "button",
    "card",
    "layout",
    "typography",
]);

// 监控缓存性能
let cache_stats = style_manager.get_cache_stats();
println!("缓存命中率: {:.2}%", cache_stats.hit_rate * 100.0);
println!("缓存大小: {} / {}", cache_stats.current_size, cache_stats.max_size);

// 如果命中率低于 80%，考虑调整缓存策略
if cache_stats.hit_rate < 0.8 {
    style_manager.optimize_cache();
}
```

### 2. 懒加载和按需注入

```rust
use css_in_rust::{css, runtime::LazyStyle};
use std::sync::LazyLock;

// 使用 LazyLock 延迟计算样式
static HEAVY_STYLE: LazyLock<String> = LazyLock::new(|| {
    css! {
        // 复杂的样式计算
        background: linear-gradient(
            45deg,
            #ff6b6b 0%,
            #4ecdc4 25%,
            #45b7d1 50%,
            #96ceb4 75%,
            #ffeaa7 100%
        );

        box-shadow:
            0 4px 6px rgba(0, 0, 0, 0.1),
            0 1px 3px rgba(0, 0, 0, 0.08),
            inset 0 1px 0 rgba(255, 255, 255, 0.1);

        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));

        animation: pulse 2s ease-in-out infinite;

        @keyframes pulse {
            0%, 100% { transform: scale(1); }
            50% { transform: scale(1.05); }
        }
    }.class_name()
});

// 条件加载样式
fn get_component_style(is_visible: bool) -> Option<String> {
    if is_visible {
        Some(HEAVY_STYLE.clone())
    } else {
        None
    }
}

// 异步样式加载
use css_in_rust::runtime::AsyncStyleLoader;

async fn load_theme_styles(theme_name: &str) -> Result<Vec<String>, CssError> {
    let loader = AsyncStyleLoader::new();

    // 并行加载多个样式文件
    let styles = loader.load_styles_parallel(&[
        format!("themes/{}/components.css", theme_name),
        format!("themes/{}/layout.css", theme_name),
        format!("themes/{}/utilities.css", theme_name),
    ]).await?;

    Ok(styles)
}
```

### 3. 样式去重和合并

```rust
use css_in_rust::runtime::StyleDeduplicator;

// 自动去重相似样式
let deduplicator = StyleDeduplicator::new()
    .with_similarity_threshold(0.8)  // 80% 相似度阈值
    .with_merge_strategy(MergeStrategy::Aggressive);

// 注册样式时自动去重
let button_primary = css! {
    background-color: #007bff;
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
};

let button_secondary = css! {
    background-color: #6c757d;  // 只有这个不同
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
};

// 去重器会自动合并相似样式
let deduplicated = deduplicator.process(&[button_primary, button_secondary]);
println!("原始样式数: 2, 去重后: {}", deduplicated.len());
```

## 🔧 构建优化

### 1. 增量编译

```rust
// css-in-rust.toml
[performance]
enable_incremental = true
incremental_cache_dir = ".cache/css-in-rust"
max_cache_size = "100MB"
cache_compression = true

[build]
parallel_processing = true
max_parallel_jobs = 8  // 根据 CPU 核心数调整
enable_build_cache = true
watch_mode_optimizations = true
```

```rust
// build.rs 中的增量编译配置
use css_in_rust::build_tools::IncrementalBuilder;

fn main() {
    let builder = IncrementalBuilder::new()
        .with_cache_dir(".cache/css-in-rust")
        .with_parallel_jobs(num_cpus::get())
        .with_change_detection(true)
        .with_dependency_tracking(true);

    // 只重新编译变更的文件
    let changed_files = builder.detect_changes()?;

    if changed_files.is_empty() {
        println!("✅ 没有变更，跳过编译");
        return;
    }

    println!("🔄 重新编译 {} 个文件", changed_files.len());

    let start = std::time::Instant::now();
    builder.build_incremental(&changed_files)?;

    println!("⚡ 增量编译完成，耗时: {:?}", start.elapsed());
}
```

### 2. 并行处理

```rust
use css_in_rust::build_tools::ParallelProcessor;
use rayon::prelude::*;

// 并行处理多个 CSS 文件
fn process_styles_parallel(css_files: Vec<PathBuf>) -> Result<Vec<ProcessedFile>, BuildError> {
    css_files
        .par_iter()  // 使用 rayon 并行迭代
        .map(|file| {
            let processor = CssProcessor::new();
            processor.process_file(file)
        })
        .collect()
}

// 并行优化
fn optimize_styles_parallel(styles: Vec<CssStyle>) -> Vec<CssStyle> {
    styles
        .par_iter()
        .map(|style| {
            let optimizer = CssOptimizer::new();
            optimizer.optimize(style.clone())
        })
        .collect()
}

// 并行压缩
fn compress_styles_parallel(styles: Vec<String>) -> Vec<String> {
    styles
        .par_iter()
        .map(|css| {
            let compressor = CssCompressor::new();
            compressor.compress(css)
        })
        .collect()
}
```

### 3. 智能缓存

```rust
use css_in_rust::build_tools::{BuildCache, CacheStrategy};
use std::collections::HashMap;

// 多层缓存策略
let cache = BuildCache::new()
    .with_memory_cache(1000)     // 内存缓存 1000 项
    .with_disk_cache("10GB")     // 磁盘缓存 10GB
    .with_compression(true)      // 启用压缩
    .with_ttl(Duration::from_secs(3600))  // 1小时过期
    .with_strategy(CacheStrategy::LRU);   // LRU 淘汰策略

// 缓存键生成
fn generate_cache_key(css_content: &str, config: &BuildConfig) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    css_content.hash(&mut hasher);
    config.hash(&mut hasher);

    format!("css-{:x}", hasher.finish())
}

// 智能缓存使用
fn build_with_cache(css_content: &str, config: &BuildConfig) -> Result<String, BuildError> {
    let cache_key = generate_cache_key(css_content, config);

    // 尝试从缓存获取
    if let Some(cached_result) = cache.get(&cache_key) {
        println!("✅ 缓存命中: {}", cache_key);
        return Ok(cached_result);
    }

    // 缓存未命中，执行构建
    println!("🔄 缓存未命中，开始构建: {}", cache_key);
    let result = build_css(css_content, config)?;

    // 存储到缓存
    cache.set(cache_key, result.clone());

    Ok(result)
}
```

## 📊 性能监控

### 1. 性能指标收集

```rust
use css_in_rust::performance::{PerformanceManager, MetricsCollector};
use std::time::Instant;

// 创建性能管理器
let perf_manager = PerformanceManager::new()
    .with_metrics_collection(true)
    .with_profiling(true)
    .with_real_time_monitoring(true);

// 收集编译指标
fn compile_with_metrics(css: &str) -> Result<String, CssError> {
    let start = Instant::now();

    // 记录开始
    perf_manager.start_operation("css_compilation");

    // 执行编译
    let result = compile_css(css)?;

    // 记录结束
    let duration = start.elapsed();
    perf_manager.end_operation("css_compilation", duration);

    // 记录额外指标
    perf_manager.record_metric("input_size", css.len() as f64);
    perf_manager.record_metric("output_size", result.len() as f64);
    perf_manager.record_metric("compression_ratio",
        result.len() as f64 / css.len() as f64);

    Ok(result)
}

// 获取性能报告
let metrics = perf_manager.get_metrics();
println!("📊 性能指标:");
println!("  编译次数: {}", metrics.compilation_count);
println!("  平均编译时间: {:?}", metrics.avg_compilation_time);
println!("  缓存命中率: {:.2}%", metrics.cache_hit_rate * 100.0);
println!("  内存使用: {} MB", metrics.memory_usage / 1024 / 1024);
println!("  平均压缩率: {:.2}%", metrics.avg_compression_ratio * 100.0);
```

### 2. 性能分析和调优

```rust
use css_in_rust::performance::PerformanceProfiler;

// 创建性能分析器
let profiler = PerformanceProfiler::new()
    .with_detailed_timing(true)
    .with_memory_tracking(true)
    .with_cpu_profiling(true);

// 开始分析会话
profiler.start_session("css_build_analysis");

// 分析不同阶段的性能
profiler.mark("parsing_start");
let parsed = parse_css(css_input)?;
profiler.mark("parsing_end");

profiler.mark("optimization_start");
let optimized = optimize_css(parsed)?;
profiler.mark("optimization_end");

profiler.mark("generation_start");
let generated = generate_css(optimized)?;
profiler.mark("generation_end");

// 结束分析会话
let report = profiler.end_session();

// 分析瓶颈
println!("🔍 性能分析报告:");
println!("  总耗时: {:?}", report.total_duration);
println!("  峰值内存: {} MB", report.peak_memory / 1024 / 1024);

for bottleneck in &report.bottlenecks {
    println!("  ⚠️ 瓶颈: {} ({:?})", bottleneck.operation, bottleneck.duration);
}

// 自动优化建议
let suggestions = profiler.get_optimization_suggestions(&report);
for suggestion in suggestions {
    println!("  💡 建议: {}", suggestion);
}
```

### 3. 实时性能监控

```rust
use css_in_rust::performance::RealTimeMonitor;
use std::sync::Arc;
use tokio::time::{interval, Duration};

// 创建实时监控器
let monitor = Arc::new(RealTimeMonitor::new());

// 启动监控任务
let monitor_clone = monitor.clone();
tokio::spawn(async move {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let stats = monitor_clone.get_current_stats();

        // 检查性能阈值
        if stats.avg_compilation_time > Duration::from_millis(100) {
            println!("⚠️ 编译时间过长: {:?}", stats.avg_compilation_time);
        }

        if stats.memory_usage > 100 * 1024 * 1024 { // 100MB
            println!("⚠️ 内存使用过高: {} MB", stats.memory_usage / 1024 / 1024);
        }

        if stats.cache_hit_rate < 0.7 {
            println!("⚠️ 缓存命中率过低: {:.2}%", stats.cache_hit_rate * 100.0);
        }
    }
});

// 性能警报
monitor.set_alert_threshold("compilation_time", Duration::from_millis(50));
monitor.set_alert_threshold("memory_usage", 50 * 1024 * 1024); // 50MB
monitor.set_alert_threshold("cache_hit_rate", 0.8);

monitor.on_alert(|alert| {
    println!("🚨 性能警报: {} - {}", alert.metric, alert.message);

    // 自动优化
    match alert.metric.as_str() {
        "compilation_time" => {
            // 启用更激进的缓存
            StyleManager::global().enable_aggressive_caching();
        }
        "memory_usage" => {
            // 清理缓存
            StyleManager::global().cleanup_cache();
        }
        "cache_hit_rate" => {
            // 预热缓存
            StyleManager::global().warmup_cache();
        }
        _ => {}
    }
});
```

## 🎯 最佳实践

### 1. 样式组织

```rust
// ✅ 好的做法：使用静态样式
use std::sync::LazyLock;

static BUTTON_BASE: LazyLock<String> = LazyLock::new(|| {
    css! {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;
    }.class_name()
});

// ✅ 好的做法：使用变体系统
static BUTTON_VARIANTS: LazyLock<CssVariants> = LazyLock::new(|| {
    css_variants! {
        base: BUTTON_BASE.clone(),
        variants: {
            size: {
                sm: { padding: 4px 8px; font-size: 12px; },
                md: { padding: 8px 16px; font-size: 14px; },
                lg: { padding: 12px 24px; font-size: 16px; }
            }
        }
    }
});

// ❌ 避免：在渲染循环中创建样式
fn bad_component(items: &[Item]) -> Html {
    html! {
        <div>
            { for items.iter().map(|item| {
                // 每次渲染都会重新计算样式
                let style = css! {
                    color: if item.is_active { "blue" } else { "gray" };
                };

                html! { <div class={style.class_name()}>{&item.name}</div> }
            }) }
        </div>
    }
}

// ✅ 好的做法：预定义条件样式
static ITEM_ACTIVE: LazyLock<String> = LazyLock::new(|| {
    css! { color: blue; }.class_name()
});

static ITEM_INACTIVE: LazyLock<String> = LazyLock::new(|| {
    css! { color: gray; }.class_name()
});

fn good_component(items: &[Item]) -> Html {
    html! {
        <div>
            { for items.iter().map(|item| {
                let class = if item.is_active {
                    ITEM_ACTIVE.clone()
                } else {
                    ITEM_INACTIVE.clone()
                };

                html! { <div class={class}>{&item.name}</div> }
            }) }
        </div>
    }
}
```

### 2. 缓存策略

```rust
// ✅ 好的做法：分层缓存
use css_in_rust::runtime::CacheManager;

// L1: 内存缓存（最快）
let l1_cache = MemoryCache::new(500);  // 500 个最常用样式

// L2: 本地存储缓存（中等速度）
let l2_cache = LocalStorageCache::new(2000);  // 2000 个样式

// L3: 磁盘缓存（较慢但容量大）
let l3_cache = DiskCache::new("10GB");  // 10GB 磁盘缓存

let cache_manager = CacheManager::new()
    .with_l1_cache(l1_cache)
    .with_l2_cache(l2_cache)
    .with_l3_cache(l3_cache)
    .with_write_through(true)   // 写入时同步到所有层
    .with_read_through(true);   // 读取时自动提升到上层

// 缓存预热
cache_manager.warmup(&[
    "button", "card", "layout", "typography"
]);
```

### 3. 构建优化

```rust
// css-in-rust.toml
[optimization]
# 生产环境配置
dead_code_elimination = true
minification = true
source_maps = false  # 生产环境关闭 source maps
usage_threshold = 0.0
aggressive_elimination = true

# 开发环境配置
[optimization.development]
dead_code_elimination = false  # 开发时保留所有样式便于调试
minification = false
source_maps = true
fast_build = true  # 启用快速构建模式

[performance]
enable_caching = true
cache_size = 10000
incremental_compilation = true
parallel_processing = true
max_parallel_jobs = 8

[hot_reload]
enable = true
port = 3001
debounce_ms = 50  # 减少防抖时间提高响应速度
```

### 4. 内存管理

```rust
use css_in_rust::runtime::MemoryManager;

// 内存监控和自动清理
let memory_manager = MemoryManager::new()
    .with_max_memory(100 * 1024 * 1024)  // 100MB 限制
    .with_cleanup_threshold(0.8)         // 80% 时开始清理
    .with_cleanup_strategy(CleanupStrategy::LRU);

// 定期清理
memory_manager.schedule_cleanup(Duration::from_secs(60));

// 手动清理
if memory_manager.memory_usage() > memory_manager.max_memory() * 0.9 {
    memory_manager.force_cleanup();
}

// 内存使用优化
fn optimize_memory_usage() {
    // 清理未使用的样式
    StyleManager::global().cleanup_unused_styles();

    // 压缩缓存
    StyleManager::global().compress_cache();

    // 释放临时对象
    std::mem::drop(temporary_styles);

    // 强制垃圾回收（如果需要）
    #[cfg(feature = "gc")]
    gc::collect();
}
```

## 📈 性能基准测试

### 1. 编译性能测试

```rust
use css_in_rust::testing::BenchmarkSuite;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_css_compilation(c: &mut Criterion) {
    let css_samples = vec![
        include_str!("../test_data/small.css"),
        include_str!("../test_data/medium.css"),
        include_str!("../test_data/large.css"),
    ];

    c.bench_function("css_compilation_small", |b| {
        b.iter(|| {
            let style = css! {
                background-color: #007bff;
                color: white;
                padding: 8px 16px;
            };
            black_box(style.class_name())
        })
    });

    c.bench_function("css_compilation_complex", |b| {
        b.iter(|| {
            let style = css! {
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                transform: translateX(10px) rotate(5deg);
                animation: pulse 2s ease-in-out infinite;

                &:hover {
                    transform: translateX(15px) rotate(10deg);
                }

                @media (max-width: 768px) {
                    transform: none;
                }
            };
            black_box(style.class_name())
        })
    });
}

criterion_group!(benches, benchmark_css_compilation);
criterion_main!(benches);
```

### 2. 运行时性能测试

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_style_injection_performance() {
        let start = Instant::now();

        // 注入 1000 个样式
        for i in 0..1000 {
            let style = css! {
                color: red;
                margin: ${i}px;
            };
            style.inject().unwrap();
        }

        let duration = start.elapsed();
        assert!(duration < Duration::from_millis(100),
               "样式注入耗时过长: {:?}", duration);
    }

    #[test]
    fn test_cache_performance() {
        let style_manager = StyleManager::global();

        // 预热缓存
        let style = css! { color: blue; };
        let class_name = style.class_name();

        let start = Instant::now();

        // 测试缓存命中性能
        for _ in 0..10000 {
            let cached_style = css! { color: blue; };
            assert_eq!(cached_style.class_name(), class_name);
        }

        let duration = start.elapsed();
        assert!(duration < Duration::from_millis(10),
               "缓存访问耗时过长: {:?}", duration);
    }

    #[test]
    fn test_memory_usage() {
        let initial_memory = get_memory_usage();

        // 创建大量样式
        let mut styles = Vec::new();
        for i in 0..1000 {
            let style = css! {
                width: ${i}px;
                height: ${i}px;
            };
            styles.push(style);
        }

        let peak_memory = get_memory_usage();
        let memory_increase = peak_memory - initial_memory;

        // 清理样式
        drop(styles);
        StyleManager::global().cleanup_cache();

        let final_memory = get_memory_usage();

        assert!(memory_increase < 10 * 1024 * 1024,
               "内存使用过多: {} MB", memory_increase / 1024 / 1024);
        assert!(final_memory - initial_memory < 1024 * 1024,
               "内存泄漏: {} KB", (final_memory - initial_memory) / 1024);
    }
}
```

## 🎯 性能优化检查清单

### ✅ 编译时优化
- [ ] 启用死代码消除
- [ ] 配置 CSS 压缩
- [ ] 使用静态分析
- [ ] 启用增量编译
- [ ] 配置并行处理
- [ ] 设置构建缓存

### ✅ 运行时优化
- [ ] 配置样式缓存
- [ ] 启用样式去重
- [ ] 使用懒加载
- [ ] 预加载关键样式
- [ ] 监控内存使用
- [ ] 定期清理缓存

### ✅ 代码优化
- [ ] 使用静态样式定义
- [ ] 避免在渲染循环中创建样式
- [ ] 使用变体系统
- [ ] 合理组织样式代码
- [ ] 减少样式复杂度

### ✅ 监控和调试
- [ ] 设置性能监控
- [ ] 配置性能警报
- [ ] 定期性能测试
- [ ] 分析性能瓶颈
- [ ] 优化热点代码

通过遵循这些优化策略和最佳实践，您可以确保 CSS-in-Rust 应用具有出色的性能表现！🚀
