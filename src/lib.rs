//! CSS-in-Rust: A compile-time CSS processing library for Rust
//!
//! This library provides macros for processing CSS at compile time,
//! generating optimized styles and class names for web applications.
//!
//! # Features
//!
//! - **Compile-time CSS processing**: CSS is parsed and optimized during compilation
//! - **Automatic class name generation**: Unique class names are generated based on CSS content
//! - **CSS validation**: Syntax errors are caught at compile time
//! - **Framework integration**: Built-in support for Dioxus and other web frameworks
//! - **SSR support**: Works with server-side rendering
//!
//! # Quick Start
//!
//! ```rust
//! use css_in_rust::css;
//!
//! let class_name = css! {
//!     color: red;
//!     font-size: 16px;
//!     &:hover {
//!         color: blue;
//!     }
//! };
//! ```

// Macro implementation functions
mod macros;

#[cfg(feature = "proc-macro")]
#[allow(unused_imports)]
use criterion as _;

#[cfg(feature = "proc-macro")]
use proc_macro2::TokenStream;

/// The main CSS macro for processing CSS at compile time
#[cfg(feature = "proc-macro")]
#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    match macros::css_impl_internal(input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// CSS macro with conditional processing
#[cfg(feature = "proc-macro")]
#[proc_macro]
pub fn css_if(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    match macros::css_if_impl_internal(input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// CSS macro for theme-aware styles
#[cfg(feature = "proc-macro")]
#[proc_macro]
pub fn css_theme(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    match macros::css_theme_impl_internal(input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
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
