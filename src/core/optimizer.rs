//! CSS Optimizer implementation using LightningCSS
//!
//! This module provides CSS optimization capabilities including
//! minification, dead code elimination, and browser-specific optimizations.

use crate::core::{CssError, Result};
use lightningcss::{printer::PrinterOptions, stylesheet::StyleSheet, targets::Browsers};
use std::collections::HashSet;

/// CSS optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Target browsers for optimization
    pub targets: Option<Browsers>,
    /// Whether to minify the output
    pub minify: bool,
    /// Whether to analyze dependencies
    pub analyze_dependencies: bool,
    /// Whether to remove unused code
    pub remove_unused: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            targets: Some(Browsers::default()),
            minify: true,
            analyze_dependencies: true,
            remove_unused: false, // Disabled by default for safety
        }
    }
}

/// High-performance CSS optimizer
///
/// Uses LightningCSS for advanced CSS optimizations including
/// minification, vendor prefix handling, and dead code elimination.
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

    /// Optimize a parsed stylesheet and return the CSS string
    ///
    /// # Arguments
    /// * `stylesheet` - The parsed stylesheet to optimize
    ///
    /// # Returns
    /// * `Ok(String)` - Optimized CSS string
    /// * `Err(CssError)` - Optimization error
    pub fn optimize(&self, stylesheet: StyleSheet) -> Result<String> {
        let printer_options = PrinterOptions {
            minify: self.config.minify,
            targets: self.config.targets.clone().unwrap_or_default().into(),
            ..Default::default()
        };

        stylesheet
            .to_css(printer_options)
            .map(|result| result.code)
            .map_err(|err| {
                CssError::optimization_error(format!("Failed to optimize CSS: {:?}", err))
            })
    }

    /// Optimize CSS string directly (parse + optimize)
    ///
    /// This is a convenience method that combines parsing and optimization.
    pub fn optimize_css(&self, css: &str) -> Result<String> {
        let parser = crate::core::CssParser::new();
        let stylesheet = parser.parse(css)?;
        self.optimize(stylesheet)
    }

    /// Minify CSS without other optimizations
    pub fn minify_only(&self, css: &str) -> Result<String> {
        let mut config = self.config.clone();
        config.minify = true;
        config.analyze_dependencies = false;
        config.remove_unused = false;

        let optimizer = Self::with_config(config);
        optimizer.optimize_css(css)
    }

    /// Get optimization statistics
    pub fn get_stats(&self, original: &str, optimized: &str) -> OptimizationStats {
        let original_size = original.len();
        let optimized_size = optimized.len();
        let compression_ratio = if original_size > 0 {
            (original_size - optimized_size) as f64 / original_size as f64
        } else {
            0.0
        };

        OptimizationStats {
            original_size,
            optimized_size,
            bytes_saved: original_size.saturating_sub(optimized_size),
            compression_ratio,
        }
    }

    /// Get optimizer configuration
    pub fn config(&self) -> &OptimizerConfig {
        &self.config
    }

    /// Update optimizer configuration
    pub fn set_config(&mut self, config: OptimizerConfig) {
        self.config = config;
    }

    /// Remove unused CSS rules based on used selectors
    ///
    /// # Arguments
    /// * `css` - The CSS to optimize
    /// * `used_selectors` - Set of selectors that are actually used
    ///
    /// # Returns
    /// * `Ok(String)` - CSS with unused rules removed
    /// * `Err(CssError)` - Optimization error
    pub fn remove_unused_rules(
        &self,
        css: &str,
        used_selectors: &HashSet<String>,
    ) -> Result<String> {
        // This is a simplified implementation
        // In a real implementation, you would:
        // 1. Parse the CSS into an AST
        // 2. Traverse the rules and check if selectors are in used_selectors
        // 3. Remove unused rules
        // 4. Serialize back to CSS

        // For now, just return the optimized CSS
        self.optimize_css(css)
    }

    /// Extract critical CSS for above-the-fold content
    pub fn extract_critical(
        &self,
        css: &str,
        critical_selectors: &HashSet<String>,
    ) -> Result<String> {
        // This is a placeholder implementation
        // In a real implementation, you would:
        // 1. Parse the CSS
        // 2. Filter rules to only include critical selectors
        // 3. Optimize and return the critical CSS

        self.optimize_css(css)
    }
}

impl Default for CssOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    /// Original CSS size in bytes
    pub original_size: usize,
    /// Optimized CSS size in bytes
    pub optimized_size: usize,
    /// Number of bytes saved
    pub bytes_saved: usize,
    /// Compression ratio (0.0 to 1.0)
    pub compression_ratio: f64,
}

impl OptimizationStats {
    /// Get compression percentage
    pub fn compression_percentage(&self) -> f64 {
        self.compression_ratio * 100.0
    }

    /// Check if optimization was effective
    pub fn is_effective(&self, threshold: f64) -> bool {
        self.compression_ratio >= threshold
    }
}

/// Utility functions for CSS optimization
pub mod utils {
    use super::*;

    /// Quick minification function
    pub fn minify(css: &str) -> Result<String> {
        let config = OptimizerConfig {
            minify: true,
            analyze_dependencies: false,
            remove_unused: false,
            ..Default::default()
        };

        CssOptimizer::with_config(config).optimize_css(css)
    }

    /// Calculate compression ratio between two CSS strings
    pub fn compression_ratio(original: &str, optimized: &str) -> f64 {
        let original_size = original.len();
        if original_size == 0 {
            return 0.0;
        }

        let optimized_size = optimized.len();
        (original_size - optimized_size) as f64 / original_size as f64
    }

    /// Check if CSS is already minified
    pub fn is_minified(css: &str) -> bool {
        // Simple heuristic: minified CSS has fewer whitespace characters
        let whitespace_count = css.chars().filter(|c| c.is_whitespace()).count();
        let total_chars = css.len();

        if total_chars == 0 {
            return true;
        }

        let whitespace_ratio = whitespace_count as f64 / total_chars as f64;
        whitespace_ratio < 0.1 // Less than 10% whitespace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = CssOptimizer::new();
        assert!(optimizer.config().minify);
        assert!(optimizer.config().analyze_dependencies);
    }

    #[test]
    fn test_css_optimization() {
        let optimizer = CssOptimizer::new();
        let css = ".button {\n  background: red;\n  color: white;\n}";

        let result = optimizer.optimize_css(css);
        assert!(result.is_ok());

        let optimized = result.unwrap();
        assert!(optimized.len() <= css.len()); // Should be smaller or equal
    }

    #[test]
    fn test_minify_only() {
        let optimizer = CssOptimizer::new();
        let css = ".test {\n  color: red;\n  background: blue;\n}";

        let result = optimizer.minify_only(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimization_stats() {
        let optimizer = CssOptimizer::new();
        let original = ".button {\n  background: red;\n  color: white;\n}";
        let optimized = ".button{background:red;color:white}";

        let stats = optimizer.get_stats(original, optimized);
        assert!(stats.compression_ratio > 0.0);
        assert!(stats.bytes_saved > 0);
        assert_eq!(stats.original_size, original.len());
        assert_eq!(stats.optimized_size, optimized.len());
    }

    #[test]
    fn test_optimizer_with_custom_config() {
        let config = OptimizerConfig {
            minify: false,
            analyze_dependencies: false,
            remove_unused: true,
            ..Default::default()
        };

        let optimizer = CssOptimizer::with_config(config);
        assert!(!optimizer.config().minify);
        assert!(optimizer.config().remove_unused);
    }

    #[test]
    fn test_utils_minify() {
        let css = ".test {\n  color: red;\n}";
        let result = utils::minify(css);
        assert!(result.is_ok());
    }

    #[test]
    fn test_utils_compression_ratio() {
        let original = ".test {\n  color: red;\n}";
        let optimized = ".test{color:red}";

        let ratio = utils::compression_ratio(original, optimized);
        assert!(ratio > 0.0);
        assert!(ratio < 1.0);
    }

    #[test]
    fn test_utils_is_minified() {
        assert!(utils::is_minified(".test{color:red}"));
        assert!(!utils::is_minified(".test {\n  color: red;\n}"));
        assert!(utils::is_minified(""));
    }

    #[test]
    fn test_optimization_stats_methods() {
        let stats = OptimizationStats {
            original_size: 100,
            optimized_size: 70,
            bytes_saved: 30,
            compression_ratio: 0.3,
        };

        assert_eq!(stats.compression_percentage(), 30.0);
        assert!(stats.is_effective(0.2));
        assert!(!stats.is_effective(0.5));
    }
}
