//! Runtime CSS injection and management
//!
//! This module provides the runtime components for CSS injection,
//! style management, and platform-specific implementations.

pub mod injector;
pub mod manager;
pub mod provider;
pub mod utils;

pub use injector::StyleInjector;
pub use manager::StyleManager;
pub use provider::{ProviderType, StyleProvider, StyleProviderInfo};
pub use utils::{generate_class_name, generate_hash};

// Re-export types from manager for convenience
pub use manager::{StyleInfo, StyleManagerStats};

use crate::core::{CssError, Result};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global style manager instance
static GLOBAL_MANAGER: Lazy<Mutex<StyleManager>> = Lazy::new(|| Mutex::new(StyleManager::new()));

/// Initialize the CSS-in-Rust runtime
///
/// This function sets up the global style management system.
/// It should be called once at the start of your application.
pub fn init_runtime() {
    // Initialize the global manager
    let _ = &*GLOBAL_MANAGER;

    #[cfg(target_arch = "wasm32")]
    {
        // Web-specific initialization
        init_web_runtime();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Server-side initialization
        init_server_runtime();
    }
}

/// Inject a style into the global style manager
///
/// This is the main entry point for style injection from the css! macro.
///
/// # Arguments
/// * `css` - The CSS string to inject
/// * `class_name` - The generated class name
///
/// # Returns
/// The class name that was injected
pub fn inject_style(css: &str, class_name: &str) -> &str {
    if let Ok(mut manager) = GLOBAL_MANAGER.lock() {
        if let Err(e) = manager.inject_style(css, class_name) {
            eprintln!("Failed to inject style '{}': {}", class_name, e);
        }
    } else {
        eprintln!("Failed to acquire style manager lock");
    }

    class_name
}

/// Remove a style from the global style manager
///
/// # Arguments
/// * `class_name` - The class name to remove
///
/// # Returns
/// * `Ok(())` - Style removed successfully
/// * `Err(CssError)` - Removal failed
pub fn remove_style(class_name: &str) -> Result<()> {
    let mut manager = GLOBAL_MANAGER
        .lock()
        .map_err(|_| CssError::runtime_error("Failed to acquire style manager lock"))?;

    manager.remove_style(class_name)
}

/// Clear all injected styles
///
/// This removes all styles that have been injected by the runtime.
pub fn clear_all_styles() -> Result<()> {
    let mut manager = GLOBAL_MANAGER
        .lock()
        .map_err(|_| CssError::runtime_error("Failed to acquire style manager lock"))?;

    manager.clear_all_styles()
}

/// Get information about currently injected styles
pub fn get_style_info() -> Result<manager::StyleManagerStats> {
    let manager = GLOBAL_MANAGER
        .lock()
        .map_err(|_| CssError::runtime_error("Failed to acquire style manager lock"))?;

    manager.get_stats()
}

/// Check if a style is currently injected
pub fn is_style_injected(class_name: &str) -> bool {
    if let Ok(manager) = GLOBAL_MANAGER.lock() {
        manager.is_injected(class_name)
    } else {
        false
    }
}

// StyleManagerStats is defined in manager.rs and re-exported

/// Web-specific runtime initialization
#[cfg(target_arch = "wasm32")]
fn init_web_runtime() {
    use wasm_bindgen::prelude::*;

    // Set up error handling for web
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Initialize web console logging
    web_sys::console::log_1(&"CSS-in-Rust runtime initialized for web".into());
}

/// Server-side runtime initialization
#[cfg(not(target_arch = "wasm32"))]
fn init_server_runtime() {
    // Server-side initialization
    println!("CSS-in-Rust runtime initialized for server");
}

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Whether to enable style deduplication
    pub deduplicate_styles: bool,
    /// Whether to enable style compression
    pub compress_styles: bool,
    /// Maximum number of styles to cache
    pub max_cache_size: usize,
    /// Whether to enable development mode features
    pub development_mode: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            deduplicate_styles: true,
            compress_styles: true,
            max_cache_size: 1000,
            development_mode: cfg!(debug_assertions),
        }
    }
}

/// Configure the runtime with custom settings
pub fn configure_runtime(config: RuntimeConfig) -> Result<()> {
    let mut manager = GLOBAL_MANAGER
        .lock()
        .map_err(|_| CssError::runtime_error("Failed to acquire style manager lock"))?;

    manager.configure(config);
    Ok(())
}

/// Utility functions for runtime management
pub mod utils {
    use super::*;
    use sha2::{Digest, Sha256};

    /// Generate a unique class name from CSS content
    pub fn generate_class_name(css: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        let hash = hasher.finalize();

        // Take first 8 characters of hex hash
        format!(
            "css-{:x}",
            &hash[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32)
        )
    }

    /// Generate a unique class name with prefix
    pub fn generate_class_name_with_prefix(css: &str, prefix: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        let hash = hasher.finalize();

        format!(
            "{}-{:x}",
            prefix,
            &hash[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32)
        )
    }

    /// Validate a class name format
    pub fn is_valid_class_name(class_name: &str) -> bool {
        // CSS class names must start with a letter, underscore, or hyphen
        // and can contain letters, digits, hyphens, and underscores
        if class_name.is_empty() {
            return false;
        }

        let first_char = class_name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' && first_char != '-' {
            return false;
        }

        class_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    /// Sanitize a class name to ensure it's valid
    pub fn sanitize_class_name(class_name: &str) -> String {
        let mut result = String::new();

        for (i, c) in class_name.chars().enumerate() {
            if i == 0 {
                if c.is_ascii_alphabetic() || c == '_' || c == '-' {
                    result.push(c);
                } else {
                    result.push('_');
                }
            } else if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                result.push(c);
            } else {
                result.push('_');
            }
        }

        if result.is_empty() {
            result.push_str("css");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_initialization() {
        init_runtime();
        // Should not panic
    }

    #[test]
    fn test_style_injection() {
        init_runtime();

        let css = ".test { color: red; }";
        let class_name = "test-class";

        let result = inject_style(css, class_name);
        assert_eq!(result, class_name);

        assert!(is_style_injected(class_name));
    }

    #[test]
    fn test_style_removal() {
        init_runtime();

        let css = ".test { color: blue; }";
        let class_name = "test-removal";

        inject_style(css, class_name);
        assert!(is_style_injected(class_name));

        let result = remove_style(class_name);
        assert!(result.is_ok());
        assert!(!is_style_injected(class_name));
    }

    #[test]
    fn test_style_info() {
        init_runtime();
        clear_all_styles().unwrap();

        let info = get_style_info().unwrap();
        let initial_count = info.style_count;

        inject_style(".test1 { color: red; }", "test1");
        inject_style(".test2 { color: blue; }", "test2");

        let info = get_style_info().unwrap();
        assert_eq!(info.style_count, initial_count + 2);
        assert!(info.class_names.contains(&"test1".to_string()));
        assert!(info.class_names.contains(&"test2".to_string()));
    }

    #[test]
    fn test_utils_generate_class_name() {
        let css = ".test { color: red; }";
        let class_name = utils::generate_class_name(css);

        assert!(class_name.starts_with("css-"));
        assert_eq!(class_name.len(), 12); // "css-" + 8 hex chars

        // Same CSS should generate same class name
        let class_name2 = utils::generate_class_name(css);
        assert_eq!(class_name, class_name2);
    }

    #[test]
    fn test_utils_class_name_validation() {
        assert!(utils::is_valid_class_name("valid-class"));
        assert!(utils::is_valid_class_name("_valid"));
        assert!(utils::is_valid_class_name("valid123"));

        assert!(!utils::is_valid_class_name("123invalid"));
        assert!(!utils::is_valid_class_name(""));
        assert!(!utils::is_valid_class_name(".invalid"));
    }

    #[test]
    fn test_utils_class_name_sanitization() {
        assert_eq!(utils::sanitize_class_name("valid-class"), "valid-class");
        assert_eq!(utils::sanitize_class_name("123invalid"), "_23invalid");
        assert_eq!(utils::sanitize_class_name(".invalid"), "_invalid");
        assert_eq!(utils::sanitize_class_name(""), "css");
    }
}
