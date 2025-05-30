//! Core CSS processing functionality
//!
//! This module provides the fundamental CSS parsing and optimization capabilities.

pub mod optimizer;
pub mod parser;

pub use optimizer::{CssOptimizer, OptimizationError, OptimizerConfig};
pub use parser::{CssParser, ParseError, ParserConfig};

/// Result type for CSS operations
pub type Result<T> = std::result::Result<T, CssError>;

/// Main error type for CSS operations
#[derive(Debug)]
pub enum CssError {
    ParseError(String),
    InjectionError(String),
    ThemeError(String),
    IoError(std::io::Error),
    OptimizationError(String),
}

impl std::fmt::Display for CssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssError::ParseError(msg) => write!(f, "CSS parsing failed: {}", msg),
            CssError::InjectionError(msg) => write!(f, "Style injection failed: {}", msg),
            CssError::ThemeError(msg) => write!(f, "Theme error: {}", msg),
            CssError::IoError(err) => write!(f, "IO error: {}", err),
            CssError::OptimizationError(msg) => write!(f, "Optimization error: {}", msg),
        }
    }
}

impl std::error::Error for CssError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CssError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CssError {
    fn from(err: std::io::Error) -> Self {
        CssError::IoError(err)
    }
}
