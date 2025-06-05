//! 语义系统模块
//!
//! 提供语义间距、尺寸等系统的定义和管理功能。
//! 这些系统将基础令牌映射为具有语义意义的设计令牌。

use super::token_system::TokenReference;
use crate::theme::core::token::{TokenSystem, TokenValue};
use serde::{Deserialize, Serialize};

/// 语义系统
pub struct SemanticSystem {
    token_system: TokenSystem,
}

impl SemanticSystem {
    /// 创建新的语义系统
    pub fn new(token_system: TokenSystem) -> Self {
        Self { token_system }
    }

    /// 获取语义值
    pub fn get_value(&self, path: &str) -> Option<String> {
        self.token_system
            .get_value(path)
            .map(|value| value.to_string())
    }

    /// 设置语义值
    pub fn set_value(&mut self, path: String, value: TokenValue) {
        self.token_system.set_value(path, value);
    }
}

/// 语义间距系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSpacing {
    /// 组件间距
    pub component: ComponentSpacing,
    /// 内边距
    pub padding: PaddingSpacing,
    /// 外边距
    pub margin: MarginSpacing,
    /// 间隙
    pub gap: GapSpacing,
    /// 布局间距
    pub layout: LayoutSpacing,
}

/// 组件间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 内边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 外边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 间隙
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 布局间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSpacing {
    pub container: TokenReference,
    pub section: TokenReference,
    pub content: TokenReference,
}

/// 语义尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSizing {
    /// 组件尺寸
    pub component: ComponentSizing,
    /// 图标尺寸
    pub icon: IconSizing,
    /// 头像尺寸
    pub avatar: AvatarSizing,
}

/// 组件尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 图标尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 头像尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

impl Default for SemanticSpacing {
    fn default() -> Self {
        Self {
            component: ComponentSpacing::default(),
            padding: PaddingSpacing::default(),
            margin: MarginSpacing::default(),
            gap: GapSpacing::default(),
            layout: LayoutSpacing::default(),
        }
    }
}

impl Default for ComponentSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("spacing.xs".to_string()),
            sm: TokenReference::Global("spacing.sm".to_string()),
            md: TokenReference::Global("spacing.md".to_string()),
            lg: TokenReference::Global("spacing.lg".to_string()),
            xl: TokenReference::Global("spacing.xl".to_string()),
        }
    }
}

impl Default for PaddingSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("spacing.xs".to_string()),
            sm: TokenReference::Global("spacing.sm".to_string()),
            md: TokenReference::Global("spacing.md".to_string()),
            lg: TokenReference::Global("spacing.lg".to_string()),
            xl: TokenReference::Global("spacing.xl".to_string()),
        }
    }
}

impl Default for MarginSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("spacing.xs".to_string()),
            sm: TokenReference::Global("spacing.sm".to_string()),
            md: TokenReference::Global("spacing.md".to_string()),
            lg: TokenReference::Global("spacing.lg".to_string()),
            xl: TokenReference::Global("spacing.xl".to_string()),
        }
    }
}

impl Default for GapSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("spacing.xs".to_string()),
            sm: TokenReference::Global("spacing.sm".to_string()),
            md: TokenReference::Global("spacing.md".to_string()),
            lg: TokenReference::Global("spacing.lg".to_string()),
            xl: TokenReference::Global("spacing.xl".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            container: TokenReference::Global("spacing.xl".to_string()),
            section: TokenReference::Global("spacing.lg".to_string()),
            content: TokenReference::Global("spacing.md".to_string()),
        }
    }
}

impl Default for SemanticSizing {
    fn default() -> Self {
        Self {
            component: ComponentSizing::default(),
            icon: IconSizing::default(),
            avatar: AvatarSizing::default(),
        }
    }
}

impl Default for ComponentSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("sizing.xs".to_string()),
            sm: TokenReference::Global("sizing.sm".to_string()),
            md: TokenReference::Global("sizing.md".to_string()),
            lg: TokenReference::Global("sizing.lg".to_string()),
            xl: TokenReference::Global("sizing.xl".to_string()),
        }
    }
}

impl Default for IconSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("sizing.icon.xs".to_string()),
            sm: TokenReference::Global("sizing.icon.sm".to_string()),
            md: TokenReference::Global("sizing.icon.md".to_string()),
            lg: TokenReference::Global("sizing.icon.lg".to_string()),
            xl: TokenReference::Global("sizing.icon.xl".to_string()),
        }
    }
}

impl Default for AvatarSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::Global("sizing.avatar.xs".to_string()),
            sm: TokenReference::Global("sizing.avatar.sm".to_string()),
            md: TokenReference::Global("sizing.avatar.md".to_string()),
            lg: TokenReference::Global("sizing.avatar.lg".to_string()),
            xl: TokenReference::Global("sizing.avatar.xl".to_string()),
        }
    }
}
