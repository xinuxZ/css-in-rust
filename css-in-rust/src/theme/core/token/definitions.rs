//! 设计令牌定义模块
//!
//! 本模块定义了设计令牌系统的基础抽象和类型，遵循单一职责原则。
//! 职责：定义令牌的基础接口、类型和约束
//!
//! 实现完整的分层令牌系统：
//! - 全局令牌（Global Tokens）：最基础的设计决策
//! - 别名令牌（Alias Tokens）：语义化的令牌引用
//! - 组件令牌（Component Tokens）：特定组件的令牌
//! - 令牌引用和变换系统

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
    /// 令牌引用（带变换）
    TokenReference(TokenReference),
    /// 颜色值
    Color(ColorValue),
    /// 尺寸值
    Dimension(DimensionValue),
    /// 字体值
    Typography(TypographyValue),
    /// 阴影值
    Shadow(ShadowValue),
    /// 空值
    Null,
}

/// 颜色值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorValue {
    pub hex: String,
    pub rgb: Option<(u8, u8, u8)>,
    pub hsl: Option<(f32, f32, f32)>,
    pub alpha: Option<f32>,
}

impl std::fmt::Display for ColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(alpha) = self.alpha {
            if alpha < 1.0 {
                if let Some((r, g, b)) = self.rgb {
                    write!(f, "rgba({}, {}, {}, {})", r, g, b, alpha)
                } else {
                    write!(f, "{}{}%", self.hex, (alpha * 100.0) as u8)
                }
            } else {
                write!(f, "{}", self.hex)
            }
        } else {
            write!(f, "{}", self.hex)
        }
    }
}

/// 尺寸值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionValue {
    pub value: f64,
    pub unit: DimensionUnit,
}

impl std::fmt::Display for DimensionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit {
            DimensionUnit::Px => write!(f, "{}px", self.value),
            DimensionUnit::Rem => write!(f, "{}rem", self.value),
            DimensionUnit::Em => write!(f, "{}em", self.value),
            DimensionUnit::Percent => write!(f, "{}%", self.value),
            DimensionUnit::Vh => write!(f, "{}vh", self.value),
            DimensionUnit::Vw => write!(f, "{}vw", self.value),
            DimensionUnit::Auto => write!(f, "auto"),
        }
    }
}

/// 尺寸单位
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionUnit {
    Px,
    Rem,
    Em,
    Percent,
    Vh,
    Vw,
    Auto,
}

impl std::fmt::Display for DimensionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px => write!(f, "px"),
            Self::Rem => write!(f, "rem"),
            Self::Em => write!(f, "em"),
            Self::Percent => write!(f, "%"),
            Self::Vh => write!(f, "vh"),
            Self::Vw => write!(f, "vw"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

/// 字体值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypographyValue {
    pub font_family: Option<String>,
    pub font_size: Option<DimensionValue>,
    pub font_weight: Option<u16>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<DimensionValue>,
}

/// 阴影值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowValue {
    pub x: DimensionValue,
    pub y: DimensionValue,
    pub blur: DimensionValue,
    pub spread: Option<DimensionValue>,
    pub color: ColorValue,
    pub inset: bool,
}

impl std::fmt::Display for ShadowValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inset_str = if self.inset { "inset " } else { "" };
        if let Some(spread) = &self.spread {
            write!(
                f,
                "{}{} {} {} {} {}",
                inset_str, self.x, self.y, self.blur, spread, self.color
            )
        } else {
            write!(
                f,
                "{}{} {} {} {}",
                inset_str, self.x, self.y, self.blur, self.color
            )
        }
    }
}

/// 令牌引用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenReference {
    /// 引用路径（如：global.color_palette.primary.500）
    pub reference: String,
    /// 可选的变换函数
    pub transform: Option<TokenTransform>,
}

/// 令牌变换
/// 令牌变换
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenTransform {
    /// 透明度变换
    Alpha(f32),
    /// 亮度变换
    Lighten(f32),
    /// 暗度变换
    Darken(f32),
    /// 饱和度变换
    Saturate(f32),
    /// 去饱和变换
    Desaturate(f32),
    /// 数学运算
    Math(MathOperation),
    /// 色相旋转
    HueRotate(f32),
    /// 对比度调整
    Contrast(f32),
    /// 缩放变换
    Scale(f32),
    /// 条件变换
    Conditional {
        condition: String,
        if_true: Box<TokenTransform>,
        if_false: Box<TokenTransform>,
    },
    /// 颜色修改
    ColorModify { operation: String, amount: f32 },
}

/// 数学运算
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MathOperation {
    Add(f32),
    Subtract(f32),
    Multiply(f32),
    Divide(f32),
    Min(f32),
    Max(f32),
    Clamp(f32, f32, f32), // min, value, max
}

/// CSS字符串转换特征
pub trait CssString {
    fn to_css_string(&self) -> String;
}

impl CssString for ColorValue {
    fn to_css_string(&self) -> String {
        if let Some(alpha) = self.alpha {
            if alpha < 1.0 {
                if let Some((r, g, b)) = self.rgb {
                    format!("rgba({}, {}, {}, {})", r, g, b, alpha)
                } else {
                    format!("{}{}%", self.hex, (alpha * 100.0) as u8)
                }
            } else {
                self.hex.clone()
            }
        } else {
            self.hex.clone()
        }
    }
}

impl CssString for DimensionValue {
    fn to_css_string(&self) -> String {
        format!("{}{}", self.value, self.unit)
    }
}

impl CssString for TypographyValue {
    fn to_css_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(weight) = self.font_weight {
            parts.push(weight.to_string());
        }

        if let Some(ref size) = self.font_size {
            let mut size_part = size.to_css_string();
            if let Some(line_height) = self.line_height {
                size_part.push_str(&format!("/{}", line_height));
            }
            parts.push(size_part);
        }

        if let Some(ref family) = self.font_family {
            parts.push(family.clone());
        }

        if parts.is_empty() {
            "inherit".to_string()
        } else {
            parts.join(" ")
        }
    }
}

impl CssString for ShadowValue {
    fn to_css_string(&self) -> String {
        let mut parts = Vec::new();

        if self.inset {
            parts.push("inset".to_string());
        }

        parts.push(self.x.to_css_string());
        parts.push(self.y.to_css_string());
        parts.push(self.blur.to_css_string());

        if let Some(ref spread) = self.spread {
            parts.push(spread.to_css_string());
        }

        parts.push(self.color.to_css_string());

        parts.join(" ")
    }
}

impl TokenValue {
    /// 创建字符串值
    pub fn string(value: String) -> Self {
        TokenValue::String(value)
    }

    /// 创建数字值
    pub fn number(value: f64) -> Self {
        TokenValue::Number(value)
    }

    /// 创建布尔值
    pub fn boolean(value: bool) -> Self {
        TokenValue::Boolean(value)
    }

    /// 创建颜色值
    pub fn color(value: ColorValue) -> Self {
        TokenValue::Color(value)
    }

    /// 创建维度值
    pub fn dimension(value: DimensionValue) -> Self {
        TokenValue::Dimension(value)
    }

    /// 创建引用值
    pub fn reference(value: TokenReference) -> Self {
        TokenValue::TokenReference(value)
    }

    /// 获取值类型
    pub fn value_type(&self) -> &'static str {
        match self {
            TokenValue::String(_) => "string",
            TokenValue::Number(_) => "number",
            TokenValue::Boolean(_) => "boolean",
            TokenValue::Color(_) => "color",
            TokenValue::Dimension(_) => "dimension",
            TokenValue::Reference(_) => "reference",
            TokenValue::Array(_) => "array",
            TokenValue::Object(_) => "object",
            TokenValue::TokenReference(_) => "token_reference",
            TokenValue::Typography(_) => "typography",
            TokenValue::Shadow(_) => "shadow",
            TokenValue::Null => "null",
        }
    }

    /// 转换为字符串表示
    pub fn to_string(&self) -> String {
        match self {
            TokenValue::String(s) => s.clone(),
            TokenValue::Number(n) => n.to_string(),
            TokenValue::Boolean(b) => b.to_string(),
            TokenValue::Reference(r) => format!("var(--{})", r.replace('.', "-")),
            TokenValue::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TokenValue::Object(obj) => format!(
                "{{{}}}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TokenValue::TokenReference(r) => r.get_reference().to_string(),
            TokenValue::Color(c) => c.to_css_string(),
            TokenValue::Dimension(d) => d.to_css_string(),
            TokenValue::Typography(t) => t.to_css_string(),
            TokenValue::Shadow(s) => s.to_css_string(),
            TokenValue::Null => "null".to_string(),
        }
    }

    /// 转换为CSS值
    pub fn to_css_value(&self) -> String {
        match self {
            TokenValue::String(s) => format!("\"{}\"", s),
            TokenValue::Number(n) => n.to_string(),
            TokenValue::Boolean(b) => b.to_string(),
            TokenValue::Reference(r) => format!("var(--{})", r.replace('.', "-")),
            TokenValue::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.to_css_value())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TokenValue::Object(obj) => format!(
                "{{{}}}",
                obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_css_value()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TokenValue::TokenReference(r) => {
                format!("var(--{})", r.get_reference().replace('.', "-"))
            }
            TokenValue::Color(c) => c.to_css_string(),
            TokenValue::Dimension(d) => d.to_css_string(),
            TokenValue::Typography(t) => t.to_css_string(),
            TokenValue::Shadow(s) => s.to_css_string(),
            TokenValue::Null => "null".to_string(),
        }
    }

    /// 获取数值（如果是数值类型）
    pub fn as_number(&self) -> Option<f64> {
        match self {
            TokenValue::Number(n) => Some(*n),
            TokenValue::Dimension(d) => Some(d.value),
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

    /// 获取颜色值
    pub fn as_color(&self) -> Option<&ColorValue> {
        match self {
            TokenValue::Color(c) => Some(c),
            _ => None,
        }
    }

    /// 获取尺寸值
    pub fn as_dimension(&self) -> Option<&DimensionValue> {
        match self {
            TokenValue::Dimension(d) => Some(d),
            _ => None,
        }
    }

    /// 获取令牌引用
    pub fn as_token_reference(&self) -> Option<&TokenReference> {
        match self {
            TokenValue::TokenReference(tr) => Some(tr),
            _ => None,
        }
    }

    /// 检查是否为引用类型
    pub fn is_reference(&self) -> bool {
        matches!(
            self,
            TokenValue::Reference(_) | TokenValue::TokenReference(_)
        )
    }

    /// 获取令牌类型
    pub fn token_type(&self) -> TokenType {
        match self {
            TokenValue::String(_) => TokenType::String,
            TokenValue::Number(_) => TokenType::Number,
            TokenValue::Boolean(_) => TokenType::Boolean,
            TokenValue::Reference(_) => TokenType::Reference,
            TokenValue::Array(_) => TokenType::Array,
            TokenValue::Object(_) => TokenType::Object,
            TokenValue::TokenReference(_) => TokenType::TokenReference,
            TokenValue::Color(_) => TokenType::Color,
            TokenValue::Dimension(_) => TokenType::Dimension,
            TokenValue::Typography(_) => TokenType::Typography,
            TokenValue::Shadow(_) => TokenType::Shadow,
            TokenValue::Null => TokenType::Null,
        }
    }
}

/// 令牌类型枚举
/// 令牌类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    String,
    Number,
    Boolean,
    Reference,
    Array,
    Object,
    TokenReference,
    Color,
    Dimension,
    Typography,
    Shadow,
    Null,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Boolean => write!(f, "Boolean"),
            TokenType::Reference => write!(f, "Reference"),
            TokenType::Array => write!(f, "Array"),
            TokenType::Object => write!(f, "Object"),
            TokenType::TokenReference => write!(f, "TokenReference"),
            TokenType::Color => write!(f, "Color"),
            TokenType::Dimension => write!(f, "Dimension"),
            TokenType::Typography => write!(f, "Typography"),
            TokenType::Shadow => write!(f, "Shadow"),
            TokenType::Null => write!(f, "Null"),
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

    /// 检查令牌是否有效
    pub fn is_valid(&self) -> bool {
        !self.segments.is_empty() && self.segments.iter().all(|s| !s.is_empty())
    }

    /// 获取最后一个段
    pub fn last_segment(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    /// 创建子路径
    pub fn child(&self, segment: &str) -> TokenPath {
        let mut new_path = self.clone();
        new_path.segments.push(segment.to_string());
        new_path
    }

    /// 获取令牌层级
    pub fn get_tier(&self) -> TokenTier {
        if self.segments.is_empty() {
            return TokenTier::Unknown;
        }

        match self.segments[0].as_str() {
            "global" => TokenTier::Global,
            "alias" => TokenTier::Alias,
            "component" => TokenTier::Component,
            _ => TokenTier::Unknown,
        }
    }
}

/// 令牌层级
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenTier {
    /// 全局令牌
    Global,
    /// 别名令牌
    Alias,
    /// 组件令牌
    Component,
    /// 未知层级
    Unknown,
}

/// 主题变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeVariant::Light => write!(f, "light"),
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::Auto => write!(f, "auto"),
        }
    }
}

// 实现各种值类型的方法

impl ColorValue {
    /// 创建新的颜色值
    pub fn new(hex: String) -> Self {
        Self {
            hex,
            rgb: None,
            hsl: None,
            alpha: None,
        }
    }

    /// 创建带透明度的颜色值
    pub fn with_alpha(hex: String, alpha: f32) -> Self {
        Self {
            hex,
            rgb: None,
            hsl: None,
            alpha: Some(alpha.clamp(0.0, 1.0)),
        }
    }

    /// 转换为CSS字符串
    pub fn to_css_string(&self) -> String {
        if let Some(alpha) = self.alpha {
            if alpha < 1.0 {
                // 如果有透明度且不为1，使用rgba格式
                if let Some((r, g, b)) = self.rgb {
                    format!("rgba({}, {}, {}, {})", r, g, b, alpha)
                } else {
                    // 从hex转换为rgba
                    let hex = self.hex.trim_start_matches('#');
                    if hex.len() == 6 {
                        if let (Ok(r), Ok(g), Ok(b)) = (
                            u8::from_str_radix(&hex[0..2], 16),
                            u8::from_str_radix(&hex[2..4], 16),
                            u8::from_str_radix(&hex[4..6], 16),
                        ) {
                            format!("rgba({}, {}, {}, {})", r, g, b, alpha)
                        } else {
                            self.hex.clone()
                        }
                    } else {
                        self.hex.clone()
                    }
                }
            } else {
                self.hex.clone()
            }
        } else {
            self.hex.clone()
        }
    }

    /// 变亮颜色
    pub fn lighten(&self, amount: f32) -> ColorValue {
        // 简单实现：返回当前颜色的副本
        // 在实际应用中，这里应该实现真正的颜色变亮逻辑
        let mut result = self.clone();
        result.hex = format!("{}/* lightened by {} */", self.hex, amount);
        result
    }

    /// 变暗颜色
    pub fn darken(&self, amount: f32) -> ColorValue {
        // 简单实现：返回当前颜色的副本
        // 在实际应用中，这里应该实现真正的颜色变暗逻辑
        let mut result = self.clone();
        result.hex = format!("{}/* darkened by {} */", self.hex, amount);
        result
    }

    /// 增加饱和度
    pub fn saturate(&self, amount: f32) -> ColorValue {
        // 简单实现：返回当前颜色的副本
        // 在实际应用中，这里应该实现真正的饱和度调整逻辑
        let mut result = self.clone();
        result.hex = format!("{}/* saturated by {} */", self.hex, amount);
        result
    }

    /// 降低饱和度
    pub fn desaturate(&self, amount: f32) -> ColorValue {
        // 简单实现：返回当前颜色的副本
        // 在实际应用中，这里应该实现真正的去饱和逻辑
        let mut result = self.clone();
        result.hex = format!("{}/* desaturated by {} */", self.hex, amount);
        result
    }

    /// 调整透明度
    pub fn fade(&self, alpha: f32) -> ColorValue {
        let mut result = self.clone();
        result.alpha = Some(alpha.clamp(0.0, 1.0));
        result
    }
}

impl DimensionValue {
    /// 创建新的尺寸值
    pub fn create(value: f64, unit: DimensionUnit) -> Self {
        Self { value, unit }
    }

    /// 创建像素值
    pub fn px(value: f64) -> Self {
        Self::create(value, DimensionUnit::Px)
    }

    /// 创建rem值
    pub fn rem(value: f64) -> Self {
        Self::create(value, DimensionUnit::Rem)
    }

    /// 创建em值
    pub fn em(value: f64) -> Self {
        Self::create(value, DimensionUnit::Em)
    }

    /// 创建百分比值
    pub fn percent(value: f64) -> Self {
        Self::create(value, DimensionUnit::Percent)
    }

    /// 缩放值
    pub fn scale(&self, factor: f32) -> Self {
        Self::create(self.value * factor as f64, self.unit.clone())
    }
}

impl TypographyValue {
    /// 创建新的字体值
    pub fn new() -> Self {
        Self {
            font_family: None,
            font_size: None,
            font_weight: None,
            line_height: None,
            letter_spacing: None,
        }
    }

    /// 设置字体族
    pub fn with_family(mut self, family: String) -> Self {
        self.font_family = Some(family);
        self
    }

    /// 设置字体大小
    pub fn with_size(mut self, size: DimensionValue) -> Self {
        self.font_size = Some(size);
        self
    }

    /// 设置字重
    pub fn with_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);
        self
    }

    /// 转换为CSS字符串（用于font简写属性）
    pub fn to_css_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(weight) = self.font_weight {
            parts.push(weight.to_string());
        }

        if let Some(ref size) = self.font_size {
            let mut size_part = size.to_css_string();
            if let Some(line_height) = self.line_height {
                size_part.push_str(&format!("/{}", line_height));
            }
            parts.push(size_part);
        }

        if let Some(ref family) = self.font_family {
            parts.push(family.clone());
        }

        if parts.is_empty() {
            "inherit".to_string()
        } else {
            parts.join(" ")
        }
    }
}

impl ShadowValue {
    /// 创建新的阴影值
    pub fn new(
        x: DimensionValue,
        y: DimensionValue,
        blur: DimensionValue,
        color: ColorValue,
    ) -> Self {
        Self {
            x,
            y,
            blur,
            spread: None,
            color,
            inset: false,
        }
    }

    /// 设置扩散半径
    pub fn with_spread(mut self, spread: DimensionValue) -> Self {
        self.spread = Some(spread);
        self
    }

    /// 设置为内阴影
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }

    /// 转换为CSS字符串
    pub fn to_css_string(&self) -> String {
        let mut parts = Vec::new();

        if self.inset {
            parts.push("inset".to_string());
        }

        parts.push(self.x.to_css_string());
        parts.push(self.y.to_css_string());
        parts.push(self.blur.to_css_string());

        if let Some(ref spread) = self.spread {
            parts.push(spread.to_css_string());
        }

        parts.push(self.color.to_css_string());

        parts.join(" ")
    }
}

impl TokenReference {
    /// 创建新的令牌引用
    pub fn create(reference: String) -> Self {
        Self {
            reference,
            transform: None,
        }
    }

    /// 创建带变换的令牌引用
    pub fn create_with_transform(reference: String, transform: TokenTransform) -> Self {
        Self {
            reference,
            transform: Some(transform),
        }
    }

    /// 获取引用路径
    pub fn get_reference(&self) -> &str {
        &self.reference
    }

    /// 获取变换
    pub fn get_transform(&self) -> Option<&TokenTransform> {
        self.transform.as_ref()
    }

    /// 转换为CSS变量引用
    pub fn to_css_var(&self) -> String {
        format!("var(--{})", self.reference.replace(".", "-"))
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

pub trait TokenDefinitions {
    fn get_token_value(&self, path: &str) -> Option<TokenValue>;
    fn set_token_value(&mut self, path: &str, value: TokenValue);
    fn get_metadata(&self, path: &str) -> Option<TokenMetadata>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefinitionsImpl {
    values: HashMap<String, TokenValue>,
    metadata: HashMap<String, TokenMetadata>,
}

impl Default for TokenDefinitionsImpl {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

impl TokenDefinitions for TokenDefinitionsImpl {
    fn get_token_value(&self, path: &str) -> Option<TokenValue> {
        self.values.get(path).cloned()
    }

    fn set_token_value(&mut self, path: &str, value: TokenValue) {
        self.values.insert(path.to_string(), value);
    }

    fn get_metadata(&self, path: &str) -> Option<TokenMetadata> {
        self.metadata.get(path).cloned()
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
    InvalidReference { path: String, reference: String },
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
            TokenValidationError::InvalidReference { path, reference } => {
                write!(f, "Invalid reference: {} in {}", reference, path)
            }
        }
    }
}

impl std::error::Error for TokenValidationError {}

/// 从String类型转换为TokenValidationError
impl From<String> for TokenValidationError {
    fn from(error: String) -> Self {
        TokenValidationError::InvalidValue(error)
    }
}

/// 从&str类型转换为TokenValidationError
impl From<&str> for TokenValidationError {
    fn from(error: &str) -> Self {
        TokenValidationError::InvalidValue(error.to_string())
    }
}
