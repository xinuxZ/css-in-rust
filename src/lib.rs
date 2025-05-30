//! # CSS-in-Rust
//!
//! High-performance CSS-in-Rust solution based on stylers + lightningcss.
//!
//! ## Features
//!
//! - 🚀 High-performance CSS parsing and optimization
//! - 🔒 Type-safe CSS with compile-time validation
//! - 🎨 Theme system with CSS variables
//! - 📱 Responsive design support
//! - 🔧 Framework adapters (Dioxus, Yew, Leptos)
//!
//! ## Quick Start
//!
//! ```rust
//! use css_in_rust::css;
//!
//! let button_class = css! {
//!     ".button {
//!         background: #007bff;
//!         color: white;
//!         padding: 8px 16px;
//!         border: none;
//!         border-radius: 4px;
//!         cursor: pointer;
//!     }
//!
//!     .button:hover {
//!         background: #0056b3;
//!     }"
//! };
//! ```

pub mod core;
pub mod macros;
pub mod runtime;

#[cfg(any(feature = "dioxus"))]
pub mod adapters;

// Re-export the main macro
pub use crate::macros::css;

// Re-export core types
pub use crate::core::{CssError, Result};
pub use crate::runtime::{inject_style, StyleProvider};

#[cfg(feature = "dioxus")]
pub use crate::adapters::dioxus::DioxusStyleProvider;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the CSS-in-Rust runtime
///
/// This function should be called once at the start of your application
/// to set up the style injection system.
pub fn init() {
    runtime::init_runtime();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init() {
        // Test that init doesn't panic
        init();
    }
}
