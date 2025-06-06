//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持通用设计体系和动态主题切换。

pub mod core;
pub mod dioxus;
pub mod systems;
pub mod theme_types;

// 内部模块
mod tests;

// Re-exports
pub use theme_types::{Theme, ThemeMode};

// Dioxus集成导出
#[cfg(feature = "dioxus")]
pub use dioxus::{
    use_style, use_theme, use_theme_toggle, use_themed_style, DioxusStyleInjector,
    DioxusThemeProvider, ThemeProvider,
};

/// 主题系统
///
/// 提供主题管理和样式处理功能
pub struct ThemeSystem;

impl ThemeSystem {
    /// 初始化主题系统
    pub fn initialize() {
        // 初始化代码

        // 如果启用了Dioxus，初始化Dioxus集成
        #[cfg(feature = "dioxus")]
        dioxus::DioxusThemeIntegration::initialize();
    }
}
