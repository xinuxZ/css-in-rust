//! 热更新演示
//!
//! 本模块演示 CSS-in-Rust 的热更新功能，包括：
//! - 实时样式注入
//! - 智能文件监控
//! - 增量更新
//! - 样式缓存管理
//! - 开发服务器集成

use css_in_rust::css;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde_json as _;
use serde as _;
use tokio as _;

/// 热更新演示主函数
pub fn run_hot_reload_demo() {
    println!("🔥 热更新系统演示");
    println!("==================");
    println!();

    // 演示基础热更新
    demo_basic_hot_reload();

    // 演示文件监控
    demo_file_watching();

    // 演示增量更新
    demo_incremental_updates();

    // 演示样式缓存
    demo_style_caching();

    // 演示开发服务器集成
    demo_dev_server_integration();

    // 演示性能监控
    demo_performance_monitoring();

    println!("✅ 热更新系统演示完成！");
    println!();
}

fn main() {
    run_hot_reload_demo();
}

/// 测试热更新配置
async fn test_hot_reload_config() {
    println!("\n--- 测试热更新配置 ---");

    // 创建默认配置
    let default_config = HotReloadConfig::default();
    println!("默认配置:");
    println!("  启用状态: {}", default_config.enabled);
    println!("  监控目录: {:?}", default_config.watch_dirs);
    println!("  文件扩展名: {:?}", default_config.file_extensions);
    println!("  忽略模式: {:?}", default_config.ignore_patterns);
    println!("  防抖延迟: {:?}", default_config.debounce_delay);
    println!("  WebSocket端口: {}", default_config.websocket_port);
    println!("  自动刷新: {}", default_config.auto_refresh);
    println!("  CSS注入: {}", default_config.css_injection);

    // 创建自定义配置
    let custom_config = HotReloadConfig {
        enabled: true,
        watch_dirs: vec![
            PathBuf::from("src"),
            PathBuf::from("styles"),
            PathBuf::from("components"),
        ],
        file_extensions: {
            let mut exts = HashSet::new();
            exts.insert("rs".to_string());
            exts.insert("css".to_string());
            exts.insert("scss".to_string());
            exts.insert("html".to_string());
            exts
        },
        ignore_patterns: vec![
            "target/**".to_string(),
            "node_modules/**".to_string(),
            "*.tmp".to_string(),
            ".git/**".to_string(),
        ],
        debounce_delay: Duration::from_millis(300),
        websocket_port: 3001,
        auto_refresh: true,
        css_injection: true,
    };

    println!("\n自定义配置:");
    println!("  监控目录数量: {}", custom_config.watch_dirs.len());
    println!("  支持的文件类型: {:?}", custom_config.file_extensions);
    println!("  忽略模式数量: {}", custom_config.ignore_patterns.len());
    println!("  防抖延迟: {}ms", custom_config.debounce_delay.as_millis());

    // 验证配置
    let is_valid = custom_config.validate();
    println!("  配置验证结果: {}", if is_valid { "有效" } else { "无效" });
}

/// 测试文件监控
async fn test_file_watcher() {
    println!("\n--- 测试文件监控 ---");

    // 创建文件监控器
    let config = HotReloadConfig::default();
    let mut watcher = FileWatcher::new(config.clone());

    println!("创建文件监控器成功");
    println!("监控目录: {:?}", config.watch_dirs);

    // 模拟启动监控
    match watcher.start().await {
        Ok(_) => {
            println!("文件监控启动成功");

            // 模拟文件变化事件
            let test_events = vec![
                "src/main.rs 被修改",
                "src/components/button.rs 被创建",
                "styles/theme.css 被删除",
                "target/debug/build.rs 被修改 (应被忽略)",
            ];

            for event in test_events {
                println!("检测到文件变化: {}", event);
                sleep(Duration::from_millis(100)).await;
            }

            // 停止监控
            watcher.stop().await;
            println!("文件监控已停止");
        }
        Err(e) => {
            println!("文件监控启动失败: {:?}", e);
        }
    }
}

/// 测试变化检测
async fn test_change_detector() {
    println!("\n--- 测试变化检测 ---");

    // 创建变化检测器
    let mut detector = ChangeDetector::new();

    // 模拟CSS变化
    let old_css = css!("color: red; font-size: 14px;");
    let new_css = css!("color: blue; font-size: 16px; font-weight: bold;");

    println!("旧CSS类名: {}", old_css);
    println!("新CSS类名: {}", new_css);

    // 检测变化
    let changes = detector.detect_css_changes(&old_css, &new_css);
    println!("\n检测到的CSS变化:");
    for change in changes {
        println!("  - {}", change);
    }

    // 模拟文件内容变化
    let old_content = r#"
        fn render_button() {
            let style = css!("background: red; padding: 8px;");
            format!("<button class=\"{}\">Click me</button>", style)
        }
    "#;

    let new_content = r#"
        fn render_button() {
            let style = css!("background: blue; padding: 12px; border-radius: 4px;");
            format!("<button class=\"{}\">Click me</button>", style)
        }
    "#;

    let file_changes = detector.detect_file_changes(old_content, new_content);
    println!("\n检测到的文件变化:");
    for change in file_changes {
        println!("  - {}", change);
    }

    // 分析变化影响
    let impact = detector.analyze_change_impact(&changes);
    println!("\n变化影响分析:");
    println!("  需要重新编译: {}", impact.needs_recompile);
    println!("  需要刷新页面: {}", impact.needs_page_refresh);
    println!("  可以热注入: {}", impact.can_hot_inject);
    println!("  影响的组件: {:?}", impact.affected_components);
}

/// 测试WebSocket服务器
async fn test_websocket_server() {
    println!("\n--- 测试WebSocket服务器 ---");

    // 创建WebSocket服务器
    let config = HotReloadConfig::default();
    let mut server = WebSocketServer::new(config.websocket_port);

    println!("创建WebSocket服务器，端口: {}", config.websocket_port);

    // 模拟启动服务器
    match server.start().await {
        Ok(_) => {
            println!("WebSocket服务器启动成功");
            println!("等待客户端连接...");

            // 模拟客户端连接
            sleep(Duration::from_millis(500)).await;
            println!("模拟客户端连接成功");

            // 模拟发送热更新消息
            let messages = vec![
                r#"{"type": "css_update", "data": {"selector": ".button", "styles": "color: blue;"}}",
                r#"{"type": "page_refresh", "data": {}}",
                r#"{"type": "component_update", "data": {"component": "Button", "html": "<button>New</button>"}}",
            ];

            for message in messages {
                server.broadcast(message).await;
                println!("发送消息: {}", message);
                sleep(Duration::from_millis(200)).await;
            }

            // 停止服务器
            server.stop().await;
            println!("WebSocket服务器已停止");
        }
        Err(e) => {
            println!("WebSocket服务器启动失败: {:?}", e);
        }
    }
}

/// 测试CSS注入
async fn test_css_injector() {
    println!("\n--- 测试CSS注入 ---");

    // 创建CSS注入器
    let injector = CssInjector::new();

    // 生成一些测试样式
    let button_style = css!("background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px;");
    let card_style = css!("background: white; border: 1px solid #dee2e6; border-radius: 8px; padding: 16px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);");
    let input_style = css!("border: 1px solid #ced4da; border-radius: 4px; padding: 8px 12px; font-size: 14px; width: 100%;");

    println!("生成的样式类名:");
    println!("  按钮样式: {}", button_style);
    println!("  卡片样式: {}", card_style);
    println!("  输入框样式: {}", input_style);

    // 模拟CSS注入过程
    let injection_script = injector.generate_injection_script(&[
        (&button_style, "background: #28a745; color: white; padding: 8px 16px; border: none; border-radius: 4px;"),
        (&card_style, "background: #f8f9fa; border: 1px solid #dee2e6; border-radius: 8px; padding: 20px; box-shadow: 0 4px 8px rgba(0,0,0,0.15);"),
    ]);

    println!("\n生成的注入脚本:");
    println!("{}", injection_script);

    // 模拟注入过程
    println!("\n模拟CSS注入过程:");
    println!("1. 检测到样式变化");
    println!("2. 生成新的CSS规则");
    println!("3. 通过WebSocket发送到浏览器");
    println!("4. 浏览器执行注入脚本");
    println!("5. 页面样式实时更新");

    // 生成注入后的HTML示例
    let updated_html = format!(
        r#"<div class="hot-reload-demo">
  <button class="{}">更新后的按钮</button>
  <div class="{}">更新后的卡片</div>
  <input class="{}" placeholder="更新后的输入框" />
</div>"#,
        button_style, card_style, input_style
    );

    println!("\n注入后的HTML示例:\n{}", updated_html);
}

/// 测试完整的热更新流程
async fn test_complete_hot_reload() {
    println!("\n--- 测试完整热更新流程 ---");

    // 创建热更新管理器
    let config = HotReloadConfig::default();
    let mut manager = HotReloadManager::new(config);

    println!("创建热更新管理器成功");

    // 启动热更新服务
    match manager.start().await {
        Ok(_) => {
            println!("热更新服务启动成功");

            // 模拟完整的开发流程
            println!("\n模拟开发流程:");

            // 1. 初始样式
            let initial_style = css!("color: black; font-size: 14px;");
            println!("1. 初始样式: {}", initial_style);

            sleep(Duration::from_millis(1000)).await;

            // 2. 修改样式
            println!("2. 开发者修改CSS...");
            let updated_style = css!("color: blue; font-size: 16px; font-weight: bold;");

            sleep(Duration::from_millis(500)).await;

            // 3. 检测变化
            println!("3. 检测到文件变化");

            sleep(Duration::from_millis(300)).await;

            // 4. 重新编译
            println!("4. 重新编译CSS");
            println!("   新样式: {}", updated_style);

            sleep(Duration::from_millis(800)).await;

            // 5. 推送更新
            println!("5. 通过WebSocket推送更新到浏览器");

            sleep(Duration::from_millis(200)).await;

            // 6. 浏览器更新
            println!("6. 浏览器接收更新并应用新样式");

            sleep(Duration::from_millis(500)).await;

            // 7. 完成
            println!("7. 热更新完成，页面样式已更新");

            // 停止服务
            manager.stop().await;
            println!("\n热更新服务已停止");
        }
        Err(e) => {
            println!("热更新服务启动失败: {:?}", e);
        }
    }
}

/// 测试开发体验功能
async fn test_dev_experience() {
    println!("\n--- 测试开发体验功能 ---");

    // 测试错误处理
    println!("\n错误处理演示:");

    // 模拟CSS语法错误
    let invalid_css = "color: ; font-size: invalid;";
    println!("无效CSS: {}", invalid_css);

    // 模拟错误检测和报告
    let errors = vec![
        "第1行: 'color' 属性值不能为空",
        "第1行: 'invalid' 不是有效的字体大小值",
    ];

    println!("检测到的错误:");
    for error in errors {
        println!("  ❌ {}", error);
    }

    // 测试性能监控
    println!("\n性能监控演示:");

    let performance_metrics = vec![
        ("文件监控延迟", "15ms"),
        ("CSS编译时间", "120ms"),
        ("WebSocket传输", "8ms"),
        ("浏览器注入时间", "25ms"),
        ("总热更新时间", "168ms"),
    ];

    for (metric, value) in performance_metrics {
        println!("  📊 {}: {}", metric, value);
    }

    // 测试开发提示
    println!("\n开发提示演示:");

    let dev_tips = vec![
        "💡 建议使用CSS变量来提高主题切换性能",
        "⚡ 检测到频繁的样式变化，考虑使用防抖优化",
        "🎨 推荐使用语义化的类名以提高可维护性",
        "🔧 当前项目已启用CSS压缩，生产构建将自动优化",
    ];

    for tip in dev_tips {
        println!("  {}", tip);
    }

    // 测试调试信息
    println!("\n调试信息演示:");

    let debug_info = vec![
        ("活跃的WebSocket连接", "2"),
        ("监控的文件数量", "156"),
        ("缓存的CSS规则", "89"),
        ("今日热更新次数", "47"),
        ("平均更新延迟", "142ms"),
    ];

    for (info, value) in debug_info {
        println!("  🔍 {}: {}", info, value);
    }

    println!("\n开发体验功能测试完成");
}
