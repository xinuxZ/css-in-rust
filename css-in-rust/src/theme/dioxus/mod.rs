pub mod examples;
pub mod hooks;
pub mod injector;
pub mod provider;
pub mod styled;

// Re-exports
pub use examples::{CalculatorExample, CompleteExample, SsrExample, TransformerExample};
pub use hooks::{
    use_css_var, use_media_query, use_style, use_theme, use_theme_toggle, use_themed_style,
    ThemeProvider, ThemeProviderProps,
};
pub use injector::DioxusStyleInjector;
pub use provider::{DioxusThemeProvider, ThemeSwitchResult};
pub use styled::{StyledButton, StyledDiv, StyledProps, StyledSpan};

/// Dioxus主题集成模块
///
/// 提供与Dioxus框架的主题和样式集成
pub struct DioxusThemeIntegration;

impl DioxusThemeIntegration {
    /// 初始化Dioxus主题集成
    pub fn initialize() {
        // 初始化代码，如果需要
    }
}
