//! # 颜色系统模块
//!
//! 提供通用的颜色管理功能，包括颜色调色板、语义颜色映射等。

use crate::theme::core::token::definitions::{ColorValue, ThemeVariant, TokenReference};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 颜色系统
///
/// 管理应用程序中使用的所有颜色，包括主色调、中性色、功能色和扩展色。
/// 颜色系统提供了一种结构化的方式来组织和访问颜色值，确保整个应用程序的颜色一致性。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::ColorSystem;
///
/// // 创建默认颜色系统
/// let mut color_system = ColorSystem::new();
///
/// // 获取主色调
/// if let Some(primary_color) = color_system.get_color("primary.500") {
///     println!("主色调: {}", primary_color);
/// }
///
/// // 设置自定义颜色
/// color_system.set_color("extended.brand.logo", "#FF5733".to_string()).unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSystem {
    /// 主色调
    pub primary: BTreeMap<String, String>,
    /// 中性色
    pub neutral: BTreeMap<String, String>,
    /// 功能色
    pub functional: BTreeMap<String, String>,
    /// 扩展色
    pub extended: BTreeMap<String, String>,
}

impl Default for ColorSystem {
    fn default() -> Self {
        let mut primary = BTreeMap::new();
        primary.insert("50".to_string(), "#e6f3ff".to_string());
        primary.insert("100".to_string(), "#b3d9ff".to_string());
        primary.insert("200".to_string(), "#80bfff".to_string());
        primary.insert("300".to_string(), "#4da6ff".to_string());
        primary.insert("400".to_string(), "#1a8cff".to_string());
        primary.insert("500".to_string(), "#0066cc".to_string());
        primary.insert("600".to_string(), "#0052a3".to_string());
        primary.insert("700".to_string(), "#003d7a".to_string());
        primary.insert("800".to_string(), "#002952".to_string());
        primary.insert("900".to_string(), "#001429".to_string());

        let mut neutral = BTreeMap::new();
        neutral.insert("50".to_string(), "#fafafa".to_string());
        neutral.insert("100".to_string(), "#f5f5f5".to_string());
        neutral.insert("200".to_string(), "#eeeeee".to_string());
        neutral.insert("300".to_string(), "#dddddd".to_string());
        neutral.insert("400".to_string(), "#bfbfbf".to_string());
        neutral.insert("500".to_string(), "#8c8c8c".to_string());
        neutral.insert("600".to_string(), "#595959".to_string());
        neutral.insert("700".to_string(), "#434343".to_string());
        neutral.insert("800".to_string(), "#262626".to_string());
        neutral.insert("900".to_string(), "#1f1f1f".to_string());

        let mut functional = BTreeMap::new();
        functional.insert("success.50".to_string(), "#f6ffed".to_string());
        functional.insert("success.500".to_string(), "#52c41a".to_string());
        functional.insert("success.600".to_string(), "#389e0d".to_string());
        functional.insert("warning.50".to_string(), "#fffbe6".to_string());
        functional.insert("warning.500".to_string(), "#faad14".to_string());
        functional.insert("warning.600".to_string(), "#d48806".to_string());
        functional.insert("error.50".to_string(), "#fff2f0".to_string());
        functional.insert("error.500".to_string(), "#ff4d4f".to_string());
        functional.insert("error.600".to_string(), "#cf1322".to_string());

        Self {
            primary,
            neutral,
            functional,
            extended: BTreeMap::new(),
        }
    }
}

impl ColorSystem {
    /// 创建新的颜色系统
    ///
    /// 初始化一个包含默认颜色值的颜色系统。默认颜色包括主色调、中性色和功能色（成功、警告、错误）。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `ColorSystem` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::ColorSystem;
    ///
    /// let color_system = ColorSystem::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取颜色值
    ///
    /// 根据路径获取颜色值。路径格式为 `category.level` 或 `category.subcategory.level`。
    ///
    /// # 参数
    ///
    /// * `path` - 颜色路径，例如 "primary.500"、"functional.success.500"
    ///
    /// # 返回值
    ///
    /// 如果找到颜色，则返回 `Some(&String)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::ColorSystem;
    ///
    /// let color_system = ColorSystem::new();
    ///
    /// // 获取主色调
    /// let primary = color_system.get_color("primary.500");
    /// assert!(primary.is_some());
    ///
    /// // 获取功能色
    /// let success = color_system.get_color("functional.success.500");
    /// assert!(success.is_some());
    ///
    /// // 获取不存在的颜色
    /// let nonexistent = color_system.get_color("nonexistent.color");
    /// assert!(nonexistent.is_none());
    /// ```
    pub fn get_color(&self, path: &str) -> Option<&String> {
        let parts: Vec<&str> = path.split('.').collect();
        match parts.as_slice() {
            ["primary", level] => self.primary.get(*level),
            ["neutral", level] => self.neutral.get(*level),
            ["functional", rest @ ..] => self.functional.get(&rest.join(".")),
            ["extended", rest @ ..] => self.extended.get(&rest.join(".")),
            _ => None,
        }
    }

    /// 设置颜色值
    ///
    /// 根据路径设置颜色值。路径格式为 `category.level` 或 `category.subcategory.level`。
    ///
    /// # 参数
    ///
    /// * `path` - 颜色路径，例如 "primary.500"、"functional.success.500"
    /// * `value` - 颜色值，例如 "#0066cc"
    ///
    /// # 返回值
    ///
    /// 如果设置成功，则返回 `Ok(())`，如果路径无效，则返回 `Err(String)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::ColorSystem;
    ///
    /// let mut color_system = ColorSystem::new();
    ///
    /// // 设置主色调
    /// color_system.set_color("primary.500", "#1890ff".to_string()).unwrap();
    ///
    /// // 设置功能色
    /// color_system.set_color("functional.success.500", "#52c41a".to_string()).unwrap();
    ///
    /// // 设置扩展色
    /// color_system.set_color("extended.brand.primary", "#FF5733".to_string()).unwrap();
    ///
    /// // 尝试设置无效路径
    /// let result = color_system.set_color("invalid", "#000000".to_string());
    /// assert!(result.is_err());
    /// ```
    pub fn set_color(&mut self, path: &str, value: String) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();
        match parts.as_slice() {
            ["primary", level] => {
                self.primary.insert(level.to_string(), value);
                Ok(())
            }
            ["neutral", level] => {
                self.neutral.insert(level.to_string(), value);
                Ok(())
            }
            ["functional", rest @ ..] => {
                self.functional.insert(rest.join("."), value);
                Ok(())
            }
            ["extended", rest @ ..] => {
                self.extended.insert(rest.join("."), value);
                Ok(())
            }
            _ => Err(format!("Invalid color path: {}", path)),
        }
    }
}

/// 颜色调色板
///
/// 提供一组结构化的颜色集合，用于构建一致的设计系统。
/// 调色板包含主色调、中性色、成功色、警告色、错误色和信息色。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::color::ColorPalette;
/// use css_in_rust::theme::core::token::definitions::ColorValue;
///
/// let mut palette = ColorPalette::default();
/// palette.apply_light_theme(); // 应用浅色主题
/// ```
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
///
/// 提供基于语义的颜色定义，使设计系统更易于理解和使用。
/// 语义颜色将调色板中的原始颜色映射到具有特定用途的颜色，如文本、背景、边框和状态颜色。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::color::SemanticColors;
/// use css_in_rust::theme::core::token::definitions::ThemeVariant;
///
/// let mut semantic_colors = SemanticColors::default();
/// semantic_colors.update_for_theme(ThemeVariant::Dark); // 更新为深色主题
/// ```
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
///
/// 定义应用程序中不同类型文本使用的颜色。
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
///
/// 定义应用程序中不同区域和状态的背景颜色。
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
///
/// 定义应用程序中不同状态下边框的颜色。
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
///
/// 定义应用程序中不同状态的颜色，如成功、警告、错误和信息。
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
            primary: TokenReference::create("global.color_palette.neutral.900".to_string()),
            secondary: TokenReference::create("global.color_palette.neutral.600".to_string()),
            disabled: TokenReference::create("global.color_palette.neutral.400".to_string()),
            placeholder: TokenReference::create("global.color_palette.neutral.500".to_string()),
        }
    }
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            primary: TokenReference::create("global.color_palette.neutral.50".to_string()),
            secondary: TokenReference::create("global.color_palette.neutral.100".to_string()),
            hover: TokenReference::create("global.color_palette.neutral.200".to_string()),
            active: TokenReference::create("global.color_palette.neutral.300".to_string()),
        }
    }
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            default: TokenReference::create("global.color_palette.neutral.300".to_string()),
            hover: TokenReference::create("global.color_palette.neutral.400".to_string()),
            focus: TokenReference::create("global.color_palette.primary.500".to_string()),
            error: TokenReference::create("global.color_palette.error.500".to_string()),
        }
    }
}

impl Default for StatusColors {
    fn default() -> Self {
        Self {
            success: TokenReference::create("global.color_palette.success.500".to_string()),
            warning: TokenReference::create("global.color_palette.warning.500".to_string()),
            error: TokenReference::create("global.color_palette.error.500".to_string()),
            info: TokenReference::create("global.color_palette.info.500".to_string()),
        }
    }
}

/// 颜色系统工具函数
impl ColorPalette {
    /// 应用浅色主题颜色
    ///
    /// 配置调色板以使用浅色主题的颜色值。这将设置主色调和中性色的所有色阶。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::color::ColorPalette;
    ///
    /// let mut palette = ColorPalette::default();
    /// palette.apply_light_theme();
    ///
    /// // 现在调色板使用浅色主题的颜色值
    /// ```
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
    ///
    /// 配置调色板以使用深色主题的颜色值。这将设置主色调和中性色的所有色阶，
    /// 通常会反转浅色主题的颜色值，使深色值用于背景，浅色值用于文本。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::color::ColorPalette;
    ///
    /// let mut palette = ColorPalette::default();
    /// palette.apply_dark_theme();
    ///
    /// // 现在调色板使用深色主题的颜色值
    /// ```
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
    ///
    /// 根据提供的主题变体（浅色、深色或自动），更新语义颜色的引用，
    /// 使其指向适合该主题的颜色值。
    ///
    /// # 参数
    ///
    /// * `theme` - 主题变体，可以是 Light、Dark 或 Auto
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::color::SemanticColors;
    /// use css_in_rust::theme::core::token::definitions::ThemeVariant;
    ///
    /// let mut semantic_colors = SemanticColors::default();
    ///
    /// // 更新为深色主题
    /// semantic_colors.update_for_theme(ThemeVariant::Dark);
    ///
    /// // 更新为浅色主题
    /// semantic_colors.update_for_theme(ThemeVariant::Light);
    ///
    /// // 更新为自动主题（跟随系统）
    /// semantic_colors.update_for_theme(ThemeVariant::Auto);
    /// ```
    pub fn update_for_theme(&mut self, theme: ThemeVariant) {
        match theme {
            ThemeVariant::Light => {
                self.text.primary =
                    TokenReference::create("global.color_palette.neutral.900".to_string());
                self.text.secondary =
                    TokenReference::create("global.color_palette.neutral.600".to_string());
                self.background.primary =
                    TokenReference::create("global.color_palette.neutral.50".to_string());
            }
            ThemeVariant::Dark => {
                self.text.primary =
                    TokenReference::create("global.color_palette.neutral.100".to_string());
                self.text.secondary =
                    TokenReference::create("global.color_palette.neutral.400".to_string());
                self.background.primary =
                    TokenReference::create("global.color_palette.neutral.900".to_string());
            }
            ThemeVariant::Auto => {
                // Auto模式使用浅色主题设置
                self.text.primary =
                    TokenReference::create("global.color_palette.neutral.900".to_string());
                self.background.primary =
                    TokenReference::create("global.color_palette.neutral.50".to_string());
            }
        }
    }
}
