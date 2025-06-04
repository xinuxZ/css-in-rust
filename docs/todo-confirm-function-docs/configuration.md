# 配置指南

本指南详细介绍 CSS-in-Rust 的各种配置选项，帮助您根据项目需求进行最佳配置。

## 📋 配置概览

CSS-in-Rust 支持多种配置方式：

- **项目配置文件** (`css-in-rust.toml`)
- **Cargo.toml 配置**
- **环境变量**
- **构建脚本配置** (`build.rs`)
- **运行时配置**

## 🔧 项目配置文件

### 1. 基础配置 (`css-in-rust.toml`)

```toml
# css-in-rust.toml
[project]
name = "my-web-app"
version = "0.1.0"
output_dir = "dist"
cache_dir = ".cache/css-in-rust"

# 编译配置
[compilation]
# 启用死代码消除
dead_code_elimination = true
# CSS 压缩
minify = true
# 生成 Source Map
source_maps = true
# 启用增量编译
incremental = true
# 并行编译
parallel = true
max_parallel_jobs = 4

# 开发配置
[development]
# 启用热更新
hot_reload = true
# WebSocket 端口
websocket_port = 3001
# 监控文件模式
watch_patterns = [
    "src/**/*.rs",
    "components/**/*.rs",
    "styles/**/*.css"
]
# 忽略模式
ignore_patterns = [
    "target/**",
    "**/.git/**",
    "**/node_modules/**",
    "**/*.tmp"
]
# 防抖延迟 (毫秒)
debounce_ms = 50

# 生产配置
[production]
# 启用激进优化
aggressive_optimization = true
# CSS 压缩级别 (1-9)
compression_level = 9
# 移除未使用的 CSS
remove_unused_css = true
# 内联小文件阈值 (字节)
inline_threshold = 1024
# 启用 Brotli 压缩
brotli_compression = true

# 主题配置
[theme]
# 默认主题
default = "light"
# 主题文件路径
themes_dir = "themes"
# 支持的主题
themes = ["light", "dark", "auto"]
# 启用主题切换动画
animated_transitions = true
# 主题切换持续时间 (毫秒)
transition_duration = 300

# 性能配置
[performance]
# 启用缓存
enable_caching = true
# 缓存策略
cache_strategy = "aggressive" # "conservative", "balanced", "aggressive"
# 最大缓存大小 (MB)
max_cache_size = 100
# 缓存过期时间 (小时)
cache_ttl = 24
# 启用预加载
preload_critical_css = true
# 懒加载阈值
lazy_load_threshold = 2048

# 诊断配置
[diagnostics]
# 启用详细日志
verbose_logging = false
# 日志级别
log_level = "info" # "error", "warn", "info", "debug", "trace"
# 性能监控
performance_monitoring = true
# 生成构建报告
generate_reports = true
# 报告输出目录
reports_dir = "reports"

# 框架特定配置
[frameworks]
# Yew 配置
[frameworks.yew]
enabled = true
features = ["hot-reload", "theme-support"]

# Leptos 配置
[frameworks.leptos]
enabled = false
ssr = true
hydration = true

# Dioxus 配置
[frameworks.dioxus]
enabled = false
platforms = ["web", "desktop"]

# 插件配置
[plugins]
# PostCSS 插件
postcss = [
    "autoprefixer",
    "cssnano"
]
# 自定义插件
custom = [
    { name = "my-plugin", path = "./plugins/my-plugin.js" }
]

# 输出配置
[output]
# CSS 文件名模式
css_filename = "[name].[contenthash:8].css"
# JS 文件名模式
js_filename = "[name].[contenthash:8].js"
# 资源文件名模式
asset_filename = "assets/[name].[contenthash:8][ext]"
# 公共路径
public_path = "/"
# 启用文件哈希
hash_filenames = true

# 实验性功能
[experimental]
# 启用 CSS 模块
css_modules = false
# 启用 CSS-in-JS 兼容模式
css_in_js_compat = false
# 启用原子化 CSS
atomic_css = false
# 启用运行时主题切换
runtime_theming = true
```

### 2. 环境特定配置

```toml
# css-in-rust.development.toml
[compilation]
minify = false
source_maps = true
optimization_level = 0

[development]
hot_reload = true
verbose_logging = true

[performance]
cache_strategy = "conservative"

# css-in-rust.production.toml
[compilation]
minify = true
source_maps = false
optimization_level = 3

[production]
aggressive_optimization = true
remove_unused_css = true

[performance]
cache_strategy = "aggressive"
preload_critical_css = true
```

## 🦀 Cargo.toml 配置

### 1. 基础依赖配置

```toml
# Cargo.toml
[package]
name = "my-web-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# 核心依赖
css-in-rust = { version = "0.1.0", features = [
    "yew",           # Yew 框架支持
    "hot-reload",    # 热更新支持
    "theme-system",  # 主题系统
    "performance",   # 性能优化
    "diagnostics",   # 诊断工具
] }

# 框架依赖
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = "0.3"

# 序列化支持
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 异步支持
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

[build-dependencies]
# 构建时依赖
css-in-rust = { version = "0.1.0", features = ["build-tools"] }

[dev-dependencies]
# 开发依赖
wasm-bindgen-test = "0.3"

# 特性配置
[features]
default = ["web"]
web = ["yew", "hot-reload"]
ssr = ["leptos/ssr"]
desktop = ["dioxus/desktop"]
hot-reload = ["css-in-rust/hot-reload"]
theme-system = ["css-in-rust/theme-system"]
performance = ["css-in-rust/performance"]
diagnostics = ["css-in-rust/diagnostics"]

# 优化配置
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.dev]
opt-level = 0
debug = true

# WASM 特定配置
[profile.release.package."*"]
opt-level = 3

# 工作空间配置
[workspace]
members = [
    "css-in-rust",
    "css-in-rust-macros",
    "examples/*"
]

# 元数据
[package.metadata.css-in-rust]
# CSS-in-Rust 特定元数据
config_file = "css-in-rust.toml"
themes_dir = "themes"
output_dir = "dist"
```

### 2. 条件编译配置

```toml
# 平台特定依赖
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tokio = { version = "1.0", features = ["full"] }

# 开发环境特定
[target.'cfg(debug_assertions)'.dependencies]
css-in-rust = { version = "0.1.0", features = ["hot-reload", "diagnostics"] }

# 生产环境特定
[target.'cfg(not(debug_assertions))'.dependencies]
css-in-rust = { version = "0.1.0", features = ["performance", "optimization"] }
```

## 🌍 环境变量配置

### 1. 开发环境变量

```bash
# .env.development
# CSS-in-Rust 配置
CSS_IN_RUST_ENV=development
CSS_IN_RUST_HOT_RELOAD=true
CSS_IN_RUST_WEBSOCKET_PORT=3001
CSS_IN_RUST_LOG_LEVEL=debug
CSS_IN_RUST_CACHE_DIR=.cache/css-in-rust

# 编译配置
CSS_IN_RUST_MINIFY=false
CSS_IN_RUST_SOURCE_MAPS=true
CSS_IN_RUST_DEAD_CODE_ELIMINATION=false

# 性能配置
CSS_IN_RUST_CACHE_STRATEGY=conservative
CSS_IN_RUST_PARALLEL_JOBS=2

# 主题配置
CSS_IN_RUST_DEFAULT_THEME=light
CSS_IN_RUST_THEME_TRANSITIONS=true

# 诊断配置
CSS_IN_RUST_VERBOSE_LOGGING=true
CSS_IN_RUST_PERFORMANCE_MONITORING=true
CSS_IN_RUST_GENERATE_REPORTS=true
```

### 2. 生产环境变量

```bash
# .env.production
# CSS-in-Rust 配置
CSS_IN_RUST_ENV=production
CSS_IN_RUST_HOT_RELOAD=false
CSS_IN_RUST_LOG_LEVEL=warn

# 编译配置
CSS_IN_RUST_MINIFY=true
CSS_IN_RUST_SOURCE_MAPS=false
CSS_IN_RUST_DEAD_CODE_ELIMINATION=true
CSS_IN_RUST_OPTIMIZATION_LEVEL=3

# 性能配置
CSS_IN_RUST_CACHE_STRATEGY=aggressive
CSS_IN_RUST_PARALLEL_JOBS=4
CSS_IN_RUST_PRELOAD_CRITICAL_CSS=true

# 压缩配置
CSS_IN_RUST_COMPRESSION_LEVEL=9
CSS_IN_RUST_BROTLI_COMPRESSION=true
CSS_IN_RUST_REMOVE_UNUSED_CSS=true

# 输出配置
CSS_IN_RUST_HASH_FILENAMES=true
CSS_IN_RUST_PUBLIC_PATH=/static/
```

### 3. 环境变量使用示例

```rust
// src/config.rs
use std::env;

/// 从环境变量加载配置
pub fn load_config_from_env() -> CssInRustConfig {
    CssInRustConfig {
        // 基础配置
        environment: env::var("CSS_IN_RUST_ENV")
            .unwrap_or_else(|_| "development".to_string()),

        // 热更新配置
        hot_reload: env::var("CSS_IN_RUST_HOT_RELOAD")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(cfg!(debug_assertions)),

        websocket_port: env::var("CSS_IN_RUST_WEBSOCKET_PORT")
            .map(|v| v.parse().unwrap_or(3001))
            .unwrap_or(3001),

        // 编译配置
        minify: env::var("CSS_IN_RUST_MINIFY")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(!cfg!(debug_assertions)),

        source_maps: env::var("CSS_IN_RUST_SOURCE_MAPS")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(cfg!(debug_assertions)),

        dead_code_elimination: env::var("CSS_IN_RUST_DEAD_CODE_ELIMINATION")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(!cfg!(debug_assertions)),

        // 性能配置
        cache_strategy: env::var("CSS_IN_RUST_CACHE_STRATEGY")
            .unwrap_or_else(|_| {
                if cfg!(debug_assertions) {
                    "conservative".to_string()
                } else {
                    "aggressive".to_string()
                }
            }),

        parallel_jobs: env::var("CSS_IN_RUST_PARALLEL_JOBS")
            .map(|v| v.parse().unwrap_or(num_cpus::get()))
            .unwrap_or(num_cpus::get()),

        // 日志配置
        log_level: env::var("CSS_IN_RUST_LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string()),

        verbose_logging: env::var("CSS_IN_RUST_VERBOSE_LOGGING")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false),

        // 主题配置
        default_theme: env::var("CSS_IN_RUST_DEFAULT_THEME")
            .unwrap_or_else(|_| "light".to_string()),

        // 输出配置
        output_dir: env::var("CSS_IN_RUST_OUTPUT_DIR")
            .unwrap_or_else(|_| "dist".to_string()),

        cache_dir: env::var("CSS_IN_RUST_CACHE_DIR")
            .unwrap_or_else(|_| ".cache/css-in-rust".to_string()),

        public_path: env::var("CSS_IN_RUST_PUBLIC_PATH")
            .unwrap_or_else(|_| "/".to_string()),
    }
}

/// 配置结构体
#[derive(Debug, Clone)]
pub struct CssInRustConfig {
    pub environment: String,
    pub hot_reload: bool,
    pub websocket_port: u16,
    pub minify: bool,
    pub source_maps: bool,
    pub dead_code_elimination: bool,
    pub cache_strategy: String,
    pub parallel_jobs: usize,
    pub log_level: String,
    pub verbose_logging: bool,
    pub default_theme: String,
    pub output_dir: String,
    pub cache_dir: String,
    pub public_path: String,
}

impl Default for CssInRustConfig {
    fn default() -> Self {
        load_config_from_env()
    }
}
```

## 🔨 构建脚本配置

### 1. 基础构建脚本 (`build.rs`)

```rust
// build.rs
use css_in_rust::build_tools::{
    CssBuildProcessor, BuildConfig, StaticAnalyzer
};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取构建环境
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_release = profile == "release";

    // 创建构建配置
    let config = BuildConfig {
        project_root: PathBuf::from(env::var("CARGO_MANIFEST_DIR")?),
        output_dir: PathBuf::from("target").join(&profile).join("css"),

        // 根据构建类型调整配置
        dead_code_elimination: is_release,
        minify: is_release,
        generate_source_maps: !is_release,

        // 优化配置
        optimization_level: if is_release { 3 } else { 0 },
        parallel_processing: true,
        max_parallel_jobs: num_cpus::get(),

        // 缓存配置
        enable_caching: true,
        cache_dir: PathBuf::from(".cache/css-in-rust"),

        // 分析配置
        usage_threshold: 0.1, // 10% 使用率阈值
        aggressive_elimination: is_release,

        // 报告配置
        generate_reports: env::var("CSS_IN_RUST_GENERATE_REPORTS")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(is_release),

        reports_dir: PathBuf::from("reports"),

        // 监控配置
        watch_files: vec![
            "src/**/*.rs".to_string(),
            "components/**/*.rs".to_string(),
            "styles/**/*.css".to_string(),
        ],

        ignore_patterns: vec![
            "target/**".to_string(),
            "**/.git/**".to_string(),
            "**/node_modules/**".to_string(),
        ],
    };

    // 创建构建处理器
    let mut processor = CssBuildProcessor::new(config)?;

    // 设置环境特定配置
    if let Ok(custom_config) = env::var("CSS_IN_RUST_CONFIG") {
        processor.load_config_file(&custom_config)?;
    }

    // 执行构建
    let result = processor.run()?;

    // 输出构建信息
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=components/");
    println!("cargo:rerun-if-changed=css-in-rust.toml");

    // 设置环境变量
    println!("cargo:rustc-env=CSS_IN_RUST_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=CSS_IN_RUST_BUILD_TIME={}", chrono::Utc::now().to_rfc3339());
    println!("cargo:rustc-env=CSS_IN_RUST_PROFILE={}", profile);

    // 输出构建统计
    if result.stats.files_processed > 0 {
        println!("cargo:warning=CSS-in-Rust: 处理了 {} 个文件", result.stats.files_processed);
        println!("cargo:warning=CSS-in-Rust: 生成了 {} bytes CSS", result.stats.total_css_size);

        if result.stats.eliminated_selectors > 0 {
            println!("cargo:warning=CSS-in-Rust: 消除了 {} 个未使用的选择器", result.stats.eliminated_selectors);
        }
    }

    Ok(())
}
```

### 2. 高级构建配置

```rust
// build.rs (高级版本)
use css_in_rust::build_tools::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置文件
    let config = load_build_config()?;

    // 创建静态分析器
    let analyzer = StaticAnalyzer::new()
        .with_root_dir(&config.project_root)
        .with_include_patterns(&config.watch_files)
        .with_exclude_patterns(&config.ignore_patterns)
        .with_dependency_analysis(true);

    // 执行静态分析
    let analysis_result = analyzer.analyze()?;

    // 创建构建处理器
    let mut processor = CssBuildProcessor::new(config.clone())?;

    // 应用分析结果
    processor.apply_analysis(&analysis_result)?;

    // 设置自定义优化器
    if config.custom_optimizations {
        processor.add_optimizer(Box::new(CustomCssOptimizer::new()));
    }

    // 设置插件
    for plugin_config in &config.plugins {
        let plugin = load_plugin(plugin_config)?;
        processor.add_plugin(plugin);
    }

    // 执行构建
    let build_result = processor.run()?;

    // 生成构建报告
    if config.generate_reports {
        generate_build_report(&build_result, &config.reports_dir)?;
    }

    // 设置 Cargo 重新构建触发器
    setup_cargo_rerun_triggers(&config)?;

    // 输出构建信息到环境变量
    export_build_info(&build_result)?;

    Ok(())
}

/// 加载构建配置
fn load_build_config() -> Result<BuildConfig, Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // 尝试加载配置文件
    let config_paths = vec![
        manifest_dir.join("css-in-rust.toml"),
        manifest_dir.join(format!("css-in-rust.{}.toml", profile)),
        manifest_dir.join(".css-in-rust.toml"),
    ];

    for config_path in config_paths {
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)?;
            let mut config: BuildConfig = toml::from_str(&config_content)?;

            // 应用环境变量覆盖
            apply_env_overrides(&mut config)?;

            return Ok(config);
        }
    }

    // 使用默认配置
    let mut config = BuildConfig::default();
    config.project_root = manifest_dir;
    apply_env_overrides(&mut config)?;

    Ok(config)
}

/// 应用环境变量覆盖
fn apply_env_overrides(config: &mut BuildConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(value) = env::var("CSS_IN_RUST_MINIFY") {
        config.minify = value.parse()?;
    }

    if let Ok(value) = env::var("CSS_IN_RUST_DEAD_CODE_ELIMINATION") {
        config.dead_code_elimination = value.parse()?;
    }

    if let Ok(value) = env::var("CSS_IN_RUST_PARALLEL_JOBS") {
        config.max_parallel_jobs = value.parse()?;
    }

    if let Ok(value) = env::var("CSS_IN_RUST_OUTPUT_DIR") {
        config.output_dir = PathBuf::from(value);
    }

    Ok(())
}

/// 设置 Cargo 重新构建触发器
fn setup_cargo_rerun_triggers(config: &BuildConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 监控源文件
    for pattern in &config.watch_files {
        if let Some(dir) = pattern.split("/**").next() {
            println!("cargo:rerun-if-changed={}", dir);
        }
    }

    // 监控配置文件
    let config_files = vec![
        "css-in-rust.toml",
        "css-in-rust.development.toml",
        "css-in-rust.production.toml",
        ".css-in-rust.toml",
    ];

    for config_file in config_files {
        println!("cargo:rerun-if-changed={}", config_file);
    }

    // 监控环境变量
    let env_vars = vec![
        "CSS_IN_RUST_MINIFY",
        "CSS_IN_RUST_DEAD_CODE_ELIMINATION",
        "CSS_IN_RUST_PARALLEL_JOBS",
        "CSS_IN_RUST_OUTPUT_DIR",
    ];

    for env_var in env_vars {
        println!("cargo:rerun-if-env-changed={}", env_var);
    }

    Ok(())
}

/// 导出构建信息到环境变量
fn export_build_info(result: &BuildResult) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-env=CSS_IN_RUST_FILES_PROCESSED={}", result.stats.files_processed);
    println!("cargo:rustc-env=CSS_IN_RUST_TOTAL_CSS_SIZE={}", result.stats.total_css_size);
    println!("cargo:rustc-env=CSS_IN_RUST_ELIMINATED_SELECTORS={}", result.stats.eliminated_selectors);
    println!("cargo:rustc-env=CSS_IN_RUST_BUILD_DURATION_MS={}", result.stats.build_duration.as_millis());

    Ok(())
}

/// 生成构建报告
fn generate_build_report(
    result: &BuildResult,
    reports_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(reports_dir)?;

    // 生成 JSON 报告
    let json_report = serde_json::to_string_pretty(&result.stats)?;
    fs::write(reports_dir.join("build-stats.json"), json_report)?;

    // 生成 HTML 报告
    let html_report = generate_html_report(&result.stats)?;
    fs::write(reports_dir.join("build-report.html"), html_report)?;

    println!("cargo:warning=构建报告已生成到 {:?}", reports_dir);

    Ok(())
}

/// 生成 HTML 报告
fn generate_html_report(stats: &BuildStats) -> Result<String, Box<dyn std::error::Error>> {
    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>CSS-in-Rust 构建报告</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .stat {{ margin: 10px 0; }}
        .value {{ font-weight: bold; color: #007bff; }}
    </style>
</head>
<body>
    <h1>CSS-in-Rust 构建报告</h1>
    <div class="stat">处理文件数: <span class="value">{}</span></div>
    <div class="stat">总 CSS 大小: <span class="value">{} bytes</span></div>
    <div class="stat">消除选择器数: <span class="value">{}</span></div>
    <div class="stat">构建耗时: <span class="value">{} ms</span></div>
    <div class="stat">构建时间: <span class="value">{}</span></div>
</body>
</html>
"#,
        stats.files_processed,
        stats.total_css_size,
        stats.eliminated_selectors,
        stats.build_duration.as_millis(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    Ok(html)
}
```

## ⚙️ 运行时配置

### 1. 动态配置管理

```rust
// src/config/runtime.rs
use css_in_rust::runtime::{RuntimeConfig, ConfigManager};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// 运行时配置管理器
pub struct RuntimeConfigManager {
    config: Arc<RwLock<RuntimeConfig>>,
    watchers: Vec<Box<dyn Fn(&RuntimeConfig) + Send + Sync>>,
}

impl RuntimeConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        let config = RuntimeConfig::default();

        Self {
            config: Arc::new(RwLock::new(config)),
            watchers: Vec::new(),
        }
    }

    /// 从本地存储加载配置
    pub fn load_from_storage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;

            if let Some(storage) = window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                if let Ok(Some(config_json)) = storage.get_item("css-in-rust-config") {
                    let config: RuntimeConfig = serde_json::from_str(&config_json)?;
                    self.update_config(config);
                }
            }
        }

        Ok(())
    }

    /// 保存配置到本地存储
    pub fn save_to_storage(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;

            let config = self.config.read().unwrap();
            let config_json = serde_json::to_string(&*config)?;

            if let Some(storage) = window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                storage.set_item("css-in-rust-config", &config_json)?;
            }
        }

        Ok(())
    }

    /// 更新配置
    pub fn update_config(&mut self, new_config: RuntimeConfig) {
        {
            let mut config = self.config.write().unwrap();
            *config = new_config;
        }

        // 通知观察者
        let config = self.config.read().unwrap();
        for watcher in &self.watchers {
            watcher(&*config);
        }

        // 保存到本地存储
        let _ = self.save_to_storage();
    }

    /// 获取配置
    pub fn get_config(&self) -> RuntimeConfig {
        self.config.read().unwrap().clone()
    }

    /// 添加配置观察者
    pub fn add_watcher<F>(&mut self, watcher: F)
    where
        F: Fn(&RuntimeConfig) + Send + Sync + 'static,
    {
        self.watchers.push(Box::new(watcher));
    }

    /// 更新主题
    pub fn set_theme(&mut self, theme_name: String) {
        let mut config = self.config.write().unwrap();
        config.theme.current = theme_name;
        drop(config);

        let config = self.config.read().unwrap();
        for watcher in &self.watchers {
            watcher(&*config);
        }
    }

    /// 切换主题
    pub fn toggle_theme(&mut self) {
        let current_theme = {
            let config = self.config.read().unwrap();
            config.theme.current.clone()
        };

        let new_theme = match current_theme.as_str() {
            "light" => "dark".to_string(),
            "dark" => "light".to_string(),
            _ => "light".to_string(),
        };

        self.set_theme(new_theme);
    }

    /// 更新性能设置
    pub fn update_performance_settings(&mut self, settings: PerformanceSettings) {
        let mut config = self.config.write().unwrap();
        config.performance = settings;
        drop(config);

        let config = self.config.read().unwrap();
        for watcher in &self.watchers {
            watcher(&*config);
        }
    }
}

/// 运行时配置结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuntimeConfig {
    pub theme: ThemeConfig,
    pub performance: PerformanceSettings,
    pub development: DevelopmentSettings,
    pub accessibility: AccessibilitySettings,
    pub user_preferences: UserPreferences,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeConfig {
    pub current: String,
    pub auto_switch: bool,
    pub transition_duration: u32,
    pub respect_system_preference: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceSettings {
    pub enable_caching: bool,
    pub lazy_loading: bool,
    pub preload_critical: bool,
    pub batch_updates: bool,
    pub debounce_ms: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DevelopmentSettings {
    pub hot_reload: bool,
    pub show_debug_info: bool,
    pub verbose_logging: bool,
    pub performance_monitoring: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessibilitySettings {
    pub high_contrast: bool,
    pub reduce_motion: bool,
    pub large_text: bool,
    pub focus_indicators: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub timezone: String,
    pub custom_properties: HashMap<String, String>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig {
                current: "light".to_string(),
                auto_switch: false,
                transition_duration: 300,
                respect_system_preference: true,
            },
            performance: PerformanceSettings {
                enable_caching: true,
                lazy_loading: true,
                preload_critical: true,
                batch_updates: true,
                debounce_ms: 16,
            },
            development: DevelopmentSettings {
                hot_reload: cfg!(debug_assertions),
                show_debug_info: cfg!(debug_assertions),
                verbose_logging: false,
                performance_monitoring: true,
            },
            accessibility: AccessibilitySettings {
                high_contrast: false,
                reduce_motion: false,
                large_text: false,
                focus_indicators: true,
            },
            user_preferences: UserPreferences {
                language: "zh-CN".to_string(),
                timezone: "Asia/Shanghai".to_string(),
                custom_properties: HashMap::new(),
            },
        }
    }
}
```

### 2. 配置 Hook 示例

```rust
// src/hooks/use_config.rs (Yew 示例)
use yew::prelude::*;
use crate::config::RuntimeConfigManager;
use std::rc::Rc;

/// 配置 Hook
#[hook]
pub fn use_config() -> (RuntimeConfig, Callback<RuntimeConfig>) {
    let config_manager = use_state(|| Rc::new(RefCell::new(RuntimeConfigManager::new())));
    let config = use_state(|| config_manager.borrow().get_config());

    // 初始化时加载配置
    use_effect_with_deps(
        {
            let config_manager = config_manager.clone();
            let config = config.clone();

            move |_| {
                let mut manager = config_manager.borrow_mut();
                let _ = manager.load_from_storage();
                config.set(manager.get_config());

                // 添加配置观察者
                manager.add_watcher({
                    let config = config.clone();
                    move |new_config| {
                        config.set(new_config.clone());
                    }
                });

                || {}
            }
        },
        (),
    );

    let update_config = {
        let config_manager = config_manager.clone();

        Callback::from(move |new_config: RuntimeConfig| {
            config_manager.borrow_mut().update_config(new_config);
        })
    };

    ((*config).clone(), update_config)
}

/// 主题 Hook
#[hook]
pub fn use_theme() -> (String, Callback<String>) {
    let (config, update_config) = use_config();

    let set_theme = {
        let config = config.clone();

        Callback::from(move |theme_name: String| {
            let mut new_config = config.clone();
            new_config.theme.current = theme_name;
            update_config.emit(new_config);
        })
    };

    (config.theme.current, set_theme)
}
```

## 📋 配置最佳实践

### ✅ 配置组织
- [ ] 使用分层配置（默认 → 环境 → 用户）
- [ ] 提供合理的默认值
- [ ] 支持环境变量覆盖
- [ ] 使用类型安全的配置结构

### ✅ 性能优化
- [ ] 缓存配置解析结果
- [ ] 使用懒加载加载大型配置
- [ ] 避免频繁的配置更新
- [ ] 合理使用配置观察者

### ✅ 开发体验
- [ ] 提供配置验证和错误提示
- [ ] 支持配置热重载
- [ ] 生成配置文档和示例
- [ ] 提供配置迁移工具

### ✅ 安全考虑
- [ ] 避免在配置中存储敏感信息
- [ ] 验证用户输入的配置值
- [ ] 使用安全的默认配置
- [ ] 限制配置文件的访问权限

通过合理的配置管理，您可以让 CSS-in-Rust 项目更加灵活、可维护和高性能！⚙️
