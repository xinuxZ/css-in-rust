# 热更新支持指南

本指南详细介绍如何配置和使用 CSS-in-Rust 的热更新功能，实现快速的开发迭代。

## 🔥 热更新概览

CSS-in-Rust 的热更新系统提供：

- **实时样式注入**: 无需刷新页面即可看到样式变化
- **智能文件监控**: 监控 Rust 文件中的 CSS 宏变化
- **增量更新**: 只更新变化的样式，保持应用状态
- **错误处理**: 优雅处理编译错误，提供详细反馈
- **性能监控**: 实时显示热更新性能指标
- **多客户端支持**: 支持多个浏览器窗口同步更新

## 🚀 快速开始

### 1. 基础配置

```rust
// Cargo.toml
[dependencies]
css-in-rust = { version = "0.1.0", features = ["hot-reload"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
css-in-rust = { version = "0.1.0", features = ["build-tools"] }
```

```rust
// src/main.rs
use css_in_rust::hot_reload::{HotReloadManager, HotReloadConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 只在开发模式下启用热更新
    #[cfg(debug_assertions)]
    {
        let config = HotReloadConfig::default()
            .with_port(3001)
            .with_watch_paths(vec![
                "src/**/*.rs".to_string(),
                "components/**/*.rs".to_string(),
            ]);

        let hot_reload = HotReloadManager::new(config).await?;
        hot_reload.start().await?;

        println!("🔥 热更新服务已启动在 ws://localhost:3001");
    }

    // 启动你的应用
    start_app().await?;

    Ok(())
}

async fn start_app() -> Result<(), Box<dyn std::error::Error>> {
    // 你的应用启动逻辑
    Ok(())
}
```

### 2. 客户端集成

```html
<!-- public/index.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>CSS-in-Rust App</title>
</head>
<body>
    <div id="app"></div>

    <!-- 只在开发模式下加载热更新脚本 -->
    <script>
        if (location.hostname === 'localhost' || location.hostname === '127.0.0.1') {
            const script = document.createElement('script');
            script.src = '/hot-reload.js';
            document.head.appendChild(script);
        }
    </script>
</body>
</html>
```

```javascript
// public/hot-reload.js
class CssInRustHotReload {
    constructor() {
        this.wsUrl = 'ws://localhost:3001';
        this.ws = null;
        this.reconnectDelay = 1000;
        this.maxReconnectDelay = 30000;
        this.reconnectAttempts = 0;

        this.connect();
    }

    connect() {
        try {
            this.ws = new WebSocket(this.wsUrl);

            this.ws.onopen = () => {
                console.log('🔥 CSS-in-Rust 热更新已连接');
                this.reconnectAttempts = 0;
                this.reconnectDelay = 1000;
            };

            this.ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            };

            this.ws.onclose = () => {
                console.log('🔌 热更新连接断开，尝试重连...');
                this.scheduleReconnect();
            };

            this.ws.onerror = (error) => {
                console.error('❌ 热更新连接错误:', error);
            };
        } catch (error) {
            console.error('❌ 无法连接热更新服务:', error);
            this.scheduleReconnect();
        }
    }

    handleMessage(message) {
        switch (message.type) {
            case 'css_hot_reload':
                this.updateCSS(message.data);
                break;
            case 'full_reload':
                window.location.reload();
                break;
            case 'build_error':
                this.showError(message.data);
                break;
            case 'build_success':
                this.hideError();
                break;
        }
    }

    updateCSS(data) {
        const { css_id, css_content, class_name } = data;

        // 查找或创建样式元素
        let styleEl = document.getElementById(`css-in-rust-${css_id}`);
        if (!styleEl) {
            styleEl = document.createElement('style');
            styleEl.id = `css-in-rust-${css_id}`;
            styleEl.type = 'text/css';
            document.head.appendChild(styleEl);
        }

        // 更新样式内容
        styleEl.textContent = css_content;

        console.log(`🎨 已更新样式: ${class_name}`);

        // 可选：添加视觉反馈
        this.flashUpdatedElements(class_name);
    }

    flashUpdatedElements(className) {
        const elements = document.querySelectorAll(`.${className}`);
        elements.forEach(el => {
            el.style.outline = '2px solid #00ff00';
            setTimeout(() => {
                el.style.outline = '';
            }, 500);
        });
    }

    showError(error) {
        // 显示编译错误覆盖层
        let overlay = document.getElementById('css-in-rust-error');
        if (!overlay) {
            overlay = document.createElement('div');
            overlay.id = 'css-in-rust-error';
            overlay.style.cssText = `
                position: fixed; top: 0; left: 0; right: 0; bottom: 0;
                background: rgba(0,0,0,0.9); color: white; z-index: 999999;
                font-family: monospace; padding: 20px; overflow: auto;
            `;
            document.body.appendChild(overlay);
        }

        overlay.innerHTML = `
            <h2 style="color: #ff6b6b;">🚨 CSS 编译错误</h2>
            <pre style="background: #2d2d2d; padding: 15px; border-radius: 5px;">${error.message}</pre>
            <p>文件: ${error.file} (行 ${error.line})</p>
        `;
    }

    hideError() {
        const overlay = document.getElementById('css-in-rust-error');
        if (overlay) {
            overlay.remove();
        }
    }

    scheduleReconnect() {
        setTimeout(() => {
            this.reconnectAttempts++;
            this.reconnectDelay = Math.min(
                this.reconnectDelay * 1.5,
                this.maxReconnectDelay
            );
            this.connect();
        }, this.reconnectDelay);
    }
}

// 自动启动
new CssInRustHotReload();
```

## ⚙️ 高级配置

### 1. 详细配置选项

```rust
use css_in_rust::hot_reload::*;
use std::time::Duration;

// 创建详细的热更新配置
let config = HotReloadConfig {
    // WebSocket 服务器配置
    websocket_host: "localhost".to_string(),
    websocket_port: 3001,
    max_connections: 100,
    heartbeat_interval: Duration::from_secs(30),

    // 文件监控配置
    watch_paths: vec![
        "src/**/*.rs".to_string(),
        "components/**/*.rs".to_string(),
        "styles/**/*.css".to_string(),
        "assets/**/*".to_string(),
    ],

    ignore_patterns: vec![
        "target/**".to_string(),
        "**/.git/**".to_string(),
        "**/node_modules/**".to_string(),
        "**/*.tmp".to_string(),
        "**/.DS_Store".to_string(),
    ],

    // 变更检测配置
    debounce_ms: 50,  // 防抖延迟
    batch_changes: true,  // 批量处理变更
    enable_incremental: true,  // 启用增量更新

    // 热更新行为
    enable_css_injection: true,  // CSS 注入
    enable_page_reload: false,   // 禁用页面刷新
    enable_error_overlay: true,  // 错误覆盖层
    enable_success_notifications: true,  // 成功通知

    // 性能配置
    max_file_size: 10 * 1024 * 1024,  // 10MB 文件大小限制
    compilation_timeout: Duration::from_secs(30),  // 编译超时
    enable_compression: true,  // 启用消息压缩

    // 调试配置
    verbose_logging: true,  // 详细日志
    log_file_changes: true,  // 记录文件变更
    log_compilation_stats: true,  // 记录编译统计
};

let hot_reload_manager = HotReloadManager::new(config).await?;
```

### 2. 自定义文件监控

```rust
use css_in_rust::hot_reload::{FileWatcher, WatchEvent, WatchEventType};
use std::path::PathBuf;

// 创建自定义文件监控器
let mut file_watcher = FileWatcher::new(FileWatcherConfig {
    watch_paths: vec![PathBuf::from("src")],
    ignore_patterns: vec![
        "**/*.tmp".to_string(),
        "**/target/**".to_string(),
    ],
    recursive: true,
    follow_symlinks: false,
    debounce_ms: 100,
})?;

// 设置事件处理器
file_watcher.on_event(|event: WatchEvent| {
    match event.event_type {
        WatchEventType::Created => {
            println!("📁 文件创建: {:?}", event.path);
        }
        WatchEventType::Modified => {
            println!("✏️ 文件修改: {:?}", event.path);

            // 检查是否是 Rust 文件
            if event.path.extension().map_or(false, |ext| ext == "rs") {
                // 分析 CSS 宏变更
                if let Ok(changes) = analyze_css_macro_changes(&event.path) {
                    for change in changes {
                        println!("🎨 CSS 宏变更: {} (行 {})", change.css_id, change.line);
                    }
                }
            }
        }
        WatchEventType::Deleted => {
            println!("🗑️ 文件删除: {:?}", event.path);
        }
        WatchEventType::Renamed { from, to } => {
            println!("📝 文件重命名: {:?} -> {:?}", from, to);
        }
    }
});

// 启动监控
file_watcher.start()?;

fn analyze_css_macro_changes(file_path: &PathBuf) -> Result<Vec<CssMacroChange>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    let analyzer = CssMacroAnalyzer::new();

    let current_macros = analyzer.extract_css_macros(&content)?;
    let previous_macros = get_cached_macros(file_path)?;

    let changes = analyzer.diff_macros(&previous_macros, &current_macros)?;

    // 缓存当前状态
    cache_macros(file_path, &current_macros)?;

    Ok(changes)
}
```

### 3. 智能变更检测

```rust
use css_in_rust::hot_reload::{ChangeDetector, ChangeType, FileChange};

// 创建智能变更检测器
let change_detector = ChangeDetector::new(ChangeDetectorConfig {
    project_root: PathBuf::from("./"),
    enable_dependency_analysis: true,
    enable_css_extraction: true,
    enable_caching: true,
    cache_dir: PathBuf::from(".cache/css-in-rust"),
})?;

// 分析文件变更
let file_change = FileChange {
    path: PathBuf::from("src/components/button.rs"),
    change_type: ChangeType::Modified,
    timestamp: std::time::SystemTime::now(),
    content: Some(std::fs::read_to_string("src/components/button.rs")?),
};

let analysis = change_detector.analyze_change(&file_change)?;

println!("🔍 变更分析结果:");
println!("  变更类型: {:?}", analysis.change_type);
println!("  影响范围: {:?}", analysis.impact_scope);
println!("  需要重新编译: {}", analysis.requires_recompilation);
println!("  CSS 变更数量: {}", analysis.css_changes.len());

// 处理 CSS 变更
for css_change in &analysis.css_changes {
    println!("  🎨 CSS 变更: {}", css_change.css_id);
    println!("     选择器: {:?}", css_change.selectors);
    println!("     变更类型: {:?}", css_change.change_type);

    // 如果是样式内容变更，触发热更新
    if css_change.change_type == CssChangeType::StyleContent {
        hot_reload_manager.trigger_css_update(css_change).await?;
    }
}

// 处理依赖变更
if !analysis.affected_dependencies.is_empty() {
    println!("  📦 影响的依赖:");
    for dep in &analysis.affected_dependencies {
        println!("     {}", dep);
    }

    // 可能需要完整重新编译
    if analysis.requires_full_rebuild {
        hot_reload_manager.trigger_full_rebuild().await?;
    }
}
```

### 4. 重新加载管理

```rust
use css_in_rust::hot_reload::{ReloadManager, ReloadConfig, BuildType};

// 配置重新加载管理器
let reload_config = ReloadConfig {
    build_command: "cargo".to_string(),
    build_args: vec!["build".to_string(), "--features".to_string(), "hot-reload".to_string()],
    build_timeout: Duration::from_secs(60),

    // 并行配置
    enable_parallel_builds: true,
    max_parallel_jobs: num_cpus::get(),

    // 增量配置
    enable_incremental: true,
    incremental_cache_dir: PathBuf::from(".cache/incremental"),

    // 重试配置
    max_retries: 3,
    retry_delay: Duration::from_secs(1),

    // 优化配置
    enable_fast_build: true,  // 开发模式快速构建
    skip_tests: true,         // 跳过测试
    skip_docs: true,          // 跳过文档生成
};

let reload_manager = ReloadManager::new(reload_config)?;

// 设置事件处理器
reload_manager.on_build_start(|build_type| {
    println!("🔨 开始构建: {:?}", build_type);
});

reload_manager.on_build_progress(|progress| {
    println!("📊 构建进度: {:.1}%", progress.percentage);
});

reload_manager.on_build_complete(|result| {
    match result {
        Ok(build_result) => {
            println!("✅ 构建成功 (耗时: {:?})", build_result.duration);
            println!("   编译文件数: {}", build_result.compiled_files);
            println!("   生成的 CSS: {} bytes", build_result.generated_css_size);
        }
        Err(error) => {
            println!("❌ 构建失败: {}", error);
        }
    }
});

// 触发不同类型的构建
reload_manager.trigger_build(BuildType::Incremental).await?;
reload_manager.trigger_build(BuildType::HotReload).await?;
reload_manager.trigger_build(BuildType::Full).await?;
```

## 🌐 WebSocket 服务器

### 1. 服务器配置

```rust
use css_in_rust::hot_reload::{WebSocketServer, WebSocketConfig, WebSocketMessage};
use tokio_tungstenite::tungstenite::Message;

// 创建 WebSocket 服务器
let ws_config = WebSocketConfig {
    host: "localhost".to_string(),
    port: 3001,
    max_connections: 100,
    heartbeat_interval: Duration::from_secs(30),
    message_buffer_size: 1024,
    enable_compression: true,
    enable_binary_messages: false,
};

let ws_server = WebSocketServer::new(ws_config)?;

// 设置消息处理器
ws_server.on_client_connected(|client_id| {
    println!("🔌 客户端连接: {}", client_id);
});

ws_server.on_client_disconnected(|client_id| {
    println!("🔌 客户端断开: {}", client_id);
});

ws_server.on_message_received(|client_id, message| {
    match message {
        WebSocketMessage::Ping => {
            // 响应心跳
            ws_server.send_to_client(client_id, WebSocketMessage::Pong)?;
        }
        WebSocketMessage::ClientInfo { user_agent, url } => {
            println!("📱 客户端信息: {} - {}", user_agent, url);
        }
        _ => {}
    }
});

// 启动服务器
ws_server.start().await?;

// 广播消息到所有客户端
ws_server.broadcast(WebSocketMessage::CssHotReload {
    css_id: "button-primary".to_string(),
    css_content: ".button-primary { background: blue; }".to_string(),
    class_name: "button-primary".to_string(),
    timestamp: std::time::SystemTime::now(),
}).await?;
```

### 2. 消息类型定义

```rust
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    // 热更新消息
    CssHotReload {
        css_id: String,
        css_content: String,
        class_name: String,
        timestamp: SystemTime,
    },

    // 完整重新加载
    FullReload {
        reason: String,
        timestamp: SystemTime,
    },

    // 构建状态
    BuildStart {
        build_type: String,
        timestamp: SystemTime,
    },

    BuildProgress {
        percentage: f32,
        message: String,
        timestamp: SystemTime,
    },

    BuildSuccess {
        duration_ms: u64,
        files_compiled: usize,
        css_size: usize,
        timestamp: SystemTime,
    },

    BuildError {
        error_message: String,
        file_path: Option<String>,
        line_number: Option<u32>,
        timestamp: SystemTime,
    },

    // 性能监控
    PerformanceUpdate {
        memory_usage: u64,
        compilation_time: u64,
        cache_hit_rate: f32,
        active_styles: usize,
        timestamp: SystemTime,
    },

    // 客户端消息
    ClientInfo {
        user_agent: String,
        url: String,
    },

    // 心跳消息
    Ping,
    Pong,
}

// 消息序列化示例
let message = WebSocketMessage::CssHotReload {
    css_id: "my-component".to_string(),
    css_content: ".my-component { color: red; }".to_string(),
    class_name: "my-component-abc123".to_string(),
    timestamp: SystemTime::now(),
};

let json = serde_json::to_string(&message)?;
println!("📤 发送消息: {}", json);
```

### 3. 客户端状态管理

```rust
use css_in_rust::hot_reload::{ClientConnection, ClientState};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// 客户端连接管理
#[derive(Debug)]
pub struct ClientManager {
    connections: Arc<RwLock<HashMap<String, ClientConnection>>>,
    next_client_id: Arc<RwLock<u64>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(1)),
        }
    }

    pub fn add_client(&self, connection: ClientConnection) -> String {
        let mut next_id = self.next_client_id.write().unwrap();
        let client_id = format!("client_{}", *next_id);
        *next_id += 1;

        let mut connections = self.connections.write().unwrap();
        connections.insert(client_id.clone(), connection);

        println!("👥 客户端已连接: {} (总数: {})", client_id, connections.len());

        client_id
    }

    pub fn remove_client(&self, client_id: &str) {
        let mut connections = self.connections.write().unwrap();
        if connections.remove(client_id).is_some() {
            println!("👥 客户端已断开: {} (总数: {})", client_id, connections.len());
        }
    }

    pub fn broadcast_message(&self, message: &WebSocketMessage) -> Result<(), Box<dyn std::error::Error>> {
        let connections = self.connections.read().unwrap();
        let json = serde_json::to_string(message)?;

        for (client_id, connection) in connections.iter() {
            if let Err(e) = connection.send_message(&json) {
                eprintln!("❌ 发送消息到客户端 {} 失败: {}", client_id, e);
            }
        }

        Ok(())
    }

    pub fn send_to_client(&self, client_id: &str, message: &WebSocketMessage) -> Result<(), Box<dyn std::error::Error>> {
        let connections = self.connections.read().unwrap();

        if let Some(connection) = connections.get(client_id) {
            let json = serde_json::to_string(message)?;
            connection.send_message(&json)?;
        }

        Ok(())
    }

    pub fn get_client_count(&self) -> usize {
        self.connections.read().unwrap().len()
    }

    pub fn get_client_info(&self, client_id: &str) -> Option<ClientConnection> {
        let connections = self.connections.read().unwrap();
        connections.get(client_id).cloned()
    }

    pub fn cleanup_inactive_clients(&self) {
        let mut connections = self.connections.write().unwrap();
        let now = SystemTime::now();

        connections.retain(|client_id, connection| {
            if let Ok(duration) = now.duration_since(connection.last_activity) {
                if duration > Duration::from_secs(300) { // 5分钟超时
                    println!("🧹 清理非活跃客户端: {}", client_id);
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
    }
}
```

## 🔧 框架集成

### 1. Yew 集成

```rust
// src/lib.rs (Yew 应用)
use yew::prelude::*;
use css_in_rust::{css, hot_reload::HotReloadProvider};

#[function_component(App)]
fn app() -> Html {
    // 在开发模式下启用热更新
    #[cfg(debug_assertions)]
    let hot_reload = use_state(|| {
        HotReloadProvider::new("ws://localhost:3001")
    });

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
    };

    html! {
        <div>
            <h1>{ "CSS-in-Rust + Yew" }</h1>
            <button class={button_style.class_name()}>
                { "点击我" }
            </button>

            // 开发模式下显示热更新状态
            #[cfg(debug_assertions)]
            <HotReloadStatus provider={(*hot_reload).clone()} />
        </div>
    }
}

#[cfg(debug_assertions)]
#[function_component(HotReloadStatus)]
fn hot_reload_status(props: &HotReloadStatusProps) -> Html {
    let status = use_state(|| "连接中...".to_string());

    // 监听热更新状态
    use_effect_with_deps(
        move |provider| {
            let status = status.clone();

            provider.on_status_change(move |new_status| {
                status.set(match new_status {
                    HotReloadStatus::Connected => "🔥 已连接".to_string(),
                    HotReloadStatus::Disconnected => "🔌 已断开".to_string(),
                    HotReloadStatus::Reconnecting => "🔄 重连中...".to_string(),
                    HotReloadStatus::Error(e) => format!("❌ 错误: {}", e),
                });
            });

            || {}
        },
        props.provider.clone(),
    );

    html! {
        <div style="position: fixed; bottom: 10px; right: 10px; background: rgba(0,0,0,0.8); color: white; padding: 5px 10px; border-radius: 3px; font-size: 12px;">
            { &*status }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct HotReloadStatusProps {
    provider: HotReloadProvider,
}
```

### 2. Leptos 集成

```rust
// src/app.rs (Leptos 应用)
use leptos::*;
use css_in_rust::{css, hot_reload::use_hot_reload};

#[component]
fn App() -> impl IntoView {
    // 在开发模式下启用热更新
    #[cfg(debug_assertions)]
    use_hot_reload("ws://localhost:3001");

    let (count, set_count) = create_signal(0);

    let button_style = css! {
        background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
        color: white;
        border: none;
        padding: 12px 24px;
        border-radius: 8px;
        cursor: pointer;
        font-size: 16px;
        transition: transform 0.2s ease;

        &:hover {
            transform: translateY(-2px);
        }

        &:active {
            transform: translateY(0);
        }
    };

    view! {
        <div>
            <h1>"CSS-in-Rust + Leptos"</h1>
            <p>"计数: " {count}</p>
            <button
                class={button_style.class_name()}
                on:click=move |_| set_count.update(|n| *n += 1)
            >
                "增加计数"
            </button>
        </div>
    }
}

// 热更新 Hook
#[cfg(debug_assertions)]
fn use_hot_reload(ws_url: &str) {
    use leptos::*;
    use wasm_bindgen::prelude::*;
    use web_sys::*;

    create_effect(move |_| {
        let window = web_sys::window().unwrap();
        let ws = WebSocket::new(ws_url).unwrap();

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let message: String = text.into();

                // 处理热更新消息
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&message) {
                    if parsed["type"] == "css_hot_reload" {
                        update_css_style(
                            &parsed["data"]["css_id"].as_str().unwrap(),
                            &parsed["data"]["css_content"].as_str().unwrap(),
                        );
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    });
}

#[cfg(debug_assertions)]
fn update_css_style(css_id: &str, css_content: &str) {
    use web_sys::*;

    let document = web_sys::window().unwrap().document().unwrap();
    let style_id = format!("css-in-rust-{}", css_id);

    let style_element = if let Some(existing) = document.get_element_by_id(&style_id) {
        existing
    } else {
        let new_style = document.create_element("style").unwrap();
        new_style.set_id(&style_id);
        document.head().unwrap().append_child(&new_style).unwrap();
        new_style
    };

    style_element.set_text_content(Some(css_content));
}
```

### 3. Dioxus 集成

```rust
// src/main.rs (Dioxus 应用)
use dioxus::prelude::*;
use css_in_rust::{css, hot_reload::DioxusHotReload};

fn main() {
    // 启动 Dioxus 应用
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // 在开发模式下启用热更新
    #[cfg(debug_assertions)]
    use_hot_reload(cx, "ws://localhost:3001");

    let count = use_state(cx, || 0);

    let container_style = css! {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    };

    let button_style = css! {
        background: #007bff;
        color: white;
        border: none;
        padding: 10px 20px;
        border-radius: 5px;
        cursor: pointer;
        font-size: 14px;

        &:hover {
            background: #0056b3;
        }
    };

    render! {
        div {
            class: "{container_style.class_name()}",
            h1 { "CSS-in-Rust + Dioxus" }
            p { "计数: {count}" }
            button {
                class: "{button_style.class_name()}",
                onclick: move |_| count.modify(|c| c + 1),
                "增加计数"
            }
        }
    }
}

#[cfg(debug_assertions)]
fn use_hot_reload(cx: Scope, ws_url: &str) {
    use_future(cx, (), |_| {
        let ws_url = ws_url.to_string();
        async move {
            let hot_reload = DioxusHotReload::new(&ws_url).await?;
            hot_reload.start().await?;
            Ok::<(), Box<dyn std::error::Error>>(())
        }
    });
}
```

## 📊 性能监控

### 1. 热更新性能统计

```rust
use css_in_rust::hot_reload::{HotReloadStats, PerformanceCollector};
use std::time::{Duration, Instant};

// 创建性能收集器
let perf_collector = PerformanceCollector::new()
    .with_detailed_timing(true)
    .with_memory_tracking(true)
    .with_network_monitoring(true);

// 监控热更新性能
let hot_reload_manager = HotReloadManager::new(config)
    .await?
    .with_performance_collector(perf_collector);

// 定期获取统计信息
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        let stats = hot_reload_manager.get_stats().await;

        println!("📊 热更新性能统计:");
        println!("  文件变更检测: {} 次", stats.file_changes_detected);
        println!("  CSS 更新: {} 次", stats.css_updates_sent);
        println!("  平均更新延迟: {:?}", stats.avg_update_latency);
        println!("  WebSocket 连接: {} 个", stats.active_connections);
        println!("  消息发送: {} 条", stats.messages_sent);
        println!("  内存使用: {:.1} MB", stats.memory_usage as f64 / 1024.0 / 1024.0);

        // 性能警告
        if stats.avg_update_latency > Duration::from_millis(500) {
            println!("⚠️ 更新延迟过高，考虑优化文件监控配置");
        }

        if stats.memory_usage > 100 * 1024 * 1024 { // 100MB
            println!("⚠️ 内存使用过高，考虑清理缓存");
        }
    }
});
```

### 2. 网络性能优化

```rust
use css_in_rust::hot_reload::{MessageCompressor, BatchProcessor};

// 消息压缩
let compressor = MessageCompressor::new()
    .with_compression_level(6)  // 平衡压缩率和速度
    .with_min_size_threshold(1024);  // 只压缩大于 1KB 的消息

// 批量处理
let batch_processor = BatchProcessor::new()
    .with_batch_size(10)  // 最多批量处理 10 个变更
    .with_batch_timeout(Duration::from_millis(100))  // 100ms 超时
    .with_priority_handling(true);  // 优先处理重要变更

// 应用到热更新管理器
let hot_reload_manager = HotReloadManager::new(config)
    .await?
    .with_message_compressor(compressor)
    .with_batch_processor(batch_processor);

// 监控网络性能
let network_stats = hot_reload_manager.get_network_stats().await;
println!("🌐 网络性能:");
println!("  消息压缩率: {:.1}%", network_stats.compression_ratio * 100.0);
println!("  平均消息大小: {} bytes", network_stats.avg_message_size);
println!("  网络吞吐量: {:.1} KB/s", network_stats.throughput_kbps);
println!("  连接延迟: {:?}", network_stats.avg_latency);
```

## 🛠️ 故障排除

### 1. 常见问题诊断

```rust
use css_in_rust::hot_reload::DiagnosticTool;

// 创建诊断工具
let diagnostic = DiagnosticTool::new();

// 检查热更新配置
let config_check = diagnostic.check_configuration(&hot_reload_config)?;
if !config_check.is_valid {
    println!("❌ 配置问题:");
    for issue in &config_check.issues {
        println!("   - {}", issue);
    }
}

// 检查文件监控
let watch_check = diagnostic.check_file_watching()?;
if !watch_check.is_working {
    println!("❌ 文件监控问题:");
    for issue in &watch_check.issues {
        println!("   - {}", issue);
    }
}

// 检查 WebSocket 连接
let ws_check = diagnostic.check_websocket_connection("ws://localhost:3001").await?;
if !ws_check.is_connected {
    println!("❌ WebSocket 连接问题:");
    println!("   错误: {}", ws_check.error.unwrap_or_default());
}

// 检查编译环境
let compile_check = diagnostic.check_compilation_environment()?;
if !compile_check.is_ready {
    println!("❌ 编译环境问题:");
    for issue in &compile_check.issues {
        println!("   - {}", issue);
    }
}

// 生成诊断报告
let report = diagnostic.generate_report().await?;
std::fs::write("hot-reload-diagnostic.json", serde_json::to_string_pretty(&report)?)?;
println!("📋 诊断报告已保存到 hot-reload-diagnostic.json");
```

### 2. 调试模式

```rust
// 启用详细调试日志
std::env::set_var("CSS_IN_RUST_LOG", "debug");
std::env::set_var("CSS_IN_RUST_HOT_RELOAD_DEBUG", "1");

// 创建调试版本的热更新管理器
let debug_config = HotReloadConfig::default()
    .with_debug_mode(true)
    .with_verbose_logging(true)
    .with_performance_profiling(true)
    .with_event_tracing(true);

let hot_reload_manager = HotReloadManager::new(debug_config).await?;

// 设置调试事件处理器
hot_reload_manager.on_debug_event(|event| {
    match event {
        DebugEvent::FileWatchEvent { path, event_type } => {
            println!("🔍 文件监控事件: {:?} - {:?}", path, event_type);
        }
        DebugEvent::CssExtractionStart { file_path } => {
            println!("🔍 开始提取 CSS: {:?}", file_path);
        }
        DebugEvent::CssExtractionComplete { file_path, css_count, duration } => {
            println!("🔍 CSS 提取完成: {:?} ({} 个样式, 耗时 {:?})", file_path, css_count, duration);
        }
        DebugEvent::WebSocketMessage { client_id, message_type, size } => {
            println!("🔍 WebSocket 消息: {} -> {} ({} bytes)", client_id, message_type, size);
        }
        DebugEvent::CompilationStart { trigger } => {
            println!("🔍 开始编译: {:?}", trigger);
        }
        DebugEvent::CompilationComplete { success, duration, output_size } => {
            println!("🔍 编译完成: {} (耗时 {:?}, 输出 {} bytes)",
                    if success { "成功" } else { "失败" }, duration, output_size);
        }
    }
});
```

## 📋 最佳实践

### ✅ 配置优化
- [ ] 合理设置防抖延迟（推荐 50-100ms）
- [ ] 配置适当的文件监控范围
- [ ] 启用增量编译和缓存
- [ ] 使用消息压缩减少网络开销

### ✅ 性能优化
- [ ] 监控热更新性能指标
- [ ] 定期清理非活跃连接
- [ ] 批量处理文件变更
- [ ] 优化 CSS 提取和分析逻辑

### ✅ 开发体验
- [ ] 提供清晰的错误信息和覆盖层
- [ ] 显示热更新状态和统计信息
- [ ] 支持多种框架和环境
- [ ] 提供调试工具和诊断功能

### ✅ 生产部署
- [ ] 在生产环境中禁用热更新
- [ ] 移除开发依赖和调试代码
- [ ] 使用环境变量控制功能开关
- [ ] 确保安全的 WebSocket 配置

通过遵循这些指南和最佳实践，您可以充分利用 CSS-in-Rust 的热更新功能，实现高效的开发体验！🔥
