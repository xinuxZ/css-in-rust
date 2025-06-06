//! Dioxus框架适配器
//!
//! 提供与Dioxus框架的集成。

use crate::adapters::DioxusAdapterTrait;

/// Dioxus组件表示
///
/// 这是Dioxus集成的占位符。
/// 实际实现中，这将与Dioxus的VNode系统集成。
#[derive(Debug, Clone)]
pub struct DioxusComponent {
    pub tag: String,
    pub classes: Vec<String>,
    pub attributes: std::collections::HashMap<String, String>,
}

impl DioxusComponent {
    /// Create a new Dioxus component
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            classes: Vec::new(),
            attributes: std::collections::HashMap::new(),
        }
    }

    /// Add a CSS class to this component
    pub fn add_class(&mut self, class_name: &str) {
        if !self.classes.contains(&class_name.to_string()) {
            self.classes.push(class_name.to_string());
        }
        self.update_class_attribute();
    }

    /// Remove a CSS class from this component
    pub fn remove_class(&mut self, class_name: &str) {
        self.classes.retain(|c| c != class_name);
        self.update_class_attribute();
    }

    /// Update the class attribute based on current classes
    fn update_class_attribute(&mut self) {
        if self.classes.is_empty() {
            self.attributes.remove("class");
        } else {
            self.attributes
                .insert("class".to_string(), self.classes.join(" "));
        }
    }
}

/// Dioxus框架适配器实现
pub struct DioxusAdapter;

impl DioxusAdapterTrait for DioxusAdapter {
    type Component = DioxusComponent;

    fn apply_class(component: &mut Self::Component, class_name: &str) {
        component.add_class(&class_name);
    }

    fn get_classes(component: &Self::Component) -> Vec<String> {
        component.classes.clone()
    }
}

/// Helper macro for creating Dioxus components with CSS
///
/// This would be expanded in a real implementation to work with Dioxus's rsx! macro.
#[macro_export]
macro_rules! dioxus_css {
    ($tag:expr, $css:expr) => {{
        let class_name = $crate::css!($css);
        let mut component = $crate::adapters::dioxus::DioxusComponent::new($tag);
        component.add_class(&class_name);
        component
    }};
}

/// Utility functions for Dioxus integration
pub mod utils {
    use super::*;

    /// Create a styled Dioxus component
    pub fn styled_component(tag: &str, css: &str) -> DioxusComponent {
        let class_name = crate::runtime::inject_style(css, &format!("dioxus-{}", tag));
        let mut component = DioxusComponent::new(tag);
        component.add_class(&class_name);
        component
    }

    /// Apply multiple CSS classes to a component
    pub fn apply_classes(component: &mut DioxusComponent, classes: &[&str]) {
        for class in classes {
            component.add_class(class);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dioxus_component_creation() {
        let component = DioxusComponent::new("div");
        assert_eq!(component.tag, "div");
        assert!(component.classes.is_empty());
    }

    #[test]
    fn test_add_class() {
        let mut component = DioxusComponent::new("div");
        component.add_class("test-class");

        assert_eq!(component.classes.len(), 1);
        assert_eq!(component.classes[0], "test-class");
        assert_eq!(
            component.attributes.get("class"),
            Some(&"test-class".to_string())
        );
    }

    #[test]
    fn test_remove_class() {
        let mut component = DioxusComponent::new("div");
        component.add_class("test-class");
        component.add_class("another-class");
        component.remove_class("test-class");

        assert_eq!(component.classes.len(), 1);
        assert_eq!(component.classes[0], "another-class");
    }

    #[test]
    fn test_framework_adapter() {
        let mut component = DioxusComponent::new("div");
        DioxusAdapter::apply_class(&mut component, "adapter-class");

        let classes = DioxusAdapter::get_classes(&component);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0], "adapter-class");
    }
}
