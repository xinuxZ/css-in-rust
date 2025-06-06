use super::injector::DioxusStyleInjector;
use crate::theme::{
    core::manager::ThemeManager,
    theme_types::{Theme, ThemeMode},
};
use std::sync::{Arc, Mutex};

/// Dioxus主题提供者
///
/// 为Dioxus应用提供主题支持
#[derive(Debug, Clone)]
pub struct DioxusThemeProvider {
    /// 主题管理器
    manager: Arc<ThemeManager>,
    /// 样式注入器
    injector: Arc<Mutex<DioxusStyleInjector>>,
    /// 是否自动检测系统主题
    auto_detect_system_theme: bool,
    /// 是否启用主题持久化
    enable_persistence: bool,
}

/// 主题切换结果
#[derive(Debug)]
pub struct ThemeSwitchResult {
    /// 是否成功
    pub success: bool,
    /// 切换耗时（毫秒）
    pub duration_ms: u64,
    /// 错误信息
    pub error: Option<String>,
}

impl DioxusThemeProvider {
    /// 创建新的主题提供者
    pub fn new(manager: Arc<ThemeManager>) -> Self {
        Self {
            manager,
            injector: Arc::new(Mutex::new(DioxusStyleInjector::new())),
            auto_detect_system_theme: true,
            enable_persistence: true,
        }
    }

    /// 设置是否自动检测系统主题
    pub fn with_auto_detect_system_theme(mut self, value: bool) -> Self {
        self.auto_detect_system_theme = value;
        self
    }

    /// 设置是否启用主题持久化
    pub fn with_persistence(mut self, value: bool) -> Self {
        self.enable_persistence = value;
        self
    }

    /// 设置样式注入器
    pub fn with_injector(mut self, injector: DioxusStyleInjector) -> Self {
        self.injector = Arc::new(Mutex::new(injector));
        self
    }

    /// 获取当前主题
    pub fn get_theme(&self) -> Option<Theme> {
        self.manager.get_current_theme()
    }

    /// 切换主题
    pub fn switch_theme(&self, theme_name: &str) -> Result<ThemeSwitchResult, String> {
        use std::time::Instant;

        let start = Instant::now();

        // 切换主题
        match self.manager.switch_theme(theme_name) {
            Ok(_) => {
                // 获取新主题
                if let Some(theme) = self.manager.get_current_theme() {
                    // 注入主题CSS变量
                    let css = theme.to_css_variables();
                    let id = format!("theme-{}", theme_name);

                    // 注入样式
                    if let Ok(mut injector) = self.injector.lock() {
                        injector.inject_style(&css, &id);
                    }

                    // 如果启用了持久化，保存主题选择
                    #[cfg(target_arch = "wasm32")]
                    if self.enable_persistence {
                        self.save_theme_preference(theme_name);
                    }

                    let duration = start.elapsed().as_millis() as u64;

                    Ok(ThemeSwitchResult {
                        success: true,
                        duration_ms: duration,
                        error: None,
                    })
                } else {
                    Err("无法获取切换后的主题".to_string())
                }
            }
            Err(e) => {
                let duration = start.elapsed().as_millis() as u64;

                Ok(ThemeSwitchResult {
                    success: false,
                    duration_ms: duration,
                    error: Some(e.to_string()),
                })
            }
        }
    }

    /// 切换主题模式
    pub fn toggle_theme_mode(&self) -> Result<ThemeSwitchResult, String> {
        use std::time::Instant;

        let start = Instant::now();

        // 获取当前主题
        if let Some(current_theme) = self.manager.get_current_theme() {
            // 确定目标模式
            let target_mode = match current_theme.mode {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
                _ => ThemeMode::Light,
            };

            // 查找对应模式的主题
            if let Some(theme_name) = self.manager.find_theme_by_mode(target_mode) {
                return self.switch_theme(&theme_name);
            } else {
                return Err("未找到对应模式的主题".to_string());
            }
        }

        Err("无法获取当前主题".to_string())
    }

    /// 初始化主题
    pub fn initialize(&self) -> Result<(), String> {
        // 如果启用了自动检测系统主题
        #[cfg(target_arch = "wasm32")]
        if self.auto_detect_system_theme {
            self.detect_system_theme();
        }

        // 如果启用了持久化，尝试加载保存的主题
        #[cfg(target_arch = "wasm32")]
        if self.enable_persistence {
            if let Some(theme_name) = self.load_theme_preference() {
                let _ = self.switch_theme(&theme_name);
                return Ok(());
            }
        }

        // 默认使用第一个可用主题
        if let Some(theme_name) = self.manager.get_available_themes().first() {
            let _ = self.switch_theme(theme_name);
        }

        Ok(())
    }

    /// 检测系统主题模式
    #[cfg(target_arch = "wasm32")]
    fn detect_system_theme(&self) {
        use web_sys::window;

        if let Some(window) = window() {
            if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
                if let Some(media_query) = media_query {
                    if media_query.matches() {
                        // 系统使用暗色模式
                        if let Some(theme_name) = self.manager.find_theme_by_mode(ThemeMode::Dark) {
                            let _ = self.switch_theme(&theme_name);
                        }
                    } else {
                        // 系统使用亮色模式
                        if let Some(theme_name) = self.manager.find_theme_by_mode(ThemeMode::Light)
                        {
                            let _ = self.switch_theme(&theme_name);
                        }
                    }
                }
            }
        }
    }

    /// 保存主题偏好
    #[cfg(target_arch = "wasm32")]
    fn save_theme_preference(&self, theme_name: &str) {
        use web_sys::window;

        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("css-in-rust-theme", theme_name);
            }
        }
    }

    /// 加载主题偏好
    #[cfg(target_arch = "wasm32")]
    fn load_theme_preference(&self) -> Option<String> {
        use web_sys::window;

        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme_name)) = storage.get_item("css-in-rust-theme") {
                    return Some(theme_name);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::core::manager::{ThemeManager, ThemeManagerConfig};

    #[test]
    fn test_theme_provider_basic() {
        // 创建主题管理器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));

        // 创建测试主题
        let mut light_theme = Theme::default();
        light_theme.name = "light".to_string();
        light_theme.mode = ThemeMode::Light;

        let mut dark_theme = Theme::default();
        dark_theme.name = "dark".to_string();
        dark_theme.mode = ThemeMode::Dark;

        // 注册主题
        manager.register_theme(light_theme).unwrap();
        manager.register_theme(dark_theme).unwrap();

        // 创建主题提供者
        let provider = DioxusThemeProvider::new(manager);

        // 测试切换主题
        let result = provider.switch_theme("light");
        assert!(result.is_ok());
        let switch_result = result.unwrap();
        assert!(switch_result.success);

        // 测试获取主题
        let theme = provider.get_theme().unwrap();
        assert_eq!(theme.name, "light");
        assert_eq!(theme.mode, ThemeMode::Light);

        // 测试切换主题模式
        let result = provider.toggle_theme_mode();
        assert!(result.is_ok());
        let theme = provider.get_theme().unwrap();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.mode, ThemeMode::Dark);
    }
}
