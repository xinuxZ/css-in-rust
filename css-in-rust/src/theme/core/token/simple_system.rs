use crate::theme::core::token::definitions::TokenValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题变体
///
/// 定义了系统支持的主题类型，包括亮色主题、暗色主题和自动主题。
/// 自动主题通常会根据用户的系统设置自动选择亮色或暗色主题。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
///
/// // 创建亮色主题
/// let light_theme = ThemeVariant::Light;
///
/// // 创建暗色主题
/// let dark_theme = ThemeVariant::Dark;
///
/// // 创建自动主题（根据系统设置）
/// let auto_theme = ThemeVariant::Auto;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeVariant {
    /// 亮色主题
    Light,
    /// 暗色主题
    Dark,
    /// 自动主题（根据系统设置）
    Auto,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::Light
    }
}

/// 简化版的令牌系统
///
/// 提供基本的令牌管理功能，适用于简单的主题需求。
/// 这个系统允许定义和管理CSS变量，并支持不同的主题变体。
///
/// 与完整的 `DesignTokenSystem` 相比，`TokenSystem` 更轻量，
/// 但功能也相对有限，主要用于简单场景或作为更复杂系统的基础。
///
/// # 特性
///
/// - 支持多种主题变体（亮色、暗色、自动）
/// - 管理自定义CSS变量
/// - 导出CSS变量字符串
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::simple_system::{TokenSystem, ThemeVariant};
///
/// // 创建一个简单的令牌系统
/// let system = TokenSystem::new()
///     .with_variant(ThemeVariant::Light)
///     .with_variable("color-primary", "#1890ff")
///     .with_variable("color-text", "#333333")
///     .with_variable("font-size-base", "14px");
///
/// // 生成CSS变量
/// let css = system.to_css_variables();
/// println!("{}", css);
/// // 输出:
/// //   --color-primary: #1890ff;
/// //   --color-text: #333333;
/// //   --font-size-base: 14px;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenSystem {
    /// 主题变体
    pub variant: ThemeVariant,
    /// 自定义变量
    pub variables: HashMap<String, String>,
}

impl Default for TokenSystem {
    fn default() -> Self {
        Self {
            variant: ThemeVariant::default(),
            variables: HashMap::new(),
        }
    }
}

impl TokenSystem {
    /// 创建新的令牌系统
    ///
    /// 初始化一个空的令牌系统，使用默认的亮色主题变体。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `TokenSystem` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主题变体
    ///
    /// 配置令牌系统使用的主题变体。
    ///
    /// # 参数
    ///
    /// * `variant` - 要使用的主题变体，如 `ThemeVariant::Light` 或 `ThemeVariant::Dark`
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `TokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::{TokenSystem, ThemeVariant};
    ///
    /// let system = TokenSystem::new()
    ///     .with_variant(ThemeVariant::Dark); // 使用暗色主题
    /// ```
    pub fn with_variant(mut self, variant: ThemeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 添加自定义变量
    ///
    /// 向令牌系统添加一个自定义CSS变量。
    ///
    /// # 参数
    ///
    /// * `name` - 变量名称，如 "color-primary" 或 "font-size-base"
    /// * `value` - 变量值，如 "#1890ff" 或 "14px"
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `TokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new()
    ///     .with_variable("color-primary", "#1890ff")
    ///     .with_variable("color-text", "#333333");
    /// ```
    pub fn with_variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(name.into(), value.into());
        self
    }

    /// 设置颜色变量
    ///
    /// 向令牌系统添加一个颜色变量。这是 `with_variable` 的便捷方法，专门用于设置颜色。
    ///
    /// # 参数
    ///
    /// * `name` - 颜色变量名称，如 "primary" 或 "text"
    /// * `value` - 颜色值，如 "#1890ff" 或 "rgb(51, 51, 51)"
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let mut system = TokenSystem::new();
    ///
    /// // 设置颜色变量
    /// system.set_color("primary", "#1890ff");
    /// system.set_color("success", "#52c41a");
    /// system.set_color("warning", "#faad14");
    /// system.set_color("error", "#f5222d");
    /// ```
    pub fn set_color(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    /// 导出CSS变量
    ///
    /// 将令牌系统中的变量导出为CSS变量格式的字符串。
    /// 这个方法会自动为变量名添加 `--` 前缀（如果没有的话）。
    ///
    /// # 返回值
    ///
    /// 如果导出成功，返回包含CSS变量定义的字符串；如果导出失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new()
    ///     .with_variable("color-primary", "#1890ff")
    ///     .with_variable("--color-text", "#333333"); // 已经有 -- 前缀
    ///
    /// let css = system.export_css_variables().unwrap();
    /// // css 包含:
    /// //   --color-primary: #1890ff;
    /// //   --color-text: #333333;
    /// ```
    pub fn export_css_variables(&self) -> Result<String, String> {
        let mut css = String::new();

        // 添加自定义变量
        for (name, value) in &self.variables {
            let var_name = if name.starts_with("--") {
                name.clone()
            } else {
                format!("--{}", name)
            };
            css.push_str(&format!("  {}: {};\n", var_name, value));
        }

        Ok(css)
    }

    /// 生成CSS变量字符串
    ///
    /// 将令牌系统中的变量导出为CSS变量格式的字符串。
    /// 与 `export_css_variables` 不同，这个方法不会返回错误，
    /// 如果导出失败，将返回空字符串。
    ///
    /// # 返回值
    ///
    /// 返回包含CSS变量定义的字符串，如果导出失败，返回空字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let system = TokenSystem::new()
    ///     .with_variable("color-primary", "#1890ff")
    ///     .with_variable("font-size-base", "14px");
    ///
    /// // 在CSS文件中使用
    /// let css = format!(":root {{\n{}}}", system.to_css_variables());
    /// // 输出:
    /// // :root {
    /// //   --color-primary: #1890ff;
    /// //   --font-size-base: 14px;
    /// // }
    /// ```
    pub fn to_css_variables(&self) -> String {
        self.export_css_variables().unwrap_or_default()
    }

    /// 获取令牌值
    ///
    /// 根据提供的路径获取令牌值。
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，例如 "spacing.component.md"
    ///
    /// # 返回值
    ///
    /// 如果找到对应的令牌值，返回 `Some(TokenValue)`；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = TokenSystem::new()
    ///     .with_variable("spacing-md", "16px");
    ///
    /// let value = system.get_value("spacing-md");
    /// ```
    pub fn get_value(&self, path: &str) -> Option<TokenValue> {
        self.variables
            .get(path)
            .map(|value| TokenValue::String(value.clone()))
    }

    /// 设置令牌值
    ///
    /// 设置指定路径的令牌值。
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，例如 "spacing.component.md"
    /// * `value` - 要设置的令牌值
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = TokenSystem::new();
    ///
    /// // 设置组件中等间距
    /// system.set_value("spacing.component.md".to_string(), TokenValue::String("16px".to_string()));
    /// ```
    pub fn set_value(&mut self, path: String, value: TokenValue) {
        match value {
            TokenValue::String(s) => {
                self.variables.insert(path, s);
            }
            _ => {
                self.variables.insert(path, value.to_string());
            }
        }
    }
}
