//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持 Ant Design 设计体系和动态主题切换。

// 子模块声明
pub mod css_variables;
pub mod design_tokens;
pub mod theme_context;
pub mod theme_provider;

// 重新导出主要类型
pub use css_variables::*;
pub use design_tokens::*;
pub use theme_context::*;
pub use theme_provider::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题配置结构体
///
/// 包含完整的 Ant Design 设计令牌和自定义主题配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 设计令牌
    pub tokens: DesignTokens,
    /// 自定义 CSS 变量
    pub custom_variables: HashMap<String, String>,
    /// 主题模式（light/dark）
    pub mode: ThemeMode,
}

/// 主题模式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

impl Default for Theme {
    /// 创建默认的 Ant Design 主题
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            tokens: DesignTokens::ant_design_default(),
            custom_variables: HashMap::new(),
            mode: ThemeMode::Light,
        }
    }
}

impl Theme {
    /// 创建新的主题实例
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// 设置主题模式
    pub fn with_mode(mut self, mode: ThemeMode) -> Self {
        self.mode = mode;
        self
    }

    /// 设置设计令牌
    pub fn with_tokens(mut self, tokens: DesignTokens) -> Self {
        self.tokens = tokens;
        self
    }

    /// 添加自定义 CSS 变量
    pub fn with_custom_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_variables.insert(name.into(), value.into());
        self
    }

    /// 获取主题令牌值
    ///
    /// 支持点分路径，如 "colors.primary" 或 "spacing.md"
    pub fn get_token(&self, path: &str) -> Option<String> {
        self.tokens.get_value(path)
    }

    /// 生成 CSS 变量声明
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 添加设计令牌变量
        css.push_str(&self.tokens.to_css_variables());

        // 添加自定义变量
        for (name, value) in &self.custom_variables {
            css.push_str(&format!("  --{}: {};\n", name, value));
        }

        css
    }

    /// 创建 Ant Design 默认主题
    pub fn ant_design() -> Self {
        Self::default()
    }

    /// 创建 Ant Design 暗色主题
    pub fn ant_design_dark() -> Self {
        Self {
            name: "ant-design-dark".to_string(),
            tokens: DesignTokens::ant_design_dark(),
            custom_variables: HashMap::new(),
            mode: ThemeMode::Dark,
        }
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

    #[test]
    fn test_token_access() {
        let theme = Theme::ant_design();

        // 测试获取主色调
        let primary = theme.get_token("colors.primary");
        assert!(primary.is_some());
    }
}
