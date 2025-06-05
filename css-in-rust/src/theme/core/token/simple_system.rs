use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题变体
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
/// 提供基本的令牌管理功能，但不包含完整的实现
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
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置主题变体
    pub fn with_variant(mut self, variant: ThemeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 添加自定义变量
    pub fn with_variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(name.into(), value.into());
        self
    }

    /// 设置颜色变量
    pub fn set_color(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    /// 导出CSS变量
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
    pub fn to_css_variables(&self) -> String {
        self.export_css_variables().unwrap_or_default()
    }
}
