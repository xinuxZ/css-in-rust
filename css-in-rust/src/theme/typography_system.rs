//! # 字体系统模块
//!
//! 提供通用的字体管理功能，包括字体族、字体大小、行高等。

use super::token_definitions::{DimensionValue, TokenReference, TypographyValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 字体系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSystem {
    /// 字体族
    pub font_families: BTreeMap<String, String>,
    /// 字体大小
    pub font_sizes: BTreeMap<String, DimensionValue>,
    /// 字体粗细
    pub font_weights: BTreeMap<String, u16>,
    /// 行高
    pub line_heights: BTreeMap<String, DimensionValue>,
    /// 字间距
    pub letter_spacings: BTreeMap<String, DimensionValue>,
}

/// 语义字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTypography {
    /// 标题字体
    pub headings: HeadingTypography,
    /// 正文字体
    pub body: BodyTypography,
    /// 标签字体
    pub labels: LabelTypography,
    /// 代码字体
    pub code: CodeTypography,
}

/// 标题字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingTypography {
    /// H1 标题
    pub h1: TokenReference,
    /// H2 标题
    pub h2: TokenReference,
    /// H3 标题
    pub h3: TokenReference,
    /// H4 标题
    pub h4: TokenReference,
    /// H5 标题
    pub h5: TokenReference,
    /// H6 标题
    pub h6: TokenReference,
}

/// 正文字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyTypography {
    /// 大号正文
    pub large: TokenReference,
    /// 标准正文
    pub medium: TokenReference,
    /// 小号正文
    pub small: TokenReference,
    /// 超小号正文
    pub xs: TokenReference,
}

/// 标签字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelTypography {
    /// 大号标签
    pub large: TokenReference,
    /// 标准标签
    pub medium: TokenReference,
    /// 小号标签
    pub small: TokenReference,
}

/// 代码字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTypography {
    /// 内联代码
    pub inline: TokenReference,
    /// 代码块
    pub block: TokenReference,
}

/// 间距系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    /// 基础间距单位
    pub base_unit: DimensionValue,
    /// 间距比例
    pub scale: BTreeMap<String, DimensionValue>,
    /// 语义间距
    pub semantic: SemanticSpacing,
}

// SemanticSpacing 现在在 semantic_system.rs 模块中定义

/// 布局间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSpacing {
    /// 页面边距
    pub page_margin: TokenReference,
    /// 容器间距
    pub container_gap: TokenReference,
    /// 栅格间距
    pub grid_gap: TokenReference,
}

/// 内容间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSpacing {
    /// 段落间距
    pub paragraph: TokenReference,
    /// 列表项间距
    pub list_item: TokenReference,
    /// 表格单元格间距
    pub table_cell: TokenReference,
}

impl Default for FontSystem {
    fn default() -> Self {
        let mut font_families = BTreeMap::new();
        font_families.insert(
            "sans".to_string(),
            "system-ui, -apple-system, sans-serif".to_string(),
        );
        font_families.insert("serif".to_string(), "Georgia, serif".to_string());
        font_families.insert(
            "mono".to_string(),
            "'SF Mono', Monaco, monospace".to_string(),
        );

        let mut font_sizes = BTreeMap::new();
        font_sizes.insert(
            "xs".to_string(),
            DimensionValue::new(12.0, super::token_definitions::DimensionUnit::Px),
        );
        font_sizes.insert(
            "sm".to_string(),
            DimensionValue::new(14.0, super::token_definitions::DimensionUnit::Px),
        );
        font_sizes.insert(
            "md".to_string(),
            DimensionValue::new(16.0, super::token_definitions::DimensionUnit::Px),
        );
        font_sizes.insert(
            "lg".to_string(),
            DimensionValue::new(18.0, super::token_definitions::DimensionUnit::Px),
        );
        font_sizes.insert(
            "xl".to_string(),
            DimensionValue::new(20.0, super::token_definitions::DimensionUnit::Px),
        );

        let mut font_weights = BTreeMap::new();
        font_weights.insert("light".to_string(), 300);
        font_weights.insert("normal".to_string(), 400);
        font_weights.insert("medium".to_string(), 500);
        font_weights.insert("semibold".to_string(), 600);
        font_weights.insert("bold".to_string(), 700);

        let mut line_heights = BTreeMap::new();
        line_heights.insert(
            "tight".to_string(),
            DimensionValue::new(1.2, super::token_definitions::DimensionUnit::Em),
        );
        line_heights.insert(
            "normal".to_string(),
            DimensionValue::new(1.5, super::token_definitions::DimensionUnit::Em),
        );
        line_heights.insert(
            "relaxed".to_string(),
            DimensionValue::new(1.75, super::token_definitions::DimensionUnit::Em),
        );

        let mut letter_spacings = BTreeMap::new();
        letter_spacings.insert(
            "tight".to_string(),
            DimensionValue::new(-0.025, super::token_definitions::DimensionUnit::Em),
        );
        letter_spacings.insert(
            "normal".to_string(),
            DimensionValue::new(0.0, super::token_definitions::DimensionUnit::Em),
        );
        letter_spacings.insert(
            "wide".to_string(),
            DimensionValue::new(0.025, super::token_definitions::DimensionUnit::Em),
        );

        Self {
            font_families,
            font_sizes,
            font_weights,
            line_heights,
            letter_spacings,
        }
    }
}

impl Default for SpacingSystem {
    fn default() -> Self {
        let base_unit = DimensionValue::new(4.0, super::token_definitions::DimensionUnit::Px);

        let mut scale = BTreeMap::new();
        scale.insert(
            "0".to_string(),
            DimensionValue::new(0.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "1".to_string(),
            DimensionValue::new(4.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "2".to_string(),
            DimensionValue::new(8.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "3".to_string(),
            DimensionValue::new(12.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "4".to_string(),
            DimensionValue::new(16.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "5".to_string(),
            DimensionValue::new(20.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "6".to_string(),
            DimensionValue::new(24.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "8".to_string(),
            DimensionValue::new(32.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "10".to_string(),
            DimensionValue::new(40.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "12".to_string(),
            DimensionValue::new(48.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "16".to_string(),
            DimensionValue::new(64.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "20".to_string(),
            DimensionValue::new(80.0, super::token_definitions::DimensionUnit::Px),
        );
        scale.insert(
            "24".to_string(),
            DimensionValue::new(96.0, super::token_definitions::DimensionUnit::Px),
        );

        Self {
            base_unit,
            scale,
            semantic: SemanticSpacing::default(),
        }
    }
}

impl Default for SemanticSpacing {
    fn default() -> Self {
        Self {
            component: ComponentSpacing::default(),
            layout: LayoutSpacing::default(),
            content: ContentSpacing::default(),
        }
    }
}

impl Default for ComponentSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("global.spacing_system.scale.1".to_string()),
            sm: TokenReference::new("global.spacing_system.scale.2".to_string()),
            md: TokenReference::new("global.spacing_system.scale.4".to_string()),
            lg: TokenReference::new("global.spacing_system.scale.6".to_string()),
            xl: TokenReference::new("global.spacing_system.scale.8".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            page_margin: TokenReference::new("global.spacing_system.scale.6".to_string()),
            container_gap: TokenReference::new("global.spacing_system.scale.8".to_string()),
            grid_gap: TokenReference::new("global.spacing_system.scale.4".to_string()),
        }
    }
}

impl Default for ContentSpacing {
    fn default() -> Self {
        Self {
            paragraph: TokenReference::new("global.spacing_system.scale.4".to_string()),
            list_item: TokenReference::new("global.spacing_system.scale.2".to_string()),
            table_cell: TokenReference::new("global.spacing_system.scale.3".to_string()),
        }
    }
}

impl Default for SemanticTypography {
    fn default() -> Self {
        Self {
            headings: HeadingTypography::default(),
            body: BodyTypography::default(),
            labels: LabelTypography::default(),
            code: CodeTypography::default(),
        }
    }
}

impl Default for HeadingTypography {
    fn default() -> Self {
        Self {
            h1: TokenReference::new("global.font_system.font_sizes.xl".to_string()),
            h2: TokenReference::new("global.font_system.font_sizes.lg".to_string()),
            h3: TokenReference::new("global.font_system.font_sizes.md".to_string()),
            h4: TokenReference::new("global.font_system.font_sizes.md".to_string()),
            h5: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
            h6: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
        }
    }
}

impl Default for BodyTypography {
    fn default() -> Self {
        Self {
            large: TokenReference::new("global.font_system.font_sizes.lg".to_string()),
            medium: TokenReference::new("global.font_system.font_sizes.md".to_string()),
            small: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
            xs: TokenReference::new("global.font_system.font_sizes.xs".to_string()),
        }
    }
}

impl Default for LabelTypography {
    fn default() -> Self {
        Self {
            large: TokenReference::new("global.font_system.font_sizes.md".to_string()),
            medium: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
            small: TokenReference::new("global.font_system.font_sizes.xs".to_string()),
        }
    }
}

impl Default for CodeTypography {
    fn default() -> Self {
        Self {
            inline: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
            block: TokenReference::new("global.font_system.font_sizes.sm".to_string()),
        }
    }
}
