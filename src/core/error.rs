//! Error types for CSS-in-Rust
//!
//! This module defines all error types used throughout the library.

use thiserror::Error;

/// Main error type for CSS-in-Rust operations
#[derive(Error, Debug, Clone)]
pub enum CssError {
    /// CSS parsing failed
    #[error("CSS parsing failed: {message}")]
    ParseError {
        /// Error message from the parser
        message: String,
        /// Line number where the error occurred (if available)
        line: Option<u32>,
        /// Column number where the error occurred (if available)
        column: Option<u32>,
    },

    /// Style injection failed
    #[error("Style injection failed: {0}")]
    InjectionError(String),

    /// Theme-related error
    #[error("Theme error: {0}")]
    ThemeError(String),

    /// Optimization error
    #[error("CSS optimization failed: {0}")]
    OptimizationError(String),

    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Feature not supported
    #[error("Feature not supported: {0}")]
    UnsupportedFeature(String),
}

impl CssError {
    /// Create a new parse error with location information
    pub fn parse_error(message: impl Into<String>, line: Option<u32>, column: Option<u32>) -> Self {
        Self::ParseError {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a new injection error
    pub fn injection_error(message: impl Into<String>) -> Self {
        Self::InjectionError(message.into())
    }

    /// Create a new theme error
    pub fn theme_error(message: impl Into<String>) -> Self {
        Self::ThemeError(message.into())
    }

    /// Create a new optimization error
    pub fn optimization_error(message: impl Into<String>) -> Self {
        Self::OptimizationError(message.into())
    }

    /// Create a new runtime error
    pub fn runtime_error(message: impl Into<String>) -> Self {
        Self::RuntimeError(message.into())
    }

    /// Check if this is a parse error
    pub fn is_parse_error(&self) -> bool {
        matches!(self, Self::ParseError { .. })
    }

    /// Check if this is an injection error
    pub fn is_injection_error(&self) -> bool {
        matches!(self, Self::InjectionError(_))
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Self::ParseError { message, .. } => message,
            Self::InjectionError(msg) => msg,
            Self::ThemeError(msg) => msg,
            Self::OptimizationError(msg) => msg,
            Self::RuntimeError(msg) => msg,
            Self::IoError(msg) => msg,
            Self::ConfigError(msg) => msg,
            Self::UnsupportedFeature(msg) => msg,
        }
    }
}

// Convert from lightningcss errors
impl From<lightningcss::error::Error<lightningcss::error::ParserError<'_>>> for CssError {
    fn from(err: lightningcss::error::Error<lightningcss::error::ParserError<'_>>) -> Self {
        Self::parse_error(
            format!("LightningCSS error: {:?}", err.kind),
            err.loc.as_ref().map(|loc| loc.line),
            err.loc.as_ref().map(|loc| loc.column),
        )
    }
}

// Convert from lightningcss printer errors
impl From<lightningcss::error::Error<lightningcss::error::PrinterError>> for CssError {
    fn from(err: lightningcss::error::Error<lightningcss::error::PrinterError>) -> Self {
        Self::optimization_error(format!("LightningCSS printer error: {:?}", err.kind))
    }
}

// Convert from std::io::Error
impl From<std::io::Error> for CssError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

/// Result type alias for CSS-in-Rust operations
pub type Result<T> = std::result::Result<T, CssError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = CssError::parse_error("test error", Some(10), Some(5));
        assert!(err.is_parse_error());
        assert_eq!(err.message(), "test error");
    }

    #[test]
    fn test_injection_error() {
        let err = CssError::injection_error("injection failed");
        assert!(err.is_injection_error());
        assert_eq!(err.message(), "injection failed");
    }

    #[test]
    fn test_error_display() {
        let err = CssError::parse_error("syntax error", Some(1), Some(1));
        let display = format!("{}", err);
        assert!(display.contains("CSS parsing failed"));
        assert!(display.contains("syntax error"));
    }
}
