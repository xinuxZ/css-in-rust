//! # 颜色系统模块
//!
//! 提供通用的颜色管理功能，包括颜色调色板、语义颜色映射等。

use super::token_definitions::{ColorValue, ThemeVariant, TokenReference};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 颜色调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// 主色调
    pub primary: BTreeMap<String, ColorValue>,
    /// 中性色
    pub neutral: BTreeMap<String, ColorValue>,
    /// 成功色
    pub success: BTreeMap<String, ColorValue>,
    /// 警告色
    pub warning: BTreeMap<String, ColorValue>,
    /// 错误色
    pub error: BTreeMap<String, ColorValue>,
    /// 信息色
    pub info: BTreeMap<String, ColorValue>,
}

/// 语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticColors {
    /// 文本颜色
    pub text: TextColors,
    /// 背景颜色
    pub background: BackgroundColors,
    /// 边框颜色
    pub border: BorderColors,
    /// 状态颜色
    pub status: StatusColors,
}

/// 文本颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    /// 主要文本
    pub primary: TokenReference,
    /// 次要文本
    pub secondary: TokenReference,
    /// 禁用文本
    pub disabled: TokenReference,
    /// 占位符文本
    pub placeholder: TokenReference,
}

/// 背景颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    /// 主要背景
    pub primary: TokenReference,
    /// 次要背景
    pub secondary: TokenReference,
    /// 悬停背景
    pub hover: TokenReference,
    /// 激活背景
    pub active: TokenReference,
}

/// 边框颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    /// 默认边框
    pub default: TokenReference,
    /// 悬停边框
    pub hover: TokenReference,
    /// 焦点边框
    pub focus: TokenReference,
    /// 错误边框
    pub error: TokenReference,
}

/// 状态颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusColors {
    /// 成功状态
    pub success: TokenReference,
    /// 警告状态
    pub warning: TokenReference,
    /// 错误状态
    pub error: TokenReference,
    /// 信息状态
    pub info: TokenReference,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: BTreeMap::new(),
            neutral: BTreeMap::new(),
            success: BTreeMap::new(),
            warning: BTreeMap::new(),
            error: BTreeMap::new(),
            info: BTreeMap::new(),
        }
    }
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            text: TextColors::default(),
            background: BackgroundColors::default(),
            border: BorderColors::default(),
            status: StatusColors::default(),
        }
    }
}

impl Default for TextColors {
    fn default() -> Self {
        Self {
            primary: TokenReference::new("global.color_palette.neutral.900".to_string()),
            secondary: TokenReference::new("global.color_palette.neutral.600".to_string()),
            disabled: TokenReference::new("global.color_palette.neutral.400".to_string()),
            placeholder: TokenReference::new("global.color_palette.neutral.500".to_string()),
        }
    }
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            primary: TokenReference::new("global.color_palette.neutral.50".to_string()),
            secondary: TokenReference::new("global.color_palette.neutral.100".to_string()),
            hover: TokenReference::new("global.color_palette.neutral.200".to_string()),
            active: TokenReference::new("global.color_palette.neutral.300".to_string()),
        }
    }
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            default: TokenReference::new("global.color_palette.neutral.300".to_string()),
            hover: TokenReference::new("global.color_palette.neutral.400".to_string()),
            focus: TokenReference::new("global.color_palette.primary.500".to_string()),
            error: TokenReference::new("global.color_palette.error.500".to_string()),
        }
    }
}

impl Default for StatusColors {
    fn default() -> Self {
        Self {
            success: TokenReference::new("global.color_palette.success.500".to_string()),
            warning: TokenReference::new("global.color_palette.warning.500".to_string()),
            error: TokenReference::new("global.color_palette.error.500".to_string()),
            info: TokenReference::new("global.color_palette.info.500".to_string()),
        }
    }
}

/// 颜色系统工具函数
impl ColorPalette {
    /// 应用浅色主题颜色
    pub fn apply_light_theme(&mut self) {
        // 设置浅色主题的主色调（使用通用蓝色方案）
        self.primary
            .insert("50".to_string(), ColorValue::new("#e6f3ff".to_string()));
        self.primary
            .insert("100".to_string(), ColorValue::new("#b3d9ff".to_string()));
        self.primary
            .insert("200".to_string(), ColorValue::new("#80bfff".to_string()));
        self.primary
            .insert("300".to_string(), ColorValue::new("#4da6ff".to_string()));
        self.primary
            .insert("400".to_string(), ColorValue::new("#1a8cff".to_string()));
        self.primary
            .insert("500".to_string(), ColorValue::new("#0066cc".to_string()));
        self.primary
            .insert("600".to_string(), ColorValue::new("#0052a3".to_string()));
        self.primary
            .insert("700".to_string(), ColorValue::new("#003d7a".to_string()));
        self.primary
            .insert("800".to_string(), ColorValue::new("#002952".to_string()));
        self.primary
            .insert("900".to_string(), ColorValue::new("#001429".to_string()));

        // 设置浅色主题的中性色
        self.neutral
            .insert("50".to_string(), ColorValue::new("#fafafa".to_string()));
        self.neutral
            .insert("100".to_string(), ColorValue::new("#f5f5f5".to_string()));
        self.neutral
            .insert("200".to_string(), ColorValue::new("#e8e8e8".to_string()));
        self.neutral
            .insert("300".to_string(), ColorValue::new("#d1d1d1".to_string()));
        self.neutral
            .insert("400".to_string(), ColorValue::new("#b4b4b4".to_string()));
        self.neutral
            .insert("500".to_string(), ColorValue::new("#9b9b9b".to_string()));
        self.neutral
            .insert("600".to_string(), ColorValue::new("#6b6b6b".to_string()));
        self.neutral
            .insert("700".to_string(), ColorValue::new("#515151".to_string()));
        self.neutral
            .insert("800".to_string(), ColorValue::new("#3b3b3b".to_string()));
        self.neutral
            .insert("900".to_string(), ColorValue::new("#262626".to_string()));
    }

    /// 应用深色主题颜色
    pub fn apply_dark_theme(&mut self) {
        // 设置深色主题的主色调
        self.primary
            .insert("50".to_string(), ColorValue::new("#001429".to_string()));
        self.primary
            .insert("100".to_string(), ColorValue::new("#002952".to_string()));
        self.primary
            .insert("200".to_string(), ColorValue::new("#003d7a".to_string()));
        self.primary
            .insert("300".to_string(), ColorValue::new("#0052a3".to_string()));
        self.primary
            .insert("400".to_string(), ColorValue::new("#0066cc".to_string()));
        self.primary
            .insert("500".to_string(), ColorValue::new("#1a8cff".to_string()));
        self.primary
            .insert("600".to_string(), ColorValue::new("#4da6ff".to_string()));
        self.primary
            .insert("700".to_string(), ColorValue::new("#80bfff".to_string()));
        self.primary
            .insert("800".to_string(), ColorValue::new("#b3d9ff".to_string()));
        self.primary
            .insert("900".to_string(), ColorValue::new("#e6f3ff".to_string()));

        // 设置深色主题的中性色
        self.neutral
            .insert("50".to_string(), ColorValue::new("#262626".to_string()));
        self.neutral
            .insert("100".to_string(), ColorValue::new("#3b3b3b".to_string()));
        self.neutral
            .insert("200".to_string(), ColorValue::new("#515151".to_string()));
        self.neutral
            .insert("300".to_string(), ColorValue::new("#6b6b6b".to_string()));
        self.neutral
            .insert("400".to_string(), ColorValue::new("#9b9b9b".to_string()));
        self.neutral
            .insert("500".to_string(), ColorValue::new("#b4b4b4".to_string()));
        self.neutral
            .insert("600".to_string(), ColorValue::new("#d1d1d1".to_string()));
        self.neutral
            .insert("700".to_string(), ColorValue::new("#e8e8e8".to_string()));
        self.neutral
            .insert("800".to_string(), ColorValue::new("#f5f5f5".to_string()));
        self.neutral
            .insert("900".to_string(), ColorValue::new("#fafafa".to_string()));
    }
}

impl SemanticColors {
    /// 更新语义颜色以适应指定主题
    pub fn update_for_theme(&mut self, theme: ThemeVariant) {
        match theme {
            ThemeVariant::Light => {
                self.text.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
                self.text.secondary =
                    TokenReference::new("global.color_palette.neutral.600".to_string());
                self.background.primary =
                    TokenReference::new("global.color_palette.neutral.50".to_string());
            }
            ThemeVariant::Dark => {
                self.text.primary =
                    TokenReference::new("global.color_palette.neutral.100".to_string());
                self.text.secondary =
                    TokenReference::new("global.color_palette.neutral.400".to_string());
                self.background.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
            }
            ThemeVariant::Auto => {
                // Auto模式使用浅色主题设置
                self.text.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
                self.background.primary =
                    TokenReference::new("global.color_palette.neutral.50".to_string());
            }
        }
    }
}
