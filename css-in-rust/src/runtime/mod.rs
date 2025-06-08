//! Runtime CSS management functionality
//!
//! This module provides runtime style injection and management capabilities.

pub mod injector;
pub mod provider;

pub use injector::{InjectionError, StyleInjector};
pub use provider::{inject_style, ProviderType, StyleProvider};

/// Style manager configuration
///
/// 配置样式管理器的行为，包括缓存大小、样式去重和提供器类型。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{StyleManagerConfig, ProviderType};
///
/// // 创建默认配置
/// let default_config = StyleManagerConfig::default();
///
/// // 创建自定义配置
/// let custom_config = StyleManagerConfig {
///     max_cached_styles: 500,
///     enable_deduplication: true,
///     provider_type: ProviderType::Web,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct StyleManagerConfig {
    /// Maximum number of cached styles
    pub max_cached_styles: usize,
    /// Whether to enable style deduplication
    pub enable_deduplication: bool,
    /// Provider type for style injection
    pub provider_type: ProviderType,
}

impl Default for StyleManagerConfig {
    /// 创建默认的样式管理器配置
    ///
    /// 默认配置设置了1000个最大缓存样式，启用样式去重，并使用自动检测的提供器类型。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleManagerConfig, ProviderType};
    ///
    /// let config = StyleManagerConfig::default();
    /// assert_eq!(config.max_cached_styles, 1000);
    /// assert_eq!(config.enable_deduplication, true);
    /// assert!(matches!(config.provider_type, ProviderType::Auto));
    /// ```
    fn default() -> Self {
        Self {
            max_cached_styles: 1000,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        }
    }
}

/// Style manager for handling CSS injection and caching
///
/// 提供高级的样式管理功能，包括样式注入、缓存和去重。
/// 它封装了底层的`StyleInjector`，并添加了额外的配置选项。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{StyleManager, StyleManagerConfig, ProviderType};
///
/// // 创建默认样式管理器
/// let manager = StyleManager::new();
///
/// // 注入样式
/// let css = ".container { max-width: 1200px; margin: 0 auto; }";
/// let class_name = "container-style";
/// manager.inject_style(css, class_name).unwrap();
///
/// // 创建自定义配置的样式管理器
/// let config = StyleManagerConfig {
///     max_cached_styles: 500,
///     enable_deduplication: true,
///     provider_type: ProviderType::Web,
/// };
/// let custom_manager = StyleManager::with_config(config);
/// ```
pub struct StyleManager {
    config: StyleManagerConfig,
    injector: StyleInjector,
}

impl StyleManager {
    /// Create a new style manager with default configuration
    ///
    /// 创建一个使用默认配置的新样式管理器实例。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 使用管理器注入样式
    /// let css = ".header { height: 60px; background: #f5f5f5; }";
    /// let class_name = "header-style";
    /// manager.inject_style(css, class_name).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            config: StyleManagerConfig::default(),
            injector: StyleInjector::new(),
        }
    }

    /// Create a new style manager with custom configuration
    ///
    /// 使用自定义配置创建新的样式管理器实例。
    ///
    /// # Arguments
    ///
    /// * `config` - 自定义的样式管理器配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::{StyleManager, StyleManagerConfig, ProviderType};
    ///
    /// // 创建自定义配置
    /// let config = StyleManagerConfig {
    ///     max_cached_styles: 200,
    ///     enable_deduplication: false,
    ///     provider_type: ProviderType::Ssr,
    /// };
    ///
    /// // 使用自定义配置创建样式管理器
    /// let manager = StyleManager::with_config(config);
    /// ```
    pub fn with_config(config: StyleManagerConfig) -> Self {
        Self {
            injector: StyleInjector::new(),
            config,
        }
    }

    /// Inject a style with the given class name
    ///
    /// 将CSS样式注入到当前环境中，并与指定的类名关联。
    /// 样式管理器会根据配置进行缓存和去重。
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS样式字符串
    /// * `class_name` - 与样式关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 注入导航栏样式
    /// let nav_css = ".nav {
    ///     display: flex;
    ///     justify-content: space-between;
    ///     padding: 1rem;
    ///     background-color: #333;
    ///     color: white;
    /// }";
    /// let result = manager.inject_style(nav_css, "nav-style");
    /// assert!(result.is_ok());
    /// ```
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        self.injector.inject_style(css, class_name)
    }

    /// Remove a style by class name
    ///
    /// 通过类名移除之前注入的样式。
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 首先注入样式
    /// manager.inject_style(".modal { position: fixed; }", "modal-style").unwrap();
    ///
    /// // 然后移除样式
    /// let result = manager.remove_style("modal-style");
    /// assert!(result.is_ok());
    /// ```
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        self.injector.remove_style(class_name)
    }

    /// Clear all injected styles
    ///
    /// 移除所有通过此管理器注入的样式。
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// let manager = StyleManager::new();
    ///
    /// // 注入多个样式
    /// manager.inject_style(".btn { padding: 8px; }", "btn-style").unwrap();
    /// manager.inject_style(".card { margin: 16px; }", "card-style").unwrap();
    ///
    /// // 清除所有注入的样式
    /// let result = manager.clear_all_styles();
    /// assert!(result.is_ok());
    /// ```
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        self.injector.clear_all_styles()
    }
}

impl Default for StyleManager {
    /// 创建一个使用默认配置的新样式管理器实例
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::runtime::StyleManager;
    ///
    /// // 使用默认构造函数创建样式管理器
    /// let manager = StyleManager::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
