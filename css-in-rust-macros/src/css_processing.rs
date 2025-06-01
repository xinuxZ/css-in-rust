use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet as LightningStyleSheet},
    targets::{Browsers, Targets},
};
use proc_macro2::TokenStream as TokenStream2;

/// Process CSS string and generate unique CSS ID with media queries and pseudo selectors

/// Parse CSS syntax for validation
/// Parse CSS syntax from token stream
pub fn parse_css_syntax(input: TokenStream2) -> syn::Result<String> {
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
pub fn optimize_css_with_lightningcss(css: &str) -> Result<String, String> {
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
#[allow(dead_code)]
pub fn deduplicate_css_rules(rules: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut deduplicated = Vec::new();

    for rule in rules {
        let normalized = rule.trim();
        if !normalized.is_empty() && seen.insert(normalized.to_string()) {
            deduplicated.push(rule.clone());
        }
    }

    deduplicated
}

/// Compress CSS by removing unnecessary whitespace
#[allow(dead_code)]
pub fn compress_css(css: &str) -> String {
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
pub fn validate_css_simple(css: &str) -> Result<String, String> {
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
pub fn validate_css_syntax(css: &str) -> Result<(), String> {
    optimize_css_with_lightningcss(css).map(|_| ())
}

/// Process media queries into CSS rules with class name placeholder
pub fn process_media_queries(media_queries: &[(String, String)]) -> String {
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
pub fn process_pseudo_selectors(pseudo_selectors: &[(String, String)]) -> String {
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
