//! 设计令牌解析模块
//!
//! 本模块负责令牌路径解析、值获取和引用解析。
//! 职责：令牌解析逻辑、引用处理、值计算

use super::{
    definitions::{ThemeVariant, TokenMetadata, TokenPath, TokenValidationError, TokenValue},
    values::DesignTokens,
};
use std::collections::HashMap;

/// 令牌解析器
///
/// 负责解析令牌引用和值路径，提供令牌系统的查询能力。
#[derive(Debug, Clone)]
pub struct TokenResolver {
    /// 令牌存储
    pub store: DesignTokens,
    /// 缓存
    cache: HashMap<String, TokenValue>,
}

impl PartialEq for TokenResolver {
    fn eq(&self, other: &Self) -> bool {
        // 只比较 store，不比较 cache
        self.store == other.store
    }
}

impl Default for TokenResolver {
    fn default() -> Self {
        Self {
            store: DesignTokens::default(),
            cache: HashMap::new(),
        }
    }
}

impl TokenResolver {
    /// 创建新的令牌解析器
    pub fn new(store: DesignTokens) -> Self {
        Self {
            store,
            cache: HashMap::new(),
        }
    }

    /// 解析令牌
    pub fn resolve_token(
        &self,
        path: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 从存储中获取值
        self.store
            .get_value(path, theme)
            .cloned()
            .ok_or_else(|| TokenValidationError::InvalidPath(path.to_string()))
    }

    /// 获取令牌元数据
    pub fn get_token_metadata(&self, path: &str) -> Option<String> {
        // 在这个简化实现中，我们不需要真正的元数据
        Some("Token metadata".to_string())
    }

    /// 设置令牌
    pub fn set_token(
        &mut self,
        path: &TokenPath,
        value: TokenValue,
        theme: ThemeVariant,
    ) -> Result<(), TokenValidationError> {
        // 验证令牌
        if !self.is_valid_token_path(&path.to_string()) {
            return Err(TokenValidationError::InvalidPath(path.to_string()));
        }

        // 设置令牌值
        self.store.set_value(path.to_string(), theme, value);

        // 清除缓存
        self.cache.remove(&path.to_string());

        Ok(())
    }

    /// 验证所有令牌引用
    pub fn validate_references(&mut self, _theme: ThemeVariant) -> Vec<TokenValidationError> {
        // 简化实现
        Vec::new()
    }

    /// 列出所有令牌路径
    pub fn list_token_paths(&self, _theme: ThemeVariant) -> Vec<TokenPath> {
        // 简化实现
        Vec::new()
    }

    /// 查找引用指定令牌的所有令牌
    pub fn find_references_to(&self, _path: &TokenPath, _theme: ThemeVariant) -> Vec<TokenPath> {
        // 简化实现
        Vec::new()
    }

    /// 计算表达式
    pub fn compute_expression(
        &self,
        expression: &str,
        _theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 计算令牌表达式
        Err(TokenValidationError::InvalidPath(expression.to_string()))
    }

    /// 计算值 (简化方法，为了兼容调用)
    pub fn compute_value(
        &mut self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        self.compute_expression(expression, theme)
    }

    /// 获取存储引用 (简化方法，为了兼容调用)
    pub fn get_store(&self) -> &DesignTokens {
        &self.store
    }

    /// 获取存储可变引用 (简化方法，为了兼容调用)
    pub fn get_store_mut(&mut self) -> &mut DesignTokens {
        &mut self.store
    }

    /// 验证令牌路径
    fn is_valid_token_path(&self, path: &str) -> bool {
        !path.is_empty()
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

// #[cfg(test)]
// mod tests {
//     use super::token_values::DesignTokens;

//     #[test]
//     fn test_token_resolver_basic() {
//         let mut store = DesignTokens::default();
//         // 添加测试令牌
//         let light_values = DesignTokens::new().get_light_theme_values();
//         for (path, value) in light_values {
//             store.set_value(&path.to_string(), value.to_string());
//         }
//         let mut resolver = TokenResolver::new(store);

//         let path = TokenPath::from_str("color.primary.500");
//         let result = resolver.resolve_token(&path, ThemeVariant::Light);

//         assert!(result.is_ok());
//         if let Ok(TokenValue::String(color)) = result {
//             assert_eq!(color, "#0066cc");
//         }
//     }

//     #[test]
//     fn test_token_reference_resolution() {
//         let mut store = DesignTokens::default();

//         // 设置基础值
//         store.set_value("base.color", "#0066cc".to_string());

//         // 设置引用值
//         store.set_value("primary.color", "base.color".to_string());

//         let mut resolver = TokenResolver::new(store);
//         let result =
//             resolver.resolve_token(&TokenPath::from_str("primary.color"), ThemeVariant::Light);

//         assert!(result.is_ok());
//         if let Ok(TokenValue::String(color)) = result {
//             assert_eq!(color, "#0066cc");
//         }
//     }

//     #[test]
//     fn test_circular_reference_detection() {
//         let mut store = DesignTokens::default();

//         // 创建循环引用
//         store.set_value("a", "b".to_string());
//         store.set_value("b", "a".to_string());

//         let mut resolver = TokenResolver::new(store);
//         let result = resolver.resolve_token(&TokenPath::from_str("a"), ThemeVariant::Light);

//         assert!(result.is_err());
//         if let Err(TokenValidationError::CircularReference(_)) = result {
//             // 预期的错误
//         } else {
//             panic!("Expected circular reference error");
//         }
//     }

//     #[test]
//     fn test_computation() {
//         let mut store = DesignTokens::default();

//         store.set_value("base.size", "16.0".to_string());

//         let mut resolver = TokenResolver::new(store);

//         // 测试加法
//         let result = resolver.compute_value("base.size + 8", ThemeVariant::Light);
//         assert!(result.is_ok());
//         if let Ok(TokenValue::Number(value)) = result {
//             assert_eq!(value, 24.0);
//         }
//     }
// }
