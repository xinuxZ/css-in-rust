//! 主题演示模块
//!
//! 展示如何使用 css-in-rust 的主题系统和组件

use crate::components::{Button, ButtonSize, ButtonType};
use css_in_rust::{
    css,
    theme::{InjectionStrategy, Theme, ThemeMode},
    theme_bridge::{
        get_current_css_variables, init_global_theme_bridge, toggle_global_theme_mode,
        with_global_theme_bridge,
    },
};
use dioxus::prelude::*;

use base64 as _;
use chrono as _;
#[allow(unused_imports)]
use css_in_rust_macros as _;
use lazy_static as _;
#[allow(unused_imports)]
use lightningcss as _;
use num_cpus as _;
#[allow(unused_imports)]
use proc_macro2 as _;
#[allow(unused_imports)]
use quote as _;
use regex as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
use sha1 as _;
#[allow(unused_imports)]
use sha2 as _;
#[allow(unused_imports)]
use syn as _;
use tempfile as _;

/// 主题演示应用
///
/// 展示了如何使用 css-in-rust 的主题系统：
/// - 全局主题管理
/// - 亮色/暗色模式切换
/// - 组件样式与主题的集成
/// - CSS 变量的动态更新
#[component]
pub fn ThemeDemo() -> Element {
    // 初始化主题状态
    let mut theme_initialized = use_signal(|| false);
    let mut current_mode = use_signal(|| "light");

    // 初始化全局主题桥接器
    use_effect(move || {
        if !theme_initialized() {
            let theme = Theme {
                name: "ant-design".to_string(),
                mode: ThemeMode::Light,
            };

            if let Ok(_) = init_global_theme_bridge(theme, InjectionStrategy::Replace, true) {
                theme_initialized.set(true);
                println!("Theme bridge initialized successfully");
            } else {
                eprintln!("Failed to initialize theme bridge");
            }
        }
    });

    // 切换主题模式的处理函数
    let toggle_theme = move |_| {
        if let Some(result) = toggle_global_theme_mode() {
            match result {
                Ok(_) => {
                    let new_mode = if current_mode() == "light" {
                        "dark"
                    } else {
                        "light"
                    };
                    current_mode.set(new_mode);
                    println!("Theme mode switched to: {}", new_mode);
                }
                Err(e) => {
                    eprintln!("Failed to toggle theme mode: {}", e);
                }
            }
        }
    };

    // 获取当前主题信息
    let theme_info = with_global_theme_bridge(|bridge| {
        format!(
            "当前主题: {} | 模式: {}",
            bridge.theme_name(),
            if bridge.is_dark_mode() {
                "暗色"
            } else {
                "亮色"
            }
        )
    })
    .unwrap_or_else(|| "主题未初始化".to_string());

    // 演示容器样式
    let container_class = css!(
        "
        max-width: 1200px;
        margin: 0 auto;
        padding: var(--ant-padding-lg, 24px);
        background-color: var(--ant-color-bg-container, #ffffff);
        color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
        min-height: 100vh;
        transition: all 0.3s ease;
        "
    );

    // 标题样式
    let title_class = css! {
        "
        font-size: var(--ant-font-size-heading-2, 30px);
        font-weight: var(--ant-font-weight-strong, 600);
        color: var(--ant-color-text-heading, rgba(0, 0, 0, 0.88));
        margin-bottom: var(--ant-margin-lg, 24px);
        text-align: center;
        "
    };

    // 部分标题样式
    let section_title_class = css! {
        "
        font-size: var(--ant-font-size-heading-4, 20px);
        font-weight: var(--ant-font-weight-strong, 600);
        color: var(--ant-color-text-heading, rgba(0, 0, 0, 0.88));
        margin: var(--ant-margin-lg, 24px) 0 var(--ant-margin, 16px) 0;
        border-bottom: 1px solid var(--ant-color-border, #d9d9d9);
        padding-bottom: var(--ant-padding-xs, 8px);
        "
    };

    // 按钮组样式
    let button_group_class = css! {
        "
        display: flex;
        flex-wrap: wrap;
        gap: var(--ant-margin, 16px);
        margin-bottom: var(--ant-margin-lg, 24px);
        "
    };

    // 信息卡片样式
    let info_card_class = css! {
        "
        background-color: var(--ant-color-fill-quaternary, rgba(0, 0, 0, 0.02));
        border: 1px solid var(--ant-color-border, #d9d9d9);
        border-radius: var(--ant-border-radius, 6px);
        padding: var(--ant-padding, 16px);
        margin-bottom: var(--ant-margin-lg, 24px);
        "
    };

    // 代码块样式
    let code_block_class = css! {
        "
        background-color: var(--ant-color-fill-tertiary, rgba(0, 0, 0, 0.04));
        border: 1px solid var(--ant-color-border, #d9d9d9);
        border-radius: var(--ant-border-radius, 6px);
        padding: var(--ant-padding, 16px);
        font-family: var(--ant-font-family-code, 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace);
        font-size: var(--ant-font-size-sm, 12px);
        line-height: 1.5;
        overflow-x: auto;
        white-space: pre-wrap;
        "
    };

    rsx! {
        div {
            class: container_class,

            // 页面标题
            h1 {
                class: title_class,
                "CSS-in-Rust 主题系统演示"
            }

            // 主题信息卡片
            div {
                class: info_card_class,
                h3 { "🎨 当前主题状态" }
                p { "{theme_info}" }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: toggle_theme,
                    "切换主题模式"
                }
            }

            // 按钮类型演示
            h2 {
                class: section_title_class,
                "按钮类型演示"
            }

            div {
                class: button_group_class,

                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_| println!("Primary button clicked!"),
                    "Primary"
                }

                Button {
                    button_type: ButtonType::Default,
                    onclick: move |_| println!("Default button clicked!"),
                    "Default"
                }

                Button {
                    button_type: ButtonType::Dashed,
                    onclick: move |_| println!("Dashed button clicked!"),
                    "Dashed"
                }

                Button {
                    button_type: ButtonType::Text,
                    onclick: move |_| println!("Text button clicked!"),
                    "Text"
                }

                Button {
                    button_type: ButtonType::Link,
                    onclick: move |_| println!("Link button clicked!"),
                    "Link"
                }
            }

            // 按钮尺寸演示
            h2 {
                class: section_title_class,
                "按钮尺寸演示"
            }

            div {
                class: button_group_class,

                Button {
                    button_type: ButtonType::Primary,
                    size: ButtonSize::Large,
                    "Large"
                }

                Button {
                    button_type: ButtonType::Primary,
                    size: ButtonSize::Middle,
                    "Middle"
                }

                Button {
                    button_type: ButtonType::Primary,
                    size: ButtonSize::Small,
                    "Small"
                }
            }

            // 按钮状态演示
            h2 {
                class: section_title_class,
                "按钮状态演示"
            }

            div {
                class: button_group_class,

                Button {
                    button_type: ButtonType::Primary,
                    danger: true,
                    "Danger Primary"
                }

                Button {
                    button_type: ButtonType::Default,
                    danger: true,
                    "Danger Default"
                }

                Button {
                    button_type: ButtonType::Primary,
                    disabled: true,
                    "Disabled"
                }

                Button {
                    button_type: ButtonType::Primary,
                    loading: true,
                    "Loading"
                }

                Button {
                    button_type: ButtonType::Primary,
                    ghost: true,
                    "Ghost"
                }
            }

            // 块级按钮演示
            h2 {
                class: section_title_class,
                "块级按钮演示"
            }

            Button {
                button_type: ButtonType::Primary,
                block: true,
                style: "margin-bottom: 16px;",
                "Block Button"
            }

            // CSS 变量信息
            h2 {
                class: section_title_class,
                "当前 CSS 变量"
            }

            div {
                class: code_block_class,
                {
                    get_current_css_variables()
                        .unwrap_or_else(|| "CSS 变量未加载".to_string())
                        .lines()
                        .take(20) // 只显示前20行
                        .collect::<Vec<_>>()
                        .join("\n")
                }
                if get_current_css_variables().map(|s| s.lines().count()).unwrap_or(0) > 20 {
                    "\n... (更多变量)"
                }
            }

            // 使用说明
            h2 {
                class: section_title_class,
                "使用说明"
            }

            div {
                class: info_card_class,

                h4 { "🚀 特性" }
                ul {
                    li { "✅ 完全集成主题系统，支持亮色/暗色模式自动切换" }
                    li { "✅ 使用 CSS 变量实现动态主题" }
                    li { "✅ 编译时 CSS 优化" }
                    li { "✅ 类型安全的样式 API" }
                    li { "✅ 与 Dioxus 框架无缝集成" }
                }

                h4 { "📝 代码示例" }
                div {
                    class: code_block_class,
                    "
// 使用 css! 宏创建样式
let button_class = css! {
    \"background-color: var(--ant-color-primary);
    color: var(--ant-color-white);
    padding: var(--ant-padding-sm);\"
};

// 使用 Button 组件
rsx! {
    Button {
        button_type: ButtonType::Primary,
        onclick: move |_| println!(\"Clicked!\"),
        \"Click me!\"
    }
}
                    "
                }
            }
        }
    }
}

/// 简单的主题切换演示
#[component]
pub fn SimpleThemeDemo() -> Element {
    let mut is_dark = use_signal(|| false);

    let toggle_theme = move |_| {
        is_dark.set(!is_dark());
        if let Some(result) = toggle_global_theme_mode() {
            match result {
                Ok(_) => println!("Theme toggled successfully"),
                Err(e) => eprintln!("Failed to toggle theme: {}", e),
            }
        }
    };

    let container_class = css! {
        "
        padding: 20px;
        background-color: var(--ant-color-bg-container, #ffffff);
        color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
        min-height: 100vh;
        transition: all 0.3s ease;
        "
    };

    rsx! {
        div {
            class: container_class,

            h1 { "简单主题演示" }

            p { "当前模式: {if is_dark() { \"暗色\" } else { \"亮色\" }}" }

            Button {
                button_type: ButtonType::Primary,
                onclick: toggle_theme,
                "切换主题"
            }

            div {
                style: "margin-top: 20px;",

                Button {
                    button_type: ButtonType::Default,
                    style: "margin-right: 10px;",
                    "默认按钮"
                }

                Button {
                    button_type: ButtonType::Primary,
                    danger: true,
                    "危险按钮"
                }
            }
        }
    }
}
