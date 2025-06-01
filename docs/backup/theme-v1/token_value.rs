//! 设计令牌值类型定义
//!
//! 定义设计令牌系统中使用的各种值类型

use serde::{Deserialize, Serialize};
use std::fmt;

/// 设计令牌值
///
/// 表示设计令牌系统中的各种值类型，支持颜色、尺寸、字符串等
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    /// 颜色值（十六进制、RGB、HSL等）
    Color(String),
    /// 尺寸值（像素、em、rem等）
    Size(String),
    /// 数字值
    Number(f64),
    /// 字符串值
    String(String),
    /// 布尔值
    Boolean(bool),
    /// 数组值
    Array(Vec<TokenValue>),
    /// 对象值
    Object(std::collections::HashMap<String, TokenValue>),
    /// 引用其他令牌
    Reference(String),
    /// 空值
    Null,
}

impl TokenValue {
    /// 创建颜色令牌值
    pub fn color(value: impl Into<String>) -> Self {
        Self::Color(value.into())
    }

    /// 创建尺寸令牌值
    pub fn size(value: impl Into<String>) -> Self {
        Self::Size(value.into())
    }

    /// 创建数字令牌值
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    /// 创建字符串令牌值
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// 创建布尔令牌值
    pub fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// 创建引用令牌值
    pub fn reference(path: impl Into<String>) -> Self {
        Self::Reference(path.into())
    }

    /// 检查是否为颜色值
    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_))
    }

    /// 检查是否为尺寸值
    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size(_))
    }

    /// 检查是否为数字值
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// 检查是否为字符串值
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// 检查是否为引用值
    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    /// 获取颜色值
    pub fn as_color(&self) -> Option<&str> {
        match self {
            Self::Color(value) => Some(value),
            _ => None,
        }
    }

    /// 获取尺寸值
    pub fn as_size(&self) -> Option<&str> {
        match self {
            Self::Size(value) => Some(value),
            _ => None,
        }
    }

    /// 获取数字值
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }

    /// 获取字符串值
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// 获取布尔值
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// 获取引用路径
    pub fn as_reference(&self) -> Option<&str> {
        match self {
            Self::Reference(path) => Some(path),
            _ => None,
        }
    }

    /// 转换为 CSS 值字符串
    pub fn to_css_value(&self) -> String {
        match self {
            Self::Color(value) => value.clone(),
            Self::Size(value) => value.clone(),
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.clone(),
            Self::Boolean(value) => value.to_string(),
            Self::Reference(path) => format!("var(--{})", path.replace(".", "-")),
            Self::Array(values) => values
                .iter()
                .map(|v| v.to_css_value())
                .collect::<Vec<_>>()
                .join(", "),
            Self::Object(_) => "/* object value */".to_string(),
            Self::Null => "initial".to_string(),
        }
    }

    /// 验证令牌值的有效性
    pub fn validate(&self) -> Result<(), TokenValueError> {
        match self {
            Self::Color(value) => {
                if value.is_empty() {
                    return Err(TokenValueError::EmptyValue("color".to_string()));
                }
                // 简单的颜色格式验证
                if !value.starts_with('#') && !value.starts_with("rgb") && !value.starts_with("hsl")
                {
                    if !is_named_color(value) {
                        return Err(TokenValueError::InvalidColorFormat(value.clone()));
                    }
                }
            }
            Self::Size(value) => {
                if value.is_empty() {
                    return Err(TokenValueError::EmptyValue("size".to_string()));
                }
                // 简单的尺寸格式验证
                if !value.ends_with("px")
                    && !value.ends_with("em")
                    && !value.ends_with("rem")
                    && !value.ends_with("%")
                    && !value.ends_with("vh")
                    && !value.ends_with("vw")
                {
                    return Err(TokenValueError::InvalidSizeFormat(value.clone()));
                }
            }
            Self::Reference(path) => {
                if path.is_empty() {
                    return Err(TokenValueError::EmptyValue("reference".to_string()));
                }
            }
            Self::Array(values) => {
                for value in values {
                    value.validate()?;
                }
            }
            Self::Object(map) => {
                for (_, value) in map {
                    value.validate()?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

impl From<String> for TokenValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for TokenValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<f64> for TokenValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for TokenValue {
    fn from(value: i32) -> Self {
        Self::Number(value as f64)
    }
}

impl From<bool> for TokenValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

/// 令牌值错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenValueError {
    /// 空值错误
    EmptyValue(String),
    /// 无效的颜色格式
    InvalidColorFormat(String),
    /// 无效的尺寸格式
    InvalidSizeFormat(String),
    /// 无效的引用路径
    InvalidReference(String),
    /// 类型不匹配
    TypeMismatch(String),
}

impl fmt::Display for TokenValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValueError::EmptyValue(type_name) => {
                write!(f, "Empty {} value", type_name)
            }
            TokenValueError::InvalidColorFormat(value) => {
                write!(f, "Invalid color format: {}", value)
            }
            TokenValueError::InvalidSizeFormat(value) => {
                write!(f, "Invalid size format: {}", value)
            }
            TokenValueError::InvalidReference(path) => {
                write!(f, "Invalid reference path: {}", path)
            }
            TokenValueError::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            }
        }
    }
}

impl std::error::Error for TokenValueError {}

/// 检查是否为命名颜色
fn is_named_color(color: &str) -> bool {
    const NAMED_COLORS: &[&str] = &[
        "red",
        "green",
        "blue",
        "white",
        "black",
        "gray",
        "grey",
        "yellow",
        "orange",
        "purple",
        "pink",
        "brown",
        "cyan",
        "magenta",
        "lime",
        "navy",
        "teal",
        "silver",
        "maroon",
        "olive",
        "aqua",
        "fuchsia",
        "transparent",
        "currentColor",
    ];

    NAMED_COLORS.contains(&color.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_value_creation() {
        let color = TokenValue::color("#ff0000");
        assert!(color.is_color());
        assert_eq!(color.as_color(), Some("#ff0000"));

        let size = TokenValue::size("16px");
        assert!(size.is_size());
        assert_eq!(size.as_size(), Some("16px"));

        let number = TokenValue::number(42.0);
        assert!(number.is_number());
        assert_eq!(number.as_number(), Some(42.0));
    }

    #[test]
    fn test_css_value_conversion() {
        let color = TokenValue::color("#ff0000");
        assert_eq!(color.to_css_value(), "#ff0000");

        let reference = TokenValue::reference("colors.primary.500");
        assert_eq!(reference.to_css_value(), "var(--colors-primary-500)");

        let array = TokenValue::Array(vec![TokenValue::size("10px"), TokenValue::size("20px")]);
        assert_eq!(array.to_css_value(), "10px, 20px");
    }

    #[test]
    fn test_validation() {
        let valid_color = TokenValue::color("#ff0000");
        assert!(valid_color.validate().is_ok());

        let invalid_color = TokenValue::color("invalid");
        assert!(invalid_color.validate().is_err());

        let valid_size = TokenValue::size("16px");
        assert!(valid_size.validate().is_ok());

        let invalid_size = TokenValue::size("invalid");
        assert!(invalid_size.validate().is_err());
    }

    #[test]
    fn test_from_conversions() {
        let from_string: TokenValue = "test".into();
        assert_eq!(from_string, TokenValue::String("test".to_string()));

        let from_number: TokenValue = 42.0.into();
        assert_eq!(from_number, TokenValue::Number(42.0));

        let from_bool: TokenValue = true.into();
        assert_eq!(from_bool, TokenValue::Boolean(true));
    }
}
