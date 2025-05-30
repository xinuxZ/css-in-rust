//! CSS-in-Rust Procedural Macros
//!
//! This crate provides procedural macros for compile-time CSS processing
//! and runtime style injection.

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
    // Validate CSS syntax
    if let Err(err) = validate_css_syntax(css) {
        return Err(Error::new(span, format!("Invalid CSS syntax: {}", err)));
    }

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(css);
    let css_id = format!("css-{}", &css_hash[..8]);
    let css_literal = css.to_string();

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
    // For now, convert the token stream to string
    // In a more sophisticated implementation, you might parse CSS properties individually
    let css_str = input.to_string();

    // Remove any surrounding quotes if present
    let css_content = if css_str.starts_with('"') && css_str.ends_with('"') {
        css_str[1..css_str.len() - 1].to_string()
    } else {
        css_str
    };

    Ok(css_content)
}

/// Validate CSS syntax (basic validation)
fn validate_css_syntax(css: &str) -> Result<(), String> {
    // Basic CSS validation
    if css.trim().is_empty() {
        return Err("CSS cannot be empty".to_string());
    }

    // Check for balanced braces (for CSS rules)
    let mut brace_count = 0;
    for ch in css.chars() {
        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count < 0 {
                    return Err("Unmatched closing brace".to_string());
                }
            }
            _ => {}
        }
    }

    if brace_count != 0 {
        return Err("Unmatched braces".to_string());
    }

    // Check for basic CSS property syntax (property: value;)
    if !css.contains(':') {
        return Err("CSS must contain at least one property-value pair".to_string());
    }

    Ok(())
}

/// Calculate SHA-256 hash of CSS content for unique class names
fn calculate_css_hash(css: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
