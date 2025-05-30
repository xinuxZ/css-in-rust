//! Runtime CSS management functionality
//!
//! This module provides runtime style injection and management capabilities.

pub mod injector;
pub mod provider;

pub use injector::{InjectionError, StyleInjector};
pub use provider::{inject_style, ProviderType, StyleProvider};

/// Style manager configuration
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
    fn default() -> Self {
        Self {
            max_cached_styles: 1000,
            enable_deduplication: true,
            provider_type: ProviderType::Auto,
        }
    }
}

/// Style manager for handling CSS injection and caching
pub struct StyleManager {
    config: StyleManagerConfig,
    injector: StyleInjector,
}

impl StyleManager {
    /// Create a new style manager with default configuration
    pub fn new() -> Self {
        Self {
            config: StyleManagerConfig::default(),
            injector: StyleInjector::new(),
        }
    }

    /// Create a new style manager with custom configuration
    pub fn with_config(config: StyleManagerConfig) -> Self {
        Self {
            injector: StyleInjector::new(),
            config,
        }
    }

    /// Inject a style with the given class name
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        self.injector.inject_style(css, class_name)
    }

    /// Remove a style by class name
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        self.injector.remove_style(class_name)
    }

    /// Clear all injected styles
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        self.injector.clear_all_styles()
    }
}

impl Default for StyleManager {
    fn default() -> Self {
        Self::new()
    }
}
