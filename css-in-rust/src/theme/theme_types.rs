use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::theme::core::token::definitions::ThemeVariant;
use crate::theme::core::token::simple_system::TokenSystem;

/// 主题定义
///
/// 表示一个完整的主题，包含名称、模式、令牌系统和自定义变量
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::theme_types::{Theme};
/// use crate::theme::core::token::definitions::ThemeVariant;
///
/// // 创建默认主题
/// let default_theme = Theme::default();
///
/// // 创建自定义主题
/// let custom_theme = Theme::new("custom-theme")
///     .with_mode(ThemeVariant::Dark)
///     .with_custom_variable("--primary-color", "#3366ff");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题模式
    pub mode: ThemeVariant,
    /// Token 系统
    pub token_system: TokenSystem,
    /// 自定义变量
    pub custom_variables: HashMap<String, String>,
}

impl Default for Theme {
    /// 创建默认主题
    ///
    /// 创建一个名为 "default" 的亮色主题，使用默认的令牌系统
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let theme = Theme::default();
    /// assert_eq!(theme.name, "default");
    /// ```
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            mode: ThemeVariant::default(),
            token_system: TokenSystem::default(),
            custom_variables: HashMap::new(),
        }
    }
}

impl Theme {
    /// 创建新主题
    ///
    /// 使用指定的名称创建一个新主题，使用默认的主题模式和令牌系统
    ///
    /// # Arguments
    ///
    /// * `name` - 主题名称
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let theme = Theme::new("my-theme");
    /// assert_eq!(theme.name, "my-theme");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mode: ThemeVariant::default(),
            token_system: TokenSystem::default(),
            custom_variables: HashMap::new(),
        }
    }

    /// 设置主题模式
    ///
    /// # Arguments
    ///
    /// * `mode` - 要设置的主题模式
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::{Theme, ThemeVariant};
    ///
    /// let theme = Theme::new("my-theme").with_mode(ThemeVariant::Dark);
    /// ```
    pub fn with_mode(mut self, mode: ThemeVariant) -> Self {
        self.mode = mode;
        self
    }

    /// 设置令牌系统
    ///
    /// # Arguments
    ///
    /// * `token_system` - 要设置的令牌系统
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    /// use css_in_rust::theme::core::token::simple_system::TokenSystem;
    ///
    /// let token_system = TokenSystem::default();
    /// let theme = Theme::new("my-theme").with_token_system(token_system);
    /// ```
    pub fn with_token_system(mut self, token_system: TokenSystem) -> Self {
        self.token_system = token_system;
        self
    }

    /// 添加自定义变量
    ///
    /// # Arguments
    ///
    /// * `name` - 变量名称
    /// * `value` - 变量值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let theme = Theme::new("my-theme")
    ///     .with_custom_variable("--primary-color", "#3366ff")
    ///     .with_custom_variable("--font-size", "16px");
    /// ```
    pub fn with_custom_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_variables.insert(name.into(), value.into());
        self
    }

    /// 添加颜色变量
    ///
    /// # Arguments
    ///
    /// * `name` - 颜色变量名
    /// * `value` - 颜色值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let mut theme = Theme::new("my-theme");
    /// theme.add_color("primary", "#3366ff");
    /// theme.add_color("secondary", "#ff6633");
    /// ```
    pub fn add_color(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();

        // 添加到令牌系统
        self.token_system.set_color(&name, &value);

        // 添加到自定义变量
        self.custom_variables
            .insert(format!("--color-{}", name), value.to_string());
    }

    /// 生成CSS变量
    ///
    /// 将主题转换为CSS变量字符串
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let mut theme = Theme::new("my-theme");
    /// theme.add_color("primary", "#3366ff");
    /// let css = theme.to_css_variables();
    /// // css 包含 "--color-primary: #3366ff;"
    /// ```
    pub fn to_css_variables(&mut self) -> String {
        // 从令牌系统获取变量
        let token_css = self.token_system.to_css_variables();

        // 合并自定义变量
        let mut css = token_css;
        for (name, value) in &self.custom_variables {
            if !name.starts_with("--") {
                css.push_str(&format!("--{}: {};\n", name, value));
            } else {
                css.push_str(&format!("{}: {};\n", name, value));
            }
        }

        css
    }
}
