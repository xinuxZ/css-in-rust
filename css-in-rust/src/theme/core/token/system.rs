//! 设计令牌系统
//!
//! 本模块提供完整的设计令牌管理系统，整合了令牌定义、值存储、解析和CSS生成功能。
//! 职责：系统级功能、高级API、主题管理
//!
//! 实现完整的分层令牌系统架构：
//! - 全局令牌（Global Tokens）：最基础的设计决策
//! - 别名令牌（Alias Tokens）：语义化的令牌引用
//! - 组件令牌（Component Tokens）：特定组件的令牌
//! - 令牌计算和变换系统
//! - 主题管理和切换
//! - CSS变量生成和导出

use super::css_generator::CssGenerator;
use super::definitions::{
    ColorValue, DimensionValue, ThemeVariant, TokenPath, TokenValue, TypographyValue,
};
use std::collections::HashMap;

/// 设计令牌系统配置
#[derive(Debug, Clone, Default)]
pub struct TokenSystemConfig {
    /// 前缀
    pub prefix: String,
    /// 是否启用压缩
    pub minify: bool,
}

/// 系统元数据
#[derive(Debug, Clone, Default)]
pub struct SystemMetadata {
    /// 系统名称
    pub name: String,
    /// 系统版本
    pub version: String,
    /// 系统描述
    pub description: String,
}

/// 全局令牌
#[derive(Debug, Clone, Default)]
pub struct GlobalTokens {
    /// 颜色
    pub colors: HashMap<String, ColorValue>,
    /// 尺寸
    pub dimensions: HashMap<String, DimensionValue>,
    /// 排版
    pub typography: HashMap<String, TypographyValue>,
}

/// 别名令牌
#[derive(Debug, Clone, Default)]
pub struct AliasTokens {
    /// 颜色别名
    pub colors: HashMap<String, String>,
    /// 尺寸别名
    pub dimensions: HashMap<String, String>,
    /// 排版别名
    pub typography: HashMap<String, String>,
}

/// 组件令牌
#[derive(Debug, Clone, Default)]
pub struct ComponentTokens {
    /// 组件特定令牌
    pub components: HashMap<String, HashMap<String, String>>,
}

/// 计算规则
#[derive(Debug, Clone, Default)]
pub struct ComputationRules {
    /// 规则映射
    pub rules: HashMap<String, String>,
}

/// 设计令牌系统
#[derive(Debug, Clone)]
pub struct DesignTokenSystem {
    /// 全局令牌（最基础的设计决策）
    pub global_tokens: GlobalTokens,
    /// 别名令牌（语义化的令牌引用）
    pub alias_tokens: AliasTokens,
    /// 组件令牌（特定组件的令牌）
    pub component_tokens: ComponentTokens,
    /// 令牌计算规则
    pub computation_rules: ComputationRules,
    /// CSS生成器
    css_generator: CssGenerator,
    /// 当前主题
    pub current_theme: ThemeVariant,
    /// 系统配置
    pub config: TokenSystemConfig,
    /// 系统元数据
    pub metadata: SystemMetadata,
}

impl Default for DesignTokenSystem {
    fn default() -> Self {
        Self {
            global_tokens: GlobalTokens::default(),
            alias_tokens: AliasTokens::default(),
            component_tokens: ComponentTokens::default(),
            computation_rules: ComputationRules::default(),
            css_generator: CssGenerator::default(),
            current_theme: ThemeVariant::Light,
            config: TokenSystemConfig::default(),
            metadata: SystemMetadata::default(),
        }
    }
}

impl DesignTokenSystem {
    /// 创建新的设计令牌系统
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置前缀
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.prefix = prefix.into();
        self
    }

    /// 设置是否启用压缩
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.config.minify = minify;
        self
    }

    /// 设置系统名称
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.metadata.name = name.into();
        self
    }

    /// 设置系统版本
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.metadata.version = version.into();
        self
    }

    /// 设置系统描述
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = description.into();
        self
    }

    /// 设置当前主题
    pub fn with_theme(mut self, theme: ThemeVariant) -> Self {
        self.current_theme = theme;
        self
    }

    /// 生成CSS变量
    pub fn generate_css_variables(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_css_variables(self.current_theme)
    }

    /// 生成主题CSS
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        self.css_generator.generate_theme_css()
    }

    /// 生成组件CSS
    pub fn generate_component_css(&mut self, component: &str) -> Result<String, String> {
        self.css_generator
            .generate_component_classes(component, self.current_theme)
    }

    /// 生成工具类CSS
    pub fn generate_utility_classes(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_utility_classes(self.current_theme)
    }

    /// 解析令牌
    pub fn resolve_token(&self, path: &str) -> Result<TokenValue, String> {
        match self
            .css_generator
            .get_resolver()
            .resolve_token(path, self.current_theme)
        {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }

    /// 设置令牌值
    pub fn set_token(&mut self, path: &str, value: TokenValue) -> Result<(), String> {
        let token_path = TokenPath::from_str(path);
        match self.css_generator.get_resolver_mut().set_token(
            &token_path,
            value,
            self.current_theme,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// 切换主题
    pub fn switch_theme(&mut self, theme: ThemeVariant) {
        self.current_theme = theme;
        // 清空缓存以确保使用新主题的值
        self.css_generator.get_resolver_mut().clear_cache();
    }
}
