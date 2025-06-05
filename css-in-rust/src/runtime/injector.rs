//! Style injection functionality
//!
//! This module provides the core style injection capabilities for different
//! target environments (web, SSR, etc.).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Style injection error
#[derive(Debug)]
pub enum InjectionError {
    InjectionFailed(String),
    RemovalFailed(String),
    PlatformNotSupported(String),
    DomOperationFailed(String),
}

impl std::fmt::Display for InjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InjectionError::InjectionFailed(msg) => write!(f, "Style injection failed: {}", msg),
            InjectionError::RemovalFailed(msg) => write!(f, "Style removal failed: {}", msg),
            InjectionError::PlatformNotSupported(msg) => {
                write!(f, "Platform not supported: {}", msg)
            }
            InjectionError::DomOperationFailed(msg) => write!(f, "DOM operation failed: {}", msg),
        }
    }
}

impl std::error::Error for InjectionError {}

/// Style injector for managing CSS injection
pub struct StyleInjector {
    injected_styles: Arc<Mutex<HashMap<String, bool>>>,
}

impl StyleInjector {
    /// Create a new style injector
    pub fn new() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Inject a style with the given class name
    pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        // Check if already injected
        if styles.contains_key(class_name) {
            return Ok(());
        }

        // Inject based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.inject_web_style(css, class_name)?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inject_server_style(css, class_name)?;
        }

        // Mark as injected
        styles.insert(class_name.to_string(), true);
        Ok(())
    }

    /// Remove a style by class name
    pub fn remove_style(&self, class_name: &str) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        if !styles.contains_key(class_name) {
            return Ok(());
        }

        // Remove based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.remove_web_style(class_name)?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.remove_server_style(class_name)?;
        }

        // Mark as removed
        styles.remove(class_name);
        Ok(())
    }

    /// Clear all injected styles
    pub fn clear_all_styles(&self) -> Result<(), InjectionError> {
        let mut styles = self.injected_styles.lock().unwrap();

        // Clear based on platform
        #[cfg(target_arch = "wasm32")]
        {
            self.clear_web_styles()?;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.clear_server_styles()?;
        }

        // Clear tracking
        styles.clear();
        Ok(())
    }

    /// Inject style in web environment
    #[cfg(target_arch = "wasm32")]
    fn inject_web_style(&self, css: &str, class_name: &str) -> Result<(), InjectionError> {
        use web_sys::{window, Document, HtmlStyleElement};

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let style_element = document
            .create_element("style")
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to create style element".to_string())
            })?
            .dyn_into::<HtmlStyleElement>()
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to cast to HtmlStyleElement".to_string())
            })?;

        style_element.set_text_content(Some(css));
        style_element
            .set_attribute("data-css-class", class_name)
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to set attribute".to_string())
            })?;

        let head = document.head().ok_or_else(|| {
            InjectionError::DomOperationFailed("No head element available".to_string())
        })?;

        head.append_child(&style_element).map_err(|_| {
            InjectionError::DomOperationFailed("Failed to append style element".to_string())
        })?;

        Ok(())
    }

    /// Remove style in web environment
    #[cfg(target_arch = "wasm32")]
    fn remove_web_style(&self, class_name: &str) -> Result<(), InjectionError> {
        use web_sys::{window, Element};

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let selector = format!("style[data-css-class='{}']", class_name);
        if let Ok(Some(element)) = document.query_selector(&selector) {
            if let Some(parent) = element.parent_node() {
                parent.remove_child(&element).map_err(|_| {
                    InjectionError::DomOperationFailed("Failed to remove style element".to_string())
                })?;
            }
        }

        Ok(())
    }

    /// Clear all styles in web environment
    #[cfg(target_arch = "wasm32")]
    fn clear_web_styles(&self) -> Result<(), InjectionError> {
        use web_sys::window;

        let window = window().ok_or_else(|| {
            InjectionError::DomOperationFailed("No window object available".to_string())
        })?;

        let document = window.document().ok_or_else(|| {
            InjectionError::DomOperationFailed("No document object available".to_string())
        })?;

        let elements = document
            .query_selector_all("style[data-css-class]")
            .map_err(|_| {
                InjectionError::DomOperationFailed("Failed to query style elements".to_string())
            })?;

        for i in 0..elements.length() {
            if let Some(element) = elements.item(i) {
                if let Some(parent) = element.parent_node() {
                    let _ = parent.remove_child(&element);
                }
            }
        }

        Ok(())
    }

    /// Inject style in server environment (SSR)
    #[cfg(not(target_arch = "wasm32"))]
    fn inject_server_style(&self, _css: &str, _class_name: &str) -> Result<(), InjectionError> {
        // In SSR mode, we might collect styles for later injection
        // For now, this is a no-op
        Ok(())
    }

    /// Remove style in server environment (SSR)
    #[cfg(not(target_arch = "wasm32"))]
    fn remove_server_style(&self, _class_name: &str) -> Result<(), InjectionError> {
        // In SSR mode, this would remove from the collected styles
        // For now, this is a no-op
        Ok(())
    }

    /// Clear all styles in server environment (SSR)
    #[cfg(not(target_arch = "wasm32"))]
    fn clear_server_styles(&self) -> Result<(), InjectionError> {
        // In SSR mode, this would clear all collected styles
        // For now, this is a no-op
        Ok(())
    }
}

impl Default for StyleInjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_injector_creation() {
        let injector = StyleInjector::new();
        assert!(injector.injected_styles.lock().unwrap().is_empty());
    }

    #[test]
    fn test_inject_style_tracking() {
        let injector = StyleInjector::new();
        let css = ".test { color: red; }";
        let class_name = "test-class";

        // First injection should succeed
        let result = injector.inject_style(css, class_name);
        assert!(result.is_ok());

        // Second injection should also succeed (idempotent)
        let result = injector.inject_style(css, class_name);
        assert!(result.is_ok());

        // Should be tracked as injected
        assert!(injector
            .injected_styles
            .lock()
            .unwrap()
            .contains_key(class_name));
    }
}
