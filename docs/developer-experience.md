# 开发体验指南

本指南将帮助您充分利用 CSS-in-Rust 的开发工具和功能，提升开发效率和体验。

## 🛠️ 开发工具概览

CSS-in-Rust 提供了完整的开发工具链：

- **智能诊断**: 实时错误检测和修复建议
- **语法高亮**: 支持多种主题的 CSS 语法高亮
- **自动补全**: 智能的 CSS 属性和值补全
- **热更新**: 快速的样式热重载
- **性能分析**: 实时性能监控和优化建议
- **调试工具**: 强大的样式调试功能

## 🎨 语法高亮

### 1. 配置语法高亮

```rust
use css_in_rust::dev_experience::{SyntaxHighlighter, HighlightTheme};

// 创建语法高亮器
let highlighter = SyntaxHighlighter::new()
    .with_theme(HighlightTheme::Dark)  // 暗色主题
    .with_line_numbers(true)
    .with_syntax_validation(true);

// 高亮 CSS 代码
let css_code = r#"
.button {
    background-color: #007bff;
    color: white;
    padding: 8px 16px;
    border-radius: 4px;
    transition: all 0.2s ease;
}

.button:hover {
    background-color: #0056b3;
    transform: translateY(-1px);
}
"#;

let highlighted = highlighter.highlight(css_code)?;
println!("{}", highlighted.html);
```

### 2. 自定义主题

```rust
use css_in_rust::dev_experience::{HighlightTheme, TextStyle};
use std::collections::HashMap;

// 创建自定义主题
let mut custom_theme = HashMap::new();
custom_theme.insert("property".to_string(), TextStyle {
    color: "#ff6b6b".to_string(),
    bold: true,
    italic: false,
    underline: false,
});
custom_theme.insert("value".to_string(), TextStyle {
    color: "#4ecdc4".to_string(),
    bold: false,
    italic: false,
    underline: false,
});
custom_theme.insert("selector".to_string(), TextStyle {
    color: "#45b7d1".to_string(),
    bold: true,
    italic: false,
    underline: false,
});

let theme = HighlightTheme::Custom(custom_theme);
let highlighter = SyntaxHighlighter::new().with_theme(theme);
```

### 3. VS Code 集成

```json
// .vscode/settings.json
{
    "css-in-rust.enableSyntaxHighlighting": true,
    "css-in-rust.theme": "dark",
    "css-in-rust.showLineNumbers": true,
    "css-in-rust.enableValidation": true,
    "css-in-rust.highlightInMacros": true
}
```

## 🔍 智能诊断

### 1. 实时错误检测

```rust
use css_in_rust::dev_experience::{DiagnosticManager, DiagnosticLevel};

// 创建诊断管理器
let diagnostic_manager = DiagnosticManager::new()
    .with_real_time_checking(true)
    .with_auto_fix_suggestions(true)
    .with_performance_warnings(true);

// 检查 CSS 代码
let css_code = r#"
.button {
    colr: red;  // 拼写错误
    margin: 10px 10px 10px 10px;  // 可以简化
    display: block;
    display: flex;  // 重复属性
}
"#;

let diagnostics = diagnostic_manager.check_css(css_code)?;

for diagnostic in &diagnostics {
    match diagnostic.level {
        DiagnosticLevel::Error => {
            println!("❌ 错误 (行 {}): {}", diagnostic.line, diagnostic.message);
            if let Some(fix) = &diagnostic.suggested_fix {
                println!("   💡 建议: {}", fix);
            }
        }
        DiagnosticLevel::Warning => {
            println!("⚠️ 警告 (行 {}): {}", diagnostic.line, diagnostic.message);
        }
        DiagnosticLevel::Info => {
            println!("ℹ️ 信息 (行 {}): {}", diagnostic.line, diagnostic.message);
        }
    }
}

// 自动修复
if let Some(fixed_css) = diagnostic_manager.auto_fix(css_code)? {
    println!("🔧 自动修复后的代码:");
    println!("{}", fixed_css);
}
```

### 2. 性能诊断

```rust
use css_in_rust::dev_experience::PerformanceDiagnostic;

// 性能诊断
let perf_diagnostic = PerformanceDiagnostic::new()
    .with_complexity_analysis(true)
    .with_optimization_suggestions(true)
    .with_bundle_size_analysis(true);

let css_code = r#"
.complex-selector div > span:nth-child(odd) + p::before {
    background: linear-gradient(45deg, red, blue, green, yellow, purple);
    box-shadow: 0 0 10px rgba(0,0,0,0.5), 0 0 20px rgba(0,0,0,0.3), 0 0 30px rgba(0,0,0,0.1);
    filter: blur(2px) brightness(1.2) contrast(1.1) saturate(1.3);
}
"#;

let analysis = perf_diagnostic.analyze(css_code)?;

println!("📊 性能分析:");
println!("  复杂度评分: {}/10", analysis.complexity_score);
println!("  渲染成本: {}", analysis.render_cost);
println!("  包大小影响: {} bytes", analysis.bundle_size_impact);

for suggestion in &analysis.optimization_suggestions {
    println!("  💡 优化建议: {}", suggestion);
}
```

### 3. 可访问性检查

```rust
use css_in_rust::dev_experience::AccessibilityChecker;

// 可访问性检查
let a11y_checker = AccessibilityChecker::new()
    .with_color_contrast_check(true)
    .with_focus_indicators(true)
    .with_screen_reader_support(true);

let css_code = r#"
.button {
    background-color: #ffff00;  // 黄色背景
    color: #ffffff;             // 白色文字 - 对比度不足
    border: none;
    outline: none;              // 移除了焦点指示器
}
"#;

let a11y_issues = a11y_checker.check(css_code)?;

for issue in &a11y_issues {
    println!("♿ 可访问性问题: {}", issue.description);
    println!("   严重程度: {:?}", issue.severity);
    if let Some(fix) = &issue.suggested_fix {
        println!("   建议修复: {}", fix);
    }
}
```

## 🚀 自动补全

### 1. CSS 属性补全

```rust
use css_in_rust::dev_experience::{AutoCompleter, CompletionContext};

// 创建自动补全器
let completer = AutoCompleter::new()
    .with_css_properties(true)
    .with_css_values(true)
    .with_custom_properties(true)
    .with_framework_classes(true);

// 获取补全建议
let context = CompletionContext {
    text: "background-c".to_string(),
    cursor_position: 12,
    file_path: "src/styles.rs".to_string(),
};

let completions = completer.get_completions(&context)?;

for completion in &completions {
    println!("📝 {}: {}", completion.label, completion.description);
    if let Some(snippet) = &completion.snippet {
        println!("   代码片段: {}", snippet);
    }
}

// 示例输出:
// 📝 background-color: 设置元素的背景颜色
//    代码片段: background-color: ${1:#ffffff};
// 📝 background-clip: 设置背景的绘制区域
//    代码片段: background-clip: ${1|border-box,padding-box,content-box|};
```

### 2. 智能值补全

```rust
// 颜色值补全
let color_context = CompletionContext {
    text: "color: #".to_string(),
    cursor_position: 8,
    file_path: "src/styles.rs".to_string(),
};

let color_completions = completer.get_color_completions(&color_context)?;

for completion in &color_completions {
    println!("🎨 {}: {}", completion.value, completion.preview);
}

// 示例输出:
// 🎨 #ff0000: ████ 红色
// 🎨 #00ff00: ████ 绿色
// 🎨 #0000ff: ████ 蓝色

// 单位补全
let unit_context = CompletionContext {
    text: "width: 100".to_string(),
    cursor_position: 11,
    file_path: "src/styles.rs".to_string(),
};

let unit_completions = completer.get_unit_completions(&unit_context)?;

for completion in &unit_completions {
    println!("📏 {}: {}", completion.unit, completion.description);
}

// 示例输出:
// 📏 px: 像素单位
// 📏 rem: 相对于根元素字体大小
// 📏 em: 相对于当前元素字体大小
// 📏 %: 百分比
// 📏 vw: 视口宽度的百分比
// 📏 vh: 视口高度的百分比
```

### 3. 框架特定补全

```rust
// Tailwind CSS 类名补全
let tailwind_completer = AutoCompleter::new()
    .with_tailwind_classes(true)
    .with_custom_config("tailwind.config.js");

let tw_context = CompletionContext {
    text: "flex-".to_string(),
    cursor_position: 5,
    file_path: "src/components.rs".to_string(),
};

let tw_completions = tailwind_completer.get_completions(&tw_context)?;

// CSS-in-Rust 宏补全
let macro_context = CompletionContext {
    text: "css_var".to_string(),
    cursor_position: 7,
    file_path: "src/styles.rs".to_string(),
};

let macro_completions = completer.get_macro_completions(&macro_context)?;

for completion in &macro_completions {
    println!("🔧 {}: {}", completion.label, completion.description);
    println!("   示例: {}", completion.example);
}
```

## 🔥 热更新开发

### 1. 开发服务器配置

```rust
// src/bin/dev-server.rs
use css_in_rust::hot_reload::{HotReloadManager, HotReloadConfig};
use css_in_rust::dev_experience::DevServer;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置热更新
    let hot_reload_config = HotReloadConfig {
        watch_paths: vec![
            "src/**/*.rs".to_string(),
            "styles/**/*.css".to_string(),
            "assets/**/*".to_string(),
        ],
        ignore_patterns: vec![
            "target/**".to_string(),
            "**/.git/**".to_string(),
            "**/node_modules/**".to_string(),
        ],
        debounce_ms: 50,
        enable_css_injection: true,
        enable_page_reload: false,  // 只注入 CSS，不刷新页面
        websocket_port: 3001,
    };

    // 创建热更新管理器
    let hot_reload_manager = HotReloadManager::new(hot_reload_config).await?;

    // 配置开发服务器
    let dev_server = DevServer::new()
        .with_port(3000)
        .with_hot_reload(hot_reload_manager)
        .with_live_reload(true)
        .with_error_overlay(true)
        .with_performance_overlay(true);

    println!("🚀 开发服务器启动在 http://localhost:3000");
    println!("🔥 热更新服务在 ws://localhost:3001");

    // 启动服务器
    dev_server.start().await?;

    Ok(())
}
```

### 2. 客户端热更新脚本

```javascript
// public/hot-reload.js
class CssInRustHotReload {
    constructor(options = {}) {
        this.wsUrl = options.wsUrl || 'ws://localhost:3001';
        this.enableErrorOverlay = options.enableErrorOverlay !== false;
        this.enablePerformanceOverlay = options.enablePerformanceOverlay || false;
        this.reconnectInterval = options.reconnectInterval || 3000;

        this.ws = null;
        this.reconnectTimer = null;
        this.injectedStyles = new Map();

        this.init();
    }

    init() {
        this.connect();
        this.setupErrorOverlay();
        this.setupPerformanceOverlay();
    }

    connect() {
        try {
            this.ws = new WebSocket(this.wsUrl);

            this.ws.onopen = () => {
                console.log('🔥 CSS-in-Rust 热更新已连接');
                this.clearReconnectTimer();
                this.hideConnectionError();
            };

            this.ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            };

            this.ws.onclose = () => {
                console.log('🔌 热更新连接已断开，尝试重连...');
                this.scheduleReconnect();
            };

            this.ws.onerror = (error) => {
                console.error('❌ 热更新连接错误:', error);
                this.showConnectionError();
            };
        } catch (error) {
            console.error('❌ 无法连接热更新服务:', error);
            this.scheduleReconnect();
        }
    }

    handleMessage(message) {
        switch (message.type) {
            case 'css_update':
                this.updateCSS(message.data);
                break;
            case 'full_reload':
                this.reloadPage();
                break;
            case 'build_error':
                this.showBuildError(message.data);
                break;
            case 'build_success':
                this.hideBuildError();
                break;
            case 'performance_update':
                this.updatePerformanceOverlay(message.data);
                break;
        }
    }

    updateCSS(data) {
        const { css_id, css_content, selector } = data;

        // 查找现有样式
        let styleElement = document.getElementById(`css-in-rust-${css_id}`);

        if (!styleElement) {
            // 创建新的样式元素
            styleElement = document.createElement('style');
            styleElement.id = `css-in-rust-${css_id}`;
            styleElement.type = 'text/css';
            document.head.appendChild(styleElement);
        }

        // 更新样式内容
        styleElement.textContent = css_content;

        // 添加更新动画
        this.animateUpdate(selector);

        console.log(`🎨 已更新样式: ${css_id}`);
    }

    animateUpdate(selector) {
        if (!selector) return;

        const elements = document.querySelectorAll(selector);
        elements.forEach(el => {
            el.style.transition = 'all 0.3s ease';
            el.style.outline = '2px solid #00ff00';

            setTimeout(() => {
                el.style.outline = '';
            }, 300);
        });
    }

    showBuildError(error) {
        if (!this.enableErrorOverlay) return;

        let overlay = document.getElementById('css-in-rust-error-overlay');

        if (!overlay) {
            overlay = document.createElement('div');
            overlay.id = 'css-in-rust-error-overlay';
            overlay.style.cssText = `
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.9);
                color: white;
                font-family: 'Monaco', 'Menlo', monospace;
                font-size: 14px;
                padding: 20px;
                z-index: 999999;
                overflow: auto;
            `;
            document.body.appendChild(overlay);
        }

        overlay.innerHTML = `
            <div style="max-width: 800px; margin: 0 auto;">
                <h2 style="color: #ff6b6b; margin-bottom: 20px;">🚨 构建错误</h2>
                <div style="background: #2d2d2d; padding: 15px; border-radius: 5px; margin-bottom: 20px;">
                    <pre style="margin: 0; white-space: pre-wrap;">${this.escapeHtml(error.message)}</pre>
                </div>
                <div style="color: #888;">
                    文件: ${error.file}<br>
                    行号: ${error.line}<br>
                    时间: ${new Date().toLocaleTimeString()}
                </div>
                <button onclick="this.parentElement.parentElement.style.display='none'"
                        style="margin-top: 20px; padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;">
                    关闭
                </button>
            </div>
        `;
    }

    hideBuildError() {
        const overlay = document.getElementById('css-in-rust-error-overlay');
        if (overlay) {
            overlay.style.display = 'none';
        }
    }

    setupPerformanceOverlay() {
        if (!this.enablePerformanceOverlay) return;

        const overlay = document.createElement('div');
        overlay.id = 'css-in-rust-perf-overlay';
        overlay.style.cssText = `
            position: fixed;
            top: 10px;
            right: 10px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            padding: 10px;
            border-radius: 5px;
            font-family: monospace;
            font-size: 12px;
            z-index: 999998;
            min-width: 200px;
        `;

        overlay.innerHTML = `
            <div><strong>CSS-in-Rust 性能</strong></div>
            <div id="perf-styles-count">样式数量: -</div>
            <div id="perf-cache-hit-rate">缓存命中率: -</div>
            <div id="perf-memory-usage">内存使用: -</div>
            <div id="perf-last-update">最后更新: -</div>
        `;

        document.body.appendChild(overlay);
    }

    updatePerformanceOverlay(data) {
        if (!this.enablePerformanceOverlay) return;

        document.getElementById('perf-styles-count').textContent =
            `样式数量: ${data.styles_count}`;
        document.getElementById('perf-cache-hit-rate').textContent =
            `缓存命中率: ${(data.cache_hit_rate * 100).toFixed(1)}%`;
        document.getElementById('perf-memory-usage').textContent =
            `内存使用: ${(data.memory_usage / 1024 / 1024).toFixed(1)}MB`;
        document.getElementById('perf-last-update').textContent =
            `最后更新: ${new Date().toLocaleTimeString()}`;
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    scheduleReconnect() {
        this.clearReconnectTimer();
        this.reconnectTimer = setTimeout(() => {
            this.connect();
        }, this.reconnectInterval);
    }

    clearReconnectTimer() {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
    }

    showConnectionError() {
        // 显示连接错误提示
    }

    hideConnectionError() {
        // 隐藏连接错误提示
    }

    reloadPage() {
        window.location.reload();
    }
}

// 自动初始化
if (typeof window !== 'undefined') {
    window.cssInRustHotReload = new CssInRustHotReload({
        enableErrorOverlay: true,
        enablePerformanceOverlay: process.env.NODE_ENV === 'development'
    });
}
```

### 3. 热更新配置

```toml
# css-in-rust.toml
[hot_reload]
enable = true
port = 3001
debounce_ms = 50

# 监控路径
watch_paths = [
    "src/**/*.rs",
    "styles/**/*.css",
    "assets/**/*",
    "components/**/*.rs"
]

# 忽略路径
ignore_patterns = [
    "target/**",
    "**/.git/**",
    "**/node_modules/**",
    "**/.DS_Store",
    "**/*.tmp"
]

# 热更新选项
enable_css_injection = true
enable_page_reload = false
enable_error_overlay = true
enable_performance_overlay = true

# WebSocket 配置
websocket_host = "localhost"
websocket_port = 3001
max_connections = 100
heartbeat_interval = 30

[development]
# 开发模式特定配置
fast_build = true
skip_optimization = true
enable_source_maps = true
verbose_logging = true
```

## 🔧 调试工具

### 1. 样式检查器

```rust
use css_in_rust::dev_experience::StyleInspector;

// 创建样式检查器
let inspector = StyleInspector::new()
    .with_dom_integration(true)
    .with_computed_styles(true)
    .with_inheritance_tracking(true);

// 检查元素样式
let element_id = "my-button";
let inspection = inspector.inspect_element(element_id)?;

println!("🔍 元素样式检查: {}", element_id);
println!("  应用的类: {:?}", inspection.applied_classes);
println!("  计算样式: {:?}", inspection.computed_styles);
println!("  继承样式: {:?}", inspection.inherited_styles);
println!("  覆盖样式: {:?}", inspection.overridden_styles);

// 样式来源追踪
for source in &inspection.style_sources {
    println!("  📍 样式来源: {} (行 {})", source.file, source.line);
    println!("     优先级: {}", source.specificity);
    println!("     规则: {}", source.rule);
}

// 性能影响分析
if let Some(perf_impact) = &inspection.performance_impact {
    println!("  ⚡ 性能影响:");
    println!("     渲染复杂度: {}", perf_impact.render_complexity);
    println!("     重排风险: {}", perf_impact.reflow_risk);
    println!("     重绘风险: {}", perf_impact.repaint_risk);
}
```

### 2. CSS 依赖分析

```rust
use css_in_rust::dev_experience::DependencyAnalyzer;

// 分析 CSS 依赖关系
let analyzer = DependencyAnalyzer::new()
    .with_project_root("./")
    .with_include_external(true)
    .with_circular_detection(true);

let analysis = analyzer.analyze()?;

println!("📊 CSS 依赖分析:");
println!("  总文件数: {}", analysis.total_files);
println!("  总依赖数: {}", analysis.total_dependencies);
println!("  最大深度: {}", analysis.max_depth);

// 循环依赖检测
if !analysis.circular_dependencies.is_empty() {
    println!("  ⚠️ 发现循环依赖:");
    for cycle in &analysis.circular_dependencies {
        println!("     {}", cycle.join(" -> "));
    }
}

// 未使用的样式
if !analysis.unused_styles.is_empty() {
    println!("  🗑️ 未使用的样式:");
    for unused in &analysis.unused_styles {
        println!("     {} ({})", unused.selector, unused.file);
    }
}

// 依赖图可视化
let graph_svg = analyzer.generate_dependency_graph()?;
std::fs::write("dependency-graph.svg", graph_svg)?;
println!("  📈 依赖图已保存到 dependency-graph.svg");
```

### 3. 实时性能监控

```rust
use css_in_rust::dev_experience::PerformanceMonitor;
use std::time::Duration;

// 创建性能监控器
let monitor = PerformanceMonitor::new()
    .with_real_time_tracking(true)
    .with_memory_profiling(true)
    .with_render_timing(true)
    .with_alert_thresholds({
        let mut thresholds = std::collections::HashMap::new();
        thresholds.insert("compilation_time".to_string(), Duration::from_millis(100));
        thresholds.insert("memory_usage".to_string(), 50.0); // MB
        thresholds.insert("cache_hit_rate".to_string(), 0.8);
        thresholds
    });

// 启动监控
monitor.start_monitoring();

// 设置警报回调
monitor.on_alert(|alert| {
    match alert.metric.as_str() {
        "compilation_time" => {
            println!("⚠️ 编译时间过长: {:?}", alert.value);
            // 自动优化建议
            println!("💡 建议启用增量编译或检查复杂样式");
        }
        "memory_usage" => {
            println!("⚠️ 内存使用过高: {:.1} MB", alert.value);
            // 自动清理
            StyleManager::global().cleanup_cache();
        }
        "cache_hit_rate" => {
            println!("⚠️ 缓存命中率过低: {:.1}%", alert.value * 100.0);
            // 缓存预热
            StyleManager::global().warmup_cache();
        }
        _ => {}
    }
});

// 获取实时统计
let stats = monitor.get_current_stats();
println!("📊 实时性能统计:");
println!("  编译次数: {}", stats.compilation_count);
println!("  平均编译时间: {:?}", stats.avg_compilation_time);
println!("  内存使用: {:.1} MB", stats.memory_usage / 1024.0 / 1024.0);
println!("  缓存命中率: {:.1}%", stats.cache_hit_rate * 100.0);
println!("  活跃样式数: {}", stats.active_styles_count);
```

## 🎯 开发工作流优化

### 1. 自动化工作流

```rust
// src/bin/dev-workflow.rs
use css_in_rust::dev_experience::WorkflowManager;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workflow = WorkflowManager::new()
        .with_auto_format(true)
        .with_auto_lint(true)
        .with_auto_optimize(true)
        .with_auto_test(true);

    // 监听文件变化
    workflow.on_file_change(|event| async move {
        match event.file_type {
            FileType::Rust => {
                // Rust 文件变化时
                println!("🦀 检测到 Rust 文件变化: {}", event.path);

                // 自动格式化
                if let Err(e) = format_rust_file(&event.path).await {
                    eprintln!("❌ 格式化失败: {}", e);
                }

                // 自动检查 CSS 宏
                if let Err(e) = check_css_macros(&event.path).await {
                    eprintln!("❌ CSS 宏检查失败: {}", e);
                }
            }
            FileType::Css => {
                // CSS 文件变化时
                println!("🎨 检测到 CSS 文件变化: {}", event.path);

                // 自动优化
                if let Err(e) = optimize_css_file(&event.path).await {
                    eprintln!("❌ CSS 优化失败: {}", e);
                }
            }
            _ => {}
        }
    });

    // 启动工作流
    workflow.start().await?;

    Ok(())
}

async fn format_rust_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 rustfmt 格式化
    let output = tokio::process::Command::new("rustfmt")
        .arg(path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(format!("rustfmt 失败: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    println!("✅ 已格式化: {}", path);
    Ok(())
}

async fn check_css_macros(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 检查 CSS 宏语法
    let content = tokio::fs::read_to_string(path).await?;
    let checker = CssMacroChecker::new();

    let issues = checker.check(&content)?;

    if !issues.is_empty() {
        println!("⚠️ 发现 CSS 宏问题:");
        for issue in issues {
            println!("   行 {}: {}", issue.line, issue.message);
        }
    }

    Ok(())
}

async fn optimize_css_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 优化 CSS 文件
    let content = tokio::fs::read_to_string(path).await?;
    let optimizer = CssOptimizer::new();

    let optimized = optimizer.optimize(&content)?;

    if optimized != content {
        tokio::fs::write(path, optimized).await?;
        println!("✅ 已优化: {}", path);
    }

    Ok(())
}
```

### 2. 测试集成

```rust
// tests/dev_experience_tests.rs
use css_in_rust::dev_experience::*;
use css_in_rust::testing::*;

#[tokio::test]
async fn test_hot_reload_functionality() {
    let test_env = TestEnvironment::new().await;

    // 启动热更新服务
    let hot_reload = HotReloadManager::new(HotReloadConfig::default()).await.unwrap();

    // 模拟文件变化
    let css_change = FileChange {
        path: "test.css".to_string(),
        change_type: ChangeType::Modified,
        content: "body { color: red; }".to_string(),
    };

    // 发送变化事件
    hot_reload.handle_change(css_change).await.unwrap();

    // 验证客户端收到更新
    let client_message = test_env.wait_for_websocket_message().await;
    assert_eq!(client_message.message_type, "css_update");

    test_env.cleanup().await;
}

#[test]
fn test_syntax_highlighting() {
    let highlighter = SyntaxHighlighter::new()
        .with_theme(HighlightTheme::Dark);

    let css = ".button { color: red; }";
    let result = highlighter.highlight(css).unwrap();

    assert!(result.html.contains("<span class=\"selector\">"));
    assert!(result.html.contains("<span class=\"property\">"));
    assert!(result.html.contains("<span class=\"value\">"));
}

#[test]
fn test_diagnostic_system() {
    let diagnostic_manager = DiagnosticManager::new();

    let css_with_errors = ".button { colr: red; }";
    let diagnostics = diagnostic_manager.check_css(css_with_errors).unwrap();

    assert!(!diagnostics.is_empty());
    assert_eq!(diagnostics[0].level, DiagnosticLevel::Error);
    assert!(diagnostics[0].message.contains("colr"));
}

#[test]
fn test_auto_completion() {
    let completer = AutoCompleter::new();

    let context = CompletionContext {
        text: "background-c".to_string(),
        cursor_position: 12,
        file_path: "test.rs".to_string(),
    };

    let completions = completer.get_completions(&context).unwrap();

    assert!(!completions.is_empty());
    assert!(completions.iter().any(|c| c.label.contains("background-color")));
}
```

### 3. CI/CD 集成

```yaml
# .github/workflows/css-in-rust-dev.yml
name: CSS-in-Rust Development

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  dev-experience-tests:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: 安装 Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy

    - name: 缓存依赖
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: 检查代码格式
      run: cargo fmt -- --check

    - name: 运行 Clippy
      run: cargo clippy -- -D warnings

    - name: 运行开发体验测试
      run: cargo test dev_experience

    - name: 测试热更新功能
      run: cargo test hot_reload

    - name: 测试语法高亮
      run: cargo test syntax_highlighting

    - name: 测试诊断系统
      run: cargo test diagnostic

    - name: 性能基准测试
      run: cargo bench --features dev-tools

    - name: 生成开发工具文档
      run: cargo doc --features dev-tools --no-deps

    - name: 上传测试报告
      uses: actions/upload-artifact@v3
      if: always()
      with:
        name: test-results
        path: |
          target/criterion
          target/doc
```

## 📚 最佳实践

### ✅ 开发环境配置
- [ ] 启用热更新和实时预览
- [ ] 配置语法高亮和自动补全
- [ ] 设置错误覆盖层和性能监控
- [ ] 使用开发服务器和调试工具

### ✅ 代码质量
- [ ] 启用实时诊断和错误检测
- [ ] 使用自动格式化和代码检查
- [ ] 配置可访问性检查
- [ ] 定期运行性能分析

### ✅ 工作流优化
- [ ] 自动化常见开发任务
- [ ] 集成测试和 CI/CD
- [ ] 使用依赖分析和优化建议
- [ ] 配置团队开发规范

通过这些开发工具和最佳实践，您可以显著提升 CSS-in-Rust 的开发体验和效率！🚀
