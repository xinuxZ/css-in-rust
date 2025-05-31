//! Build Tools Module
//!
//! This module provides build-time tools for CSS analysis and optimization.

pub mod build_script;
pub mod static_analyzer;

pub use static_analyzer::{
    AnalysisMetadata, CssMacroCall, CssSelectors, CssUsageReport, StaticAnalyzer,
};

pub use build_script::{BuildConfig, BuildError, BuildResult, CssBuildProcessor, ProcessedFile};
