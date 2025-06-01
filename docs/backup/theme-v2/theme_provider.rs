//! 主题提供者组件
//!
//! 为 Dioxus 应用提供主题上下文管理，支持：
//! - 全局主题状态管理
//! - 主题切换
//! - CSS 变量注入
//! - 子组件主题访问

use super::css_variables::CssVariableInjector;
use super::design_tokens::DesignTokens;
use super::{Theme, ThemeMode};
use dioxus::prelude::*;
use std::rc::Rc;

/// 主题上下文
#[derive(Clone, Debug)]
pub struct ThemeContext {
    /// 当前主题
    pub theme: Rc<Theme>,
    /// 设计令牌
    pub design_tokens: Rc<DesignTokens>,
    /// 主题切换函数
    pub set_theme: Rc<dyn Fn(Theme)>,
    /// 模式切换函数
    pub toggle_mode: Rc<dyn Fn()>,
}

impl ThemeContext {
    /// 获取当前主题模式
    pub fn mode(&self) -> ThemeMode {
        self.theme.mode
    }

    /// 检查是否为暗色模式
    pub fn is_dark(&self) -> bool {
        matches!(self.theme.mode, ThemeMode::Dark)
    }

    /// 检查是否为亮色模式
    pub fn is_light(&self) -> bool {
        matches!(self.theme.mode, ThemeMode::Light)
    }

    /// 获取主题名称
    pub fn theme_name(&self) -> &str {
        &self.theme.name
    }

    /// 生成当前主题的 CSS 变量
    pub fn css_variables(&self) -> String {
        self.theme.generate_design_tokens_css(&self.design_tokens)
    }
}

/// 主题提供者属性
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    /// 初始主题
    #[props(default = Theme::ant_design_default())]
    pub initial_theme: Theme,
    /// 子组件
    pub children: Element,
    /// 是否自动注入 CSS 变量
    #[props(default = true)]
    pub auto_inject_css: bool,
    /// CSS 注入策略
    #[props(default)]
    pub injection_strategy: super::css_variables::InjectionStrategy,
}

/// 主题提供者组件
///
/// 为整个应用或组件树的一部分提供主题上下文。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::{ThemeProvider, Theme};
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         ThemeProvider {
///             initial_theme: Theme::ant_design_default(),
///             div {
///                 class: "app",
///                 "Hello, themed world!"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    // 主题状态
    let mut theme_state = use_signal(|| props.initial_theme.clone());

    // 设计令牌状态（根据主题模式动态更新）
    let design_tokens = use_memo(move || {
        let theme = theme_state.read();
        match theme.mode {
            ThemeMode::Light => Rc::new(DesignTokens::ant_design_light()),
            ThemeMode::Dark => Rc::new(DesignTokens::ant_design_dark()),
        }
    });

    // CSS 变量注入器
    let css_injector = use_memo(move || CssVariableInjector::new(props.injection_strategy.clone()));

    // 主题切换函数
    let set_theme = use_callback(move |new_theme: Theme| {
        theme_state.set(new_theme);
    });

    // 模式切换函数
    let toggle_mode = use_callback(move |_| {
        let mut theme = theme_state.write();
        theme.mode = match theme.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
    });

    // 创建主题上下文
    let theme_context = use_memo(move || ThemeContext {
        theme: Rc::new(theme_state.read().clone()),
        design_tokens: design_tokens(),
        set_theme: Rc::new(move |theme| set_theme.call(theme)),
        toggle_mode: Rc::new(move |_| toggle_mode.call(())),
    });

    // 自动注入 CSS 变量
    use_effect(move || {
        if props.auto_inject_css {
            let context = theme_context();
            let css = context.css_variables();
            css_injector().inject_css_variables(&css);
        }
    });

    rsx! {
        // 提供主题上下文
        use_context_provider(|| theme_context()),

        // 渲染子组件
        {props.children}
    }
}

/// 使用主题钩子
///
/// 在组件中获取当前主题上下文。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::use_theme;
///
/// #[component]
/// fn ThemedButton() -> Element {
///     let theme = use_theme();
///
///     rsx! {
///         button {
///             class: if theme.is_dark() { "btn-dark" } else { "btn-light" },
///             onclick: move |_| theme.toggle_mode.call(()),
///             "Toggle Theme"
///         }
///     }
/// }
/// ```
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

/// 主题模式切换器组件
///
/// 提供一个简单的主题模式切换按钮。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::ThemeToggle;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         div {
///             ThemeToggle {
///                 class: "theme-toggle-btn"
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeToggle(
    #[props(default = "theme-toggle".to_string())] class: String,
    #[props(default = "🌙".to_string())] dark_icon: String,
    #[props(default = "☀️".to_string())] light_icon: String,
) -> Element {
    let theme = use_theme();

    rsx! {
        button {
            class: "{class}",
            onclick: move |_| (theme.toggle_mode)(),
            title: if theme.is_dark() { "切换到亮色模式" } else { "切换到暗色模式" },
            if theme.is_dark() {
                "{light_icon}"
            } else {
                "{dark_icon}"
            }
        }
    }
}

/// 主题信息显示组件
///
/// 显示当前主题的基本信息，主要用于调试。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::ThemeInfo;
///
/// #[component]
/// fn DebugPanel() -> Element {
///     rsx! {
///         div {
///             class: "debug-panel",
///             ThemeInfo {}
///         }
///     }
/// }
/// ```
#[component]
pub fn ThemeInfo() -> Element {
    let theme = use_theme();

    rsx! {
        div {
            class: "theme-info",
            h3 { "当前主题信息" }
            p { "主题名称: {theme.theme_name()}" }
            p { "主题模式: {theme.mode():?}" }
            p { "是否暗色模式: {theme.is_dark()}" }
        }
    }
}

/// 条件主题渲染组件
///
/// 根据当前主题模式条件性地渲染不同的内容。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::ConditionalTheme;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         ConditionalTheme {
///             light: rsx! { div { "亮色模式内容" } },
///             dark: rsx! { div { "暗色模式内容" } }
///         }
///     }
/// }
/// ```
#[component]
pub fn ConditionalTheme(light: Element, dark: Element) -> Element {
    let theme = use_theme();

    if theme.is_dark() {
        dark
    } else {
        light
    }
}

/// 主题类名生成器
///
/// 根据当前主题模式生成相应的 CSS 类名。
pub struct ThemeClassBuilder {
    base_class: String,
    light_suffix: String,
    dark_suffix: String,
}

impl ThemeClassBuilder {
    /// 创建新的类名生成器
    pub fn new(base_class: impl Into<String>) -> Self {
        Self {
            base_class: base_class.into(),
            light_suffix: "light".to_string(),
            dark_suffix: "dark".to_string(),
        }
    }

    /// 设置亮色模式后缀
    pub fn light_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.light_suffix = suffix.into();
        self
    }

    /// 设置暗色模式后缀
    pub fn dark_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.dark_suffix = suffix.into();
        self
    }

    /// 生成主题相关的类名
    pub fn build(&self, theme: &ThemeContext) -> String {
        let suffix = if theme.is_dark() {
            &self.dark_suffix
        } else {
            &self.light_suffix
        };
        format!("{}-{}", self.base_class, suffix)
    }

    /// 生成包含基础类名和主题类名的完整类名
    pub fn build_with_base(&self, theme: &ThemeContext) -> String {
        format!("{} {}", self.base_class, self.build(theme))
    }
}

/// 使用主题类名钩子
///
/// 便捷地生成主题相关的 CSS 类名。
///
/// # 示例
///
/// ```rust
/// use dioxus::prelude::*;
/// use css_in_rust::theme::{use_theme_class, ThemeClassBuilder};
///
/// #[component]
/// fn ThemedCard() -> Element {
///     let card_class = use_theme_class(
///         ThemeClassBuilder::new("card")
///             .light_suffix("light")
///             .dark_suffix("dark")
///     );
///
///     rsx! {
///         div {
///             class: "{card_class}",
///             "主题化卡片"
///         }
///     }
/// }
/// ```
pub fn use_theme_class(builder: ThemeClassBuilder) -> String {
    let theme = use_theme();
    builder.build_with_base(&theme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_class_builder() {
        let builder = ThemeClassBuilder::new("button")
            .light_suffix("light")
            .dark_suffix("dark");

        // 模拟主题上下文
        let light_theme = Theme::ant_design_default();
        let dark_theme = Theme::ant_design_dark();

        let light_context = ThemeContext {
            theme: Rc::new(light_theme),
            design_tokens: Rc::new(DesignTokens::ant_design_light()),
            set_theme: Rc::new(|_| {}),
            toggle_mode: Rc::new(|| {}),
        };

        let dark_context = ThemeContext {
            theme: Rc::new(dark_theme),
            design_tokens: Rc::new(DesignTokens::ant_design_dark()),
            set_theme: Rc::new(|_| {}),
            toggle_mode: Rc::new(|| {}),
        };

        assert_eq!(builder.build(&light_context), "button-light");
        assert_eq!(builder.build(&dark_context), "button-dark");
        assert_eq!(
            builder.build_with_base(&light_context),
            "button button-light"
        );
        assert_eq!(builder.build_with_base(&dark_context), "button button-dark");
    }
}
