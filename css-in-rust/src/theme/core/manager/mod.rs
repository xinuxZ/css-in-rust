pub mod theme_history;

use crate::theme::theme_types::ThemeMode;
use crate::theme::Theme;
use std::sync::{Arc, RwLock};
pub use theme_history::ThemeHistory;

/// 主题管理器配置
#[derive(Debug, Clone)]
pub struct ThemeManagerConfig {
    /// 默认主题名称
    pub default_theme: String,
    /// 是否启用主题历史记录
    pub enable_history: bool,
    /// 是否启用主题变更事件
    pub enable_events: bool,
    /// 是否启用主题持久化
    pub enable_persistence: bool,
    /// 持久化存储键
    pub storage_key: String,
}

impl Default for ThemeManagerConfig {
    fn default() -> Self {
        Self {
            default_theme: "default".to_string(),
            enable_history: true,
            enable_events: true,
            enable_persistence: false,
            storage_key: "theme-preference".to_string(),
        }
    }
}

/// 主题管理器
#[derive(Debug, Default)]
pub struct ThemeManager {
    /// 当前主题
    current_theme: Arc<RwLock<Theme>>,
    /// 配置
    config: ThemeManagerConfig,
    /// 主题历史
    theme_history: ThemeHistory,
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(config: ThemeManagerConfig) -> Self {
        Self {
            current_theme: Arc::new(RwLock::new(Theme::default())),
            config,
            theme_history: ThemeHistory::new(),
        }
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.current_theme.read().ok().map(|theme| theme.clone())
    }

    /// 设置当前主题
    pub fn set_theme(&self, theme: Theme) -> Result<(), String> {
        if let Ok(mut current) = self.current_theme.write() {
            // 如果启用了历史记录，添加到历史
            if self.config.enable_history {
                self.theme_history.add_theme(&theme.name)?;
            }

            *current = theme;
            Ok(())
        } else {
            Err("无法获取当前主题写锁".to_string())
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

    /// 获取主题历史
    pub fn get_theme_history(&self) -> &ThemeHistory {
        &self.theme_history
    }

    /// 返回到上一个主题
    pub fn go_back_theme(&self) -> Result<Option<()>, String> {
        if let Some(prev_theme_name) = self.theme_history.get_previous_theme()? {
            if let Ok(mut current) = self.current_theme.write() {
                current.name = prev_theme_name;
                Ok(Some(()))
            } else {
                Err("无法获取当前主题写锁".to_string())
            }
        } else {
            Ok(None)
        }
    }

    /// 前进到下一个主题
    pub fn go_forward_theme(&self) -> Result<Option<()>, String> {
        if let Some(next_theme_name) = self.theme_history.get_next_theme()? {
            if let Ok(mut current) = self.current_theme.write() {
                current.name = next_theme_name;
                Ok(Some(()))
            } else {
                Err("无法获取当前主题写锁".to_string())
            }
        } else {
            Ok(None)
        }
    }

    /// 清除主题历史
    pub fn clear_theme_history(&self) -> Result<(), String> {
        self.theme_history.clear_history()
    }
}
