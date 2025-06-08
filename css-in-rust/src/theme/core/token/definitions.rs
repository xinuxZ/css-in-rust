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

/// 令牌值
///
/// 表示设计系统中的各种类型的令牌值，如颜色、尺寸、字体等。
/// 令牌值可以是基本类型（如字符串、数字）或复合类型（如颜色、尺寸）。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::token::definitions::{TokenValue, ColorValue, DimensionValue, DimensionUnit};
///
/// // 创建字符串令牌值
/// let string_token = TokenValue::string("Roboto".to_string());
///
/// // 创建数字令牌值
/// let number_token = TokenValue::number(16.0);
///
/// // 创建颜色令牌值
/// let color = ColorValue::new("#007bff".to_string());
/// let color_token = TokenValue::color(color);
///
/// // 创建尺寸令牌值
/// let dimension = DimensionValue::px(16.0);
/// let dimension_token = TokenValue::dimension(dimension);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    /// 字符串值（如颜色、字体名称）
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let font_family = TokenValue::String("Roboto, sans-serif".to_string());
    /// ```
    String(String),

    /// 数值（如尺寸、权重）
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let font_weight = TokenValue::Number(500.0);
    /// ```
    Number(f64),

    /// 布尔值（如是否启用）
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let is_rounded = TokenValue::Boolean(true);
    /// ```
    Boolean(bool),

    /// 引用其他令牌
    ///
    /// 使用字符串路径直接引用其他令牌，不支持变换。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let primary_color_ref = TokenValue::Reference("colors.primary".to_string());
    /// ```
    Reference(String),

    /// 数组值（如字体栈）
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let font_stack = TokenValue::Array(vec![
    ///     TokenValue::String("Roboto".to_string()),
    ///     TokenValue::String("Arial".to_string()),
    ///     TokenValue::String("sans-serif".to_string()),
    /// ]);
    /// ```
    Array(Vec<TokenValue>),

    /// 对象值（如复合属性）
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    /// use std::collections::HashMap;
    ///
    /// let mut button_props = HashMap::new();
    /// button_props.insert("padding".to_string(), TokenValue::String("8px 16px".to_string()));
    /// button_props.insert("borderRadius".to_string(), TokenValue::String("4px".to_string()));
    ///
    /// let button = TokenValue::Object(button_props);
    /// ```
    Object(HashMap<String, TokenValue>),

    /// 令牌引用（带变换）
    ///
    /// 使用TokenReference结构体引用其他令牌，支持应用变换。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, TokenReference, TokenTransform};
    ///
    /// // 引用主色并应用50%透明度
    /// let reference = TokenReference::create_with_transform(
    ///     "colors.primary".to_string(),
    ///     TokenTransform::Alpha(0.5)
    /// );
    /// let token = TokenValue::TokenReference(reference);
    /// ```
    TokenReference(TokenReference),

    /// 颜色值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, ColorValue};
    ///
    /// let color = ColorValue::new("#007bff".to_string());
    /// let color_token = TokenValue::Color(color);
    ///
    /// // 带透明度的颜色
    /// let transparent_color = ColorValue::with_alpha("#007bff".to_string(), 0.5);
    /// let transparent_token = TokenValue::Color(transparent_color);
    /// ```
    Color(ColorValue),

    /// 尺寸值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, DimensionValue, DimensionUnit};
    ///
    /// // 像素尺寸
    /// let px_size = DimensionValue::px(16.0);
    /// let px_token = TokenValue::Dimension(px_size);
    ///
    /// // rem尺寸
    /// let rem_size = DimensionValue::rem(1.0);
    /// let rem_token = TokenValue::Dimension(rem_size);
    ///
    /// // 百分比尺寸
    /// let percent_size = DimensionValue::percent(100.0);
    /// let percent_token = TokenValue::Dimension(percent_size);
    /// ```
    Dimension(DimensionValue),

    /// 字体值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, TypographyValue, DimensionValue};
    ///
    /// let mut typography = TypographyValue::new();
    /// typography = typography
    ///     .with_family("Roboto, sans-serif".to_string())
    ///     .with_size(DimensionValue::px(16.0))
    ///     .with_weight(500);
    ///
    /// let typography_token = TokenValue::Typography(typography);
    /// ```
    Typography(TypographyValue),

    /// 阴影值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, ShadowValue, DimensionValue, ColorValue};
    ///
    /// let shadow = ShadowValue::new(
    ///     DimensionValue::px(0.0),
    ///     DimensionValue::px(4.0),
    ///     DimensionValue::px(8.0),
    ///     ColorValue::with_alpha("#000000".to_string(), 0.2)
    /// ).with_spread(DimensionValue::px(0.0));
    ///
    /// let shadow_token = TokenValue::Shadow(shadow);
    /// ```
    Shadow(ShadowValue),

    /// 空值
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let null_token = TokenValue::Null;
    /// ```
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
            TokenValue::Color(_) => TokenType::Color,
            TokenValue::Typography(_) => TokenType::Typography,
            TokenValue::Shadow(_) => TokenType::Shadow,
            _ => TokenType::Custom(self.value_type().to_string()),
        }
    }
}

/// 令牌类型枚举
/// 令牌类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    /// 颜色令牌
    Color,
    /// 间距令牌
    Spacing,
    /// 排版令牌
    Typography,
    /// 边框令牌
    Border,
    /// 阴影令牌
    Shadow,
    /// 圆角令牌
    Radius,
    /// 层叠令牌
    ZIndex,
    /// 不透明度令牌
    Opacity,
    /// 动画令牌
    Animation,
    /// 自定义令牌
    Custom(String),
}

impl TokenType {
    /// 获取令牌类型的CSS前缀
    ///
    /// 返回用于CSS变量的前缀，例如 `--color-`、`--spacing-` 等。
    ///
    /// # Returns
    ///
    /// 令牌类型对应的CSS变量前缀
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenType;
    ///
    /// let color_token = TokenType::Color;
    /// assert_eq!(color_token.css_prefix(), "--color-");
    ///
    /// let custom_token = TokenType::Custom("brand".to_string());
    /// assert_eq!(custom_token.css_prefix(), "--brand-");
    /// ```
    pub fn css_prefix(&self) -> String {
        match self {
            TokenType::Color => "--color-".to_string(),
            TokenType::Spacing => "--spacing-".to_string(),
            TokenType::Typography => "--typography-".to_string(),
            TokenType::Border => "--border-".to_string(),
            TokenType::Shadow => "--shadow-".to_string(),
            TokenType::Radius => "--radius-".to_string(),
            TokenType::ZIndex => "--z-index-".to_string(),
            TokenType::Opacity => "--opacity-".to_string(),
            TokenType::Animation => "--animation-".to_string(),
            TokenType::Custom(name) => format!("--{}-", name),
        }
    }

    /// 获取令牌类型的名称
    ///
    /// 返回令牌类型的字符串表示
    ///
    /// # Returns
    ///
    /// 令牌类型的名称
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenType;
    ///
    /// let color_token = TokenType::Color;
    /// assert_eq!(color_token.name(), "color");
    ///
    /// let custom_token = TokenType::Custom("brand".to_string());
    /// assert_eq!(custom_token.name(), "brand");
    /// ```
    pub fn name(&self) -> String {
        match self {
            TokenType::Color => "color".to_string(),
            TokenType::Spacing => "spacing".to_string(),
            TokenType::Typography => "typography".to_string(),
            TokenType::Border => "border".to_string(),
            TokenType::Shadow => "shadow".to_string(),
            TokenType::Radius => "radius".to_string(),
            TokenType::ZIndex => "z-index".to_string(),
            TokenType::Opacity => "opacity".to_string(),
            TokenType::Animation => "animation".to_string(),
            TokenType::Custom(name) => name.clone(),
        }
    }

    /// 从字符串创建令牌类型
    ///
    /// # Arguments
    ///
    /// * `name` - 令牌类型名称
    ///
    /// # Returns
    ///
    /// 对应的令牌类型
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::token::definitions::TokenType;
    ///
    /// let color_token = TokenType::from_str("color");
    /// assert_eq!(color_token, TokenType::Color);
    ///
    /// let custom_token = TokenType::from_str("brand");
    /// assert_eq!(custom_token, TokenType::Custom("brand".to_string()));
    /// ```
    pub fn from_str(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "color" => TokenType::Color,
            "spacing" => TokenType::Spacing,
            "typography" => TokenType::Typography,
            "border" => TokenType::Border,
            "shadow" => TokenType::Shadow,
            "radius" => TokenType::Radius,
            "z-index" | "zindex" => TokenType::ZIndex,
            "opacity" => TokenType::Opacity,
            "animation" => TokenType::Animation,
            _ => TokenType::Custom(name.to_string()),
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
        // 将颜色转换为HSL，增加亮度，然后转回原格式
        let (h, s, l) = self.to_hsl();
        let new_l = (l + amount).clamp(0.0, 1.0);
        self.from_hsl(h, s, new_l)
    }

    /// 变暗颜色
    pub fn darken(&self, amount: f32) -> ColorValue {
        // 将颜色转换为HSL，降低亮度，然后转回原格式
        let (h, s, l) = self.to_hsl();
        let new_l = (l - amount).clamp(0.0, 1.0);
        self.from_hsl(h, s, new_l)
    }

    /// 增加饱和度
    pub fn saturate(&self, amount: f32) -> ColorValue {
        // 将颜色转换为HSL，增加饱和度，然后转回原格式
        let (h, s, l) = self.to_hsl();
        let new_s = (s + amount).clamp(0.0, 1.0);
        self.from_hsl(h, new_s, l)
    }

    /// 降低饱和度
    pub fn desaturate(&self, amount: f32) -> ColorValue {
        // 将颜色转换为HSL，降低饱和度，然后转回原格式
        let (h, s, l) = self.to_hsl();
        let new_s = (s - amount).clamp(0.0, 1.0);
        self.from_hsl(h, new_s, l)
    }

    /// 调整透明度
    pub fn fade(&self, alpha: f32) -> ColorValue {
        let mut result = self.clone();
        result.alpha = Some(alpha.clamp(0.0, 1.0));
        result
    }

    /// 将颜色转换为HSL格式
    fn to_hsl(&self) -> (f32, f32, f32) {
        // 如果已经有HSL值，直接返回
        if let Some(hsl) = self.hsl {
            return hsl;
        }

        // 从RGB转换为HSL
        if let Some((r, g, b)) = self.rgb {
            return Self::rgb_to_hsl(r, g, b);
        }

        // 从HEX转换为HSL
        let hex = self.hex.trim_start_matches('#');
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return Self::rgb_to_hsl(r, g, b);
            }
        }

        // 默认返回黑色的HSL值
        (0.0, 0.0, 0.0)
    }

    /// 从HSL创建颜色
    fn from_hsl(&self, h: f32, s: f32, l: f32) -> ColorValue {
        // 从HSL转换为RGB
        let (r, g, b) = Self::hsl_to_rgb(h, s, l);

        // 创建新的颜色值
        let mut result = self.clone();
        result.rgb = Some((r, g, b));
        result.hsl = Some((h, s, l));

        // 更新HEX值
        result.hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

        result
    }

    /// RGB转HSL的辅助方法
    fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r_f = r as f32 / 255.0;
        let g_f = g as f32 / 255.0;
        let b_f = b as f32 / 255.0;

        let max = r_f.max(g_f).max(b_f);
        let min = r_f.min(g_f).min(b_f);
        let delta = max - min;

        // 计算亮度
        let l = (max + min) / 2.0;

        // 灰度没有饱和度和色相
        if delta.abs() < 0.00001 {
            return (0.0, 0.0, l);
        }

        // 计算饱和度
        let s = if l > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };

        // 计算色相
        let h = if r_f == max {
            (g_f - b_f) / delta + (if g_f < b_f { 6.0 } else { 0.0 })
        } else if g_f == max {
            (b_f - r_f) / delta + 2.0
        } else {
            (r_f - g_f) / delta + 4.0
        };

        (h * 60.0 / 360.0, s, l)
    }

    /// HSL转RGB的辅助方法
    fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        // 无饱和度时为灰度
        if s.abs() < 0.00001 {
            let value = (l * 255.0) as u8;
            return (value, value, value);
        }

        let h = h * 360.0; // 转换为0-360度

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0 * l - q;

        let r = Self::hue_to_rgb(p, q, h + 120.0);
        let g = Self::hue_to_rgb(p, q, h);
        let b = Self::hue_to_rgb(p, q, h - 120.0);

        ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }

    /// 色相转RGB分量的辅助方法
    fn hue_to_rgb(p: f32, q: f32, mut h: f32) -> f32 {
        // 确保h在0-360范围内
        while h < 0.0 {
            h += 360.0;
        }
        while h >= 360.0 {
            h -= 360.0;
        }

        if h < 60.0 {
            p + (q - p) * h / 60.0
        } else if h < 180.0 {
            q
        } else if h < 240.0 {
            p + (q - p) * (240.0 - h) / 60.0
        } else {
            p
        }
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
    TypeMismatch {
        expected: String,
        actual: String,
    },
    InvalidReference {
        path: String,
        reference: String,
    },
    /// 无效的表达式
    InvalidExpression(String),
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
            TokenValidationError::InvalidExpression(expression) => {
                write!(f, "Invalid expression: {}", expression)
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
