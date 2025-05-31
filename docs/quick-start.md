# 快速开始指南

本指南将帮助您在几分钟内开始使用 CSS-in-Rust。

## 🚀 安装

### 1. 添加依赖

在您的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
css-in-rust = "0.1.0"
css-in-rust-macros = "0.1.0"

# 如果您需要构建时优化
[build-dependencies]
css-in-rust = { version = "0.1.0", features = ["build-tools"] }
```

### 2. 选择框架支持

根据您使用的前端框架，添加相应的特性：

```toml
# Yew 框架
css-in-rust = { version = "0.1.0", features = ["yew"] }

# Leptos 框架
css-in-rust = { version = "0.1.0", features = ["leptos"] }

# Dioxus 框架
css-in-rust = { version = "0.1.0", features = ["dioxus"] }
```

## 📝 第一个样式

### 基础用法

```rust
use css_in_rust::css;

fn main() {
    // 创建一个简单的按钮样式
    let button_style = css! {
        background-color: #007bff;
        color: white;
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        cursor: pointer;

        // 悬停效果
        &:hover {
            background-color: #0056b3;
        }
    };

    // 获取生成的 CSS 类名
    println!("CSS 类名: {}", button_style.class_name());

    // 获取生成的 CSS 内容
    println!("CSS 内容: {}", button_style.css_content());
}
```

### 在 Yew 中使用

```rust
use yew::prelude::*;
use css_in_rust::css;

#[function_component(App)]
fn app() -> Html {
    let button_style = css! {
        background-color: #007bff;
        color: white;
        padding: 12px 24px;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 16px;
        font-weight: 500;
        transition: all 0.2s ease;

        &:hover {
            background-color: #0056b3;
            transform: translateY(-1px);
        }

        &:active {
            transform: translateY(0);
        }
    };

    html! {
        <div>
            <h1>{"欢迎使用 CSS-in-Rust!"}</h1>
            <button class={button_style.class_name()}>
                {"点击我"}
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### 在 Leptos 中使用

```rust
use leptos::*;
use css_in_rust::css;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let button_style = css! {
        background-color: #28a745;
        color: white;
        padding: 10px 20px;
        border: none;
        border-radius: 5px;
        cursor: pointer;

        &:hover {
            background-color: #218838;
        }
    };

    view! { cx,
        <div>
            <h1>"CSS-in-Rust with Leptos"</h1>
            <button class=button_style.class_name()>
                "Success Button"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
```

## 🎨 条件样式

使用 `css_if!` 宏根据条件应用不同的样式：

```rust
use css_in_rust::css_if;

fn create_button(is_primary: bool, is_disabled: bool) -> String {
    css_if! {
        // 基础样式
        base: {
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-weight: 500;
            transition: all 0.2s ease;
        },

        // 主要按钮样式
        is_primary => {
            background-color: #007bff;
            color: white;

            &:hover {
                background-color: #0056b3;
            }
        },

        // 次要按钮样式
        !is_primary => {
            background-color: #f8f9fa;
            color: #212529;
            border: 1px solid #dee2e6;

            &:hover {
                background-color: #e9ecef;
            }
        },

        // 禁用状态
        is_disabled => {
            opacity: 0.6;
            cursor: not-allowed;

            &:hover {
                background-color: inherit;
            }
        }
    }.class_name()
}

// 使用示例
let primary_button = create_button(true, false);
let disabled_button = create_button(false, true);
```

## 🌈 主题系统

### 定义主题

```rust
use css_in_rust::theme;

// 浅色主题
let light_theme = theme! {
    primary: #007bff,
    secondary: #6c757d,
    success: #28a745,
    danger: #dc3545,
    warning: #ffc107,
    info: #17a2b8,

    background: #ffffff,
    surface: #f8f9fa,
    text: #212529,
    text_secondary: #6c757d,
    border: #dee2e6,

    shadow: "0 2px 4px rgba(0, 0, 0, 0.1)"
};

// 深色主题
let dark_theme = theme! {
    primary: #0d6efd,
    secondary: #6c757d,
    success: #198754,
    danger: #dc3545,
    warning: #fd7e14,
    info: #0dcaf0,

    background: #212529,
    surface: #343a40,
    text: #ffffff,
    text_secondary: #adb5bd,
    border: #495057,

    shadow: "0 2px 4px rgba(0, 0, 0, 0.3)"
};
```

### 使用主题变量

```rust
use css_in_rust::{css, Theme};

// 设置当前主题
Theme::set_current(light_theme);

// 使用主题变量创建样式
let card_style = css! {
    background-color: var(--surface);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px;
    box-shadow: var(--shadow);

    .title {
        color: var(--primary);
        font-size: 18px;
        font-weight: 600;
        margin-bottom: 8px;
    }

    .content {
        color: var(--text_secondary);
        line-height: 1.5;
    }
};

// 主题切换
fn toggle_theme() {
    let current = Theme::get_current();
    if current.name == "light" {
        Theme::set_current(dark_theme);
    } else {
        Theme::set_current(light_theme);
    }
}
```

## 📱 响应式设计

```rust
use css_in_rust::css;

let responsive_layout = css! {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 16px;

    // 平板及以上
    @media (min-width: 768px) {
        flex-direction: row;
        padding: 24px;
        gap: 24px;
    }

    // 桌面及以上
    @media (min-width: 1024px) {
        max-width: 1200px;
        margin: 0 auto;
        padding: 32px;
    }

    .sidebar {
        flex: 0 0 auto;

        @media (min-width: 768px) {
            flex: 0 0 250px;
        }
    }

    .main {
        flex: 1;
        min-width: 0; // 防止 flex 项目溢出
    }
};
```

## 🔥 热更新设置

### 1. 创建开发服务器

创建 `src/bin/dev-server.rs`：

```rust
use css_in_rust::hot_reload::{HotReloadManager, HotReloadConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = HotReloadConfig {
        watch_paths: vec![
            "src/".into(),
            "styles/".into(),
        ],
        ignore_patterns: vec![
            "target/**".to_string(),
            "**/.git/**".to_string(),
            "**/.DS_Store".to_string(),
        ],
        websocket_port: 3001,
        enable_css_hot_reload: true,
        enable_full_reload: false,
        debounce_ms: 100,
    };

    let mut hot_reload = HotReloadManager::new(config);

    println!("🔥 启动热更新服务器...");
    hot_reload.start().await?;

    println!("✅ 热更新服务器已启动在端口 3001");
    println!("📁 监听目录: src/, styles/");
    println!("🛑 按 Ctrl+C 停止服务器");

    // 等待中断信号
    tokio::signal::ctrl_c().await?;

    println!("🛑 正在停止热更新服务器...");
    hot_reload.stop().await;

    Ok(())
}
```

### 2. 添加客户端脚本

在您的 HTML 文件中添加：

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
        (function() {
            if (typeof window === 'undefined') return;

            const ws = new WebSocket('ws://localhost:3001');

            ws.onopen = function() {
                console.log('🔥 热更新已连接');
            };

            ws.onmessage = function(event) {
                const message = JSON.parse(event.data);

                switch (message.type) {
                    case 'CssHotReload':
                        updateCSS(message.data.css_content);
                        console.log('🎨 CSS 已热更新');
                        break;

                    case 'FullReload':
                        console.log('🔄 完全重新加载');
                        window.location.reload();
                        break;

                    case 'BuildStatus':
                        handleBuildStatus(message.data);
                        break;
                }
            };

            ws.onclose = function() {
                console.log('❌ 热更新连接已断开');
            };

            ws.onerror = function(error) {
                console.error('🚨 热更新错误:', error);
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

            function handleBuildStatus(data) {
                if (data.status === 'Error') {
                    console.error('🚨 构建错误:', data.message);
                } else {
                    console.log('✅ 构建成功:', data.message);
                }
            }
        })();
    </script>
</body>
</html>
```

### 3. 启动开发环境

```bash
# 终端 1: 启动热更新服务器
cargo run --bin dev-server

# 终端 2: 启动您的应用
cargo run --example your-app
# 或者使用 trunk (对于 Yew)
trunk serve
```

## 🛠️ 构建优化

### 创建 build.rs

```rust
// build.rs
use css_in_rust::build_tools::{CssBuildProcessor, BuildConfig};

fn main() {
    let config = BuildConfig {
        project_root: std::env::var("CARGO_MANIFEST_DIR").unwrap().into(),
        output_dir: "dist".into(),
        enable_dead_code_elimination: true,
        enable_minification: true,
        generate_report: true,
        usage_threshold: 0.0,
        aggressive_elimination: false,
    };

    let processor = CssBuildProcessor::new(config);

    match processor.run() {
        Ok(result) => {
            println!("✅ CSS 构建完成");
            println!("📊 处理了 {} 个文件", result.processed_files.len());
            println!("🗜️ 压缩率: {:.1}%", result.compression_ratio * 100.0);
        }
        Err(e) => {
            eprintln!("❌ CSS 构建失败: {}", e);
            std::process::exit(1);
        }
    }

    // 告诉 Cargo 何时重新运行构建脚本
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=styles/");
    println!("cargo:rerun-if-changed=css-in-rust.toml");
}
```

## 📋 项目配置

创建 `css-in-rust.toml` 配置文件：

```toml
[general]
name = "my-css-app"
output_dir = "dist"
development = true

[optimization]
dead_code_elimination = true
minification = true
source_maps = true
usage_threshold = 0.0

[hot_reload]
enable = true
port = 3001
watch_paths = ["src/", "styles/"]
ignore_patterns = ["target/**", "**/.git/**"]

[performance]
enable_caching = true
cache_size = 1000
incremental_compilation = true
parallel_processing = true

[diagnostics]
syntax_check = true
performance_hints = true
accessibility_check = true
strict_mode = false
```

## 🎯 下一步

现在您已经掌握了 CSS-in-Rust 的基础用法，可以探索更多高级功能：

1. **[变体系统](variants.md)** - 创建可复用的组件变体
2. **[动画系统](animations.md)** - 添加流畅的动画效果
3. **[性能优化](performance.md)** - 优化应用性能
4. **[主题定制](theming.md)** - 深入了解主题系统
5. **[框架集成](frameworks.md)** - 与不同框架的深度集成

## 🆘 需要帮助？

- 📖 查看[完整文档](README.md)
- 💬 加入 [Discord 社区](https://discord.gg/css-in-rust)
- 🐛 [报告问题](https://github.com/your-org/css-in-rust/issues)
- 💡 [功能建议](https://github.com/your-org/css-in-rust/discussions)

祝您使用愉快！🎉
