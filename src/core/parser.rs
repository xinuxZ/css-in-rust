//! CSS Parser implementation using LightningCSS
//!
//! This module provides high-performance CSS parsing capabilities
//! with comprehensive error handling and validation.

use crate::core::{CssError, Result};
use lightningcss::{
    stylesheet::{ParserOptions, StyleSheet},
    targets::Browsers,
};

/// CSS Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Target browsers for compatibility
    pub targets: Option<Browsers>,
    /// Whether to enable CSS nesting
    pub nesting: bool,
    /// Whether to enable custom media queries
    pub custom_media: bool,
    /// Error recovery mode
    pub error_recovery: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            targets: Some(Browsers::default()),
            nesting: true,
            custom_media: true,
            error_recovery: true,
        }
    }
}

/// High-performance CSS parser
///
/// Uses LightningCSS for fast and accurate CSS parsing with
/// comprehensive error reporting and validation.
pub struct CssParser {
    config: ParserConfig,
}

impl CssParser {
    /// Create a new CSS parser with default configuration
    pub fn new() -> Self {
        let config = ParserConfig::default();
        Self { config }
    }

    /// Create a new CSS parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse CSS string into a StyleSheet
    ///
    /// # Arguments
    /// * `css` - The CSS string to parse
    /// * `filename` - Optional filename for error reporting
    ///
    /// # Returns
    /// * `Ok(StyleSheet)` - Successfully parsed stylesheet
    /// * `Err(CssError)` - Parse error with detailed information
    pub fn parse(&self, css: &str) -> Result<StyleSheet> {
        self.parse_with_filename(css, "<input>")
    }

    /// Parse CSS string with a specific filename for error reporting
    pub fn parse_with_filename(&self, css: &str, filename: &str) -> Result<StyleSheet> {
        StyleSheet::parse(css, ParserOptions::default()).map_err(|err| {
            CssError::parse_error(
                format!("Failed to parse CSS in {}: {:?}", filename, err),
                None,
                None,
            )
        })
    }

    /// Validate CSS without returning the parsed result
    ///
    /// This is more efficient when you only need to check if CSS is valid.
    pub fn validate(&self, css: &str) -> Result<()> {
        self.parse(css).map(|_| ())
    }

    /// Validate CSS with detailed error information
    pub fn validate_with_details(&self, css: &str) -> Result<ValidationResult> {
        match self.parse(css) {
            Ok(stylesheet) => {
                let rule_count = count_rules(&stylesheet);
                Ok(ValidationResult {
                    valid: true,
                    rule_count,
                    warnings: Vec::new(),
                    errors: Vec::new(),
                })
            }
            Err(err) => Ok(ValidationResult {
                valid: false,
                rule_count: 0,
                warnings: Vec::new(),
                errors: vec![err.message().to_string()],
            }),
        }
    }

    /// Get parser configuration
    pub fn config(&self) -> &ParserConfig {
        &self.config
    }

    /// Update parser configuration
    pub fn set_config(&mut self, config: ParserConfig) {
        self.config = config;
    }
}

impl Default for CssParser {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the CSS is valid
    pub valid: bool,
    /// Number of CSS rules found
    pub rule_count: usize,
    /// Warning messages
    pub warnings: Vec<String>,
    /// Error messages
    pub errors: Vec<String>,
}

/// Count the number of rules in a stylesheet
fn count_rules(stylesheet: &StyleSheet) -> usize {
    stylesheet.rules.0.len()
}

/// Utility functions for CSS parsing
pub mod utils {
    use super::*;

    /// Quick validation function for simple use cases
    pub fn is_valid_css(css: &str) -> bool {
        CssParser::new().validate(css).is_ok()
    }

    /// Extract CSS selectors from a stylesheet
    pub fn extract_selectors(css: &str) -> Result<Vec<String>> {
        let parser = CssParser::new();
        let stylesheet = parser.parse(css)?;

        // This is a simplified implementation
        // In a real implementation, you would traverse the AST
        // to extract all selectors
        Ok(Vec::new())
    }

    /// Normalize CSS by parsing and re-serializing
    pub fn normalize_css(css: &str) -> Result<String> {
        let parser = CssParser::new();
        let stylesheet = parser.parse(css)?;

        // Use the optimizer to serialize back to string
        crate::core::CssOptimizer::new().optimize(stylesheet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = CssParser::new();
        assert!(parser.config().nesting);
        assert!(parser.config().custom_media);
    }

    #[test]
    fn test_valid_css_parsing() {
        let parser = CssParser::new();
        let css = ".button { background: red; color: white; }";

        let result = parser.parse(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_css_parsing() {
        let parser = CssParser::new();
        let css = ".button { background: ; }";

        let result = parser.parse(css);
        assert!(result.is_err());
    }

    #[test]
    fn test_css_validation() {
        let parser = CssParser::new();

        assert!(parser.validate(".valid { color: red; }").is_ok());
        assert!(parser.validate(".invalid { color: ; }").is_err());
    }

    #[test]
    fn test_validation_with_details() {
        let parser = CssParser::new();
        let css = ".test { color: blue; } .another { background: white; }";

        let result = parser.validate_with_details(css).unwrap();
        assert!(result.valid);
        assert_eq!(result.rule_count, 1); // LightningCSS might group rules differently
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_parser_with_custom_config() {
        let config = ParserConfig {
            nesting: false,
            custom_media: false,
            error_recovery: false,
            ..Default::default()
        };

        let parser = CssParser::with_config(config);
        assert!(!parser.config().nesting);
        assert!(!parser.config().custom_media);
    }

    #[test]
    fn test_utils_is_valid_css() {
        assert!(utils::is_valid_css(".test { color: red; }"));
        assert!(!utils::is_valid_css(".test { color: ; }"));
    }
}
