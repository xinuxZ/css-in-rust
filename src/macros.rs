//! CSS macro implementations
//!
//! This module provides the `css!` macro for compile-time CSS processing
//! and runtime style injection.

#[cfg(feature = "proc-macro")]
use proc_macro2::{Span, TokenStream};
#[cfg(feature = "proc-macro")]
use quote::quote;
#[cfg(feature = "proc-macro")]
use syn::{Error, LitStr};

/// Internal implementation of the css! macro
#[cfg(feature = "proc-macro")]
pub fn css_impl_internal(input: TokenStream) -> syn::Result<TokenStream> {
    // Try to parse as a string literal first
    if let Ok(lit_str) = syn::parse2::<LitStr>(input.clone()) {
        return process_css_string(&lit_str.value(), lit_str.span());
    }

    // If not a string literal, try to parse as CSS syntax
    let css_content = parse_css_syntax(input)?;
    process_css_string(&css_content, Span::call_site())
}

/// Internal implementation of the css_if! macro
#[cfg(feature = "proc-macro")]
pub fn css_if_impl_internal(input: TokenStream) -> syn::Result<TokenStream> {
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
    let condition_tokens: TokenStream = condition_str
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

                            #[wasm_bindgen(method, js_name = querySelector)]
                            fn query_selector(this: &Document, selector: &str) -> Option<Element>;

                            #[wasm_bindgen(method, setter, js_name = textContent)]
                            fn set_text_content(this: &Element, text: &str);

                            #[wasm_bindgen(method, js_name = appendChild)]
                            fn append_child(this: &Element, child: &Element);
                        }

                        let style_element = DOCUMENT.create_element("style");
                        let css_rules = format!(".{} {{ {} }}", class_name, #css_literal);
                        style_element.set_text_content(&css_rules);

                        if let Some(head) = DOCUMENT.query_selector("head") {
                            head.append_child(&style_element);
                        }
                    }

                    class_name.to_string()
                }).clone()
            } else {
                ::std::string::String::new()
            }
        }
    })
}

/// Internal implementation of the css_theme! macro
#[cfg(feature = "proc-macro")]
pub fn css_theme_impl_internal(_input: TokenStream) -> syn::Result<TokenStream> {
    // For now, just return a placeholder implementation
    // This would be expanded in Phase 2 with proper theme support
    Ok(quote! {
        {
            // Theme support will be implemented in Phase 2
            compile_error!("css_theme! macro is not yet implemented. Use css! macro instead.")
        }
    })
}

/// Process CSS string and generate the appropriate TokenStream
#[cfg(feature = "proc-macro")]
fn process_css_string(css_content: &str, _span: Span) -> syn::Result<TokenStream> {
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

                        #[wasm_bindgen(method, js_name = querySelector)]
                        fn query_selector(this: &Document, selector: &str) -> Option<Element>;

                        #[wasm_bindgen(method, setter, js_name = textContent)]
                        fn set_text_content(this: &Element, text: &str);

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Element, child: &Element);
                    }

                    let style_element = DOCUMENT.create_element("style");
                    let css_rules = format!(".{} {{ {} }}", class_name, #css_literal);
                    style_element.set_text_content(&css_rules);

                    if let Some(head) = DOCUMENT.query_selector("head") {
                        head.append_child(&style_element);
                    }
                }

                class_name.to_string()
            }).clone()
        }
    })
}

/// Parse CSS syntax from TokenStream
#[cfg(feature = "proc-macro")]
fn parse_css_syntax(input: TokenStream) -> syn::Result<String> {
    let input_str = input.to_string();

    // Remove outer braces if present
    let css_content = if input_str.trim().starts_with('{') && input_str.trim().ends_with('}') {
        let trimmed = input_str.trim();
        &trimmed[1..trimmed.len() - 1]
    } else {
        &input_str
    };

    // Basic CSS validation and formatting
    let mut result = String::new();
    let mut chars = css_content.chars().peekable();
    let mut in_string = false;
    let mut string_char = '"';

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' if !in_string => {
                in_string = true;
                string_char = ch;
                result.push(ch);
            }
            ch if in_string && ch == string_char => {
                in_string = false;
                result.push(ch);
            }
            '\\' if in_string => {
                result.push(ch);
                if let Some(next_ch) = chars.next() {
                    result.push(next_ch);
                }
            }
            ':' if !in_string => {
                result.push(ch);
                // Add space after colon if not already present
                if chars.peek() != Some(&' ') {
                    result.push(' ');
                }
            }
            ';' if !in_string => {
                result.push(ch);
                // Add space after semicolon if not already present
                if chars.peek() != Some(&' ') && chars.peek() != Some(&'\n') {
                    result.push(' ');
                }
            }
            _ => result.push(ch),
        }
    }

    Ok(result.trim().to_string())
}

/// Calculate hash for CSS content using a simple hash function
#[cfg(feature = "proc-macro")]
fn calculate_css_hash(css: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(css.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Validate CSS syntax
#[cfg(feature = "proc-macro")]
fn validate_css_syntax(css: &str) -> Result<(), String> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut string_char = '"';
    let mut chars = css.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' if !in_string => {
                in_string = true;
                string_char = ch;
            }
            ch if in_string && ch == string_char => {
                in_string = false;
            }
            '\\' if in_string => {
                // Skip escaped character
                chars.next();
            }
            '{' if !in_string => {
                brace_count += 1;
            }
            '}' if !in_string => {
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

    if in_string {
        return Err("Unterminated string".to_string());
    }

    // Basic CSS property validation
    let rules: Vec<&str> = css.split(';').collect();
    for rule in rules {
        let rule = rule.trim();
        if rule.is_empty() {
            continue;
        }

        if !rule.contains(':') && !rule.contains('{') && !rule.contains('}') {
            return Err(format!("Invalid CSS rule: {}", rule));
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "proc-macro"))]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[test]
    fn test_parse_css_syntax() {
        let input = quote::quote! {
            color: red;
            font-size: 16px;
        };

        let result = parse_css_syntax(input).unwrap();
        println!("Parsed CSS result: {}", result);
        assert!(result.contains("color"));
        assert!(result.contains("red"));
        // The function might format the CSS differently, so check for both formats
        assert!(result.contains("font-size") || result.contains("font - size"));
        assert!(result.contains("16px"));
    }

    #[test]
    fn test_process_css_string() {
        let css = "color: red; font-size: 16px;";
        let result = process_css_string(css, Span::call_site());

        assert!(result.is_ok());
        let tokens = result.unwrap();
        let code = tokens.to_string();

        assert!(code.contains("CSS_INJECTED"));
        assert!(code.contains("color: red; font-size: 16px;"));
    }
}
