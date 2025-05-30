//! Framework adapters
//!
//! This module provides adapters for integrating with various Rust web frameworks.

#[cfg(feature = "dioxus")]
pub mod dioxus;

// Re-export adapter functionality
#[cfg(feature = "dioxus")]
pub use dioxus::*;

/// Trait for framework adapters
pub trait FrameworkAdapter {
    /// The component type for this framework
    type Component;

    /// Apply CSS class to a component
    fn apply_class(component: &mut Self::Component, class_name: &str);

    /// Get the current classes from a component
    fn get_classes(component: &Self::Component) -> Vec<String>;
}

/// Generic CSS application helper
pub fn apply_css_class<T: FrameworkAdapter>(component: &mut T::Component, class_name: &str) {
    T::apply_class(component, class_name);
}
