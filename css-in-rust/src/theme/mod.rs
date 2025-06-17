//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持通用设计体系和动态主题切换。
//!
//! 主题系统采用分层架构设计：
//! - 核心层 (core)：提供基础的 Token 系统、CSS 生成、缓存机制等
//! - 适配层 (adapter)：连接核心层和框架层，提供通用的适配接口
//! - 系统层 (systems)：提供特定领域的系统实现，如颜色系统、排版系统等
//!
//! # Examples
//!
//! ```
//! use css_in_rust::theme::{Theme, ThemeVariant, ThemeSystem};
//! use css_in_rust::theme::core::token::definitions::ThemeVariant;
//!
//! // 初始化主题系统
//! ThemeSystem::initialize();
//!
//! // 创建自定义主题
//! let theme = Theme::new("custom-theme")
//!     .with_mode(ThemeVariant::Dark)
//!     .with_custom_variable("--primary-color", "#3366ff");
//! ```

pub mod core;
pub mod systems;
pub mod theme_types;

// Re-exports
pub use core::ThemeVariant;
pub use theme_types::Theme;

// Dioxus集成导出
#[cfg(feature = "dioxus")]
pub use dioxus::{
    use_style, use_theme, use_theme_toggle, use_themed_style, DioxusStyleInjector,
    DioxusThemeProvider, ThemeProvider,
};

/// 主题系统
///
/// 提供主题管理和样式处理功能的入口点。负责初始化和配置主题系统的各个组件。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::ThemeSystem;
///
/// // 初始化主题系统
/// ThemeSystem::initialize();
///
/// // 现在可以使用主题系统的各种功能
/// ```
pub struct ThemeSystem;

impl ThemeSystem {
    /// 初始化主题系统
    ///
    /// 设置主题系统的基础组件和默认配置。如果启用了相应的特性，
    /// 也会初始化框架特定的集成，如 Dioxus 集成。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::ThemeSystem;
    ///
    /// // 初始化主题系统
    /// ThemeSystem::initialize();
    /// ```
    pub fn initialize() {
        // 初始化主题系统的核心组件
        core::manager::ThemeManager::initialize_global();

        // 初始化默认设计令牌
        Self::initialize_default_tokens();

        // 初始化缓存系统
        core::cache::cache_manager::CacheManager::initialize_global();

        // 如果启用了Dioxus，初始化Dioxus集成
        #[cfg(feature = "dioxus")]
        dioxus::DioxusThemeIntegration::initialize();

        log::debug!("Theme system initialized");
    }

    /// 初始化默认设计令牌
    fn initialize_default_tokens() {
        // 这里可以注册默认的设计令牌，如全局颜色、间距等
        // 例如，可以注册品牌颜色、标准间距等核心设计令牌

        #[cfg(feature = "debug")]
        log::debug!("Default design tokens initialized");
    }
}
