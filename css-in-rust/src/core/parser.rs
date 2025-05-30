//! CSS parsing functionality
//! CSS Parser Module
//!
//! This module provides CSS parsing capabilities using lightningcss as the core engine
//! for high-performance CSS processing and optimization.

#[cfg(feature = "optimizer")]
use lightningcss::{
    error::Error as LightningError,
    printer::PrinterOptions,
    stylesheet::{ParserOptions, StyleSheet as LightningStyleSheet},
    targets::{Browsers, Targets},
};
// use std::collections::HashMap; // Unused import

/// Configuration for CSS parser
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
#[derive(Debug)]
pub enum ParseError {
    #[cfg(feature = "optimizer")]
    LightningCssError(LightningError<()>),
    InvalidInput(String),
    ProcessingError(String),
}

impl std::fmt::Display for ParseError {
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
    fn from(err: LightningError<()>) -> Self {
        ParseError::LightningCssError(err)
    }
}

/// CSS stylesheet wrapper around lightningcss
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
pub struct CssParser {
    config: ParserConfig,
}

impl CssParser {
    /// Create a new CSS parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a new CSS parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse CSS string into a stylesheet
    #[cfg(feature = "optimizer")]
    pub fn parse(&self, css: &str) -> Result<StyleSheet, ParseError> {
        if css.trim().is_empty() {
            return Ok(StyleSheet {
                source: css.to_string(),
                optimized: String::new(),
                metadata: StyleSheetMetadata::default(),
            });
        }

        // Parse CSS using lightningcss with default options
        let stylesheet =
            LightningStyleSheet::parse(css, ParserOptions::default()).map_err(|e| {
                ParseError::ProcessingError(format!("lightningcss parse error: {:?}", e))
            })?;

        // Create printer options for optimization
        let mut printer_options = PrinterOptions::default();
        printer_options.minify = true;
        if let Some(targets) = &self.config.targets {
            printer_options.targets = Targets::from(targets.clone());
        }

        // Generate optimized CSS
        let optimized_css = stylesheet
            .to_css(printer_options)
            .map_err(|e| ParseError::ProcessingError(format!("Failed to optimize CSS: {:?}", e)))?;

        Ok(StyleSheet {
            source: css.to_string(),
            optimized: optimized_css.code,
            metadata: StyleSheetMetadata::default(),
        })
    }

    /// Parse CSS string into a stylesheet (fallback implementation)
    #[cfg(not(feature = "optimizer"))]
    pub fn parse(&self, css: &str) -> Result<StyleSheet, ParseError> {
        if css.trim().is_empty() {
            return Ok(StyleSheet {
                source: css.to_string(),
                optimized: String::new(),
                metadata: StyleSheetMetadata::default(),
            });
        }

        // Simple fallback: basic minification if enabled
        let optimized = if self.config.minify {
            css.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            css.to_string()
        };

        Ok(StyleSheet {
            source: css.to_string(),
            optimized,
            metadata: StyleSheetMetadata::default(),
        })
    }

    /// Extract metadata from parsed stylesheet
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
        let css = ".button { color: red;";
        let result = parser.parse(css);
        assert!(result.is_err());
    }
}
