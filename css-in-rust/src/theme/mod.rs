//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持通用设计体系和动态主题切换。

pub mod core;
pub mod systems;

pub mod theme_types;

pub use core::{
    css::{generator::CssGenerator, variables::CssVariables},
    token::{ThemeVariant, TokenSystem, TokenValue},
};

pub use theme_types::Theme;

// Re-exports from systems
pub use systems::{
    typography::{FontSystem, TypographySystem},
    ColorSystem, SemanticSpacing,
};

impl Theme {
    /// 生成设计令牌的CSS变量
    pub fn generate_design_tokens_css(&self) -> String {
        let mut css = String::from(":root {\n");

        // 添加全局令牌
        css.push_str(&self.token_system.to_css_variables());

        css.push_str("}\n");
        css
    }
}
