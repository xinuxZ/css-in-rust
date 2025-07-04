//! Style provider functionality
//!
//! This module provides the main interface for style injection and management.

use crate::runtime::injector::InjectionEnvironment;
use crate::runtime::StyleInjector;
use std::sync::OnceLock;

/// Global style injector instance
static STYLE_INJECTOR: OnceLock<StyleInjector> = OnceLock::new();

/// Provider type for style injection
///
/// 定义了不同的样式注入提供器类型，用于适配不同的运行环境。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::ProviderType;
///
/// // 选择适合当前环境的提供器类型
/// let provider_type = ProviderType::Auto;
///
/// // 为Web环境指定提供器
/// let web_provider = ProviderType::Web;
///
/// // 为服务端渲染指定提供器
/// let ssr_provider = ProviderType::Ssr;
///
/// // 为测试环境指定无操作提供器
/// let test_provider = ProviderType::Noop;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    /// Automatically detect the best provider
    Auto,
    /// Web browser DOM injection
    Web,
    /// Server-side rendering
    Ssr,
    /// No-op provider for testing
    Noop,
    /// Isomorphic provider for both server and client
    Isomorphic,
}

impl ProviderType {
    /// 转换为对应的注入环境
    pub(crate) fn to_injection_environment(&self) -> InjectionEnvironment {
        match self {
            ProviderType::Auto => {
                // 自动检测环境
                #[cfg(target_arch = "wasm32")]
                return InjectionEnvironment::Browser;

                #[cfg(not(target_arch = "wasm32"))]
                return InjectionEnvironment::Server;
            }
            ProviderType::Web => InjectionEnvironment::Browser,
            ProviderType::Ssr => InjectionEnvironment::Server,
            ProviderType::Noop => InjectionEnvironment::Noop,
            ProviderType::Isomorphic => InjectionEnvironment::Isomorphic,
        }
    }
}

/// Trait for style providers
///
/// 定义了样式提供器必须实现的核心功能接口，包括样式注入、移除和清理。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{StyleProvider, InjectionError};
///
/// // 实现自定义样式提供器
/// struct CustomStyleProvider;
///
/// impl StyleProvider for CustomStyleProvider {
///     fn inject_styles(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
///         // 自定义样式注入逻辑
///         println!("注入样式: {} 到类 {}", css, class_name);
///         Ok(())
///     }
///
///     fn remove_styles(&self, class_name: &str) -> Result<(), InjectionError> {
///         // 自定义样式移除逻辑
///         println!("移除样式: {}", class_name);
///         Ok(())
///     }
///
///     fn clear_all_styles(&self) -> Result<(), InjectionError> {
///         // 自定义清理所有样式的逻辑
///         println!("清理所有样式");
///         Ok(())
///     }
/// }
/// ```
pub trait StyleProvider {
    /// Inject styles into the target environment
    ///
    /// # Arguments
    ///
    /// * `css` - 要注入的CSS字符串
    /// * `class_name` - 与CSS关联的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    fn inject_styles(
        &self,
        css: &str,
        class_name: &str,
    ) -> Result<(), crate::runtime::InjectionError>;

    /// Remove styles from the target environment
    ///
    /// # Arguments
    ///
    /// * `class_name` - 要移除的样式的类名
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    fn remove_styles(&self, class_name: &str) -> Result<(), crate::runtime::InjectionError>;

    /// Clear all styles from the target environment
    ///
    /// # Returns
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
    fn clear_all_styles(&self) -> Result<(), crate::runtime::InjectionError>;
}

/// Initialize the global style system
///
/// 初始化全局样式系统，确保样式注入器已创建。在使用其他样式函数前调用此函数是一个好习惯。
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::provider;
///
/// // 初始化样式系统
/// provider::init();
///
/// // 现在可以安全地使用样式注入功能
/// let class_name = "my-class";
/// let css = "color: red; font-size: 16px;";
/// provider::inject_style(css, class_name);
/// ```
pub fn init() {
    let _ = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
}

/// Initialize the global style system with specific provider type
///
/// 使用指定的提供器类型初始化全局样式系统。
/// 这对于同构应用特别有用，可以明确指定是在服务端还是客户端环境中运行。
///
/// # Arguments
///
/// * `provider_type` - 要使用的提供器类型
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{provider, ProviderType};
///
/// // 初始化为同构模式
/// provider::init_with_provider(ProviderType::Isomorphic);
///
/// // 现在样式会同时在服务端和客户端处理
/// let class_name = "my-class";
/// let css = "color: red; font-size: 16px;";
/// provider::inject_style(css, class_name);
/// ```
pub fn init_with_provider(provider_type: ProviderType) {
    let _ = STYLE_INJECTOR.get_or_init(|| {
        let env = provider_type.to_injection_environment();
        match env {
            InjectionEnvironment::Browser => StyleInjector::new(),
            InjectionEnvironment::Server => StyleInjector::new_ssr(),
            InjectionEnvironment::Isomorphic => StyleInjector::new_isomorphic(),
            InjectionEnvironment::Noop => StyleInjector::new_noop(),
        }
    });
}

/// Inject CSS into the document
///
/// 将CSS样式注入到文档中，并与指定的类名关联。
///
/// # Arguments
///
/// * `css` - The CSS string to inject
/// * `class_name` - The class name to use for the CSS rule
///
/// # Returns
///
/// The class name that was used for the CSS rule
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::provider;
///
/// // 初始化样式系统
/// provider::init();
///
/// // 注入按钮样式
/// let button_class = "btn-primary";
/// let button_css = ".btn-primary {
///     background-color: #0066cc;
///     color: white;
///     padding: 8px 16px;
///     border-radius: 4px;
/// }";
/// let result = provider::inject_style(button_css, button_class);
///
/// assert_eq!(result, button_class);
/// ```
pub fn inject_style(css: &str, class_name: &str) -> String {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());

    // 尝试注入样式，如果失败则记录错误但仍返回类名
    if let Err(e) = injector.inject_style(css, class_name) {
        eprintln!("Failed to inject style for class '{}': {:?}", class_name, e);
    }

    class_name.to_string()
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
/// use css_in_rust::runtime::provider;
///
/// // 初始化样式系统
/// provider::init();
///
/// // 注入样式
/// let class_name = "temporary-style";
/// let css = ".temporary-style { color: blue; }";
/// provider::inject_style(css, class_name);
///
/// // 当不再需要该样式时移除它
/// let result = provider::remove_style(class_name);
/// assert!(result.is_ok());
/// ```
pub fn remove_style(class_name: &str) -> Result<(), crate::runtime::InjectionError> {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
    injector.remove_style(class_name)
}

/// Clear all injected styles
///
/// 清除所有通过样式注入器注入的样式。
///
/// # Returns
///
/// 成功时返回`Ok(())`，失败时返回包含错误信息的`InjectionError`
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::provider;
///
/// // 初始化样式系统
/// provider::init();
///
/// // 注入多个样式
/// provider::inject_style(".btn { padding: 8px; }", "btn");
/// provider::inject_style(".card { margin: 16px; }", "card");
///
/// // 清除所有注入的样式
/// let result = provider::clear_all_styles();
/// assert!(result.is_ok());
/// ```
pub fn clear_all_styles() -> Result<(), crate::runtime::InjectionError> {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
    injector.clear_all_styles()
}

/// Get the current environment of the style injector
///
/// 获取当前样式注入器的运行环境。
///
/// # Returns
///
/// 当前的注入环境
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::provider;
/// use css_in_rust::runtime::injector::InjectionEnvironment;
///
/// // 初始化样式系统
/// provider::init();
///
/// // 获取当前环境
/// let env = provider::current_environment();
/// ```
pub fn current_environment() -> InjectionEnvironment {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
    injector.environment()
}

/// Generate HTML style tags for server-side rendering
///
/// 生成包含所有收集样式的HTML样式标签，用于服务端渲染。
///
/// # Returns
///
/// 包含所有样式的HTML字符串，如果失败则返回空字符串
///
/// # Examples
///
/// ```
/// use css_in_rust::runtime::{provider, ProviderType};
///
/// // 初始化为SSR模式
/// provider::init_with_provider(ProviderType::Ssr);
///
/// // 注入一些样式
/// provider::inject_style("color: blue;", "text-blue");
/// provider::inject_style("margin: 16px;", "m-4");
///
/// // 生成HTML样式标签
/// let html = provider::generate_style_html();
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn generate_style_html() -> String {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new_ssr());
    match injector.generate_style_html() {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Failed to generate style HTML: {:?}", e);
            String::new()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn generate_style_html() -> String {
    // 在客户端环境中，这个方法不会生成任何内容
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_style() {
        let css = ".test { color: red; }";
        let class_name = "test-class";

        let result = inject_style(css, class_name);
        assert_eq!(result, class_name);
    }

    #[test]
    fn test_provider_type_equality() {
        assert_eq!(ProviderType::Auto, ProviderType::Auto);
        assert_ne!(ProviderType::Auto, ProviderType::Web);
    }

    #[test]
    fn test_provider_type_to_injection_environment() {
        assert_eq!(
            ProviderType::Web.to_injection_environment(),
            InjectionEnvironment::Browser
        );
        assert_eq!(
            ProviderType::Ssr.to_injection_environment(),
            InjectionEnvironment::Server
        );
        assert_eq!(
            ProviderType::Noop.to_injection_environment(),
            InjectionEnvironment::Noop
        );
        assert_eq!(
            ProviderType::Isomorphic.to_injection_environment(),
            InjectionEnvironment::Isomorphic
        );
    }
}
