//! CSS-in-Rust 演示应用
//!
//! 这个应用展示了如何使用 css-in-rust 与 Dioxus 框架结合
//! 实现主题化的 Ant Design 组件

use css_in_rust::theme_bridge::{init_global_theme_bridge, InjectionStrategy, Theme, ThemeMode};
use css_in_rust_examples::{SimpleThemeDemo, ThemeDemo};
use dioxus::prelude::*;

fn main() {
    // 初始化日志
    #[cfg(debug_assertions)]
    {
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();
    }

    // 启动 Dioxus 应用
    launch(App);
}

/// 主应用组件
#[component]
fn App() -> Element {
    // 应用状态
    let mut current_demo = use_signal(|| "theme");
    let mut theme_initialized = use_signal(|| false);

    // 初始化全局主题
    use_effect(move || {
        if !theme_initialized() {
            let theme = Theme {
                name: "ant-design".to_string(),
                mode: ThemeMode::Light,
            };

            match init_global_theme_bridge(theme, InjectionStrategy::Replace, true) {
                Ok(_) => {
                    theme_initialized.set(true);
                    println!("✅ Global theme bridge initialized successfully");
                }
                Err(e) => {
                    eprintln!("❌ Failed to initialize global theme bridge: {}", e);
                }
            }
        }
    });

    // 导航样式
    let nav_style = css! {
        "
        background-color: var(--ant-color-bg-container, #ffffff);
        border-bottom: 1px solid var(--ant-color-border, #d9d9d9);
        padding: var(--ant-padding, 16px) var(--ant-padding-lg, 24px);
        margin-bottom: var(--ant-margin-lg, 24px);
        display: flex;
        gap: var(--ant-margin, 16px);
        align-items: center;
        "
    };

    let nav_button_style = css! {
        "
        background: none;
        border: 1px solid var(--ant-color-border, #d9d9d9);
        color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
        padding: var(--ant-padding-xs, 8px) var(--ant-padding-sm, 12px);
        border-radius: var(--ant-border-radius, 6px);
        cursor: pointer;
        transition: all 0.2s;

        &:hover {
            border-color: var(--ant-color-primary, #1677ff);
            color: var(--ant-color-primary, #1677ff);
        }

        &.active {
            background-color: var(--ant-color-primary, #1677ff);
            border-color: var(--ant-color-primary, #1677ff);
            color: var(--ant-color-white, #ffffff);
        }
        "
    };

    // 应用容器样式
    let app_container_style = css! {
        "
        min-height: 100vh;
        background-color: var(--ant-color-bg-layout, #f5f5f5);
        color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
        transition: all 0.3s ease;
        font-family: var(--ant-font-family, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif);
        "
    };

    rsx! {
        div {
            class: app_container_style,

            // 导航栏
            nav {
                class: nav_style,

                h1 {
                    style: "margin: 0; font-size: 24px; font-weight: 600;",
                    "🎨 CSS-in-Rust 演示"
                }

                div {
                    style: "margin-left: auto; display: flex; gap: 8px;",

                    button {
                        class: format!("{} {}", nav_button_style, if current_demo() == "theme" { "active" } else { "" }),
                        onclick: move |_| current_demo.set("theme"),
                        "完整演示"
                    }

                    button {
                        class: format!("{} {}", nav_button_style, if current_demo() == "simple" { "active" } else { "" }),
                        onclick: move |_| current_demo.set("simple"),
                        "简单演示"
                    }
                }
            }

            // 主要内容区域
            main {
                match current_demo().as_str() {
                    "simple" => rsx! { SimpleThemeDemo {} },
                    _ => rsx! { ThemeDemo {} },
                }
            }

            // 页脚
            footer {
                style: "
                    text-align: center;
                    padding: 24px;
                    color: var(--ant-color-text-secondary, rgba(0, 0, 0, 0.45));
                    border-top: 1px solid var(--ant-color-border, #d9d9d9);
                    background-color: var(--ant-color-bg-container, #ffffff);
                ",

                p {
                    style: "margin: 0;",
                    "🚀 Powered by "
                    strong { "css-in-rust" }
                    " + "
                    strong { "Dioxus" }
                }

                p {
                    style: "margin: 8px 0 0 0; font-size: 12px;",
                    "展示了编译时 CSS 优化、主题系统集成和类型安全的样式 API"
                }
            }
        }
    }
}

/// 全局样式重置
const GLOBAL_STYLES: &str = r#"
/* 全局样式重置 */
* {
    box-sizing: border-box;
}

body {
    margin: 0;
    padding: 0;
    font-family: var(--ant-font-family, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif);
    background-color: var(--ant-color-bg-layout, #f5f5f5);
    color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
    transition: background-color 0.3s ease, color 0.3s ease;
}

/* 默认的 CSS 变量（亮色主题） */
:root {
    /* 颜色变量 */
    --ant-color-primary: #1677ff;
    --ant-color-primary-hover: #4096ff;
    --ant-color-primary-active: #0958d9;
    --ant-color-primary-bg: rgba(22, 119, 255, 0.1);

    --ant-color-success: #52c41a;
    --ant-color-warning: #faad14;
    --ant-color-error: #ff4d4f;
    --ant-color-error-hover: #ff7875;
    --ant-color-error-active: #d9363e;
    --ant-color-error-bg: rgba(255, 77, 79, 0.1);
    --ant-color-info: #1677ff;

    --ant-color-text: rgba(0, 0, 0, 0.88);
    --ant-color-text-secondary: rgba(0, 0, 0, 0.65);
    --ant-color-text-tertiary: rgba(0, 0, 0, 0.45);
    --ant-color-text-quaternary: rgba(0, 0, 0, 0.25);
    --ant-color-text-heading: rgba(0, 0, 0, 0.88);

    --ant-color-bg-container: #ffffff;
    --ant-color-bg-layout: #f5f5f5;
    --ant-color-bg-spotlight: #ffffff;

    --ant-color-border: #d9d9d9;
    --ant-color-border-secondary: #f0f0f0;

    --ant-color-fill: rgba(0, 0, 0, 0.15);
    --ant-color-fill-secondary: rgba(0, 0, 0, 0.06);
    --ant-color-fill-tertiary: rgba(0, 0, 0, 0.04);
    --ant-color-fill-quaternary: rgba(0, 0, 0, 0.02);

    --ant-color-white: #ffffff;
    --ant-color-link: #1677ff;
    --ant-color-link-hover: #4096ff;
    --ant-color-link-active: #0958d9;

    /* 尺寸变量 */
    --ant-control-height: 32px;
    --ant-control-height-lg: 40px;
    --ant-control-height-sm: 24px;

    /* 间距变量 */
    --ant-padding-xs: 8px;
    --ant-padding-sm: 12px;
    --ant-padding: 16px;
    --ant-padding-lg: 24px;
    --ant-padding-xl: 32px;

    --ant-margin-xs: 8px;
    --ant-margin-sm: 12px;
    --ant-margin: 16px;
    --ant-margin-lg: 24px;
    --ant-margin-xl: 32px;

    /* 字体变量 */
    --ant-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif;
    --ant-font-family-code: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;

    --ant-font-size: 14px;
    --ant-font-size-sm: 12px;
    --ant-font-size-lg: 16px;
    --ant-font-size-xl: 20px;

    --ant-font-size-heading-1: 38px;
    --ant-font-size-heading-2: 30px;
    --ant-font-size-heading-3: 24px;
    --ant-font-size-heading-4: 20px;
    --ant-font-size-heading-5: 16px;

    --ant-font-weight: 400;
    --ant-font-weight-strong: 600;

    --ant-line-height: 1.5714285714285714;

    /* 边框变量 */
    --ant-border-radius: 6px;
    --ant-border-radius-sm: 4px;
    --ant-border-radius-lg: 8px;

    --ant-line-width: 1px;
    --ant-line-type: solid;

    /* 阴影变量 */
    --ant-box-shadow: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);
    --ant-box-shadow-secondary: 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05);

    /* 其他变量 */
    --ant-opacity-disabled: 0.25;
    --ant-z-index-base: 0;
    --ant-z-index-popup: 1000;
}

/* 暗色主题变量 */
[data-theme="dark"] {
    --ant-color-text: rgba(255, 255, 255, 0.85);
    --ant-color-text-secondary: rgba(255, 255, 255, 0.65);
    --ant-color-text-tertiary: rgba(255, 255, 255, 0.45);
    --ant-color-text-quaternary: rgba(255, 255, 255, 0.25);
    --ant-color-text-heading: rgba(255, 255, 255, 0.85);

    --ant-color-bg-container: #141414;
    --ant-color-bg-layout: #000000;
    --ant-color-bg-spotlight: #141414;

    --ant-color-border: #424242;
    --ant-color-border-secondary: #303030;

    --ant-color-fill: rgba(255, 255, 255, 0.18);
    --ant-color-fill-secondary: rgba(255, 255, 255, 0.12);
    --ant-color-fill-tertiary: rgba(255, 255, 255, 0.08);
    --ant-color-fill-quaternary: rgba(255, 255, 255, 0.04);
}
"#;

/// 注入全局样式
#[component]
fn GlobalStyles() -> Element {
    rsx! {
        style { {GLOBAL_STYLES} }
    }
}
