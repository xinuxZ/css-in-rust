//! Style provider interface and implementations
//!
//! This module defines the StyleProvider trait and provides implementations
//! for different environments and use cases.

use crate::core::{CssError, Result};
use std::collections::HashMap;

/// Style provider interface
///
/// Defines the contract for style management systems.
pub trait StyleProvider {
    /// Inject styles into the target environment
    fn inject_styles(&self, styles: &str) -> Result<()>;

    /// Remove styles by identifier
    fn remove_styles(&self, id: &str) -> Result<()>;

    /// Clear all managed styles
    fn clear_all_styles(&self) -> Result<()>;

    /// Get information about managed styles
    fn get_style_info(&self) -> Result<StyleProviderInfo>;

    /// Check if styles are supported in the current environment
    fn is_supported(&self) -> bool;

    /// Get the provider type
    fn provider_type(&self) -> ProviderType;
}

/// Information about a style provider's state
#[derive(Debug, Clone)]
pub struct StyleProviderInfo {
    /// Provider type
    pub provider_type: ProviderType,
    /// Number of managed styles
    pub style_count: usize,
    /// Total size of managed styles in bytes
    pub total_size: usize,
    /// Whether the provider is active
    pub is_active: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of style providers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderType {
    /// Web browser DOM-based provider
    Web,
    /// Server-side rendering provider
    Server,
    /// In-memory provider for testing
    Memory,
    /// File-based provider
    File,
    /// Dioxus framework provider
    Dioxus,
    /// Custom provider implementation
    Custom(String),
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::Web => write!(f, "Web"),
            ProviderType::Server => write!(f, "Server"),
            ProviderType::Memory => write!(f, "Memory"),
            ProviderType::File => write!(f, "File"),
            ProviderType::Dioxus => write!(f, "Dioxus"),
            ProviderType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

/// Web-based style provider for browser environments
#[cfg(target_arch = "wasm32")]
pub struct WebStyleProvider {
    injector: crate::runtime::injector::WebStyleInjector,
}

#[cfg(target_arch = "wasm32")]
impl WebStyleProvider {
    /// Create a new web style provider
    pub fn new() -> Self {
        Self {
            injector: crate::runtime::injector::WebStyleInjector::new(),
        }
    }

    /// Check if the web environment is available
    pub fn is_web_available() -> bool {
        web_sys::window().is_some()
    }
}

#[cfg(target_arch = "wasm32")]
impl StyleProvider for WebStyleProvider {
    fn inject_styles(&self, styles: &str) -> Result<()> {
        // Generate a unique ID for this style block
        let id = crate::runtime::utils::generate_class_name(styles);
        self.injector.inject_style(styles, &id)
    }

    fn remove_styles(&self, id: &str) -> Result<()> {
        self.injector.remove_style(id)
    }

    fn clear_all_styles(&self) -> Result<()> {
        self.injector.clear_all_styles()
    }

    fn get_style_info(&self) -> Result<StyleProviderInfo> {
        Ok(StyleProviderInfo {
            provider_type: ProviderType::Web,
            style_count: self.injector.style_count(),
            total_size: 0, // Would need to calculate actual size
            is_active: Self::is_web_available(),
            metadata: HashMap::new(),
        })
    }

    fn is_supported(&self) -> bool {
        Self::is_web_available()
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Web
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for WebStyleProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Server-side style provider for SSR environments
#[cfg(not(target_arch = "wasm32"))]
pub struct ServerStyleProvider {
    injector: crate::runtime::injector::ServerStyleInjector,
}

#[cfg(not(target_arch = "wasm32"))]
impl ServerStyleProvider {
    /// Create a new server style provider
    pub fn new() -> Self {
        Self {
            injector: crate::runtime::injector::ServerStyleInjector::new(),
        }
    }

    /// Get all styles as a single CSS string for SSR
    pub fn get_styles_for_ssr(&self) -> Result<String> {
        self.injector.get_styles_string()
    }

    /// Get styles as a map for more granular control
    pub fn get_styles_map(&self) -> Result<HashMap<String, String>> {
        self.injector.get_styles_map()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StyleProvider for ServerStyleProvider {
    fn inject_styles(&self, styles: &str) -> Result<()> {
        let id = crate::runtime::utils::generate_class_name(styles);
        self.injector.inject_style(styles, &id)
    }

    fn remove_styles(&self, id: &str) -> Result<()> {
        self.injector.remove_style(id)
    }

    fn clear_all_styles(&self) -> Result<()> {
        self.injector.clear_all_styles()
    }

    fn get_style_info(&self) -> Result<StyleProviderInfo> {
        let styles_map = self.get_styles_map()?;
        let total_size = styles_map.values().map(|s| s.len()).sum();

        Ok(StyleProviderInfo {
            provider_type: ProviderType::Server,
            style_count: self.injector.style_count(),
            total_size,
            is_active: true,
            metadata: HashMap::new(),
        })
    }

    fn is_supported(&self) -> bool {
        true // Server-side is always supported
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Server
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for ServerStyleProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// In-memory style provider for testing and development
pub struct MemoryStyleProvider {
    styles: std::sync::Mutex<HashMap<String, String>>,
}

impl MemoryStyleProvider {
    /// Create a new memory style provider
    pub fn new() -> Self {
        Self {
            styles: std::sync::Mutex::new(HashMap::new()),
        }
    }

    /// Get all stored styles
    pub fn get_all_styles(&self) -> Result<HashMap<String, String>> {
        let styles = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;
        Ok(styles.clone())
    }

    /// Get a specific style by ID
    pub fn get_style(&self, id: &str) -> Result<Option<String>> {
        let styles = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;
        Ok(styles.get(id).cloned())
    }
}

impl StyleProvider for MemoryStyleProvider {
    fn inject_styles(&self, styles: &str) -> Result<()> {
        let id = crate::runtime::utils::generate_class_name(styles);
        let mut style_map = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;
        style_map.insert(id, styles.to_string());
        Ok(())
    }

    fn remove_styles(&self, id: &str) -> Result<()> {
        let mut style_map = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;
        style_map.remove(id);
        Ok(())
    }

    fn clear_all_styles(&self) -> Result<()> {
        let mut style_map = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;
        style_map.clear();
        Ok(())
    }

    fn get_style_info(&self) -> Result<StyleProviderInfo> {
        let style_map = self
            .styles
            .lock()
            .map_err(|_| CssError::runtime_error("Failed to acquire styles lock"))?;

        let total_size = style_map.values().map(|s| s.len()).sum();

        Ok(StyleProviderInfo {
            provider_type: ProviderType::Memory,
            style_count: style_map.len(),
            total_size,
            is_active: true,
            metadata: HashMap::new(),
        })
    }

    fn is_supported(&self) -> bool {
        true
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Memory
    }
}

impl Default for MemoryStyleProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a platform-appropriate style provider
pub fn create_provider() -> Box<dyn StyleProvider + Send + Sync> {
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WebStyleProvider::new())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(ServerStyleProvider::new())
    }
}

/// Create a memory-based style provider for testing
pub fn create_memory_provider() -> MemoryStyleProvider {
    MemoryStyleProvider::new()
}

/// Style provider factory for different environments
pub struct StyleProviderFactory;

impl StyleProviderFactory {
    /// Create a provider based on the current environment
    pub fn create_default() -> Box<dyn StyleProvider + Send + Sync> {
        create_provider()
    }

    /// Create a memory provider
    pub fn create_memory() -> MemoryStyleProvider {
        create_memory_provider()
    }

    /// Create a provider of a specific type
    pub fn create_by_type(
        provider_type: ProviderType,
    ) -> Result<Box<dyn StyleProvider + Send + Sync>> {
        match provider_type {
            ProviderType::Memory => Ok(Box::new(MemoryStyleProvider::new())),
            #[cfg(target_arch = "wasm32")]
            ProviderType::Web => Ok(Box::new(WebStyleProvider::new())),
            #[cfg(not(target_arch = "wasm32"))]
            ProviderType::Server => Ok(Box::new(ServerStyleProvider::new())),
            _ => Err(CssError::UnsupportedFeature(format!(
                "Provider type {} is not supported in this environment",
                provider_type
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_provider() {
        let provider = MemoryStyleProvider::new();

        assert_eq!(provider.provider_type(), ProviderType::Memory);
        assert!(provider.is_supported());

        let info = provider.get_style_info().unwrap();
        assert_eq!(info.style_count, 0);
        assert_eq!(info.total_size, 0);

        let css = ".test { color: red; }";
        provider.inject_styles(css).unwrap();

        let info = provider.get_style_info().unwrap();
        assert_eq!(info.style_count, 1);
        assert!(info.total_size > 0);

        let all_styles = provider.get_all_styles().unwrap();
        assert_eq!(all_styles.len(), 1);
        assert!(all_styles.values().any(|s| s.contains("color: red")));
    }

    #[test]
    fn test_provider_factory() {
        let provider = StyleProviderFactory::create_default();
        assert!(provider.is_supported());

        let memory_provider = StyleProviderFactory::create_memory();
        assert_eq!(memory_provider.provider_type(), ProviderType::Memory);

        let provider_by_type = StyleProviderFactory::create_by_type(ProviderType::Memory).unwrap();
        assert_eq!(provider_by_type.provider_type(), ProviderType::Memory);
    }

    #[test]
    fn test_provider_type_display() {
        assert_eq!(ProviderType::Web.to_string(), "Web");
        assert_eq!(ProviderType::Server.to_string(), "Server");
        assert_eq!(ProviderType::Memory.to_string(), "Memory");
        assert_eq!(
            ProviderType::Custom("test".to_string()).to_string(),
            "Custom(test)"
        );
    }

    #[test]
    fn test_style_provider_info() {
        let info = StyleProviderInfo {
            provider_type: ProviderType::Memory,
            style_count: 5,
            total_size: 1024,
            is_active: true,
            metadata: HashMap::new(),
        };

        assert_eq!(info.style_count, 5);
        assert_eq!(info.total_size, 1024);
        assert!(info.is_active);
    }
}
