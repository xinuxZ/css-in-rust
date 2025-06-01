// Theme and variant processing for CSS
use crate::utility_conversion::convert_utility_to_css;

/// Processed CSS with variants and themes
#[derive(Debug, Clone)]
pub struct ProcessedCss {
    pub css: String,
    pub media_queries: Vec<(String, String)>,
    pub pseudo_selectors: Vec<(String, String)>,
}

/// Process CSS string with variant and theme variable support
pub fn process_css_with_variants_and_themes(css: &str) -> syn::Result<ProcessedCss> {
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

/// Parse responsive variant (e.g., "sm:bg-blue-500" -> ("sm", "bg-blue-500"))
fn parse_responsive_variant(input: &str) -> Option<(String, &str)> {
    let responsive_prefixes = ["sm:", "md:", "lg:", "xl:", "2xl:"];

    for prefix in &responsive_prefixes {
        if let Some(class) = input.strip_prefix(prefix) {
            return Some((prefix.trim_end_matches(':').to_string(), class));
        }
    }

    None
}

/// Parse pseudo-class variant (e.g., "hover:bg-blue-500" -> ("hover", "bg-blue-500"))
fn parse_pseudo_variant(input: &str) -> Option<(String, &str)> {
    let pseudo_prefixes = [
        "hover:",
        "focus:",
        "active:",
        "visited:",
        "disabled:",
        "first:",
        "last:",
        "odd:",
        "even:",
        "focus-within:",
        "focus-visible:",
        "target:",
        "checked:",
        "indeterminate:",
        "default:",
    ];

    for prefix in &pseudo_prefixes {
        if let Some(class) = input.strip_prefix(prefix) {
            return Some((prefix.trim_end_matches(':').to_string(), class));
        }
    }

    None
}

/// Get media query for responsive variant
fn get_media_query_for_variant(variant: &str) -> String {
    match variant {
        "sm" => "(min-width: 640px)".to_string(),
        "md" => "(min-width: 768px)".to_string(),
        "lg" => "(min-width: 1024px)".to_string(),
        "xl" => "(min-width: 1280px)".to_string(),
        "2xl" => "(min-width: 1536px)".to_string(),
        _ => format!("(min-width: 640px)"), // fallback
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
