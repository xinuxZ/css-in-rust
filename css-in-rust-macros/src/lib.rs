//! CSS-in-Rust Macros
//!
//! This crate provides procedural macros for compile-time CSS processing
//! and runtime style injection with theme variable and variant support.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

// Module declarations
mod cache_management;
mod css_processing;
mod hash_utils;
mod macro_definitions;
mod theme_variants;
mod utility_conversion;
mod wasm_types;

use macro_definitions::{
    css_class_impl_internal, css_if_impl_internal, css_impl_internal, css_multi_if_impl_internal,
};

mod css_macro;
mod dioxus_macros;

/// CSS macro for compile-time CSS processing and runtime injection
/// Now supports theme variables and variant syntax
///
/// # Examples
///
/// ```rust
/// use css_in_rust_macros::css;
///
/// // Basic CSS
/// let class_name = css!("color: red; font-size: 16px;");
///
/// // Theme variables
/// let class_name = css!("color: var(--primary-color); font-size: var(--font-size-base);");
///
/// // Variant syntax
/// let class_name = css!("hover:bg-primary-500 sm:text-lg dark:text-white");
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    css_macro::css_impl(input)
}

/// Conditional CSS macro that only applies styles when condition is true
///
/// # Examples
///
/// ```rust
/// use css_in_rust_macros::css_if;
///
/// let is_active = true;
/// let class_name = css_if!(is_active, "background-color: blue;");
/// ```
#[proc_macro]
pub fn css_if(input: TokenStream) -> TokenStream {
    css_macro::css_if_impl(input)
}

/// CSS class macro for generating CSS class names
///
/// # Examples
///
/// ```rust
/// use css_in_rust_macros::css_class;
///
/// let class_name = css_class!("my-component");
/// ```
#[proc_macro]
pub fn css_class(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_class_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// Multi-condition CSS macro - applies CSS based on complex conditions
/// Supports AND (&), OR (|) operators for combining conditions
///
/// # Examples
///
/// ```rust
/// use css_in_rust_macros::css_multi_if;
///
/// let is_active = true;
/// let is_large = false;
///
/// // AND condition
/// let class_name = css_multi_if!(is_active & is_large, "background-color: blue;");
///
/// // OR condition
/// let class_name = css_multi_if!(is_active | is_large, "background-color: green;");
///
/// // Complex condition
/// let class_name = css_multi_if!((is_active & !is_large) | (is_large & !is_active), "background-color: yellow;");
/// ```
#[proc_macro]
pub fn css_multi_if(input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match css_multi_if_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

// 导出Dioxus专用宏
#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    dioxus_macros::styled_component_impl(input)
}

#[proc_macro]
pub fn styled_component_with_props(input: TokenStream) -> TokenStream {
    dioxus_macros::styled_component_with_props_impl(input)
}

#[proc_macro]
pub fn themed_style(input: TokenStream) -> TokenStream {
    dioxus_macros::themed_style_impl(input)
}
