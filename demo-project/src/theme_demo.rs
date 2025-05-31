//! 主题系统演示
//!
//! 本模块演示 CSS-in-Rust 的主题系统功能，包括：
//! - 主题定义和切换
//! - 设计令牌管理
//! - 亮色/暗色主题
//! - 主题变量
//! - 动态主题切换

use css_in_rust::{
    css, CssVariableManager, Theme, ThemeContext, ThemeManager, ThemeMode, ThemeProvider,
};

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde as _;
use serde_json as _;
use tokio as _;

/// 主题演示主函数
fn main() {
    println!("🎨 主题系统演示");
    println!("=================");
    println!();

    // 演示主题定义
    demo_theme_definition();

    // 演示设计令牌
    demo_design_tokens();

    // 演示亮色/暗色主题
    demo_light_dark_themes();

    // 演示主题变量
    demo_theme_variables();

    // 演示动态主题切换
    demo_dynamic_theme_switching();

    println!("✅ 主题系统演示完成！");
    println!();
}

/// 演示主题定义
fn demo_theme_definition() {
    println!("📝 1. 主题定义");

    // 定义默认主题
    let default_theme = css! {
        :root {
            --color-primary: #007bff;
            --color-secondary: #6c757d;
            --color-success: #28a745;
            --color-danger: #dc3545;
            --color-warning: #ffc107;
            --color-info: #17a2b8;
            --color-light: #f8f9fa;
            --color-dark: #343a40;

            --spacing-xs: 4px;
            --spacing-sm: 8px;
            --spacing-md: 16px;
            --spacing-lg: 24px;
            --spacing-xl: 32px;

            --font-size-xs: 12px;
            --font-size-sm: 14px;
            --font-size-md: 16px;
            --font-size-lg: 18px;
            --font-size-xl: 24px;

            --border-radius-sm: 2px;
            --border-radius-md: 4px;
            --border-radius-lg: 8px;
            --border-radius-xl: 12px;
        }
    };

    println!("   ✅ 默认主题: {}", default_theme);

    // 定义企业主题
    let enterprise_theme = css! {
        :root[data-theme="enterprise"] {
            --color-primary: #2c3e50;
            --color-secondary: #95a5a6;
            --color-accent: #e74c3c;
            --color-background: #ecf0f1;
            --color-surface: #ffffff;
            --color-text: #2c3e50;
            --border-radius-sm: 2px;
            --border-radius-md: 4px;
            --border-radius-lg: 8px;
        }
    };

    println!("   ✅ 企业主题: {}", enterprise_theme);
    println!();
}

/// 演示设计令牌
fn demo_design_tokens() {
    println!("🎯 2. 设计令牌系统");

    // 使用设计令牌创建组件样式
    let button_style = css! {
        background-color: var(--color-primary);
        color: white;
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--border-radius-md);
        font-size: var(--font-size-sm);
        font-weight: 500;
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            filter: brightness(110%);
            transform: translateY(-1px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
        }

        &:active {
            transform: translateY(0);
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }

        &:disabled {
            background-color: var(--color-secondary);
            cursor: not-allowed;
            opacity: 0.6;
        }

        &.secondary {
            background-color: var(--color-secondary);
        }

        &.success {
            background-color: var(--color-success);
        }

        &.danger {
            background-color: var(--color-danger);
        }
    };

    println!("   ✅ 按钮样式（设计令牌）: {}", button_style);

    let card_style = css! {
        background-color: var(--color-light);
        border: 1px solid var(--color-secondary, #dee2e6);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-lg);
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        transition: box-shadow 0.2s ease;

        &:hover {
            box-shadow: 0 4px 8px rgba(0,0,0,0.15);
        }

        .card-header {
            font-size: var(--font-size-lg);
            font-weight: 600;
            margin-bottom: var(--spacing-md);
            color: var(--color-dark);
        }

        .card-body {
            color: var(--color-dark);
            line-height: 1.6;
        }
    };

    println!("   ✅ 卡片样式（设计令牌）: {}", card_style);
    println!();
}

/// 演示亮色/暗色主题
fn demo_light_dark_themes() {
    println!("🌓 3. 亮色/暗色主题");

    // 定义亮色主题
    let light_theme = css! {
        :root[data-theme="light"] {
            --color-background: #ffffff;
            --color-surface: #f8f9fa;
            --color-primary: #007bff;
            --color-on-primary: #ffffff;
            --color-secondary: #6c757d;
            --color-on-secondary: #ffffff;
            --color-text-primary: #212529;
            --color-text-secondary: #6c757d;
            --color-border: #dee2e6;
            --color-shadow: rgba(0, 0, 0, 0.1);
        }
    };

    // 定义暗色主题
    let dark_theme = css! {
        :root[data-theme="dark"] {
            --color-background: #121212;
            --color-surface: #1e1e1e;
            --color-primary: #bb86fc;
            --color-on-primary: #000000;
            --color-secondary: #03dac6;
            --color-on-secondary: #000000;
            --color-text-primary: #ffffff;
            --color-text-secondary: #b3b3b3;
            --color-border: #333333;
            --color-shadow: rgba(0, 0, 0, 0.3);
        }

        @media (prefers-color-scheme: dark) {
            :root:not([data-theme]) {
                --color-background: #121212;
                --color-surface: #1e1e1e;
                --color-primary: #bb86fc;
                --color-text-primary: #ffffff;
                --color-text-secondary: #b3b3b3;
            }
        }
    };

    // 创建主题感知的样式
    let theme_aware_style = css! {
        background-color: var(--color-surface);
        color: var(--color-text-primary);
        border: 1px solid var(--color-border);
        padding: var(--spacing-md);
        border-radius: var(--border-radius-md);
        box-shadow: 0 2px 4px var(--color-shadow);
        transition: all 0.3s ease;

        .title {
            color: var(--color-text-primary);
            font-weight: bold;
            margin-bottom: var(--spacing-sm);
        }

        .subtitle {
            color: var(--color-text-secondary);
            font-size: var(--font-size-sm);
        }

        .actions {
            margin-top: var(--spacing-md);
            display: flex;
            gap: var(--spacing-sm);
        }
    };

    println!("   ✅ 亮色主题: {}", light_theme);
    println!("   ✅ 暗色主题: {}", dark_theme);
    println!("   ✅ 主题感知样式: {}", theme_aware_style);
    println!();
}

/// 演示主题变量
fn demo_theme_variables() {
    println!("🔧 4. 主题变量系统");

    // 创建使用主题变量的复杂组件
    let complex_component_style = css! {
        // 组件级变量
        --component-padding: var(--spacing-md);
        --component-border-radius: var(--border-radius-lg);
        --component-shadow: 0 4px 6px var(--color-shadow);

        // 状态变量
        --component-bg: var(--color-surface);
        --component-border: var(--color-border);
        --component-text: var(--color-text-primary);

        // 应用变量
        background: var(--component-bg);
        border: 1px solid var(--component-border);
        border-radius: var(--component-border-radius);
        padding: var(--component-padding);
        color: var(--component-text);
        box-shadow: var(--component-shadow);
        transition: all 0.3s ease;

        // 嵌套元素使用变量
        .header {
            border-bottom: 1px solid var(--component-border);
            padding-bottom: calc(var(--component-padding) / 2);
            margin-bottom: var(--component-padding);
            font-weight: 600;
            font-size: var(--font-size-lg);
        }

        .content {
            line-height: 1.6;
            margin-bottom: var(--component-padding);
        }

        .footer {
            padding-top: calc(var(--component-padding) / 2);
            border-top: 1px solid var(--component-border);
            font-size: var(--font-size-sm);
            color: var(--color-text-secondary);
        }

        // 状态修饰符
        &:hover {
            --component-shadow: 0 8px 12px var(--color-shadow);
            transform: translateY(-2px);
        }

        &[data-state="active"] {
            --component-border: var(--color-primary);
            --component-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
        }

        &[data-state="error"] {
            --component-border: var(--color-danger);
            --component-bg: rgba(220, 53, 69, 0.05);
        }

        &[data-state="success"] {
            --component-border: var(--color-success);
            --component-bg: rgba(40, 167, 69, 0.05);
        }
    };

    println!(
        "   ✅ 复杂组件样式（主题变量）: {}",
        complex_component_style
    );

    // 演示变量继承
    let variable_inheritance = css! {
        .theme-container {
            --local-spacing: calc(var(--spacing-md) * 1.5);
            --local-color: var(--color-primary);

            padding: var(--local-spacing);
            border-left: 4px solid var(--local-color);

            .nested-element {
                // 继承父级变量
                margin: var(--local-spacing);
                color: var(--local-color);

                // 重新定义局部变量
                --local-color: var(--color-secondary);

                .deeply-nested {
                    // 使用重新定义的变量
                    border-color: var(--local-color);
                }
            }
        }
    };

    println!("   ✅ 变量继承样式: {}", variable_inheritance);
    println!();
}

/// 演示动态主题切换
fn demo_dynamic_theme_switching() {
    println!("🔄 5. 动态主题切换");

    // 主题切换器样式
    let theme_switcher = css! {
        .theme-switcher {
            display: flex;
            gap: var(--spacing-sm);
            padding: var(--spacing-sm);
            background: var(--color-surface);
            border-radius: var(--border-radius-lg);
            border: 1px solid var(--color-border);

            .theme-option {
                padding: var(--spacing-xs) var(--spacing-sm);
                border: 1px solid transparent;
                border-radius: var(--border-radius-md);
                cursor: pointer;
                transition: all 0.2s ease;
                font-size: var(--font-size-sm);

                &:hover {
                    background: var(--color-primary);
                    color: var(--color-on-primary);
                }

                &[data-active="true"] {
                    background: var(--color-primary);
                    color: var(--color-on-primary);
                    border-color: var(--color-primary);
                }
            }
        }
    };

    println!("   ✅ 主题切换器: {}", theme_switcher);

    // 演示主题切换动画
    let theme_transition_style = css! {
        * {
            transition:
                background-color 0.3s ease,
                color 0.3s ease,
                border-color 0.3s ease,
                box-shadow 0.3s ease;
        }

        [data-theme-switching] {
            pointer-events: none;

            &::before {
                content: "";
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: var(--color-background);
                opacity: 0;
                z-index: 9999;
                animation: themeSwitch 0.3s ease;
            }
        }

        @keyframes themeSwitch {
            0% { opacity: 0; }
            50% { opacity: 0.1; }
            100% { opacity: 0; }
        }
    };

    println!("   ✅ 主题切换动画: {}", theme_transition_style);

    // 模拟主题切换功能
    let themes = vec!["light", "dark", "enterprise", "auto"];

    for theme_name in themes {
        println!("   🔄 切换到主题: {}", theme_name);

        // 创建在当前主题下的样式
        let themed_style = css! {
            background: var(--color-primary);
            color: var(--color-on-primary);
            padding: var(--spacing-md);
            border-radius: var(--border-radius-md);
            margin: var(--spacing-sm);
            transition: all 0.3s ease;

            &::before {
                content: attr(data-theme);
                font-size: var(--font-size-xs);
                opacity: 0.7;
            }
        };

        println!("      ✅ 主题样式: {}", themed_style);
    }

    println!("   💾 主题持久化功能:");
    println!("      - 本地存储当前主题");
    println!("      - 页面刷新后恢复主题");
    println!("      - 系统主题偏好检测");
    println!("      - 主题切换历史记录");

    println!();
}

/// 测试基础主题功能
fn test_basic_theme() {
    println!("\n--- 测试基础主题功能 ---");

    // 创建自定义主题
    let custom_theme = Theme::new("custom-theme")
        .with_mode(ThemeMode::Light)
        .with_custom_variable("primary-color", "#ff6b6b")
        .with_custom_variable("secondary-color", "#4ecdc4")
        .with_custom_variable("background-color", "#ffffff")
        .with_custom_variable("text-color", "#333333");

    println!("自定义主题名称: {}", custom_theme.name);
    println!("主题模式: {:?}", custom_theme.mode);
    println!("自定义变量数量: {}", custom_theme.custom_variables.len());

    // 获取主题变量
    if let Some(primary) = custom_theme.custom_variables.get("primary-color") {
        println!("主色调: {}", primary);
    }

    // 生成CSS变量
    let css_vars = custom_theme.to_css_variables();
    println!("生成的CSS变量:\n{}", css_vars);
}

/// 测试Ant Design主题
fn test_ant_design_theme() {
    println!("\n--- 测试Ant Design主题 ---");

    // 创建Ant Design默认主题
    let ant_theme = Theme::ant_design();
    println!("Ant Design主题名称: {}", ant_theme.name);
    println!("主题模式: {:?}", ant_theme.mode);

    // 获取设计令牌
    if let Some(primary_color) = ant_theme.get_token("colors.primary") {
        println!("Ant Design主色调: {}", primary_color);
    }

    // 创建暗色主题
    let dark_theme = Theme::ant_design_dark();
    println!("Ant Design暗色主题名称: {}", dark_theme.name);
    println!("暗色主题模式: {:?}", dark_theme.mode);

    // 比较两个主题的差异
    let light_css = ant_theme.to_css_variables();
    let dark_css = dark_theme.to_css_variables();

    println!("\n浅色主题CSS变量 (前200字符):");
    println!("{}", &light_css[..light_css.len().min(200)]);

    println!("\n暗色主题CSS变量 (前200字符):");
    println!("{}", &dark_css[..dark_css.len().min(200)]);
}

/// 测试主题切换
fn test_theme_switching() {
    println!("\n--- 测试主题切换 ---");

    // 创建主题提供者
    let provider = ThemeProvider::new();

    // 注册主题
    let light_theme = Theme::ant_design();
    let dark_theme = Theme::ant_design_dark();

    if let Err(e) = provider.register_theme("ant-design", light_theme) {
        println!("注册亮色主题失败: {}", e);
        return;
    }

    if let Err(e) = provider.register_theme("ant-design-dark", dark_theme) {
        println!("注册暗色主题失败: {}", e);
        return;
    }

    // 获取当前主题
    match provider.current_theme() {
        Ok(current) => {
            println!("当前主题: {}", current.name);
        }
        Err(e) => {
            println!("获取当前主题失败: {}", e);
        }
    }

    // 获取可用主题列表
    match provider.registered_themes() {
        Ok(themes) => {
            println!("可用主题: {:?}", themes);
        }
        Err(e) => {
            println!("获取主题列表失败: {}", e);
        }
    }

    // 尝试切换到暗色主题
    match provider.switch_theme("ant-design-dark") {
        Ok(result) => {
            println!("主题切换成功: {}", result.success);
            if let Some(error) = result.error {
                println!("切换错误: {}", error);
            }
        }
        Err(e) => {
            println!("主题切换失败: {}", e);
        }
    }

    // 验证切换后的主题
    match provider.current_theme() {
        Ok(current) => {
            println!("切换后的主题: {}", current.name);
        }
        Err(e) => {
            println!("获取切换后主题失败: {}", e);
        }
    }
}

/// 测试CSS变量生成
fn test_css_variables() {
    println!("\n--- 测试CSS变量生成 ---");

    // 创建CSS变量管理器
    let mut manager = CssVariableManager::new();

    // 从主题生成变量
    let theme = Theme::ant_design();
    match manager.generate_from_theme(&theme) {
        Ok(_) => {
            println!("从主题生成CSS变量成功");
        }
        Err(e) => {
            println!("生成CSS变量失败: {}", e);
        }
    }

    // 添加自定义变量
    manager.update_variable("custom-spacing", "16px");
    manager.update_variable("custom-font-size", "14px");
    manager.update_variable("custom-border-radius", "6px");

    // 生成最终CSS
    let css_output = manager.to_css();
    println!("\n生成的CSS变量 (前300字符):");
    println!("{}", &css_output[..css_output.len().min(300)]);

    // 测试变量获取
    if let Some(spacing) = manager.get_variable("custom-spacing") {
        println!("\n自定义间距变量: {}", spacing);
    }
}

/// 测试主题上下文
fn test_theme_context() {
    println!("\n--- 测试主题上下文 ---");

    // 创建主题上下文
    let context = ThemeContext::new();

    // 注册自定义主题
    let custom_theme = Theme::new("test-theme")
        .with_mode(ThemeMode::Light)
        .with_custom_variable("test-color", "#ff0000");

    match context.register_theme(custom_theme) {
        Ok(_) => {
            println!("注册自定义主题成功");
        }
        Err(e) => {
            println!("注册主题失败: {}", e);
        }
    }

    // 获取可用主题
    match context.available_themes() {
        Ok(themes) => {
            println!("上下文中的可用主题: {:?}", themes);
        }
        Err(e) => {
            println!("获取主题列表失败: {}", e);
        }
    }

    // 切换主题
    match context.switch_theme("test-theme") {
        Ok(_) => {
            println!("在上下文中切换主题成功");
        }
        Err(e) => {
            println!("在上下文中切换主题失败: {}", e);
        }
    }

    // 获取主题令牌
    match context.get_token("test-color") {
        Ok(Some(value)) => {
            println!("获取主题令牌 'test-color': {}", value);
        }
        Ok(None) => {
            println!("主题令牌 'test-color' 不存在");
        }
        Err(e) => {
            println!("获取主题令牌失败: {}", e);
        }
    }
}

/// 测试主题管理器
fn test_theme_manager() {
    println!("\n--- 测试主题管理器 ---");

    // 创建主题管理器
    let manager = ThemeManager::new();

    // 切换主题
    match manager.switch_theme("ant-design-dark") {
        Ok(result) => {
            println!("管理器切换主题成功: {:?}", result.success);
        }
        Err(e) => {
            println!("管理器切换主题失败: {}", e);
        }
    }

    // 测试历史记录功能
    match manager.switch_theme("ant-design") {
        Ok(_) => {
            println!("切换回默认主题成功");
        }
        Err(e) => {
            println!("切换回默认主题失败: {}", e);
        }
    }

    // 测试回退功能
    match manager.go_back() {
        Ok(Some(result)) => {
            println!("回退到上一个主题成功: {:?}", result.success);
        }
        Ok(None) => {
            println!("没有可回退的主题");
        }
        Err(e) => {
            println!("回退主题失败: {}", e);
        }
    }

    // 获取当前主题
    match manager.provider().current_theme() {
        Ok(theme) => {
            println!("管理器当前主题: {}", theme.name);
        }
        Err(e) => {
            println!("获取管理器当前主题失败: {}", e);
        }
    }
}
