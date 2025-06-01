//! 设计令牌系统
//!
//! 本模块提供完整的设计令牌管理系统，整合了令牌定义、值存储、解析和CSS生成功能。
//! 职责：系统级功能、高级API、主题管理

use super::{
    css_generator::CssGenerator,
    token_definitions::{
        ThemeVariant, TokenDefinitions, TokenPath, TokenValidationError, TokenValue,
    },
    token_resolver::TokenResolver,
    token_values::{AntDesignTokenValues, TokenValueStore},
};
use std::collections::HashMap;

/// 设计令牌系统
#[derive(Debug)]
pub struct DesignTokenSystem {
    /// CSS生成器
    css_generator: CssGenerator,
    /// 当前主题
    current_theme: ThemeVariant,
    /// 系统配置
    config: TokenSystemConfig,
}

/// 令牌系统配置
#[derive(Debug, Clone)]
pub struct TokenSystemConfig {
    /// CSS变量前缀
    pub css_prefix: String,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 是否压缩CSS输出
    pub minify_css: bool,
    /// 是否启用严格模式（严格的类型检查）
    pub strict_mode: bool,
}

impl Default for TokenSystemConfig {
    fn default() -> Self {
        Self {
            css_prefix: "ant".to_string(),
            enable_cache: true,
            minify_css: false,
            strict_mode: false,
        }
    }
}

impl DesignTokenSystem {
    /// 创建新的设计令牌系统
    pub fn new() -> Self {
        Self::with_config(TokenSystemConfig::default())
    }

    /// 使用配置创建设计令牌系统
    pub fn with_config(config: TokenSystemConfig) -> Self {
        let store = AntDesignTokenValues::create_default_store();
        let resolver = TokenResolver::new(store);
        let css_generator = CssGenerator::new(resolver)
            .with_prefix(config.css_prefix.clone())
            .with_minify(config.minify_css);

        Self {
            css_generator,
            current_theme: ThemeVariant::Light,
            config,
        }
    }

    /// 使用自定义存储创建系统
    pub fn with_store(store: TokenValueStore, config: TokenSystemConfig) -> Self {
        let resolver = TokenResolver::new(store);
        let css_generator = CssGenerator::new(resolver)
            .with_prefix(config.css_prefix.clone())
            .with_minify(config.minify_css);

        Self {
            css_generator,
            current_theme: ThemeVariant::Light,
            config,
        }
    }

    /// 获取令牌值
    pub fn get_token(&mut self, path: &str) -> Result<TokenValue, TokenValidationError> {
        let token_path = TokenPath::from_str(path);
        self.css_generator
            .get_resolver_mut()
            .resolve_token(&token_path, self.current_theme)
    }

    /// 设置令牌值
    pub fn set_token(&mut self, path: &str, value: TokenValue) -> Result<(), String> {
        let token_path = TokenPath::from_str(path);
        self.css_generator.get_resolver_mut().set_token_value(
            &token_path,
            value,
            self.current_theme,
        )
    }

    /// 批量设置令牌值
    pub fn set_tokens_batch(&mut self, tokens: HashMap<String, TokenValue>) -> Result<(), String> {
        for (path, value) in tokens {
            self.set_token(&path, value)?;
        }
        Ok(())
    }

    /// 切换主题
    pub fn switch_theme(&mut self, theme: ThemeVariant) {
        self.current_theme = theme;
        // 清空缓存以确保使用新主题的值
        self.css_generator.get_resolver_mut().clear_cache();
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> ThemeVariant {
        self.current_theme
    }

    /// 生成当前主题的CSS变量
    pub fn generate_css_variables(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_css_variables(self.current_theme)
    }

    /// 生成完整的主题CSS
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        self.css_generator.generate_theme_css()
    }

    /// 生成组件样式类
    pub fn generate_component_css(&mut self, component: &str) -> Result<String, String> {
        self.css_generator
            .generate_component_classes(component, self.current_theme)
    }

    /// 生成实用工具类
    pub fn generate_utility_css(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_utility_classes(self.current_theme)
    }

    /// 验证所有令牌引用
    pub fn validate_tokens(&mut self) -> Vec<TokenValidationError> {
        self.css_generator
            .get_resolver_mut()
            .validate_references(self.current_theme)
    }

    /// 列出所有令牌路径
    pub fn list_tokens(&self) -> Vec<String> {
        self.css_generator
            .get_resolver()
            .list_token_paths(self.current_theme)
            .into_iter()
            .map(|path| path.to_string())
            .collect()
    }

    /// 搜索令牌
    pub fn search_tokens(&self, query: &str) -> Vec<String> {
        self.list_tokens()
            .into_iter()
            .filter(|path| path.contains(query))
            .collect()
    }

    /// 获取令牌的CSS变量名
    pub fn get_css_var_name(&self, path: &str) -> String {
        let token_path = TokenPath::from_str(path);
        format!(
            "--{}-{}",
            self.config.css_prefix,
            token_path.segments.join("-")
        )
    }

    /// 导出令牌为JSON
    pub fn export_tokens_json(&mut self) -> Result<String, String> {
        let paths = self
            .css_generator
            .get_resolver()
            .list_token_paths(self.current_theme);
        let mut tokens = HashMap::new();

        for path in paths {
            match self
                .css_generator
                .get_resolver_mut()
                .resolve_token(&path, self.current_theme)
            {
                Ok(value) => {
                    tokens.insert(path.to_string(), value);
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to resolve token {}: {}",
                        path.to_string(),
                        e
                    ));
                }
            }
        }

        serde_json::to_string_pretty(&tokens)
            .map_err(|e| format!("Failed to serialize tokens: {}", e))
    }

    /// 从JSON导入令牌
    pub fn import_tokens_json(&mut self, json: &str) -> Result<(), String> {
        let tokens: HashMap<String, TokenValue> =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        self.set_tokens_batch(tokens)
    }

    /// 创建主题变体
    pub fn create_theme_variant(
        &mut self,
        base_theme: ThemeVariant,
        new_theme: ThemeVariant,
        overrides: HashMap<String, TokenValue>,
    ) -> Result<(), String> {
        // 复制基础主题
        self.css_generator
            .get_resolver_mut()
            .get_store_mut()
            .copy_theme(base_theme, new_theme);

        // 应用覆盖值
        let old_theme = self.current_theme;
        self.current_theme = new_theme;

        for (path, value) in overrides {
            self.set_token(&path, value)?;
        }

        self.current_theme = old_theme;
        Ok(())
    }

    /// 获取令牌的所有引用者
    pub fn get_token_references(&self, path: &str) -> Vec<String> {
        let token_path = TokenPath::from_str(path);
        self.css_generator
            .get_resolver()
            .find_references_to(&token_path, self.current_theme)
            .into_iter()
            .map(|path| path.to_string())
            .collect()
    }

    /// 计算令牌表达式
    pub fn compute_expression(
        &mut self,
        expression: &str,
    ) -> Result<TokenValue, TokenValidationError> {
        self.css_generator
            .get_resolver_mut()
            .compute_value(expression, self.current_theme)
    }

    /// 获取系统配置
    pub fn get_config(&self) -> &TokenSystemConfig {
        &self.config
    }

    /// 更新系统配置
    pub fn update_config(&mut self, config: TokenSystemConfig) {
        self.config = config;
        // 重新配置CSS生成器
        let store = std::mem::replace(
            self.css_generator.get_resolver_mut().get_store_mut(),
            TokenValueStore::new(),
        );
        let resolver = TokenResolver::new(store);
        self.css_generator = CssGenerator::new(resolver)
            .with_prefix(self.config.css_prefix.clone())
            .with_minify(self.config.minify_css);
    }

    /// 获取支持的主题列表
    pub fn get_supported_themes(&self) -> Vec<ThemeVariant> {
        self.css_generator
            .get_resolver()
            .get_store()
            .get_supported_themes()
    }

    /// 清空指定主题的所有令牌
    pub fn clear_theme(&mut self, theme: ThemeVariant) {
        self.css_generator
            .get_resolver_mut()
            .get_store_mut()
            .clear_theme(theme);
    }

    /// 重置为默认令牌值
    pub fn reset_to_defaults(&mut self) {
        let store = AntDesignTokenValues::create_default_store();
        let resolver = TokenResolver::new(store);
        self.css_generator = CssGenerator::new(resolver)
            .with_prefix(self.config.css_prefix.clone())
            .with_minify(self.config.minify_css);
    }
}

impl Default for DesignTokenSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局令牌系统实例
static mut GLOBAL_TOKEN_SYSTEM: Option<DesignTokenSystem> = None;
static mut GLOBAL_TOKEN_SYSTEM_INITIALIZED: bool = false;

/// 获取全局令牌系统实例
pub fn get_global_token_system() -> &'static mut DesignTokenSystem {
    unsafe {
        if !GLOBAL_TOKEN_SYSTEM_INITIALIZED {
            GLOBAL_TOKEN_SYSTEM = Some(DesignTokenSystem::new());
            GLOBAL_TOKEN_SYSTEM_INITIALIZED = true;
        }
        GLOBAL_TOKEN_SYSTEM.as_mut().unwrap()
    }
}

/// 初始化全局令牌系统
pub fn init_global_token_system(config: TokenSystemConfig) {
    unsafe {
        GLOBAL_TOKEN_SYSTEM = Some(DesignTokenSystem::with_config(config));
        GLOBAL_TOKEN_SYSTEM_INITIALIZED = true;
    }
}

/// 便捷宏：获取令牌值
#[macro_export]
macro_rules! token {
    ($path:expr) => {
        $crate::theme::get_global_token_system().get_token($path)
    };
}

/// 便捷宏：获取CSS变量名
#[macro_export]
macro_rules! css_var {
    ($path:expr) => {
        $crate::theme::get_global_token_system().get_css_var_name($path)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_token_system_creation() {
        let system = DesignTokenSystem::new();
        assert_eq!(system.get_current_theme(), ThemeVariant::Light);
    }

    #[test]
    fn test_token_operations() {
        let mut system = DesignTokenSystem::new();

        // 测试获取令牌
        let result = system.get_token("color.primary.500");
        assert!(result.is_ok());

        // 测试设置令牌
        let set_result =
            system.set_token("test.token", TokenValue::String("test-value".to_string()));
        assert!(set_result.is_ok());

        // 测试获取设置的令牌
        let get_result = system.get_token("test.token");
        assert!(get_result.is_ok());
    }

    #[test]
    fn test_theme_switching() {
        let mut system = DesignTokenSystem::new();

        assert_eq!(system.get_current_theme(), ThemeVariant::Light);

        system.switch_theme(ThemeVariant::Dark);
        assert_eq!(system.get_current_theme(), ThemeVariant::Dark);
    }

    #[test]
    fn test_css_generation() {
        let mut system = DesignTokenSystem::new();

        let css_vars = system.generate_css_variables();
        assert!(css_vars.is_ok());

        let theme_css = system.generate_theme_css();
        assert!(theme_css.is_ok());
    }

    #[test]
    fn test_token_validation() {
        let mut system = DesignTokenSystem::new();

        let errors = system.validate_tokens();
        // 默认配置应该没有验证错误
        assert!(errors.is_empty());
    }

    #[test]
    fn test_global_token_system() {
        let system = get_global_token_system();
        assert_eq!(system.get_current_theme(), ThemeVariant::Light);
    }
}
