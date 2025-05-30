//! 变体类型定义模块
//!
//! 定义各种变体的具体类型和枚举，提供类型安全的变体操作。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 尺寸变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SizeVariant {
    /// 超小尺寸
    XS,
    /// 小尺寸
    SM,
    /// 中等尺寸（默认）
    MD,
    /// 大尺寸
    LG,
    /// 超大尺寸
    XL,
    /// 自定义尺寸
    Custom(String),
}

/// 颜色变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ColorVariant {
    /// 主色调
    Primary,
    /// 次要色调
    Secondary,
    /// 成功色
    Success,
    /// 警告色
    Warning,
    /// 危险色
    Danger,
    /// 信息色
    Info,
    /// 浅色
    Light,
    /// 深色
    Dark,
    /// 默认色
    Default,
    /// 自定义颜色
    Custom(String),
}

/// 状态变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StateVariant {
    /// 正常状态
    Normal,
    /// 悬停状态
    Hover,
    /// 激活状态
    Active,
    /// 焦点状态
    Focus,
    /// 禁用状态
    Disabled,
    /// 加载状态
    Loading,
    /// 选中状态
    Selected,
    /// 错误状态
    Error,
    /// 自定义状态
    Custom(String),
}

/// 形状变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ShapeVariant {
    /// 默认形状
    Default,
    /// 圆角
    Round,
    /// 圆形
    Circle,
    /// 方形
    Square,
    /// 自定义形状
    Custom(String),
}

/// 布局变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LayoutVariant {
    /// 块级布局
    Block,
    /// 内联布局
    Inline,
    /// 弹性布局
    Flex,
    /// 网格布局
    Grid,
    /// 绝对定位
    Absolute,
    /// 相对定位
    Relative,
    /// 固定定位
    Fixed,
    /// 粘性定位
    Sticky,
    /// 自定义布局
    Custom(String),
}

/// 变体值枚举
///
/// 统一的变体值类型，支持所有变体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariantValue {
    /// 尺寸变体
    Size(SizeVariant),
    /// 颜色变体
    Color(ColorVariant),
    /// 状态变体
    State(StateVariant),
    /// 形状变体
    Shape(ShapeVariant),
    /// 布局变体
    Layout(LayoutVariant),
    /// 字符串值
    String(String),
    /// 布尔值
    Boolean(bool),
    /// 数值
    Number(f64),
}

/// 变体组合
///
/// 表示多个变体的组合
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariantCombination {
    /// 变体映射
    pub variants: HashMap<String, VariantValue>,
    /// 组合名称
    pub name: Option<String>,
    /// 优先级
    pub priority: u32,
}

impl SizeVariant {
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            SizeVariant::XS => "xs".to_string(),
            SizeVariant::SM => "sm".to_string(),
            SizeVariant::MD => "md".to_string(),
            SizeVariant::LG => "lg".to_string(),
            SizeVariant::XL => "xl".to_string(),
            SizeVariant::Custom(s) => s.clone(),
        }
    }

    /// 从字符串解析
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "xs" => Ok(SizeVariant::XS),
            "sm" => Ok(SizeVariant::SM),
            "md" => Ok(SizeVariant::MD),
            "lg" => Ok(SizeVariant::LG),
            "xl" => Ok(SizeVariant::XL),
            _ => Ok(SizeVariant::Custom(s.to_string())),
        }
    }
}

impl ColorVariant {
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            ColorVariant::Primary => "primary".to_string(),
            ColorVariant::Secondary => "secondary".to_string(),
            ColorVariant::Success => "success".to_string(),
            ColorVariant::Warning => "warning".to_string(),
            ColorVariant::Danger => "danger".to_string(),
            ColorVariant::Info => "info".to_string(),
            ColorVariant::Light => "light".to_string(),
            ColorVariant::Dark => "dark".to_string(),
            ColorVariant::Default => "default".to_string(),
            ColorVariant::Custom(s) => s.clone(),
        }
    }

    /// 从字符串解析
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "primary" => Ok(ColorVariant::Primary),
            "secondary" => Ok(ColorVariant::Secondary),
            "success" => Ok(ColorVariant::Success),
            "warning" => Ok(ColorVariant::Warning),
            "danger" => Ok(ColorVariant::Danger),
            "info" => Ok(ColorVariant::Info),
            "light" => Ok(ColorVariant::Light),
            "dark" => Ok(ColorVariant::Dark),
            "default" => Ok(ColorVariant::Default),
            _ => Ok(ColorVariant::Custom(s.to_string())),
        }
    }
}

impl StateVariant {
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            StateVariant::Normal => "normal".to_string(),
            StateVariant::Hover => "hover".to_string(),
            StateVariant::Active => "active".to_string(),
            StateVariant::Focus => "focus".to_string(),
            StateVariant::Disabled => "disabled".to_string(),
            StateVariant::Loading => "loading".to_string(),
            StateVariant::Selected => "selected".to_string(),
            StateVariant::Error => "error".to_string(),
            StateVariant::Custom(s) => s.clone(),
        }
    }

    /// 从字符串解析
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(StateVariant::Normal),
            "hover" => Ok(StateVariant::Hover),
            "active" => Ok(StateVariant::Active),
            "focus" => Ok(StateVariant::Focus),
            "disabled" => Ok(StateVariant::Disabled),
            "loading" => Ok(StateVariant::Loading),
            "selected" => Ok(StateVariant::Selected),
            "error" => Ok(StateVariant::Error),
            _ => Ok(StateVariant::Custom(s.to_string())),
        }
    }

    /// 获取对应的 CSS 伪类
    pub fn to_pseudo_class(&self) -> Option<String> {
        match self {
            StateVariant::Hover => Some(":hover".to_string()),
            StateVariant::Active => Some(":active".to_string()),
            StateVariant::Focus => Some(":focus".to_string()),
            StateVariant::Disabled => Some(":disabled".to_string()),
            _ => None,
        }
    }
}

impl VariantValue {
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            VariantValue::Size(s) => s.to_string(),
            VariantValue::Color(c) => c.to_string(),
            VariantValue::State(st) => st.to_string(),
            VariantValue::Shape(sh) => match sh {
                ShapeVariant::Default => "default".to_string(),
                ShapeVariant::Round => "round".to_string(),
                ShapeVariant::Circle => "circle".to_string(),
                ShapeVariant::Square => "square".to_string(),
                ShapeVariant::Custom(s) => s.clone(),
            },
            VariantValue::Layout(l) => match l {
                LayoutVariant::Block => "block".to_string(),
                LayoutVariant::Inline => "inline".to_string(),
                LayoutVariant::Flex => "flex".to_string(),
                LayoutVariant::Grid => "grid".to_string(),
                LayoutVariant::Absolute => "absolute".to_string(),
                LayoutVariant::Relative => "relative".to_string(),
                LayoutVariant::Fixed => "fixed".to_string(),
                LayoutVariant::Sticky => "sticky".to_string(),
                LayoutVariant::Custom(s) => s.clone(),
            },
            VariantValue::String(s) => s.clone(),
            VariantValue::Boolean(b) => b.to_string(),
            VariantValue::Number(n) => n.to_string(),
        }
    }

    /// 从字符串和类型解析
    pub fn from_string(variant_type: &str, value: &str) -> Result<Self, String> {
        match variant_type {
            "size" => Ok(VariantValue::Size(SizeVariant::from_string(value)?)),
            "color" => Ok(VariantValue::Color(ColorVariant::from_string(value)?)),
            "state" => Ok(VariantValue::State(StateVariant::from_string(value)?)),
            "shape" => Ok(VariantValue::Shape(match value {
                "default" => ShapeVariant::Default,
                "round" => ShapeVariant::Round,
                "circle" => ShapeVariant::Circle,
                "square" => ShapeVariant::Square,
                _ => ShapeVariant::Custom(value.to_string()),
            })),
            "layout" => Ok(VariantValue::Layout(match value {
                "block" => LayoutVariant::Block,
                "inline" => LayoutVariant::Inline,
                "flex" => LayoutVariant::Flex,
                "grid" => LayoutVariant::Grid,
                "absolute" => LayoutVariant::Absolute,
                "relative" => LayoutVariant::Relative,
                "fixed" => LayoutVariant::Fixed,
                "sticky" => LayoutVariant::Sticky,
                _ => LayoutVariant::Custom(value.to_string()),
            })),
            _ => {
                // 尝试解析为其他类型
                if let Ok(b) = value.parse::<bool>() {
                    Ok(VariantValue::Boolean(b))
                } else if let Ok(n) = value.parse::<f64>() {
                    Ok(VariantValue::Number(n))
                } else {
                    Ok(VariantValue::String(value.to_string()))
                }
            }
        }
    }
}

impl VariantCombination {
    /// 创建新的变体组合
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            name: None,
            priority: 0,
        }
    }

    /// 添加变体
    pub fn with_variant(mut self, key: &str, value: VariantValue) -> Self {
        self.variants.insert(key.to_string(), value);
        self
    }

    /// 设置名称
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// 获取变体值
    pub fn get_variant(&self, key: &str) -> Option<&VariantValue> {
        self.variants.get(key)
    }

    /// 检查是否包含变体
    pub fn has_variant(&self, key: &str) -> bool {
        self.variants.contains_key(key)
    }

    /// 合并其他变体组合
    pub fn merge(&mut self, other: &VariantCombination) {
        for (key, value) in &other.variants {
            self.variants.insert(key.clone(), value.clone());
        }
        if other.priority > self.priority {
            self.priority = other.priority;
        }
    }
}

impl Default for VariantCombination {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_variant_conversion() {
        let size = SizeVariant::MD;
        assert_eq!(size.to_string(), "md");

        let parsed = SizeVariant::from_string("lg").unwrap();
        assert_eq!(parsed, SizeVariant::LG);
    }

    #[test]
    fn test_color_variant_conversion() {
        let color = ColorVariant::Primary;
        assert_eq!(color.to_string(), "primary");

        let parsed = ColorVariant::from_string("danger").unwrap();
        assert_eq!(parsed, ColorVariant::Danger);
    }

    #[test]
    fn test_state_variant_pseudo_class() {
        let hover = StateVariant::Hover;
        assert_eq!(hover.to_pseudo_class(), Some(":hover".to_string()));

        let normal = StateVariant::Normal;
        assert_eq!(normal.to_pseudo_class(), None);
    }

    #[test]
    fn test_variant_combination() {
        let combination = VariantCombination::new()
            .with_variant("size", VariantValue::Size(SizeVariant::LG))
            .with_variant("color", VariantValue::Color(ColorVariant::Primary))
            .with_name("large-primary")
            .with_priority(10);

        assert!(combination.has_variant("size"));
        assert!(combination.has_variant("color"));
        assert_eq!(combination.name, Some("large-primary".to_string()));
        assert_eq!(combination.priority, 10);
    }
}
