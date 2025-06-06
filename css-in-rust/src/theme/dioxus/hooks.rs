use super::provider::DioxusThemeProvider;
use crate::theme::theme_types::Theme;
use dioxus::prelude::*;
use std::sync::Arc;

/// 主题上下文
#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    /// 主题提供者
    provider: Arc<DioxusThemeProvider>,
    /// 当前主题
    current_theme: UseState<Option<Theme>>,
}

/// 使用主题钩子
///
/// 在Dioxus组件中获取和使用主题
pub fn use_theme(cx: &ScopeState) -> Option<Theme> {
    let context = use_context::<ThemeContext>(cx)?;
    context.current_theme.get().clone()
}

/// 使用主题切换钩子
///
/// 返回一个函数，用于切换主题模式
pub fn use_theme_toggle(cx: &ScopeState) -> impl Fn() + '_ {
    let context = match use_context::<ThemeContext>(cx) {
        Some(ctx) => ctx,
        None => panic!("ThemeContext未在组件树中找到"),
    };

    let theme_state = context.current_theme.clone();
    let provider = context.provider.clone();

    move || {
        if let Ok(result) = provider.toggle_theme_mode() {
            if result.success {
                if let Some(new_theme) = provider.get_theme() {
                    theme_state.set(Some(new_theme));
                }
            }
        }
    }
}

/// 使用样式钩子
///
/// 在Dioxus组件中使用CSS样式
pub fn use_style(cx: &ScopeState, css: &str) -> String {
    use crate::css;

    // 生成唯一类名
    let class_name = css!(css);

    // 返回类名
    class_name
}

/// 使用主题化样式钩子
///
/// 在Dioxus组件中使用依赖于主题的CSS样式
pub fn use_themed_style<F>(cx: &ScopeState, css_fn: F) -> String
where
    F: Fn(&Theme) -> String + 'static,
{
    let theme = use_theme(cx);
    let css = if let Some(theme) = theme {
        css_fn(&theme)
    } else {
        // 默认样式
        "".to_string()
    };

    use_style(cx, &css)
}

/// 主题提供者组件
#[derive(Props, PartialEq)]
pub struct ThemeProviderProps {
    /// 主题提供者
    #[props(into)]
    provider: Arc<DioxusThemeProvider>,

    /// 子元素
    children: Element,
}

/// 主题提供者组件
#[component]
pub fn ThemeProvider(cx: Scope<ThemeProviderProps>) -> Element {
    let provider = cx.props.provider.clone();

    // 初始化主题
    let _ = provider.initialize();

    // 创建主题状态
    let theme_state = use_state(cx, || provider.get_theme());

    // 创建主题上下文
    let theme_context = ThemeContext {
        provider: provider.clone(),
        current_theme: theme_state.clone(),
    };

    // 提供主题上下文
    use_context_provider(cx, || theme_context);

    cx.props.children.clone()
}

/// 使用CSS变量钩子
///
/// 在Dioxus组件中使用CSS变量
pub fn use_css_var(cx: &ScopeState, var_name: &str) -> String {
    format!("var(--{})", var_name)
}

/// 使用媒体查询钩子
///
/// 在Dioxus组件中使用响应式样式
pub fn use_media_query(
    cx: &ScopeState,
    query: &str,
    matches_css: &str,
    not_matches_css: &str,
) -> String {
    let media_query = format!(
        "@media {} {{ .responsive {{ {} }} }} @media not {} {{ .responsive {{ {} }} }}",
        query, matches_css, query, not_matches_css
    );

    use_style(cx, &media_query)
}
