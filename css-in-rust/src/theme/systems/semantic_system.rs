use crate::theme::core::token::TokenReference;
use serde::{Deserialize, Serialize};

/// 语义化间距
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
    /// 区块间距
    pub section_gap: TokenReference,
}

/// 内容间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSpacing {
    /// 段落间距
    pub paragraph: TokenReference,
    /// 列表项间距
    pub list_item: TokenReference,
    /// 标题间距
    pub heading: TokenReference,
    /// 内联元素间距
    pub inline: TokenReference,
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
            xs: TokenReference::create("spacing.2".to_string()),
            sm: TokenReference::create("spacing.4".to_string()),
            md: TokenReference::create("spacing.6".to_string()),
            lg: TokenReference::create("spacing.8".to_string()),
            xl: TokenReference::create("spacing.12".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            page_margin: TokenReference::create("spacing.6".to_string()),
            container_gap: TokenReference::create("spacing.4".to_string()),
            grid_gap: TokenReference::create("spacing.4".to_string()),
            section_gap: TokenReference::create("spacing.8".to_string()),
        }
    }
}

impl Default for ContentSpacing {
    fn default() -> Self {
        Self {
            paragraph: TokenReference::create("spacing.4".to_string()),
            list_item: TokenReference::create("spacing.2".to_string()),
            heading: TokenReference::create("spacing.6".to_string()),
            inline: TokenReference::create("spacing.1".to_string()),
        }
    }
}
