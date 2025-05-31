# API 参考文档

本文档详细介绍了 CSS-in-Rust 的所有 API。

## 📚 目录

- [核心宏](#核心宏)
- [运行时 API](#运行时-api)
- [主题系统](#主题系统)
- [性能工具](#性能工具)
- [构建工具](#构建工具)
- [热更新](#热更新)
- [开发工具](#开发工具)
- [类型定义](#类型定义)

## 🎯 核心宏

### `css!`

编译时 CSS 处理宏，生成优化的样式类。

**语法**:
```rust
css! {
    // CSS 规则
}
```

**返回值**: `CssStyle`

**示例**:
```rust
use css_in_rust::css;

let button_style = css! {
    background-color: #007bff;
    color: white;
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;

    &:hover {
        background-color: #0056b3;
    }

    &:focus {
        outline: 2px solid #007bff;
        outline-offset: 2px;
    }

    // 嵌套选择器
    .icon {
        margin-right: 8px;
    }

    // 媒体查询
    @media (max-width: 768px) {
        padding: 6px 12px;
        font-size: 14px;
    }
};
```

**支持的 CSS 特性**:
- 标准 CSS 属性
- 嵌套选择器 (`&`, `.class`, `#id`)
- 伪类和伪元素 (`:hover`, `::before`)
- 媒体查询 (`@media`)
- 关键帧动画 (`@keyframes`)
- CSS 变量 (`var(--variable)`)
- CSS 函数 (`calc()`, `rgb()`, `hsl()`)

### `css_if!`

条件样式宏，根据条件应用不同样式。

**语法**:
```rust
css_if! {
    base: {
        // 基础样式
    },
    condition1 => {
        // 条件1为真时的样式
    },
    !condition2 => {
        // 条件2为假时的样式
    },
    condition3 && condition4 => {
        // 多条件组合
    }
}
```

**返回值**: `CssStyle`

**示例**:
```rust
use css_in_rust::css_if;

fn create_button_style(is_primary: bool, is_large: bool, is_disabled: bool) -> CssStyle {
    css_if! {
        base: {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-weight: 500;
            transition: all 0.2s ease;
        },

        is_primary => {
            background-color: #007bff;
            color: white;

            &:hover {
                background-color: #0056b3;
            }
        },

        !is_primary => {
            background-color: #f8f9fa;
            color: #212529;
            border: 1px solid #dee2e6;

            &:hover {
                background-color: #e9ecef;
            }
        },

        is_large => {
            padding: 12px 24px;
            font-size: 16px;
        },

        !is_large => {
            padding: 8px 16px;
            font-size: 14px;
        },

        is_disabled => {
            opacity: 0.6;
            cursor: not-allowed;

            &:hover {
                background-color: inherit;
            }
        }
    }
}
```

### `css_variants!`

变体系统宏，创建可复用的组件变体。

**语法**:
```rust
css_variants! {
    base: {
        // 基础样式
    },
    variants: {
        variant_name: {
            option1: { /* 样式 */ },
            option2: { /* 样式 */ }
        }
    },
    default_variants: {
        variant_name: "option1"
    }
}
```

**返回值**: `CssVariants`

**示例**:
```rust
use css_in_rust::css_variants;

let button_variants = css_variants! {
    base: {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s ease;
    },

    variants: {
        size: {
            sm: {
                padding: 4px 8px;
                font-size: 12px;
            },
            md: {
                padding: 8px 16px;
                font-size: 14px;
            },
            lg: {
                padding: 12px 24px;
                font-size: 16px;
            }
        },

        variant: {
            primary: {
                background-color: #007bff;
                color: white;

                &:hover {
                    background-color: #0056b3;
                }
            },
            secondary: {
                background-color: #6c757d;
                color: white;

                &:hover {
                    background-color: #545b62;
                }
            },
            outline: {
                background-color: transparent;
                border: 1px solid #007bff;
                color: #007bff;

                &:hover {
                    background-color: #007bff;
                    color: white;
                }
            }
        },

        rounded: {
            true: {
                border-radius: 9999px;
            },
            false: {
                border-radius: 4px;
            }
        }
    },

    default_variants: {
        size: "md",
        variant: "primary",
        rounded: false
    }
};

// 使用变体
let primary_large = button_variants.apply([
    ("size", "lg"),
    ("variant", "primary")
]);

let outline_rounded = button_variants.apply([
    ("variant", "outline"),
    ("rounded", true)
]);
```

### `theme!`

定义主题变量。

**语法**:
```rust
theme! {
    variable_name: value,
    another_variable: value
}
```

**返回值**: `Theme`

**示例**:
```rust
use css_in_rust::theme;

let light_theme = theme! {
    // 颜色
    primary: #007bff,
    secondary: #6c757d,
    success: #28a745,
    danger: #dc3545,
    warning: #ffc107,
    info: #17a2b8,

    // 中性色
    white: #ffffff,
    black: #000000,
    gray_50: #f9fafb,
    gray_100: #f3f4f6,
    gray_200: #e5e7eb,
    gray_300: #d1d5db,
    gray_400: #9ca3af,
    gray_500: #6b7280,
    gray_600: #4b5563,
    gray_700: #374151,
    gray_800: #1f2937,
    gray_900: #111827,

    // 语义颜色
    background: #ffffff,
    surface: #f8f9fa,
    text: #212529,
    text_secondary: #6c757d,
    border: #dee2e6,

    // 字体
    font_family_sans: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
    font_family_mono: "SFMono-Regular, Menlo, Monaco, Consolas, monospace",

    // 字体大小
    font_size_xs: 0.75rem,
    font_size_sm: 0.875rem,
    font_size_base: 1rem,
    font_size_lg: 1.125rem,
    font_size_xl: 1.25rem,
    font_size_2xl: 1.5rem,
    font_size_3xl: 1.875rem,
    font_size_4xl: 2.25rem,

    // 间距
    spacing_0: 0,
    spacing_1: 0.25rem,
    spacing_2: 0.5rem,
    spacing_3: 0.75rem,
    spacing_4: 1rem,
    spacing_5: 1.25rem,
    spacing_6: 1.5rem,
    spacing_8: 2rem,
    spacing_10: 2.5rem,
    spacing_12: 3rem,
    spacing_16: 4rem,

    // 圆角
    border_radius_none: 0,
    border_radius_sm: 0.125rem,
    border_radius: 0.25rem,
    border_radius_md: 0.375rem,
    border_radius_lg: 0.5rem,
    border_radius_xl: 0.75rem,
    border_radius_2xl: 1rem,
    border_radius_full: 9999px,

    // 阴影
    shadow_sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
    shadow: "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)",
    shadow_md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
    shadow_lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
    shadow_xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",

    // 断点
    breakpoint_sm: 640px,
    breakpoint_md: 768px,
    breakpoint_lg: 1024px,
    breakpoint_xl: 1280px,
    breakpoint_2xl: 1536px
};
```

### `keyframes!`

定义 CSS 动画关键帧。

**语法**:
```rust
keyframes! {
    from {
        // 起始样式
    }
    to {
        // 结束样式
    }

    // 或使用百分比
    0% {
        // 0% 样式
    }
    50% {
        // 50% 样式
    }
    100% {
        // 100% 样式
    }
}
```

**返回值**: `Keyframes`

**示例**:
```rust
use css_in_rust::{keyframes, css};

// 淡入动画
let fade_in = keyframes! {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
};

// 弹跳动画
let bounce = keyframes! {
    0%, 20%, 53%, 80%, 100% {
        animation-timing-function: cubic-bezier(0.215, 0.61, 0.355, 1);
        transform: translate3d(0, 0, 0);
    }
    40%, 43% {
        animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
        transform: translate3d(0, -30px, 0);
    }
    70% {
        animation-timing-function: cubic-bezier(0.755, 0.05, 0.855, 0.06);
        transform: translate3d(0, -15px, 0);
    }
    90% {
        transform: translate3d(0, -4px, 0);
    }
};

// 脉冲动画
let pulse = keyframes! {
    0% {
        transform: scale(1);
    }
    50% {
        transform: scale(1.05);
    }
    100% {
        transform: scale(1);
    }
};

// 在样式中使用动画
let animated_button = css! {
    background-color: #007bff;
    color: white;
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;

    // 应用淡入动画
    animation: ${fade_in} 0.3s ease-out;

    &:hover {
        // 悬停时应用脉冲动画
        animation: ${pulse} 0.5s ease-in-out infinite;
    }

    &:active {
        // 点击时应用弹跳动画
        animation: ${bounce} 0.6s ease-out;
    }
};
```

### `global_css!`

定义全局 CSS 样式。

**语法**:
```rust
global_css! {
    // 全局 CSS 规则
}
```

**示例**:
```rust
use css_in_rust::global_css;

global_css! {
    :root {
        --font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        --line-height: 1.5;
    }

    * {
        box-sizing: border-box;
    }

    body {
        margin: 0;
        font-family: var(--font-family);
        line-height: var(--line-height);
        color: #212529;
        background-color: #ffffff;
    }

    h1, h2, h3, h4, h5, h6 {
        margin-top: 0;
        margin-bottom: 0.5rem;
        font-weight: 600;
        line-height: 1.2;
    }

    p {
        margin-top: 0;
        margin-bottom: 1rem;
    }

    a {
        color: #007bff;
        text-decoration: none;

        &:hover {
            text-decoration: underline;
        }
    }

    button {
        font-family: inherit;
    }

    input, textarea, select {
        font-family: inherit;
        font-size: inherit;
    }
};
```

### `reset_css!`

应用 CSS 重置样式。

**语法**:
```rust
reset_css! {
    // 重置类型
}
```

**支持的重置类型**:
- `modern` - 现代 CSS 重置
- `normalize` - Normalize.css 风格重置
- `minimal` - 最小化重置

**示例**:
```rust
use css_in_rust::reset_css;

// 现代 CSS 重置
reset_css! {
    modern
};

// 或者自定义重置
reset_css! {
    *,
    *::before,
    *::after {
        box-sizing: border-box;
    }

    * {
        margin: 0;
    }

    html,
    body {
        height: 100%;
    }

    body {
        line-height: 1.5;
        -webkit-font-smoothing: antialiased;
    }

    img,
    picture,
    video,
    canvas,
    svg {
        display: block;
        max-width: 100%;
    }

    input,
    button,
    textarea,
    select {
        font: inherit;
    }

    p,
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
        overflow-wrap: break-word;
    }
};
```

## 🏃 运行时 API

### `CssStyle`

表示编译后的 CSS 样式。

**方法**:

#### `class_name() -> String`

获取生成的 CSS 类名。

```rust
let style = css! { color: red; };
let class = style.class_name(); // "css-1a2b3c4d"
```

#### `css_content() -> String`

获取生成的 CSS 内容。

```rust
let style = css! { color: red; };
let content = style.css_content(); // ".css-1a2b3c4d { color: red; }"
```

#### `inject() -> Result<(), CssError>`

将样式注入到 DOM 中。

```rust
let style = css! { color: red; };
style.inject()?;
```

#### `remove() -> Result<(), CssError>`

从 DOM 中移除样式。

```rust
style.remove()?;
```

### `CssVariants`

表示组件变体集合。

**方法**:

#### `apply(variants: &[("variant_name", "option")]) -> CssStyle`

应用指定的变体组合。

```rust
let variants = css_variants! {
    // ... 变体定义
};

let style = variants.apply([
    ("size", "lg"),
    ("variant", "primary")
]);
```

#### `get_variant(name: &str, option: &str) -> Option<CssStyle>`

获取特定变体选项的样式。

```rust
let size_lg = variants.get_variant("size", "lg");
```

#### `list_variants() -> Vec<String>`

列出所有可用的变体名称。

```rust
let variant_names = variants.list_variants(); // ["size", "variant", "rounded"]
```

#### `list_options(variant: &str) -> Vec<String>`

列出指定变体的所有选项。

```rust
let size_options = variants.list_options("size"); // ["sm", "md", "lg"]
```

### `StyleManager`

全局样式管理器。

**方法**:

#### `global() -> &'static StyleManager`

获取全局样式管理器实例。

```rust
use css_in_rust::runtime::StyleManager;

let manager = StyleManager::global();
```

#### `inject_style(id: &str, css: &str) -> Result<(), CssError>`

注入样式到 DOM。

```rust
manager.inject_style("button", ".button { color: red; }")?;
```

#### `remove_style(id: &str) -> Result<(), CssError>`

从 DOM 移除样式。

```rust
manager.remove_style("button")?;
```

#### `get_injected_styles() -> Vec<String>`

获取所有已注入的样式 ID。

```rust
let styles = manager.get_injected_styles();
```

#### `clear_all() -> Result<(), CssError>`

清除所有注入的样式。

```rust
manager.clear_all()?;
```

#### `set_config(config: StyleManagerConfig)`

设置样式管理器配置。

```rust
use css_in_rust::runtime::StyleManagerConfig;

let config = StyleManagerConfig {
    enable_caching: true,
    cache_size: 1000,
    enable_deduplication: true,
    enable_compression: true,
    lazy_loading: true,
};

manager.set_config(config);
```

## 🎨 主题系统

### `Theme`

主题定义和管理。

**方法**:

#### `set_current(theme: Theme)`

设置当前主题。

```rust
use css_in_rust::Theme;

Theme::set_current(light_theme);
```

#### `get_current() -> Theme`

获取当前主题。

```rust
let current = Theme::get_current();
```

#### `get_variable(name: &str) -> Option<String>`

获取主题变量值。

```rust
let primary_color = Theme::get_variable("primary");
```

#### `set_variable(name: &str, value: &str)`

设置主题变量值。

```rust
Theme::set_variable("primary", "#ff0000");
```

#### `apply_to_dom() -> Result<(), CssError>`

将主题变量应用到 DOM。

```rust
Theme::apply_to_dom()?;
```

### `ThemeManager`

主题管理器。

**方法**:

#### `new() -> ThemeManager`

创建新的主题管理器。

```rust
use css_in_rust::themes::ThemeManager;

let theme_manager = ThemeManager::new();
```

#### `register_theme(name: &str, theme: Theme)`

注册主题。

```rust
theme_manager.register_theme("light", light_theme);
theme_manager.register_theme("dark", dark_theme);
```

#### `set_theme(name: &str) -> Result<(), ThemeError>`

切换到指定主题。

```rust
theme_manager.set_theme("dark")?;
```

#### `get_theme(name: &str) -> Option<&Theme>`

获取指定主题。

```rust
let dark_theme = theme_manager.get_theme("dark");
```

#### `list_themes() -> Vec<String>`

列出所有注册的主题。

```rust
let themes = theme_manager.list_themes(); // ["light", "dark"]
```

#### `watch_system_theme() -> Result<(), ThemeError>`

监听系统主题变化。

```rust
theme_manager.watch_system_theme()?;
```

## ⚡ 性能工具

### `PerformanceManager`

性能管理和监控。

**方法**:

#### `new() -> PerformanceManager`

创建性能管理器。

```rust
use css_in_rust::performance::PerformanceManager;

let perf_manager = PerformanceManager::new();
```

#### `with_config(config: PerformanceConfig) -> PerformanceManager`

使用配置创建性能管理器。

```rust
use css_in_rust::performance::PerformanceConfig;

let config = PerformanceConfig {
    enable_metrics: true,
    enable_profiling: true,
    enable_caching: true,
    cache_size: 10000,
    enable_incremental: true,
};

let perf_manager = PerformanceManager::with_config(config);
```

#### `start_profiling(session_name: &str)`

开始性能分析会话。

```rust
perf_manager.start_profiling("css-compilation");
```

#### `end_profiling(session_name: &str) -> ProfilingReport`

结束性能分析会话并获取报告。

```rust
let report = perf_manager.end_profiling("css-compilation");
println!("总耗时: {:?}", report.total_duration);
println!("内存使用: {} MB", report.peak_memory / 1024 / 1024);
```

#### `get_metrics() -> PerformanceMetrics`

获取性能指标。

```rust
let metrics = perf_manager.get_metrics();
println!("编译次数: {}", metrics.compilation_count);
println!("缓存命中率: {:.2}%", metrics.cache_hit_rate * 100.0);
```

#### `clear_cache()`

清除性能缓存。

```rust
perf_manager.clear_cache();
```

### `MetricsCollector`

指标收集器。

**方法**:

#### `new() -> MetricsCollector`

创建指标收集器。

```rust
use css_in_rust::performance::MetricsCollector;

let collector = MetricsCollector::new();
```

#### `record_operation(op_type: OperationType, duration: Duration)`

记录操作指标。

```rust
use css_in_rust::performance::OperationType;
use std::time::Duration;

collector.record_operation(
    OperationType::Parsing,
    Duration::from_millis(50)
);
```

#### `get_stats() -> PerformanceStats`

获取统计数据。

```rust
let stats = collector.get_stats();
println!("平均解析时间: {:?}", stats.avg_parsing_time);
```

## 🔧 构建工具

### `CssBuildProcessor`

构建时 CSS 处理器。

**方法**:

#### `new(config: BuildConfig) -> CssBuildProcessor`

创建构建处理器。

```rust
use css_in_rust::build_tools::{CssBuildProcessor, BuildConfig};

let config = BuildConfig {
    project_root: "./".into(),
    output_dir: "dist".into(),
    enable_dead_code_elimination: true,
    enable_minification: true,
    generate_report: true,
    usage_threshold: 0.0,
    aggressive_elimination: false,
};

let processor = CssBuildProcessor::new(config);
```

#### `run() -> Result<BuildResult, BuildError>`

执行构建处理。

```rust
let result = processor.run()?;
println!("处理了 {} 个文件", result.processed_files.len());
println!("压缩率: {:.1}%", result.compression_ratio * 100.0);
```

#### `analyze_usage() -> Result<CssUsageReport, BuildError>`

分析 CSS 使用情况。

```rust
let usage_report = processor.analyze_usage()?;
println!("使用的类: {:?}", usage_report.used_classes);
println!("未使用的类: {:?}", usage_report.unused_classes);
```

### `StaticAnalyzer`

静态代码分析器。

**方法**:

#### `new(project_root: PathBuf) -> StaticAnalyzer`

创建静态分析器。

```rust
use css_in_rust::build_tools::StaticAnalyzer;
use std::path::PathBuf;

let analyzer = StaticAnalyzer::new(PathBuf::from("./"));
```

#### `analyze() -> Result<CssUsageReport, AnalysisError>`

执行静态分析。

```rust
let report = analyzer.analyze()?;
println!("分析了 {} 个文件", report.analyzed_files.len());
println!("找到 {} 个 CSS 宏调用", report.css_macro_calls.len());
```

#### `set_include_patterns(patterns: Vec<String>)`

设置包含模式。

```rust
analyzer.set_include_patterns(vec![
    "src/**/*.rs".to_string(),
    "components/**/*.rs".to_string(),
]);
```

#### `set_exclude_patterns(patterns: Vec<String>)`

设置排除模式。

```rust
analyzer.set_exclude_patterns(vec![
    "target/**".to_string(),
    "**/.git/**".to_string(),
]);
```

## 🔥 热更新

### `HotReloadManager`

热更新管理器。

**方法**:

#### `new(config: HotReloadConfig) -> HotReloadManager`

创建热更新管理器。

```rust
use css_in_rust::hot_reload::{HotReloadManager, HotReloadConfig};

let config = HotReloadConfig {
    watch_paths: vec!["src/".into(), "styles/".into()],
    ignore_patterns: vec!["target/**".to_string()],
    websocket_port: 3001,
    enable_css_hot_reload: true,
    enable_full_reload: false,
    debounce_ms: 100,
};

let mut hot_reload = HotReloadManager::new(config);
```

#### `start() -> Result<(), HotReloadError>`

启动热更新服务。

```rust
hot_reload.start().await?;
```

#### `stop() -> Result<(), HotReloadError>`

停止热更新服务。

```rust
hot_reload.stop().await?;
```

#### `is_running() -> bool`

检查服务是否运行。

```rust
if hot_reload.is_running() {
    println!("热更新服务正在运行");
}
```

#### `get_stats() -> HotReloadStats`

获取热更新统计。

```rust
let stats = hot_reload.get_stats();
println!("重载次数: {}", stats.reload_count);
println!("连接的客户端: {}", stats.connected_clients);
```

### `FileWatcher`

文件监控器。

**方法**:

#### `new(config: FileWatcherConfig) -> FileWatcher`

创建文件监控器。

```rust
use css_in_rust::hot_reload::{FileWatcher, FileWatcherConfig};

let config = FileWatcherConfig {
    watch_paths: vec!["src/".into()],
    ignore_patterns: vec!["target/**".to_string()],
    recursive: true,
    debounce_ms: 100,
};

let watcher = FileWatcher::new(config);
```

#### `start_watching() -> Result<(), WatchError>`

开始监控文件。

```rust
watcher.start_watching()?;
```

#### `stop_watching() -> Result<(), WatchError>`

停止监控文件。

```rust
watcher.stop_watching()?;
```

#### `add_event_handler<F>(handler: F)` where `F: Fn(WatchEvent) + Send + 'static`

添加事件处理器。

```rust
watcher.add_event_handler(|event| {
    println!("文件变更: {:?}", event);
});
```

## 🛠️ 开发工具

### `DiagnosticManager`

诊断管理器。

**方法**:

#### `new() -> DiagnosticManager`

创建诊断管理器。

```rust
use css_in_rust::dev_experience::DiagnosticManager;

let diagnostics = DiagnosticManager::new();
```

#### `with_config(config: DiagnosticConfig) -> DiagnosticManager`

使用配置创建诊断管理器。

```rust
use css_in_rust::dev_experience::DiagnosticConfig;

let config = DiagnosticConfig {
    enable_syntax_check: true,
    enable_performance_hints: true,
    enable_accessibility_check: true,
    enable_unused_detection: true,
    strict_mode: false,
};

let diagnostics = DiagnosticManager::with_config(config);
```

#### `analyze_css(css: &str) -> Vec<Diagnostic>`

分析 CSS 代码。

```rust
let css_code = r#"
    .button {
        background-color: #007bff;
        color: white;
    }
"#;

let issues = diagnostics.analyze_css(css_code);
for issue in issues {
    println!("{}: {}", issue.level, issue.message);
}
```

#### `analyze_file(path: &Path) -> Result<Vec<Diagnostic>, DiagnosticError>`

分析文件。

```rust
use std::path::Path;

let issues = diagnostics.analyze_file(Path::new("styles.css"))?;
```

#### `enable_real_time_analysis()`

启用实时分析。

```rust
diagnostics.enable_real_time_analysis();
```

### `SyntaxHighlighter`

语法高亮器。

**方法**:

#### `new() -> SyntaxHighlighter`

创建语法高亮器。

```rust
use css_in_rust::dev_experience::SyntaxHighlighter;

let highlighter = SyntaxHighlighter::new();
```

#### `highlight_css(css: &str) -> Result<String, HighlightError>`

高亮 CSS 代码。

```rust
let css = ".button { color: red; }";
let highlighted = highlighter.highlight_css(css)?;
```

#### `set_theme(theme: HighlightTheme)`

设置高亮主题。

```rust
use css_in_rust::dev_experience::HighlightTheme;

highlighter.set_theme(HighlightTheme::Dark);
```

#### `highlight_to_html(css: &str) -> Result<String, HighlightError>`

生成 HTML 高亮代码。

```rust
let html = highlighter.highlight_to_html(css)?;
```

## 📋 类型定义

### 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum CssError {
    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("编译错误: {0}")]
    CompilationError(String),

    #[error("运行时错误: {0}")]
    RuntimeError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("主题未找到: {0}")]
    ThemeNotFound(String),

    #[error("变量未找到: {0}")]
    VariableNotFound(String),

    #[error("主题格式错误: {0}")]
    FormatError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("构建失败: {0}")]
    BuildFailed(String),

    #[error("分析错误: {0}")]
    AnalysisError(String),

    #[error("文件错误: {0}")]
    FileError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum HotReloadError {
    #[error("服务器启动失败: {0}")]
    ServerStartFailed(String),

    #[error("文件监控失败: {0}")]
    WatchError(String),

    #[error("WebSocket 错误: {0}")]
    WebSocketError(String),
}
```

### 配置类型

```rust
#[derive(Debug, Clone)]
pub struct StyleManagerConfig {
    pub enable_caching: bool,
    pub cache_size: usize,
    pub enable_deduplication: bool,
    pub enable_compression: bool,
    pub lazy_loading: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_metrics: bool,
    pub enable_profiling: bool,
    pub enable_caching: bool,
    pub cache_size: usize,
    pub enable_incremental: bool,
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub project_root: PathBuf,
    pub output_dir: PathBuf,
    pub enable_dead_code_elimination: bool,
    pub enable_minification: bool,
    pub generate_report: bool,
    pub usage_threshold: f64,
    pub aggressive_elimination: bool,
}

#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    pub watch_paths: Vec<PathBuf>,
    pub ignore_patterns: Vec<String>,
    pub websocket_port: u16,
    pub enable_css_hot_reload: bool,
    pub enable_full_reload: bool,
    pub debounce_ms: u64,
}

#[derive(Debug, Clone)]
pub struct DiagnosticConfig {
    pub enable_syntax_check: bool,
    pub enable_performance_hints: bool,
    pub enable_accessibility_check: bool,
    pub enable_unused_detection: bool,
    pub strict_mode: bool,
}
```

### 结果类型

```rust
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub processed_files: Vec<ProcessedFile>,
    pub compression_ratio: f64,
    pub total_size_before: u64,
    pub total_size_after: u64,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct CssUsageReport {
    pub used_classes: HashSet<String>,
    pub used_ids: HashSet<String>,
    pub unused_classes: HashSet<String>,
    pub unused_ids: HashSet<String>,
    pub analyzed_files: Vec<PathBuf>,
    pub css_macro_calls: Vec<CssMacroCall>,
    pub metadata: AnalysisMetadata,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub compilation_count: u64,
    pub compilation_time: Duration,
    pub cache_hit_rate: f64,
    pub memory_usage: u64,
    pub css_size_before: u64,
    pub css_size_after: u64,
}

#[derive(Debug, Clone)]
pub struct ProfilingReport {
    pub session_name: String,
    pub total_duration: Duration,
    pub peak_memory: u64,
    pub operations: Vec<ProfiledOperation>,
    pub bottlenecks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HotReloadStats {
    pub reload_count: u64,
    pub connected_clients: u32,
    pub average_reload_time: Duration,
    pub last_reload_time: Option<SystemTime>,
}
```

---

这份 API 参考文档涵盖了 CSS-in-Rust 的所有主要功能和接口。如需更多详细信息，请参考具体的模块文档或示例代码。
