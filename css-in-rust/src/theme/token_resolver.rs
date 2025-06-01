//! 设计令牌解析模块
//!
//! 本模块负责令牌路径解析、值获取和引用解析。
//! 职责：令牌解析逻辑、引用处理、值计算

use super::token_definitions::{
    ThemeVariant, TokenDefinitions, TokenPath, TokenValidationError, TokenValue,
};
use super::token_values::TokenValueStore;
use std::collections::{HashMap, HashSet};

/// 令牌解析器
#[derive(Debug)]
pub struct TokenResolver {
    /// 令牌值存储
    store: TokenValueStore,
    /// 解析缓存
    cache: HashMap<(TokenPath, ThemeVariant), TokenValue>,
    /// 当前解析栈（用于检测循环引用）
    resolution_stack: HashSet<TokenPath>,
}

impl TokenResolver {
    /// 创建新的令牌解析器
    pub fn new(store: TokenValueStore) -> Self {
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
            .get_value(path, theme)
            .ok_or_else(|| TokenValidationError::MissingReference(path.to_string()))?
            .clone();

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
    pub fn get_store(&self) -> &TokenValueStore {
        &self.store
    }

    /// 获取存储可变引用
    pub fn get_store_mut(&mut self) -> &mut TokenValueStore {
        &mut self.store
    }

    /// 验证令牌引用的完整性
    pub fn validate_references(&mut self, theme: ThemeVariant) -> Vec<TokenValidationError> {
        let mut errors = Vec::new();
        let paths = self.store.list_paths(theme);

        for path in paths {
            if let Err(error) = self.resolve_token(&path, theme) {
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
        let paths = self.store.list_paths(theme);

        for path in paths {
            if let Some(value) = self.store.get_value(&path, theme) {
                if self.value_references_path(value, target_path) {
                    references.push(path);
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
        self.store.get_value(path, theme).cloned()
    }

    fn set_token_value(
        &mut self,
        path: &TokenPath,
        value: TokenValue,
        theme: ThemeVariant,
    ) -> Result<(), String> {
        self.store.set_value(path.clone(), value, theme);
        // 清空相关缓存
        self.cache
            .retain(|(cached_path, cached_theme), _| cached_path != path || *cached_theme != theme);
        Ok(())
    }

    fn get_token_metadata(
        &self,
        path: &TokenPath,
    ) -> Option<super::token_definitions::TokenMetadata> {
        self.store.get_metadata(path).cloned()
    }

    fn list_token_paths(&self, theme: ThemeVariant) -> Vec<TokenPath> {
        self.store.list_paths(theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::token_values::AntDesignTokenValues;

    #[test]
    fn test_token_resolver_basic() {
        let store = AntDesignTokenValues::create_default_store();
        let mut resolver = TokenResolver::new(store);

        let path = TokenPath::from_str("color.primary.500");
        let result = resolver.resolve_token(&path, ThemeVariant::Light);

        assert!(result.is_ok());
        if let Ok(TokenValue::String(color)) = result {
            assert_eq!(color, "#1677ff");
        }
    }

    #[test]
    fn test_token_reference_resolution() {
        let mut store = TokenValueStore::new();

        // 设置基础值
        store.set_value(
            TokenPath::from_str("base.color"),
            TokenValue::String("#1677ff".to_string()),
            ThemeVariant::Light,
        );

        // 设置引用值
        store.set_value(
            TokenPath::from_str("primary.color"),
            TokenValue::Reference("base.color".to_string()),
            ThemeVariant::Light,
        );

        let mut resolver = TokenResolver::new(store);
        let result =
            resolver.resolve_token(&TokenPath::from_str("primary.color"), ThemeVariant::Light);

        assert!(result.is_ok());
        if let Ok(TokenValue::String(color)) = result {
            assert_eq!(color, "#1677ff");
        }
    }

    #[test]
    fn test_circular_reference_detection() {
        let mut store = TokenValueStore::new();

        // 创建循环引用
        store.set_value(
            TokenPath::from_str("a"),
            TokenValue::Reference("b".to_string()),
            ThemeVariant::Light,
        );
        store.set_value(
            TokenPath::from_str("b"),
            TokenValue::Reference("a".to_string()),
            ThemeVariant::Light,
        );

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
        let mut store = TokenValueStore::new();

        store.set_value(
            TokenPath::from_str("base.size"),
            TokenValue::Number(16.0),
            ThemeVariant::Light,
        );

        let mut resolver = TokenResolver::new(store);

        // 测试加法
        let result = resolver.compute_value("base.size + 8", ThemeVariant::Light);
        assert!(result.is_ok());
        if let Ok(TokenValue::Number(value)) = result {
            assert_eq!(value, 24.0);
        }
    }
}
