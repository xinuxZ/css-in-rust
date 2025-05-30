//! CSS-in-Rust Procedural Macros
//!
//! This crate provides procedural macros for compile-time CSS processing
//! and runtime style injection.

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet as LightningStyleSheet},
    targets::{Browsers, Targets},
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{Error, LitStr};

/// CSS macro for compile-time CSS processing and runtime injection
///
/// # Examples
///
/// ```rust
/// use css_in_rust_macros::css;
///
/// let class_name = css!("color: red; font-size: 16px;");
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

/// Internal implementation of the css! macro
fn css_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Try to parse as a string literal first
    if let Ok(lit_str) = syn::parse2::<LitStr>(input.clone()) {
        return process_css_string(&lit_str.value(), lit_str.span());
    }

    // If not a string literal, try to parse as CSS syntax
    let css_content = parse_css_syntax(input)?;
    process_css_string(&css_content, Span::call_site())
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

    // Validate CSS syntax
    if let Err(err) = validate_css_syntax(css_content) {
        return Err(Error::new(
            Span::call_site(),
            format!("Invalid CSS syntax: {}", err),
        ));
    }

    let css_literal = css_content.to_string();

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(css_content);
    let css_id = format!("css-{}", &css_hash[..8]);
    let css_id_literal = css_id;

    Ok(quote! {
        {
            if #condition_tokens {
                // Use a static to ensure the CSS is only processed once
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
                        let css_rule = format!(".{} {{ {} }}", class_name, #css_literal);
                        style_element.set_inner_html(&css_rule);
                        let head = DOCUMENT.head();
                        head.append_child(&style_element.into());
                    }

                    class_name.to_string()
                }).clone()
            } else {
                String::new()
            }
        }
    })
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

    Ok(quote! {
        #unique_class
    })
}

/// Process CSS string and generate runtime injection code
fn process_css_string(css: &str, span: Span) -> syn::Result<TokenStream2> {
    // Validate and optimize CSS using lightningcss
    let optimized_css = match optimize_css_with_lightningcss(css) {
        Ok(optimized) => optimized,
        Err(err) => {
            return Err(Error::new(span, format!("CSS processing error: {}", err)));
        }
    };

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(&optimized_css);
    let css_id = format!("css-{}", &css_hash[..8]);
    let css_literal = optimized_css;

    Ok(quote! {
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
                    let css_rule = format!(".{} {{ {} }}", class_name, #css_literal);
                    style_element.set_inner_html(&css_rule);
                    let head = DOCUMENT.head();
                    head.append_child(&style_element.into());
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
    })
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
        return Err("CSS cannot be empty".to_string());
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

/// Simple CSS validation for special syntax
fn validate_css_simple(css: &str) -> Result<String, String> {
    let trimmed = css.trim();

    if trimmed.is_empty() {
        return Err("CSS cannot be empty".to_string());
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
fn validate_css_syntax(css: &str) -> Result<(), String> {
    optimize_css_with_lightningcss(css).map(|_| ())
}

/// Calculate SHA-256 hash of CSS content for unique class names
fn calculate_css_hash(css: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
