//! CSS macro implementation
//!
//! This module provides the `css!` macro for compile-time CSS processing
//! and runtime style injection.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Error, LitStr};

/// The main CSS macro for processing CSS at compile time
///
/// # Examples
///
/// ```rust
/// use css_in_rust::css;
///
/// let class_name = css! {
///     color: red;
///     font-size: 16px;
///     &:hover {
///         color: blue;
///     }
/// };
/// ```
///
/// ```rust
/// use css_in_rust::css;
///
/// let class_name = css!(r#"
///     .button {
///         background: #007bff;
///         border: none;
///         padding: 8px 16px;
///         border-radius: 4px;
///     }
///     .button:hover {
///         background: #0056b3;
///     }
/// "#);
/// ```
/// Implementation of the css! macro
pub fn css_impl(input: TokenStream) -> syn::Result<TokenStream> {
    css_impl_internal(input)
}

/// Internal implementation of the css! macro
fn css_impl_internal(input: TokenStream) -> syn::Result<TokenStream> {
    // Try to parse as a string literal first
    if let Ok(lit_str) = syn::parse2::<LitStr>(input.clone()) {
        return process_css_string(&lit_str.value(), lit_str.span());
    }

    // If not a string literal, treat as CSS-like syntax
    let css_content = parse_css_syntax(input)?;
    process_css_string(&css_content, Span::call_site())
}

/// Process CSS string and generate the appropriate code
fn process_css_string(css: &str, span: Span) -> syn::Result<TokenStream> {
    // Validate CSS syntax at compile time
    if let Err(err) = validate_css_syntax(css) {
        return Err(Error::new(span, format!("Invalid CSS syntax: {}", err)));
    }

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(css);
    let css_id = format!("css_{}", &css_hash[..8]);

    // Generate the runtime code
    let css_literal = css;
    let css_id_literal = css_id;

    Ok(quote! {
        {
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
                        #[wasm_bindgen(js_namespace = console)]
                        fn log(s: &str);

                        type Document;
                        type Element;
                        type HTMLStyleElement;

                        #[wasm_bindgen(js_name = document)]
                        static DOCUMENT: Document;

                        #[wasm_bindgen(method, js_name = createElement)]
                        fn create_element(this: &Document, tag_name: &str) -> Element;

                        #[wasm_bindgen(method, js_name = querySelector)]
                        fn query_selector(this: &Document, selector: &str) -> Option<Element>;

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Element, child: &Element);

                        #[wasm_bindgen(method, setter = textContent)]
                        fn set_text_content(this: &Element, text: &str);
                    }

                    let style_element = DOCUMENT.create_element("style");
                    let css_content = format!(".{} {{ {} }}", class_name, #css_literal);
                    style_element.set_text_content(&css_content);

                    if let Some(head) = DOCUMENT.query_selector("head") {
                        head.append_child(&style_element);
                    }
                }

                class_name.to_string()
            }).clone()
        }
    })
}

/// Parse CSS-like syntax from token stream
fn parse_css_syntax(input: TokenStream) -> syn::Result<String> {
    let mut css = String::new();
    let tokens: Vec<_> = input.into_iter().collect();

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            proc_macro2::TokenTree::Ident(ident) => {
                css.push_str(&ident.to_string());
            }
            proc_macro2::TokenTree::Punct(punct) => {
                css.push(punct.as_char());
                if punct.as_char() == ':' {
                    css.push(' ');
                }
            }
            proc_macro2::TokenTree::Literal(lit) => {
                let lit_str = lit.to_string();
                // Remove quotes from string literals
                if lit_str.starts_with('"') && lit_str.ends_with('"') {
                    css.push_str(&lit_str[1..lit_str.len() - 1]);
                } else {
                    css.push_str(&lit_str);
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                css.push(match group.delimiter() {
                    proc_macro2::Delimiter::Brace => '{',
                    proc_macro2::Delimiter::Bracket => '[',
                    proc_macro2::Delimiter::Parenthesis => '(',
                    proc_macro2::Delimiter::None => ' ',
                });

                let inner_css = parse_css_syntax(group.stream())?;
                css.push_str(&inner_css);

                css.push(match group.delimiter() {
                    proc_macro2::Delimiter::Brace => '}',
                    proc_macro2::Delimiter::Bracket => ']',
                    proc_macro2::Delimiter::Parenthesis => ')',
                    proc_macro2::Delimiter::None => ' ',
                });
            }
        }

        // Add space between tokens for readability
        if i < tokens.len() - 1 {
            match (&tokens[i], &tokens[i + 1]) {
                (proc_macro2::TokenTree::Punct(p), _) if p.as_char() == ';' => {
                    css.push('\n');
                }
                (proc_macro2::TokenTree::Group(_), _) => {
                    css.push('\n');
                }
                _ => {
                    css.push(' ');
                }
            }
        }

        i += 1;
    }

    Ok(css)
}

/// Validate CSS syntax at compile time
fn validate_css_syntax(css: &str) -> Result<(), String> {
    // Basic validation - check for balanced braces
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for ch in css.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => escape_next = true,
            '"' | '\'' if !in_string => in_string = true,
            '"' | '\'' if in_string => in_string = false,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => brace_count -= 1,
            _ => {}
        }

        if brace_count < 0 {
            return Err("Unmatched closing brace".to_string());
        }
    }

    if brace_count != 0 {
        return Err("Unmatched opening brace".to_string());
    }

    if in_string {
        return Err("Unterminated string".to_string());
    }

    // Check for basic CSS property syntax
    let lines: Vec<&str> = css.lines().map(|line| line.trim()).collect();
    for line in lines {
        if line.is_empty()
            || line.starts_with('@')
            || line.starts_with('}')
            || line.starts_with('{')
        {
            continue;
        }

        // Check if it looks like a selector or property
        if line.contains(':') && !line.contains('{') {
            // This looks like a property declaration
            if !line.ends_with(';') && !line.ends_with('}') {
                return Err(format!(
                    "Property declaration should end with semicolon: {}",
                    line
                ));
            }
        }
    }

    Ok(())
}

/// Calculate hash for CSS content
fn calculate_css_hash(css: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// CSS macro for conditional styles
///
/// # Examples
///
/// ```rust
/// use css_in_rust::css_if;
///
/// let is_active = true;
/// let class_name = css_if!(is_active, {
///     background: blue;
///     color: white;
/// });
/// ```
/// Implementation of the css_if! macro
pub fn css_if_impl(input: TokenStream) -> syn::Result<TokenStream> {
    css_if_impl_internal(input)
}

/// Internal implementation of the css_if! macro
fn css_if_impl_internal(input: TokenStream) -> syn::Result<TokenStream> {
    let tokens: Vec<_> = input.into_iter().collect();

    if tokens.len() < 3 {
        return Err(Error::new(
            Span::call_site(),
            "css_if! requires a condition and CSS block",
        ));
    }

    // Find the comma separator
    let mut comma_pos = None;
    for (i, token) in tokens.iter().enumerate() {
        if let proc_macro2::TokenTree::Punct(punct) = token {
            if punct.as_char() == ',' {
                comma_pos = Some(i);
                break;
            }
        }
    }

    let comma_pos = comma_pos.ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "css_if! requires a comma between condition and CSS",
        )
    })?;

    // Split into condition and CSS parts
    let condition_tokens: TokenStream = tokens[..comma_pos].iter().cloned().collect();
    let css_tokens: TokenStream = tokens[comma_pos + 1..].iter().cloned().collect();

    // Parse the CSS part
    let css_content = parse_css_syntax(css_tokens)?;

    // Validate CSS
    if let Err(err) = validate_css_syntax(&css_content) {
        return Err(Error::new(
            Span::call_site(),
            format!("Invalid CSS syntax: {}", err),
        ));
    }

    let css_literal = css_content.clone();

    // Generate a unique identifier for this CSS block
    let css_hash = calculate_css_hash(&css_content);
    let css_id = format!("css_{}", &css_hash[..8]);
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

                            #[wasm_bindgen(js_name = document)]
                            static DOCUMENT: Document;

                            #[wasm_bindgen(method, js_name = createElement)]
                            fn create_element(this: &Document, tag_name: &str) -> Element;

                            #[wasm_bindgen(method, js_name = querySelector)]
                            fn query_selector(this: &Document, selector: &str) -> Option<Element>;

                            #[wasm_bindgen(method, js_name = appendChild)]
                            fn append_child(this: &Element, child: &Element);

                            #[wasm_bindgen(method, setter = textContent)]
                            fn set_text_content(this: &Element, text: &str);
                        }

                        let style_element = DOCUMENT.create_element("style");
                        let css_content = format!(".{} {{ {} }}", class_name, #css_literal);
                        style_element.set_text_content(&css_content);

                        if let Some(head) = DOCUMENT.query_selector("head") {
                            head.append_child(&style_element);
                        }
                    }

                    class_name.to_string()
                }).clone()
            } else {
                String::new()
            }
        }
    })
}

/// CSS macro for theme-aware styles
///
/// # Examples
///
/// ```rust
/// use css_in_rust::css_theme;
///
/// let class_name = css_theme!("dark", {
///     background: #333;
///     color: white;
/// }, "light", {
///     background: white;
///     color: #333;
/// });
/// ```
/// Implementation of the css_theme! macro
pub fn css_theme_impl(input: TokenStream) -> syn::Result<TokenStream> {
    css_theme_impl_internal(input)
}

/// Internal implementation of the css_theme! macro
fn css_theme_impl_internal(_input: TokenStream) -> syn::Result<TokenStream> {
    // For now, just return a placeholder implementation
    // This would be expanded in Phase 2 with proper theme support
    Ok(quote! {
        {
            // Theme support will be implemented in Phase 2
            compile_error!("css_theme! macro is not yet implemented. Use css! macro instead.")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_validate_css_syntax() {
        assert!(validate_css_syntax("color: red;").is_ok());
        assert!(validate_css_syntax(".class { color: red; }").is_ok());
        assert!(validate_css_syntax("color: red").is_err()); // Missing semicolon
        assert!(validate_css_syntax(".class { color: red;").is_err()); // Unmatched brace
    }

    #[test]
    fn test_calculate_css_hash() {
        let css1 = "color: red;";
        let css2 = "color: blue;";
        let css3 = "color: red;";

        let hash1 = calculate_css_hash(css1);
        let hash2 = calculate_css_hash(css2);
        let hash3 = calculate_css_hash(css3);

        assert_ne!(hash1, hash2);
        assert_eq!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA256 produces 64 character hex string
    }

    #[test]
    fn test_parse_css_syntax() {
        let input = quote! {
            color: red;
            font-size: 16px;
        };

        let result = parse_css_syntax(input).unwrap();
        assert!(result.contains("color"));
        assert!(result.contains("red"));
        assert!(result.contains("font-size"));
        assert!(result.contains("16px"));
    }

    #[test]
    fn test_process_css_string() {
        let css = "color: red; font-size: 16px;";
        let result = process_css_string(css, Span::call_site());

        assert!(result.is_ok());
        let tokens = result.unwrap();
        let code = tokens.to_string();

        assert!(code.contains("inject_style"));
        assert!(code.contains("color: red; font-size: 16px;"));
    }
}
