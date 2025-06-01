//! 设计令牌定义模块
//!
//! 本模块定义了设计令牌系统的基础抽象和类型，遵循单一职责原则。
//! 职责：定义令牌的基础接口、类型和约束

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 令牌值类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    /// 字符串值（如颜色、字体名称）
    String(String),
    /// 数值（如尺寸、权重）
    Number(f64),
    /// 布尔值（如是否启用）
    Boolean(bool),
    /// 引用其他令牌
    Reference(String),
    /// 数组值（如字体栈）
    Array(Vec<TokenValue>),
    /// 对象值（如复合属性）
    Object(HashMap<String, TokenValue>),
}

impl TokenValue {
    /// 转换为字符串表示
    pub fn to_string(&self) -> String {
        match self {
            TokenValue::String(s) => s.clone(),
            TokenValue::Number(n) => n.to_string(),
            TokenValue::Boolean(b) => b.to_string(),
            TokenValue::Reference(r) => format!("var(--{})", r.replace(".", "-")),
            TokenValue::Array(arr) => arr
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            TokenValue::Object(_) => "[object]".to_string(),
        }
    }

    /// 转换为CSS值
    pub fn to_css_value(&self) -> String {
        match self {
            TokenValue::String(s) => s.clone(),
            TokenValue::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            TokenValue::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
            TokenValue::Reference(r) => format!("var(--{})", r.replace(".", "-")),
            TokenValue::Array(arr) => arr
                .iter()
                .map(|v| v.to_css_value())
                .collect::<Vec<_>>()
                .join(", "),
            TokenValue::Object(_) => "initial".to_string(),
        }
    }

    /// 获取数值（如果是数值类型）
    pub fn as_number(&self) -> Option<f64> {
        match self {
            TokenValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// 获取字符串（如果是字符串类型）
    pub fn as_string(&self) -> Option<&str> {
        match self {
            TokenValue::String(s) => Some(s),
            _ => None,
        }
    }
}

/// 令牌路径，用于标识令牌的位置
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenPath {
    /// 路径段，如 ["color", "primary", "500"]
    pub segments: Vec<String>,
}

impl TokenPath {
    /// 从字符串创建令牌路径
    pub fn from_str(path: &str) -> Self {
        Self {
            segments: path.split('.').map(|s| s.to_string()).collect(),
        }
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        self.segments.join(".")
    }

    /// 转换为CSS变量名
    pub fn to_css_var_name(&self) -> String {
        format!("--{}", self.segments.join("-"))
    }

    /// 添加路径段
    pub fn push(&mut self, segment: &str) {
        self.segments.push(segment.to_string());
    }

    /// 获取父路径
    pub fn parent(&self) -> Option<TokenPath> {
        if self.segments.len() > 1 {
            Some(TokenPath {
                segments: self.segments[..self.segments.len() - 1].to_vec(),
            })
        } else {
            None
        }
    }
}

/// 主题变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ThemeVariant {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        ThemeVariant::Light
    }
}

/// 令牌元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenMetadata {
    /// 令牌描述
    pub description: Option<String>,
    /// 令牌类型（如 color, size, font）
    pub token_type: String,
    /// 是否已弃用
    pub deprecated: bool,
    /// 别名（其他可用的名称）
    pub aliases: Vec<String>,
    /// 标签（用于分类和搜索）
    pub tags: Vec<String>,
}

impl Default for TokenMetadata {
    fn default() -> Self {
        Self {
            description: None,
            token_type: "unknown".to_string(),
            deprecated: false,
            aliases: Vec::new(),
            tags: Vec::new(),
        }
    }
}

/// 令牌定义的基础trait
pub trait TokenDefinitions {
    /// 获取令牌值
    fn get_token_value(&self, path: &TokenPath, theme: ThemeVariant) -> Option<TokenValue>;

    /// 设置令牌值
    fn set_token_value(
        &mut self,
        path: &TokenPath,
        value: TokenValue,
        theme: ThemeVariant,
    ) -> Result<(), String>;

    /// 获取令牌元数据
    fn get_token_metadata(&self, path: &TokenPath) -> Option<TokenMetadata>;

    /// 列出所有令牌路径
    fn list_token_paths(&self, theme: ThemeVariant) -> Vec<TokenPath>;

    /// 检查令牌是否存在
    fn has_token(&self, path: &TokenPath, theme: ThemeVariant) -> bool {
        self.get_token_value(path, theme).is_some()
    }

    /// 解析令牌引用
    fn resolve_reference(&self, reference: &str, theme: ThemeVariant) -> Option<TokenValue> {
        let path = TokenPath::from_str(reference);
        self.get_token_value(&path, theme)
    }
}

/// 令牌类别枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenCategory {
    Color,
    Typography,
    Spacing,
    Sizing,
    Border,
    Shadow,
    Motion,
    Component,
}

impl TokenCategory {
    /// 获取类别的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenCategory::Color => "color",
            TokenCategory::Typography => "typography",
            TokenCategory::Spacing => "spacing",
            TokenCategory::Sizing => "sizing",
            TokenCategory::Border => "border",
            TokenCategory::Shadow => "shadow",
            TokenCategory::Motion => "motion",
            TokenCategory::Component => "component",
        }
    }
}

/// 令牌验证错误
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValidationError {
    InvalidPath(String),
    InvalidValue(String),
    CircularReference(String),
    MissingReference(String),
    TypeMismatch { expected: String, actual: String },
}

impl std::fmt::Display for TokenValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValidationError::InvalidPath(path) => write!(f, "Invalid token path: {}", path),
            TokenValidationError::InvalidValue(value) => {
                write!(f, "Invalid token value: {}", value)
            }
            TokenValidationError::CircularReference(path) => {
                write!(f, "Circular reference detected: {}", path)
            }
            TokenValidationError::MissingReference(reference) => {
                write!(f, "Missing reference: {}", reference)
            }
            TokenValidationError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for TokenValidationError {}
