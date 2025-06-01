//! CSS-in-Rust Macros
//!
//! This crate provides procedural macros for compile-time CSS processing
//! and runtime style injection with theme variable and variant support.

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet as LightningStyleSheet},
    targets::{Browsers, Targets},
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use syn::{Error, LitStr};

/// Global CSS cache to avoid duplicate processing and injection
static CSS_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

/// Initialize the CSS cache
fn get_css_cache() -> &'static Mutex<HashMap<String, String>> {
    CSS_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Check if CSS is already cached and return the class name if found
fn get_cached_css(css_hash: &str) -> Option<String> {
    if let Ok(cache) = get_css_cache().lock() {
        cache.get(css_hash).cloned()
    } else {
        None
    }
}

/// Cache the CSS with its hash and class name
fn cache_css(css_hash: String, class_name: String) {
    if let Ok(mut cache) = get_css_cache().lock() {
        cache.insert(css_hash, class_name);
    }
}

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
    let input2 = TokenStream2::from(input);
    match css_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
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
    let input2 = TokenStream2::from(input);
    match css_if_impl_internal(input2) {
        Ok(tokens) => TokenStream::from(tokens),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
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

/// Internal implementation of the css! macro
fn css_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
    let css_content = if let Ok(lit_str) = syn::parse2::<LitStr>(input.clone()) {
        lit_str.value()
    } else {
        parse_css_syntax(input)?
    };

    // Calculate hash for caching
    let css_hash = calculate_css_hash(&css_content);

    // Check cache first
    if let Some(cached_class) = get_cached_css(&css_hash) {
        return Ok(quote! { #cached_class });
    }

    // Generate CSS ID
    let css_id = format!("css-{}", &css_hash[..8]);

    // Process CSS with caching
    let result = process_css_with_cache(&css_content, &css_id)?;

    // Cache the result
    cache_css(css_hash, css_id.clone());

    Ok(result)
}

/// Internal implementation of the css_if! macro
fn css_if_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse the input to extract condition and CSS
    let input_str = input.to_string();

    // Simple parsing: split by comma to get condition and CSS
    let parts: Vec<&str> = input_str.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(Error::new(
            Span::call_site(),
            "css_if! macro requires condition and CSS separated by comma",
        ));
    }

    let condition_str = parts[0].trim();
    let css_str = parts[1].trim();

    // Remove quotes from CSS string if present
    let css_content = if css_str.starts_with('"') && css_str.ends_with('"') {
        &css_str[1..css_str.len() - 1]
    } else {
        css_str
    };

    // Parse condition tokens
    let condition_tokens: TokenStream2 = condition_str
        .parse()
        .map_err(|_| Error::new(Span::call_site(), "Invalid condition syntax"))?;

    // Calculate hash for caching
    let css_hash = calculate_css_hash(css_content);

    // Check cache first
    if let Some(cached_class) = get_cached_css(&css_hash) {
        return Ok(quote! {
            {
                if #condition_tokens {
                    #cached_class
                } else {
                    String::new()
                }
            }
        });
    }

    // Generate CSS ID
    let css_id = format!("css-{}", &css_hash[..8]);

    // Process CSS with caching
    let css_processing_result = process_css_with_cache(css_content, &css_id)?;

    let tokens = quote! {
        {
            if #condition_tokens {
                #css_processing_result
            } else {
                String::new()
            }
        }
    };

    Ok(tokens)
}

/// Internal implementation of the css_class! macro
fn css_class_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
    let lit_str = syn::parse2::<LitStr>(input)?;
    let class_name = lit_str.value();

    // Validate class name
    if class_name.is_empty() {
        return Err(Error::new(lit_str.span(), "Class name cannot be empty"));
    }

    // Generate a hash-based class name to ensure uniqueness
    let css_hash = calculate_css_hash(&class_name);
    let unique_class = format!("{}-{}", class_name, &css_hash[..8]);

    let tokens = quote! {
        #unique_class
    };

    Ok(tokens)
}

/// Processed CSS result with variants and media queries
#[derive(Debug, Clone)]
struct ProcessedCss {
    css: String,
    media_queries: Vec<(String, String)>,
    pseudo_selectors: Vec<(String, String)>,
}

/// Process CSS string with variant and theme variable support
fn process_css_with_variants_and_themes(css: &str) -> syn::Result<ProcessedCss> {
    let mut base_css = String::new();
    let mut media_queries = Vec::new();
    let mut pseudo_selectors = Vec::new();

    // Check if this is variant syntax (space-separated classes) or traditional CSS
    if css.contains(':') && !css.contains(';') && !css.contains('{') {
        // This looks like variant syntax: "hover:bg-primary-500 sm:text-lg dark:text-white"
        let classes: Vec<&str> = css.split_whitespace().collect();

        for class in classes {
            if let Some((variant, property)) = class.split_once(':') {
                let css_property = convert_utility_to_css(property)?;

                match variant {
                    // Pseudo-class variants
                    "hover" => pseudo_selectors.push(("hover".to_string(), css_property)),
                    "focus" => pseudo_selectors.push(("focus".to_string(), css_property)),
                    "focus-within" => {
                        pseudo_selectors.push(("focus-within".to_string(), css_property))
                    }
                    "focus-visible" => {
                        pseudo_selectors.push(("focus-visible".to_string(), css_property))
                    }
                    "active" => pseudo_selectors.push(("active".to_string(), css_property)),
                    "visited" => pseudo_selectors.push(("visited".to_string(), css_property)),
                    "target" => pseudo_selectors.push(("target".to_string(), css_property)),

                    // Form state variants
                    "disabled" => pseudo_selectors.push(("disabled".to_string(), css_property)),
                    "enabled" => pseudo_selectors.push(("enabled".to_string(), css_property)),
                    "checked" => pseudo_selectors.push(("checked".to_string(), css_property)),
                    "indeterminate" => {
                        pseudo_selectors.push(("indeterminate".to_string(), css_property))
                    }
                    "default" => pseudo_selectors.push(("default".to_string(), css_property)),
                    "required" => pseudo_selectors.push(("required".to_string(), css_property)),
                    "valid" => pseudo_selectors.push(("valid".to_string(), css_property)),
                    "invalid" => pseudo_selectors.push(("invalid".to_string(), css_property)),
                    "in-range" => pseudo_selectors.push(("in-range".to_string(), css_property)),
                    "out-of-range" => {
                        pseudo_selectors.push(("out-of-range".to_string(), css_property))
                    }
                    "placeholder-shown" => {
                        pseudo_selectors.push(("placeholder-shown".to_string(), css_property))
                    }
                    "autofill" => pseudo_selectors.push(("autofill".to_string(), css_property)),
                    "read-only" => pseudo_selectors.push(("read-only".to_string(), css_property)),

                    // Positional variants
                    "first" => pseudo_selectors.push(("first-child".to_string(), css_property)),
                    "last" => pseudo_selectors.push(("last-child".to_string(), css_property)),
                    "only" => pseudo_selectors.push(("only-child".to_string(), css_property)),
                    "odd" => pseudo_selectors.push(("nth-child(odd)".to_string(), css_property)),
                    "even" => pseudo_selectors.push(("nth-child(even)".to_string(), css_property)),
                    "first-of-type" => {
                        pseudo_selectors.push(("first-of-type".to_string(), css_property))
                    }
                    "last-of-type" => {
                        pseudo_selectors.push(("last-of-type".to_string(), css_property))
                    }
                    "only-of-type" => {
                        pseudo_selectors.push(("only-of-type".to_string(), css_property))
                    }

                    // Responsive variants (mobile-first)
                    "xs" => media_queries.push(("(min-width: 480px)".to_string(), css_property)),
                    "sm" => media_queries.push(("(min-width: 640px)".to_string(), css_property)),
                    "md" => media_queries.push(("(min-width: 768px)".to_string(), css_property)),
                    "lg" => media_queries.push(("(min-width: 1024px)".to_string(), css_property)),
                    "xl" => media_queries.push(("(min-width: 1280px)".to_string(), css_property)),
                    "2xl" => media_queries.push(("(min-width: 1536px)".to_string(), css_property)),

                    // Max-width responsive variants
                    "max-xs" => {
                        media_queries.push(("(max-width: 479px)".to_string(), css_property))
                    }
                    "max-sm" => {
                        media_queries.push(("(max-width: 639px)".to_string(), css_property))
                    }
                    "max-md" => {
                        media_queries.push(("(max-width: 767px)".to_string(), css_property))
                    }
                    "max-lg" => {
                        media_queries.push(("(max-width: 1023px)".to_string(), css_property))
                    }
                    "max-xl" => {
                        media_queries.push(("(max-width: 1279px)".to_string(), css_property))
                    }

                    // Theme variants
                    "dark" => media_queries
                        .push(("(prefers-color-scheme: dark)".to_string(), css_property)),
                    "light" => media_queries
                        .push(("(prefers-color-scheme: light)".to_string(), css_property)),

                    // Motion variants
                    "motion-safe" => media_queries.push((
                        "(prefers-reduced-motion: no-preference)".to_string(),
                        css_property,
                    )),
                    "motion-reduce" => media_queries
                        .push(("(prefers-reduced-motion: reduce)".to_string(), css_property)),

                    // Print variant
                    "print" => media_queries.push(("print".to_string(), css_property)),

                    _ => {
                        // Unknown variant, treat as regular CSS
                        base_css.push_str(&format!("{}: {}; ", variant, property));
                    }
                }
            } else {
                // No variant, convert utility class to CSS
                let css_property = convert_utility_to_css(class)?;
                base_css.push_str(&css_property);
                base_css.push(' ');
            }
        }
    } else {
        // Traditional CSS syntax, process theme variables
        base_css = process_theme_variables(css)?;
    }

    Ok(ProcessedCss {
        css: base_css.trim().to_string(),
        media_queries,
        pseudo_selectors,
    })
}

/// Convert utility class to CSS property
fn convert_utility_to_css(utility: &str) -> syn::Result<String> {
    // Handle color utilities with theme variables
    if utility.starts_with("bg-") {
        let color = &utility[3..];
        if color.starts_with("primary")
            || color.starts_with("success")
            || color.starts_with("warning")
            || color.starts_with("error")
        {
            return Ok(format!(
                "background-color: var(--{}-color);",
                color.replace('-', "-")
            ));
        }
        return Ok(format!(
            "background-color: {};",
            convert_color_value(color)?
        ));
    }

    if utility.starts_with("text-") {
        let value = &utility[5..];
        if value.starts_with("primary")
            || value.starts_with("success")
            || value.starts_with("warning")
            || value.starts_with("error")
        {
            return Ok(format!("color: var(--{}-color);", value.replace('-', "-")));
        }
        // Handle text sizes
        match value {
            "xs" => return Ok("font-size: var(--font-size-xs);".to_string()),
            "sm" => return Ok("font-size: var(--font-size-sm);".to_string()),
            "base" => return Ok("font-size: var(--font-size-base);".to_string()),
            "lg" => return Ok("font-size: var(--font-size-lg);".to_string()),
            "xl" => return Ok("font-size: var(--font-size-xl);".to_string()),
            "2xl" => return Ok("font-size: var(--font-size-2xl);".to_string()),
            _ => return Ok(format!("color: {};", convert_color_value(value)?)),
        }
    }

    // Handle spacing utilities
    if utility.starts_with("p-") || utility.starts_with("m-") {
        let property = if utility.starts_with("p-") {
            "padding"
        } else {
            "margin"
        };
        let value = &utility[2..];
        let spacing_value = convert_spacing_value(value)?;
        return Ok(format!("{}: {};", property, spacing_value));
    }

    // Handle border utilities
    if utility.starts_with("border") {
        if utility == "border" {
            return Ok("border: 1px solid var(--border-color);".to_string());
        }
        if utility.starts_with("border-") {
            let value = &utility[7..];
            return Ok(format!("border-color: {};", convert_color_value(value)?));
        }
    }

    // Handle rounded utilities
    if utility.starts_with("rounded") {
        if utility == "rounded" {
            return Ok("border-radius: var(--border-radius);".to_string());
        }
        if utility.starts_with("rounded-") {
            let value = &utility[8..];
            let radius_value = convert_radius_value(value)?;
            return Ok(format!("border-radius: {};", radius_value));
        }
    }

    // Handle display utilities
    match utility {
        "block" => return Ok("display: block;".to_string()),
        "inline-block" => return Ok("display: inline-block;".to_string()),
        "inline" => return Ok("display: inline;".to_string()),
        "flex" => return Ok("display: flex;".to_string()),
        "inline-flex" => return Ok("display: inline-flex;".to_string()),
        "grid" => return Ok("display: grid;".to_string()),
        "inline-grid" => return Ok("display: inline-grid;".to_string()),
        "hidden" => return Ok("display: none;".to_string()),
        _ => {}
    }

    // Handle position utilities
    match utility {
        "static" => return Ok("position: static;".to_string()),
        "fixed" => return Ok("position: fixed;".to_string()),
        "absolute" => return Ok("position: absolute;".to_string()),
        "relative" => return Ok("position: relative;".to_string()),
        "sticky" => return Ok("position: sticky;".to_string()),
        _ => {}
    }

    // Handle flexbox utilities
    if utility.starts_with("flex-") {
        let value = &utility[5..];
        match value {
            "row" => return Ok("flex-direction: row;".to_string()),
            "row-reverse" => return Ok("flex-direction: row-reverse;".to_string()),
            "col" => return Ok("flex-direction: column;".to_string()),
            "col-reverse" => return Ok("flex-direction: column-reverse;".to_string()),
            "wrap" => return Ok("flex-wrap: wrap;".to_string()),
            "wrap-reverse" => return Ok("flex-wrap: wrap-reverse;".to_string()),
            "nowrap" => return Ok("flex-wrap: nowrap;".to_string()),
            "1" => return Ok("flex: 1 1 0%;".to_string()),
            "auto" => return Ok("flex: 1 1 auto;".to_string()),
            "initial" => return Ok("flex: 0 1 auto;".to_string()),
            "none" => return Ok("flex: none;".to_string()),
            _ => {}
        }
    }

    // Handle justify-content utilities
    if utility.starts_with("justify-") {
        let value = &utility[8..];
        match value {
            "start" => return Ok("justify-content: flex-start;".to_string()),
            "end" => return Ok("justify-content: flex-end;".to_string()),
            "center" => return Ok("justify-content: center;".to_string()),
            "between" => return Ok("justify-content: space-between;".to_string()),
            "around" => return Ok("justify-content: space-around;".to_string()),
            "evenly" => return Ok("justify-content: space-evenly;".to_string()),
            _ => {}
        }
    }

    // Handle align-items utilities
    if utility.starts_with("items-") {
        let value = &utility[6..];
        match value {
            "start" => return Ok("align-items: flex-start;".to_string()),
            "end" => return Ok("align-items: flex-end;".to_string()),
            "center" => return Ok("align-items: center;".to_string()),
            "baseline" => return Ok("align-items: baseline;".to_string()),
            "stretch" => return Ok("align-items: stretch;".to_string()),
            _ => {}
        }
    }

    // Handle width utilities
    if utility.starts_with("w-") {
        let value = &utility[2..];
        let width_value = convert_size_value(value)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
        return Ok(format!("width: {};", width_value));
    }

    // Handle height utilities
    if utility.starts_with("h-") {
        let value = &utility[2..];
        let height_value = convert_size_value(value)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
        return Ok(format!("height: {};", height_value));
    }

    // Handle overflow utilities
    match utility {
        "overflow-auto" => return Ok("overflow: auto;".to_string()),
        "overflow-hidden" => return Ok("overflow: hidden;".to_string()),
        "overflow-visible" => return Ok("overflow: visible;".to_string()),
        "overflow-scroll" => return Ok("overflow: scroll;".to_string()),
        "overflow-x-auto" => return Ok("overflow-x: auto;".to_string()),
        "overflow-x-hidden" => return Ok("overflow-x: hidden;".to_string()),
        "overflow-y-auto" => return Ok("overflow-y: auto;".to_string()),
        "overflow-y-hidden" => return Ok("overflow-y: hidden;".to_string()),
        _ => {}
    }

    // Handle font utilities
    if utility.starts_with("text-") {
        let value = &utility[5..];
        match value {
            "xs" => return Ok("font-size: 0.75rem; line-height: 1rem;".to_string()),
            "sm" => return Ok("font-size: 0.875rem; line-height: 1.25rem;".to_string()),
            "base" => return Ok("font-size: 1rem; line-height: 1.5rem;".to_string()),
            "lg" => return Ok("font-size: 1.125rem; line-height: 1.75rem;".to_string()),
            "xl" => return Ok("font-size: 1.25rem; line-height: 1.75rem;".to_string()),
            "2xl" => return Ok("font-size: 1.5rem; line-height: 2rem;".to_string()),
            "3xl" => return Ok("font-size: 1.875rem; line-height: 2.25rem;".to_string()),
            "4xl" => return Ok("font-size: 2.25rem; line-height: 2.5rem;".to_string()),
            "5xl" => return Ok("font-size: 3rem; line-height: 1;".to_string()),
            "6xl" => return Ok("font-size: 3.75rem; line-height: 1;".to_string()),
            "7xl" => return Ok("font-size: 4.5rem; line-height: 1;".to_string()),
            "8xl" => return Ok("font-size: 6rem; line-height: 1;".to_string()),
            "9xl" => return Ok("font-size: 8rem; line-height: 1;".to_string()),
            "left" => return Ok("text-align: left;".to_string()),
            "center" => return Ok("text-align: center;".to_string()),
            "right" => return Ok("text-align: right;".to_string()),
            "justify" => return Ok("text-align: justify;".to_string()),
            _ => {
                // Handle text colors
                if let Ok(color_value) = convert_color_value(value) {
                    return Ok(format!("color: {};", color_value));
                }
            }
        }
    }

    // Handle font weight utilities
    if utility.starts_with("font-") {
        let value = &utility[5..];
        match value {
            "thin" => return Ok("font-weight: 100;".to_string()),
            "extralight" => return Ok("font-weight: 200;".to_string()),
            "light" => return Ok("font-weight: 300;".to_string()),
            "normal" => return Ok("font-weight: 400;".to_string()),
            "medium" => return Ok("font-weight: 500;".to_string()),
            "semibold" => return Ok("font-weight: 600;".to_string()),
            "bold" => return Ok("font-weight: 700;".to_string()),
            "extrabold" => return Ok("font-weight: 800;".to_string()),
            "black" => return Ok("font-weight: 900;".to_string()),
            _ => {}
        }
    }

    // Handle opacity utilities
    if utility.starts_with("opacity-") {
        let value = &utility[8..];
        let opacity_value = match value {
            "0" => "0",
            "5" => "0.05",
            "10" => "0.1",
            "20" => "0.2",
            "25" => "0.25",
            "30" => "0.3",
            "40" => "0.4",
            "50" => "0.5",
            "60" => "0.6",
            "70" => "0.7",
            "75" => "0.75",
            "80" => "0.8",
            "90" => "0.9",
            "95" => "0.95",
            "100" => "1",
            _ => value,
        };
        return Ok(format!("opacity: {};", opacity_value));
    }

    // Handle z-index utilities
    if utility.starts_with("z-") {
        let value = &utility[2..];
        let z_value = match value {
            "0" => "0",
            "10" => "10",
            "20" => "20",
            "30" => "30",
            "40" => "40",
            "50" => "50",
            "auto" => "auto",
            _ => value,
        };
        return Ok(format!("z-index: {};", z_value));
    }

    // Handle shadow utilities
    match utility {
        "shadow-sm" => return Ok("box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);".to_string()),
        "shadow" => {
            return Ok(
                "box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-md" => {
            return Ok(
                "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-lg" => {
            return Ok(
                "box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-xl" => {
            return Ok(
                "box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);"
                    .to_string(),
            )
        }
        "shadow-2xl" => return Ok("box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);".to_string()),
        "shadow-inner" => return Ok("box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);".to_string()),
        "shadow-none" => return Ok("box-shadow: none;".to_string()),
        _ => {}
    }

    // Handle cursor utilities
    if utility.starts_with("cursor-") {
        let value = &utility[7..];
        match value {
            "auto" => return Ok("cursor: auto;".to_string()),
            "default" => return Ok("cursor: default;".to_string()),
            "pointer" => return Ok("cursor: pointer;".to_string()),
            "wait" => return Ok("cursor: wait;".to_string()),
            "text" => return Ok("cursor: text;".to_string()),
            "move" => return Ok("cursor: move;".to_string()),
            "help" => return Ok("cursor: help;".to_string()),
            "not-allowed" => return Ok("cursor: not-allowed;".to_string()),
            _ => {}
        }
    }

    // Handle transition utilities
    match utility {
        "transition" => return Ok("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-none" => return Ok("transition-property: none;".to_string()),
        "transition-all" => return Ok("transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-colors" => return Ok("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-opacity" => return Ok("transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-shadow" => return Ok("transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        "transition-transform" => return Ok("transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms;".to_string()),
        _ => {}
    }

    // Fallback: treat as custom CSS property
    Ok(format!("{}: {};", utility.replace('-', "-"), "inherit"))
}

/// Convert color value to CSS
fn convert_color_value(color: &str) -> syn::Result<String> {
    match color {
        "white" => Ok("#ffffff".to_string()),
        "black" => Ok("#000000".to_string()),
        "transparent" => Ok("transparent".to_string()),
        "current" => Ok("currentColor".to_string()),
        _ => {
            // Handle numbered colors like "gray-500", "blue-600"
            if color.contains('-') {
                let parts: Vec<&str> = color.split('-').collect();
                if parts.len() == 2 {
                    return Ok(format!("var(--{}-{})", parts[0], parts[1]));
                }
            }
            Ok(format!("var(--color-{})", color))
        }
    }
}

/// Convert spacing value to CSS
fn convert_spacing_value(value: &str) -> syn::Result<String> {
    match value {
        "0" => Ok("0".to_string()),
        "1" => Ok("var(--spacing-1)".to_string()),
        "2" => Ok("var(--spacing-2)".to_string()),
        "3" => Ok("var(--spacing-3)".to_string()),
        "4" => Ok("var(--spacing-4)".to_string()),
        "5" => Ok("var(--spacing-5)".to_string()),
        "6" => Ok("var(--spacing-6)".to_string()),
        "8" => Ok("var(--spacing-8)".to_string()),
        "10" => Ok("var(--spacing-10)".to_string()),
        "12" => Ok("var(--spacing-12)".to_string()),
        "16" => Ok("var(--spacing-16)".to_string()),
        "20" => Ok("var(--spacing-20)".to_string()),
        "24" => Ok("var(--spacing-24)".to_string()),
        "32" => Ok("var(--spacing-32)".to_string()),
        "auto" => Ok("auto".to_string()),
        _ => Ok(format!("{}px", value)),
    }
}

/// Convert radius value to CSS
fn convert_radius_value(value: &str) -> syn::Result<String> {
    match value {
        "none" => Ok("0".to_string()),
        "sm" => Ok("0.125rem".to_string()),
        "md" => Ok("0.375rem".to_string()),
        "lg" => Ok("0.5rem".to_string()),
        "xl" => Ok("0.75rem".to_string()),
        "2xl" => Ok("1rem".to_string()),
        "3xl" => Ok("1.5rem".to_string()),
        "full" => Ok("9999px".to_string()),
        _ => {
            if value.ends_with("px") || value.ends_with("rem") || value.ends_with("em") {
                Ok(value.to_string())
            } else {
                Ok(format!("{}px", value))
            }
        }
    }
}

/// Convert size value to CSS
fn convert_size_value(value: &str) -> Result<String, String> {
    match value {
        "0" => Ok("0px".to_string()),
        "px" => Ok("1px".to_string()),
        "0.5" => Ok("0.125rem".to_string()),
        "1" => Ok("0.25rem".to_string()),
        "1.5" => Ok("0.375rem".to_string()),
        "2" => Ok("0.5rem".to_string()),
        "2.5" => Ok("0.625rem".to_string()),
        "3" => Ok("0.75rem".to_string()),
        "3.5" => Ok("0.875rem".to_string()),
        "4" => Ok("1rem".to_string()),
        "5" => Ok("1.25rem".to_string()),
        "6" => Ok("1.5rem".to_string()),
        "7" => Ok("1.75rem".to_string()),
        "8" => Ok("2rem".to_string()),
        "9" => Ok("2.25rem".to_string()),
        "10" => Ok("2.5rem".to_string()),
        "11" => Ok("2.75rem".to_string()),
        "12" => Ok("3rem".to_string()),
        "14" => Ok("3.5rem".to_string()),
        "16" => Ok("4rem".to_string()),
        "20" => Ok("5rem".to_string()),
        "24" => Ok("6rem".to_string()),
        "28" => Ok("7rem".to_string()),
        "32" => Ok("8rem".to_string()),
        "36" => Ok("9rem".to_string()),
        "40" => Ok("10rem".to_string()),
        "44" => Ok("11rem".to_string()),
        "48" => Ok("12rem".to_string()),
        "52" => Ok("13rem".to_string()),
        "56" => Ok("14rem".to_string()),
        "60" => Ok("15rem".to_string()),
        "64" => Ok("16rem".to_string()),
        "72" => Ok("18rem".to_string()),
        "80" => Ok("20rem".to_string()),
        "96" => Ok("24rem".to_string()),
        "auto" => Ok("auto".to_string()),
        "full" => Ok("100%".to_string()),
        "screen" => Ok("100vh".to_string()),
        "min" => Ok("min-content".to_string()),
        "max" => Ok("max-content".to_string()),
        "fit" => Ok("fit-content".to_string()),
        _ => {
            // Handle fractions like 1/2, 1/3, 2/3, etc.
            if value.contains('/') {
                let parts: Vec<&str> = value.split('/').collect();
                if parts.len() == 2 {
                    if let (Ok(numerator), Ok(denominator)) =
                        (parts[0].parse::<f32>(), parts[1].parse::<f32>())
                    {
                        let percentage = (numerator / denominator) * 100.0;
                        return Ok(format!("{}%", percentage));
                    }
                }
            }
            // Handle custom values with units
            if value.ends_with("px")
                || value.ends_with("rem")
                || value.ends_with("em")
                || value.ends_with("%")
                || value.ends_with("vh")
                || value.ends_with("vw")
            {
                Ok(value.to_string())
            } else {
                // Try to parse as number and add rem
                if let Ok(_) = value.parse::<f32>() {
                    Ok(format!("{}rem", value))
                } else {
                    Ok(value.to_string())
                }
            }
        }
    }
}

/// Process theme variables in CSS
fn process_theme_variables(css: &str) -> syn::Result<String> {
    let mut processed = css.to_string();

    // Handle nested selectors with & syntax
    if processed.contains('&') {
        processed = expand_nested_selectors(&processed);
    }

    // Validate theme variable references
    validate_theme_variables(&processed)?;

    Ok(processed)
}

/// Expand nested selectors (& syntax) into flat CSS
fn expand_nested_selectors(css: &str) -> String {
    // Simple implementation for & syntax
    // This is a basic version - a full implementation would need proper CSS parsing
    let mut result = String::new();
    let lines: Vec<&str> = css.lines().collect();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('&') {
            // Replace & with the parent selector placeholder
            let expanded = trimmed.replace('&', ".{class_name}");
            result.push_str(&expanded);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Validate theme variable references
fn validate_theme_variables(css: &str) -> syn::Result<()> {
    // Simple validation for var() syntax without regex
    let mut chars = css.chars().peekable();
    let mut in_var = false;
    let mut var_content = String::new();

    while let Some(ch) = chars.next() {
        if ch == 'v' && chars.peek() == Some(&'a') {
            // Check for "var(" pattern
            let mut temp = String::new();
            temp.push(ch);
            for _ in 0..3 {
                if let Some(next_ch) = chars.next() {
                    temp.push(next_ch);
                }
            }
            if temp == "var(" {
                in_var = true;
                var_content.clear();
                continue;
            }
        }

        if in_var {
            if ch == ')' {
                // End of var() - validate content
                if !var_content.trim().starts_with("--") && !var_content.trim().is_empty() {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("Invalid CSS variable reference: {}", var_content.trim()),
                    ));
                }
                in_var = false;
                var_content.clear();
            } else {
                var_content.push(ch);
            }
        }
    }

    Ok(())
}

/// Process CSS string and generate runtime injection code
fn process_css_string(css: &str, span: Span) -> syn::Result<TokenStream2> {
    // Process CSS with variant and theme support
    let processed_css = process_css_with_variants_and_themes(css)?;

    // Validate and optimize CSS using lightningcss
    let optimized_css = optimize_css_with_lightningcss(&processed_css.css);

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(css);
    let css_id = format!("css-{}", &css_hash[..8]);

    // Handle media queries and pseudo selectors
    let media_queries = &processed_css.media_queries;
    let pseudo_selectors = &processed_css.pseudo_selectors;
    let media_css = process_media_queries(media_queries);
    let pseudo_css = process_pseudo_selectors(pseudo_selectors);

    let tokens = quote! {
        {
            // Use a static to ensure the CSS is only processed once
            static CSS_INJECTED: ::std::sync::OnceLock<::std::string::String> = ::std::sync::OnceLock::new();

            CSS_INJECTED.get_or_init(|| {
                let class_name = #css_id;

                // Inject CSS into document head (web target only)
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen]
                    extern "C" {
                        type Document;
                        type Element;
                        type Node;

                        #[wasm_bindgen(js_name = document, js_namespace = window)]
                        static DOCUMENT: Document;

                        #[wasm_bindgen(method, js_name = createElement)]
                        fn create_element(this: &Document, tag_name: &str) -> Element;

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Node, child: &Node);

                        #[wasm_bindgen(method, setter = innerHTML)]
                        fn set_inner_html(this: &Element, html: &str);

                        #[wasm_bindgen(method, getter = head)]
                        fn head(this: &Document) -> Element;
                    }

                    // Check if style element already exists
                    let style_id = format!("css-cache-{}", #css_hash);
                    if DOCUMENT.get_element_by_id(&style_id).is_none() {
                        let style_element = DOCUMENT.create_element("style");
                        style_element.set_attribute("id", &style_id).ok();

                        let mut css_rules_vec = Vec::new();

                        // Base CSS rule
                        if !#optimized_css.is_empty() {
                            css_rules_vec.push(format!(".{} {{ {} }}", class_name, #optimized_css));
                        }

                        // Add media queries
                        let media_css = #media_css;
                        if !media_css.is_empty() {
                            let media_with_class = media_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(media_with_class);
                        }

                        // Add pseudo selectors
                        let pseudo_css = #pseudo_css;
                        if !pseudo_css.is_empty() {
                            let pseudo_with_class = pseudo_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(pseudo_with_class);
                        }

                        // Apply optimizations: deduplicate and compress
                        let deduplicated_rules = {
                            let mut seen = std::collections::HashSet::new();
                            let mut deduplicated = Vec::new();
                            for rule in &css_rules_vec {
                                let normalized = rule.trim();
                                if !normalized.is_empty() && seen.insert(normalized.to_string()) {
                                    deduplicated.push(rule.clone());
                                }
                            }
                            deduplicated
                        };

                        let css_rules = deduplicated_rules.join("\n")
                            .lines()
                            .map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join("")
                            .replace("; ", ";")
                            .replace(": ", ":")
                            .replace(" {", "{")
                            .replace("{ ", "{")
                            .replace(" }", "}")
                            .replace("} ", "}");

                        style_element.set_inner_html(&css_rules);
                        let head = DOCUMENT.head();
                        head.append_child(&style_element.into());
                    }
                }

                // For non-web targets, just return the class name
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // In server-side rendering or other contexts,
                    // you might want to collect CSS for later injection
                    // For now, we just return the class name
                }

                class_name.to_string()
            }).clone()
        }
    };

    Ok(tokens)
}

/// Parse CSS syntax from token stream
fn parse_css_syntax(input: TokenStream2) -> syn::Result<String> {
    use proc_macro2::{Delimiter, TokenTree};

    let mut css_parts = Vec::new();
    let mut tokens = input.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Ident(ident) => {
                css_parts.push(ident.to_string());
            }
            TokenTree::Punct(punct) => {
                css_parts.push(punct.to_string());
            }
            TokenTree::Literal(lit) => {
                css_parts.push(lit.to_string());
            }
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::Brace {
                    // This is a variable interpolation {variable}
                    let var_content = group.stream().to_string();
                    // For compile-time processing, we need to treat this as a placeholder
                    // The actual variable substitution will happen at runtime
                    css_parts.push(format!("var(--{})", var_content.replace(' ', "")));
                } else {
                    css_parts.push(group.to_string());
                }
            }
        }
    }

    let css_content = css_parts.join("");

    // Clean up the CSS content
    let cleaned = css_content
        .replace(" :", ":")
        .replace(": ", ":")
        .replace(" ;", ";")
        .replace("; ", ";")
        .trim()
        .to_string();

    Ok(cleaned)
}

/// Optimize CSS using lightningcss at compile time
fn optimize_css_with_lightningcss(css: &str) -> Result<String, String> {
    if css.trim().is_empty() {
        return Ok(String::new());
    }

    // Check if CSS contains special syntax that lightningcss can't handle
    let has_special_syntax = css.contains('&') || css.contains("var(--") || css.contains('{');

    if has_special_syntax {
        // Use simple validation for CSS with special syntax
        return validate_css_simple(css);
    }

    // For standard CSS, use lightningcss
    let wrapped_css = if !css.trim_start().starts_with('.')
        && !css.trim_start().starts_with('#')
        && !css.trim_start().starts_with('@')
    {
        format!(".temp {{ {} }}", css)
    } else {
        css.to_string()
    };

    // Try to parse with lightningcss
    let result = match LightningStyleSheet::parse(&wrapped_css, ParserOptions::default()) {
        Ok(stylesheet) => {
            // Create printer options for optimization
            let printer_options = PrinterOptions {
                minify: true,
                targets: Targets::from(Browsers::default()),
                ..Default::default()
            };

            // Generate optimized CSS
            match stylesheet.to_css(printer_options) {
                Ok(result) => {
                    // Extract the CSS properties from the wrapped result
                    let optimized = if !css.trim_start().starts_with('.')
                        && !css.trim_start().starts_with('#')
                        && !css.trim_start().starts_with('@')
                    {
                        // Remove the temporary wrapper
                        let code = result.code;
                        if let Some(start) = code.find('{') {
                            if let Some(end) = code.rfind('}') {
                                code[start + 1..end].trim().to_string()
                            } else {
                                code
                            }
                        } else {
                            code
                        }
                    } else {
                        result.code
                    };
                    Ok(optimized)
                }
                Err(_) => {
                    // Fallback to simple validation
                    validate_css_simple(css)
                }
            }
        }
        Err(_) => {
            // Fallback to simple validation
            validate_css_simple(css)
        }
    };

    result
}

/// Deduplicate CSS rules
fn deduplicate_css_rules(css_rules: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut deduplicated = Vec::new();

    for rule in css_rules {
        let normalized = rule.trim();
        if !normalized.is_empty() && seen.insert(normalized.to_string()) {
            deduplicated.push(rule.clone());
        }
    }

    deduplicated
}

/// Compress CSS by removing unnecessary whitespace
fn compress_css(css: &str) -> String {
    css.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .replace("; ", ";")
        .replace(": ", ":")
        .replace(" {", "{")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("} ", "}")
        .replace(",", ", ")
}

/// Simple CSS validation for special syntax
fn validate_css_simple(css: &str) -> Result<String, String> {
    let trimmed = css.trim();

    if trimmed.is_empty() {
        return Ok(String::new());
    }

    // Basic syntax checks
    let open_braces = trimmed.matches('{').count();
    let close_braces = trimmed.matches('}').count();

    // For CSS fragments (properties only), braces should be balanced or absent
    if open_braces != close_braces {
        return Err("Unbalanced braces in CSS".to_string());
    }

    // Check for basic CSS property syntax
    if !trimmed.contains(':') && !trimmed.contains('{') {
        return Err("Invalid CSS syntax: missing property declarations".to_string());
    }

    // Return the CSS as-is for special syntax
    Ok(trimmed.to_string())
}

/// Validate CSS syntax using lightningcss (for backward compatibility)
#[allow(dead_code)]
fn validate_css_syntax(css: &str) -> Result<(), String> {
    optimize_css_with_lightningcss(css).map(|_| ())
}

/// Process media queries into CSS rules with class name placeholder
fn process_media_queries(media_queries: &[(String, String)]) -> String {
    let mut css_rules = String::new();
    for (media, css) in media_queries {
        // Use {class_name} as placeholder that will be replaced at runtime
        css_rules.push_str(&format!(
            "@media {} {{ .{class_name} {{ {} }} }}",
            media,
            css,
            class_name = "{class_name}"
        ));
    }
    css_rules
}

/// Process pseudo selectors into CSS rules with class name placeholder
fn process_pseudo_selectors(pseudo_selectors: &[(String, String)]) -> String {
    let mut css_rules = String::new();
    for (pseudo, css) in pseudo_selectors {
        // Use {class_name} as placeholder that will be replaced at runtime
        css_rules.push_str(&format!(
            ".{class_name}:{} {{ {} }}",
            pseudo,
            css,
            class_name = "{class_name}"
        ));
    }
    css_rules
}

/// Calculate SHA-256 hash of CSS content for unique class names
fn calculate_css_hash(css: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Internal implementation of the css_multi_if! macro
fn css_multi_if_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse the input to extract condition and CSS
    let input_str = input.to_string();

    // Find the last comma to separate condition from CSS
    let comma_pos = input_str.rfind(',').ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "css_multi_if! macro requires condition and CSS separated by comma",
        )
    })?;

    let condition_str = input_str[..comma_pos].trim();
    let css_str = input_str[comma_pos + 1..].trim();

    // Remove quotes from CSS string if present
    let css_content = if css_str.starts_with('"') && css_str.ends_with('"') {
        &css_str[1..css_str.len() - 1]
    } else {
        css_str
    };

    // Parse condition tokens - support complex expressions
    let condition_tokens: TokenStream2 = condition_str
        .parse()
        .map_err(|_| Error::new(Span::call_site(), "Invalid condition syntax"))?;

    // Process CSS with variant and theme variable support
    let processed_css = process_css_with_variants_and_themes(css_content)?;

    let css_literal = processed_css.css;
    let _media_queries = processed_css.media_queries;
    let _pseudo_selectors = processed_css.pseudo_selectors;

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(css_content);
    let css_id = format!("css-multi-{}", &css_hash[..8]);
    let css_id_literal = css_id.clone();

    // 处理媒体查询和伪选择器
    let media_css = process_media_queries(&_media_queries);
    let pseudo_css = process_pseudo_selectors(&_pseudo_selectors);

    // 优化 CSS
    let optimized_css = optimize_css_with_lightningcss(&css_literal);

    let tokens = quote! {
        {
            if #condition_tokens {
                // Use a static to ensure the CSS is only processed once per condition combination
                static CSS_INJECTED: ::std::sync::OnceLock<::std::string::String> = ::std::sync::OnceLock::new();

                CSS_INJECTED.get_or_init(|| {
                    let class_name = #css_id_literal;

                    // Inject CSS into document head (web target only)
                    #[cfg(target_arch = "wasm32")]
                    {
                        use wasm_bindgen::prelude::*;

                        #[wasm_bindgen]
                        extern "C" {
                            type Document;
                            type Element;
                            type HtmlStyleElement;
                            type Node;

                            #[wasm_bindgen(js_namespace = console)]
                            fn log(s: &str);

                            #[wasm_bindgen(js_name = document, js_namespace = window)]
                            static DOCUMENT: Document;

                            #[wasm_bindgen(method, js_name = createElement)]
                            fn create_element(this: &Document, tag_name: &str) -> Element;

                            #[wasm_bindgen(method, js_name = appendChild)]
                            fn append_child(this: &Node, child: &Node);

                            #[wasm_bindgen(method, setter = innerHTML)]
                            fn set_inner_html(this: &Element, html: &str);

                            #[wasm_bindgen(method, getter = head)]
                            fn head(this: &Document) -> Element;
                        }

                        let style_element = DOCUMENT.create_element("style");
                        let mut css_rules_vec = Vec::new();

                        // Base CSS rule
                        if !#optimized_css.is_empty() {
                            css_rules_vec.push(format!(".{} {{ {} }}", class_name, #optimized_css));
                        }

                        // Add media queries
                        let media_css = #media_css;
                        if !media_css.is_empty() {
                            let media_with_class = media_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(media_with_class);
                        }

                        // Add pseudo selectors
                        let pseudo_css = #pseudo_css;
                        if !pseudo_css.is_empty() {
                            let pseudo_with_class = pseudo_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(pseudo_with_class);
                        }

                        // Apply optimizations: deduplicate and compress
                        let deduplicated_rules = {
                            let mut seen = std::collections::HashSet::new();
                            let mut deduplicated = Vec::new();
                            for rule in &css_rules_vec {
                                let normalized = rule.trim();
                                if !normalized.is_empty() && seen.insert(normalized.to_string()) {
                                    deduplicated.push(rule.clone());
                                }
                            }
                            deduplicated
                        };

                        let css_rules = deduplicated_rules.join("\n")
                            .lines()
                            .map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join("")
                            .replace("; ", ";")
                            .replace(": ", ":")
                            .replace(" {", "{")
                            .replace("{ ", "{")
                            .replace(" }", "}")
                            .replace("} ", "}");

                        style_element.set_inner_html(&css_rules);
                        let head = DOCUMENT.head();
                        head.append_child(&style_element.into());
                    }

                    class_name.to_string()
                }).clone()
            } else {
                String::new()
            }
        }
    };

    Ok(tokens)
}

/// Enhanced CSS processing with caching support
fn process_css_with_cache(css_content: &str, css_id: &str) -> syn::Result<TokenStream2> {
    let css_hash = calculate_css_hash(css_content);

    // Check cache first
    if let Some(cached_class) = get_cached_css(&css_hash) {
        return Ok(quote! { #cached_class });
    }

    // Process CSS if not cached
    let processed_css = process_css_with_variants_and_themes(css_content)?;
    let optimized_css = optimize_css_with_lightningcss(&processed_css.css);

    // 处理媒体查询和伪选择器
    let media_css = process_media_queries(&processed_css.media_queries);
    let pseudo_css = process_pseudo_selectors(&processed_css.pseudo_selectors);

    let class_name = css_id.to_string();

    let tokens = quote! {
        {
            // Use a static to ensure the CSS is only processed once
            static CSS_INJECTED: ::std::sync::OnceLock<::std::string::String> = ::std::sync::OnceLock::new();

            CSS_INJECTED.get_or_init(|| {
                let class_name = #class_name;

                // Cache the result
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // For non-WASM targets, just cache the class name
                    use std::collections::HashMap;
                    use std::sync::{Mutex, OnceLock};

                    static CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
                    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

                    if let Ok(mut cache_guard) = cache.lock() {
                        cache_guard.insert(#css_hash.to_string(), class_name.clone());
                    }
                }

                // Inject CSS into document head (web target only)
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::prelude::*;

                    #[wasm_bindgen]
                    extern "C" {
                        type Document;
                        type Element;
                        type Node;

                        #[wasm_bindgen(js_name = document, js_namespace = window)]
                        static DOCUMENT: Document;

                        #[wasm_bindgen(method, js_name = createElement)]
                        fn create_element(this: &Document, tag_name: &str) -> Element;

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Node, child: &Node);

                        #[wasm_bindgen(method, setter = innerHTML)]
                        fn set_inner_html(this: &Element, html: &str);

                        #[wasm_bindgen(method, getter = head)]
                        fn head(this: &Document) -> Element;

                        #[wasm_bindgen(method, js_name = getElementById)]
                        fn get_element_by_id(this: &Document, id: &str) -> Option<Element>;
                    }

                    // Check if style element already exists
                    let style_id = format!("css-cache-{}", #css_hash);
                    if DOCUMENT.get_element_by_id(&style_id).is_none() {
                        let style_element = DOCUMENT.create_element("style");
                        style_element.set_attribute("id", &style_id).ok();

                        let mut css_rules_vec = Vec::new();

                        // Base CSS rule
                        if !#optimized_css.is_empty() {
                            css_rules_vec.push(format!(".{} {{ {} }}", class_name, #optimized_css));
                        }

                        // Add media queries
                        let media_css = #media_css;
                        if !media_css.is_empty() {
                            let media_with_class = media_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(media_with_class);
                        }

                        // Add pseudo selectors
                        let pseudo_css = #pseudo_css;
                        if !pseudo_css.is_empty() {
                            let pseudo_with_class = pseudo_css.replace("{class_name}", &class_name);
                            css_rules_vec.push(pseudo_with_class);
                        }

                        // Apply optimizations: deduplicate and compress
                        let deduplicated_rules = {
                            let mut seen = std::collections::HashSet::new();
                            let mut deduplicated = Vec::new();
                            for rule in &css_rules_vec {
                                let normalized = rule.trim();
                                if !normalized.is_empty() && seen.insert(normalized.to_string()) {
                                    deduplicated.push(rule.clone());
                                }
                            }
                            deduplicated
                        };

                        let css_rules = deduplicated_rules.join("\n")
                            .lines()
                            .map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join("")
                            .replace("; ", ";")
                            .replace(": ", ":")
                            .replace(" {", "{")
                            .replace("{ ", "{")
                            .replace(" }", "}")
                            .replace("} ", "}");

                        style_element.set_inner_html(&css_rules);
                        let head = DOCUMENT.head();
                        head.append_child(&style_element.into());
                    }
                }

                class_name.to_string()
            }).clone()
        }
    };

    Ok(tokens)
}
