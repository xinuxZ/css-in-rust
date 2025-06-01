//! 设计令牌系统
//!
//! 提供设计令牌的核心管理功能，包括：
//! - 令牌解析和验证
//! - 令牌引用解析
//! - 令牌变换和计算
//! - 令牌导入导出
//! - 令牌版本管理

use super::design_tokens::DesignTokens;
use super::token_value::TokenValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 设计令牌系统错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenSystemError {
    /// 令牌未找到
    TokenNotFound(String),
    /// 循环引用
    CircularReference(String),
    /// 无效的令牌引用
    InvalidReference(String),
    /// 类型不匹配
    TypeMismatch(String),
    /// 解析错误
    ParseError(String),
    /// 验证错误
    ValidationError(String),
}

impl std::fmt::Display for TokenSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenNotFound(token) => write!(f, "令牌未找到: {}", token),
            Self::CircularReference(path) => write!(f, "检测到循环引用: {}", path),
            Self::InvalidReference(reference) => write!(f, "无效的令牌引用: {}", reference),
            Self::TypeMismatch(msg) => write!(f, "类型不匹配: {}", msg),
            Self::ParseError(msg) => write!(f, "解析错误: {}", msg),
            Self::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl std::error::Error for TokenSystemError {}

/// 设计令牌系统结果类型
pub type TokenSystemResult<T> = Result<T, TokenSystemError>;

/// 令牌引用路径
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenPath {
    /// 路径段
    pub segments: Vec<String>,
}

impl TokenPath {
    /// 创建新的令牌路径
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    /// 从字符串创建令牌路径
    pub fn from_string(path: &str) -> TokenSystemResult<Self> {
        if path.is_empty() {
            return Err(TokenSystemError::ParseError("路径不能为空".to_string()));
        }

        let segments: Vec<String> = path
            .split('.')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return Err(TokenSystemError::ParseError("无效的路径格式".to_string()));
        }

        Ok(Self::new(segments))
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        self.segments.join(".")
    }

    /// 获取父路径
    pub fn parent(&self) -> Option<TokenPath> {
        if self.segments.len() > 1 {
            Some(TokenPath::new(
                self.segments[..self.segments.len() - 1].to_vec(),
            ))
        } else {
            None
        }
    }

    /// 获取最后一个段
    pub fn last_segment(&self) -> Option<&String> {
        self.segments.last()
    }

    /// 添加段
    pub fn append(&self, segment: &str) -> TokenPath {
        let mut segments = self.segments.clone();
        segments.push(segment.to_string());
        TokenPath::new(segments)
    }
}

/// 令牌引用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenReference {
    /// 引用路径
    pub path: TokenPath,
    /// 变换函数（可选）
    pub transform: Option<TokenTransform>,
}

impl TokenReference {
    /// 创建新的令牌引用
    pub fn new(path: TokenPath) -> Self {
        Self {
            path,
            transform: None,
        }
    }

    /// 添加变换
    pub fn with_transform(mut self, transform: TokenTransform) -> Self {
        self.transform = Some(transform);
        self
    }

    /// 从字符串解析引用
    pub fn from_string(reference: &str) -> TokenSystemResult<Self> {
        // 解析格式: {path.to.token} 或 {path.to.token|transform}
        let reference = reference.trim();

        if !reference.starts_with('{') || !reference.ends_with('}') {
            return Err(TokenSystemError::InvalidReference(
                "引用必须以 { 开始并以 } 结束".to_string(),
            ));
        }

        let content = &reference[1..reference.len() - 1];
        let parts: Vec<&str> = content.split('|').collect();

        let path = TokenPath::from_string(parts[0])?;
        let transform = if parts.len() > 1 {
            Some(TokenTransform::from_string(parts[1])?)
        } else {
            None
        };

        Ok(Self { path, transform })
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        let path_str = self.path.to_string();
        if let Some(transform) = &self.transform {
            format!("{{{path_str}|{}}}", transform.to_string())
        } else {
            format!("{{{path_str}}}")
        }
    }
}

/// 令牌变换
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenTransform {
    /// 透明度变换
    Alpha(f64),
    /// 亮度变换
    Lighten(f64),
    /// 暗度变换
    Darken(f64),
    /// 饱和度变换
    Saturate(f64),
    /// 去饱和变换
    Desaturate(f64),
    /// 数值乘法
    Multiply(f64),
    /// 数值除法
    Divide(f64),
    /// 数值加法
    Add(f64),
    /// 数值减法
    Subtract(f64),
    /// 单位转换
    ConvertUnit(String),
    /// 自定义函数
    Custom(String, Vec<String>),
}

impl TokenTransform {
    /// 从字符串解析变换
    pub fn from_string(transform: &str) -> TokenSystemResult<Self> {
        let transform = transform.trim();

        if let Some(captures) = regex::Regex::new(r"^(\w+)\(([^)]+)\)$")
            .unwrap()
            .captures(transform)
        {
            let function = &captures[1];
            let args = &captures[2];

            match function {
                "alpha" => {
                    let value: f64 = args
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的透明度值".to_string()))?;
                    Ok(Self::Alpha(value))
                }
                "lighten" => {
                    let value: f64 = args
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的亮度值".to_string()))?;
                    Ok(Self::Lighten(value))
                }
                "darken" => {
                    let value: f64 = args
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的暗度值".to_string()))?;
                    Ok(Self::Darken(value))
                }
                "multiply" => {
                    let value: f64 = args
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的乘数值".to_string()))?;
                    Ok(Self::Multiply(value))
                }
                "divide" => {
                    let value: f64 = args
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的除数值".to_string()))?;
                    Ok(Self::Divide(value))
                }
                _ => {
                    let args: Vec<String> = args.split(',').map(|s| s.trim().to_string()).collect();
                    Ok(Self::Custom(function.to_string(), args))
                }
            }
        } else {
            Err(TokenSystemError::ParseError(format!(
                "无效的变换格式: {}",
                transform
            )))
        }
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            Self::Alpha(value) => format!("alpha({})", value),
            Self::Lighten(value) => format!("lighten({})", value),
            Self::Darken(value) => format!("darken({})", value),
            Self::Saturate(value) => format!("saturate({})", value),
            Self::Desaturate(value) => format!("desaturate({})", value),
            Self::Multiply(value) => format!("multiply({})", value),
            Self::Divide(value) => format!("divide({})", value),
            Self::Add(value) => format!("add({})", value),
            Self::Subtract(value) => format!("subtract({})", value),
            Self::ConvertUnit(unit) => format!("convert({})", unit),
            Self::Custom(name, args) => format!("{}({})", name, args.join(", ")),
        }
    }

    /// 应用变换到令牌值
    pub fn apply(&self, value: &TokenValue) -> TokenSystemResult<TokenValue> {
        match self {
            Self::Alpha(alpha) => self.apply_alpha(value, *alpha),
            Self::Lighten(amount) => self.apply_lighten(value, *amount),
            Self::Darken(amount) => self.apply_darken(value, *amount),
            Self::Multiply(factor) => self.apply_multiply(value, *factor),
            Self::Divide(divisor) => self.apply_divide(value, *divisor),
            Self::Add(addend) => self.apply_add(value, *addend),
            Self::Subtract(subtrahend) => self.apply_subtract(value, *subtrahend),
            _ => Err(TokenSystemError::TypeMismatch(
                "不支持的变换类型".to_string(),
            )),
        }
    }

    /// 应用透明度变换
    fn apply_alpha(&self, value: &TokenValue, alpha: f64) -> TokenSystemResult<TokenValue> {
        match value {
            TokenValue::Color(color) => {
                // 这里应该实现颜色透明度变换
                // 简化实现，返回原值
                Ok(value.clone())
            }
            _ => Err(TokenSystemError::TypeMismatch(
                "透明度变换只能应用于颜色值".to_string(),
            )),
        }
    }

    /// 应用亮度变换
    fn apply_lighten(&self, value: &TokenValue, amount: f64) -> TokenSystemResult<TokenValue> {
        match value {
            TokenValue::Color(color) => {
                // 这里应该实现颜色亮度变换
                // 简化实现，返回原值
                Ok(value.clone())
            }
            _ => Err(TokenSystemError::TypeMismatch(
                "亮度变换只能应用于颜色值".to_string(),
            )),
        }
    }

    /// 应用暗度变换
    fn apply_darken(&self, value: &TokenValue, amount: f64) -> TokenSystemResult<TokenValue> {
        match value {
            TokenValue::Color(color) => {
                // 这里应该实现颜色暗度变换
                // 简化实现，返回原值
                Ok(value.clone())
            }
            _ => Err(TokenSystemError::TypeMismatch(
                "暗度变换只能应用于颜色值".to_string(),
            )),
        }
    }

    /// 应用乘法变换
    fn apply_multiply(&self, value: &TokenValue, factor: f64) -> TokenSystemResult<TokenValue> {
        match value {
            TokenValue::Number(num) => Ok(TokenValue::Number(num * factor)),
            TokenValue::Size(size) => {
                // 解析尺寸值并应用乘法
                if let Some(captures) = regex::Regex::new(r"^([0-9.]+)(.*)$")
                    .unwrap()
                    .captures(size)
                {
                    let num: f64 = captures[1]
                        .parse()
                        .map_err(|_| TokenSystemError::ParseError("无效的尺寸数值".to_string()))?;
                    let unit = &captures[2];
                    Ok(TokenValue::Size(format!("{}{}", num * factor, unit)))
                } else {
                    Err(TokenSystemError::ParseError("无法解析尺寸值".to_string()))
                }
            }
            _ => Err(TokenSystemError::TypeMismatch(
                "乘法变换只能应用于数值或尺寸值".to_string(),
            )),
        }
    }

    /// 应用除法变换
    fn apply_divide(&self, value: &TokenValue, divisor: f64) -> TokenSystemResult<TokenValue> {
        if divisor == 0.0 {
            return Err(TokenSystemError::ParseError("除数不能为零".to_string()));
        }
        self.apply_multiply(value, 1.0 / divisor)
    }

    /// 应用加法变换
    fn apply_add(&self, value: &TokenValue, addend: f64) -> TokenSystemResult<TokenValue> {
        match value {
            TokenValue::Number(num) => Ok(TokenValue::Number(num + addend)),
            _ => Err(TokenSystemError::TypeMismatch(
                "加法变换只能应用于数值".to_string(),
            )),
        }
    }

    /// 应用减法变换
    fn apply_subtract(&self, value: &TokenValue, subtrahend: f64) -> TokenSystemResult<TokenValue> {
        self.apply_add(value, -subtrahend)
    }
}

/// 设计令牌解析器
#[derive(Debug)]
pub struct TokenResolver {
    /// 令牌存储
    tokens: HashMap<TokenPath, TokenValue>,
    /// 解析缓存
    cache: HashMap<TokenPath, TokenValue>,
    /// 解析栈（用于检测循环引用）
    resolution_stack: Vec<TokenPath>,
}

impl TokenResolver {
    /// 创建新的令牌解析器
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            cache: HashMap::new(),
            resolution_stack: Vec::new(),
        }
    }

    /// 从设计令牌创建解析器
    pub fn from_design_tokens(design_tokens: &DesignTokens) -> Self {
        let mut resolver = Self::new();
        resolver.load_design_tokens(design_tokens);
        resolver
    }

    /// 加载设计令牌
    pub fn load_design_tokens(&mut self, design_tokens: &DesignTokens) {
        // 加载颜色令牌
        self.load_color_tokens(&design_tokens.colors);

        // 加载其他令牌类型...
        // 这里可以扩展加载其他类型的令牌
    }

    /// 加载颜色令牌
    fn load_color_tokens(&mut self, colors: &super::design_tokens::ColorTokens) {
        // 加载主色调
        self.load_color_scale("colors.primary", &colors.primary);
        self.load_color_scale("colors.success", &colors.success);
        self.load_color_scale("colors.warning", &colors.warning);
        self.load_color_scale("colors.error", &colors.error);
        self.load_color_scale("colors.info", &colors.info);
        self.load_color_scale("colors.neutral", &colors.neutral);

        // 加载文本颜色
        self.set_token("colors.text.primary", colors.text.primary.clone());
        self.set_token("colors.text.secondary", colors.text.secondary.clone());
        self.set_token("colors.text.disabled", colors.text.disabled.clone());
        self.set_token("colors.text.inverse", colors.text.inverse.clone());

        // 加载背景颜色
        self.set_token(
            "colors.background.primary",
            colors.background.primary.clone(),
        );
        self.set_token(
            "colors.background.secondary",
            colors.background.secondary.clone(),
        );
        self.set_token(
            "colors.background.container",
            colors.background.container.clone(),
        );
        self.set_token(
            "colors.background.elevated",
            colors.background.elevated.clone(),
        );
        self.set_token(
            "colors.background.overlay",
            colors.background.overlay.clone(),
        );

        // 加载边框颜色
        self.set_token("colors.border.default", colors.border.default.clone());
        self.set_token("colors.border.divider", colors.border.divider.clone());
        self.set_token("colors.border.focus", colors.border.focus.clone());
        self.set_token("colors.border.error", colors.border.error.clone());
    }

    /// 加载颜色比例
    fn load_color_scale(&mut self, prefix: &str, scale: &super::design_tokens::ColorScale) {
        self.set_token(&format!("{}.50", prefix), scale._50.clone());
        self.set_token(&format!("{}.100", prefix), scale._100.clone());
        self.set_token(&format!("{}.200", prefix), scale._200.clone());
        self.set_token(&format!("{}.300", prefix), scale._300.clone());
        self.set_token(&format!("{}.400", prefix), scale._400.clone());
        self.set_token(&format!("{}.500", prefix), scale._500.clone());
        self.set_token(&format!("{}.600", prefix), scale._600.clone());
        self.set_token(&format!("{}.700", prefix), scale._700.clone());
        self.set_token(&format!("{}.800", prefix), scale._800.clone());
        self.set_token(&format!("{}.900", prefix), scale._900.clone());
        self.set_token(&format!("{}.950", prefix), scale._950.clone());
    }

    /// 设置令牌
    pub fn set_token(&mut self, path: &str, value: TokenValue) {
        if let Ok(token_path) = TokenPath::from_string(path) {
            self.tokens.insert(token_path, value);
            self.clear_cache();
        }
    }

    /// 获取令牌
    pub fn get_token(&self, path: &str) -> TokenSystemResult<&TokenValue> {
        let token_path = TokenPath::from_string(path)?;
        self.tokens
            .get(&token_path)
            .ok_or_else(|| TokenSystemError::TokenNotFound(path.to_string()))
    }

    /// 解析令牌引用
    pub fn resolve_reference(
        &mut self,
        reference: &TokenReference,
    ) -> TokenSystemResult<TokenValue> {
        // 检查缓存
        if let Some(cached_value) = self.cache.get(&reference.path) {
            return Ok(cached_value.clone());
        }

        // 检查循环引用
        if self.resolution_stack.contains(&reference.path) {
            return Err(TokenSystemError::CircularReference(
                self.resolution_stack
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(" -> "),
            ));
        }

        // 添加到解析栈
        self.resolution_stack.push(reference.path.clone());

        // 解析令牌值
        let result = self.resolve_token_value(&reference.path);

        // 从解析栈中移除
        self.resolution_stack.pop();

        let mut resolved_value = result?;

        // 应用变换
        if let Some(transform) = &reference.transform {
            resolved_value = transform.apply(&resolved_value)?;
        }

        // 缓存结果
        self.cache
            .insert(reference.path.clone(), resolved_value.clone());

        Ok(resolved_value)
    }

    /// 解析令牌值
    fn resolve_token_value(&mut self, path: &TokenPath) -> TokenSystemResult<TokenValue> {
        // 首先尝试直接获取令牌
        if let Some(value) = self.tokens.get(path) {
            match value {
                TokenValue::Reference(ref_str) => {
                    // 递归解析引用
                    let reference = TokenReference::from_string(ref_str)?;
                    self.resolve_reference(&reference)
                }
                _ => Ok(value.clone()),
            }
        } else {
            Err(TokenSystemError::TokenNotFound(path.to_string()))
        }
    }

    /// 解析字符串中的所有引用
    pub fn resolve_string(&mut self, input: &str) -> TokenSystemResult<String> {
        let mut result = input.to_string();

        // 查找所有引用模式 {path.to.token}
        let reference_regex = regex::Regex::new(r"\{([^}]+)\}").unwrap();

        for captures in reference_regex.captures_iter(input) {
            let full_match = &captures[0];
            let reference_str = &captures[1];

            let reference = TokenReference::from_string(&format!("{{{}}}", reference_str))?;
            let resolved_value = self.resolve_reference(&reference)?;

            result = result.replace(full_match, &resolved_value.to_css_value());
        }

        Ok(result)
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 验证所有令牌引用
    pub fn validate_all_references(&mut self) -> TokenSystemResult<()> {
        let token_paths: Vec<TokenPath> = self.tokens.keys().cloned().collect();

        for path in token_paths {
            if let Some(value) = self.tokens.get(&path).cloned() {
                if let TokenValue::Reference(ref_str) = value {
                    let reference = TokenReference::from_string(&ref_str)?;
                    self.resolve_reference(&reference)?;
                }
            }
        }

        Ok(())
    }

    /// 获取所有令牌路径
    pub fn list_tokens(&self) -> Vec<String> {
        self.tokens.keys().map(|path| path.to_string()).collect()
    }

    /// 导出令牌为 JSON
    pub fn export_tokens(&self) -> TokenSystemResult<String> {
        let mut export_map = HashMap::new();

        for (path, value) in &self.tokens {
            export_map.insert(path.to_string(), value);
        }

        serde_json::to_string_pretty(&export_map)
            .map_err(|e| TokenSystemError::ParseError(e.to_string()))
    }

    /// 从 JSON 导入令牌
    pub fn import_tokens(&mut self, json: &str) -> TokenSystemResult<()> {
        let import_map: HashMap<String, TokenValue> =
            serde_json::from_str(json).map_err(|e| TokenSystemError::ParseError(e.to_string()))?;

        for (path_str, value) in import_map {
            let path = TokenPath::from_string(&path_str)?;
            self.tokens.insert(path, value);
        }

        self.clear_cache();
        Ok(())
    }
}

impl Default for TokenResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_path_creation() {
        let path = TokenPath::from_string("colors.primary.500").unwrap();
        assert_eq!(path.segments, vec!["colors", "primary", "500"]);
        assert_eq!(path.to_string(), "colors.primary.500");
    }

    #[test]
    fn test_token_reference_parsing() {
        let reference = TokenReference::from_string("{colors.primary.500}").unwrap();
        assert_eq!(reference.path.to_string(), "colors.primary.500");
        assert!(reference.transform.is_none());

        let reference_with_transform =
            TokenReference::from_string("{colors.primary.500|alpha(0.5)}").unwrap();
        assert_eq!(
            reference_with_transform.path.to_string(),
            "colors.primary.500"
        );
        assert!(reference_with_transform.transform.is_some());
    }

    #[test]
    fn test_token_transform_parsing() {
        let transform = TokenTransform::from_string("alpha(0.5)").unwrap();
        assert_eq!(transform, TokenTransform::Alpha(0.5));

        let transform = TokenTransform::from_string("multiply(2.0)").unwrap();
        assert_eq!(transform, TokenTransform::Multiply(2.0));
    }

    #[test]
    fn test_token_resolver() {
        let mut resolver = TokenResolver::new();
        resolver.set_token("colors.primary", TokenValue::color("#1677ff"));

        let value = resolver.get_token("colors.primary").unwrap();
        assert_eq!(value, &TokenValue::color("#1677ff"));
    }

    #[test]
    fn test_circular_reference_detection() {
        let mut resolver = TokenResolver::new();
        resolver.set_token("a", TokenValue::Reference("{b}".to_string()));
        resolver.set_token("b", TokenValue::Reference("{a}".to_string()));

        let reference = TokenReference::from_string("{a}").unwrap();
        let result = resolver.resolve_reference(&reference);

        assert!(matches!(
            result,
            Err(TokenSystemError::CircularReference(_))
        ));
    }
}
