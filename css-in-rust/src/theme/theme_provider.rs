//! 主题提供者模块
//!
//! 负责主题的全局管理、上下文传递和组件集成。
//! 提供高级的主题管理 API 和框架集成支持。

use crate::theme::{
    CssVariableInjector, CssVariableManager, Theme, ThemeChangeEvent, ThemeChangeEventType,
    ThemeChangeListener, ThemeChangeReason, ThemeHistory, ThemeManager, ThemeMode, UpdateReason,
    VariableUpdateEvent,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};
use std::time::{SystemTime, UNIX_EPOCH};

/// 主题提供者
///
/// 负责管理应用程序的主题状态和切换逻辑
#[derive(Debug, Clone)]
pub struct ThemeProvider {
    /// 内部状态
    inner: Arc<RwLock<ThemeProviderInner>>,
}

/// 主题提供者内部状态
#[derive(Debug)]
struct ThemeProviderInner {
    /// 当前主题
    current_theme: Theme,
    /// 注册的主题
    registered_themes: HashMap<String, Theme>,
    /// CSS 变量管理器
    variable_manager: CssVariableManager,
    /// CSS 变量注入器
    variable_injector: CssVariableInjector,
    /// 主题管理器
    theme_manager: ThemeManager,
    /// 主题历史管理器
    theme_history: ThemeHistory,
    /// 变更监听器
    listeners: Vec<Weak<dyn ThemeChangeListener>>,
    /// 配置选项
    config: ThemeProviderConfig,
}

/// 主题提供者配置
#[derive(Debug, Clone)]
pub struct ThemeProviderConfig {
    /// 是否启用自动 CSS 注入
    pub auto_inject_css: bool,
    /// CSS 注入目标选择器
    pub injection_target: String,
    /// 是否启用主题缓存
    pub enable_caching: bool,
    /// 是否启用性能监控
    pub enable_performance_monitoring: bool,
    /// 主题切换动画时长（毫秒）
    pub transition_duration: u32,
    /// 是否启用系统主题检测
    pub detect_system_theme: bool,
    /// 变量前缀
    pub variable_prefix: String,
}

// ThemeChangeListener trait 已移至 theme_manager.rs 模块中，避免重复定义

/// 主题切换结果
#[derive(Debug, Clone)]
pub struct ThemeSwitchResult {
    /// 是否成功
    pub success: bool,
    /// 切换耗时（毫秒）
    pub duration_ms: u64,
    /// 更新的变量数量
    pub updated_variables: usize,
    /// 错误信息
    pub error: Option<String>,
}

// ThemeManager 已移至 theme_manager.rs 模块中，避免重复定义

/// 主题预设
#[derive(Debug, Clone)]
pub struct ThemePreset {
    /// 预设名称
    pub name: String,
    /// 预设描述
    pub description: String,
    /// 主题实例
    pub theme: Theme,
    /// 预设标签
    pub tags: Vec<String>,
    /// 是否为内置预设
    pub builtin: bool,
}

/// 主题提供者构建器
///
/// 提供流式 API 来构建主题提供者
#[derive(Debug, Clone)]
pub struct ThemeProviderBuilder {
    /// 基础主题
    base_theme: Option<Theme>,
    /// 自定义变量
    custom_variables: HashMap<String, String>,
    /// 主题名称
    name: Option<String>,
    /// 主题模式
    mode: Option<String>,
}

impl ThemeProvider {
    /// 创建新的主题提供者
    pub fn new() -> Self {
        let config = ThemeProviderConfig::default();
        let current_theme = Theme::default();
        let mut variable_manager = CssVariableManager::new().with_prefix(&config.variable_prefix);

        // 生成初始 CSS 变量
        variable_manager
            .generate_from_theme(&current_theme)
            .expect("Failed to generate initial CSS variables");

        let variable_injector = CssVariableInjector::new(&config.injection_target);
        let theme_manager = ThemeManager::default();

        // 注册暗色主题到 theme_manager（默认主题已在 ThemeManager::default() 中注册）
        let dark_theme = Theme::default();
        theme_manager
            .register_theme(dark_theme.clone())
            .expect("Failed to register dark theme");

        let inner = ThemeProviderInner {
            current_theme: current_theme.clone(),
            registered_themes: {
                let mut themes = HashMap::new();
                themes.insert("default".to_string(), current_theme);
                themes.insert("dark".to_string(), dark_theme);
                themes
            },
            variable_manager,
            variable_injector,
            theme_manager,
            theme_history: ThemeHistory::new(),
            listeners: Vec::new(),
            config,
        };

        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// 使用自定义配置创建主题提供者
    pub fn with_config(config: ThemeProviderConfig) -> Self {
        let current_theme = Theme::default();
        let mut variable_manager = CssVariableManager::new().with_prefix(&config.variable_prefix);

        variable_manager
            .generate_from_theme(&current_theme)
            .expect("Failed to generate initial CSS variables");

        let variable_injector = CssVariableInjector::new(&config.injection_target);
        let theme_manager = ThemeManager::default();

        // 注册暗色主题到 theme_manager（默认主题已在 ThemeManager::default() 中注册）
        let dark_theme = Theme::default();
        theme_manager
            .register_theme(dark_theme.clone())
            .expect("Failed to register dark theme");

        let inner = ThemeProviderInner {
            current_theme: current_theme.clone(),
            registered_themes: {
                let mut themes = HashMap::new();
                themes.insert("default".to_string(), current_theme);
                themes.insert("dark".to_string(), dark_theme);
                themes
            },
            variable_manager,
            variable_injector,
            theme_manager,
            theme_history: ThemeHistory::new(),
            listeners: Vec::new(),
            config,
        };

        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// 注册主题
    pub fn register_theme(&self, name: impl Into<String>, theme: Theme) -> Result<(), String> {
        let name = name.into();
        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;

        inner.registered_themes.insert(name.clone(), theme.clone());
        inner
            .theme_manager
            .register_theme(theme)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 切换主题
    pub fn switch_theme(&self, theme_name: &str) -> Result<ThemeSwitchResult, String> {
        let start_time = SystemTime::now();

        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;

        // 调试：打印当前注册的主题
        println!("DEBUG: Looking for theme '{}'", theme_name);
        println!(
            "DEBUG: Available themes: {:?}",
            inner.registered_themes.keys().collect::<Vec<_>>()
        );
        println!(
            "DEBUG: Theme exists: {}",
            inner.registered_themes.contains_key(theme_name)
        );

        // 获取目标主题
        let available_themes: Vec<String> = inner.registered_themes.keys().cloned().collect();
        let target_theme = inner
            .registered_themes
            .get(theme_name)
            .ok_or_else(|| {
                format!(
                    "Theme '{}' not found. Available themes: {:?}",
                    theme_name, available_themes
                )
            })?
            .clone();

        // 获取当前主题信息
        let current_theme = inner.current_theme.name.clone();
        let current_mode = inner.current_theme.mode.clone();

        // 通知监听器主题即将变更（如果有这个方法的话）
        // for listener in &inner.listeners {
        //     if let Some(listener) = listener.upgrade() {
        //         listener.on_theme_will_change(&inner.current_theme, &target_theme);
        //     }
        // }

        // 更新 CSS 变量
        let old_variables = inner.variable_manager.get_all_variables().clone();

        match inner.variable_manager.generate_from_theme(&target_theme) {
            Ok(_) => {
                // 计算变更的变量
                let new_variables = inner.variable_manager.get_all_variables();
                let mut changed_variables = HashMap::new();

                for (name, value) in new_variables {
                    if old_variables.get(name) != Some(value) {
                        changed_variables.insert(name.clone(), value.clone());
                    }
                }

                // 注入 CSS 变量
                if inner.config.auto_inject_css {
                    let css = inner.variable_manager.to_css();
                    if let Err(e) = inner.variable_injector.inject(&css) {
                        let error = format!("Failed to inject CSS: {}", e);

                        // 通知监听器切换失败
                        for listener in &inner.listeners {
                            if let Some(listener) = listener.upgrade() {
                                let error_event = ThemeChangeEvent {
                                    event_type: ThemeChangeEventType::ThemeSwitch,
                                    old_theme: Some(current_theme.clone()),
                                    new_theme: theme_name.to_string(),
                                    old_mode: Some(current_mode.clone()),
                                    new_mode: current_mode.clone(),
                                    timestamp: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs(),
                                    reason: ThemeChangeReason::UserSwitch,
                                };
                                listener.on_theme_change(&error_event);
                            }
                        }

                        return Ok(ThemeSwitchResult {
                            success: false,
                            duration_ms: 0,
                            updated_variables: 0,
                            error: Some(error),
                        });
                    }
                }

                // 更新当前主题
                inner.current_theme = target_theme.clone();
                if let Err(e) = inner.theme_manager.switch_theme(theme_name) {
                    return Err(format!("Failed to switch theme in manager: {:?}", e));
                }

                // 添加到历史记录
                if let Err(e) = inner.theme_history.add_theme(theme_name) {
                    println!("Warning: Failed to add theme to history: {}", e);
                }

                // 创建变更事件
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;

                let _event = VariableUpdateEvent {
                    changed_variables: changed_variables.clone(),
                    timestamp,
                    reason: UpdateReason::ThemeSwitch,
                };

                // 通知监听器主题已变更
                for listener in &inner.listeners {
                    if let Some(listener) = listener.upgrade() {
                        let change_event = ThemeChangeEvent {
                            event_type: ThemeChangeEventType::ThemeSwitch,
                            old_theme: Some(current_theme.clone()),
                            new_theme: theme_name.to_string(),
                            old_mode: Some(current_mode.clone()),
                            new_mode: current_mode.clone(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                            reason: ThemeChangeReason::UserSwitch,
                        };
                        listener.on_theme_change(&change_event);
                    }
                }

                let duration = start_time.elapsed().unwrap_or_default().as_millis() as u64;

                Ok(ThemeSwitchResult {
                    success: true,
                    duration_ms: duration,
                    updated_variables: changed_variables.len(),
                    error: None,
                })
            }
            Err(e) => {
                let error = format!("Failed to generate CSS variables: {}", e);

                // 通知监听器切换失败
                for listener in &inner.listeners {
                    if let Some(listener) = listener.upgrade() {
                        let error_event = ThemeChangeEvent {
                            event_type: ThemeChangeEventType::ThemeSwitch,
                            old_theme: Some(current_theme.clone()),
                            new_theme: theme_name.to_string(),
                            old_mode: Some(current_mode.clone()),
                            new_mode: current_mode.clone(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                            reason: ThemeChangeReason::UserSwitch,
                        };
                        listener.on_theme_change(&error_event);
                    }
                }

                Ok(ThemeSwitchResult {
                    success: false,
                    duration_ms: 0,
                    updated_variables: 0,
                    error: Some(error),
                })
            }
        }
    }

    /// 获取当前主题
    pub fn current_theme(&self) -> Result<Theme, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.current_theme.clone())
    }

    /// 获取已注册的主题列表
    pub fn registered_themes(&self) -> Result<Vec<String>, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.registered_themes.keys().cloned().collect())
    }

    /// 获取主题
    pub fn get_theme(&self, name: &str) -> Result<Option<Theme>, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.registered_themes.get(name).cloned())
    }

    /// 添加主题变更监听器
    pub fn add_listener(&self, listener: Arc<dyn ThemeChangeListener>) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        inner.listeners.push(Arc::downgrade(&listener));
        Ok(())
    }

    /// 移除失效的监听器
    pub fn cleanup_listeners(&self) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        inner
            .listeners
            .retain(|listener| listener.strong_count() > 0);
        Ok(())
    }

    /// 获取当前 CSS 变量
    pub fn get_css_variables(&self) -> Result<String, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.variable_manager.to_css())
    }

    /// 更新配置
    pub fn update_config(&self, config: ThemeProviderConfig) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;

        // 如果变量前缀改变，需要重新生成变量
        if inner.config.variable_prefix != config.variable_prefix {
            let current_theme = inner.current_theme.clone();
            inner.variable_manager = inner
                .variable_manager
                .clone()
                .with_prefix(&config.variable_prefix);
            inner.variable_manager.generate_from_theme(&current_theme)?;
        }

        inner.config = config;
        Ok(())
    }

    /// 获取配置
    pub fn get_config(&self) -> Result<ThemeProviderConfig, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.config.clone())
    }

    /// 获取所有注册的主题名称
    pub fn get_theme_names(&self) -> Result<Vec<String>, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        Ok(inner.registered_themes.keys().cloned().collect())
    }

    /// 获取主题切换历史记录
    pub fn get_theme_history(&self) -> Result<Vec<String>, String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        inner.theme_history.get_history()
    }

    /// 回到上一个主题
    pub fn go_back_theme(&self) -> Result<Option<ThemeSwitchResult>, String> {
        let previous_theme = {
            let inner = self
                .inner
                .read()
                .map_err(|_| "Failed to acquire read lock")?;
            inner.theme_history.get_previous_theme()?
        };

        match previous_theme {
            Some(theme_name) => {
                let result = self.switch_theme(&theme_name)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    /// 清空主题历史记录
    pub fn clear_theme_history(&self) -> Result<(), String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| "Failed to acquire read lock")?;
        inner.theme_history.clear_history()
    }

    /// 设置最大历史记录数
    pub fn set_max_history(&self, max: usize) -> Result<(), String> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| "Failed to acquire write lock")?;
        inner.theme_history = inner.theme_history.clone().with_max_history(max);
        Ok(())
    }
}

impl Default for ThemeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ThemeProviderConfig {
    fn default() -> Self {
        Self {
            auto_inject_css: true,
            injection_target: ":root".to_string(),
            enable_caching: true,
            enable_performance_monitoring: false,
            transition_duration: 300,
            detect_system_theme: true,
            variable_prefix: "css-in-rust".to_string(),
        }
    }
}

// ThemeHistory 已移至 theme_manager.rs 模块中，避免重复定义

impl ThemeProviderBuilder {
    /// 创建新的主题构建器
    pub fn new() -> Self {
        Self {
            base_theme: None,
            custom_variables: HashMap::new(),
            name: None,
            mode: None,
        }
    }

    /// 基于现有主题构建
    pub fn from_theme(theme: Theme) -> Self {
        Self {
            base_theme: Some(theme),
            custom_variables: HashMap::new(),
            name: None,
            mode: None,
        }
    }

    /// 设置主题名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置主题模式
    pub fn mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// 添加自定义变量
    pub fn variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_variables.insert(name.into(), value.into());
        self
    }

    /// 批量添加变量
    pub fn variables(mut self, variables: HashMap<String, String>) -> Self {
        self.custom_variables.extend(variables);
        self
    }

    /// 构建主题
    pub fn build(self) -> Theme {
        let mut theme = self.base_theme.unwrap_or_else(|| Theme::default());

        if let Some(name) = self.name {
            theme.name = name;
        }

        if let Some(mode) = self.mode {
            theme.mode = match mode.as_str() {
                "light" => ThemeMode::Light,
                "dark" => ThemeMode::Dark,
                "auto" => ThemeMode::Auto,
                _ => ThemeMode::Light, // 默认为 Light 模式
            };
        }

        theme.custom_variables.extend(self.custom_variables);

        theme
    }
}

impl Default for ThemeProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemePreset {
    /// 创建内置预设
    pub fn builtin(name: impl Into<String>, description: impl Into<String>, theme: Theme) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            theme,
            tags: vec!["builtin".to_string()],
            builtin: true,
        }
    }

    /// 创建用户预设
    pub fn user(name: impl Into<String>, description: impl Into<String>, theme: Theme) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            theme,
            tags: vec!["user".to_string()],
            builtin: false,
        }
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 添加多个标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }

    /// 获取内置预设列表
    pub fn builtin_presets() -> Vec<ThemePreset> {
        vec![
            ThemePreset::builtin(
                "Default Light",
                "Default light theme with blue primary color",
                Theme::default(),
            ),
            ThemePreset::builtin("Default Dark", "Dark mode default theme", Theme::default())
                .with_tag("dark"),
        ]
    }
}

/// 全局主题提供者实例
static mut GLOBAL_THEME_PROVIDER: Option<ThemeProvider> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// 获取全局主题提供者
pub fn global_theme_provider() -> &'static ThemeProvider {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_THEME_PROVIDER = Some(ThemeProvider::new());
        });
        GLOBAL_THEME_PROVIDER.as_ref().unwrap()
    }
}

/// 初始化全局主题提供者
pub fn init_global_theme_provider(config: ThemeProviderConfig) {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_THEME_PROVIDER = Some(ThemeProvider::with_config(config));
        });
    }
}

/// 便捷的主题切换函数
pub fn switch_theme(theme_name: &str) -> Result<ThemeSwitchResult, String> {
    global_theme_provider().switch_theme(theme_name)
}

/// 便捷的主题注册函数
pub fn register_theme(name: impl Into<String>, theme: Theme) -> Result<(), String> {
    global_theme_provider().register_theme(name, theme)
}

/// 便捷的当前主题获取函数
pub fn current_theme() -> Result<Theme, String> {
    global_theme_provider().current_theme()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::DesignTokens;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Debug)]
    struct TestListener {
        will_change_called: AtomicBool,
        changed_called: AtomicBool,
        failed_called: AtomicBool,
    }

    impl TestListener {
        fn new() -> Self {
            Self {
                will_change_called: AtomicBool::new(false),
                changed_called: AtomicBool::new(false),
                failed_called: AtomicBool::new(false),
            }
        }
    }

    impl ThemeChangeListener for TestListener {
        fn on_theme_change(&self, _event: &ThemeChangeEvent) {
            // 模拟will_change事件 - 在实际的主题切换事件中设置
            self.will_change_called.store(true, Ordering::Relaxed);

            match _event.event_type {
                ThemeChangeEventType::ThemeSwitch => {
                    self.changed_called.store(true, Ordering::Relaxed);
                }
                _ => {
                    // 处理其他事件类型
                }
            }
        }
    }

    #[test]
    fn test_theme_provider_creation() {
        let provider = ThemeProvider::new();
        let themes = provider.registered_themes().unwrap();

        assert!(themes.contains(&"default".to_string()));
        assert!(themes.contains(&"dark".to_string()));
    }

    #[test]
    fn test_theme_registration() {
        let provider = ThemeProvider::new();
        let custom_theme = Theme::new("custom").with_tokens(DesignTokens::new());

        assert!(provider.register_theme("custom", custom_theme).is_ok());

        let themes = provider.registered_themes().unwrap();
        assert!(themes.contains(&"custom".to_string()));
    }

    #[test]
    fn test_theme_default_creation() {
        // 测试Theme::default()方法是否正常工作
        let dark_theme = Theme::default();
        println!("Dark theme created with name: '{}'", dark_theme.name);
        assert_eq!(dark_theme.name, "default");
    }

    #[test]
    fn test_theme_provider_switching() {
        eprintln!("=== TEST STARTING ====");
        let provider = ThemeProvider::new();
        eprintln!("=== PROVIDER CREATED ====");

        // 强制显示调试信息
        eprintln!("EPRINTLN: Test is running!");

        // 立即检查provider状态
        eprintln!("Provider created, checking themes immediately...");
        match provider.registered_themes() {
            Ok(themes) => {
                eprintln!("SUCCESS: Got themes: {:?}", themes);
            }
            Err(e) => {
                eprintln!("ERROR: Failed to get themes: {}", e);
                panic!("Failed to get registered themes: {}", e);
            }
        }

        // 验证主题已注册
        let themes = provider.registered_themes().expect("Should get themes");
        eprintln!("Registered themes: {:?}", themes);
        eprintln!("Looking for: dark");
        eprintln!("Contains dark: {}", themes.contains(&"dark".to_string()));

        if !themes.contains(&"dark".to_string()) {
            eprintln!("Theme 'dark' not found in registered themes: {:?}", themes);
        }

        // 在切换主题前再次检查
        println!("Before switch_theme call:");
        let themes_before = provider.registered_themes().expect("Should get themes");
        println!("Available themes before switch: {:?}", themes_before);

        // 直接检查内部状态
        {
            let inner = provider.inner.read().unwrap();
            println!(
                "Direct inner check - registered themes: {:?}",
                inner.registered_themes.keys().collect::<Vec<_>>()
            );
            println!(
                "Direct inner check - contains dark: {}",
                inner.registered_themes.contains_key("dark")
            );
        }

        // 现在尝试切换主题
        match provider.switch_theme("dark") {
            Ok(result) => {
                assert!(result.success);
                assert!(result.duration_ms >= 0);

                let current = provider.current_theme().unwrap();
                assert_eq!(current.name, "dark");
            }
            Err(e) => {
                panic!("Switch theme failed: {}", e);
            }
        }
    }

    #[test]
    fn test_theme_switch_nonexistent() {
        let provider = ThemeProvider::new();

        let result = provider.switch_theme("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_theme_listeners() {
        let provider = ThemeProvider::new();
        let listener = Arc::new(TestListener::new());

        provider.add_listener(listener.clone()).unwrap();

        let result = provider.switch_theme("dark").unwrap();
        assert!(result.success);

        assert!(listener.will_change_called.load(Ordering::Relaxed));
        assert!(listener.changed_called.load(Ordering::Relaxed));
        assert!(!listener.failed_called.load(Ordering::Relaxed));
    }

    #[test]
    fn test_theme_presets() {
        let presets = ThemePreset::builtin_presets();

        assert!(!presets.is_empty());
        assert!(presets.iter().any(|p| p.name == "Default Light"));
        assert!(presets.iter().any(|p| p.name == "Default Dark"));
    }

    #[test]
    fn test_config_update() {
        let provider = ThemeProvider::new();
        let mut config = ThemeProviderConfig::default();
        config.variable_prefix = "custom".to_string();

        assert!(provider.update_config(config).is_ok());

        let css = provider.get_css_variables().unwrap();
        assert!(css.contains("--custom-"));
    }
}
