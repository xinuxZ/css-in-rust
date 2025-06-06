use crate::cache_management::{cache_css, get_cached_css};
use crate::css_processing::{
    optimize_css_with_lightningcss, parse_css_syntax, process_media_queries,
    process_pseudo_selectors,
};
use crate::hash_utils::calculate_css_hash;
use crate::theme_variants::process_css_with_variants_and_themes;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Error, LitStr, Result};

/// Internal implementation of the css! macro
pub fn css_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
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
pub fn css_if_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
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
pub fn css_class_impl_internal(input: TokenStream2) -> syn::Result<TokenStream2> {
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

/// Internal implementation of the css_multi_if! macro
pub fn css_multi_if_impl_internal(input: TokenStream2) -> Result<TokenStream2> {
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

    // Process media queries and pseudo selectors
    let media_css = process_media_queries(&_media_queries);
    let pseudo_css = process_pseudo_selectors(&_pseudo_selectors);

    // Optimize CSS
    let optimized_css =
        optimize_css_with_lightningcss(&css_literal).unwrap_or_else(|_| css_literal.clone());

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

                    // Inline wasm_bindgen declarations to ensure proper scope
                    #[wasm_bindgen]
                    extern "C" {
                        type Document;
                        type Element;
                        type Node;

                        #[wasm_bindgen(method, getter, js_name = head)]
                        fn head(this: &Document) -> Element;

                        #[wasm_bindgen(method, js_name = createElement)]
                        fn create_element(this: &Document, tag_name: &str) -> Element;

                        #[wasm_bindgen(method, js_name = getElementById)]
                        fn get_element_by_id(this: &Document, id: &str) -> Option<Element>;

                        #[wasm_bindgen(method, js_name = setAttribute)]
                        fn set_attribute(this: &Element, name: &str, value: &str);

                        #[wasm_bindgen(method, setter, js_name = innerHTML)]
                        fn set_inner_html(this: &Element, html: &str);

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Element, child: &Node);

                        #[wasm_bindgen(js_name = document)]
                        static DOCUMENT: Document;
                    }

                    impl From<Element> for Node {
                        fn from(element: Element) -> Node {
                            element.unchecked_into()
                        }
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

                        // let css_rules = crate::css_processing::compress_css(deduplicated_rules.join("\n"));
                        let css_rules = deduplicated_rules.join("\n")
                            .lines().map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join("")
                            .replace("; ", ";")
                            .replace(": ", ":")
                            .replace(" {", "{")
                            .replace("{ ", "{")
                            .replace(" }", "}")
                            .replace("} ", "}")
                            .replace(",", ", ");

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
fn process_css_with_cache(css_content: &str, css_id: &str) -> Result<TokenStream2> {
    let css_hash = calculate_css_hash(css_content);

    // Check cache first
    if let Some(cached_class) = get_cached_css(&css_hash) {
        return Ok(quote! { #cached_class });
    }

    // Process CSS if not cached
    let processed_css = process_css_with_variants_and_themes(css_content)?;
    let optimized_css = optimize_css_with_lightningcss(&processed_css.css)
        .unwrap_or_else(|_| processed_css.css.clone());

    // Process media queries and pseudo selectors
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
                    // use std::collections::HashMap;
                    // use std::sync::{Mutex, OnceLock};

                    // static CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
                    // let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

                    // if let Ok(mut cache_guard) = cache.lock() {
                    //     cache_guard.insert(#css_hash.to_string(), class_name.clone());
                    // }

                    // For non-WASM targets, caching is handled at compile time
                    // No runtime caching needed for non-WASM targets
                }

                // Inject CSS into document head (web target only)
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::prelude::*;

                    // Inline wasm_bindgen declarations to ensure proper scope
                    #[wasm_bindgen]
                    extern "C" {
                        type Document;
                        type Element;
                        type Node;

                        #[wasm_bindgen(method, getter, js_name = head)]
                        fn head(this: &Document) -> Element;

                        #[wasm_bindgen(method, js_name = createElement)]
                        fn create_element(this: &Document, tag_name: &str) -> Element;

                        #[wasm_bindgen(method, js_name = getElementById)]
                        fn get_element_by_id(this: &Document, id: &str) -> Option<Element>;

                        #[wasm_bindgen(method, js_name = setAttribute)]
                        fn set_attribute(this: &Element, name: &str, value: &str);

                        #[wasm_bindgen(method, setter, js_name = innerHTML)]
                        fn set_inner_html(this: &Element, html: &str);

                        #[wasm_bindgen(method, js_name = appendChild)]
                        fn append_child(this: &Element, child: &Node);

                        #[wasm_bindgen(js_name = document)]
                        static DOCUMENT: Document;
                    }

                    impl From<Element> for Node {
                        fn from(element: Element) -> Node {
                            element.unchecked_into()
                        }
                    }

                    // Check if style element already exists
                    let style_id = format!("css-cache-{}", #css_hash);
                    if DOCUMENT.get_element_by_id(&style_id).is_none() {
                        let style_element = DOCUMENT.create_element("style");
                        style_element.set_attribute("id", &style_id);

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

                        // let css_rules = compress_css(deduplicated_rules.join("\n"));
                        let css_rules = deduplicated_rules.join("\n")
                            .lines().map(|line| line.trim())
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join("")
                            .replace("; ", ";")
                            .replace(": ", ":")
                            .replace(" {", "{")
                            .replace("{ ", "{")
                            .replace(" }", "}")
                            .replace("} ", "}")
                            .replace(",", ", ");

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
