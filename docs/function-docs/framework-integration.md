# 框架集成指南

本指南详细介绍如何将 CSS-in-Rust 集成到各种 Rust Web 框架中，实现最佳的开发体验。

## 🎯 支持的框架

CSS-in-Rust 提供对以下主流 Rust Web 框架的原生支持：

- **Yew** - 现代化的 Rust/WebAssembly 框架
- **Leptos** - 全栈 Rust Web 框架
- **Dioxus** - 跨平台 GUI 框架
- **Sycamore** - 响应式 Web 框架
- **Percy** - 模块化 Web 框架
- **Seed** - 前端 Web 应用框架

## 🌟 Yew 集成

### 1. 基础设置

```toml
# Cargo.toml
[dependencies]
yew = "0.21"
css-in-rust = { version = "0.1.0", features = ["yew"] }
wasm-bindgen = "0.2"
web-sys = "0.3"

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Document",
  "Element",
  "HtmlElement",
  "Window",
]
```

### 2. 基础组件示例

```rust
// src/components/button.rs
use yew::prelude::*;
use css_in_rust::{css, css_if, theme};

/// 按钮组件属性
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// 按钮文本
    pub children: Children,
    /// 按钮变体
    #[prop_or_default]
    pub variant: ButtonVariant,
    /// 按钮大小
    #[prop_or_default]
    pub size: ButtonSize,
    /// 是否禁用
    #[prop_or_default]
    pub disabled: bool,
    /// 是否加载中
    #[prop_or_default]
    pub loading: bool,
    /// 点击事件处理器
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// 按钮组件
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_context::<ThemeContext>().expect("主题上下文未找到");

    // 基础样式
    let base_style = css! {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        border: none;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        text-decoration: none;
        outline: none;

        &:focus-visible {
            box-shadow: 0 0 0 2px ${theme.colors.primary}40;
        }

        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
    };

    // 变体样式
    let variant_style = match props.variant {
        ButtonVariant::Primary => css! {
            background: ${theme.colors.primary};
            color: ${theme.colors.primary_text};

            &:hover:not(:disabled) {
                background: ${theme.colors.primary_hover};
            }

            &:active:not(:disabled) {
                background: ${theme.colors.primary_active};
            }
        },
        ButtonVariant::Secondary => css! {
            background: ${theme.colors.secondary};
            color: ${theme.colors.secondary_text};
            border: 1px solid ${theme.colors.border};

            &:hover:not(:disabled) {
                background: ${theme.colors.secondary_hover};
            }
        },
        ButtonVariant::Danger => css! {
            background: ${theme.colors.danger};
            color: white;

            &:hover:not(:disabled) {
                background: ${theme.colors.danger_hover};
            }
        },
        ButtonVariant::Ghost => css! {
            background: transparent;
            color: ${theme.colors.text};
            border: 1px solid transparent;

            &:hover:not(:disabled) {
                background: ${theme.colors.ghost_hover};
                border-color: ${theme.colors.border};
            }
        },
    };

    // 大小样式
    let size_style = match props.size {
        ButtonSize::Small => css! {
            padding: 4px 8px;
            font-size: 12px;
            min-height: 24px;
        },
        ButtonSize::Medium => css! {
            padding: 8px 16px;
            font-size: 14px;
            min-height: 32px;
        },
        ButtonSize::Large => css! {
            padding: 12px 24px;
            font-size: 16px;
            min-height: 40px;
        },
    };

    // 状态样式
    let loading_style = css_if!(props.loading, {
        position: relative;
        color: transparent;

        &::after {
            content: "";
            position: absolute;
            width: 16px;
            height: 16px;
            border: 2px solid currentColor;
            border-radius: 50%;
            border-top-color: transparent;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            to {
                transform: rotate(360deg);
            }
        }
    });

    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            if !props.disabled && !props.loading {
                onclick.emit(e);
            }
        })
    };

    html! {
        <button
            class={classes!(
                base_style.class_name(),
                variant_style.class_name(),
                size_style.class_name(),
                loading_style.class_name(),
            )}
            disabled={props.disabled || props.loading}
            {onclick}
        >
            {props.children.clone()}
        </button>
    }
}
```

### 3. 主题提供者

```rust
// src/theme/provider.rs
use yew::prelude::*;
use css_in_rust::theme::{Theme, ThemeManager};
use std::rc::Rc;

/// 主题上下文
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Rc<Theme>,
    pub set_theme: Callback<String>,
}

/// 主题提供者属性
#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
    #[prop_or_default]
    pub initial_theme: Option<String>,
}

/// 主题提供者组件
#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme_manager = use_memo(|_| ThemeManager::new(), ());

    let current_theme = use_state(|| {
        props.initial_theme
            .as_ref()
            .and_then(|name| theme_manager.get_theme(name))
            .unwrap_or_else(|| theme_manager.get_default_theme())
    });

    let set_theme = {
        let current_theme = current_theme.clone();
        let theme_manager = theme_manager.clone();

        Callback::from(move |theme_name: String| {
            if let Some(theme) = theme_manager.get_theme(&theme_name) {
                current_theme.set(theme);

                // 保存到本地存储
                if let Some(storage) = web_sys::window()
                    .and_then(|w| w.local_storage().ok())
                    .flatten()
                {
                    let _ = storage.set_item("css-in-rust-theme", &theme_name);
                }
            }
        })
    };

    let theme_context = ThemeContext {
        theme: Rc::new((*current_theme).clone()),
        set_theme,
    };

    html! {
        <ContextProvider<ThemeContext> context={theme_context}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

/// 主题切换器组件
#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_context = use_context::<ThemeContext>()
        .expect("ThemeToggle 必须在 ThemeProvider 内使用");

    let toggle_style = css! {
        padding: 8px;
        border: 1px solid ${theme_context.theme.colors.border};
        border-radius: 4px;
        background: ${theme_context.theme.colors.background};
        color: ${theme_context.theme.colors.text};
        cursor: pointer;

        &:hover {
            background: ${theme_context.theme.colors.hover};
        }
    };

    let onclick = {
        let set_theme = theme_context.set_theme.clone();
        let current_theme = &theme_context.theme.name;

        let next_theme = if current_theme == "light" {
            "dark".to_string()
        } else {
            "light".to_string()
        };

        Callback::from(move |_: MouseEvent| {
            set_theme.emit(next_theme.clone());
        })
    };

    html! {
        <button class={toggle_style.class_name()} {onclick}>
            {if theme_context.theme.name == "light" { "🌙" } else { "☀️" }}
        </button>
    }
}
```

### 4. 应用入口

```rust
// src/main.rs
use yew::prelude::*;
use css_in_rust::global_css;
mod components;
mod theme;

use components::*;
use theme::*;

/// 全局样式
fn setup_global_styles() {
    global_css! {
        * {
            box-sizing: border-box;
        }

        body {
            margin: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.5;
        }

        #app {
            min-height: 100vh;
        }
    }
}

/// 主应用组件
#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set((*counter).max(1) - 1);
        })
    };

    let container_style = css! {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
        text-align: center;
    };

    let counter_style = css! {
        font-size: 2rem;
        margin: 20px 0;
        font-weight: bold;
    };

    let button_group_style = css! {
        display: flex;
        gap: 10px;
        justify-content: center;
        margin: 20px 0;
    };

    html! {
        <ThemeProvider initial_theme={Some("light".to_string())}>
            <div class={container_style.class_name()}>
                <div style="position: absolute; top: 20px; right: 20px;">
                    <ThemeToggle />
                </div>

                <h1>{"CSS-in-Rust + Yew"}</h1>

                <div class={counter_style.class_name()}>
                    {"计数: "}{*counter}
                </div>

                <div class={button_group_style.class_name()}>
                    <Button variant={ButtonVariant::Secondary} onclick={decrement}>
                        {"减少"}
                    </Button>

                    <Button variant={ButtonVariant::Primary} onclick={increment}>
                        {"增加"}
                    </Button>

                    <Button
                        variant={ButtonVariant::Danger}
                        onclick={Callback::from(move |_| counter.set(0))}
                    >
                        {"重置"}
                    </Button>
                </div>

                <div style="margin-top: 40px;">
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Small}>
                        {"小按钮"}
                    </Button>
                    {" "}
                    <Button variant={ButtonVariant::Primary} size={ButtonSize::Medium}>
                        {"中按钮"}
                    </Button>
                    {" "}
                    <Button variant={ButtonVariant::Secondary} size={ButtonSize::Large}>
                        {"大按钮"}
                    </Button>
                </div>
            </div>
        </ThemeProvider>
    }
}

fn main() {
    setup_global_styles();
    yew::Renderer::<App>::new().render();
}
```

## 🚀 Leptos 集成

### 1. 项目设置

```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.5", features = ["csr"] }
css-in-rust = { version = "0.1.0", features = ["leptos"] }
wasm-bindgen = "0.2"
web-sys = "0.3"
```

### 2. 响应式组件

```rust
// src/components/card.rs
use leptos::*;
use css_in_rust::{css, css_variants};

/// 卡片组件属性
#[derive(Debug, Clone, PartialEq)]
pub struct CardProps {
    pub title: String,
    pub children: Children,
    pub variant: CardVariant,
    pub elevated: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

/// 卡片组件
#[component]
pub fn Card(
    /// 卡片标题
    title: String,
    /// 卡片内容
    children: Children,
    /// 卡片变体
    #[prop(default = CardVariant::Default)]
    variant: CardVariant,
    /// 是否有阴影
    #[prop(default = false)]
    elevated: bool,
) -> impl IntoView {
    // 基础样式
    let base_style = css! {
        border-radius: 8px;
        padding: 16px;
        background: white;
        border: 1px solid #e1e5e9;
        transition: all 0.2s ease;
    };

    // 变体样式
    let variant_styles = css_variants! {
        default: {
            border-color: #e1e5e9;
        },
        primary: {
            border-color: #007bff;
            background: #f8f9ff;
        },
        success: {
            border-color: #28a745;
            background: #f8fff9;
        },
        warning: {
            border-color: #ffc107;
            background: #fffef8;
        },
        error: {
            border-color: #dc3545;
            background: #fff8f8;
        }
    };

    // 阴影样式
    let elevated_style = css! {
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);

        &:hover {
            box-shadow: 0 8px 15px rgba(0, 0, 0, 0.15);
            transform: translateY(-2px);
        }
    };

    let title_style = css! {
        margin: 0 0 12px 0;
        font-size: 18px;
        font-weight: 600;
        color: #2c3e50;
    };

    let variant_class = match variant {
        CardVariant::Default => variant_styles.default(),
        CardVariant::Primary => variant_styles.primary(),
        CardVariant::Success => variant_styles.success(),
        CardVariant::Warning => variant_styles.warning(),
        CardVariant::Error => variant_styles.error(),
    };

    view! {
        <div class=move || {
            let mut classes = vec![
                base_style.class_name(),
                variant_class.class_name(),
            ];

            if elevated {
                classes.push(elevated_style.class_name());
            }

            classes.join(" ")
        }>
            <h3 class={title_style.class_name()}>{title}</h3>
            <div>{children()}</div>
        </div>
    }
}
```

### 3. 信号驱动的样式

```rust
// src/components/progress.rs
use leptos::*;
use css_in_rust::{css, css_if};

/// 进度条组件
#[component]
pub fn ProgressBar(
    /// 当前进度 (0-100)
    #[prop(into)]
    value: Signal<f64>,
    /// 最大值
    #[prop(default = 100.0)]
    max: f64,
    /// 是否显示文本
    #[prop(default = true)]
    show_text: bool,
    /// 是否有动画
    #[prop(default = true)]
    animated: bool,
) -> impl IntoView {
    let container_style = css! {
        width: 100%;
        height: 20px;
        background: #f0f0f0;
        border-radius: 10px;
        overflow: hidden;
        position: relative;
    };

    let bar_style = css! {
        height: 100%;
        background: linear-gradient(90deg, #007bff, #0056b3);
        transition: width 0.3s ease;
        border-radius: 10px;
        position: relative;
    };

    let animated_style = css_if!(animated, {
        &::after {
            content: "";
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(
                90deg,
                transparent,
                rgba(255, 255, 255, 0.3),
                transparent
            );
            animation: shimmer 2s infinite;
        }

        @keyframes shimmer {
            0% { transform: translateX(-100%); }
            100% { transform: translateX(100%); }
        }
    });

    let text_style = css! {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        font-size: 12px;
        font-weight: 600;
        color: white;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    };

    // 计算进度百分比
    let percentage = move || {
        let val = value.get();
        ((val / max) * 100.0).min(100.0).max(0.0)
    };

    view! {
        <div class={container_style.class_name()}>
            <div
                class=move || {
                    let mut classes = vec![bar_style.class_name()];
                    if animated {
                        classes.push(animated_style.class_name());
                    }
                    classes.join(" ")
                }
                style=move || format!("width: {:.1}%", percentage())
            >
                {move || show_text.then(|| view! {
                    <span class={text_style.class_name()}>
                        {move || format!("{:.0}%", percentage())}
                    </span>
                })}
            </div>
        </div>
    }
}
```

### 4. Leptos 应用示例

```rust
// src/app.rs
use leptos::*;
use css_in_rust::{css, global_css};
use crate::components::*;

/// 设置全局样式
fn setup_global_styles() {
    global_css! {
        body {
            margin: 0;
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            background: #f8f9fa;
        }

        * {
            box-sizing: border-box;
        }
    }
}

/// 主应用组件
#[component]
fn App() -> impl IntoView {
    setup_global_styles();

    let (progress, set_progress) = create_signal(0.0);
    let (is_loading, set_is_loading) = create_signal(false);

    let container_style = css! {
        max-width: 1200px;
        margin: 0 auto;
        padding: 20px;
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 20px;
    };

    let header_style = css! {
        grid-column: 1 / -1;
        text-align: center;
        margin-bottom: 20px;

        h1 {
            color: #2c3e50;
            margin: 0;
        }
    };

    // 模拟进度更新
    let start_progress = move |_| {
        set_is_loading.set(true);
        set_progress.set(0.0);

        let handle = set_interval(
            move || {
                set_progress.update(|p| {
                    *p += 10.0;
                    if *p >= 100.0 {
                        set_is_loading.set(false);
                    }
                });
            },
            std::time::Duration::from_millis(200),
        );

        // 清理定时器
        create_effect(move |_| {
            if !is_loading.get() {
                handle.clear();
            }
        });
    };

    view! {
        <div class={container_style.class_name()}>
            <header class={header_style.class_name()}>
                <h1>"CSS-in-Rust + Leptos"</h1>
            </header>

            <Card title="基础卡片".to_string() variant=CardVariant::Default>
                <p>"这是一个基础卡片组件，展示了 CSS-in-Rust 在 Leptos 中的使用。"</p>
            </Card>

            <Card title="主要信息".to_string() variant=CardVariant::Primary elevated=true>
                <p>"这是一个带有阴影效果的主要信息卡片。"</p>
                <ProgressBar value=progress show_text=true animated=true />
                <br />
                <button
                    on:click=start_progress
                    disabled=move || is_loading.get()
                >
                    {move || if is_loading.get() { "加载中..." } else { "开始进度" }}
                </button>
            </Card>

            <Card title="成功状态".to_string() variant=CardVariant::Success>
                <p>"✅ 操作已成功完成！"</p>
            </Card>

            <Card title="警告信息".to_string() variant=CardVariant::Warning>
                <p>"⚠️ 请注意这个重要信息。"</p>
            </Card>

            <Card title="错误状态".to_string() variant=CardVariant::Error>
                <p>"❌ 发生了一个错误，请重试。"</p>
            </Card>
        </div>
    }
}

pub fn main() {
    mount_to_body(App);
}
```

## 🎮 Dioxus 集成

### 1. 项目配置

```toml
# Cargo.toml
[dependencies]
dioxus = { version = "0.4", features = ["web"] }
css-in-rust = { version = "0.1.0", features = ["dioxus"] }
```

### 2. 组件系统

```rust
// src/components/layout.rs
use dioxus::prelude::*;
use css_in_rust::{css, css_variants};

/// 布局组件属性
#[derive(Props)]
pub struct LayoutProps<'a> {
    children: Element<'a>,
    #[props(default = LayoutType::Default)]
    layout_type: LayoutType,
    #[props(default = false)]
    full_height: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LayoutType {
    #[default]
    Default,
    Sidebar,
    Grid,
    Flex,
}

/// 布局组件
pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    let base_style = css! {
        width: 100%;
        padding: 20px;
        box-sizing: border-box;
    };

    let layout_styles = css_variants! {
        default: {
            max-width: 1200px;
            margin: 0 auto;
        },
        sidebar: {
            display: grid;
            grid-template-columns: 250px 1fr;
            gap: 20px;
            max-width: 1400px;
            margin: 0 auto;
        },
        grid: {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            max-width: 1200px;
            margin: 0 auto;
        },
        flex: {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            max-width: 1200px;
            margin: 0 auto;
        }
    };

    let full_height_style = css! {
        min-height: 100vh;
    };

    let layout_class = match cx.props.layout_type {
        LayoutType::Default => layout_styles.default(),
        LayoutType::Sidebar => layout_styles.sidebar(),
        LayoutType::Grid => layout_styles.grid(),
        LayoutType::Flex => layout_styles.flex(),
    };

    let class_names = if cx.props.full_height {
        format!("{} {} {}",
            base_style.class_name(),
            layout_class.class_name(),
            full_height_style.class_name()
        )
    } else {
        format!("{} {}",
            base_style.class_name(),
            layout_class.class_name()
        )
    };

    render! {
        div {
            class: "{class_names}",
            {&cx.props.children}
        }
    }
}
```

### 3. 状态管理集成

```rust
// src/components/counter.rs
use dioxus::prelude::*;
use css_in_rust::{css, css_if};

/// 计数器组件
pub fn Counter(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    let is_even = count.get() % 2 == 0;

    let container_style = css! {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
        padding: 24px;
        border-radius: 12px;
        background: white;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    };

    let counter_display_style = css! {
        font-size: 3rem;
        font-weight: bold;
        transition: all 0.3s ease;
    };

    let even_style = css_if!(is_even, {
        color: #007bff;
        text-shadow: 0 0 10px rgba(0, 123, 255, 0.3);
    });

    let odd_style = css_if!(!is_even, {
        color: #dc3545;
        text-shadow: 0 0 10px rgba(220, 53, 69, 0.3);
    });

    let button_group_style = css! {
        display: flex;
        gap: 12px;
    };

    let button_style = css! {
        padding: 8px 16px;
        border: none;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            transform: translateY(-1px);
        }

        &:active {
            transform: translateY(0);
        }
    };

    let increment_button_style = css! {
        background: #28a745;
        color: white;

        &:hover {
            background: #218838;
        }
    };

    let decrement_button_style = css! {
        background: #dc3545;
        color: white;

        &:hover {
            background: #c82333;
        }
    };

    let reset_button_style = css! {
        background: #6c757d;
        color: white;

        &:hover {
            background: #5a6268;
        }
    };

    render! {
        div {
            class: "{container_style.class_name()}",

            h2 { "计数器" }

            div {
                class: "{counter_display_style.class_name()} {even_style.class_name()} {odd_style.class_name()}",
                "{count}"
            }

            div {
                class: "{button_group_style.class_name()}",

                button {
                    class: "{button_style.class_name()} {decrement_button_style.class_name()}",
                    onclick: move |_| count.set(count.get() - 1),
                    "减少"
                }

                button {
                    class: "{button_style.class_name()} {increment_button_style.class_name()}",
                    onclick: move |_| count.set(count.get() + 1),
                    "增加"
                }

                button {
                    class: "{button_style.class_name()} {reset_button_style.class_name()}",
                    onclick: move |_| count.set(0),
                    "重置"
                }
            }

            p {
                color: if is_even { "#007bff" } else { "#dc3545" },
                "当前数字是 {if is_even { "偶数" } else { "奇数" }}"
            }
        }
    }
}
```

### 4. Dioxus 应用入口

```rust
// src/main.rs
use dioxus::prelude::*;
use css_in_rust::global_css;

mod components;
use components::*;

/// 设置全局样式
fn setup_global_styles() {
    global_css! {
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }

        #main {
            min-height: 100vh;
        }
    }
}

/// 主应用组件
fn App(cx: Scope) -> Element {
    setup_global_styles();

    render! {
        Layout {
            layout_type: LayoutType::Grid,
            full_height: true,

            div {
                style: "grid-column: 1 / -1; text-align: center; margin-bottom: 20px;",
                h1 {
                    style: "color: white; font-size: 2.5rem; margin-bottom: 10px;",
                    "CSS-in-Rust + Dioxus"
                }
                p {
                    style: "color: rgba(255, 255, 255, 0.8); font-size: 1.1rem;",
                    "现代化的 Rust Web 开发体验"
                }
            }

            Counter {}

            div {
                style: "background: white; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);",
                h3 { "功能特性" }
                ul {
                    li { "🎨 类型安全的 CSS-in-Rust" }
                    li { "⚡ 零运行时开销" }
                    li { "🔥 热更新支持" }
                    li { "🎯 死代码消除" }
                    li { "🌈 主题系统" }
                }
            }

            div {
                style: "background: white; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);",
                h3 { "性能优势" }
                ul {
                    li { "📦 更小的包体积" }
                    li { "🚀 更快的渲染速度" }
                    li { "💾 智能缓存机制" }
                    li { "🔧 编译时优化" }
                }
            }
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
```

## 🌸 Sycamore 集成

### 1. 基础设置

```toml
# Cargo.toml
[dependencies]
sycamore = { version = "0.8", features = ["web"] }
css-in-rust = { version = "0.1.0", features = ["sycamore"] }
```

### 2. 响应式组件

```rust
// src/components/todo.rs
use sycamore::prelude::*;
use css_in_rust::{css, css_if};

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

/// 待办事项组件
#[component]
pub fn TodoApp<G: Html>(cx: Scope) -> View<G> {
    let todos = create_signal(cx, Vec::<TodoItem>::new());
    let input_value = create_signal(cx, String::new());
    let filter = create_signal(cx, TodoFilter::All);

    let container_style = css! {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    };

    let header_style = css! {
        text-align: center;
        margin-bottom: 30px;

        h1 {
            color: #2c3e50;
            margin: 0;
        }
    };

    let input_group_style = css! {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    };

    let input_style = css! {
        flex: 1;
        padding: 10px;
        border: 2px solid #e1e5e9;
        border-radius: 4px;
        font-size: 16px;

        &:focus {
            outline: none;
            border-color: #007bff;
        }
    };

    let add_button_style = css! {
        padding: 10px 20px;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;

        &:hover {
            background: #0056b3;
        }
    };

    let add_todo = move |_| {
        let text = input_value.get().trim().to_string();
        if !text.is_empty() {
            let new_todo = TodoItem {
                id: todos.get().len() as u32 + 1,
                text,
                completed: false,
            };
            todos.modify().push(new_todo);
            input_value.set(String::new());
        }
    };

    view! { cx,
        div(class=container_style.class_name()) {
            header(class=header_style.class_name()) {
                h1 { "待办事项" }
            }

            div(class=input_group_style.class_name()) {
                input(
                    class=input_style.class_name(),
                    type="text",
                    placeholder="添加新的待办事项...",
                    bind:value=input_value,
                    on:keydown=move |e: web_sys::KeyboardEvent| {
                        if e.key() == "Enter" {
                            add_todo(());
                        }
                    }
                )
                button(
                    class=add_button_style.class_name(),
                    on:click=add_todo
                ) { "添加" }
            }

            TodoList(todos=todos, filter=filter)
            TodoFilters(filter=filter)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

/// 待办事项列表组件
#[component]
pub fn TodoList<G: Html>(
    cx: Scope,
    todos: &'static Signal<Vec<TodoItem>>,
    filter: &'static Signal<TodoFilter>,
) -> View<G> {
    let list_style = css! {
        list-style: none;
        padding: 0;
        margin: 0;
    };

    let filtered_todos = create_memo(cx, move || {
        let todos = todos.get();
        let filter = *filter.get();

        todos.iter().filter(|todo| {
            match filter {
                TodoFilter::All => true,
                TodoFilter::Active => !todo.completed,
                TodoFilter::Completed => todo.completed,
            }
        }).cloned().collect::<Vec<_>>()
    });

    view! { cx,
        ul(class=list_style.class_name()) {
            Keyed(
                iterable=filtered_todos,
                key=|todo| todo.id,
                view=move |cx, todo| {
                    view! { cx, TodoItem(todo=todo, todos=todos) }
                }
            )
        }
    }
}

/// 单个待办事项组件
#[component]
pub fn TodoItem<G: Html>(
    cx: Scope,
    todo: TodoItem,
    todos: &'static Signal<Vec<TodoItem>>,
) -> View<G> {
    let item_style = css! {
        display: flex;
        align-items: center;
        padding: 12px;
        border-bottom: 1px solid #e1e5e9;
        transition: background-color 0.2s ease;

        &:hover {
            background: #f8f9fa;
        }
    };

    let completed_style = css_if!(todo.completed, {
        text-decoration: line-through;
        color: #6c757d;
        opacity: 0.7;
    });

    let checkbox_style = css! {
        margin-right: 12px;
        transform: scale(1.2);
    };

    let text_style = css! {
        flex: 1;
        font-size: 16px;
    };

    let delete_button_style = css! {
        background: #dc3545;
        color: white;
        border: none;
        padding: 4px 8px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 12px;

        &:hover {
            background: #c82333;
        }
    };

    let toggle_completed = move |_| {
        todos.modify().iter_mut().for_each(|t| {
            if t.id == todo.id {
                t.completed = !t.completed;
            }
        });
    };

    let delete_todo = move |_| {
        todos.modify().retain(|t| t.id != todo.id);
    };

    view! { cx,
        li(class=item_style.class_name()) {
            input(
                class=checkbox_style.class_name(),
                type="checkbox",
                checked=todo.completed,
                on:change=toggle_completed
            )
            span(
                class=format!("{} {}", text_style.class_name(), completed_style.class_name())
            ) { (todo.text) }
            button(
                class=delete_button_style.class_name(),
                on:click=delete_todo
            ) { "删除" }
        }
    }
}

/// 过滤器组件
#[component]
pub fn TodoFilters<G: Html>(
    cx: Scope,
    filter: &'static Signal<TodoFilter>,
) -> View<G> {
    let filters_style = css! {
        display: flex;
        justify-content: center;
        gap: 10px;
        margin-top: 20px;
    };

    let filter_button_style = css! {
        padding: 6px 12px;
        border: 1px solid #dee2e6;
        background: white;
        border-radius: 4px;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: #e9ecef;
        }
    };

    let active_filter_style = css! {
        background: #007bff;
        color: white;
        border-color: #007bff;

        &:hover {
            background: #0056b3;
        }
    };

    view! { cx,
        div(class=filters_style.class_name()) {
            button(
                class=if *filter.get() == TodoFilter::All {
                    format!("{} {}", filter_button_style.class_name(), active_filter_style.class_name())
                } else {
                    filter_button_style.class_name().to_string()
                },
                on:click=move |_| filter.set(TodoFilter::All)
            ) { "全部" }

            button(
                class=if *filter.get() == TodoFilter::Active {
                    format!("{} {}", filter_button_style.class_name(), active_filter_style.class_name())
                } else {
                    filter_button_style.class_name().to_string()
                },
                on:click=move |_| filter.set(TodoFilter::Active)
            ) { "未完成" }

            button(
                class=if *filter.get() == TodoFilter::Completed {
                    format!("{} {}", filter_button_style.class_name(), active_filter_style.class_name())
                } else {
                    filter_button_style.class_name().to_string()
                },
                on:click=move |_| filter.set(TodoFilter::Completed)
            ) { "已完成" }
        }
    }
}
```

## 📋 最佳实践

### ✅ 通用最佳实践

1. **组件设计**
   - 保持组件的单一职责
   - 使用 Props 传递样式配置
   - 提供合理的默认值
   - 支持样式覆盖和扩展

2. **样式组织**
   - 将基础样式与变体样式分离
   - 使用语义化的样式名称
   - 避免过度嵌套的样式
   - 利用 CSS 变量实现主题化

3. **性能优化**
   - 使用 `css_if!` 进行条件样式
   - 避免在渲染函数中创建样式
   - 利用框架的记忆化机制
   - 合理使用样式缓存

4. **类型安全**
   - 定义明确的 Props 类型
   - 使用枚举表示样式变体
   - 利用 Rust 的类型系统防止错误
   - 提供完整的类型注解

### ✅ 框架特定优化

**Yew:**
- 使用 `use_memo` 缓存复杂样式计算
- 利用 `use_context` 实现主题传递
- 合理使用 `use_effect` 处理样式副作用

**Leptos:**
- 利用信号系统实现响应式样式
- 使用 `create_memo` 优化样式计算
- 合理使用 `create_effect` 处理样式更新

**Dioxus:**
- 使用 `use_state` 管理样式状态
- 利用 `use_memo` 缓存样式计算
- 合理使用生命周期钩子

**Sycamore:**
- 利用响应式系统实现动态样式
- 使用 `create_memo` 优化性能
- 合理使用信号更新样式

通过遵循这些指南和最佳实践，您可以在各种 Rust Web 框架中充分发挥 CSS-in-Rust 的优势，构建高性能、类型安全的现代 Web 应用！🚀
