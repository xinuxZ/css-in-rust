//! 主题上下文模块
//!
//! 提供主题的全局状态管理和上下文传递功能。
//! 支持主题切换、主题继承和主题作用域管理。

use super::{Theme, ThemeMode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// 主题上下文内部状态
#[derive(Clone)]
struct ThemeContextInner {
    /// 当前主题
    current_theme: Theme,
    /// 注册的主题
    themes: HashMap<String, Theme>,
    /// 主题变更监听器
    listeners: Arc<RwLock<Vec<Box<dyn Fn(&Theme) + Send + Sync>>>>,
}

impl std::fmt::Debug for ThemeContextInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeContextInner")
            .field("current_theme", &self.current_theme)
            .field("themes", &self.themes)
            .field("listeners", &"<listeners>")
            .finish()
    }
}

/// 主题上下文管理器
///
/// 负责管理全局主题状态和主题切换逻辑
#[derive(Debug, Clone)]
pub struct ThemeContext {
    /// 内部状态
    inner: Arc<RwLock<ThemeContextInner>>,
}

/// 主题作用域
///
/// 为特定组件或区域提供局部主题覆盖
#[derive(Debug, Clone)]
pub struct ThemeScope {
    /// 父级主题上下文
    parent_context: Option<ThemeContext>,
    /// 当前作用域主题
    scoped_theme: Theme,
    /// 主题覆盖配置
    overrides: HashMap<String, String>,
}

/// 主题变更事件
#[derive(Debug, Clone)]
pub struct ThemeChangeEvent {
    /// 旧主题名称
    pub old_theme: String,
    /// 新主题名称
    pub new_theme: String,
    /// 变更时间戳
    pub timestamp: u64,
    /// 变更原因
    pub reason: ThemeChangeReason,
}

/// 主题变更原因
#[derive(Debug, Clone, PartialEq)]
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

/// 主题监听器类型
pub type ThemeListener = Box<dyn Fn(&Theme) + Send + Sync>;

impl ThemeContext {
    /// 创建新的主题上下文
    pub fn new() -> Self {
        let default_theme = Theme::ant_design();
        let mut themes = HashMap::new();
        themes.insert(default_theme.name.clone(), default_theme.clone());

        let inner = ThemeContextInner {
            current_theme: default_theme,
            themes,
            listeners: Arc::new(RwLock::new(Vec::new())),
        };

        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// 使用指定主题创建上下文
    pub fn with_theme(theme: Theme) -> Self {
        let mut themes = HashMap::new();
        themes.insert(theme.name.clone(), theme.clone());

        let inner = ThemeContextInner {
            current_theme: theme,
            themes,
            listeners: Arc::new(RwLock::new(Vec::new())),
        };

        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// 注册主题
    pub fn register_theme(&self, theme: Theme) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        if inner.themes.contains_key(&theme.name) {
            return Err(format!("Theme '{}' already exists", theme.name));
        }

        let theme_name = theme.name.clone();
        inner.themes.insert(theme_name, theme);
        Ok(())
    }

    /// 注册多个主题
    pub fn register_themes(&self, themes: Vec<Theme>) -> Result<(), String> {
        for theme in themes {
            self.register_theme(theme)?;
        }
        Ok(())
    }

    /// 获取当前主题
    pub fn current_theme(&self) -> Result<Theme, String> {
        let inner = self
            .inner
            .read()
            .map_err(|e| format!("Failed to read inner: {}", e))?;
        Ok(inner.current_theme.clone())
    }

    /// 获取主题列表
    pub fn available_themes(&self) -> Result<Vec<String>, String> {
        let inner = self
            .inner
            .read()
            .map_err(|e| format!("Failed to read inner: {}", e))?;
        Ok(inner.themes.keys().cloned().collect())
    }

    /// 切换到指定主题
    pub fn switch_theme(&self, theme_name: &str) -> Result<(), String> {
        self.switch_theme_with_reason(theme_name, ThemeChangeReason::Programmatic)
    }

    /// 带原因的主题切换
    pub fn switch_theme_with_reason(
        &self,
        theme_name: &str,
        reason: ThemeChangeReason,
    ) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        let new_theme = inner
            .themes
            .get(theme_name)
            .ok_or_else(|| format!("Theme '{}' not found", theme_name))?
            .clone();

        let old_theme_name = inner.current_theme.name.clone();

        // 更新当前主题
        inner.current_theme = new_theme.clone();

        // 释放写锁以避免死锁
        drop(inner);

        // 创建变更事件
        let _event = ThemeChangeEvent {
            old_theme: old_theme_name,
            new_theme: theme_name.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            reason,
        };

        // 通知监听器
        self.notify_listeners(&new_theme)?;

        Ok(())
    }

    /// 切换主题模式（亮色/暗色）
    pub fn switch_mode(&self, mode: ThemeMode) -> Result<(), String> {
        let current_theme = self.current_theme()?;

        let target_theme_name = match mode {
            ThemeMode::Light => {
                if current_theme.name.contains("dark") {
                    current_theme.name.replace("-dark", "")
                } else {
                    current_theme.name
                }
            }
            ThemeMode::Dark => {
                if current_theme.name.contains("dark") {
                    current_theme.name
                } else {
                    format!("{}-dark", current_theme.name)
                }
            }
            ThemeMode::Auto => {
                // 根据系统偏好自动选择
                self.detect_system_theme_preference()
            }
        };

        self.switch_theme_with_reason(&target_theme_name, ThemeChangeReason::SystemAuto)
    }

    /// 检测系统主题偏好
    fn detect_system_theme_preference(&self) -> String {
        // 在实际实现中，这里应该检测系统的暗色模式偏好
        // 目前返回默认主题
        "default".to_string()
    }

    /// 获取主题令牌值
    pub fn get_token(&self, path: &str) -> Result<Option<String>, String> {
        let theme = self.current_theme()?;
        Ok(theme.get_token(path))
    }

    /// 添加主题变更监听器
    pub fn add_listener<F>(&self, listener: F) -> Result<(), String>
    where
        F: Fn(&Theme) + Send + Sync + 'static,
    {
        let inner = self
            .inner
            .read()
            .map_err(|e| format!("Failed to read inner: {}", e))?;
        let mut listeners = inner
            .listeners
            .write()
            .map_err(|e| format!("Failed to write listeners: {}", e))?;
        listeners.push(Box::new(listener));
        Ok(())
    }

    /// 通知所有监听器
    fn notify_listeners(&self, theme: &Theme) -> Result<(), String> {
        let inner = self
            .inner
            .read()
            .map_err(|e| format!("Failed to read inner: {}", e))?;
        let listeners = inner
            .listeners
            .read()
            .map_err(|e| format!("Failed to read listeners: {}", e))?;

        for listener in listeners.iter() {
            listener(theme);
        }

        Ok(())
    }

    /// 创建主题作用域
    pub fn create_scope(&self, overrides: HashMap<String, String>) -> Result<ThemeScope, String> {
        let current_theme = self.current_theme()?;

        Ok(ThemeScope {
            parent_context: Some(self.clone()),
            scoped_theme: current_theme,
            overrides,
        })
    }

    /// 生成当前主题的 CSS 变量
    pub fn to_css_variables(&self) -> Result<String, String> {
        let theme = self.current_theme()?;
        Ok(theme.to_css_variables())
    }

    /// 预加载主题资源
    pub fn preload_themes(&self, theme_names: Vec<&str>) -> Result<(), String> {
        // 在实际实现中，这里可以预加载主题相关的 CSS 文件或资源
        // 目前只是验证主题是否存在
        let inner = self
            .inner
            .read()
            .map_err(|e| format!("Failed to read inner: {}", e))?;
        let themes = &inner.themes;

        for theme_name in theme_names {
            if !themes.contains_key(theme_name) {
                return Err(format!("Theme '{}' not found for preloading", theme_name));
            }
        }

        Ok(())
    }
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeScope {
    /// 创建新的主题作用域
    pub fn new(theme: Theme) -> Self {
        Self {
            parent_context: None,
            scoped_theme: theme,
            overrides: HashMap::new(),
        }
    }

    /// 添加主题覆盖
    pub fn with_override(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.overrides.insert(key.into(), value.into());
        self
    }

    /// 批量添加主题覆盖
    pub fn with_overrides(mut self, overrides: HashMap<String, String>) -> Self {
        self.overrides.extend(overrides);
        self
    }

    /// 获取作用域内的主题令牌值
    pub fn get_token(&self, path: &str) -> Option<String> {
        // 首先检查覆盖值
        if let Some(override_value) = self.overrides.get(path) {
            return Some(override_value.clone());
        }

        // 然后检查作用域主题
        self.scoped_theme.get_token(path)
    }

    /// 生成作用域 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = self.scoped_theme.to_css_variables();

        // 添加覆盖变量
        for (key, value) in &self.overrides {
            css.push_str(&format!("  --{}: {};\n", key, value));
        }

        css
    }

    /// 应用到父级上下文
    pub fn apply_to_context(&self, context: &ThemeContext) -> Result<(), String> {
        if let Some(_parent) = &self.parent_context {
            // 创建临时主题并应用到上下文
            let mut temp_theme = self.scoped_theme.clone();

            // 应用覆盖
            for (key, value) in &self.overrides {
                temp_theme
                    .custom_variables
                    .insert(key.clone(), value.clone());
            }

            context.register_theme(temp_theme.clone())?;
            context.switch_theme(&temp_theme.name)?;
        }

        Ok(())
    }
}

thread_local! {
    static GLOBAL_THEME_CONTEXT: RefCell<Option<ThemeContext>> = RefCell::new(None);
}

/// 主题上下文工具函数
pub struct ThemeContextUtils;

impl ThemeContextUtils {
    /// 初始化全局主题上下文
    pub fn init_global_context(context: ThemeContext) {
        GLOBAL_THEME_CONTEXT.with(|ctx| {
            *ctx.borrow_mut() = Some(context);
        });
    }

    /// 获取全局主题上下文
    pub fn get_global_context() -> Option<ThemeContext> {
        GLOBAL_THEME_CONTEXT.with(|ctx| ctx.borrow().clone())
    }

    /// 使用全局上下文执行操作
    pub fn with_global_context<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&ThemeContext) -> R,
    {
        Self::get_global_context().map(|ctx| f(&ctx))
    }

    /// 获取全局主题令牌
    pub fn get_global_token(path: &str) -> Option<String> {
        Self::with_global_context(|ctx| ctx.get_token(path).unwrap_or(None)).flatten()
    }

    /// 切换全局主题
    pub fn switch_global_theme(theme_name: &str) -> Result<(), String> {
        Self::get_global_context()
            .ok_or_else(|| "Global theme context not initialized".to_string())?
            .switch_theme(theme_name)
    }

    /// 切换全局主题模式
    pub fn switch_global_mode(mode: ThemeMode) -> Result<(), String> {
        Self::get_global_context()
            .ok_or_else(|| "Global theme context not initialized".to_string())?
            .switch_mode(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_context_creation() {
        let context = ThemeContext::new();
        let current = context.current_theme().unwrap();
        assert_eq!(current.name, "ant-design");
    }

    #[test]
    fn test_theme_registration() {
        let context = ThemeContext::new();
        let custom_theme = Theme::new("custom").with_mode(ThemeMode::Dark);

        assert!(context.register_theme(custom_theme).is_ok());

        let themes = context.available_themes().unwrap();
        assert!(themes.contains(&"custom".to_string()));
    }

    #[test]
    fn test_theme_switching() {
        let context = ThemeContext::new();
        let dark_theme = Theme::ant_design_dark();

        context.register_theme(dark_theme).unwrap();
        assert!(context.switch_theme("ant-design-dark").is_ok());

        let current = context.current_theme().unwrap();
        assert_eq!(current.name, "ant-design-dark");
    }

    #[test]
    fn test_theme_scope() {
        let context = ThemeContext::new();
        let mut overrides = HashMap::new();
        overrides.insert("primary-color".to_string(), "#ff0000".to_string());

        let scope = context.create_scope(overrides).unwrap();
        assert_eq!(
            scope.get_token("primary-color"),
            Some("#ff0000".to_string())
        );
    }

    #[test]
    fn test_token_access() {
        let context = ThemeContext::new();
        let token = context.get_token("colors.primary").unwrap();
        assert!(token.is_some());
    }

    #[test]
    fn test_mode_switching() {
        let context = ThemeContext::new();
        context.register_theme(Theme::ant_design_dark()).unwrap();

        assert!(context.switch_mode(ThemeMode::Dark).is_ok());

        let current = context.current_theme().unwrap();
        assert_eq!(current.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_global_context_utils() {
        let context = ThemeContext::new();
        ThemeContextUtils::init_global_context(context);

        let global_context = ThemeContextUtils::get_global_context();
        assert!(global_context.is_some());

        let token = ThemeContextUtils::get_global_token("colors.primary");
        assert!(token.is_some());
    }
}
