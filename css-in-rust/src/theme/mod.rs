//! 主题系统模块
//!
//! 提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。
//! 支持 Ant Design 设计体系和动态主题切换。

// 子模块声明
pub mod css_variables;
pub mod theme_manager;
pub mod theme_provider;

// mod theme_provider;
pub mod css_generator;
pub mod token_definitions;
pub mod token_resolver;
pub mod token_system;
pub mod token_values;

// CSS 生成相关
pub use css_variables::*;

// 令牌解析相关
pub use token_definitions::*;
pub use token_resolver::*;

// 设计令牌系统
pub use token_system::*;
pub use token_values::*;

// 类型别名
pub type DesignTokens = token_values::DesignTokens;

// 重新导出主要类型
pub use css_generator::CssGenerator;
pub use theme_manager::*;
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
    /// 获取设计令牌值
    /// 支持点分路径，如 "colors.primary" 或 "spacing.md"
    pub fn get_token(&self, path: &str) -> Option<String> {
        // 将字符串路径转换为TokenPath
        let token_path = crate::theme::token_definitions::TokenPath::from_str(path);

        // 将ThemeMode转换为ThemeVariant
        let theme_variant = match self.mode {
            ThemeMode::Light => ThemeVariant::Light,
            ThemeMode::Dark => ThemeVariant::Dark,
            ThemeMode::Auto => ThemeVariant::Light, // 默认使用Light
        };

        // 获取令牌值并转换为字符串
        if let Some(token_value) = self.tokens.get_value(&token_path.to_string()) {
            Some(token_value.to_string())
        } else {
            None
        }
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
        Self {
            name: "ant-design".to_string(),
            tokens: DesignTokens::ant_design_default(),
            custom_variables: HashMap::new(),
            mode: ThemeMode::Light,
        }
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

    /// 生成设计令牌的 CSS 变量
    pub fn generate_design_tokens_css(
        &self,
        design_tokens: &DesignTokens,
    ) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 生成颜色变量
        variables.insert(
            "color-primary".to_string(),
            design_tokens.colors.primary.clone(),
        );
        variables.insert(
            "color-success".to_string(),
            design_tokens.colors.success.clone(),
        );
        variables.insert(
            "color-warning".to_string(),
            design_tokens.colors.warning.clone(),
        );
        variables.insert(
            "color-error".to_string(),
            design_tokens.colors.error.clone(),
        );
        variables.insert("color-info".to_string(), design_tokens.colors.info.clone());

        // 文本颜色
        variables.insert(
            "text-primary".to_string(),
            design_tokens.colors.text.primary.clone(),
        );
        variables.insert(
            "text-secondary".to_string(),
            design_tokens.colors.text.secondary.clone(),
        );
        variables.insert(
            "text-disabled".to_string(),
            design_tokens.colors.text.disabled.clone(),
        );
        variables.insert(
            "text-inverse".to_string(),
            design_tokens.colors.text.inverse.clone(),
        );

        // 背景颜色
        variables.insert(
            "bg-primary".to_string(),
            design_tokens.colors.background.primary.clone(),
        );
        variables.insert(
            "bg-secondary".to_string(),
            design_tokens.colors.background.secondary.clone(),
        );
        variables.insert(
            "bg-tertiary".to_string(),
            design_tokens.colors.background.tertiary.clone(),
        );
        variables.insert(
            "bg-inverse".to_string(),
            design_tokens.colors.background.inverse.clone(),
        );

        // 生成字体变量
        variables.insert(
            "font-family-sans".to_string(),
            design_tokens.typography.font_family.sans.clone(),
        );
        variables.insert(
            "font-family-serif".to_string(),
            design_tokens.typography.font_family.serif.clone(),
        );
        variables.insert(
            "font-family-mono".to_string(),
            design_tokens.typography.font_family.mono.clone(),
        );

        variables.insert(
            "font-size-xs".to_string(),
            design_tokens.typography.font_size.xs.clone(),
        );
        variables.insert(
            "font-size-sm".to_string(),
            design_tokens.typography.font_size.sm.clone(),
        );
        variables.insert(
            "font-size-md".to_string(),
            design_tokens.typography.font_size.md.clone(),
        );
        variables.insert(
            "font-size-lg".to_string(),
            design_tokens.typography.font_size.lg.clone(),
        );
        variables.insert(
            "font-size-xl".to_string(),
            design_tokens.typography.font_size.xl.clone(),
        );
        variables.insert(
            "font-size-xxl".to_string(),
            design_tokens.typography.font_size.xxl.clone(),
        );
        variables.insert(
            "font-size-xxxl".to_string(),
            design_tokens.typography.font_size.xxxl.clone(),
        );

        // 生成间距变量
        variables.insert("spacing-xs".to_string(), design_tokens.spacing.xs.clone());
        variables.insert("spacing-sm".to_string(), design_tokens.spacing.sm.clone());
        variables.insert("spacing-md".to_string(), design_tokens.spacing.md.clone());
        variables.insert("spacing-lg".to_string(), design_tokens.spacing.lg.clone());
        variables.insert("spacing-xl".to_string(), design_tokens.spacing.xl.clone());
        variables.insert("spacing-xxl".to_string(), design_tokens.spacing.xxl.clone());
        variables.insert(
            "spacing-xxxl".to_string(),
            design_tokens.spacing.xxxl.clone(),
        );

        // 生成边框变量
        variables.insert(
            "border-width-none".to_string(),
            design_tokens.borders.width.none.clone(),
        );
        variables.insert(
            "border-width-thin".to_string(),
            design_tokens.borders.width.thin.clone(),
        );
        variables.insert(
            "border-width-medium".to_string(),
            design_tokens.borders.width.medium.clone(),
        );
        variables.insert(
            "border-width-thick".to_string(),
            design_tokens.borders.width.thick.clone(),
        );

        variables.insert(
            "border-radius-none".to_string(),
            design_tokens.borders.radius.none.clone(),
        );
        variables.insert(
            "border-radius-sm".to_string(),
            design_tokens.borders.radius.sm.clone(),
        );
        variables.insert(
            "border-radius-md".to_string(),
            design_tokens.borders.radius.md.clone(),
        );
        variables.insert(
            "border-radius-lg".to_string(),
            design_tokens.borders.radius.lg.clone(),
        );
        variables.insert(
            "border-radius-xl".to_string(),
            design_tokens.borders.radius.xl.clone(),
        );
        variables.insert(
            "border-radius-full".to_string(),
            design_tokens.borders.radius.full.clone(),
        );

        // 生成阴影变量
        variables.insert("shadow-sm".to_string(), design_tokens.shadows.sm.clone());
        variables.insert("shadow-md".to_string(), design_tokens.shadows.md.clone());
        variables.insert("shadow-lg".to_string(), design_tokens.shadows.lg.clone());
        variables.insert("shadow-xl".to_string(), design_tokens.shadows.xl.clone());
        variables.insert(
            "shadow-inner".to_string(),
            design_tokens.shadows.inner.clone(),
        );

        // 生成动画变量
        variables.insert(
            "duration-fast".to_string(),
            design_tokens.motion.duration.fast.clone(),
        );
        variables.insert(
            "duration-normal".to_string(),
            design_tokens.motion.duration.normal.clone(),
        );
        variables.insert(
            "duration-slow".to_string(),
            design_tokens.motion.duration.slow.clone(),
        );

        variables.insert(
            "easing-linear".to_string(),
            design_tokens.motion.easing.linear.clone(),
        );
        variables.insert(
            "easing-ease-in".to_string(),
            design_tokens.motion.easing.ease_in.clone(),
        );
        variables.insert(
            "easing-ease-out".to_string(),
            design_tokens.motion.easing.ease_out.clone(),
        );
        variables.insert(
            "easing-ease-in-out".to_string(),
            design_tokens.motion.easing.ease_in_out.clone(),
        );

        // 生成断点变量
        // 断点变量
        variables.insert(
            "breakpoint-xs".to_string(),
            design_tokens.breakpoints.xs.clone(),
        );
        variables.insert(
            "breakpoint-sm".to_string(),
            design_tokens.breakpoints.sm.clone(),
        );
        variables.insert(
            "breakpoint-md".to_string(),
            design_tokens.breakpoints.md.clone(),
        );
        variables.insert(
            "breakpoint-lg".to_string(),
            design_tokens.breakpoints.lg.clone(),
        );
        variables.insert(
            "breakpoint-xl".to_string(),
            design_tokens.breakpoints.xl.clone(),
        );
        variables.insert(
            "breakpoint-xxl".to_string(),
            design_tokens.breakpoints.xxl.clone(),
        );

        variables
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
