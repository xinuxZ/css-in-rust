#[cfg(feature = "dioxus")]
pub mod examples;
#[cfg(feature = "dioxus")]
pub mod hooks;
#[cfg(feature = "dioxus")]
pub mod injector;
#[cfg(feature = "dioxus")]
pub mod provider;
#[cfg(feature = "dioxus")]
pub mod styled;

// Re-exports
#[cfg(feature = "dioxus")]
pub use examples::{CalculatorExample, CompleteExample, SsrExample, TransformerExample};
#[cfg(feature = "dioxus")]
pub use hooks::{
    use_css_var, use_media_query, use_style, use_theme, use_theme_toggle, use_themed_style,
    ThemeProvider, ThemeProviderProps,
};
#[cfg(feature = "dioxus")]
pub use injector::DioxusStyleInjector;
#[cfg(feature = "dioxus")]
pub use provider::{DioxusThemeProvider, ThemeSwitchResult};
#[cfg(feature = "dioxus")]
pub use styled::{StyledButton, StyledDiv, StyledProps, StyledSpan};

/// Dioxus主题集成模块
///
/// 提供与Dioxus框架的主题和样式集成
pub struct DioxusThemeIntegration;

impl DioxusThemeIntegration {
    /// 初始化Dioxus主题集成
    pub fn initialize() {
        #[cfg(feature = "dioxus")]
        {
            use crate::theme::core::cache::CacheManager;
            use crate::theme::core::manager::ThemeManager;
            use crate::theme::theme_types::{Theme, ThemeMode};

            // 初始化Dioxus特定的缓存
            CacheManager::initialize_global_with("dioxus-theme");

            // 准备默认的Dioxus主题
            if let Ok(manager) = ThemeManager::get_global() {
                // 如果没有当前主题，设置默认主题
                if manager.get_current_theme().is_none() {
                    let default_theme = Theme::new("dioxus-default").with_mode(ThemeMode::Light);

                    let _ = manager.set_theme(default_theme);
                }
            }

            log::debug!("Dioxus theme integration initialized");
        }
    }
}

// 当dioxus特性未启用时提供空实现
#[cfg(not(feature = "dioxus"))]
pub struct DummyThemeProvider;

#[cfg(not(feature = "dioxus"))]
impl DummyThemeProvider {
    pub fn new() -> Self {
        Self
    }
}
