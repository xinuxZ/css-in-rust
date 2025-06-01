//! Button 组件 - 集成 css-in-rust 主题系统
//!
//! 这是一个使用 css-in-rust 重新实现的 Ant Design Button 组件
//! 展示了如何将主题系统与组件样式完美结合

use css_in_rust::{css, theme_bridge::with_global_theme_bridge};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// 按钮类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonType {
    /// 主按钮
    Primary,
    /// 默认按钮
    Default,
    /// 虚线按钮
    Dashed,
    /// 文本按钮
    Text,
    /// 链接按钮
    Link,
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonSize {
    /// 大尺寸
    Large,
    /// 中等尺寸（默认）
    Middle,
    /// 小尺寸
    Small,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Middle
    }
}

/// 按钮形状
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonShape {
    /// 默认形状
    Default,
    /// 圆形按钮
    Circle,
    /// 圆角按钮
    Round,
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Default
    }
}

/// 按钮 HTML 类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonHtmlType {
    /// 提交按钮
    Submit,
    /// 重置按钮
    Reset,
    /// 普通按钮
    Button,
}

impl Default for ButtonHtmlType {
    fn default() -> Self {
        Self::Button
    }
}

/// 按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// 按钮类型
    #[props(default)]
    pub button_type: ButtonType,

    /// 按钮尺寸
    #[props(default)]
    pub size: ButtonSize,

    /// 按钮形状
    #[props(default)]
    pub shape: ButtonShape,

    /// HTML 按钮类型
    #[props(default)]
    pub html_type: ButtonHtmlType,

    /// 是否为危险按钮
    #[props(default = false)]
    pub danger: bool,

    /// 是否为幽灵按钮
    #[props(default = false)]
    pub ghost: bool,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否加载中
    #[props(default = false)]
    pub loading: bool,

    /// 是否为块级按钮
    #[props(default = false)]
    pub block: bool,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 点击事件处理器
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// 子元素
    children: Element,
}

/// Button 组件 - 使用 css-in-rust 主题系统
///
/// # 特性
///
/// - 完全集成主题系统，支持亮色/暗色模式自动切换
/// - 使用 CSS 变量实现动态主题
/// - 编译时 CSS 优化
/// - 类型安全的样式 API
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::examples::components::Button;
///
/// fn app() -> Element {
///     rsx! {
///         Button {
///             button_type: ButtonType::Primary,
///             onclick: move |_| {
///                 println!("Button clicked!");
///             },
///             "Primary Button"
///         }
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // 生成基础样式
    let base_class = css! {
        "
        display: inline-flex;
        align-items: center;
        justify-content: center;
        position: relative;
        box-sizing: border-box;
        outline: none;
        cursor: pointer;
        user-select: none;
        touch-action: manipulation;
        transition: all 0.2s cubic-bezier(0.645, 0.045, 0.355, 1);

        /* 基础尺寸和间距 */
        height: var(--ant-control-height, 32px);
        padding: var(--ant-padding-xs, 4px) var(--ant-padding-sm, 15px);
        font-size: var(--ant-font-size, 14px);
        line-height: var(--ant-line-height, 1.5714285714285714);
        border-radius: var(--ant-border-radius, 6px);

        /* 边框样式 */
        border: var(--ant-line-width, 1px) var(--ant-line-type, solid) var(--ant-color-border, #d9d9d9);

        /* 字体样式 */
        font-family: var(--ant-font-family, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif);
        font-weight: var(--ant-font-weight, 400);
        text-align: center;
        text-decoration: none;

        /* 禁用状态 */
        &:disabled {
            cursor: not-allowed;
            opacity: var(--ant-opacity-disabled, 0.25);
        }

        /* Hover 状态 */
        &:hover:not(:disabled) {
            border-color: var(--ant-color-primary-hover, #4096ff);
            color: var(--ant-color-primary-hover, #4096ff);
        }

        /* Focus 状态 */
        &:focus:not(:disabled) {
            border-color: var(--ant-color-primary, #1677ff);
            box-shadow: 0 0 0 2px var(--ant-color-primary-bg, rgba(22, 119, 255, 0.1));
        }

        /* Active 状态 */
        &:active:not(:disabled) {
            border-color: var(--ant-color-primary-active, #0958d9);
            color: var(--ant-color-primary-active, #0958d9);
        }
        "
    };

    // 根据按钮类型生成样式
    let type_class = match props.button_type {
        ButtonType::Primary => css! {
            "
            background-color: var(--ant-color-primary, #1677ff);
            border-color: var(--ant-color-primary, #1677ff);
            color: var(--ant-color-white, #ffffff);

            &:hover:not(:disabled) {
                background-color: var(--ant-color-primary-hover, #4096ff);
                border-color: var(--ant-color-primary-hover, #4096ff);
                color: var(--ant-color-white, #ffffff);
            }

            &:focus:not(:disabled) {
                background-color: var(--ant-color-primary, #1677ff);
                border-color: var(--ant-color-primary, #1677ff);
                color: var(--ant-color-white, #ffffff);
            }

            &:active:not(:disabled) {
                background-color: var(--ant-color-primary-active, #0958d9);
                border-color: var(--ant-color-primary-active, #0958d9);
                color: var(--ant-color-white, #ffffff);
            }
            "
        },
        ButtonType::Default => css! {
            "
            background-color: var(--ant-color-bg-container, #ffffff);
            border-color: var(--ant-color-border, #d9d9d9);
            color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
            "
        },
        ButtonType::Dashed => css! {
            "
            background-color: var(--ant-color-bg-container, #ffffff);
            border-style: dashed;
            border-color: var(--ant-color-border, #d9d9d9);
            color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
            "
        },
        ButtonType::Text => css! {
            "
            background-color: transparent;
            border-color: transparent;
            color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
            box-shadow: none;

            &:hover:not(:disabled) {
                background-color: var(--ant-color-fill-tertiary, rgba(0, 0, 0, 0.04));
                border-color: transparent;
                color: var(--ant-color-text, rgba(0, 0, 0, 0.88));
            }

            &:focus:not(:disabled) {
                background-color: var(--ant-color-fill-tertiary, rgba(0, 0, 0, 0.04));
                border-color: transparent;
                box-shadow: none;
            }

            &:active:not(:disabled) {
                background-color: var(--ant-color-fill-secondary, rgba(0, 0, 0, 0.06));
                border-color: transparent;
            }
            "
        },
        ButtonType::Link => css! {
            "
            background-color: transparent;
            border-color: transparent;
            color: var(--ant-color-link, #1677ff);
            box-shadow: none;

            &:hover:not(:disabled) {
                background-color: transparent;
                border-color: transparent;
                color: var(--ant-color-link-hover, #4096ff);
            }

            &:focus:not(:disabled) {
                background-color: transparent;
                border-color: transparent;
                color: var(--ant-color-link, #1677ff);
                box-shadow: none;
            }

            &:active:not(:disabled) {
                background-color: transparent;
                border-color: transparent;
                color: var(--ant-color-link-active, #0958d9);
            }
            "
        },
    };

    // 根据尺寸生成样式
    let size_class = match props.size {
        ButtonSize::Large => css! {
            "
            height: var(--ant-control-height-lg, 40px);
            padding: var(--ant-padding-sm, 7px) var(--ant-padding, 15px);
            font-size: var(--ant-font-size-lg, 16px);
            border-radius: var(--ant-border-radius-lg, 8px);
            "
        },
        ButtonSize::Middle => css! { "" }, // 默认尺寸
        ButtonSize::Small => css! {
            "
            height: var(--ant-control-height-sm, 24px);
            padding: var(--ant-padding-xs, 0px) var(--ant-padding-xs, 7px);
            font-size: var(--ant-font-size-sm, 12px);
            border-radius: var(--ant-border-radius-sm, 4px);
            "
        },
    };

    // 根据形状生成样式
    let shape_class = match props.shape {
        ButtonShape::Circle => css! {
            "
            min-width: var(--ant-control-height, 32px);
            padding-left: 0;
            padding-right: 0;
            border-radius: 50%;
            "
        },
        ButtonShape::Round => css! {
            "
            border-radius: var(--ant-control-height, 32px);
            "
        },
        ButtonShape::Default => css! { "" },
    };

    // 危险按钮样式
    let danger_class = if props.danger {
        css! {
            "
            color: var(--ant-color-error, #ff4d4f);
            border-color: var(--ant-color-error, #ff4d4f);

            &:hover:not(:disabled) {
                color: var(--ant-color-error-hover, #ff7875);
                border-color: var(--ant-color-error-hover, #ff7875);
            }

            &:focus:not(:disabled) {
                color: var(--ant-color-error, #ff4d4f);
                border-color: var(--ant-color-error, #ff4d4f);
                box-shadow: 0 0 0 2px var(--ant-color-error-bg, rgba(255, 77, 79, 0.1));
            }

            &:active:not(:disabled) {
                color: var(--ant-color-error-active, #d9363e);
                border-color: var(--ant-color-error-active, #d9363e);
            }
            "
        }
    } else {
        css! { "" }
    };

    // 主要危险按钮样式
    let primary_danger_class = if props.danger && props.button_type == ButtonType::Primary {
        css! {
            "
            background-color: var(--ant-color-error, #ff4d4f);
            border-color: var(--ant-color-error, #ff4d4f);
            color: var(--ant-color-white, #ffffff);

            &:hover:not(:disabled) {
                background-color: var(--ant-color-error-hover, #ff7875);
                border-color: var(--ant-color-error-hover, #ff7875);
                color: var(--ant-color-white, #ffffff);
            }

            &:focus:not(:disabled) {
                background-color: var(--ant-color-error, #ff4d4f);
                border-color: var(--ant-color-error, #ff4d4f);
                color: var(--ant-color-white, #ffffff);
            }

            &:active:not(:disabled) {
                background-color: var(--ant-color-error-active, #d9363e);
                border-color: var(--ant-color-error-active, #d9363e);
                color: var(--ant-color-white, #ffffff);
            }
            "
        }
    } else {
        css! { "" }
    };

    // 幽灵按钮样式
    let ghost_class = if props.ghost {
        css! {
            "
            background-color: transparent;

            &:hover:not(:disabled),
            &:focus:not(:disabled),
            &:active:not(:disabled) {
                background-color: transparent;
            }
            "
        }
    } else {
        css! { "" }
    };

    // 块级按钮样式
    let block_class = if props.block {
        css! {
            "
            width: 100%;
            display: block;
            "
        }
    } else {
        css! { "" }
    };

    // 加载状态样式
    let loading_class = if props.loading {
        css! {
            "
            pointer-events: none;

            .ant-btn-loading-icon {
                display: inline-block;
                margin-right: var(--ant-margin-xs, 8px);
                animation: ant-spin 1s infinite linear;
            }

            @keyframes ant-spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }
            "
        }
    } else {
        css! { "" }
    };

    // 组合所有样式类
    let mut combined_classes = vec![
        base_class,
        type_class,
        size_class,
        shape_class,
        danger_class,
        primary_danger_class,
        ghost_class,
        block_class,
        loading_class,
    ];

    // 添加自定义类名
    if let Some(custom_class) = &props.class {
        combined_classes.push(custom_class.clone());
    }

    let final_class = combined_classes.join(" ");

    // 获取 HTML 类型
    let html_type = match props.html_type {
        ButtonHtmlType::Submit => "submit",
        ButtonHtmlType::Reset => "reset",
        ButtonHtmlType::Button => "button",
    };

    rsx! {
        button {
            class: final_class,
            style: props.style.clone().unwrap_or_default(),
            r#type: html_type,
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },

            // 加载图标
            if props.loading {
                span {
                    class: "ant-btn-loading-icon",
                    "⟳" // 简单的加载图标，实际项目中应该使用 SVG 或图标字体
                }
            }

            // 按钮内容
            span {
                class: "ant-btn-content",
                {props.children}
            }
        }
    }
}

/// 按钮组件的便捷构造函数
impl ButtonProps {
    /// 创建主按钮
    pub fn primary() -> Self {
        Self {
            button_type: ButtonType::Primary,
            ..Default::default()
        }
    }

    /// 创建危险按钮
    pub fn danger() -> Self {
        Self {
            danger: true,
            ..Default::default()
        }
    }

    /// 创建大尺寸按钮
    pub fn large() -> Self {
        Self {
            size: ButtonSize::Large,
            ..Default::default()
        }
    }

    /// 创建小尺寸按钮
    pub fn small() -> Self {
        Self {
            size: ButtonSize::Small,
            ..Default::default()
        }
    }

    /// 创建块级按钮
    pub fn block() -> Self {
        Self {
            block: true,
            ..Default::default()
        }
    }

    /// 设置点击事件处理器
    pub fn on_click(mut self, handler: EventHandler<MouseEvent>) -> Self {
        self.onclick = Some(handler);
        self
    }

    /// 设置自定义类名
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// 设置自定义样式
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }
}

/// 为 ButtonProps 实现 Default trait
impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            button_type: ButtonType::default(),
            size: ButtonSize::default(),
            shape: ButtonShape::default(),
            html_type: ButtonHtmlType::default(),
            danger: false,
            ghost: false,
            disabled: false,
            loading: false,
            block: false,
            class: None,
            style: None,
            onclick: None,
            children: None,
        }
    }
}
