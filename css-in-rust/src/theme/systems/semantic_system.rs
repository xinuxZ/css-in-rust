//! # 语义系统模块
//!
//! 提供基于语义的设计令牌，将抽象的设计概念映射到具体的样式值。
//! 语义系统使设计更加一致，并使样式更容易理解和维护。

use crate::theme::core::token::TokenReference;
use serde::{Deserialize, Serialize};

/// 语义化间距
///
/// 定义基于语义的间距系统，将抽象的间距概念（如"小"、"中"、"大"）
/// 映射到具体的间距值。这使得设计系统更加灵活，因为可以在不改变组件代码的情况下
/// 调整整个应用程序的间距。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic_system::SemanticSpacing;
///
/// // 创建默认语义间距
/// let semantic_spacing = SemanticSpacing::default();
///
/// // 使用组件间距
/// // let button_padding = semantic_spacing.component.md;
/// ```
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
///
/// 定义组件内部和组件之间使用的间距。这些间距值通常用于按钮内边距、
/// 卡片内边距、表单元素间距等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic_system::ComponentSpacing;
/// use css_in_rust::theme::core::token::TokenReference;
///
/// // 创建默认组件间距
/// let component_spacing = ComponentSpacing::default();
///
/// // 使用组件间距
/// // let button_padding = component_spacing.md;
/// ```
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
///
/// 定义页面布局中使用的间距。这些间距值通常用于页面边距、
/// 容器间距、栅格间距和区块间距等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic_system::LayoutSpacing;
/// use css_in_rust::theme::core::token::TokenReference;
///
/// // 创建默认布局间距
/// let layout_spacing = LayoutSpacing::default();
///
/// // 使用布局间距
/// // let page_margin = layout_spacing.page_margin;
/// ```
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
///
/// 定义内容元素之间使用的间距。这些间距值通常用于段落间距、
/// 列表项间距、标题间距和内联元素间距等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic_system::ContentSpacing;
/// use css_in_rust::theme::core::token::TokenReference;
///
/// // 创建默认内容间距
/// let content_spacing = ContentSpacing::default();
///
/// // 使用内容间距
/// // let paragraph_margin = content_spacing.paragraph;
/// ```
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
