//! 设计令牌解析模块
//!
//! 本模块负责令牌路径解析、值获取和引用解析。
//! 职责：令牌解析逻辑、引用处理、值计算

use super::{
    definitions::{ThemeVariant, TokenPath, TokenValidationError, TokenValue},
    values::DesignTokens,
};
use std::collections::{HashMap, HashSet};

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
        let token_value = self
            .store
            .get_value(path, theme.clone())
            .cloned()
            .ok_or_else(|| TokenValidationError::InvalidPath(path.to_string()))?;

        // 处理引用类型
        match token_value {
            TokenValue::Reference(ref_path) => {
                // 防止循环引用
                if ref_path == path {
                    return Err(TokenValidationError::CircularReference(format!(
                        "Circular reference detected: {} references itself",
                        path
                    )));
                }

                // 递归解析引用
                self.resolve_token(&ref_path, theme)
            }
            TokenValue::TokenReference(token_ref) => {
                // 获取引用路径
                let ref_path = token_ref.get_reference();

                // 防止循环引用
                if ref_path == path {
                    return Err(TokenValidationError::CircularReference(format!(
                        "Circular reference detected: {} references itself",
                        path
                    )));
                }

                // 递归解析引用
                let resolved = self.resolve_token(ref_path, theme)?;

                // 应用变换（如果有）
                if let Some(_transform) = token_ref.get_transform() {
                    // TODO: 实现变换
                    Ok(resolved)
                } else {
                    Ok(resolved)
                }
            }
            // 非引用类型直接返回
            _ => Ok(token_value),
        }
    }

    /// 获取令牌元数据
    pub fn get_token_metadata(&self, path: &str) -> Option<String> {
        // 检查路径是否有效
        if !self.is_valid_token_path(path) {
            return None;
        }

        // 创建基本元数据描述
        let token_type = if path.starts_with("color") {
            "color"
        } else if path.starts_with("spacing") {
            "spacing"
        } else if path.starts_with("typography") {
            "typography"
        } else if path.starts_with("component") {
            "component"
        } else {
            "other"
        };

        Some(format!("Token: {} (type: {})", path, token_type))
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
    pub fn validate_references(&mut self, theme: ThemeVariant) -> Vec<TokenValidationError> {
        let mut errors = Vec::new();
        let mut visited = HashSet::new();

        // 获取所有令牌路径
        let all_paths = self.list_token_paths(theme.clone());

        // 检查每个令牌的引用
        for token_path in all_paths {
            let path_str = token_path.to_string();

            // 验证单个令牌的引用
            if let Err(err) = self.validate_token_references(&path_str, theme.clone(), &mut visited)
            {
                errors.push(err);
            }
        }

        errors
    }

    /// 验证单个令牌的引用
    fn validate_token_references(
        &self,
        path: &str,
        theme: ThemeVariant,
        visited: &mut HashSet<String>,
    ) -> Result<(), TokenValidationError> {
        // 检查循环引用
        if !visited.insert(path.to_string()) {
            return Err(TokenValidationError::CircularReference(format!(
                "Circular reference detected in token: {}",
                path
            )));
        }

        // 获取令牌值
        let token_value = match self.store.get_value(path, theme.clone()) {
            Some(value) => value.clone(),
            None => {
                visited.remove(path);
                return Err(TokenValidationError::InvalidPath(path.to_string()));
            }
        };

        // 检查引用类型
        match token_value {
            TokenValue::Reference(ref_path) => {
                // 验证引用的令牌
                let result = self.validate_token_references(&ref_path, theme, visited);
                visited.remove(path);
                result
            }
            TokenValue::TokenReference(token_ref) => {
                // 验证引用的令牌
                let ref_path = token_ref.get_reference();
                let result = self.validate_token_references(ref_path, theme, visited);
                visited.remove(path);
                result
            }
            _ => {
                // 非引用类型，无需验证
                visited.remove(path);
                Ok(())
            }
        }
    }

    /// 列出所有令牌路径
    pub fn list_token_paths(&self, _theme: ThemeVariant) -> Vec<TokenPath> {
        // 获取存储中的所有路径
        let paths = self.store.get_all_paths();

        // 将字符串路径转换为 TokenPath 对象
        paths.iter().map(|path| TokenPath::from_str(path)).collect()
    }

    /// 查找引用指定令牌的所有令牌
    pub fn find_references_to(&self, path: &TokenPath, theme: ThemeVariant) -> Vec<TokenPath> {
        let mut references = Vec::new();
        let target_path = path.to_string();

        // 获取所有令牌路径
        let all_paths = self.list_token_paths(theme.clone());

        // 检查每个令牌是否引用了目标令牌
        for token_path in all_paths {
            let path_str = token_path.to_string();

            // 获取原始令牌值（不解析引用）
            if let Some(value) = self.store.get_value(&path_str, theme.clone()) {
                match value {
                    TokenValue::Reference(ref_path) => {
                        if ref_path == &target_path {
                            references.push(token_path);
                        }
                    }
                    TokenValue::TokenReference(token_ref) => {
                        if token_ref.get_reference() == target_path {
                            references.push(token_path);
                        }
                    }
                    _ => {}
                }
            }
        }

        references
    }

    /// 计算表达式
    pub fn compute_expression(
        &self,
        expression: &str,
        theme: ThemeVariant,
    ) -> Result<TokenValue, TokenValidationError> {
        // 检查表达式是否为空
        if expression.trim().is_empty() {
            return Err(TokenValidationError::InvalidExpression(
                "Empty expression".to_string(),
            ));
        }

        // 检查是否是简单的令牌引用
        if !expression.contains('+')
            && !expression.contains('-')
            && !expression.contains('*')
            && !expression.contains('/')
        {
            // 尝试解析为令牌路径
            return self.resolve_token(expression, theme);
        }

        // 解析表达式
        let mut result = 0.0;
        let mut current_num = String::new();
        let mut current_op = '+';
        let mut chars = expression.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' | '.' => {
                    current_num.push(c);
                }
                '+' | '-' | '*' | '/' => {
                    // 处理前一个操作数
                    if !current_num.is_empty() {
                        let num: f64 = current_num.parse().map_err(|_| {
                            TokenValidationError::InvalidExpression(format!(
                                "Invalid number: {}",
                                current_num
                            ))
                        })?;
                        result = self.apply_operation(result, num, current_op)?;
                        current_num.clear();
                    } else if c == '-'
                        && (chars.peek().unwrap_or(&' ').is_digit(10) || chars.peek() == Some(&'.'))
                    {
                        // 处理负数
                        current_num.push(c);
                        continue;
                    } else {
                        // 处理令牌引用
                        let token_path = expression.trim();
                        let token_value = self.resolve_token(token_path, theme)?;

                        match token_value {
                            TokenValue::Number(n) => {
                                result = self.apply_operation(result, n, current_op)?;
                            }
                            _ => {
                                return Err(TokenValidationError::InvalidExpression(format!(
                                    "Token {} is not a number",
                                    token_path
                                )));
                            }
                        }
                    }
                    current_op = c;
                }
                ' ' | '\t' | '\n' | '\r' => {
                    // 忽略空白字符
                    continue;
                }
                _ => {
                    // 可能是令牌引用的一部分
                    if current_num.is_empty() {
                        // 尝试解析为令牌路径
                        let parts: Vec<&str> = expression
                            .split(|c| c == '+' || c == '-' || c == '*' || c == '/')
                            .collect();
                        if parts.len() >= 2 {
                            let left_part = parts[0].trim();
                            let right_part = parts[1].trim();

                            // 解析左侧令牌
                            let left_value = self.resolve_token(left_part, theme)?;
                            let left_num = match left_value {
                                TokenValue::Number(n) => n,
                                _ => {
                                    return Err(TokenValidationError::InvalidExpression(format!(
                                        "Token {} is not a number",
                                        left_part
                                    )))
                                }
                            };

                            // 解析右侧令牌或数字
                            let right_num =
                                if right_part.chars().all(|c| c.is_digit(10) || c == '.') {
                                    right_part.parse::<f64>().map_err(|_| {
                                        TokenValidationError::InvalidExpression(format!(
                                            "Invalid number: {}",
                                            right_part
                                        ))
                                    })?
                                } else {
                                    let right_value = self.resolve_token(right_part, theme)?;
                                    match right_value {
                                        TokenValue::Number(n) => n,
                                        _ => {
                                            return Err(TokenValidationError::InvalidExpression(
                                                format!("Token {} is not a number", right_part),
                                            ))
                                        }
                                    }
                                };

                            // 查找操作符
                            let op = expression
                                .chars()
                                .find(|&c| c == '+' || c == '-' || c == '*' || c == '/')
                                .unwrap_or('+');

                            // 执行操作
                            return Ok(TokenValue::Number(
                                self.apply_operation(left_num, right_num, op)?,
                            ));
                        }

                        return Err(TokenValidationError::InvalidExpression(format!(
                            "Invalid character in expression: {}",
                            c
                        )));
                    }
                }
            }
        }

        // 处理最后一个数字
        if !current_num.is_empty() {
            let num: f64 = current_num.parse().map_err(|_| {
                TokenValidationError::InvalidExpression(format!("Invalid number: {}", current_num))
            })?;
            result = self.apply_operation(result, num, current_op)?;
        }

        Ok(TokenValue::Number(result))
    }

    /// 应用数学运算
    fn apply_operation(
        &self,
        left: f64,
        right: f64,
        op: char,
    ) -> Result<f64, TokenValidationError> {
        match op {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => {
                if right == 0.0 {
                    Err(TokenValidationError::InvalidExpression(
                        "Division by zero".to_string(),
                    ))
                } else {
                    Ok(left / right)
                }
            }
            _ => Err(TokenValidationError::InvalidExpression(format!(
                "Unsupported operation: {}",
                op
            ))),
        }
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

#[cfg(test)]
mod tests {
    use super::super::definitions::{ColorValue, DimensionUnit, DimensionValue, TokenPath};
    use super::*;

    fn create_test_resolver() -> TokenResolver {
        let mut store = DesignTokens::default();

        // 添加数值令牌
        store.set_value(
            "size.base".to_string(),
            ThemeVariant::Light,
            TokenValue::Number(16.0),
        );
        store.set_value(
            "size.large".to_string(),
            ThemeVariant::Light,
            TokenValue::Number(24.0),
        );
        store.set_value(
            "size.small".to_string(),
            ThemeVariant::Light,
            TokenValue::Number(12.0),
        );

        // 添加引用令牌
        store.set_value(
            "size.medium".to_string(),
            ThemeVariant::Light,
            TokenValue::Reference("size.base".to_string()),
        );
        store.set_value(
            "size.xl".to_string(),
            ThemeVariant::Light,
            TokenValue::Reference("size.large".to_string()),
        );

        // 添加颜色令牌
        store.set_value(
            "color.primary".to_string(),
            ThemeVariant::Light,
            TokenValue::Color(ColorValue::new("#1890ff".to_string())),
        );
        store.set_value(
            "color.success".to_string(),
            ThemeVariant::Light,
            TokenValue::Color(ColorValue::new("#52c41a".to_string())),
        );

        // 添加尺寸令牌
        store.set_value(
            "spacing.small".to_string(),
            ThemeVariant::Light,
            TokenValue::Dimension(DimensionValue::create(4.0, DimensionUnit::Px)),
        );
        store.set_value(
            "spacing.medium".to_string(),
            ThemeVariant::Light,
            TokenValue::Dimension(DimensionValue::create(8.0, DimensionUnit::Px)),
        );

        // 添加循环引用（用于测试）
        store.set_value(
            "test.circular1".to_string(),
            ThemeVariant::Light,
            TokenValue::Reference("test.circular2".to_string()),
        );
        store.set_value(
            "test.circular2".to_string(),
            ThemeVariant::Light,
            TokenValue::Reference("test.circular1".to_string()),
        );

        TokenResolver::new(store)
    }

    #[test]
    fn test_resolve_token_basic() {
        let resolver = create_test_resolver();
        let value = resolver.resolve_token("size.base", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(16.0));
    }

    #[test]
    fn test_resolve_token_reference() {
        let resolver = create_test_resolver();
        let value = resolver.resolve_token("size.medium", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(16.0));
    }

    #[test]
    fn test_compute_expression_addition() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.base + 4", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(20.0));
    }

    #[test]
    fn test_compute_expression_subtraction() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.large - 4", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(20.0));
    }

    #[test]
    fn test_compute_expression_multiplication() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.small * 2", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(24.0));
    }

    #[test]
    fn test_compute_expression_division() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.large / 2", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(12.0));
    }

    #[test]
    fn test_compute_expression_with_references() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.medium + size.small", ThemeVariant::Light);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), TokenValue::Number(28.0));
    }

    #[test]
    fn test_compute_expression_invalid() {
        let resolver = create_test_resolver();
        let value = resolver.compute_expression("size.unknown + 4", ThemeVariant::Light);
        assert!(value.is_err());
    }

    #[test]
    fn test_list_token_paths() {
        let resolver = create_test_resolver();
        let paths = resolver.list_token_paths(ThemeVariant::Light);

        // 验证路径数量
        assert!(paths.len() >= 10);

        // 验证包含特定路径
        let path_strings: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
        assert!(path_strings.contains(&"size.base".to_string()));
        assert!(path_strings.contains(&"color.primary".to_string()));
        assert!(path_strings.contains(&"spacing.small".to_string()));
    }

    #[test]
    fn test_find_references_to() {
        let resolver = create_test_resolver();
        let references =
            resolver.find_references_to(&TokenPath::from_str("size.base"), ThemeVariant::Light);

        // 验证引用数量
        assert_eq!(references.len(), 1);

        // 验证引用路径
        assert_eq!(references[0].to_string(), "size.medium");
    }

    #[test]
    fn test_validate_references() {
        let mut resolver = create_test_resolver();
        let errors = resolver.validate_references(ThemeVariant::Light);

        // 验证检测到循环引用
        assert!(!errors.is_empty());

        // 验证错误类型
        let has_circular_error = errors
            .iter()
            .any(|e| matches!(e, TokenValidationError::CircularReference(_)));
        assert!(has_circular_error);
    }

    #[test]
    fn test_get_token_metadata() {
        let resolver = create_test_resolver();
        let metadata = resolver.get_token_metadata("color.primary");

        // 验证元数据存在
        assert!(metadata.is_some());

        // 验证元数据内容
        let metadata_str = metadata.unwrap();
        assert!(metadata_str.contains("color.primary"));
        assert!(metadata_str.contains("color"));
    }
}
