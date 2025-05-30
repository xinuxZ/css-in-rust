//! Core CSS processing functionality
//!
//! This module contains the fundamental components for CSS parsing,
//! optimization, and error handling.

pub mod error;
pub mod optimizer;
pub mod parser;

pub use error::{CssError, Result};
pub use optimizer::CssOptimizer;
pub use parser::CssParser;

/// CSS processing pipeline
///
/// Combines parsing and optimization into a single convenient interface.
pub struct CssProcessor {
    parser: CssParser,
    optimizer: CssOptimizer,
}

impl CssProcessor {
    /// Create a new CSS processor with default settings
    pub fn new() -> Self {
        Self {
            parser: CssParser::new(),
            optimizer: CssOptimizer::new(),
        }
    }

    /// Process CSS string through the complete pipeline
    ///
    /// This method parses the CSS, validates it, and applies optimizations.
    pub fn process(&self, css: &str) -> Result<String> {
        let stylesheet = self.parser.parse(css)?;
        let optimized = self.optimizer.optimize(stylesheet)?;
        Ok(optimized)
    }

    /// Validate CSS without optimization
    pub fn validate(&self, css: &str) -> Result<()> {
        self.parser.parse(css)?;
        Ok(())
    }
}

impl Default for CssProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_processor_creation() {
        let processor = CssProcessor::new();
        assert!(processor.validate(".test { color: red; }").is_ok());
    }

    #[test]
    fn test_css_processor_default() {
        let processor = CssProcessor::default();
        assert!(processor.validate(".test { color: blue; }").is_ok());
    }

    #[test]
    fn test_invalid_css() {
        let processor = CssProcessor::new();
        assert!(processor.validate(".test { color: ; }").is_err());
    }
}
