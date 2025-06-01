//! 主题管理器
//!
//! 提供高级主题管理功能，包括：
//! - 主题注册和管理
//! - 主题持久化
//! - 动态主题加载
//! - 主题验证
//! - 主题缓存

use super::css_variables::{CssVariableInjector, CssVariableManager, InjectionStrategy};
use super::design_tokens::DesignTokens;
use super::{Theme, ThemeMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

/// 主题管理器错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeManagerError {
    /// 主题未找到
    ThemeNotFound(String),
    /// 主题已存在
    ThemeAlreadyExists(String),
    /// 无效的主题数据
    InvalidThemeData(String),
    /// 序列化错误
    SerializationError(String),
    /// 反序列化错误
    DeserializationError(String),
    /// CSS 注入错误
    CssInjectionError(String),
}

impl std::fmt::Display for ThemeManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThemeNotFound(name) => write!(f, "主题未找到: {}", name),
            Self::ThemeAlreadyExists(name) => write!(f, "主题已存在: {}", name),
            Self::InvalidThemeData(msg) => write!(f, "无效的主题数据: {}", msg),
            Self::SerializationError(msg) => write!(f, "序列化错误: {}", msg),
            Self::DeserializationError(msg) => write!(f, "反序列化错误: {}", msg),
            Self::CssInjectionError(msg) => write!(f, "CSS 注入错误: {}", msg),
        }
    }
}

impl std::error::Error for ThemeManagerError {}

/// 主题管理器结果类型
pub type ThemeManagerResult<T> = Result<T, ThemeManagerError>;

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// 默认主题名称
    pub default_theme: String,
    /// 是否启用主题持久化
    pub enable_persistence: bool,
    /// 持久化存储键
    pub storage_key: String,
    /// CSS 注入策略
    pub injection_strategy: InjectionStrategy,
    /// 是否启用主题验证
    pub enable_validation: bool,
    /// 是否启用主题缓存
    pub enable_cache: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            default_theme: "ant-design-default".to_string(),
            enable_persistence: true,
            storage_key: "app-theme".to_string(),
            injection_strategy: InjectionStrategy::StyleTag,
            enable_validation: true,
            enable_cache: true,
        }
    }
}

/// 主题元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// 主题名称
    pub name: String,
    /// 主题显示名称
    pub display_name: String,
    /// 主题描述
    pub description: String,
    /// 主题版本
    pub version: String,
    /// 主题作者
    pub author: String,
    /// 主题标签
    pub tags: Vec<String>,
    /// 是否为内置主题
    pub is_builtin: bool,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

impl Default for ThemeMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            description: String::new(),
            version: "1.0.0".to_string(),
            author: String::new(),
            tags: Vec::new(),
            is_builtin: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// 主题包装器
#[derive(Debug, Clone)]
pub struct ThemeWrapper {
    /// 主题实例
    pub theme: Theme,
    /// 主题元数据
    pub metadata: ThemeMetadata,
    /// 设计令牌
    pub design_tokens: DesignTokens,
}

impl ThemeWrapper {
    /// 创建新的主题包装器
    pub fn new(theme: Theme, metadata: ThemeMetadata, design_tokens: DesignTokens) -> Self {
        Self {
            theme,
            metadata,
            design_tokens,
        }
    }

    /// 创建 Ant Design 默认主题包装器
    pub fn ant_design_default() -> Self {
        let theme = Theme::ant_design_default();
        let metadata = ThemeMetadata {
            name: "ant-design-default".to_string(),
            display_name: "Ant Design 默认主题".to_string(),
            description: "Ant Design 官方默认亮色主题".to_string(),
            version: "5.0.0".to_string(),
            author: "Ant Design Team".to_string(),
            tags: vec!["official".to_string(), "light".to_string()],
            is_builtin: true,
            ..Default::default()
        };
        let design_tokens = DesignTokens::ant_design_light();

        Self::new(theme, metadata, design_tokens)
    }

    /// 创建 Ant Design 暗色主题包装器
    pub fn ant_design_dark() -> Self {
        let theme = Theme::ant_design_dark();
        let metadata = ThemeMetadata {
            name: "ant-design-dark".to_string(),
            display_name: "Ant Design 暗色主题".to_string(),
            description: "Ant Design 官方暗色主题".to_string(),
            version: "5.0.0".to_string(),
            author: "Ant Design Team".to_string(),
            tags: vec!["official".to_string(), "dark".to_string()],
            is_builtin: true,
            ..Default::default()
        };
        let design_tokens = DesignTokens::ant_design_dark();

        Self::new(theme, metadata, design_tokens)
    }

    /// 生成 CSS 变量
    pub fn generate_css_variables(&self) -> String {
        self.theme.generate_design_tokens_css(&self.design_tokens)
    }

    /// 验证主题数据
    pub fn validate(&self) -> ThemeManagerResult<()> {
        // 验证主题名称
        if self.metadata.name.is_empty() {
            return Err(ThemeManagerError::InvalidThemeData(
                "主题名称不能为空".to_string(),
            ));
        }

        // 验证版本格式
        if !self.is_valid_version(&self.metadata.version) {
            return Err(ThemeManagerError::InvalidThemeData(format!(
                "无效的版本格式: {}",
                self.metadata.version
            )));
        }

        Ok(())
    }

    /// 检查版本格式是否有效
    fn is_valid_version(&self, version: &str) -> bool {
        // 简单的语义版本验证
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() == 3 && parts.iter().all(|part| part.parse::<u32>().is_ok())
    }
}

/// 主题管理器
#[derive(Debug)]
pub struct ThemeManager {
    /// 注册的主题
    themes: HashMap<String, ThemeWrapper>,
    /// 当前活动主题
    current_theme: Option<String>,
    /// 配置
    config: ThemeConfig,
    /// CSS 变量管理器
    css_manager: CssVariableManager,
    /// CSS 变量注入器
    css_injector: CssVariableInjector,
    /// 主题缓存
    cache: HashMap<String, String>, // 主题名 -> CSS 缓存
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(config: ThemeConfig) -> Self {
        let css_injector = CssVariableInjector::new(config.injection_strategy.clone());

        Self {
            themes: HashMap::new(),
            current_theme: None,
            config,
            css_manager: CssVariableManager::new(),
            css_injector,
            cache: HashMap::new(),
        }
    }

    /// 使用默认配置创建主题管理器
    pub fn default() -> Self {
        Self::new(ThemeConfig::default())
    }

    /// 初始化内置主题
    pub fn init_builtin_themes(&mut self) -> ThemeManagerResult<()> {
        // 注册 Ant Design 默认主题
        self.register_theme(ThemeWrapper::ant_design_default())?;

        // 注册 Ant Design 暗色主题
        self.register_theme(ThemeWrapper::ant_design_dark())?;

        // 设置默认主题
        if self.current_theme.is_none() {
            self.set_current_theme(&self.config.default_theme)?;
        }

        Ok(())
    }

    /// 注册主题
    pub fn register_theme(&mut self, theme_wrapper: ThemeWrapper) -> ThemeManagerResult<()> {
        let theme_name = theme_wrapper.metadata.name.clone();

        // 验证主题数据
        if self.config.enable_validation {
            theme_wrapper.validate()?;
        }

        // 检查主题是否已存在
        if self.themes.contains_key(&theme_name) {
            return Err(ThemeManagerError::ThemeAlreadyExists(theme_name));
        }

        // 注册主题
        self.themes.insert(theme_name.clone(), theme_wrapper);

        // 预生成 CSS 缓存
        if self.config.enable_cache {
            self.generate_css_cache(&theme_name)?;
        }

        Ok(())
    }

    /// 注销主题
    pub fn unregister_theme(&mut self, theme_name: &str) -> ThemeManagerResult<()> {
        // 检查主题是否存在
        if !self.themes.contains_key(theme_name) {
            return Err(ThemeManagerError::ThemeNotFound(theme_name.to_string()));
        }

        // 如果是当前主题，需要先切换到其他主题
        if self.current_theme.as_ref() == Some(&theme_name.to_string()) {
            // 尝试切换到默认主题
            if theme_name != self.config.default_theme {
                self.set_current_theme(&self.config.default_theme)?;
            } else {
                // 如果要删除的是默认主题，清空当前主题
                self.current_theme = None;
            }
        }

        // 删除主题和缓存
        self.themes.remove(theme_name);
        self.cache.remove(theme_name);

        Ok(())
    }

    /// 获取主题
    pub fn get_theme(&self, theme_name: &str) -> ThemeManagerResult<&ThemeWrapper> {
        self.themes
            .get(theme_name)
            .ok_or_else(|| ThemeManagerError::ThemeNotFound(theme_name.to_string()))
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> ThemeManagerResult<&ThemeWrapper> {
        let current_name = self
            .current_theme
            .as_ref()
            .ok_or_else(|| ThemeManagerError::ThemeNotFound("没有设置当前主题".to_string()))?;

        self.get_theme(current_name)
    }

    /// 设置当前主题
    pub fn set_current_theme(&mut self, theme_name: &str) -> ThemeManagerResult<()> {
        // 检查主题是否存在
        if !self.themes.contains_key(theme_name) {
            return Err(ThemeManagerError::ThemeNotFound(theme_name.to_string()));
        }

        // 设置当前主题
        self.current_theme = Some(theme_name.to_string());

        // 应用主题 CSS
        self.apply_current_theme()?;

        // 持久化主题选择
        if self.config.enable_persistence {
            self.persist_current_theme()?;
        }

        Ok(())
    }

    /// 切换主题模式（亮色/暗色）
    pub fn toggle_theme_mode(&mut self) -> ThemeManagerResult<()> {
        let current_theme = self.get_current_theme()?.clone();
        let current_mode = current_theme.theme.mode;

        // 根据当前模式切换到对应的主题
        let target_theme = match current_mode {
            ThemeMode::Light => "ant-design-dark",
            ThemeMode::Dark => "ant-design-default",
        };

        self.set_current_theme(target_theme)
    }

    /// 获取所有主题列表
    pub fn list_themes(&self) -> Vec<&ThemeMetadata> {
        self.themes
            .values()
            .map(|wrapper| &wrapper.metadata)
            .collect()
    }

    /// 按标签筛选主题
    pub fn filter_themes_by_tag(&self, tag: &str) -> Vec<&ThemeMetadata> {
        self.themes
            .values()
            .filter(|wrapper| wrapper.metadata.tags.contains(&tag.to_string()))
            .map(|wrapper| &wrapper.metadata)
            .collect()
    }

    /// 搜索主题
    pub fn search_themes(&self, query: &str) -> Vec<&ThemeMetadata> {
        let query_lower = query.to_lowercase();
        self.themes
            .values()
            .filter(|wrapper| {
                wrapper.metadata.name.to_lowercase().contains(&query_lower)
                    || wrapper
                        .metadata
                        .display_name
                        .to_lowercase()
                        .contains(&query_lower)
                    || wrapper
                        .metadata
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || wrapper
                        .metadata
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .map(|wrapper| &wrapper.metadata)
            .collect()
    }

    /// 应用当前主题
    fn apply_current_theme(&mut self) -> ThemeManagerResult<()> {
        let current_theme = self.get_current_theme()?.clone();

        // 获取或生成 CSS
        let css = if self.config.enable_cache {
            self.get_cached_css(&current_theme.metadata.name)?
        } else {
            current_theme.generate_css_variables()
        };

        // 注入 CSS 变量
        self.css_injector
            .inject_css_variables(&css)
            .map_err(|e| ThemeManagerError::CssInjectionError(e.to_string()))?;

        Ok(())
    }

    /// 生成 CSS 缓存
    fn generate_css_cache(&mut self, theme_name: &str) -> ThemeManagerResult<()> {
        let theme_wrapper = self.get_theme(theme_name)?.clone();
        let css = theme_wrapper.generate_css_variables();
        self.cache.insert(theme_name.to_string(), css);
        Ok(())
    }

    /// 获取缓存的 CSS
    fn get_cached_css(&mut self, theme_name: &str) -> ThemeManagerResult<String> {
        if let Some(css) = self.cache.get(theme_name) {
            Ok(css.clone())
        } else {
            // 生成并缓存 CSS
            self.generate_css_cache(theme_name)?;
            Ok(self.cache.get(theme_name).unwrap().clone())
        }
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 重新生成所有缓存
    pub fn regenerate_cache(&mut self) -> ThemeManagerResult<()> {
        self.clear_cache();

        for theme_name in self.themes.keys().cloned().collect::<Vec<_>>() {
            self.generate_css_cache(&theme_name)?;
        }

        Ok(())
    }

    /// 持久化当前主题
    fn persist_current_theme(&self) -> ThemeManagerResult<()> {
        if let Some(current_theme) = &self.current_theme {
            // 这里应该实现实际的持久化逻辑
            // 例如保存到 localStorage、文件系统等
            log::info!("持久化主题: {}", current_theme);
        }
        Ok(())
    }

    /// 从持久化存储加载主题
    pub fn load_persisted_theme(&mut self) -> ThemeManagerResult<()> {
        // 这里应该实现实际的加载逻辑
        // 例如从 localStorage、文件系统等加载
        log::info!("加载持久化主题");
        Ok(())
    }

    /// 导出主题配置
    pub fn export_theme(&self, theme_name: &str) -> ThemeManagerResult<String> {
        let theme_wrapper = self.get_theme(theme_name)?;

        serde_json::to_string_pretty(theme_wrapper)
            .map_err(|e| ThemeManagerError::SerializationError(e.to_string()))
    }

    /// 导入主题配置
    pub fn import_theme(&mut self, theme_data: &str) -> ThemeManagerResult<()> {
        let theme_wrapper: ThemeWrapper = serde_json::from_str(theme_data)
            .map_err(|e| ThemeManagerError::DeserializationError(e.to_string()))?;

        self.register_theme(theme_wrapper)
    }

    /// 获取配置
    pub fn config(&self) -> &ThemeConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, config: ThemeConfig) {
        self.config = config;
        // 重新创建 CSS 注入器
        self.css_injector = CssVariableInjector::new(self.config.injection_strategy.clone());
    }
}

// 为 ThemeWrapper 实现序列化支持
impl Serialize for ThemeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("ThemeWrapper", 3)?;
        state.serialize_field("theme", &self.theme)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.serialize_field("design_tokens", &self.design_tokens)?;
        state.end()
    }
}

// 为 ThemeWrapper 实现反序列化支持
impl<'de> Deserialize<'de> for ThemeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct ThemeWrapperVisitor;

        impl<'de> Visitor<'de> for ThemeWrapperVisitor {
            type Value = ThemeWrapper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ThemeWrapper")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ThemeWrapper, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut theme = None;
                let mut metadata = None;
                let mut design_tokens = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "theme" => {
                            if theme.is_some() {
                                return Err(de::Error::duplicate_field("theme"));
                            }
                            theme = Some(map.next_value()?);
                        }
                        "metadata" => {
                            if metadata.is_some() {
                                return Err(de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        }
                        "design_tokens" => {
                            if design_tokens.is_some() {
                                return Err(de::Error::duplicate_field("design_tokens"));
                            }
                            design_tokens = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<serde_json::Value>()?;
                        }
                    }
                }

                let theme = theme.ok_or_else(|| de::Error::missing_field("theme"))?;
                let metadata = metadata.ok_or_else(|| de::Error::missing_field("metadata"))?;
                let design_tokens =
                    design_tokens.ok_or_else(|| de::Error::missing_field("design_tokens"))?;

                Ok(ThemeWrapper::new(theme, metadata, design_tokens))
            }
        }

        const FIELDS: &[&str] = &["theme", "metadata", "design_tokens"];
        deserializer.deserialize_struct("ThemeWrapper", FIELDS, ThemeWrapperVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let config = ThemeConfig::default();
        let manager = ThemeManager::new(config);

        assert!(manager.themes.is_empty());
        assert!(manager.current_theme.is_none());
    }

    #[test]
    fn test_theme_registration() {
        let mut manager = ThemeManager::default();
        let theme_wrapper = ThemeWrapper::ant_design_default();

        assert!(manager.register_theme(theme_wrapper).is_ok());
        assert!(manager.themes.contains_key("ant-design-default"));
    }

    #[test]
    fn test_theme_switching() {
        let mut manager = ThemeManager::default();
        manager.init_builtin_themes().unwrap();

        assert!(manager.set_current_theme("ant-design-default").is_ok());
        assert_eq!(
            manager.current_theme,
            Some("ant-design-default".to_string())
        );

        assert!(manager.set_current_theme("ant-design-dark").is_ok());
        assert_eq!(manager.current_theme, Some("ant-design-dark".to_string()));
    }

    #[test]
    fn test_theme_search() {
        let mut manager = ThemeManager::default();
        manager.init_builtin_themes().unwrap();

        let results = manager.search_themes("ant");
        assert_eq!(results.len(), 2);

        let results = manager.search_themes("dark");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "ant-design-dark");
    }

    #[test]
    fn test_theme_validation() {
        let theme = Theme::ant_design_default();
        let metadata = ThemeMetadata {
            name: "".to_string(), // 无效的空名称
            ..Default::default()
        };
        let design_tokens = DesignTokens::ant_design_light();

        let theme_wrapper = ThemeWrapper::new(theme, metadata, design_tokens);
        assert!(theme_wrapper.validate().is_err());
    }
}
