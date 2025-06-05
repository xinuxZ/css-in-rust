use crate::theme::theme_types::ThemeMode;
use crate::theme::Theme;
use std::sync::{Arc, RwLock};

/// 主题管理器配置
#[derive(Debug, Clone)]
pub struct ThemeManagerConfig {
    /// 默认主题名称
    pub default_theme: String,
    /// 是否启用主题历史记录
    pub enable_history: bool,
    /// 是否启用主题变更事件
    pub enable_events: bool,
}

impl Default for ThemeManagerConfig {
    fn default() -> Self {
        Self {
            default_theme: "default".to_string(),
            enable_history: true,
            enable_events: true,
        }
    }
}

/// 主题管理器
#[derive(Default)]
pub struct ThemeManager {
    /// 当前主题
    current_theme: Arc<RwLock<Theme>>,
    /// 配置
    config: ThemeManagerConfig,
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(config: ThemeManagerConfig) -> Self {
        Self {
            current_theme: Arc::new(RwLock::new(Theme::default())),
            config,
        }
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.current_theme.read().ok().map(|theme| theme.clone())
    }

    /// 设置当前主题
    pub fn set_theme(&self, theme: Theme) {
        if let Ok(mut current) = self.current_theme.write() {
            *current = theme;
        }
    }

    /// 切换主题模式
    pub fn toggle_theme_mode(&self) {
        if let Ok(mut theme) = self.current_theme.write() {
            theme.mode = match theme.mode {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Auto => ThemeMode::Light,
            };
        }
    }
}
