//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持通用设计体系和动态主题切换。

pub mod adapter;
pub mod core;
pub mod systems;
#[cfg(test)]
mod tests;
pub mod theme_types;

// Re-exports
pub use core::token::{
    definitions::{
        DimensionUnit, DimensionValue, TokenDefinitions, TokenDefinitionsImpl, TokenMetadata,
    },
    values::TokenValues,
};

pub use core::{
    cache::{ComponentStyleCache, StyleCache},
    css::{dependency::DependencyTracker, generator::CssGenerator, variables::CssVariables},
    manager::ThemeManager,
    optimize::{OptimizeConfig, StyleOptimizer},
};

pub use adapter::{
    frameworks::{DioxusAdapter, ReactAdapter},
    injection::StyleInjector,
    provider::ThemeProviderAdapter,
    ssr::SsrSupport,
};

pub use theme_types::{Theme, ThemeMode};

// Re-exports from systems
pub use systems::{
    typography::{FontSystem, TypographySystem},
    ColorSystem, SemanticSpacing,
};

impl Theme {
    /// 生成设计令牌的CSS变量
    pub fn generate_design_tokens_css(&mut self) -> String {
        let mut css = String::new();

        // 添加CSS变量
        css.push_str(":root {\n");
        css.push_str("  --primary-color: #007bff;\n");
        css.push_str("  --secondary-color: #6c757d;\n");
        css.push_str("  --success-color: #28a745;\n");
        css.push_str("  --danger-color: #dc3545;\n");
        css.push_str("  --warning-color: #ffc107;\n");
        css.push_str("  --info-color: #17a2b8;\n");
        css.push_str("  --light-color: #f8f9fa;\n");
        css.push_str("  --dark-color: #343a40;\n");
        css.push_str("}\n");

        css
    }
}
