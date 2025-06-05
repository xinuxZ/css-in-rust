use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::theme::core::token::TokenSystem;

/// 主题配置结构体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// 主题名称
    pub name: String,
    /// 主题模式
    pub mode: ThemeMode,
    /// Token 系统
    pub token_system: TokenSystem,
    /// 自定义变量
    pub custom_variables: HashMap<String, String>,
}

/// 主题模式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            mode: ThemeMode::default(),
            token_system: TokenSystem::default(),
            custom_variables: HashMap::new(),
        }
    }
}

impl Theme {
    /// 创建新主题
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

    /// 设置 Token 系统
    pub fn with_token_system(mut self, token_system: TokenSystem) -> Self {
        self.token_system = token_system;
        self
    }

    /// 添加自定义变量
    pub fn with_custom_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_variables.insert(name.into(), value.into());
        self
    }

    /// 添加颜色变量
    pub fn add_color(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();

        // 添加到自定义变量
        self.custom_variables
            .insert(format!("color-{}", name), value.clone());

        // 同时添加到令牌系统中
        self.token_system
            .set_color(&format!("color.{}", name), &value);
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&mut self) -> String {
        let mut css = String::new();

        // 添加 Token 系统的变量
        if let Ok(token_css) = self.token_system.export_css_variables() {
            css.push_str(&token_css);
        }

        // 添加自定义变量
        for (name, value) in &self.custom_variables {
            css.push_str(&format!("  --{}: {};\n", name, value));
        }

        css
    }
}
