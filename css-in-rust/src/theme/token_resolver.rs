//! 设计令牌解析模块
//!
//! 本模块负责令牌路径解析、值获取和引用解析。
//! 职责：令牌解析逻辑、引用处理、值计算

use super::token_definitions::{
    MathOperation, ThemeVariant, TokenDefinitions, TokenPath, TokenReference, TokenTransform,
    TokenValidationError, TokenValue,
};
use super::token_values::DesignTokens;
use std::collections::{HashMap, HashSet};

/// 令牌解析器
#[derive(Debug)]
pub struct TokenResolver {
    /// 令牌值存储
    store: DesignTokens,
    /// 解析缓存
    cache: HashMap<(TokenPath, ThemeVariant), TokenValue>,
    /// 当前解析栈（用于检测循环引用）
    resolution_stack: HashSet<TokenPath>,
}

impl TokenResolver {
    /// 创建新的令牌解析器
    pub fn new(store: DesignTokens) -> Self {
        Self {
            store,
            cache: HashMap::new(),
            resolution_stack: HashSet::new(),
        }
    }

    /// 解析令牌值
    pub fn resolve_token(
        &mut self,
        path: &TokenPath,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 检查缓存
        let cache_key = (path.clone(), theme);
        if let Some(cached_value) = self.cache.get(&cache_key) {
            return Ok(cached_value.clone());
        }

        // 检查循环引用
        if self.resolution_stack.contains(path) {
            return Err(TokenValidationError::CircularReference(path.to_string()));
        }

        // 添加到解析栈
        self.resolution_stack.insert(path.clone());

        // 获取原始值
        let raw_value = self
            .store
            .get_value(&path.to_string())
            .ok_or_else(|| TokenValidationError::MissingReference(path.to_string()))
            .map(|s| TokenValue::String(s))?;

        // 解析值（处理引用）
        let resolved_value = self.resolve_value(&raw_value, theme)?;

        // 从解析栈中移除
        self.resolution_stack.remove(path);

        // 缓存结果
        self.cache.insert(cache_key, resolved_value.clone());

        Ok(resolved_value)
    }

    /// 解析令牌值（处理引用和计算）
    fn resolve_value(
        &mut self,
        value: &TokenValue,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        match value {
            TokenValue::Reference(reference) => {
                let ref_path = TokenPath::from_str(reference);
                self.resolve_token(&ref_path, theme)
            }
            TokenValue::Array(arr) => {
                let mut resolved_arr = Vec::new();
                for item in arr {
                    resolved_arr.push(self.resolve_value(item, theme)?);
                }
                Ok(TokenValue::Array(resolved_arr))
            }
            TokenValue::Object(obj) => {
                let mut resolved_obj = HashMap::new();
                for (key, val) in obj {
                    resolved_obj.insert(key.clone(), self.resolve_value(val, theme)?);
                }
                Ok(TokenValue::Object(resolved_obj))
            }
            _ => Ok(value.clone()),
        }
    }

    /// 批量解析令牌
    pub fn resolve_tokens_batch(
        &mut self,
        paths: &[TokenPath],
        theme: ThemeVariant,
    ) -> HashMap<TokenPath, Result<TokenValue, TokenValidationError>> {
        let mut results = HashMap::new();
        for path in paths {
            let result = self.resolve_token(path, theme);
            results.insert(path.clone(), result);
        }
        results
    }

    /// 清空缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 获取存储引用
    pub fn get_store(&self) -> &DesignTokens {
        &self.store
    }

    /// 获取存储可变引用
    pub fn get_store_mut(&mut self) -> &mut DesignTokens {
        &mut self.store
    }

    /// 验证令牌引用的完整性
    pub fn validate_references(&mut self, theme: ThemeVariant) -> Vec<TokenValidationError> {
        let mut errors = Vec::new();
        let theme_str = match theme {
            ThemeVariant::Light => "light",
            ThemeVariant::Dark => "dark",
            ThemeVariant::Auto => "light", // 默认使用light主题
        };
        let paths = self.store.list_paths(theme_str);

        for path in paths {
            let token_path = TokenPath::from_str(&path);
            if let Err(error) = self.resolve_token(&token_path, theme) {
                errors.push(error);
            }
        }

        errors
    }

    /// 查找令牌的所有引用者
    pub fn find_references_to(
        &self,
        target_path: &TokenPath,
        theme: ThemeVariant,
    ) -> Vec<TokenPath> {
        let mut references = Vec::new();
        let theme_str = match theme {
            ThemeVariant::Light => "light",
            ThemeVariant::Dark => "dark",
            ThemeVariant::Auto => "light", // 默认使用light主题
        };
        let paths = self.store.list_paths(theme_str);

        for path in paths {
            let token_path = TokenPath::from_str(&path);
            if let Some(value_str) = self.store.get_value(&path) {
                let value = TokenValue::String(value_str);
                if self.value_references_path(&value, target_path) {
                    references.push(token_path);
                }
            }
        }

        references
    }

    /// 检查值是否引用了指定路径
    fn value_references_path(&self, value: &TokenValue, target_path: &TokenPath) -> bool {
        match value {
            TokenValue::Reference(reference) => {
                let ref_path = TokenPath::from_str(reference);
                ref_path == *target_path
            }
            TokenValue::Array(arr) => arr
                .iter()
                .any(|item| self.value_references_path(item, target_path)),
            TokenValue::Object(obj) => obj
                .values()
                .any(|val| self.value_references_path(val, target_path)),
            _ => false,
        }
    }

    /// 计算令牌值（支持数学运算）
    pub fn compute_value(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 简单的表达式解析器，支持基本的数学运算
        if expression.contains('+') {
            self.compute_addition(expression, theme)
        } else if expression.contains('-') {
            self.compute_subtraction(expression, theme)
        } else if expression.contains('*') {
            self.compute_multiplication(expression, theme)
        } else if expression.contains('/') {
            self.compute_division(expression, theme)
        } else {
            // 尝试解析为令牌引用
            let path = TokenPath::from_str(expression);
            self.resolve_token(&path, theme)
        }
    }

    /// 应用令牌变换
    pub fn apply_transform(
        &mut self,
        value: &TokenValue,
        transform: &TokenTransform,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        match transform {
            TokenTransform::Alpha(alpha) => match value {
                TokenValue::Color(color) => {
                    let mut new_color = color.clone();
                    new_color.alpha = Some(*alpha);
                    Ok(TokenValue::Color(new_color))
                }
                _ => Err(TokenValidationError::TypeMismatch {
                    expected: "Color".to_string(),
                    actual: format!("{:?}", value),
                }),
            },
            TokenTransform::Lighten(amount) => match value {
                TokenValue::Color(color) => Ok(TokenValue::Color(color.lighten(*amount))),
                _ => Err(TokenValidationError::TypeMismatch {
                    expected: "Color".to_string(),
                    actual: format!("{:?}", value),
                }),
            },
            TokenTransform::Darken(amount) => match value {
                TokenValue::Color(color) => Ok(TokenValue::Color(color.darken(*amount))),
                _ => Err(TokenValidationError::TypeMismatch {
                    expected: "Color".to_string(),
                    actual: format!("{:?}", value),
                }),
            },
            TokenTransform::Saturate(amount) => match value {
                TokenValue::Color(color) => Ok(TokenValue::Color(color.saturate(*amount))),
                _ => Err(TokenValidationError::TypeMismatch {
                    expected: "Color".to_string(),
                    actual: format!("{:?}", value),
                }),
            },
            TokenTransform::Desaturate(amount) => match value {
                TokenValue::Color(color) => Ok(TokenValue::Color(color.desaturate(*amount))),
                _ => Err(TokenValidationError::TypeMismatch {
                    expected: "Color".to_string(),
                    actual: format!("{:?}", value),
                }),
            },
            TokenTransform::HueRotate(degrees) => {
                match value {
                    TokenValue::Color(color) => {
                        // 简单实现，实际应该进行色相旋转
                        let mut new_color = color.clone();
                        new_color.hex =
                            format!("{}/* hue rotated by {} degrees */", color.hex, degrees);
                        Ok(TokenValue::Color(new_color))
                    }
                    _ => Err(TokenValidationError::TypeMismatch {
                        expected: "Color".to_string(),
                        actual: format!("{:?}", value),
                    }),
                }
            }
            TokenTransform::Contrast(amount) => {
                match value {
                    TokenValue::Color(color) => {
                        // 简单实现，实际应该调整对比度
                        let mut new_color = color.clone();
                        new_color.hex =
                            format!("{}/* contrast adjusted by {} */", color.hex, amount);
                        Ok(TokenValue::Color(new_color))
                    }
                    _ => Err(TokenValidationError::TypeMismatch {
                        expected: "Color".to_string(),
                        actual: format!("{:?}", value),
                    }),
                }
            }
            TokenTransform::Math(operation) => self.apply_math_operation(value, operation, theme),
            TokenTransform::ColorModify { operation, amount } => {
                self.apply_color_modification(value, operation, *amount)
            }
            TokenTransform::Scale(factor) => self.apply_scale_transform(value, *factor),
            TokenTransform::Conditional {
                condition,
                if_true,
                if_false,
            } => {
                // 递归应用条件变换
                let condition_result = self.evaluate_condition(condition, theme)?;
                let selected_transform = if condition_result { if_true } else { if_false };
                self.apply_transform(value, selected_transform, theme)
            }
        }
    }

    /// 应用数学运算变换
    fn apply_math_operation(
        &mut self,
        value: &TokenValue,
        operation: &MathOperation,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        match operation {
            MathOperation::Add(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                self.add_values(value, &operand_value)
            }
            MathOperation::Subtract(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                self.subtract_values(value, &operand_value)
            }
            MathOperation::Multiply(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                self.multiply_values(value, &operand_value)
            }
            MathOperation::Divide(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                self.divide_values(value, &operand_value)
            }
            MathOperation::Min(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                // 实现min逻辑
                match (value, &operand_value) {
                    (TokenValue::Number(a), TokenValue::Number(b)) => {
                        Ok(TokenValue::Number(a.min(*b)))
                    }
                    _ => Err(TokenValidationError::TypeMismatch {
                        expected: "Number".to_string(),
                        actual: format!("{:?}", value),
                    }),
                }
            }
            MathOperation::Max(operand) => {
                let operand_value = TokenValue::Number(*operand as f64);
                // 实现max逻辑
                match (value, &operand_value) {
                    (TokenValue::Number(a), TokenValue::Number(b)) => {
                        Ok(TokenValue::Number(a.max(*b)))
                    }
                    _ => Err(TokenValidationError::TypeMismatch {
                        expected: "Number".to_string(),
                        actual: format!("{:?}", value),
                    }),
                }
            }
            MathOperation::Clamp(min_val, max_val, _) => {
                // 实现clamp逻辑
                match value {
                    TokenValue::Number(val) => {
                        let clamped = val.max(*min_val as f64).min(*max_val as f64);
                        Ok(TokenValue::Number(clamped))
                    }
                    _ => Err(TokenValidationError::TypeMismatch {
                        expected: "Number".to_string(),
                        actual: format!("{:?}", value),
                    }),
                }
            }
        }
    }

    /// 应用颜色修改变换
    fn apply_color_modification(
        &self,
        value: &TokenValue,
        operation: &str,
        amount: f32,
    ) -> Result<TokenValue, TokenValidationError> {
        match value {
            TokenValue::Color(color_value) => {
                let modified_color = match operation {
                    "lighten" => color_value.lighten(amount),
                    "darken" => color_value.darken(amount),
                    "saturate" => color_value.saturate(amount),
                    "desaturate" => color_value.desaturate(amount),
                    "fade" => color_value.fade(amount),
                    _ => {
                        return Err(TokenValidationError::InvalidValue(format!(
                            "Unknown color operation: {}",
                            operation
                        )))
                    }
                };
                Ok(TokenValue::Color(modified_color))
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "color".to_string(),
                actual: value.token_type().to_string(),
            }),
        }
    }

    /// 应用缩放变换
    fn apply_scale_transform(
        &self,
        value: &TokenValue,
        factor: f32,
    ) -> Result<TokenValue, TokenValidationError> {
        match value {
            TokenValue::Number(n) => Ok(TokenValue::Number(n * factor as f64)),
            TokenValue::Dimension(dim) => {
                let scaled_dim = dim.scale(factor);
                Ok(TokenValue::Dimension(scaled_dim))
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number or dimension".to_string(),
                actual: value.token_type().to_string(),
            }),
        }
    }

    /// 评估条件表达式
    fn evaluate_condition(
        &mut self,
        condition: &str,
        theme: ThemeVariant,
    ) -> Result<bool, TokenValidationError> {
        // 简单的条件评估，支持主题检查
        if condition == "theme.light" {
            Ok(theme == ThemeVariant::Light)
        } else if condition == "theme.dark" {
            Ok(theme == ThemeVariant::Dark)
        } else {
            // 可以扩展支持更复杂的条件表达式
            Err(TokenValidationError::InvalidValue(format!(
                "Unknown condition: {}",
                condition
            )))
        }
    }

    /// 值加法运算
    fn add_values(
        &self,
        left: &TokenValue,
        right: &TokenValue,
    ) -> Result<TokenValue, TokenValidationError> {
        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a + b)),
            (TokenValue::Dimension(a), TokenValue::Dimension(b)) => {
                Ok(TokenValue::Dimension(a.add(b)?))
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "compatible numeric types".to_string(),
                actual: format!("{} and {}", left.token_type(), right.token_type()),
            }),
        }
    }

    /// 值减法运算
    fn subtract_values(
        &self,
        left: &TokenValue,
        right: &TokenValue,
    ) -> Result<TokenValue, TokenValidationError> {
        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a - b)),
            (TokenValue::Dimension(a), TokenValue::Dimension(b)) => {
                Ok(TokenValue::Dimension(a.subtract(b)?))
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "compatible numeric types".to_string(),
                actual: format!("{} and {}", left.token_type(), right.token_type()),
            }),
        }
    }

    /// 值乘法运算
    fn multiply_values(
        &self,
        left: &TokenValue,
        right: &TokenValue,
    ) -> Result<TokenValue, TokenValidationError> {
        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a * b)),
            (TokenValue::Dimension(a), TokenValue::Number(b)) => {
                Ok(TokenValue::Dimension(a.multiply(*b)))
            }
            (TokenValue::Number(a), TokenValue::Dimension(b)) => {
                Ok(TokenValue::Dimension(b.multiply(*a)))
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", left.token_type(), right.token_type()),
            }),
        }
    }

    /// 值除法运算
    fn divide_values(
        &self,
        left: &TokenValue,
        right: &TokenValue,
    ) -> Result<TokenValue, TokenValidationError> {
        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => {
                if *b == 0.0 {
                    Err(TokenValidationError::InvalidValue(
                        "Division by zero".to_string(),
                    ))
                } else {
                    Ok(TokenValue::Number(a / b))
                }
            }
            (TokenValue::Dimension(a), TokenValue::Number(b)) => {
                if *b == 0.0 {
                    Err(TokenValidationError::InvalidValue(
                        "Division by zero".to_string(),
                    ))
                } else {
                    Ok(TokenValue::Dimension(
                        a.divide(*b)
                            .map_err(|e| TokenValidationError::InvalidValue(e))?,
                    ))
                }
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number".to_string(),
                actual: "mixed".to_string(),
            }),
        }
    }

    /// 计算加法表达式
    fn compute_addition(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        let parts: Vec<&str> = expression.split('+').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(TokenValidationError::InvalidValue(expression.to_string()));
        }

        let left = self.parse_operand(parts[0], theme)?;
        let right = self.parse_operand(parts[1], theme)?;

        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a + b)),
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number".to_string(),
                actual: "mixed".to_string(),
            }),
        }
    }

    /// 计算减法表达式
    fn compute_subtraction(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        let parts: Vec<&str> = expression.split('-').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(TokenValidationError::InvalidValue(expression.to_string()));
        }

        let left = self.parse_operand(parts[0], theme)?;
        let right = self.parse_operand(parts[1], theme)?;

        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a - b)),
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number".to_string(),
                actual: "mixed".to_string(),
            }),
        }
    }

    /// 计算乘法表达式
    fn compute_multiplication(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        let parts: Vec<&str> = expression.split('*').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(TokenValidationError::InvalidValue(expression.to_string()));
        }

        let left = self.parse_operand(parts[0], theme)?;
        let right = self.parse_operand(parts[1], theme)?;

        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => Ok(TokenValue::Number(a * b)),
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number".to_string(),
                actual: "mixed".to_string(),
            }),
        }
    }

    /// 计算除法表达式
    fn compute_division(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        let parts: Vec<&str> = expression.split('/').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(TokenValidationError::InvalidValue(expression.to_string()));
        }

        let left = self.parse_operand(parts[0], theme)?;
        let right = self.parse_operand(parts[1], theme)?;

        match (left, right) {
            (TokenValue::Number(a), TokenValue::Number(b)) => {
                if b == 0.0 {
                    Err(TokenValidationError::InvalidValue(
                        "Division by zero".to_string(),
                    ))
                } else {
                    Ok(TokenValue::Number(a / b))
                }
            }
            (TokenValue::Dimension(a), TokenValue::Number(b)) => {
                if b == 0.0 {
                    Err(TokenValidationError::InvalidValue(
                        "Division by zero".to_string(),
                    ))
                } else {
                    Ok(TokenValue::Dimension(
                        a.divide(b)
                            .map_err(|e| TokenValidationError::InvalidValue(e))?,
                    ))
                }
            }
            _ => Err(TokenValidationError::TypeMismatch {
                expected: "number".to_string(),
                actual: "mixed".to_string(),
            }),
        }
    }

    /// 解析操作数
    fn parse_operand(
        &mut self,
        operand: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 尝试解析为数字
        if let Ok(number) = operand.parse::<f64>() {
            return Ok(TokenValue::Number(number));
        }

        // 尝试解析为令牌引用
        let path = TokenPath::from_str(operand);
        self.resolve_token(&path, theme)
    }
}

impl TokenDefinitions for TokenResolver {
    fn get_token_value(&self, path: &TokenPath, theme: ThemeVariant) -> Option<TokenValue> {
        // 注意：这里返回原始值，不进行解析
        self.store
            .get_value(&path.to_string())
            .map(|s| TokenValue::String(s))
    }

    fn set_token_value(
        &mut self,
        path: &TokenPath,
        value: TokenValue,
        theme: ThemeVariant,
    ) -> Result<(), String> {
        self.store.set_value(&path.to_string(), value.to_string())?;
        // 清空相关缓存
        self.cache
            .retain(|(cached_path, cached_theme), _| cached_path != path || *cached_theme != theme);
        Ok(())
    }

    fn get_token_metadata(
        &self,
        path: &TokenPath,
    ) -> Option<super::token_definitions::TokenMetadata> {
        self.store.get_metadata(&path.to_string())
    }

    fn list_token_paths(&self, theme: ThemeVariant) -> Vec<TokenPath> {
        let theme_str = match theme {
            ThemeVariant::Light => "light",
            ThemeVariant::Dark => "dark",
            ThemeVariant::Auto => "light", // 默认使用light主题
        };
        self.store
            .list_paths(theme_str)
            .into_iter()
            .map(|path| TokenPath::from_str(&path))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::token_values::DesignTokens;

    #[test]
    fn test_token_resolver_basic() {
        let mut store = DesignTokens::default();
        // 添加测试令牌
        let light_values = DesignTokens::new().get_light_theme_values();
        for (path, value) in light_values {
            store.set_value(&path.to_string(), value.to_string());
        }
        let mut resolver = TokenResolver::new(store);

        let path = TokenPath::from_str("color.primary.500");
        let result = resolver.resolve_token(&path, ThemeVariant::Light);

        assert!(result.is_ok());
        if let Ok(TokenValue::String(color)) = result {
            assert_eq!(color, "#0066cc");
        }
    }

    #[test]
    fn test_token_reference_resolution() {
        let mut store = DesignTokens::default();

        // 设置基础值
        store.set_value("base.color", "#0066cc".to_string());

        // 设置引用值
        store.set_value("primary.color", "base.color".to_string());

        let mut resolver = TokenResolver::new(store);
        let result =
            resolver.resolve_token(&TokenPath::from_str("primary.color"), ThemeVariant::Light);

        assert!(result.is_ok());
        if let Ok(TokenValue::String(color)) = result {
            assert_eq!(color, "#0066cc");
        }
    }

    #[test]
    fn test_circular_reference_detection() {
        let mut store = DesignTokens::default();

        // 创建循环引用
        store.set_value("a", "b".to_string());
        store.set_value("b", "a".to_string());

        let mut resolver = TokenResolver::new(store);
        let result = resolver.resolve_token(&TokenPath::from_str("a"), ThemeVariant::Light);

        assert!(result.is_err());
        if let Err(TokenValidationError::CircularReference(_)) = result {
            // 预期的错误
        } else {
            panic!("Expected circular reference error");
        }
    }

    #[test]
    fn test_computation() {
        let mut store = DesignTokens::default();

        store.set_value("base.size", "16.0".to_string());

        let mut resolver = TokenResolver::new(store);

        // 测试加法
        let result = resolver.compute_value("base.size + 8", ThemeVariant::Light);
        assert!(result.is_ok());
        if let Ok(TokenValue::Number(value)) = result {
            assert_eq!(value, 24.0);
        }
    }
}
