//! # 字体系统模块
//!
//! 提供通用的字体管理功能，包括字体族、字体大小、行高等。

use crate::theme::core::token::{
    DimensionUnit, DimensionValue, TokenDefinitions, TokenMetadata, TokenReference, TokenValue,
};
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

/// 字体系统
#[derive(Debug, Clone)]
pub struct TypographySystem {
    font_sizes: Vec<DimensionValue>,
    line_heights: Vec<DimensionValue>,
    font_weights: Vec<i32>,
    font_families: Vec<String>,
}

impl TypographySystem {
    /// 创建新的字体系统
    pub fn new() -> Self {
        Self {
            font_sizes: Vec::new(),
            line_heights: Vec::new(),
            font_weights: Vec::new(),
            font_families: Vec::new(),
        }
    }

    /// 获取字体大小
    pub fn get_font_size(&self, path: &str) -> Option<String> {
        let index = path.parse::<usize>().ok()?;
        self.font_sizes.get(index).map(|s| s.to_string())
    }

    /// 获取字体族
    pub fn get_font_family(&self, path: &str) -> Option<String> {
        let index = path.parse::<usize>().ok()?;
        self.font_families.get(index).cloned()
    }

    /// 设置字体大小
    pub fn set_font_size(&mut self, path: String, size: f64, unit: String) {
        let unit = match unit.as_str() {
            "px" => DimensionUnit::Px,
            "rem" => DimensionUnit::Rem,
            "em" => DimensionUnit::Em,
            _ => return,
        };
        let value = DimensionValue::new(size, unit);
        if let Ok(index) = path.parse::<usize>() {
            if index < self.font_sizes.len() {
                self.font_sizes[index] = value;
            } else {
                self.font_sizes.push(value);
            }
        }
    }

    /// 设置字体族
    pub fn set_font_family(&mut self, path: String, family: String) {
        if let Ok(index) = path.parse::<usize>() {
            if index < self.font_families.len() {
                self.font_families[index] = family;
            } else {
                self.font_families.push(family);
            }
        }
    }

    /// 添加字体大小
    pub fn add_font_size(&mut self, size: DimensionValue) {
        self.font_sizes.push(size);
    }

    /// 添加行高
    pub fn add_line_height(&mut self, height: DimensionValue) {
        self.line_heights.push(height);
    }

    /// 添加字重
    pub fn add_font_weight(&mut self, weight: i32) {
        self.font_weights.push(weight);
    }

    /// 添加字体族
    pub fn add_font_family(&mut self, family: String) {
        self.font_families.push(family);
    }
}

impl TokenDefinitions for TypographySystem {
    fn get_token_value(&self, path: &str) -> Option<TokenValue> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "font_size" => self.get_font_size(parts[1]).map(|s| TokenValue::String(s)),
            "font_family" => self
                .get_font_family(parts[1])
                .map(|s| TokenValue::String(s)),
            _ => None,
        }
    }

    fn set_token_value(&mut self, path: &str, value: TokenValue) {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return;
        }

        match (parts[0], value) {
            ("font_size", TokenValue::Dimension(size)) => {
                self.set_font_size(parts[1].to_string(), size.value, size.unit.to_string());
            }
            ("font_family", TokenValue::String(family)) => {
                self.set_font_family(parts[1].to_string(), family);
            }
            _ => {}
        }
    }

    fn get_metadata(&self, _path: &str) -> Option<TokenMetadata> {
        None
    }
}

impl Default for TypographySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FontSystem {
    fn default() -> Self {
        let mut font_families = BTreeMap::new();
        font_families.insert("sans".to_string(), "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif".to_string());
        font_families.insert(
            "serif".to_string(),
            "Georgia, Cambria, 'Times New Roman', Times, serif".to_string(),
        );
        font_families.insert(
            "mono".to_string(),
            "SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"
                .to_string(),
        );

        let mut font_sizes = BTreeMap::new();
        font_sizes.insert(
            "xs".to_string(),
            DimensionValue::create(0.75, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "sm".to_string(),
            DimensionValue::create(0.875, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "md".to_string(),
            DimensionValue::create(1.0, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "lg".to_string(),
            DimensionValue::create(1.125, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "xl".to_string(),
            DimensionValue::create(1.25, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "2xl".to_string(),
            DimensionValue::create(1.5, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "3xl".to_string(),
            DimensionValue::create(1.875, DimensionUnit::Rem),
        );
        font_sizes.insert(
            "4xl".to_string(),
            DimensionValue::create(2.25, DimensionUnit::Rem),
        );

        let mut font_weights = BTreeMap::new();
        font_weights.insert("light".to_string(), 300);
        font_weights.insert("normal".to_string(), 400);
        font_weights.insert("medium".to_string(), 500);
        font_weights.insert("semibold".to_string(), 600);
        font_weights.insert("bold".to_string(), 700);

        let mut line_heights = BTreeMap::new();
        line_heights.insert(
            "none".to_string(),
            DimensionValue::create(1.0, DimensionUnit::Em),
        );
        line_heights.insert(
            "tight".to_string(),
            DimensionValue::create(1.25, DimensionUnit::Em),
        );
        line_heights.insert(
            "snug".to_string(),
            DimensionValue::create(1.375, DimensionUnit::Em),
        );
        line_heights.insert(
            "normal".to_string(),
            DimensionValue::create(1.5, DimensionUnit::Em),
        );
        line_heights.insert(
            "relaxed".to_string(),
            DimensionValue::create(1.625, DimensionUnit::Em),
        );
        line_heights.insert(
            "loose".to_string(),
            DimensionValue::create(2.0, DimensionUnit::Em),
        );

        let mut letter_spacings = BTreeMap::new();
        letter_spacings.insert(
            "tighter".to_string(),
            DimensionValue::create(-0.05, DimensionUnit::Em),
        );
        letter_spacings.insert(
            "tight".to_string(),
            DimensionValue::create(-0.025, DimensionUnit::Em),
        );
        letter_spacings.insert(
            "normal".to_string(),
            DimensionValue::create(0.0, DimensionUnit::Em),
        );
        letter_spacings.insert(
            "wide".to_string(),
            DimensionValue::create(0.025, DimensionUnit::Em),
        );
        letter_spacings.insert(
            "wider".to_string(),
            DimensionValue::create(0.05, DimensionUnit::Em),
        );
        letter_spacings.insert(
            "widest".to_string(),
            DimensionValue::create(0.1, DimensionUnit::Em),
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
            h1: TokenReference::create("typography.headings.h1".to_string()),
            h2: TokenReference::create("typography.headings.h2".to_string()),
            h3: TokenReference::create("typography.headings.h3".to_string()),
            h4: TokenReference::create("typography.headings.h4".to_string()),
            h5: TokenReference::create("typography.headings.h5".to_string()),
            h6: TokenReference::create("typography.headings.h6".to_string()),
        }
    }
}

impl Default for BodyTypography {
    fn default() -> Self {
        Self {
            large: TokenReference::create("typography.body.large".to_string()),
            medium: TokenReference::create("typography.body.medium".to_string()),
            small: TokenReference::create("typography.body.small".to_string()),
            xs: TokenReference::create("typography.body.xs".to_string()),
        }
    }
}

impl Default for LabelTypography {
    fn default() -> Self {
        Self {
            large: TokenReference::create("typography.labels.large".to_string()),
            medium: TokenReference::create("typography.labels.medium".to_string()),
            small: TokenReference::create("typography.labels.small".to_string()),
        }
    }
}

impl Default for CodeTypography {
    fn default() -> Self {
        Self {
            inline: TokenReference::create("typography.code.inline".to_string()),
            block: TokenReference::create("typography.code.block".to_string()),
        }
    }
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

/// 语义间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSpacing {
    /// 组件间距
    pub component: ComponentSpacing,
    /// 布局间距
    pub layout: LayoutSpacing,
    /// 内容间距
    pub content: ContentSpacing,
}

/// 组件间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    /// 超小间距
    pub xs: TokenReference,
    /// 小间距
    pub sm: TokenReference,
    /// 中等间距
    pub md: TokenReference,
    /// 大间距
    pub lg: TokenReference,
    /// 超大间距
    pub xl: TokenReference,
}

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

impl Default for SpacingSystem {
    fn default() -> Self {
        let base_unit = DimensionValue::new(4.0, DimensionUnit::Px);

        let mut scale = BTreeMap::new();
        scale.insert("0".to_string(), DimensionValue::new(0.0, DimensionUnit::Px));
        scale.insert("1".to_string(), DimensionValue::new(4.0, DimensionUnit::Px));
        scale.insert("2".to_string(), DimensionValue::new(8.0, DimensionUnit::Px));
        scale.insert(
            "3".to_string(),
            DimensionValue::new(12.0, DimensionUnit::Px),
        );
        scale.insert(
            "4".to_string(),
            DimensionValue::new(16.0, DimensionUnit::Px),
        );
        scale.insert(
            "5".to_string(),
            DimensionValue::new(20.0, DimensionUnit::Px),
        );
        scale.insert(
            "6".to_string(),
            DimensionValue::new(24.0, DimensionUnit::Px),
        );
        scale.insert(
            "8".to_string(),
            DimensionValue::new(32.0, DimensionUnit::Px),
        );
        scale.insert(
            "10".to_string(),
            DimensionValue::new(40.0, DimensionUnit::Px),
        );
        scale.insert(
            "12".to_string(),
            DimensionValue::new(48.0, DimensionUnit::Px),
        );
        scale.insert(
            "16".to_string(),
            DimensionValue::new(64.0, DimensionUnit::Px),
        );
        scale.insert(
            "20".to_string(),
            DimensionValue::new(80.0, DimensionUnit::Px),
        );
        scale.insert(
            "24".to_string(),
            DimensionValue::new(96.0, DimensionUnit::Px),
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
            xs: TokenReference::create("global.spacing_system.scale.1".to_string()),
            sm: TokenReference::create("global.spacing_system.scale.2".to_string()),
            md: TokenReference::create("global.spacing_system.scale.4".to_string()),
            lg: TokenReference::create("global.spacing_system.scale.6".to_string()),
            xl: TokenReference::create("global.spacing_system.scale.8".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            page_margin: TokenReference::create("global.spacing_system.scale.6".to_string()),
            container_gap: TokenReference::create("global.spacing_system.scale.8".to_string()),
            grid_gap: TokenReference::create("global.spacing_system.scale.4".to_string()),
        }
    }
}

impl Default for ContentSpacing {
    fn default() -> Self {
        Self {
            paragraph: TokenReference::create("global.spacing_system.scale.4".to_string()),
            list_item: TokenReference::create("global.spacing_system.scale.2".to_string()),
            table_cell: TokenReference::create("global.spacing_system.scale.3".to_string()),
        }
    }
}

impl TokenDefinitions for FontSystem {
    fn get_token_value(&self, path: &str) -> Option<TokenValue> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return None;
        }

        match parts[0] {
            "font_size" => self
                .font_sizes
                .get(parts[1])
                .map(|v| TokenValue::Dimension(v.clone())),
            "font_family" => self
                .font_families
                .get(parts[1])
                .map(|s| TokenValue::String(s.clone())),
            "font_weight" => self
                .font_weights
                .get(parts[1])
                .map(|w| TokenValue::Number(*w as f64)),
            "line_height" => self
                .line_heights
                .get(parts[1])
                .map(|v| TokenValue::Dimension(v.clone())),
            "letter_spacing" => self
                .letter_spacings
                .get(parts[1])
                .map(|v| TokenValue::Dimension(v.clone())),
            _ => None,
        }
    }

    fn set_token_value(&mut self, path: &str, value: TokenValue) {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() < 2 {
            return;
        }

        match (parts[0], value) {
            ("font_size", TokenValue::Dimension(v)) => {
                self.font_sizes.insert(parts[1].to_string(), v);
            }
            ("font_family", TokenValue::String(s)) => {
                self.font_families.insert(parts[1].to_string(), s);
            }
            ("font_weight", TokenValue::Number(n)) => {
                self.font_weights.insert(parts[1].to_string(), n as u16);
            }
            ("line_height", TokenValue::Dimension(v)) => {
                self.line_heights.insert(parts[1].to_string(), v);
            }
            ("letter_spacing", TokenValue::Dimension(v)) => {
                self.letter_spacings.insert(parts[1].to_string(), v);
            }
            _ => {}
        }
    }

    fn get_metadata(&self, _path: &str) -> Option<TokenMetadata> {
        None
    }
}
