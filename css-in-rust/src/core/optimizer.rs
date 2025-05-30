//! CSS optimization functionality
//! CSS Optimizer Module
//!
//! This module provides CSS optimization capabilities leveraging lightningcss
//! for high-performance minification and optimization.

use crate::core::parser::{ParseError, StyleSheet};
use lightningcss::{
    printer::PrinterOptions, stylesheet::StyleSheet as LightningStyleSheet, targets::Browsers,
};

/// Configuration for CSS optimizer based on lightningcss
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Whether to minify the CSS output
    pub minify: bool,
    /// Target browsers for optimization
    pub targets: Option<Browsers>,
    /// Whether to analyze dependencies for unused code elimination
    pub analyze_dependencies: bool,
    /// Whether to enable vendor prefix removal based on targets
    pub vendor_prefix: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            minify: true,
            targets: Some(Browsers::default()),
            analyze_dependencies: false,
            vendor_prefix: true,
        }
    }
}

/// CSS optimization error
#[derive(Debug)]
pub enum OptimizationError {
    ParseError(ParseError),
    OptimizationFailed(String),
    InvalidConfiguration(String),
}

impl std::fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationError::ParseError(err) => {
                write!(f, "Parse error during optimization: {}", err)
            }
            OptimizationError::OptimizationFailed(msg) => write!(f, "Optimization failed: {}", msg),
            OptimizationError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
        }
    }
}

impl std::error::Error for OptimizationError {}

impl From<ParseError> for OptimizationError {
    fn from(err: ParseError) -> Self {
        OptimizationError::ParseError(err)
    }
}

/// CSS optimizer using lightningcss
pub struct CssOptimizer {
    config: OptimizerConfig,
}

impl CssOptimizer {
    /// Create a new CSS optimizer with default configuration
    pub fn new() -> Self {
        Self {
            config: OptimizerConfig::default(),
        }
    }

    /// Create a new CSS optimizer with custom configuration
    pub fn with_config(config: OptimizerConfig) -> Self {
        Self { config }
    }

    /// Optimize a CSS stylesheet using lightningcss
    pub fn optimize(&self, stylesheet: StyleSheet) -> Result<String, OptimizationError> {
        // If the stylesheet already has optimized CSS from parsing, use it
        if !stylesheet.optimized.is_empty() && self.config.minify {
            return Ok(stylesheet.optimized);
        }

        // Otherwise, optimize the source CSS
        self.optimize_string(&stylesheet.source)
    }

    /// Optimize CSS string directly using lightningcss
    pub fn optimize_string(&self, css: &str) -> Result<String, OptimizationError> {
        use lightningcss::stylesheet::ParserOptions;

        // Parse CSS with lightningcss
        let stylesheet =
            LightningStyleSheet::parse(css, ParserOptions::default()).map_err(|e| {
                OptimizationError::OptimizationFailed(format!("Failed to parse CSS: {:?}", e))
            })?;

        // Create printer options based on config
        let printer_options = PrinterOptions {
            minify: self.config.minify,
            targets: self.config.targets.clone().unwrap_or_default().into(),
            ..Default::default()
        };

        // Generate optimized CSS
        let result = stylesheet.to_css(printer_options).map_err(|e| {
            OptimizationError::OptimizationFailed(format!("Failed to optimize CSS: {:?}", e))
        })?;

        Ok(result.code)
    }
}

impl Default for CssOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::{StyleSheet, StyleSheetMetadata};

    #[test]
    fn test_optimize_string() {
        let optimizer = CssOptimizer::new();
        let css = ".button {\n  color: red;\n  font-size: 16px;\n}";
        let result = optimizer.optimize_string(css).unwrap();

        // With lightningcss minification, the result should be smaller
        assert!(result.len() <= css.len());
        assert!(result.contains(".button"));
        assert!(result.contains("color:red") || result.contains("color: red"));
    }

    #[test]
    fn test_optimize_stylesheet() {
        let optimizer = CssOptimizer::new();
        let stylesheet = StyleSheet {
            source: ".test { margin: 0px; }".to_string(),
            optimized: String::new(),
            metadata: StyleSheetMetadata::default(),
        };

        let result = optimizer.optimize(stylesheet).unwrap();
        assert!(result.contains(".test"));
        assert!(result.contains("margin"));
    }

    #[test]
    fn test_optimize_with_preoptimized() {
        let optimizer = CssOptimizer::new();
        let stylesheet = StyleSheet {
            source: ".test { margin: 0px; }".to_string(),
            optimized: ".test{margin:0}".to_string(),
            metadata: StyleSheetMetadata::default(),
        };

        let result = optimizer.optimize(stylesheet).unwrap();
        // Should use the pre-optimized version
        assert_eq!(result, ".test{margin:0}");
    }
}
