//! Style provider functionality
//!
//! This module provides the main interface for style injection and management.

use crate::runtime::StyleInjector;
use std::sync::OnceLock;

/// Global style injector instance
static STYLE_INJECTOR: OnceLock<StyleInjector> = OnceLock::new();

/// Provider type for style injection
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
}

/// Trait for style providers
pub trait StyleProvider {
    /// Inject styles into the target environment
    fn inject_styles(
        &self,
        css: &str,
        class_name: &str,
    ) -> Result<(), crate::runtime::InjectionError>;

    /// Remove styles from the target environment
    fn remove_styles(&self, class_name: &str) -> Result<(), crate::runtime::InjectionError>;

    /// Clear all styles from the target environment
    fn clear_all_styles(&self) -> Result<(), crate::runtime::InjectionError>;
}

/// Initialize the global style system
pub fn init() {
    let _ = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
}

/// Inject CSS into the document
///
/// # Arguments
///
/// * `css` - The CSS string to inject
/// * `class_name` - The class name to use for the CSS rule
///
/// # Returns
///
/// The class name that was used for the CSS rule
pub fn inject_style(css: &str, class_name: &str) -> String {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());

    // 尝试注入样式，如果失败则记录错误但仍返回类名
    if let Err(e) = injector.inject_style(css, class_name) {
        eprintln!("Failed to inject style for class '{}': {:?}", class_name, e);
    }

    class_name.to_string()
}

/// Remove a style by class name
pub fn remove_style(class_name: &str) -> Result<(), crate::runtime::InjectionError> {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
    injector.remove_style(class_name)
}

/// Clear all injected styles
pub fn clear_all_styles() -> Result<(), crate::runtime::InjectionError> {
    let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());
    injector.clear_all_styles()
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
}
