//! 语义系统模块
//!
//! 提供语义间距、尺寸等系统的定义和管理功能。
//! 这些系统将基础令牌映射为具有语义意义的设计令牌。

use crate::theme::core::token::TokenReference;
use crate::theme::core::token::{TokenSystem, TokenValue};
use serde::{Deserialize, Serialize};

/// 语义系统
///
/// 管理设计系统中的语义令牌，将基础令牌映射为具有特定语义意义的设计令牌。
/// 语义系统使设计更加一致，并使样式更容易理解和维护。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::TokenSystem;
/// use css_in_rust::theme::systems::semantic::SemanticSystem;
///
/// // 创建令牌系统
/// let token_system = TokenSystem::new();
///
/// // 创建语义系统
/// let mut semantic_system = SemanticSystem::new(token_system);
///
/// // 获取语义值
/// if let Some(value) = semantic_system.get_value("spacing.component.md") {
///     println!("组件中等间距: {}", value);
/// }
/// ```
pub struct SemanticSystem {
    token_system: TokenSystem,
}

impl SemanticSystem {
    /// 创建新的语义系统
    ///
    /// 初始化一个新的语义系统，使用提供的令牌系统作为基础。
    ///
    /// # 参数
    ///
    /// * `token_system` - 用于存储和管理令牌的令牌系统
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `SemanticSystem` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::TokenSystem;
    /// use css_in_rust::theme::systems::semantic::SemanticSystem;
    ///
    /// let token_system = TokenSystem::new();
    /// let semantic_system = SemanticSystem::new(token_system);
    /// ```
    pub fn new(token_system: TokenSystem) -> Self {
        Self { token_system }
    }

    /// 获取语义值
    ///
    /// 根据路径获取语义值。
    ///
    /// # 参数
    ///
    /// * `path` - 语义值的路径，例如 "spacing.component.md"
    ///
    /// # 返回值
    ///
    /// 如果找到语义值，则返回 `Some(String)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::{TokenSystem, TokenValue};
    /// use css_in_rust::theme::systems::semantic::SemanticSystem;
    ///
    /// let mut token_system = TokenSystem::new();
    /// token_system.set_value("spacing.component.md".to_string(), TokenValue::String("16px".to_string()));
    ///
    /// let semantic_system = SemanticSystem::new(token_system);
    /// let value = semantic_system.get_value("spacing.component.md");
    /// assert_eq!(value, Some("16px".to_string()));
    /// ```
    pub fn get_value(&self, path: &str) -> Option<String> {
        self.token_system
            .get_value(path)
            .map(|value| value.to_string())
    }

    /// 设置语义值
    ///
    /// 设置指定路径的语义值。
    ///
    /// # 参数
    ///
    /// * `path` - 语义值的路径，例如 "spacing.component.md"
    /// * `value` - 要设置的值
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::{TokenSystem, TokenValue};
    /// use css_in_rust::theme::systems::semantic::SemanticSystem;
    ///
    /// let token_system = TokenSystem::new();
    /// let mut semantic_system = SemanticSystem::new(token_system);
    ///
    /// // 设置组件中等间距
    /// semantic_system.set_value("spacing.component.md".to_string(), TokenValue::String("16px".to_string()));
    ///
    /// // 验证设置的值
    /// assert_eq!(semantic_system.get_value("spacing.component.md"), Some("16px".to_string()));
    /// ```
    pub fn set_value(&mut self, path: String, value: TokenValue) {
        self.token_system.set_value(path, value);
    }
}

/// 语义间距系统
///
/// 定义基于语义的间距系统，包括组件间距、内边距、外边距、间隙和布局间距。
/// 这使得设计系统更加灵活，因为可以在不改变组件代码的情况下调整整个应用程序的间距。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::SemanticSpacing;
///
/// // 创建默认语义间距
/// let semantic_spacing = SemanticSpacing::default();
/// ```
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
///
/// 定义组件内部和组件之间使用的间距。这些间距值通常用于按钮内边距、
/// 卡片内边距、表单元素间距等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::ComponentSpacing;
///
/// // 创建默认组件间距
/// let component_spacing = ComponentSpacing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    /// 超小间距，适用于紧凑的UI元素
    pub xs: TokenReference,
    /// 小间距，适用于小型UI元素
    pub sm: TokenReference,
    /// 中等间距，适用于标准UI元素
    pub md: TokenReference,
    /// 大间距，适用于强调的UI元素
    pub lg: TokenReference,
    /// 超大间距，适用于特别突出的UI元素
    pub xl: TokenReference,
}

/// 内边距
///
/// 定义元素内部的间距。这些间距值通常用于容器、卡片、按钮等元素的内边距。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::PaddingSpacing;
///
/// // 创建默认内边距
/// let padding_spacing = PaddingSpacing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingSpacing {
    /// 超小内边距，适用于紧凑的UI元素
    pub xs: TokenReference,
    /// 小内边距，适用于小型UI元素
    pub sm: TokenReference,
    /// 中等内边距，适用于标准UI元素
    pub md: TokenReference,
    /// 大内边距，适用于强调的UI元素
    pub lg: TokenReference,
    /// 超大内边距，适用于特别突出的UI元素
    pub xl: TokenReference,
}

/// 外边距
///
/// 定义元素外部的间距。这些间距值通常用于元素之间的间距。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::MarginSpacing;
///
/// // 创建默认外边距
/// let margin_spacing = MarginSpacing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSpacing {
    /// 超小外边距，适用于紧凑布局
    pub xs: TokenReference,
    /// 小外边距，适用于紧凑布局
    pub sm: TokenReference,
    /// 中等外边距，适用于标准布局
    pub md: TokenReference,
    /// 大外边距，适用于宽松布局
    pub lg: TokenReference,
    /// 超大外边距，适用于特别宽松的布局
    pub xl: TokenReference,
}

/// 间隙
///
/// 定义元素之间的间隙。这些间距值通常用于栅格系统、弹性布局等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::GapSpacing;
///
/// // 创建默认间隙
/// let gap_spacing = GapSpacing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapSpacing {
    /// 超小间隙，适用于紧凑的栅格系统
    pub xs: TokenReference,
    /// 小间隙，适用于紧凑的栅格系统
    pub sm: TokenReference,
    /// 中等间隙，适用于标准栅格系统
    pub md: TokenReference,
    /// 大间隙，适用于宽松的栅格系统
    pub lg: TokenReference,
    /// 超大间隙，适用于特别宽松的栅格系统
    pub xl: TokenReference,
}

/// 布局间距
///
/// 定义页面布局中使用的间距。这些间距值通常用于页面边距、
/// 容器间距和区块间距等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::LayoutSpacing;
///
/// // 创建默认布局间距
/// let layout_spacing = LayoutSpacing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSpacing {
    /// 容器间距，适用于页面主容器的内边距
    pub container: TokenReference,
    /// 区块间距，适用于页面主要区块之间的间距
    pub section: TokenReference,
    /// 内容间距，适用于内容区块内的间距
    pub content: TokenReference,
}

/// 语义尺寸
///
/// 定义基于语义的尺寸系统，包括组件尺寸、图标尺寸和头像尺寸。
/// 这使得设计系统更加灵活，因为可以在不改变组件代码的情况下调整整个应用程序的尺寸。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::SemanticSizing;
///
/// // 创建默认语义尺寸
/// let semantic_sizing = SemanticSizing::default();
/// ```
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
///
/// 定义组件的尺寸。这些尺寸值通常用于按钮、输入框、选择器等组件的高度和宽度。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::ComponentSizing;
///
/// // 创建默认组件尺寸
/// let component_sizing = ComponentSizing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSizing {
    /// 超小组件尺寸，适用于紧凑的UI元素
    pub xs: TokenReference,
    /// 小组件尺寸，适用于小型UI元素
    pub sm: TokenReference,
    /// 中等组件尺寸，适用于标准UI元素
    pub md: TokenReference,
    /// 大组件尺寸，适用于强调的UI元素
    pub lg: TokenReference,
    /// 超大组件尺寸，适用于特别突出的UI元素
    pub xl: TokenReference,
}

/// 图标尺寸
///
/// 定义图标的尺寸。这些尺寸值通常用于系统图标、按钮图标等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::IconSizing;
///
/// // 创建默认图标尺寸
/// let icon_sizing = IconSizing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSizing {
    /// 超小图标尺寸，适用于辅助图标
    pub xs: TokenReference,
    /// 小图标尺寸，适用于小型UI元素中的图标
    pub sm: TokenReference,
    /// 中等图标尺寸，适用于标准UI元素中的图标
    pub md: TokenReference,
    /// 大图标尺寸，适用于强调的UI元素中的图标
    pub lg: TokenReference,
    /// 超大图标尺寸，适用于特别突出的UI元素中的图标
    pub xl: TokenReference,
}

/// 头像尺寸
///
/// 定义头像的尺寸。这些尺寸值通常用于用户头像、团队头像等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::semantic::AvatarSizing;
///
/// // 创建默认头像尺寸
/// let avatar_sizing = AvatarSizing::default();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarSizing {
    /// 超小头像尺寸，适用于列表或表格中的头像
    pub xs: TokenReference,
    /// 小头像尺寸，适用于评论或消息中的头像
    pub sm: TokenReference,
    /// 中等头像尺寸，适用于卡片或个人资料中的头像
    pub md: TokenReference,
    /// 大头像尺寸，适用于个人资料详情中的头像
    pub lg: TokenReference,
    /// 超大头像尺寸，适用于个人资料页面的主头像
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
            xs: TokenReference::create("spacing.xs".to_string()),
            sm: TokenReference::create("spacing.sm".to_string()),
            md: TokenReference::create("spacing.md".to_string()),
            lg: TokenReference::create("spacing.lg".to_string()),
            xl: TokenReference::create("spacing.xl".to_string()),
        }
    }
}

impl Default for PaddingSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::create("spacing.xs".to_string()),
            sm: TokenReference::create("spacing.sm".to_string()),
            md: TokenReference::create("spacing.md".to_string()),
            lg: TokenReference::create("spacing.lg".to_string()),
            xl: TokenReference::create("spacing.xl".to_string()),
        }
    }
}

impl Default for MarginSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::create("spacing.xs".to_string()),
            sm: TokenReference::create("spacing.sm".to_string()),
            md: TokenReference::create("spacing.md".to_string()),
            lg: TokenReference::create("spacing.lg".to_string()),
            xl: TokenReference::create("spacing.xl".to_string()),
        }
    }
}

impl Default for GapSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::create("spacing.xs".to_string()),
            sm: TokenReference::create("spacing.sm".to_string()),
            md: TokenReference::create("spacing.md".to_string()),
            lg: TokenReference::create("spacing.lg".to_string()),
            xl: TokenReference::create("spacing.xl".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            container: TokenReference::create("spacing.xl".to_string()),
            section: TokenReference::create("spacing.lg".to_string()),
            content: TokenReference::create("spacing.md".to_string()),
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
            xs: TokenReference::create("sizing.xs".to_string()),
            sm: TokenReference::create("sizing.sm".to_string()),
            md: TokenReference::create("sizing.md".to_string()),
            lg: TokenReference::create("sizing.lg".to_string()),
            xl: TokenReference::create("sizing.xl".to_string()),
        }
    }
}

impl Default for IconSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::create("sizing.icon.xs".to_string()),
            sm: TokenReference::create("sizing.icon.sm".to_string()),
            md: TokenReference::create("sizing.icon.md".to_string()),
            lg: TokenReference::create("sizing.icon.lg".to_string()),
            xl: TokenReference::create("sizing.icon.xl".to_string()),
        }
    }
}

impl Default for AvatarSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::create("sizing.avatar.xs".to_string()),
            sm: TokenReference::create("sizing.avatar.sm".to_string()),
            md: TokenReference::create("sizing.avatar.md".to_string()),
            lg: TokenReference::create("sizing.avatar.lg".to_string()),
            xl: TokenReference::create("sizing.avatar.xl".to_string()),
        }
    }
}
