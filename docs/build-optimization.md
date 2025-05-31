# 构建工具优化指南

本指南详细介绍 CSS-in-Rust 的构建工具优化策略，帮助您提升构建性能、减少构建时间并优化输出质量。

## 📋 构建优化概览

CSS-in-Rust 提供多层次的构建优化：

- **编译时优化** - 死代码消除、CSS 压缩、静态分析
- **构建流程优化** - 增量编译、并行处理、智能缓存
- **输出优化** - 文件分割、懒加载、压缩优化
- **开发体验优化** - 快速重建、热更新、错误诊断

## 🚀 快速开始

### 1. 基础构建配置

```rust
// build.rs
use css_in_rust::build_tools::{
    CssBuildProcessor, BuildConfig, OptimizationLevel
};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_release = profile == "release";

    let config = BuildConfig {
        project_root: PathBuf::from(env::var("CARGO_MANIFEST_DIR")?),
        output_dir: PathBuf::from("target").join(&profile).join("css"),

        // 基础优化设置
        optimization_level: if is_release {
            OptimizationLevel::Aggressive
        } else {
            OptimizationLevel::Development
        },

        // 启用并行处理
        parallel_processing: true,
        max_parallel_jobs: num_cpus::get(),

        // 启用缓存
        enable_caching: true,
        cache_dir: PathBuf::from(".cache/css-in-rust"),

        ..Default::default()
    };

    let mut processor = CssBuildProcessor::new(config)?;
    let result = processor.run()?;

    println!("构建完成: 处理了 {} 个文件", result.stats.files_processed);

    Ok(())
}
```

### 2. 优化配置文件

```toml
# css-in-rust.toml
[build]
# 优化级别: "development", "balanced", "aggressive"
optimization_level = "aggressive"

# 并行处理
parallel_processing = true
max_parallel_jobs = 8

# 缓存配置
enable_caching = true
cache_strategy = "aggressive"
max_cache_size_mb = 500
cache_ttl_hours = 24

# 增量编译
incremental_compilation = true
track_dependencies = true

# 输出优化
minify_css = true
remove_unused_css = true
generate_source_maps = false

# 文件监控
watch_patterns = [
    "src/**/*.rs",
    "components/**/*.rs",
    "styles/**/*.css"
]

ignore_patterns = [
    "target/**",
    "**/.git/**",
    "**/node_modules/**"
]
```

## ⚡ 编译时优化

### 1. 死代码消除优化

```rust
// 配置死代码消除
use css_in_rust::build_tools::{
    DeadCodeEliminationConfig, UsageAnalyzer
};

/// 配置高级死代码消除
fn configure_dead_code_elimination() -> DeadCodeEliminationConfig {
    DeadCodeEliminationConfig {
        // 启用激进消除模式
        aggressive_elimination: true,

        // 使用率阈值 (低于此值的样式将被移除)
        usage_threshold: 0.05, // 5%

        // 保留关键样式
        preserve_critical_css: true,

        // 分析深度
        analysis_depth: AnalysisDepth::Deep,

        // 跨文件分析
        cross_file_analysis: true,

        // 动态样式检测
        detect_dynamic_styles: true,

        // 保留模式
        preserve_patterns: vec![
            r"^\.(critical|important)-.*".to_string(),
            r"^\.(layout|grid|flex)-.*".to_string(),
        ],

        // 排除模式
        exclude_patterns: vec![
            r"^\.(test|debug)-.*".to_string(),
        ],
    }
}

/// 自定义使用率分析器
struct CustomUsageAnalyzer {
    config: DeadCodeEliminationConfig,
    usage_stats: HashMap<String, UsageStats>,
}

impl CustomUsageAnalyzer {
    /// 分析样式使用情况
    pub fn analyze_usage(&mut self, source_files: &[PathBuf]) -> Result<UsageReport, AnalysisError> {
        let mut report = UsageReport::new();

        for file_path in source_files {
            let file_content = std::fs::read_to_string(file_path)?;

            // 解析 CSS 宏调用
            let macro_calls = self.extract_css_macro_calls(&file_content)?;

            for macro_call in macro_calls {
                // 分析样式选择器
                let selectors = self.parse_selectors(&macro_call.css_content)?;

                // 记录使用情况
                for selector in selectors {
                    self.record_usage(&selector, file_path, macro_call.line_number);
                }
            }
        }

        // 生成使用报告
        self.generate_usage_report()
    }

    /// 提取 CSS 宏调用
    fn extract_css_macro_calls(&self, content: &str) -> Result<Vec<CssMacroCall>, ParseError> {
        let mut calls = Vec::new();

        // 使用正则表达式匹配 css! 宏
        let css_macro_regex = regex::Regex::new(r"css!\s*\{([^}]+)\}")?;

        for (line_num, line) in content.lines().enumerate() {
            if let Some(captures) = css_macro_regex.captures(line) {
                if let Some(css_content) = captures.get(1) {
                    calls.push(CssMacroCall {
                        line_number: line_num + 1,
                        css_content: css_content.as_str().to_string(),
                        file_path: PathBuf::new(), // 将在调用处设置
                    });
                }
            }
        }

        Ok(calls)
    }

    /// 解析 CSS 选择器
    fn parse_selectors(&self, css_content: &str) -> Result<Vec<CssSelector>, ParseError> {
        let mut selectors = Vec::new();

        // 解析类选择器
        let class_regex = regex::Regex::new(r"\.([a-zA-Z][a-zA-Z0-9_-]*)")?;
        for captures in class_regex.captures_iter(css_content) {
            if let Some(class_name) = captures.get(1) {
                selectors.push(CssSelector {
                    selector_type: SelectorType::Class,
                    name: class_name.as_str().to_string(),
                    specificity: calculate_specificity(&format!(".{}", class_name.as_str())),
                });
            }
        }

        // 解析 ID 选择器
        let id_regex = regex::Regex::new(r"#([a-zA-Z][a-zA-Z0-9_-]*)")?;
        for captures in id_regex.captures_iter(css_content) {
            if let Some(id_name) = captures.get(1) {
                selectors.push(CssSelector {
                    selector_type: SelectorType::Id,
                    name: id_name.as_str().to_string(),
                    specificity: calculate_specificity(&format!("#{}", id_name.as_str())),
                });
            }
        }

        Ok(selectors)
    }

    /// 记录样式使用情况
    fn record_usage(&mut self, selector: &CssSelector, file_path: &PathBuf, line_number: usize) {
        let key = format!("{}:{}", selector.selector_type.as_str(), selector.name);

        let stats = self.usage_stats.entry(key).or_insert_with(|| UsageStats {
            selector: selector.clone(),
            usage_count: 0,
            files: HashSet::new(),
            first_seen: std::time::SystemTime::now(),
            last_seen: std::time::SystemTime::now(),
        });

        stats.usage_count += 1;
        stats.files.insert(file_path.clone());
        stats.last_seen = std::time::SystemTime::now();
    }
}

/// 计算 CSS 选择器特异性
fn calculate_specificity(selector: &str) -> u32 {
    let mut specificity = 0;

    // ID 选择器权重: 100
    specificity += selector.matches('#').count() as u32 * 100;

    // 类选择器权重: 10
    specificity += selector.matches('.').count() as u32 * 10;

    // 元素选择器权重: 1
    let element_count = selector.split_whitespace()
        .filter(|s| !s.starts_with('.') && !s.starts_with('#'))
        .count();
    specificity += element_count as u32;

    specificity
}
```

### 2. CSS 压缩优化

```rust
// CSS 压缩配置
use css_in_rust::build_tools::{
    CssMinifier, MinificationConfig, CompressionLevel
};

/// 配置 CSS 压缩
fn configure_css_minification() -> MinificationConfig {
    MinificationConfig {
        // 压缩级别
        compression_level: CompressionLevel::Maximum,

        // 移除注释
        remove_comments: true,

        // 移除空白字符
        remove_whitespace: true,

        // 合并相同规则
        merge_duplicate_rules: true,

        // 优化选择器
        optimize_selectors: true,

        // 简化颜色值
        simplify_colors: true,

        // 压缩数值
        compress_numbers: true,

        // 移除未使用的前缀
        remove_unused_prefixes: true,

        // 保留重要注释
        preserve_important_comments: true,

        // 自定义优化规则
        custom_optimizations: vec![
            OptimizationRule::RemoveEmptyRules,
            OptimizationRule::MergeMediaQueries,
            OptimizationRule::OptimizeKeyframes,
        ],
    }
}

/// 自定义 CSS 压缩器
struct AdvancedCssMinifier {
    config: MinificationConfig,
    optimization_stats: OptimizationStats,
}

impl AdvancedCssMinifier {
    /// 压缩 CSS 内容
    pub fn minify(&mut self, css_content: &str) -> Result<MinificationResult, MinificationError> {
        let start_time = std::time::Instant::now();
        let original_size = css_content.len();

        let mut result = css_content.to_string();

        // 应用各种优化
        result = self.remove_comments(&result)?;
        result = self.remove_whitespace(&result)?;
        result = self.merge_duplicate_rules(&result)?;
        result = self.optimize_selectors(&result)?;
        result = self.simplify_colors(&result)?;
        result = self.compress_numbers(&result)?;
        result = self.apply_custom_optimizations(&result)?;

        let final_size = result.len();
        let compression_ratio = (original_size - final_size) as f64 / original_size as f64;

        // 更新统计信息
        self.optimization_stats.total_files += 1;
        self.optimization_stats.total_original_size += original_size;
        self.optimization_stats.total_compressed_size += final_size;
        self.optimization_stats.total_time += start_time.elapsed();

        Ok(MinificationResult {
            minified_css: result,
            original_size,
            compressed_size: final_size,
            compression_ratio,
            optimizations_applied: self.get_applied_optimizations(),
            processing_time: start_time.elapsed(),
        })
    }

    /// 移除注释
    fn remove_comments(&self, css: &str) -> Result<String, MinificationError> {
        if !self.config.remove_comments {
            return Ok(css.to_string());
        }

        let comment_regex = regex::Regex::new(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")?;

        let mut result = css.to_string();

        // 保留重要注释 (以 /*! 开头)
        if self.config.preserve_important_comments {
            let important_comment_regex = regex::Regex::new(r"/\*![^*]*\*+(?:[^/*][^*]*\*+)*/")?;
            let important_comments: Vec<_> = important_comment_regex
                .find_iter(&result)
                .map(|m| m.as_str().to_string())
                .collect();

            // 移除所有注释
            result = comment_regex.replace_all(&result, "").to_string();

            // 重新添加重要注释
            for comment in important_comments {
                result = format!("{} {}", comment, result);
            }
        } else {
            result = comment_regex.replace_all(&result, "").to_string();
        }

        Ok(result)
    }

    /// 移除空白字符
    fn remove_whitespace(&self, css: &str) -> Result<String, MinificationError> {
        if !self.config.remove_whitespace {
            return Ok(css.to_string());
        }

        let mut result = css.to_string();

        // 移除多余的空白字符
        result = regex::Regex::new(r"\s+")?.replace_all(&result, " ").to_string();

        // 移除选择器周围的空白
        result = regex::Regex::new(r"\s*\{\s*")?.replace_all(&result, "{").to_string();
        result = regex::Regex::new(r"\s*\}\s*")?.replace_all(&result, "}").to_string();

        // 移除属性周围的空白
        result = regex::Regex::new(r"\s*:\s*")?.replace_all(&result, ":").to_string();
        result = regex::Regex::new(r"\s*;\s*")?.replace_all(&result, ";").to_string();

        // 移除逗号周围的空白
        result = regex::Regex::new(r"\s*,\s*")?.replace_all(&result, ",").to_string();

        Ok(result.trim().to_string())
    }

    /// 合并重复规则
    fn merge_duplicate_rules(&self, css: &str) -> Result<String, MinificationError> {
        if !self.config.merge_duplicate_rules {
            return Ok(css.to_string());
        }

        // 解析 CSS 规则
        let rules = self.parse_css_rules(css)?;

        // 按选择器分组
        let mut grouped_rules: HashMap<String, Vec<CssProperty>> = HashMap::new();

        for rule in rules {
            let selector_key = rule.selectors.join(",");
            grouped_rules.entry(selector_key)
                .or_insert_with(Vec::new)
                .extend(rule.properties);
        }

        // 重新构建 CSS
        let mut result = String::new();
        for (selectors, properties) in grouped_rules {
            result.push_str(&selectors);
            result.push('{');

            for property in properties {
                result.push_str(&format!("{}:{}", property.name, property.value));
                if property.important {
                    result.push_str("!important");
                }
                result.push(';');
            }

            result.push('}');
        }

        Ok(result)
    }
}
```

## 🔄 构建流程优化

### 1. 增量编译配置

```rust
// 增量编译管理器
use css_in_rust::build_tools::{
    IncrementalCompiler, DependencyTracker, BuildCache
};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::SystemTime;

/// 增量编译管理器
pub struct IncrementalBuildManager {
    dependency_tracker: DependencyTracker,
    build_cache: BuildCache,
    last_build_time: Option<SystemTime>,
    file_checksums: HashMap<PathBuf, String>,
}

impl IncrementalBuildManager {
    /// 创建新的增量编译管理器
    pub fn new(cache_dir: PathBuf) -> Result<Self, BuildError> {
        Ok(Self {
            dependency_tracker: DependencyTracker::new(),
            build_cache: BuildCache::new(cache_dir)?,
            last_build_time: None,
            file_checksums: HashMap::new(),
        })
    }

    /// 检查是否需要重新编译
    pub fn needs_rebuild(&mut self, source_files: &[PathBuf]) -> Result<bool, BuildError> {
        // 检查是否是首次构建
        if self.last_build_time.is_none() {
            return Ok(true);
        }

        let last_build = self.last_build_time.unwrap();

        // 检查源文件是否有变化
        for file_path in source_files {
            let metadata = std::fs::metadata(file_path)?;
            let modified_time = metadata.modified()?;

            if modified_time > last_build {
                return Ok(true);
            }

            // 检查文件内容是否有变化
            let current_checksum = self.calculate_file_checksum(file_path)?;
            if let Some(cached_checksum) = self.file_checksums.get(file_path) {
                if current_checksum != *cached_checksum {
                    return Ok(true);
                }
            } else {
                return Ok(true);
            }
        }

        // 检查依赖文件是否有变化
        let dependencies = self.dependency_tracker.get_all_dependencies();
        for dep_path in dependencies {
            if dep_path.exists() {
                let metadata = std::fs::metadata(&dep_path)?;
                let modified_time = metadata.modified()?;

                if modified_time > last_build {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// 执行增量构建
    pub fn incremental_build(
        &mut self,
        source_files: &[PathBuf],
        config: &BuildConfig,
    ) -> Result<IncrementalBuildResult, BuildError> {
        let start_time = std::time::Instant::now();

        // 分析变更的文件
        let changed_files = self.analyze_changed_files(source_files)?;

        // 计算受影响的文件
        let affected_files = self.dependency_tracker
            .get_affected_files(&changed_files)?;

        // 从缓存中加载未变更的文件
        let mut cached_results = HashMap::new();
        for file_path in source_files {
            if !affected_files.contains(file_path) {
                if let Some(cached_result) = self.build_cache.get(file_path)? {
                    cached_results.insert(file_path.clone(), cached_result);
                }
            }
        }

        // 只编译受影响的文件
        let mut compilation_results = HashMap::new();
        for file_path in &affected_files {
            let result = self.compile_single_file(file_path, config)?;
            compilation_results.insert(file_path.clone(), result.clone());

            // 更新缓存
            self.build_cache.set(file_path, &result)?;

            // 更新文件校验和
            let checksum = self.calculate_file_checksum(file_path)?;
            self.file_checksums.insert(file_path.clone(), checksum);
        }

        // 合并结果
        let mut all_results = cached_results;
        all_results.extend(compilation_results);

        // 更新构建时间
        self.last_build_time = Some(SystemTime::now());

        Ok(IncrementalBuildResult {
            compiled_files: affected_files,
            cached_files: source_files.iter()
                .filter(|f| !affected_files.contains(f))
                .cloned()
                .collect(),
            total_files: source_files.len(),
            build_time: start_time.elapsed(),
            cache_hit_ratio: (source_files.len() - affected_files.len()) as f64 / source_files.len() as f64,
            results: all_results,
        })
    }

    /// 分析变更的文件
    fn analyze_changed_files(&mut self, source_files: &[PathBuf]) -> Result<HashSet<PathBuf>, BuildError> {
        let mut changed_files = HashSet::new();

        for file_path in source_files {
            let current_checksum = self.calculate_file_checksum(file_path)?;

            if let Some(cached_checksum) = self.file_checksums.get(file_path) {
                if current_checksum != *cached_checksum {
                    changed_files.insert(file_path.clone());
                }
            } else {
                // 新文件
                changed_files.insert(file_path.clone());
            }
        }

        Ok(changed_files)
    }

    /// 计算文件校验和
    fn calculate_file_checksum(&self, file_path: &PathBuf) -> Result<String, BuildError> {
        use sha2::{Sha256, Digest};

        let content = std::fs::read(file_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        Ok(format!("{:x}", result))
    }

    /// 编译单个文件
    fn compile_single_file(
        &self,
        file_path: &PathBuf,
        config: &BuildConfig,
    ) -> Result<CompilationResult, BuildError> {
        // 读取文件内容
        let content = std::fs::read_to_string(file_path)?;

        // 解析 CSS 宏
        let css_macros = self.extract_css_macros(&content)?;

        // 编译 CSS
        let mut compiled_css = String::new();
        for css_macro in css_macros {
            let processed_css = self.process_css_macro(&css_macro, config)?;
            compiled_css.push_str(&processed_css);
        }

        // 应用优化
        let optimized_css = if config.optimization_level != OptimizationLevel::None {
            self.optimize_css(&compiled_css, config)?
        } else {
            compiled_css
        };

        Ok(CompilationResult {
            file_path: file_path.clone(),
            original_content: content,
            compiled_css: optimized_css,
            compilation_time: std::time::Instant::now().elapsed(),
            optimizations_applied: vec![], // 根据实际优化填充
        })
    }
}
```

### 2. 并行处理优化

```rust
// 并行构建处理器
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// 并行构建处理器
pub struct ParallelBuildProcessor {
    max_parallel_jobs: usize,
    build_stats: Arc<Mutex<ParallelBuildStats>>,
}

impl ParallelBuildProcessor {
    /// 创建新的并行构建处理器
    pub fn new(max_parallel_jobs: Option<usize>) -> Self {
        let jobs = max_parallel_jobs.unwrap_or_else(num_cpus::get);

        Self {
            max_parallel_jobs: jobs,
            build_stats: Arc::new(Mutex::new(ParallelBuildStats::new())),
        }
    }

    /// 并行处理文件
    pub fn process_files_parallel(
        &self,
        files: &[PathBuf],
        config: &BuildConfig,
    ) -> Result<ParallelBuildResult, BuildError> {
        let start_time = std::time::Instant::now();

        // 配置 Rayon 线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.max_parallel_jobs)
            .build()
            .map_err(|e| BuildError::ThreadPoolError(e.to_string()))?;

        // 并行处理文件
        let results: Result<Vec<_>, _> = pool.install(|| {
            files.par_iter()
                .map(|file_path| {
                    let file_start = std::time::Instant::now();

                    // 处理单个文件
                    let result = self.process_single_file(file_path, config);

                    // 更新统计信息
                    {
                        let mut stats = self.build_stats.lock().unwrap();
                        stats.files_processed += 1;
                        stats.total_processing_time += file_start.elapsed();

                        if result.is_ok() {
                            stats.successful_files += 1;
                        } else {
                            stats.failed_files += 1;
                        }
                    }

                    result
                })
                .collect()
        });

        let file_results = results?;
        let total_time = start_time.elapsed();

        // 合并结果
        let mut combined_css = String::new();
        let mut total_original_size = 0;
        let mut total_compressed_size = 0;

        for result in &file_results {
            combined_css.push_str(&result.compiled_css);
            total_original_size += result.original_size;
            total_compressed_size += result.compressed_size;
        }

        // 计算并行效率
        let stats = self.build_stats.lock().unwrap();
        let parallel_efficiency = if total_time.as_millis() > 0 {
            stats.total_processing_time.as_millis() as f64 /
            (total_time.as_millis() as f64 * self.max_parallel_jobs as f64)
        } else {
            0.0
        };

        Ok(ParallelBuildResult {
            file_results,
            combined_css,
            total_files: files.len(),
            successful_files: stats.successful_files,
            failed_files: stats.failed_files,
            total_time,
            parallel_efficiency,
            threads_used: self.max_parallel_jobs,
            total_original_size,
            total_compressed_size,
        })
    }

    /// 处理单个文件
    fn process_single_file(
        &self,
        file_path: &PathBuf,
        config: &BuildConfig,
    ) -> Result<FileProcessingResult, BuildError> {
        let start_time = std::time::Instant::now();

        // 读取文件
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| BuildError::FileReadError(file_path.clone(), e.to_string()))?;

        let original_size = content.len();

        // 提取和处理 CSS
        let css_content = self.extract_and_process_css(&content, config)?;

        // 应用优化
        let optimized_css = if config.enable_optimization {
            self.optimize_css(&css_content, config)?
        } else {
            css_content
        };

        let compressed_size = optimized_css.len();
        let processing_time = start_time.elapsed();

        Ok(FileProcessingResult {
            file_path: file_path.clone(),
            compiled_css: optimized_css,
            original_size,
            compressed_size,
            processing_time,
            thread_id: rayon::current_thread_index().unwrap_or(0),
        })
    }

    /// 提取和处理 CSS
    fn extract_and_process_css(
        &self,
        content: &str,
        config: &BuildConfig,
    ) -> Result<String, BuildError> {
        // CSS 宏提取逻辑
        let css_macro_regex = regex::Regex::new(r"css!\s*\{([^}]+)\}")
            .map_err(|e| BuildError::RegexError(e.to_string()))?;

        let mut processed_css = String::new();

        for captures in css_macro_regex.captures_iter(content) {
            if let Some(css_match) = captures.get(1) {
                let css_content = css_match.as_str();

                // 处理 CSS 内容
                let processed = self.process_css_content(css_content, config)?;
                processed_css.push_str(&processed);
                processed_css.push('\n');
            }
        }

        Ok(processed_css)
    }

    /// 处理 CSS 内容
    fn process_css_content(
        &self,
        css_content: &str,
        config: &BuildConfig,
    ) -> Result<String, BuildError> {
        let mut result = css_content.to_string();

        // 应用预处理器
        if config.enable_preprocessing {
            result = self.apply_preprocessing(&result)?;
        }

        // 应用变量替换
        if config.enable_variables {
            result = self.apply_variable_substitution(&result)?;
        }

        // 应用自动前缀
        if config.enable_autoprefixer {
            result = self.apply_autoprefixer(&result)?;
        }

        Ok(result)
    }
}

/// 并行构建统计信息
#[derive(Debug, Clone)]
pub struct ParallelBuildStats {
    pub files_processed: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub total_processing_time: std::time::Duration,
}

impl ParallelBuildStats {
    pub fn new() -> Self {
        Self {
            files_processed: 0,
            successful_files: 0,
            failed_files: 0,
            total_processing_time: std::time::Duration::new(0, 0),
        }
    }
}
```

## 💾 缓存优化策略

### 1. 智能缓存管理

```rust
// 智能缓存管理器
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};

/// 智能缓存管理器
pub struct SmartCacheManager {
    cache_dir: PathBuf,
    cache_strategy: CacheStrategy,
    max_cache_size: usize,
    ttl: Duration,
    cache_index: CacheIndex,
}

/// 缓存策略
#[derive(Debug, Clone)]
pub enum CacheStrategy {
    /// 保守策略 - 只缓存稳定的结果
    Conservative,
    /// 平衡策略 - 缓存大部分结果
    Balanced,
    /// 激进策略 - 缓存所有可能的结果
    Aggressive,
}

/// 缓存索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheIndex {
    entries: HashMap<String, CacheEntry>,
    total_size: usize,
    last_cleanup: SystemTime,
}

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    key: String,
    file_path: PathBuf,
    content_hash: String,
    result_hash: String,
    size: usize,
    created_at: SystemTime,
    last_accessed: SystemTime,
    access_count: u32,
    dependencies: Vec<PathBuf>,
}

impl SmartCacheManager {
    /// 创建新的缓存管理器
    pub fn new(
        cache_dir: PathBuf,
        strategy: CacheStrategy,
        max_size_mb: usize,
        ttl_hours: u64,
    ) -> Result<Self, CacheError> {
        let cache_index = Self::load_or_create_index(&cache_dir)?;

        Ok(Self {
            cache_dir,
            cache_strategy: strategy,
            max_cache_size: max_size_mb * 1024 * 1024, // 转换为字节
            ttl: Duration::from_secs(ttl_hours * 3600),
            cache_index,
        })
    }

    /// 获取缓存结果
    pub fn get(&mut self, key: &str, dependencies: &[PathBuf]) -> Result<Option<CachedResult>, CacheError> {
        // 检查缓存条目是否存在
        if let Some(entry) = self.cache_index.entries.get_mut(key) {
            // 检查是否过期
            if self.is_expired(entry) {
                self.remove_entry(key)?;
                return Ok(None);
            }

            // 检查依赖是否有变化
            if self.dependencies_changed(entry, dependencies)? {
                self.remove_entry(key)?;
                return Ok(None);
            }

            // 更新访问信息
            entry.last_accessed = SystemTime::now();
            entry.access_count += 1;

            // 加载缓存结果
            let result = self.load_cached_result(entry)?;
            return Ok(Some(result));
        }

        Ok(None)
    }

    /// 设置缓存结果
    pub fn set(
        &mut self,
        key: &str,
        file_path: &PathBuf,
        content_hash: &str,
        result: &CachedResult,
        dependencies: &[PathBuf],
    ) -> Result<(), CacheError> {
        // 检查缓存策略
        if !self.should_cache(result) {
            return Ok(());
        }

        // 序列化结果
        let serialized = bincode::serialize(result)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;

        let result_size = serialized.len();

        // 检查缓存空间
        self.ensure_cache_space(result_size)?;

        // 创建缓存文件
        let cache_file_path = self.cache_dir.join(format!("{}.cache", key));
        std::fs::write(&cache_file_path, &serialized)
            .map_err(|e| CacheError::WriteError(cache_file_path.clone(), e.to_string()))?;

        // 创建缓存条目
        let entry = CacheEntry {
            key: key.to_string(),
            file_path: file_path.clone(),
            content_hash: content_hash.to_string(),
            result_hash: self.calculate_result_hash(&serialized),
            size: result_size,
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 1,
            dependencies: dependencies.to_vec(),
        };

        // 更新索引
        self.cache_index.entries.insert(key.to_string(), entry);
        self.cache_index.total_size += result_size;

        // 保存索引
        self.save_index()?;

        Ok(())
    }

    /// 检查是否应该缓存
    fn should_cache(&self, result: &CachedResult) -> bool {
        match self.cache_strategy {
            CacheStrategy::Conservative => {
                // 只缓存大文件或复杂处理结果
                result.processing_time.as_millis() > 100 || result.original_size > 10240
            }
            CacheStrategy::Balanced => {
                // 缓存大部分结果，除了非常小的文件
                result.original_size > 1024
            }
            CacheStrategy::Aggressive => {
                // 缓存所有结果
                true
            }
        }
    }

    /// 确保缓存空间
    fn ensure_cache_space(&mut self, required_size: usize) -> Result<(), CacheError> {
        // 检查是否需要清理
        if self.cache_index.total_size + required_size > self.max_cache_size {
            self.cleanup_cache(required_size)?;
        }

        Ok(())
    }

    /// 清理缓存
    fn cleanup_cache(&mut self, required_size: usize) -> Result<(), CacheError> {
        let mut entries_to_remove = Vec::new();
        let target_size = self.max_cache_size - required_size;

        // 按优先级排序（LRU + 访问频率）
        let mut sorted_entries: Vec<_> = self.cache_index.entries.iter().collect();
        sorted_entries.sort_by(|a, b| {
            let score_a = self.calculate_cache_score(a.1);
            let score_b = self.calculate_cache_score(b.1);
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 移除低优先级条目
        let mut current_size = self.cache_index.total_size;
        for (key, entry) in sorted_entries {
            if current_size <= target_size {
                break;
            }

            entries_to_remove.push(key.clone());
            current_size -= entry.size;
        }

        // 执行移除
        for key in entries_to_remove {
            self.remove_entry(&key)?;
        }

        Ok(())
    }

    /// 计算缓存分数（用于 LRU 算法）
    fn calculate_cache_score(&self, entry: &CacheEntry) -> f64 {
        let now = SystemTime::now();

        // 时间因子（越久未访问分数越低）
        let time_factor = if let Ok(duration) = now.duration_since(entry.last_accessed) {
            1.0 / (duration.as_secs() as f64 + 1.0)
        } else {
            0.0
        };

        // 访问频率因子
        let frequency_factor = entry.access_count as f64;

        // 大小因子（大文件优先保留）
        let size_factor = (entry.size as f64).log10();

        // 综合分数
        time_factor * 0.4 + frequency_factor * 0.4 + size_factor * 0.2
    }

    /// 检查依赖是否有变化
    fn dependencies_changed(&self, entry: &CacheEntry, current_deps: &[PathBuf]) -> Result<bool, CacheError> {
        // 检查依赖数量是否变化
        if entry.dependencies.len() != current_deps.len() {
            return Ok(true);
        }

        // 检查每个依赖文件的修改时间
        for dep_path in &entry.dependencies {
            if dep_path.exists() {
                let metadata = std::fs::metadata(dep_path)
                    .map_err(|e| CacheError::FileError(dep_path.clone(), e.to_string()))?;

                let modified_time = metadata.modified()
                    .map_err(|e| CacheError::FileError(dep_path.clone(), e.to_string()))?;

                if modified_time > entry.created_at {
                    return Ok(true);
                }
            } else {
                // 依赖文件被删除
                return Ok(true);
            }
        }

        Ok(false)
    }
}
```

## 📊 构建性能监控

### 1. 性能指标收集

```rust
// 构建性能监控器
use std::time::{Instant, Duration};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 构建性能监控器
pub struct BuildPerformanceMonitor {
    metrics: BuildMetrics,
    phase_timers: HashMap<String, Instant>,
    enabled: bool,
}

/// 构建指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetrics {
    pub total_build_time: Duration,
    pub compilation_time: Duration,
    pub optimization_time: Duration,
    pub io_time: Duration,
    pub cache_time: Duration,

    pub files_processed: usize,
    pub total_input_size: usize,
    pub total_output_size: usize,
    pub compression_ratio: f64,

    pub cache_hits: usize,
    pub cache_misses: usize,
    pub cache_hit_ratio: f64,

    pub parallel_efficiency: f64,
    pub threads_used: usize,

    pub phase_timings: HashMap<String, Duration>,
    pub file_timings: HashMap<String, Duration>,

    pub memory_usage: MemoryMetrics,
    pub error_count: usize,
    pub warning_count: usize,
}

/// 内存使用指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub peak_memory_usage: usize,
    pub average_memory_usage: usize,
    pub memory_samples: Vec<MemorySample>,
}

/// 内存采样
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySample {
    pub timestamp: std::time::SystemTime,
    pub memory_usage: usize,
    pub phase: String,
}

impl BuildPerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new(enabled: bool) -> Self {
        Self {
            metrics: BuildMetrics::new(),
            phase_timers: HashMap::new(),
            enabled,
        }
    }

    /// 开始监控构建阶段
    pub fn start_phase(&mut self, phase_name: &str) {
        if !self.enabled {
            return;
        }

        self.phase_timers.insert(phase_name.to_string(), Instant::now());

        // 记录内存使用情况
        if let Ok(memory_usage) = self.get_current_memory_usage() {
            self.metrics.memory_usage.memory_samples.push(MemorySample {
                timestamp: std::time::SystemTime::now(),
                memory_usage,
                phase: phase_name.to_string(),
            });
        }
    }

    /// 结束监控构建阶段
    pub fn end_phase(&mut self, phase_name: &str) {
        if !self.enabled {
            return;
        }

        if let Some(start_time) = self.phase_timers.remove(phase_name) {
            let duration = start_time.elapsed();
            self.metrics.phase_timings.insert(phase_name.to_string(), duration);

            // 更新特定阶段的时间
            match phase_name {
                "compilation" => self.metrics.compilation_time = duration,
                "optimization" => self.metrics.optimization_time = duration,
                "io" => self.metrics.io_time = duration,
                "cache" => self.metrics.cache_time = duration,
                _ => {}
            }
        }
    }

    /// 记录文件处理时间
    pub fn record_file_timing(&mut self, file_path: &str, duration: Duration) {
        if !self.enabled {
            return;
        }

        self.metrics.file_timings.insert(file_path.to_string(), duration);
    }

    /// 记录缓存命中
    pub fn record_cache_hit(&mut self) {
        if !self.enabled {
            return;
        }

        self.metrics.cache_hits += 1;
        self.update_cache_hit_ratio();
    }

    /// 记录缓存未命中
    pub fn record_cache_miss(&mut self) {
        if !self.enabled {
            return;
        }

        self.metrics.cache_misses += 1;
        self.update_cache_hit_ratio();
    }

    /// 更新缓存命中率
    fn update_cache_hit_ratio(&mut self) {
        let total = self.metrics.cache_hits + self.metrics.cache_misses;
        if total > 0 {
            self.metrics.cache_hit_ratio = self.metrics.cache_hits as f64 / total as f64;
        }
    }

    /// 获取当前内存使用情况
    fn get_current_memory_usage(&self) -> Result<usize, std::io::Error> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            let output = Command::new("ps")
                .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
                .output()?;

            let memory_kb = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<usize>()
                .unwrap_or(0);

            Ok(memory_kb * 1024) // 转换为字节
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 其他平台的实现
            Ok(0)
        }
    }

    /// 生成性能报告
    pub fn generate_report(&mut self) -> BuildPerformanceReport {
        if !self.enabled {
            return BuildPerformanceReport::empty();
        }

        // 计算总构建时间
        self.metrics.total_build_time = self.metrics.phase_timings
            .values()
            .sum();

        // 计算压缩比
        if self.metrics.total_input_size > 0 {
            self.metrics.compression_ratio =
                (self.metrics.total_input_size - self.metrics.total_output_size) as f64 /
                self.metrics.total_input_size as f64;
        }

        // 计算内存统计
        self.calculate_memory_stats();

        BuildPerformanceReport {
            metrics: self.metrics.clone(),
            recommendations: self.generate_recommendations(),
            bottlenecks: self.identify_bottlenecks(),
        }
    }

    /// 计算内存统计
    fn calculate_memory_stats(&mut self) {
        if self.metrics.memory_usage.memory_samples.is_empty() {
            return;
        }

        let memory_values: Vec<usize> = self.metrics.memory_usage.memory_samples
            .iter()
            .map(|sample| sample.memory_usage)
            .collect();

        self.metrics.memory_usage.peak_memory_usage =
            *memory_values.iter().max().unwrap_or(&0);

        self.metrics.memory_usage.average_memory_usage =
            memory_values.iter().sum::<usize>() / memory_values.len();
    }

    /// 生成优化建议
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        // 缓存命中率建议
        if self.metrics.cache_hit_ratio < 0.5 {
            recommendations.push(
                "缓存命中率较低，考虑调整缓存策略或增加缓存大小".to_string()
            );
        }

        // 并行效率建议
        if self.metrics.parallel_efficiency < 0.7 {
            recommendations.push(
                "并行效率较低，考虑减少线程数或优化任务分配".to_string()
            );
        }

        // 内存使用建议
        if self.metrics.memory_usage.peak_memory_usage > 1024 * 1024 * 1024 { // 1GB
            recommendations.push(
                "内存使用量较高，考虑启用流式处理或减少并行度".to_string()
            );
        }

        // 编译时间建议
        if self.metrics.compilation_time.as_millis() > 5000 {
            recommendations.push(
                "编译时间较长，考虑启用增量编译或优化代码结构".to_string()
            );
        }

        recommendations
    }

    /// 识别性能瓶颈
    fn identify_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        // 找出最耗时的阶段
        if let Some((phase, duration)) = self.metrics.phase_timings
            .iter()
            .max_by_key(|(_, duration)| *duration) {

            let percentage = duration.as_millis() as f64 /
                self.metrics.total_build_time.as_millis() as f64 * 100.0;

            if percentage > 50.0 {
                bottlenecks.push(format!(
                    "阶段 '{}' 占用了 {:.1}% 的构建时间",
                    phase, percentage
                ));
            }
        }

        // 找出最耗时的文件
        if let Some((file, duration)) = self.metrics.file_timings
            .iter()
            .max_by_key(|(_, duration)| *duration) {

            if duration.as_millis() > 1000 {
                bottlenecks.push(format!(
                    "文件 '{}' 处理时间过长: {} ms",
                    file, duration.as_millis()
                ));
            }
        }

        bottlenecks
    }
}
```

## 🔧 构建工具配置最佳实践

### ✅ 开发环境优化
- [ ] 启用增量编译减少重复工作
- [ ] 使用适度的并行度避免资源竞争
- [ ] 启用缓存但使用保守策略
- [ ] 禁用重度优化以提升构建速度
- [ ] 启用详细日志便于调试

### ✅ 生产环境优化
- [ ] 启用所有优化选项
- [ ] 使用激进的缓存策略
- [ ] 启用死代码消除
- [ ] 启用 CSS 压缩和混淆
- [ ] 生成构建报告用于分析

### ✅ 性能监控
- [ ] 定期分析构建性能报告
- [ ] 监控缓存命中率
- [ ] 跟踪构建时间趋势
- [ ] 识别和优化性能瓶颈
- [ ] 调整并行度和内存使用

### ✅ 缓存管理
- [ ] 根据项目规模选择合适的缓存策略
- [ ] 定期清理过期缓存
- [ ] 监控缓存大小和命中率
- [ ] 配置合理的 TTL 时间
- [ ] 使用依赖跟踪确保缓存一致性

通过合理的构建工具优化，您可以显著提升 CSS-in-Rust 项目的构建性能和开发体验！🚀
