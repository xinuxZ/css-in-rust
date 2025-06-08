//! # 设计系统模块
//!
//! 提供设计系统的核心组件，包括颜色系统、排版系统、间距系统和语义系统。
//! 这些系统共同构成了一个完整的设计令牌系统，用于确保应用程序的视觉一致性。
//!
//! ## 主要组件
//!
//! - **颜色系统**：管理应用程序中使用的所有颜色，包括主色调、中性色、功能色和扩展色。
//! - **排版系统**：管理字体族、字体大小、字体粗细、行高等排版样式。
//! - **间距系统**：管理基础间距单位、间距比例和语义间距。
//! - **语义系统**：将基础令牌映射为具有语义意义的设计令牌。

pub mod color;
pub mod semantic;
pub mod semantic_system;
pub mod spacing;
pub mod typography;

// Re-exports
pub use color::ColorSystem;
pub use semantic_system::{ComponentSpacing, ContentSpacing, LayoutSpacing, SemanticSpacing};
pub use spacing::SpacingSystem;
pub use typography::{
    BodyTypography, CodeTypography, FontSystem, HeadingTypography, LabelTypography,
    SemanticTypography,
};
