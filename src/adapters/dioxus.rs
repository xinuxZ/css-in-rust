//! Dioxus framework adapter for css-in-rust
//!
//! This module provides integration with the Dioxus UI framework,
//! allowing styles to be injected into Dioxus components and applications.

use crate::core::Result;
use crate::{
    core::CssError,
    runtime::{ProviderType, StyleInfo, StyleProvider, StyleProviderInfo},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

/// Dioxus-specific style provider that integrates with Dioxus's rendering system
#[derive(Debug, Clone)]
pub struct DioxusStyleProvider {
    styles: Arc<Mutex<HashMap<String, StyleInfo>>>,
    #[cfg(target_arch = "wasm32")]
    document: Option<web_sys::Document>,
}

impl DioxusStyleProvider {
    /// Create a new Dioxus style provider
    pub fn new() -> Self {
        Self {
            styles: Arc::new(Mutex::new(HashMap::new())),
            #[cfg(target_arch = "wasm32")]
            document: web_sys::window().and_then(|w| w.document()),
        }
    }

    /// Create a new provider with a specific document (for testing)
    #[cfg(target_arch = "wasm32")]
    pub fn with_document(document: web_sys::Document) -> Self {
        Self {
            styles: Arc::new(Mutex::new(HashMap::new())),
            document: Some(document),
        }
    }

    /// Inject styles into the Dioxus application's head
    #[cfg(target_arch = "wasm32")]
    fn inject_into_head(&self, class_name: &str, css_content: &str) -> Result<(), CssError> {
        let document = self
            .document
            .as_ref()
            .ok_or_else(|| CssError::Runtime("Document not available".to_string()))?;

        let head = document
            .head()
            .ok_or_else(|| CssError::Runtime("Head element not found".to_string()))?;

        // Check if style element already exists
        let style_id = format!("css-in-rust-{}", class_name);
        if document.get_element_by_id(&style_id).is_some() {
            return Ok(()); // Style already injected
        }

        // Create new style element
        let style_element = document
            .create_element("style")
            .map_err(|e| CssError::Runtime(format!("Failed to create style element: {:?}", e)))?;

        style_element.set_id(&style_id);
        style_element
            .set_attribute("type", "text/css")
            .map_err(|e| CssError::Runtime(format!("Failed to set style type: {:?}", e)))?;
        style_element.set_text_content(Some(css_content));

        head.append_child(&style_element)
            .map_err(|e| CssError::Runtime(format!("Failed to append style to head: {:?}", e)))?;

        Ok(())
    }

    /// Remove styles from the Dioxus application's head
    #[cfg(target_arch = "wasm32")]
    fn remove_from_head(&self, class_name: &str) -> Result<(), CssError> {
        let document = self
            .document
            .as_ref()
            .ok_or_else(|| CssError::Runtime("Document not available".to_string()))?;

        let style_id = format!("css-in-rust-{}", class_name);
        if let Some(element) = document.get_element_by_id(&style_id) {
            element.remove();
        }

        Ok(())
    }

    /// Get the number of injected styles
    pub fn style_count(&self) -> usize {
        self.styles.lock().unwrap().len()
    }

    /// Get all injected class names
    pub fn get_class_names(&self) -> Vec<String> {
        self.styles.lock().unwrap().keys().cloned().collect()
    }

    /// Check if the provider supports the current platform
    pub fn is_supported() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window().is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            true // Server-side rendering support
        }
    }
}

impl Default for DioxusStyleProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleProvider for DioxusStyleProvider {
    fn inject_styles(&self, styles: &str) -> Result<()> {
        // For simplicity, treat the entire styles string as one injection
        let class_name = crate::runtime::generate_class_name(styles);

        let style_info = StyleInfo {
            class_name: class_name.clone(),
            css_content: styles.to_string(),
            hash: crate::runtime::generate_hash(styles),
            injected_at: std::time::SystemTime::now(),
        };

        {
            let mut style_map = self.styles.lock().unwrap();
            style_map.insert(class_name.clone(), style_info);
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.inject_to_head(&class_name, styles)
                .map_err(|e| crate::core::CssError::injection_error(e.to_string()))?;
        }

        Ok(())
    }

    fn remove_styles(&self, id: &str) -> Result<()> {
        {
            let mut styles = self.styles.lock().unwrap();
            styles.remove(id);
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.remove_from_head(id)
                .map_err(|e| crate::core::CssError::injection_error(e.to_string()))?;
        }

        Ok(())
    }

    fn clear_all_styles(&self) -> Result<()> {
        let class_names: Vec<String> = {
            let styles = self.styles.lock().unwrap();
            styles.keys().cloned().collect()
        };

        for class_name in class_names {
            self.remove_styles(&class_name)?;
        }

        Ok(())
    }

    fn get_style_info(&self) -> Result<StyleProviderInfo> {
        let styles = self.styles.lock().unwrap();
        Ok(StyleProviderInfo {
            provider_type: ProviderType::Dioxus,
            style_count: styles.len(),
            total_size: styles.values().map(|s| s.css_content.len()).sum(),
            is_active: true,
            metadata: std::collections::HashMap::new(),
        })
    }

    fn is_supported(&self) -> bool {
        Self::is_supported()
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Dioxus
    }
}

/// Dioxus hook for using CSS-in-Rust styles
#[cfg(feature = "dioxus")]
pub fn use_style(css_content: &str) -> String {
    use ::dioxus_core::prelude::*;

    let class_name = use_memo(move || crate::runtime::generate_class_name(css_content));

    use_effect(move || {
        let mut provider = DioxusStyleProvider::new();
        if let Err(e) = provider.inject_style(&class_name, css_content) {
            log::error!("Failed to inject style: {:?}", e);
        }

        // Cleanup function
        move || {
            let mut provider = DioxusStyleProvider::new();
            if let Err(e) = provider.remove_style(&class_name) {
                log::error!("Failed to remove style: {:?}", e);
            }
        }
    });

    class_name
}

/// Dioxus component for injecting global styles
#[cfg(feature = "dioxus")]
#[component]
pub fn GlobalStyle(css: String) -> Element {
    use_effect(move || {
        let class_name = format!("global-{}", crate::runtime::generate_class_name(&css));
        let mut provider = DioxusStyleProvider::new();

        if let Err(e) = provider.inject_style(&class_name, &css) {
            log::error!("Failed to inject global style: {:?}", e);
        }

        // Cleanup function
        move || {
            let mut provider = DioxusStyleProvider::new();
            if let Err(e) = provider.remove_style(&class_name) {
                log::error!("Failed to remove global style: {:?}", e);
            }
        }
    });

    rsx! {}
}

/// Utility function to extract all styles for SSR
pub fn extract_styles_for_ssr(provider: &DioxusStyleProvider) -> String {
    let styles = provider.styles.lock().unwrap();
    let mut css_output = String::new();

    for (class_name, style_info) in styles.iter() {
        css_output.push_str(&format!(
            "/* {} */\n{}\n\n",
            class_name, style_info.css_content
        ));
    }

    css_output
}

/// Create a style tag with all injected styles for SSR
pub fn create_style_tag_for_ssr(provider: &DioxusStyleProvider) -> String {
    let styles = extract_styles_for_ssr(provider);
    if styles.is_empty() {
        return String::new();
    }

    format!(
        "<style type=\"text/css\" data-css-in-rust>\n{}\n</style>",
        styles
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dioxus_provider_creation() {
        let provider = DioxusStyleProvider::new();
        assert_eq!(provider.style_count(), 0);
        assert!(provider.supports_platform());
        assert_eq!(provider.get_provider_type(), ProviderType::Dioxus);
    }

    #[test]
    fn test_dioxus_provider_style_injection() {
        let mut provider = DioxusStyleProvider::new();

        let css = ".test { color: red; }";
        let class_name = "test-class";

        let result = provider.inject_style(class_name, css);
        assert!(result.is_ok());

        assert!(provider.is_style_injected(class_name));
        assert_eq!(provider.style_count(), 1);

        let info = provider.get_style_info(class_name);
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.class_name, class_name);
        assert_eq!(info.css_content, css);
    }

    #[test]
    fn test_dioxus_provider_style_removal() {
        let mut provider = DioxusStyleProvider::new();

        let css = ".test { color: red; }";
        let class_name = "test-class";

        provider.inject_style(class_name, css).unwrap();
        assert!(provider.is_style_injected(class_name));

        let result = provider.remove_style(class_name);
        assert!(result.is_ok());
        assert!(!provider.is_style_injected(class_name));
        assert_eq!(provider.style_count(), 0);
    }

    #[test]
    fn test_dioxus_provider_clear_all() {
        let mut provider = DioxusStyleProvider::new();

        provider
            .inject_style("class1", ".class1 { color: red; }")
            .unwrap();
        provider
            .inject_style("class2", ".class2 { color: blue; }")
            .unwrap();

        assert_eq!(provider.style_count(), 2);

        let result = provider.clear_all_styles();
        assert!(result.is_ok());
        assert_eq!(provider.style_count(), 0);
    }

    #[test]
    fn test_extract_styles_for_ssr() {
        let mut provider = DioxusStyleProvider::new();

        provider
            .inject_style("class1", ".class1 { color: red; }")
            .unwrap();
        provider
            .inject_style("class2", ".class2 { color: blue; }")
            .unwrap();

        let styles = extract_styles_for_ssr(&provider);
        assert!(styles.contains(".class1 { color: red; }"));
        assert!(styles.contains(".class2 { color: blue; }"));
    }

    #[test]
    fn test_create_style_tag_for_ssr() {
        let mut provider = DioxusStyleProvider::new();

        provider
            .inject_style("class1", ".class1 { color: red; }")
            .unwrap();

        let style_tag = create_style_tag_for_ssr(&provider);
        assert!(style_tag.starts_with("<style"));
        assert!(style_tag.contains(".class1 { color: red; }"));
        assert!(style_tag.ends_with("</style>"));
    }

    #[test]
    fn test_empty_ssr_styles() {
        let provider = DioxusStyleProvider::new();

        let styles = extract_styles_for_ssr(&provider);
        assert!(styles.is_empty());

        let style_tag = create_style_tag_for_ssr(&provider);
        assert!(style_tag.is_empty());
    }

    #[test]
    fn test_invalid_inputs() {
        let mut provider = DioxusStyleProvider::new();

        // Empty class name
        let result = provider.inject_style("", ".test { color: red; }");
        assert!(result.is_err());

        // Empty CSS content
        let result = provider.inject_style("test", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_class_names() {
        let mut provider = DioxusStyleProvider::new();

        provider
            .inject_style("class1", ".class1 { color: red; }")
            .unwrap();
        provider
            .inject_style("class2", ".class2 { color: blue; }")
            .unwrap();

        let class_names = provider.get_class_names();
        assert_eq!(class_names.len(), 2);
        assert!(class_names.contains(&"class1".to_string()));
        assert!(class_names.contains(&"class2".to_string()));
    }
}
