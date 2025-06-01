# CSS-in-Rust 完整指南

一个现代化的 Rust CSS-in-Rust 解决方案，提供类型安全、高性能的样式管理。

## 🚀 特性

### 核心功能
- **类型安全的 CSS**: 编译时 CSS 验证和类型检查
- **高性能优化**: 基于 LightningCSS 的极速 CSS 处理
- **智能死代码消除**: 自动移除未使用的 CSS 规则
- **主题系统**: 内置主题切换和自定义主题支持
- **变体系统**: 响应式设计和状态变体
- **多框架支持**: 支持 Yew、Leptos、Dioxus 等主流框架

### 开发体验
- **热更新**: 实时 CSS 热重载，无需刷新页面
- **智能诊断**: 详细的错误提示和性能建议
- **语法高亮**: IDE 中的 CSS 语法高亮支持
- **自动补全**: 智能的 CSS 属性和值补全
- **性能分析**: 详细的编译和运行时性能指标

### 构建优化
- **增量编译**: 只重新编译变更的样式
- **并行处理**: 多线程 CSS 处理和优化
- **缓存系统**: 智能缓存减少重复编译
- **Tree Shaking**: 自动移除未使用的样式代码
- **压缩优化**: 生产环境 CSS 压缩和优化

## 📦 安装

### 基础安装

```toml
[dependencies]
css-in-rust = "0.1.0"
css-in-rust-macros = "0.1.0"

[build-dependencies]
css-in-rust = { version = "0.1.0", features = ["build-tools"] }
```

### 功能特性

```toml
[dependencies]
css-in-rust = { version = "0.1.0", features = [
    "themes",           # 主题系统
    "variants",         # 变体系统
    "hot-reload",       # 热更新
    "performance",      # 性能优化
    "diagnostics",      # 诊断工具
    "yew",             # Yew 框架支持
    "leptos",          # Leptos 框架支持
    "dioxus",          # Dioxus 框架支持
] }
```

## 🎯 快速开始

### 基础用法

```rust
use css_in_rust::css;

fn main() {
    // 基础 CSS 样式
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

        &:active {
            transform: translateY(1px);
        }
    };

    println!("Button class: {}", button_style.class_name());
}
```

### 条件样式

```rust
use css_in_rust::{css, css_if};

fn component(is_primary: bool, is_disabled: bool) -> String {
    css_if! {
        // 基础样式
        base: {
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        },

        // 条件样式
        is_primary => {
            background-color: #007bff;
            color: white;
        },

        !is_primary => {
            background-color: #f8f9fa;
            color: #212529;
            border: 1px solid #dee2e6;
        },

        is_disabled => {
            opacity: 0.6;
            cursor: not-allowed;
        }
    }.class_name()
}
```

### 主题系统

```rust
use css_in_rust::{css, theme, Theme};

// 定义主题
let light_theme = theme! {
    primary: #007bff,
    secondary: #6c757d,
    background: #ffffff,
    text: #212529,
    border: #dee2e6
};

let dark_theme = theme! {
    primary: #0d6efd,
    secondary: #6c757d,
    background: #212529,
    text: #ffffff,
    border: #495057
};

// 使用主题变量
let card_style = css! {
    background-color: var(--background);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px;

    .header {
        color: var(--primary);
        font-weight: bold;
        margin-bottom: 8px;
    }
};

// 应用主题
Theme::set_current(light_theme);
```

### 响应式设计

```rust
use css_in_rust::{css, breakpoints};

let responsive_grid = css! {
    display: grid;
    gap: 16px;

    // 移动端：单列
    grid-template-columns: 1fr;

    // 平板：双列
    @media (min-width: 768px) {
        grid-template-columns: repeat(2, 1fr);
    }

    // 桌面：三列
    @media (min-width: 1024px) {
        grid-template-columns: repeat(3, 1fr);
    }

    // 大屏：四列
    @media (min-width: 1280px) {
        grid-template-columns: repeat(4, 1fr);
    }
};
```

## 🎨 高级功能

### 变体系统

```rust
use css_in_rust::{css_variants, Variant};

// 定义按钮变体
let button_variants = css_variants! {
    base: {
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s ease;
    },

    variants: {
        // 尺寸变体
        size: {
            small: {
                padding: 4px 8px;
                font-size: 12px;
            },
            medium: {
                padding: 8px 16px;
                font-size: 14px;
            },
            large: {
                padding: 12px 24px;
                font-size: 16px;
            }
        },

        // 颜色变体
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
        }
    },

    // 默认变体
    default_variants: {
        size: "medium",
        variant: "primary"
    }
};

// 使用变体
let primary_large = button_variants.apply([
    ("size", "large"),
    ("variant", "primary")
]);

let outline_small = button_variants.apply([
    ("size", "small"),
    ("variant", "outline")
]);
```

### 动画系统

```rust
use css_in_rust::{css, keyframes, animation};

// 定义关键帧动画
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

// 使用动画
let animated_card = css! {
    background: white;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

    // 应用淡入动画
    animation: ${fade_in} 0.3s ease-out;

    &:hover {
        // 悬停时应用弹跳动画
        animation: ${bounce} 1s;
    }
};
```

### 全局样式

```rust
use css_in_rust::{global_css, reset_css};

// CSS 重置
reset_css! {
    // 使用内置的现代 CSS 重置
    modern
};

// 全局样式
global_css! {
    :root {
        --font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        --line-height: 1.5;
        --color-primary: #007bff;
        --color-secondary: #6c757d;
    }

    body {
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
        color: var(--color-primary);
        text-decoration: none;

        &:hover {
            text-decoration: underline;
        }
    }
};
```

## 🔧 框架集成

### Yew 集成

```rust
use yew::prelude::*;
use css_in_rust::css;

#[function_component(Button)]
fn button(props: &ButtonProps) -> Html {
    let style = css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;

        &:hover {
            background-color: #0056b3;
        }
    };

    html! {
        <button class={style.class_name()} onclick={props.onclick.clone()}>
            {&props.children}
        </button>
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}
```

### Leptos 集成

```rust
use leptos::*;
use css_in_rust::css;

#[component]
fn Button(
    cx: Scope,
    #[prop(into)] on_click: Callback<MouseEvent>,
    children: Children,
) -> impl IntoView {
    let style = css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;

        &:hover {
            background-color: #0056b3;
        }
    };

    view! { cx,
        <button
            class=style.class_name()
            on:click=move |ev| on_click.call(ev)
        >
            {children(cx)}
        </button>
    }
}
```

### Dioxus 集成

```rust
use dioxus::prelude::*;
use css_in_rust::css;

#[derive(Props)]
struct ButtonProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let style = css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;

        &:hover {
            background-color: #0056b3;
        }
    };

    render! {
        button {
            class: "{style.class_name()}",
            onclick: move |evt| cx.props.onclick.call(evt),
            &cx.props.children
        }
    }
}
```

## ⚡ 性能优化

### 编译时优化

```rust
// build.rs
use css_in_rust::build_tools::CssBuildProcessor;

fn main() {
    let processor = CssBuildProcessor::new()
        .with_dead_code_elimination(true)
        .with_minification(true)
        .with_source_maps(true)
        .with_cache(true);

    if let Err(e) = processor.run() {
        panic!("CSS 构建失败: {}", e);
    }

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=styles/");
}
```

### 运行时优化

```rust
use css_in_rust::runtime::{StyleManager, StyleManagerConfig};

// 配置样式管理器
let config = StyleManagerConfig {
    enable_caching: true,
    cache_size: 1000,
    enable_deduplication: true,
    enable_compression: true,
    lazy_loading: true,
};

let style_manager = StyleManager::with_config(config);

// 预加载关键样式
style_manager.preload_critical_styles(&[
    "button",
    "card",
    "layout"
]);
```

### 性能监控

```rust
use css_in_rust::performance::{PerformanceManager, PerformanceConfig};

let perf_config = PerformanceConfig {
    enable_metrics: true,
    enable_profiling: true,
    enable_caching: true,
    cache_size: 10000,
    enable_incremental: true,
};

let perf_manager = PerformanceManager::with_config(perf_config);

// 获取性能指标
let metrics = perf_manager.get_metrics();
println!("编译时间: {:?}", metrics.compilation_time);
println!("缓存命中率: {:.2}%", metrics.cache_hit_rate * 100.0);
println!("内存使用: {} MB", metrics.memory_usage / 1024 / 1024);
```

## 🔥 热更新

### 开发服务器配置

```rust
use css_in_rust::hot_reload::{HotReloadManager, HotReloadConfig};

#[tokio::main]
async fn main() {
    let config = HotReloadConfig {
        watch_paths: vec![
            "src/".into(),
            "styles/".into(),
        ],
        ignore_patterns: vec![
            "target/**".to_string(),
            "**/.git/**".to_string(),
        ],
        websocket_port: 3001,
        enable_css_hot_reload: true,
        enable_full_reload: true,
        debounce_ms: 100,
    };

    let mut hot_reload = HotReloadManager::new(config);

    // 启动热更新服务
    hot_reload.start().await.expect("启动热更新失败");

    println!("🔥 热更新服务已启动");

    // 保持服务运行
    tokio::signal::ctrl_c().await.expect("等待 Ctrl+C 信号失败");

    hot_reload.stop().await;
}
```

### 客户端集成

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>CSS-in-Rust App</title>
</head>
<body>
    <div id="app"></div>

    <!-- 热更新客户端 -->
    <script>
        if (typeof window !== 'undefined') {
            const ws = new WebSocket('ws://localhost:3001');

            ws.onmessage = function(event) {
                const message = JSON.parse(event.data);

                switch (message.type) {
                    case 'CssHotReload':
                        // 热重载 CSS
                        updateCSS(message.data.css_content);
                        break;

                    case 'FullReload':
                        // 完全重新加载
                        window.location.reload();
                        break;

                    case 'BuildStatus':
                        // 显示构建状态
                        showBuildStatus(message.data.status, message.data.message);
                        break;
                }
            };

            function updateCSS(cssContent) {
                const styleId = 'css-in-rust-hot-reload';
                let styleElement = document.getElementById(styleId);

                if (!styleElement) {
                    styleElement = document.createElement('style');
                    styleElement.id = styleId;
                    document.head.appendChild(styleElement);
                }

                styleElement.textContent = cssContent;
            }

            function showBuildStatus(status, message) {
                console.log(`构建状态: ${status} - ${message}`);
            }
        }
    </script>
</body>
</html>
```

## 🛠️ 开发工具

### VS Code 插件

安装 CSS-in-Rust VS Code 插件以获得最佳开发体验：

- CSS 语法高亮
- 智能代码补全
- 错误诊断
- 格式化支持
- 主题预览

### CLI 工具

```bash
# 安装 CLI 工具
cargo install css-in-rust-cli

# 分析 CSS 使用情况
css-in-rust analyze --project ./my-project

# 优化 CSS 文件
css-in-rust optimize --input styles/ --output dist/

# 生成性能报告
css-in-rust report --format html --output report.html

# 启动开发服务器
css-in-rust dev --port 3000 --hot-reload
```

### 诊断工具

```rust
use css_in_rust::dev_experience::{DiagnosticManager, DiagnosticConfig};

let diagnostic_config = DiagnosticConfig {
    enable_syntax_check: true,
    enable_performance_hints: true,
    enable_accessibility_check: true,
    enable_unused_detection: true,
    strict_mode: false,
};

let diagnostics = DiagnosticManager::with_config(diagnostic_config);

// 分析 CSS 代码
let css_code = r#"
    .button {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
    }
"#;

let issues = diagnostics.analyze_css(css_code);
for issue in issues {
    println!("{}: {} ({}:{})",
        issue.level,
        issue.message,
        issue.location.line,
        issue.location.column
    );
}
```

## 📚 API 参考

### 核心宏

#### `css!`

编译时 CSS 处理宏，生成优化的样式类。

```rust
let style = css! {
    // CSS 规则
};
```

#### `css_if!`

条件样式宏，根据条件应用不同样式。

```rust
let style = css_if! {
    base: { /* 基础样式 */ },
    condition => { /* 条件样式 */ }
};
```

#### `css_class!`

生成 CSS 类名，不注入样式。

```rust
let class_name = css_class!("my-component");
```

#### `theme!`

定义主题变量。

```rust
let theme = theme! {
    primary: #007bff,
    secondary: #6c757d
};
```

#### `keyframes!`

定义 CSS 动画关键帧。

```rust
let animation = keyframes! {
    from { opacity: 0; }
    to { opacity: 1; }
};
```

### 运行时 API

#### `StyleManager`

样式管理器，负责运行时样式注入和管理。

```rust
use css_in_rust::runtime::StyleManager;

let manager = StyleManager::new();
manager.inject_style("button", css_content);
manager.remove_style("button");
```

#### `ThemeManager`

主题管理器，处理主题切换和变量管理。

```rust
use css_in_rust::themes::ThemeManager;

let theme_manager = ThemeManager::new();
theme_manager.set_theme("dark");
theme_manager.get_variable("primary");
```

### 构建工具 API

#### `CssBuildProcessor`

构建时 CSS 处理器。

```rust
use css_in_rust::build_tools::CssBuildProcessor;

let processor = CssBuildProcessor::new()
    .with_dead_code_elimination(true)
    .with_minification(true);

processor.run()?;
```

#### `StaticAnalyzer`

静态代码分析器。

```rust
use css_in_rust::build_tools::StaticAnalyzer;

let analyzer = StaticAnalyzer::new(project_root);
let report = analyzer.analyze()?;
```

## 🔧 配置

### 项目配置

创建 `css-in-rust.toml` 配置文件：

```toml
[general]
# 项目名称
name = "my-app"
# 输出目录
output_dir = "dist"
# 是否启用开发模式
development = true

[optimization]
# 启用死代码消除
dead_code_elimination = true
# 启用压缩
minification = true
# 启用 Source Maps
source_maps = true
# 使用阈值
usage_threshold = 0.0

[themes]
# 默认主题
default = "light"
# 主题文件路径
theme_dir = "themes/"
# 启用主题切换
enable_switching = true

[hot_reload]
# 启用热更新
enable = true
# WebSocket 端口
port = 3001
# 监听路径
watch_paths = ["src/", "styles/"]
# 忽略模式
ignore_patterns = ["target/**", "**/.git/**"]

[performance]
# 启用缓存
enable_caching = true
# 缓存大小
cache_size = 10000
# 启用增量编译
incremental_compilation = true
# 启用并行处理
parallel_processing = true

[diagnostics]
# 启用语法检查
syntax_check = true
# 启用性能提示
performance_hints = true
# 启用可访问性检查
accessibility_check = true
# 严格模式
strict_mode = false
```

### 环境变量

```bash
# 开发模式
CSS_IN_RUST_DEV=true

# 日志级别
CSS_IN_RUST_LOG=info

# 缓存目录
CSS_IN_RUST_CACHE_DIR=.cache/css-in-rust

# 热更新端口
CSS_IN_RUST_HMR_PORT=3001

# 性能分析
CSS_IN_RUST_PROFILE=true
```

## 🚀 最佳实践

### 1. 组织样式代码

```rust
// styles/mod.rs
pub mod components;
pub mod layouts;
pub mod themes;
pub mod utilities;

// 重新导出常用样式
pub use components::*;
pub use layouts::*;
pub use utilities::*;
```

```rust
// styles/components/button.rs
use css_in_rust::{css, css_variants};

pub fn button_base() -> String {
    css! {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;

        &:focus {
            outline: 2px solid var(--focus-color);
            outline-offset: 2px;
        }

        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
    }.class_name()
}

pub fn button_variants() -> ButtonVariants {
    css_variants! {
        base: button_base(),

        variants: {
            size: {
                sm: { padding: 4px 8px; font-size: 12px; },
                md: { padding: 8px 16px; font-size: 14px; },
                lg: { padding: 12px 24px; font-size: 16px; }
            },

            variant: {
                primary: {
                    background-color: var(--primary);
                    color: white;
                },
                secondary: {
                    background-color: var(--secondary);
                    color: white;
                },
                outline: {
                    background-color: transparent;
                    border: 1px solid var(--primary);
                    color: var(--primary);
                }
            }
        },

        default_variants: {
            size: "md",
            variant: "primary"
        }
    }
}
```

### 2. 性能优化

```rust
// 使用静态样式避免重复计算
use std::sync::LazyLock;

static BUTTON_STYLE: LazyLock<String> = LazyLock::new(|| {
    css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
    }.class_name()
});

// 在组件中使用
fn button_component() -> Html {
    html! {
        <button class={BUTTON_STYLE.clone()}>
            {"Click me"}
        </button>
    }
}
```

### 3. 主题设计

```rust
// themes/mod.rs
use css_in_rust::theme;

pub fn light_theme() -> Theme {
    theme! {
        // 颜色系统
        primary: #007bff,
        primary-hover: #0056b3,
        primary-active: #004085,

        secondary: #6c757d,
        secondary-hover: #545b62,
        secondary-active: #3d4142,

        // 语义颜色
        success: #28a745,
        warning: #ffc107,
        danger: #dc3545,
        info: #17a2b8,

        // 中性色
        white: #ffffff,
        gray-100: #f8f9fa,
        gray-200: #e9ecef,
        gray-300: #dee2e6,
        gray-400: #ced4da,
        gray-500: #adb5bd,
        gray-600: #6c757d,
        gray-700: #495057,
        gray-800: #343a40,
        gray-900: #212529,
        black: #000000,

        // 字体
        font-family-sans: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        font-family-mono: "SFMono-Regular, Menlo, Monaco, Consolas, monospace",

        // 字体大小
        font-size-xs: 0.75rem,
        font-size-sm: 0.875rem,
        font-size-base: 1rem,
        font-size-lg: 1.125rem,
        font-size-xl: 1.25rem,
        font-size-2xl: 1.5rem,
        font-size-3xl: 1.875rem,
        font-size-4xl: 2.25rem,

        // 间距
        spacing-0: 0,
        spacing-1: 0.25rem,
        spacing-2: 0.5rem,
        spacing-3: 0.75rem,
        spacing-4: 1rem,
        spacing-5: 1.25rem,
        spacing-6: 1.5rem,
        spacing-8: 2rem,
        spacing-10: 2.5rem,
        spacing-12: 3rem,
        spacing-16: 4rem,

        // 圆角
        border-radius-none: 0,
        border-radius-sm: 0.125rem,
        border-radius: 0.25rem,
        border-radius-md: 0.375rem,
        border-radius-lg: 0.5rem,
        border-radius-xl: 0.75rem,
        border-radius-2xl: 1rem,
        border-radius-full: 9999px,

        // 阴影
        shadow-sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
        shadow: "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)",
        shadow-md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
        shadow-lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
        shadow-xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",

        // 断点
        breakpoint-sm: 640px,
        breakpoint-md: 768px,
        breakpoint-lg: 1024px,
        breakpoint-xl: 1280px,
        breakpoint-2xl: 1536px
    }
}

pub fn dark_theme() -> Theme {
    theme! {
        // 继承 light_theme 并覆盖特定值
        ..light_theme(),

        // 深色模式特定颜色
        primary: #0d6efd,

        // 背景色
        white: #1a1a1a,
        gray-100: #2d2d2d,
        gray-200: #3d3d3d,
        gray-300: #4d4d4d,
        gray-800: #e0e0e0,
        gray-900: #ffffff,
        black: #ffffff
    }
}
```

### 4. 响应式设计

```rust
use css_in_rust::{css, breakpoint};

// 定义响应式工具
pub fn responsive_grid(cols: &[u8]) -> String {
    css! {
        display: grid;
        gap: 1rem;

        // 移动端
        grid-template-columns: repeat(${cols[0]}, 1fr);

        // 平板
        @media (min-width: 768px) {
            grid-template-columns: repeat(${cols[1]}, 1fr);
        }

        // 桌面
        @media (min-width: 1024px) {
            grid-template-columns: repeat(${cols[2]}, 1fr);
        }

        // 大屏
        @media (min-width: 1280px) {
            grid-template-columns: repeat(${cols[3]}, 1fr);
        }
    }.class_name()
}

// 使用响应式网格
let grid_class = responsive_grid(&[1, 2, 3, 4]); // 1列 -> 2列 -> 3列 -> 4列
```

### 5. 可访问性

```rust
use css_in_rust::css;

// 可访问的按钮样式
pub fn accessible_button() -> String {
    css! {
        // 基础样式
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;

        // 焦点样式
        &:focus {
            outline: 2px solid var(--focus-color, #007bff);
            outline-offset: 2px;
        }

        // 高对比度模式支持
        @media (prefers-contrast: high) {
            border: 2px solid currentColor;
        }

        // 减少动画偏好
        @media (prefers-reduced-motion: reduce) {
            transition: none;
        }

        // 禁用状态
        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }

        // 确保最小触摸目标大小 (44x44px)
        min-height: 44px;
        min-width: 44px;
    }.class_name()
}
```

## 🐛 故障排除

### 常见问题

#### 1. 样式不生效

**问题**: CSS 样式没有应用到元素上。

**解决方案**:
- 检查类名是否正确应用
- 确认样式已正确注入到 DOM
- 检查 CSS 优先级冲突
- 验证构建配置

```rust
// 调试样式注入
use css_in_rust::runtime::StyleManager;

let manager = StyleManager::global();
let injected_styles = manager.get_injected_styles();
println!("已注入的样式: {:?}", injected_styles);
```

#### 2. 热更新不工作

**问题**: 文件修改后样式没有自动更新。

**解决方案**:
- 检查 WebSocket 连接状态
- 确认文件监听路径配置
- 检查防火墙设置
- 验证端口是否被占用

```rust
// 检查热更新状态
use css_in_rust::hot_reload::HotReloadManager;

let manager = HotReloadManager::new(config);
if !manager.is_connected() {
    println!("热更新连接失败");
}
```

#### 3. 构建性能问题

**问题**: CSS 编译速度慢。

**解决方案**:
- 启用增量编译
- 使用缓存
- 减少不必要的文件监听
- 优化 CSS 复杂度

```rust
// 性能分析
use css_in_rust::performance::PerformanceProfiler;

let profiler = PerformanceProfiler::new();
profiler.start_session("css-compilation");

// ... CSS 编译代码 ...

let report = profiler.end_session();
println!("编译耗时: {:?}", report.total_duration);
```

#### 4. 内存使用过高

**问题**: 应用内存占用过多。

**解决方案**:
- 调整缓存大小
- 启用样式去重
- 清理未使用的样式
- 使用懒加载

```rust
// 内存优化配置
use css_in_rust::runtime::StyleManagerConfig;

let config = StyleManagerConfig {
    cache_size: 500,  // 减少缓存大小
    enable_deduplication: true,  // 启用去重
    lazy_loading: true,  // 启用懒加载
    auto_cleanup: true,  // 自动清理
    ..Default::default()
};
```

### 调试工具

#### 1. 样式检查器

```rust
use css_in_rust::debug::StyleInspector;

let inspector = StyleInspector::new();

// 检查特定元素的样式
let element_styles = inspector.inspect_element("button");
println!("元素样式: {:#?}", element_styles);

// 检查样式冲突
let conflicts = inspector.find_conflicts();
for conflict in conflicts {
    println!("样式冲突: {} vs {}", conflict.rule1, conflict.rule2);
}
```

#### 2. 性能分析器

```rust
use css_in_rust::debug::PerformanceAnalyzer;

let analyzer = PerformanceAnalyzer::new();

// 分析编译性能
let compile_report = analyzer.analyze_compilation();
println!("编译瓶颈: {:#?}", compile_report.bottlenecks);

// 分析运行时性能
let runtime_report = analyzer.analyze_runtime();
println!("运行时指标: {:#?}", runtime_report.metrics);
```

#### 3. 依赖分析器

```rust
use css_in_rust::debug::DependencyAnalyzer;

let analyzer = DependencyAnalyzer::new();

// 分析样式依赖关系
let dependencies = analyzer.analyze_dependencies();
for dep in dependencies {
    println!("{} 依赖于 {:?}", dep.style, dep.dependencies);
}

// 查找循环依赖
let cycles = analyzer.find_circular_dependencies();
if !cycles.is_empty() {
    println!("发现循环依赖: {:#?}", cycles);
}
```

## 📈 迁移指南

### 从其他 CSS-in-JS 方案迁移

#### 从 styled-components 迁移

**之前 (styled-components)**:
```javascript
const Button = styled.button`
  background-color: #007bff;
  color: white;
  padding: 8px 16px;
  border: none;
  border-radius: 4px;

  &:hover {
    background-color: #0056b3;
  }
`;
```

**之后 (CSS-in-Rust)**:
```rust
use css_in_rust::css;

fn button_component() -> Html {
    let style = css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;

        &:hover {
            background-color: #0056b3;
        }
    };

    html! {
        <button class={style.class_name()}>
            {"Click me"}
        </button>
    }
}
```

#### 从 Emotion 迁移

**之前 (Emotion)**:
```javascript
const cardStyle = css`
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
`;
```

**之后 (CSS-in-Rust)**:
```rust
use css_in_rust::css;

let card_style = css! {
    background: white;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
};
```

### 从传统 CSS 迁移

#### 1. 全局样式转换

**之前 (global.css)**:
```css
.button {
  background-color: #007bff;
  color: white;
  padding: 8px 16px;
}

.button:hover {
  background-color: #0056b3;
}

.button.primary {
  background-color: #007bff;
}

.button.secondary {
  background-color: #6c757d;
}
```

**之后 (CSS-in-Rust)**:
```rust
use css_in_rust::{css, css_variants};

let button_variants = css_variants! {
    base: {
        color: white;
        padding: 8px 16px;

        &:hover {
            opacity: 0.9;
        }
    },

    variants: {
        variant: {
            primary: {
                background-color: #007bff;

                &:hover {
                    background-color: #0056b3;
                }
            },
            secondary: {
                background-color: #6c757d;

                &:hover {
                    background-color: #545b62;
                }
            }
        }
    }
};
```

#### 2. 媒体查询转换

**之前 (CSS)**:
```css
.grid {
  display: grid;
  gap: 16px;
  grid-template-columns: 1fr;
}

@media (min-width: 768px) {
  .grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (min-width: 1024px) {
  .grid {
    grid-template-columns: repeat(3, 1fr);
  }
}
```

**之后 (CSS-in-Rust)**:
```rust
use css_in_rust::css;

let grid_style = css! {
    display: grid;
    gap: 16px;
    grid-template-columns: 1fr;

    @media (min-width: 768px) {
        grid-template-columns: repeat(2, 1fr);
    }

    @media (min-width: 1024px) {
        grid-template-columns: repeat(3, 1fr);
    }
};
```

## 🤝 贡献指南

我们欢迎社区贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/your-org/css-in-rust.git
cd css-in-rust

# 安装依赖
cargo build

# 运行测试
cargo test

# 运行示例
cargo run --example basic

# 启动开发服务器
cargo run --bin dev-server
```

### 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
feat: 添加新功能
fix: 修复 bug
docs: 更新文档
style: 代码格式化
refactor: 重构代码
test: 添加测试
chore: 构建工具或辅助工具的变动
```

## 📄 许可证

MIT License - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

感谢以下项目和社区的启发：

- [LightningCSS](https://github.com/parcel-bundler/lightningcss) - 高性能 CSS 解析和转换
- [styled-components](https://styled-components.com/) - CSS-in-JS 的先驱
- [Tailwind CSS](https://tailwindcss.com/) - 实用优先的 CSS 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Yew](https://yew.rs/) - Rust 前端框架

## 📞 支持

- 📖 [文档](https://css-in-rust.dev)
- 💬 [Discord 社区](https://discord.gg/css-in-rust)
- 🐛 [问题反馈](https://github.com/your-org/css-in-rust/issues)
- 💡 [功能请求](https://github.com/your-org/css-in-rust/discussions)

---

**CSS-in-Rust** - 让 Rust 中的样式管理变得简单、安全、高效！ 🎨✨
