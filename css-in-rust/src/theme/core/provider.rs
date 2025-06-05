use super::manager::ThemeManager;
use crate::theme::theme_types::ThemeMode;
use crate::theme::Theme;
use std::sync::Arc;

/// 主题提供者配置
#[derive(Debug, Clone)]
pub struct ThemeProviderConfig {
    /// 是否自动检测系统主题
    pub auto_detect_system_theme: bool,
    /// 是否启用主题持久化
    pub enable_persistence: bool,
    /// 持久化存储键
    pub storage_key: String,
}

impl Default for ThemeProviderConfig {
    fn default() -> Self {
        Self {
            auto_detect_system_theme: true,
            enable_persistence: true,
            storage_key: "theme-preference".to_string(),
        }
    }
}

/// 主题提供者
pub struct ThemeProvider {
    /// 主题管理器
    manager: Arc<ThemeManager>,
    /// 配置
    config: ThemeProviderConfig,
}

impl ThemeProvider {
    /// 创建新的主题提供者
    pub fn new(manager: Arc<ThemeManager>, config: ThemeProviderConfig) -> Self {
        Self { manager, config }
    }

    /// 获取当前主题
    pub fn get_theme(&self) -> Option<Theme> {
        self.manager.get_current_theme()
    }

    /// 设置主题
    pub fn set_theme(&self, theme: Theme) {
        self.manager.set_theme(theme);
    }

    /// 切换主题模式
    pub fn toggle_theme_mode(&self) {
        self.manager.toggle_theme_mode();
    }

    /// 获取当前主题模式
    pub fn get_theme_mode(&self) -> Option<ThemeMode> {
        self.get_theme().map(|theme| theme.mode)
    }
}
