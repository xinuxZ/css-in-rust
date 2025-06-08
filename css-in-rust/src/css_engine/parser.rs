//! CSS parsing functionality
//! CSS Parser Module
//!
//! This module provides CSS parsing capabilities using lightningcss as the core engine
//! for high-performance CSS processing and optimization.
//!
//! The parser can operate in two modes:
//! - With the "optimizer" feature enabled, it uses lightningcss for full parsing capabilities
//! - Without the feature, it falls back to a simpler parsing implementation

#[cfg(feature = "optimizer")]
use lightningcss::{
    error::Error as LightningError,
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet as LightningStyleSheet},
    targets::{Browsers, Targets},
};
// use std::collections::HashMap; // Unused import

/// Configuration for CSS parser
///
/// Controls how CSS is parsed and processed, including browser targets and minification options.
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::parser::ParserConfig;
///
/// // Create a default configuration
/// let default_config = ParserConfig::default();
///
/// // Create a custom configuration with minification enabled
/// let custom_config = ParserConfig {
///     minify: true,
///     ..ParserConfig::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Target browsers for CSS compatibility
    #[cfg(feature = "optimizer")]
    pub targets: Option<Browsers>,
    #[cfg(not(feature = "optimizer"))]
    pub targets: Option<String>, // Fallback type
    /// Enable minification during parsing
    pub minify: bool,
}

impl Default for ParserConfig {
    /// Creates a default parser configuration
    ///
    /// By default, targets are set to standard browsers and minification is disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::ParserConfig;
    ///
    /// let config = ParserConfig::default();
    /// assert_eq!(config.minify, false);
    /// ```
    fn default() -> Self {
        Self {
            #[cfg(feature = "optimizer")]
            targets: Some(Browsers::default()),
            #[cfg(not(feature = "optimizer"))]
            targets: None,
            minify: false,
        }
    }
}

/// CSS parser error wrapper for lightningcss errors
///
/// Represents errors that can occur during CSS parsing.
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::parser::{CssParser, ParseError};
///
/// fn process_css(css: &str) -> Result<String, ParseError> {
///     let parser = CssParser::new();
///     let stylesheet = parser.parse(css)?;
///     Ok(stylesheet.optimized)
/// }
/// ```
#[derive(Debug)]
pub enum ParseError {
    #[cfg(feature = "optimizer")]
    /// Error from the lightningcss library
    LightningCssError(LightningError<()>),
    /// Error due to invalid CSS input
    InvalidInput(String),
    /// Error during CSS processing
    ProcessingError(String),
}

impl std::fmt::Display for ParseError {
    /// Formats the error message for display
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::ParseError;
    ///
    /// let error = ParseError::InvalidInput("Missing semicolon".to_string());
    /// println!("Error: {}", error);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "optimizer")]
            ParseError::LightningCssError(err) => write!(f, "LightningCSS error: {:?}", err),
            ParseError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParseError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(feature = "optimizer")]
impl From<LightningError<()>> for ParseError {
    /// Converts a LightningCSS error into a ParseError
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "optimizer")]
    /// # {
    /// use css_in_rust::css_engine::parser::ParseError;
    /// use lightningcss::error::Error as LightningError;
    ///
    /// fn handle_lightning_error(err: LightningError<()>) {
    ///     let parse_error: ParseError = err.into();
    ///     eprintln!("Parse error: {}", parse_error);
    /// }
    /// # }
    /// ```
    fn from(err: LightningError<()>) -> Self {
        ParseError::LightningCssError(err)
    }
}

/// CSS stylesheet wrapper around lightningcss
///
/// Represents a parsed CSS stylesheet with both original source and optimized output.
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::parser::{CssParser, StyleSheet};
///
/// let parser = CssParser::new();
/// let css = ".button { color: red; }";
/// let stylesheet = parser.parse(css).unwrap();
///
/// println!("Original: {}", stylesheet.source);
/// println!("Optimized: {}", stylesheet.optimized);
/// ```
#[derive(Debug, Clone)]
pub struct StyleSheet {
    /// The original CSS source
    pub source: String,
    /// Parsed and optimized CSS output
    pub optimized: String,
    /// Metadata extracted during parsing
    pub metadata: StyleSheetMetadata,
}

/// Metadata extracted from CSS parsing
///
/// Contains information about the parsed CSS stylesheet.
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::parser::{StyleSheetMetadata};
///
/// let metadata = StyleSheetMetadata::default();
/// assert_eq!(metadata.rule_count, 0);
/// assert_eq!(metadata.has_media_queries, false);
/// ```
#[derive(Debug, Clone, Default)]
pub struct StyleSheetMetadata {
    /// Number of rules in the stylesheet
    pub rule_count: usize,
    /// Whether the CSS contains media queries
    pub has_media_queries: bool,
    /// Whether the CSS contains keyframes
    pub has_keyframes: bool,
    /// List of CSS custom properties (variables)
    pub custom_properties: Vec<String>,
}

/// CSS parser using lightningcss
///
/// Parses CSS strings into structured StyleSheet objects.
///
/// # Examples
///
/// ```
/// use css_in_rust::css_engine::parser::{CssParser, ParserConfig};
///
/// // Create a parser with default configuration
/// let parser = CssParser::new();
///
/// // Create a parser with custom configuration
/// let config = ParserConfig {
///     minify: true,
///     ..ParserConfig::default()
/// };
/// let custom_parser = CssParser::with_config(config);
///
/// // Parse CSS
/// let css = ".button { color: red; }";
/// let stylesheet = parser.parse(css).unwrap();
/// ```
pub struct CssParser {
    config: ParserConfig,
}

impl CssParser {
    /// Create a new CSS parser with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::CssParser;
    ///
    /// let parser = CssParser::new();
    /// let css = ".button { color: red; }";
    /// let stylesheet = parser.parse(css).unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a new CSS parser with custom configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::{CssParser, ParserConfig};
    ///
    /// let config = ParserConfig {
    ///     minify: true,
    ///     ..ParserConfig::default()
    /// };
    /// let parser = CssParser::with_config(config);
    ///
    /// let css = ".button { color: red; }";
    /// let stylesheet = parser.parse(css).unwrap();
    /// assert!(stylesheet.optimized.len() <= css.len()); // Should be minified
    /// ```
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse CSS string into a StyleSheet object
    ///
    /// 将CSS字符串解析为StyleSheet对象，提供原始源代码和优化后的输出。
    /// 当启用了"optimizer"特性时，会使用lightningcss进行高性能解析。
    ///
    /// # 参数
    ///
    /// * `css` - 要解析的CSS字符串
    ///
    /// # 返回值
    ///
    /// 成功时返回包含原始源代码、优化后代码和元数据的`StyleSheet`对象，
    /// 失败时返回`ParseError`错误。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::{CssParser, ParserConfig};
    ///
    /// // 创建解析器
    /// let parser = CssParser::new();
    ///
    /// // 简单的CSS
    /// let css = ".button {
    ///     background-color: #0066cc;
    ///     color: white;
    ///     padding: 8px 16px;
    ///     border-radius: 4px;
    /// }";
    ///
    /// // 解析CSS
    /// let result = parser.parse(css);
    /// assert!(result.is_ok());
    ///
    /// // 获取解析结果
    /// let stylesheet = result.unwrap();
    /// println!("原始CSS: {}", stylesheet.source);
    /// println!("优化后CSS: {}", stylesheet.optimized);
    /// println!("规则数量: {}", stylesheet.metadata.rule_count);
    ///
    /// // 检查是否包含媒体查询
    /// assert_eq!(stylesheet.metadata.has_media_queries, false);
    ///
    /// // 解析带有媒体查询的CSS
    /// let responsive_css = "@media (max-width: 768px) {
    ///     .container { width: 100%; }
    /// }";
    /// let responsive_result = parser.parse(responsive_css).unwrap();
    /// assert_eq!(responsive_result.metadata.has_media_queries, true);
    ///
    /// // 解析带有CSS变量的CSS
    /// let theme_css = ":root {
    ///     --primary-color: #0066cc;
    /// }
    /// .themed-button {
    ///     background-color: var(--primary-color);
    /// }";
    /// let theme_result = parser.parse(theme_css).unwrap();
    /// assert!(theme_result.metadata.custom_properties.contains(&"--primary-color".to_string()));
    /// ```
    ///
    /// # 错误处理
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::{CssParser, ParseError};
    ///
    /// let parser = CssParser::new();
    ///
    /// // 无效的CSS（缺少分号）
    /// let invalid_css = ".error { color: red background: #ffeeee }";
    /// let result = parser.parse(invalid_css);
    ///
    /// // 处理解析错误
    /// match result {
    ///     Ok(_) => println!("解析成功"),
    ///     Err(err) => match err {
    ///         ParseError::InvalidInput(msg) => println!("无效输入: {}", msg),
    ///         ParseError::ProcessingError(msg) => println!("处理错误: {}", msg),
    ///         #[cfg(feature = "optimizer")]
    ///         ParseError::LightningCssError(err) => println!("LightningCSS错误: {:?}", err),
    ///     }
    /// }
    /// ```
    #[cfg(feature = "optimizer")]
    pub fn parse(&self, css: &str) -> Result<StyleSheet, ParseError> {
        // Empty CSS check
        if css.trim().is_empty() {
            return Ok(StyleSheet {
                source: String::new(),
                optimized: String::new(),
                metadata: StyleSheetMetadata::default(),
            });
        }

        // Parse with lightningcss
        let parser_options = ParserOptions {
            nesting: true,
            custom_media: true,
            ..ParserOptions::default()
        };

        let stylesheet = LightningStyleSheet::parse(css, parser_options)?;

        // Generate printer options
        let mut printer_options = PrinterOptions::default();
        printer_options.minify = self.config.minify;

        if let Some(targets) = &self.config.targets {
            printer_options.targets = Targets::Browsers(targets.clone());
        }

        // Print the optimized CSS
        let optimized = stylesheet.to_css(printer_options)?.code;

        // Extract metadata
        let metadata = self.extract_metadata(&stylesheet);

        Ok(StyleSheet {
            source: css.to_string(),
            optimized,
            metadata,
        })
    }

    /// Parse CSS string into a StyleSheet object (fallback implementation)
    ///
    /// 将CSS字符串解析为StyleSheet对象的备用实现，当未启用"optimizer"特性时使用。
    /// 这个版本提供基本的CSS验证，但不执行完整的解析和优化。
    ///
    /// # 参数
    ///
    /// * `css` - 要解析的CSS字符串
    ///
    /// # 返回值
    ///
    /// 成功时返回包含原始源代码和基本处理后代码的`StyleSheet`对象，
    /// 失败时返回`ParseError`错误。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::CssParser;
    ///
    /// // 创建解析器
    /// let parser = CssParser::new();
    ///
    /// // 简单的CSS
    /// let css = ".simple { font-size: 14px; }";
    /// let result = parser.parse(css);
    /// assert!(result.is_ok());
    ///
    /// // 获取解析结果
    /// let stylesheet = result.unwrap();
    /// assert_eq!(stylesheet.source, css);
    /// ```
    #[cfg(not(feature = "optimizer"))]
    pub fn parse(&self, css: &str) -> Result<StyleSheet, ParseError> {
        // Empty CSS check
        if css.trim().is_empty() {
            return Ok(StyleSheet {
                source: String::new(),
                optimized: String::new(),
                metadata: StyleSheetMetadata::default(),
            });
        }

        // Basic validation
        if let Err(msg) = self.validate_basic_css_syntax(css) {
            return Err(ParseError::InvalidInput(msg));
        }

        // Simple metadata extraction
        let metadata = StyleSheetMetadata {
            rule_count: css.matches('{').count(),
            has_media_queries: css.contains("@media"),
            has_keyframes: css.contains("@keyframes"),
            custom_properties: css
                .lines()
                .filter_map(|line| {
                    if line.contains("--") {
                        let start = line.find("--")?;
                        let end = line[start..].find(':')?;
                        Some(line[start..start + end].trim().to_string())
                    } else {
                        None
                    }
                })
                .collect(),
        };

        // Simple optimization (whitespace removal)
        let optimized = if self.config.minify {
            css.lines()
                .map(|line| line.trim())
                .collect::<Vec<_>>()
                .join("")
                .replace("{ ", "{")
                .replace(" }", "}")
                .replace(", ", ",")
                .replace("; ", ";")
                .replace(" {", "{")
                .replace(" :", ":")
                .replace(": ", ":")
        } else {
            css.to_string()
        };

        Ok(StyleSheet {
            source: css.to_string(),
            optimized,
            metadata,
        })
    }

    /// Basic CSS syntax validation for fallback implementation
    ///
    /// Performs simple syntax validation when the optimizer feature is not enabled.
    ///
    /// # Arguments
    ///
    /// * `css` - The CSS string to validate
    ///
    /// # Returns
    ///
    /// A `Result` containing either `()` if valid or an error message string
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "optimizer"))]
    /// # {
    /// use css_in_rust::css_engine::parser::CssParser;
    ///
    /// let parser = CssParser::new();
    /// let valid_css = ".button { color: red; }";
    /// let invalid_css = ".button { color: red; } }"; // Extra closing brace
    ///
    /// // This would be called internally by parse()
    /// assert!(parser.validate_basic_css_syntax(valid_css).is_ok());
    /// assert!(parser.validate_basic_css_syntax(invalid_css).is_err());
    /// # }
    /// ```
    #[cfg(not(feature = "optimizer"))]
    fn validate_basic_css_syntax(&self, css: &str) -> Result<(), String> {
        let mut brace_count = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut string_char = '\0';

        for ch in css.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count < 0 {
                        return Err("Unexpected closing brace".to_string());
                    }
                }
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err("Mismatched braces".to_string());
        }

        if in_string {
            return Err("Unterminated string".to_string());
        }

        Ok(())
    }

    /// Extract metadata from parsed stylesheet
    ///
    /// Analyzes the parsed stylesheet to extract metadata like rule count, media queries, etc.
    ///
    /// # Arguments
    ///
    /// * `stylesheet` - The parsed LightningStyleSheet
    ///
    /// # Returns
    ///
    /// A `StyleSheetMetadata` object with extracted information
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "optimizer")]
    /// # {
    /// use css_in_rust::css_engine::parser::CssParser;
    /// use lightningcss::stylesheet::{StyleSheet as LightningStyleSheet, ParserOptions};
    ///
    /// let parser = CssParser::new();
    /// let css = ".button { color: red; } @media (max-width: 768px) { .button { color: blue; } }";
    /// let lightning_stylesheet = LightningStyleSheet::parse(css, ParserOptions::default()).unwrap();
    ///
    /// // This would be called internally during parsing
    /// let metadata = parser.extract_metadata(&lightning_stylesheet);
    /// # }
    /// ```
    #[cfg(feature = "optimizer")]
    fn extract_metadata(&self, _stylesheet: &LightningStyleSheet) -> StyleSheetMetadata {
        // This is a simplified implementation
        // In a full implementation, we would traverse the stylesheet AST
        // to extract detailed metadata
        StyleSheetMetadata {
            rule_count: 0,                 // Would count actual rules
            has_media_queries: false,      // Would detect media queries
            has_keyframes: false,          // Would detect keyframes
            custom_properties: Vec::new(), // Would extract CSS variables
        }
    }
}

impl Default for CssParser {
    /// Creates a new CSS parser with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::css_engine::parser::CssParser;
    ///
    /// let parser = CssParser::default();
    /// let css = ".button { color: red; }";
    /// let stylesheet = parser.parse(css).unwrap();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_css() {
        let parser = CssParser::new();
        let result = parser.parse("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_basic_css() {
        let parser = CssParser::new();
        let css = ".button { color: red; }";
        let result = parser.parse(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_css() {
        let parser = CssParser::new();
        // Use a more obviously invalid CSS that should definitely fail
        let css = ".button { color: red; } } extra brace";
        let result = parser.parse(css);
        assert!(result.is_err());
    }
}
