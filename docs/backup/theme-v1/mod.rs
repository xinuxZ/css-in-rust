//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括：
//! - 设计令牌（Design Tokens）
//! - CSS 变量生成
//! - 主题切换
//! - 自定义主题支持

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 子模块声明
pub mod ant_design_presets;
pub mod css_variables;
pub mod design_token_system;
pub mod design_tokens;
pub mod theme_manager;
pub mod theme_provider;
pub mod token_value;

// 重新导出主要类型
pub use css_variables::*;
pub use design_token_system::*;
pub use design_tokens::*;
pub use theme_manager::*;
pub use theme_provider::*;
pub use token_value::TokenValue;

/// 主题配置结构体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 设计令牌
    pub tokens: DesignTokens,
    /// 自定义 CSS 变量
    pub custom_variables: HashMap<String, String>,
    /// 主题模式
    pub mode: ThemeMode,
}

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    /// 亮色模式
    Light,
    /// 暗色模式
    Dark,
    /// 自动模式（跟随系统）
    Auto,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Light
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

impl Theme {
    /// 创建新的主题实例
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tokens: DesignTokens::default(),
            custom_variables: HashMap::new(),
            mode: ThemeMode::Light,
        }
    }

    /// 从设计令牌创建主题
    pub fn from_design_tokens(name: String, design_tokens: DesignTokens, mode: ThemeMode) -> Self {
        Self {
            name,
            tokens: design_tokens,
            custom_variables: HashMap::new(),
            mode,
        }
    }

    /// 设置主题模式
    pub fn with_mode(mut self, mode: ThemeMode) -> Self {
        self.mode = mode;
        self
    }

    /// 设置主题模式（可变引用）
    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
    }

    /// 设置设计令牌
    pub fn with_tokens(mut self, tokens: DesignTokens) -> Self {
        self.tokens = tokens;
        self
    }

    /// 添加自定义 CSS 变量
    pub fn with_custom_variable(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_variables.insert(key.into(), value.into());
        self
    }

    /// 设置自定义 CSS 变量
    pub fn set_custom_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.custom_variables.insert(key.into(), value.into());
    }

    /// 移除自定义 CSS 变量
    pub fn remove_custom_variable(&mut self, key: &str) -> Option<String> {
        self.custom_variables.remove(key)
    }

    /// 获取自定义 CSS 变量
    pub fn get_custom_variable(&self, key: &str) -> Option<&String> {
        self.custom_variables.get(key)
    }

    /// 生成 CSS 变量字符串
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 生成设计令牌的 CSS 变量
        css.push_str(&self.tokens.to_css_variables());

        // 添加自定义 CSS 变量
        for (key, value) in &self.custom_variables {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        css
    }

    /// 生成 CSS 变量（兼容旧版本）
    pub fn generate_css_variables(&self) -> String {
        let mut css = String::new();
        css.push_str(":root {\n");

        css.push_str(&self.to_css_variables());

        css.push_str("}\n");
        css
    }

    /// 创建 Ant Design 默认主题
    pub fn ant_design() -> Self {
        Self {
            name: "ant-design".to_string(),
            tokens: DesignTokens::ant_design_light(),
            custom_variables: HashMap::new(),
            mode: ThemeMode::Light,
        }
    }

    /// 创建 Ant Design 默认主题（兼容旧版本）
    pub fn ant_design_default() -> Self {
        let design_tokens = DesignTokens::ant_design_light();
        Self::from_design_tokens(
            "Ant Design Default".to_string(),
            design_tokens,
            ThemeMode::Light,
        )
    }

    /// 创建 Ant Design 暗色主题
    pub fn ant_design_dark() -> Self {
        let design_tokens = DesignTokens::ant_design_dark();
        Self::from_design_tokens(
            "Ant Design Dark".to_string(),
            design_tokens,
            ThemeMode::Dark,
        )
    }

    /// 生成完整的设计令牌 CSS 变量
    pub fn generate_design_tokens_css(&self, design_tokens: &DesignTokens) -> String {
        let mut css = String::new();
        css.push_str(":root {\n");

        // 生成所有设计令牌的 CSS 变量
        css.push_str(&design_tokens.to_css_variables());

        // 添加自定义 CSS 变量
        for (key, value) in &self.custom_variables {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        css.push_str("}\n");
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("test-theme")
            .with_mode(ThemeMode::Dark)
            .with_custom_variable("custom-color", "#ff0000");

        assert_eq!(theme.name, "test-theme");
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(
            theme.custom_variables.get("custom-color"),
            Some(&"#ff0000".to_string())
        );
    }

    #[test]
    fn test_css_variables_generation() {
        let theme = Theme::ant_design().with_custom_variable("test-var", "test-value");

        let css = theme.to_css_variables();
        assert!(css.contains("--test-var: test-value;"));
    }
}
