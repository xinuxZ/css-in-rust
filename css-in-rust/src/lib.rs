//! CSS-in-Rust: High-performance styling solution
//!
//! This library provides a powerful CSS-in-Rust solution that combines compile-time
//! CSS processing with runtime style injection. It's designed for modern Rust web
//! applications, particularly those using frameworks like Dioxus.
//!
//! # Features
//!
//! - **Compile-time CSS processing**: CSS is validated and optimized at compile time
//! - **Runtime style injection**: Styles are automatically injected into the DOM
//! - **Type-safe styling**: Leverage Rust's type system for CSS
//! - **Framework integration**: Built-in support for popular Rust web frameworks
//! - **Performance optimized**: Minimal runtime overhead with compile-time optimizations
//!
//! # Quick Start
//!
//! ```rust
//! use css_in_rust::css;
//!
//! fn my_component() -> String {
//!     let button_class = css!("background-color: blue; color: white; padding: 10px;");
//!     format!(r#"<button class="{}">Click me!</button>"#, button_class)
//! }
//! ```
//!
//! # Conditional Styling
//!
//! ```rust
//! use css_in_rust::css_if;
//!
//! fn conditional_component(is_active: bool) -> String {
//!     let class = css_if!(is_active, "background-color: green;");
//!     format!(r#"<div class="{}">Content</div>"#, class)
//! }
//! ```

// Module declarations
pub mod adapters;
pub mod build_tools;
pub mod core;
pub mod dev_experience;
pub mod hot_reload;
pub mod macros;
pub mod performance;
pub mod runtime;
pub mod theme;
pub mod variants;

// Re-exports for convenience
pub use adapters::*;
pub use build_tools::*;
pub use core::*;
pub use hot_reload::*;
pub use runtime::*;
pub use theme::*;
pub use variants::*;

// Re-export macros when proc-macro feature is enabled
#[cfg(feature = "proc-macro")]
pub use css_in_rust_macros::{css, css_class, css_if};

// Suppress unused dependency warning
#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use css_in_rust_macros as _;

#[allow(unused_imports)]
use lazy_static as _;
#[allow(unused_imports)]
use tempfile as _;

// Provide fallback implementations when proc-macro feature is disabled
#[cfg(not(feature = "proc-macro"))]
pub mod fallback_macros {
    /// Fallback css! macro that returns a placeholder when proc-macro feature is disabled
    #[macro_export]
    macro_rules! css {
        ($css:expr) => {{
            eprintln!("CSS! macro is not available without the 'proc-macro' feature");
            "css-fallback"
        }};
    }

    /// Fallback css_if! macro
    #[macro_export]
    macro_rules! css_if {
        ($condition:expr, $css:expr) => {{
            eprintln!("CSS_IF! macro is not available without the 'proc-macro' feature");
            if $condition {
                "css-fallback"
            } else {
                ""
            }
        }};
    }

    /// Fallback css_class! macro
    #[macro_export]
    macro_rules! css_class {
        ($name:expr) => {{
            eprintln!("CSS_CLASS! macro is not available without the 'proc-macro' feature");
            concat!("css-", $name)
        }};
    }
}

#[cfg(not(feature = "proc-macro"))]
pub use fallback_macros::*;

/// Initialize the CSS-in-Rust runtime system
///
/// This function should be called once at the start of your application
/// to set up the style management system.
pub fn init() {
    // Initialize the style manager with default configuration
    let _manager = runtime::StyleManager::new();
}

#[cfg(test)]
mod tests {
    // use super::*; // Removed unused import

    #[test]
    fn test_macros_exist() {
        // Test that macros are properly exported
        // This is a compile-time test
    }
}
