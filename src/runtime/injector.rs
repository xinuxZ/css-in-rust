//! Style injection implementation
//!
//! This module provides platform-specific style injection capabilities
//! for web browsers and server-side rendering.

use crate::core::{CssError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Style injection interface
///
/// Provides a unified interface for injecting styles across different platforms.
pub trait StyleInjector {
    /// Inject a style into the target environment
    fn inject_style(&self, css: &str, class_name: &str) -> Result<()>;

    /// Remove a style from the target environment
    fn remove_style(&self, class_name: &str) -> Result<()>;

    /// Clear all injected styles
    fn clear_all_styles(&self) -> Result<()>;

    /// Check if a style is currently injected
    fn is_injected(&self, class_name: &str) -> bool;

    /// Get the number of injected styles
    fn style_count(&self) -> usize;
}

/// Web-based style injector for browser environments
#[cfg(target_arch = "wasm32")]
pub struct WebStyleInjector {
    injected_styles: Arc<Mutex<HashMap<String, web_sys::HtmlStyleElement>>>,
}

#[cfg(target_arch = "wasm32")]
impl WebStyleInjector {
    /// Create a new web style injector
    pub fn new() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the document head element
    fn get_head(&self) -> Result<web_sys::Element> {
        let window = web_sys::window()
            .ok_or_else(|| CssError::injection_error("No window object available"))?;

        let document = window
            .document()
            .ok_or_else(|| CssError::injection_error("No document object available"))?;

        document
            .head()
            .ok_or_else(|| CssError::injection_error("No head element found"))
    }

    /// Create a style element with the given CSS
    fn create_style_element(
        &self,
        css: &str,
        class_name: &str,
    ) -> Result<web_sys::HtmlStyleElement> {
        let window = web_sys::window()
            .ok_or_else(|| CssError::injection_error("No window object available"))?;

        let document = window
            .document()
            .ok_or_else(|| CssError::injection_error("No document object available"))?;

        let style_element = document
            .create_element("style")
            .map_err(|_| CssError::injection_error("Failed to create style element"))?
            .dyn_into::<web_sys::HtmlStyleElement>()
            .map_err(|_| CssError::injection_error("Failed to cast to HtmlStyleElement"))?;

        // Set the CSS content
        style_element.set_text_content(Some(css));

        // Add identifying attributes
        style_element
            .set_attribute("data-css-in-rust", "true")
            .map_err(|_| CssError::injection_error("Failed to set data attribute"))?;

        style_element
            .set_attribute("data-class-name", class_name)
            .map_err(|_| CssError::injection_error("Failed to set class name attribute"))?;

        Ok(style_element)
    }
}

#[cfg(target_arch = "wasm32")]
impl StyleInjector for WebStyleInjector {
    fn inject_style(&self, css: &str, class_name: &str) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        // Check if already injected
        if styles.contains_key(class_name) {
            return Ok(());
        }

        // Create and inject the style element
        let style_element = self.create_style_element(css, class_name)?;
        let head = self.get_head()?;

        head.append_child(&style_element)
            .map_err(|_| CssError::injection_error("Failed to append style element to head"))?;

        // Store reference for later removal
        styles.insert(class_name.to_string(), style_element);

        Ok(())
    }

    fn remove_style(&self, class_name: &str) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        if let Some(style_element) = styles.remove(class_name) {
            if let Some(parent) = style_element.parent_node() {
                parent
                    .remove_child(&style_element)
                    .map_err(|_| CssError::injection_error("Failed to remove style element"))?;
            }
        }

        Ok(())
    }

    fn clear_all_styles(&self) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        for (_, style_element) in styles.drain() {
            if let Some(parent) = style_element.parent_node() {
                let _ = parent.remove_child(&style_element);
            }
        }

        Ok(())
    }

    fn is_injected(&self, class_name: &str) -> bool {
        if let Ok(styles) = self.injected_styles.lock() {
            styles.contains_key(class_name)
        } else {
            false
        }
    }

    fn style_count(&self) -> usize {
        if let Ok(styles) = self.injected_styles.lock() {
            styles.len()
        } else {
            0
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for WebStyleInjector {
    fn default() -> Self {
        Self::new()
    }
}

/// Server-side style injector for SSR environments
#[cfg(not(target_arch = "wasm32"))]
pub struct ServerStyleInjector {
    injected_styles: Arc<Mutex<HashMap<String, String>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ServerStyleInjector {
    /// Create a new server style injector
    pub fn new() -> Self {
        Self {
            injected_styles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get all injected styles as a single CSS string
    pub fn get_styles_string(&self) -> Result<String> {
        let styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        let mut css = String::new();
        for (_, style_css) in styles.iter() {
            css.push_str(style_css);
            css.push('\n');
        }

        Ok(css)
    }

    /// Get injected styles as a map of class names to CSS
    pub fn get_styles_map(&self) -> Result<HashMap<String, String>> {
        let styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        Ok(styles.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StyleInjector for ServerStyleInjector {
    fn inject_style(&self, css: &str, class_name: &str) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        styles.insert(class_name.to_string(), css.to_string());
        Ok(())
    }

    fn remove_style(&self, class_name: &str) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        styles.remove(class_name);
        Ok(())
    }

    fn clear_all_styles(&self) -> Result<()> {
        let mut styles = self
            .injected_styles
            .lock()
            .map_err(|_| CssError::injection_error("Failed to acquire styles lock"))?;

        styles.clear();
        Ok(())
    }

    fn is_injected(&self, class_name: &str) -> bool {
        if let Ok(styles) = self.injected_styles.lock() {
            styles.contains_key(class_name)
        } else {
            false
        }
    }

    fn style_count(&self) -> usize {
        if let Ok(styles) = self.injected_styles.lock() {
            styles.len()
        } else {
            0
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for ServerStyleInjector {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a platform-appropriate style injector
pub fn create_injector() -> Box<dyn StyleInjector + Send + Sync> {
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WebStyleInjector::new())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(ServerStyleInjector::new())
    }
}

/// Utility functions for style injection
pub mod utils {
    use super::*;

    /// Wrap CSS with a scoped class selector
    pub fn scope_css(css: &str, class_name: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you would parse the CSS and properly scope selectors
        if css.trim().starts_with('.') || css.trim().starts_with('#') {
            css.to_string()
        } else {
            format!(".{} {{ {} }}", class_name, css)
        }
    }

    /// Extract selectors from CSS (simplified)
    pub fn extract_selectors(css: &str) -> Vec<String> {
        // This is a very simplified implementation
        // In practice, you would use a proper CSS parser
        let mut selectors = Vec::new();

        for line in css.lines() {
            let trimmed = line.trim();
            if trimmed.contains('{') {
                if let Some(selector) = trimmed.split('{').next() {
                    selectors.push(selector.trim().to_string());
                }
            }
        }

        selectors
    }

    /// Validate CSS syntax (basic check)
    pub fn is_valid_css_syntax(css: &str) -> bool {
        // Basic validation: check for balanced braces
        let mut brace_count = 0;

        for char in css.chars() {
            match char {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }

        brace_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_injector() {
        let injector = create_injector();
        assert_eq!(injector.style_count(), 0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_server_injector() {
        let injector = ServerStyleInjector::new();

        assert_eq!(injector.style_count(), 0);
        assert!(!injector.is_injected("test"));

        let result = injector.inject_style(".test { color: red; }", "test");
        assert!(result.is_ok());
        assert_eq!(injector.style_count(), 1);
        assert!(injector.is_injected("test"));

        let styles_string = injector.get_styles_string().unwrap();
        assert!(styles_string.contains("color: red"));

        let result = injector.remove_style("test");
        assert!(result.is_ok());
        assert_eq!(injector.style_count(), 0);
        assert!(!injector.is_injected("test"));
    }

    #[test]
    fn test_utils_scope_css() {
        let css = "color: red; background: blue;";
        let scoped = utils::scope_css(css, "my-class");
        assert!(scoped.contains("my-class"));
        assert!(scoped.contains("color: red"));
    }

    #[test]
    fn test_utils_extract_selectors() {
        let css = ".button { color: red; }\n#header { background: blue; }";
        let selectors = utils::extract_selectors(css);
        assert_eq!(selectors.len(), 2);
        assert!(selectors.contains(&".button".to_string()));
        assert!(selectors.contains(&"#header".to_string()));
    }

    #[test]
    fn test_utils_css_validation() {
        assert!(utils::is_valid_css_syntax(".test { color: red; }"));
        assert!(utils::is_valid_css_syntax(".a { } .b { }"));
        assert!(!utils::is_valid_css_syntax(".test { color: red;"));
        assert!(!utils::is_valid_css_syntax(".test } color: red; {"));
    }
}
