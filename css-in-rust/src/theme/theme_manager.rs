//! 主题管理器 - 提供完整的主题系统功能
//!
//! 这个模块实现了一个功能完整的主题管理系统，支持：
//! - 多主题切换
//! - 深色/浅色模式
//! - 主题变量动态计算
//! - 主题继承和覆盖
//! - 运行时主题更新

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{DesignTokenSystem, DesignTokens, Theme, ThemeMode, TokenValue};

/// 主题历史记录管理器
#[derive(Debug, Clone)]
pub struct ThemeHistory {
    /// 历史记录
    history: Arc<RwLock<Vec<String>>>,
    /// 最大历史记录数
    max_history: usize,
}

impl ThemeHistory {
    /// 创建新的主题历史管理器
    pub fn new() -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            max_history: 10,
        }
    }

    /// 设置最大历史记录数
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// 添加主题到历史记录
    pub fn add_theme(&self, theme_name: &str) -> Result<(), String> {
        let mut history = self
            .history
            .write()
            .map_err(|_| "Failed to acquire write lock")?;

        history.push(theme_name.to_string());

        // 限制历史记录数量
        if history.len() > self.max_history {
            history.remove(0);
        }

        Ok(())
    }

    /// 获取主题历史记录
    pub fn get_history(&self) -> Result<Vec<String>, String> {
        let history = self
            .history
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(history.clone())
    }

    /// 获取上一个主题
    pub fn get_previous_theme(&self) -> Result<Option<String>, String> {
        let history = self
            .history
            .read()
            .map_err(|_| "Failed to acquire read lock")?;

        if history.len() >= 2 {
            Ok(Some(history[history.len() - 2].clone()))
        } else {
            Ok(None)
        }
    }

    /// 清空历史记录
    pub fn clear_history(&self) -> Result<(), String> {
        let mut history = self
            .history
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        history.clear();
        Ok(())
    }
}

impl Default for ThemeHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// 主题管理器 - 核心主题系统
#[derive(Debug, Clone)]
pub struct ThemeManager {
    /// 当前激活的主题
    current_theme: Arc<RwLock<Theme>>,
    /// 注册的主题集合
    registered_themes: Arc<RwLock<HashMap<String, Theme>>>,
    /// 设计令牌系统
    token_system: Arc<RwLock<DesignTokenSystem>>,
    /// 主题变更监听器
    listeners: Arc<RwLock<Vec<Box<dyn ThemeChangeListener>>>>,
    /// 主题历史记录
    theme_history: ThemeHistory,
    /// 主题配置
    config: ThemeManagerConfig,
}

/// 主题管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManagerConfig {
    /// 是否启用自动深色模式检测
    pub auto_dark_mode: bool,
    /// 是否启用主题过渡动画
    pub enable_transitions: bool,
    /// 主题切换动画持续时间（毫秒）
    pub transition_duration: u32,
    /// 是否启用主题缓存
    pub enable_caching: bool,
    /// 默认主题名称
    pub default_theme: String,
    /// 默认主题模式
    pub default_mode: ThemeMode,
}

/// 主题变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeChangeEvent {
    /// 事件类型
    pub event_type: ThemeChangeEventType,
    /// 旧主题（如果适用）
    pub old_theme: Option<String>,
    /// 新主题
    pub new_theme: String,
    /// 旧模式（如果适用）
    pub old_mode: Option<ThemeMode>,
    /// 新模式
    pub new_mode: ThemeMode,
    /// 事件时间戳
    pub timestamp: u64,
    /// 变更原因
    pub reason: ThemeChangeReason,
}

/// 主题变更事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeChangeEventType {
    /// 主题切换
    ThemeSwitch,
    /// 模式切换
    ModeSwitch,
    /// 主题更新
    ThemeUpdate,
    /// 令牌更新
    TokenUpdate,
}

/// 主题变更原因
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeChangeReason {
    /// 用户手动切换
    UserSwitch,
    /// 系统自动切换（如暗色模式）
    SystemAuto,
    /// 程序化切换
    Programmatic,
    /// 作用域覆盖
    ScopeOverride,
}

/// 主题作用域
///
/// 为特定组件或区域提供局部主题覆盖
#[derive(Debug, Clone)]
pub struct ThemeScope {
    /// 父级主题上下文
    pub parent_context: Option<String>,
    /// 当前作用域主题
    pub scoped_theme: Theme,
    /// 主题覆盖配置
    pub overrides: HashMap<String, String>,
}

/// 主题变更监听器
pub trait ThemeChangeListener: Send + Sync + std::fmt::Debug {
    /// 处理主题变更事件
    fn on_theme_change(&self, event: &ThemeChangeEvent);
}

/// 主题构建器
#[derive(Debug, Default)]
pub struct ThemeBuilder {
    name: Option<String>,
    design_tokens: Option<DesignTokens>,
    custom_variables: HashMap<String, String>,
    mode: Option<ThemeMode>,
    parent_theme: Option<String>,
}

/// 主题覆盖配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeOverride {
    /// 要覆盖的令牌路径
    pub token_path: String,
    /// 新的令牌值
    pub new_value: TokenValue,
    /// 覆盖条件（可选）
    pub condition: Option<String>,
}

impl Default for ThemeManagerConfig {
    fn default() -> Self {
        Self {
            auto_dark_mode: true,
            enable_transitions: true,
            transition_duration: 300,
            enable_caching: true,
            default_theme: "ant-design".to_string(),
            default_mode: ThemeMode::Light,
        }
    }
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(config: ThemeManagerConfig) -> Self {
        let default_theme =
            Self::create_default_theme(&config.default_theme, config.clone().default_mode);
        let mut registered_themes = HashMap::new();
        registered_themes.insert(config.default_theme.clone(), default_theme.clone());

        Self {
            current_theme: Arc::new(RwLock::new(default_theme)),
            registered_themes: Arc::new(RwLock::new(registered_themes)),
            token_system: Arc::new(RwLock::new(DesignTokenSystem::ant_design_default())),
            listeners: Arc::new(RwLock::new(Vec::new())),
            theme_history: ThemeHistory::new(),
            config,
        }
    }

    /// 创建默认主题管理器
    pub fn default() -> Self {
        Self::new(ThemeManagerConfig::default())
    }

    /// 注册新主题
    pub fn register_theme(&self, theme: Theme) -> Result<(), ThemeError> {
        let mut themes = self.registered_themes.write().unwrap();
        themes.insert(theme.name.clone(), theme);
        Ok(())
    }

    /// 切换到指定主题
    pub fn switch_theme(&self, theme_name: &str) -> Result<(), ThemeError> {
        let themes = self.registered_themes.read().unwrap();
        let theme = themes
            .get(theme_name)
            .ok_or_else(|| ThemeError::ThemeNotFound(theme_name.to_string()))?;

        let old_theme = {
            let current = self.current_theme.read().unwrap();
            current.clone()
        };

        {
            let mut current = self.current_theme.write().unwrap();
            *current = theme.clone();
        }

        // 添加到历史记录
        if let Err(e) = self.theme_history.add_theme(theme_name) {
            eprintln!("Warning: Failed to add theme to history: {}", e);
        }

        // 触发主题变更事件
        self.emit_theme_change_event(ThemeChangeEvent {
            event_type: ThemeChangeEventType::ThemeSwitch,
            old_theme: Some(old_theme.name),
            new_theme: theme.name.clone(),
            old_mode: Some(old_theme.mode),
            new_mode: theme.mode.clone(),
            timestamp: Self::current_timestamp(),
            reason: ThemeChangeReason::UserSwitch,
        });

        Ok(())
    }

    /// 切换主题模式（深色/浅色）
    pub fn switch_mode(&self, mode: ThemeMode) -> Result<(), ThemeError> {
        let old_mode = {
            let current = self.current_theme.read().unwrap();
            current.mode.clone()
        };

        {
            let mut current = self.current_theme.write().unwrap();
            current.mode = mode.clone();
        }

        // 触发模式变更事件
        let current_name = {
            let current = self.current_theme.read().unwrap();
            current.name.clone()
        };

        self.emit_theme_change_event(ThemeChangeEvent {
            event_type: ThemeChangeEventType::ModeSwitch,
            old_theme: None,
            new_theme: current_name,
            old_mode: Some(old_mode),
            new_mode: mode,
            timestamp: Self::current_timestamp(),
            reason: ThemeChangeReason::UserSwitch,
        });

        Ok(())
    }

    /// 获取当前主题
    pub fn current_theme(&self) -> Theme {
        self.current_theme.read().unwrap().clone()
    }

    /// 获取所有注册的主题名称
    pub fn get_theme_names(&self) -> Vec<String> {
        self.registered_themes
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }

    /// 应用主题覆盖
    pub fn apply_overrides(&self, overrides: Vec<ThemeOverride>) -> Result<(), ThemeError> {
        let mut token_system = self.token_system.write().unwrap();

        for override_config in overrides {
            token_system.set_token(&override_config.token_path, override_config.new_value)?;
        }

        // 触发令牌更新事件
        let current_name = {
            let current = self.current_theme.read().unwrap();
            current.name.clone()
        };

        self.emit_theme_change_event(ThemeChangeEvent {
            event_type: ThemeChangeEventType::TokenUpdate,
            old_theme: None,
            new_theme: current_name,
            old_mode: None,
            new_mode: self.current_theme().mode,
            timestamp: Self::current_timestamp(),
            reason: ThemeChangeReason::Programmatic,
        });

        Ok(())
    }

    /// 生成当前主题的 CSS
    pub fn generate_css(&self) -> Result<String, ThemeError> {
        let current_theme = self.current_theme();
        let token_system = self.token_system.read().unwrap();

        let mut css = String::new();

        // 生成 CSS 变量
        css.push_str(":root {\n");

        // 从设计令牌系统生成 CSS 变量
        let css_vars = token_system.export_as_css_variables()?;
        css.push_str(&css_vars);

        // 添加自定义变量
        for (key, value) in &current_theme.custom_variables {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        css.push_str("}\n");

        // 如果启用过渡动画，添加过渡样式
        if self.config.enable_transitions {
            css.push_str(&format!(
                "* {{ transition: all {}ms ease-in-out; }}\n",
                self.config.transition_duration
            ));
        }

        Ok(css)
    }

    /// 添加主题变更监听器
    pub fn add_listener(&self, listener: Box<dyn ThemeChangeListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }

    /// 移除所有监听器
    pub fn clear_listeners(&self) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.clear();
    }

    /// 创建主题作用域
    pub fn create_scope(&self, overrides: HashMap<String, String>) -> ThemeScope {
        ThemeScope {
            parent_context: Some(self.current_theme().name),
            scoped_theme: self.current_theme(),
            overrides,
        }
    }

    /// 创建主题构建器
    pub fn theme_builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }

    /// 获取设计令牌值
    pub fn get_token_value(&self, path: &str) -> Result<TokenValue, ThemeError> {
        let token_system = self.token_system.read().unwrap();
        token_system
            .get_token_value(path)
            .ok_or_else(|| ThemeError::TokenNotFound(path.to_string()))
    }

    /// 设置设计令牌值
    pub fn set_token_value(&self, path: &str, value: TokenValue) -> Result<(), ThemeError> {
        let mut token_system = self.token_system.write().unwrap();
        token_system.set_token(path, value)
    }

    // 私有辅助方法

    fn create_default_theme(name: &str, mode: ThemeMode) -> Theme {
        Theme {
            name: name.to_string(),
            tokens: DesignTokens::ant_design_default(),
            custom_variables: HashMap::new(),
            mode,
        }
    }

    fn emit_theme_change_event(&self, event: ThemeChangeEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_theme_change(&event);
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    /// 获取主题切换历史记录
    pub fn get_theme_history(&self) -> Result<Vec<String>, String> {
        self.theme_history.get_history()
    }

    /// 回到上一个主题
    pub fn go_back_theme(&self) -> Result<Option<()>, String> {
        if let Some(previous_theme) = self.theme_history.get_previous_theme()? {
            match self.switch_theme(&previous_theme) {
                Ok(_) => Ok(Some(())),
                Err(e) => Err(format!("Failed to switch to previous theme: {:?}", e)),
            }
        } else {
            Ok(None)
        }
    }

    /// 清空主题历史记录
    pub fn clear_theme_history(&self) -> Result<(), String> {
        self.theme_history.clear_history()
    }

    /// 设置最大历史记录数
    pub fn set_max_history(&mut self, max: usize) {
        // 创建新的ThemeHistory实例，保留现有历史记录
        let current_history = self.theme_history.get_history().unwrap_or_default();
        let mut new_theme_history = ThemeHistory::new().with_max_history(max);

        // 将现有历史记录添加到新的历史管理器中，但要遵守新的最大限制
        let start_index = if current_history.len() > max {
            current_history.len() - max
        } else {
            0
        };

        for theme_name in &current_history[start_index..] {
            let _ = new_theme_history.add_theme(theme_name);
        }

        self.theme_history = new_theme_history;
    }

    /// 获取主题历史管理器的引用
    pub fn theme_history(&self) -> &ThemeHistory {
        &self.theme_history
    }
}

impl ThemeBuilder {
    /// 设置主题名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置设计令牌
    pub fn design_tokens(mut self, tokens: DesignTokens) -> Self {
        self.design_tokens = Some(tokens);
        self
    }

    /// 添加自定义变量
    pub fn custom_variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_variables.insert(key.into(), value.into());
        self
    }

    /// 设置主题模式
    pub fn mode(mut self, mode: ThemeMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// 设置父主题（用于继承）
    pub fn parent_theme(mut self, parent: impl Into<String>) -> Self {
        self.parent_theme = Some(parent.into());
        self
    }

    /// 构建主题
    pub fn build(self) -> Result<Theme, ThemeError> {
        let name = self.name.ok_or(ThemeError::MissingThemeName)?;
        let design_tokens = self
            .design_tokens
            .unwrap_or_else(DesignTokens::ant_design_default);
        let mode = self.mode.unwrap_or(ThemeMode::Light);

        Ok(Theme {
            name,
            tokens: design_tokens,
            custom_variables: self.custom_variables,
            mode,
        })
    }
}

/// 主题系统错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeError {
    /// 主题未找到
    ThemeNotFound(String),
    /// 令牌未找到
    TokenNotFound(String),
    /// 缺少主题名称
    MissingThemeName,
    /// 令牌系统错误
    TokenSystemError(String),
    /// CSS 生成错误
    CssGenerationError(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::ThemeNotFound(name) => write!(f, "Theme not found: {}", name),
            ThemeError::TokenNotFound(path) => write!(f, "Token not found: {}", path),
            ThemeError::MissingThemeName => write!(f, "Theme name is required"),
            ThemeError::TokenSystemError(msg) => write!(f, "Token system error: {}", msg),
            ThemeError::CssGenerationError(msg) => write!(f, "CSS generation error: {}", msg),
        }
    }
}

impl std::error::Error for ThemeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::default();
        let current = manager.current_theme();
        assert_eq!(current.name, "ant-design");
        assert_eq!(current.mode, ThemeMode::Light);
    }

    #[test]
    fn test_theme_registration_and_switching() {
        let manager = ThemeManager::default();

        // 创建自定义主题
        let custom_theme = ThemeManager::theme_builder()
            .name("custom")
            .mode(ThemeMode::Dark)
            .build()
            .unwrap();

        // 注册主题
        manager.register_theme(custom_theme).unwrap();

        // 切换主题
        manager.switch_theme("custom").unwrap();

        let current = manager.current_theme();
        assert_eq!(current.name, "custom");
        assert_eq!(current.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_mode_switching() {
        let manager = ThemeManager::default();

        // 切换到深色模式
        manager.switch_mode(ThemeMode::Dark).unwrap();

        let current = manager.current_theme();
        assert_eq!(current.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_css_generation() {
        let manager = ThemeManager::default();
        let css = manager.generate_css().unwrap();

        assert!(css.contains(":root {"));
        assert!(css.contains("}"));
    }

    #[test]
    fn test_theme_builder() {
        let theme = ThemeManager::theme_builder()
            .name("test-theme")
            .mode(ThemeMode::Dark)
            .custom_variable("primary-color", "#1890ff")
            .build()
            .unwrap();

        assert_eq!(theme.name, "test-theme");
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(
            theme.custom_variables.get("primary-color"),
            Some(&"#1890ff".to_string())
        );
    }

    // #[test]
    // fn test_theme_builder() {
    //     let theme = ThemeBuilder::new()
    //         .name("Custom Theme")
    //         .mode("light")
    //         .variable("primary-color", "#ff0000")
    //         .variable("secondary-color", "#00ff00")
    //         .build();

    //     assert_eq!(theme.name, "Custom Theme");
    //     assert_eq!(theme.mode, ThemeMode::Light);
    //     assert_eq!(
    //         theme.custom_variables.get("primary-color"),
    //         Some(&"#ff0000".to_string())
    //     );
    //     assert_eq!(
    //         theme.custom_variables.get("secondary-color"),
    //         Some(&"#00ff00".to_string())
    //     );
    // }
}
