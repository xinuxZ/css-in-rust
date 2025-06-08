pub mod theme_history;

use crate::theme::theme_types::{Theme, ThemeMode};
use std::sync::{Arc, RwLock};
use theme_history::ThemeHistory;

/// 主题管理器配置
///
/// 控制主题管理器的行为，包括默认主题、历史记录和持久化等功能。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::manager::ThemeManagerConfig;
///
/// let config = ThemeManagerConfig {
///     default_theme: "light".to_string(),
///     enable_history: true,
///     enable_events: true,
///     enable_persistence: true,
///     storage_key: "theme-preference".to_string(),
/// };
///
/// // 或使用默认配置
/// let default_config = ThemeManagerConfig::default();
/// ```
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
    /// 创建默认配置
    ///
    /// 默认配置使用 "light" 作为默认主题，启用历史记录和事件，但不启用持久化。
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::ThemeManagerConfig;
    ///
    /// let config = ThemeManagerConfig::default();
    /// assert_eq!(config.default_theme, "light");
    /// assert!(config.enable_history);
    /// ```
    fn default() -> Self {
        Self {
            default_theme: "light".to_string(),
            enable_history: true,
            enable_events: true,
            enable_persistence: false,
            storage_key: "theme-preference".to_string(),
        }
    }
}

/// 主题管理器
///
/// 负责管理主题的核心组件，提供主题切换、历史记录和主题状态管理功能。
///
/// # Examples
///
/// ```
/// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
/// use css_in_rust::theme::theme_types::{Theme, ThemeMode};
///
/// // 创建主题管理器
/// let manager = ThemeManager::new(ThemeManagerConfig::default());
///
/// // 获取当前主题
/// if let Some(theme) = manager.get_current_theme() {
///     println!("当前主题: {}", theme.name);
/// }
///
/// // 设置新主题
/// let dark_theme = Theme::new("dark").with_mode(ThemeMode::Dark);
/// manager.set_theme(dark_theme).unwrap();
///
/// // 切换主题模式
/// manager.toggle_theme_mode();
/// ```
pub struct ThemeManager {
    /// 当前主题
    current_theme: Arc<RwLock<Theme>>,
    /// 配置
    config: ThemeManagerConfig,
    /// 主题历史
    theme_history: ThemeHistory,
}

impl PartialEq for ThemeManager {
    fn eq(&self, other: &Self) -> bool {
        self.config.default_theme == other.config.default_theme
    }
}

impl ThemeManager {
    /// 创建新的主题管理器
    ///
    /// # Arguments
    ///
    /// * `config` - 主题管理器配置
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    ///
    /// // 使用默认配置创建管理器
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 使用自定义配置创建管理器
    /// let custom_config = ThemeManagerConfig {
    ///     default_theme: "dark".to_string(),
    ///     enable_history: true,
    ///     enable_events: true,
    ///     enable_persistence: true,
    ///     storage_key: "my-theme-preference".to_string(),
    /// };
    /// let custom_manager = ThemeManager::new(custom_config);
    /// ```
    pub fn new(config: ThemeManagerConfig) -> Self {
        Self {
            current_theme: Arc::new(RwLock::new(Theme::default())),
            config,
            theme_history: ThemeHistory::new(),
        }
    }

    /// 获取当前主题
    ///
    /// # Returns
    ///
    /// 当前主题的克隆，如果出错则返回 `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// if let Some(theme) = manager.get_current_theme() {
    ///     println!("当前主题: {}", theme.name);
    ///     println!("主题模式: {:?}", theme.mode);
    /// }
    /// ```
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.current_theme.read().ok().map(|t| t.clone())
    }

    /// 设置主题
    ///
    /// # Arguments
    ///
    /// * `theme` - 要设置的主题
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`, 失败返回错误信息
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::{Theme, ThemeMode};
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 创建并设置暗色主题
    /// let dark_theme = Theme::new("dark").with_mode(ThemeMode::Dark);
    /// manager.set_theme(dark_theme).unwrap();
    ///
    /// // 验证主题已设置
    /// if let Some(theme) = manager.get_current_theme() {
    ///     assert_eq!(theme.name, "dark");
    ///     assert!(matches!(theme.mode, ThemeMode::Dark));
    /// }
    /// ```
    pub fn set_theme(&self, theme: Theme) -> Result<(), String> {
        // 更新当前主题
        if let Ok(mut current) = self.current_theme.write() {
            // 如果启用了历史记录，添加到历史
            if self.config.enable_history {
                if let Err(e) = self.theme_history.add_theme(&current.name) {
                    eprintln!("添加主题到历史记录失败: {}", e);
                }
            }

            *current = theme;
            Ok(())
        } else {
            Err("无法获取主题写锁".to_string())
        }
    }

    /// 切换主题模式
    ///
    /// 在亮色和暗色主题之间切换
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::ThemeMode;
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 切换主题模式
    /// manager.toggle_theme_mode();
    ///
    /// // 验证模式已切换
    /// if let Some(theme) = manager.get_current_theme() {
    ///     assert!(matches!(theme.mode, ThemeMode::Dark));
    /// }
    ///
    /// // 再次切换回亮色模式
    /// manager.toggle_theme_mode();
    /// ```
    pub fn toggle_theme_mode(&self) {
        if let Ok(mut theme) = self.current_theme.write() {
            theme.mode = match theme.mode {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
                ThemeMode::Auto => ThemeMode::Light,
            };
        }
    }

    /// 获取主题历史记录
    ///
    /// # Returns
    ///
    /// 主题历史记录对象的引用
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 获取主题历史
    /// let history = manager.get_theme_history();
    ///
    /// // 查看历史记录
    /// if let Ok(themes) = history.get_history() {
    ///     for theme in themes {
    ///         println!("历史主题: {}", theme);
    ///     }
    /// }
    /// ```
    pub fn get_theme_history(&self) -> &ThemeHistory {
        &self.theme_history
    }

    /// 返回到上一个主题
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(Some(()))` 如果有上一个主题，`Ok(None)` 如果没有上一个主题，
    /// 或者 `Err` 如果出现错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 设置几个主题以创建历史记录
    /// manager.set_theme(Theme::new("theme1")).unwrap();
    /// manager.set_theme(Theme::new("theme2")).unwrap();
    ///
    /// // 返回到上一个主题
    /// if let Ok(Some(_)) = manager.go_back_theme() {
    ///     // 成功返回到上一个主题
    ///     if let Some(theme) = manager.get_current_theme() {
    ///         assert_eq!(theme.name, "theme1");
    ///     }
    /// }
    /// ```
    pub fn go_back_theme(&self) -> Result<Option<()>, String> {
        if let Ok(Some(theme_name)) = self.theme_history.get_previous_theme() {
            // 创建新主题
            let theme = Theme::new(theme_name);

            // 设置主题
            if let Ok(mut current) = self.current_theme.write() {
                *current = theme;
                Ok(Some(()))
            } else {
                Err("无法获取主题写锁".to_string())
            }
        } else {
            Ok(None)
        }
    }

    /// 前进到下一个主题
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(Some(()))` 如果有下一个主题，`Ok(None)` 如果没有下一个主题，
    /// 或者 `Err` 如果出现错误
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 设置几个主题以创建历史记录
    /// manager.set_theme(Theme::new("theme1")).unwrap();
    /// manager.set_theme(Theme::new("theme2")).unwrap();
    ///
    /// // 返回到上一个主题
    /// manager.go_back_theme().unwrap();
    ///
    /// // 前进到下一个主题
    /// if let Ok(Some(_)) = manager.go_forward_theme() {
    ///     // 成功前进到下一个主题
    ///     if let Some(theme) = manager.get_current_theme() {
    ///         assert_eq!(theme.name, "theme2");
    ///     }
    /// }
    /// ```
    pub fn go_forward_theme(&self) -> Result<Option<()>, String> {
        if let Ok(Some(theme_name)) = self.theme_history.get_next_theme() {
            // 创建新主题
            let theme = Theme::new(theme_name);

            // 设置主题
            if let Ok(mut current) = self.current_theme.write() {
                *current = theme;
                Ok(Some(()))
            } else {
                Err("无法获取主题写锁".to_string())
            }
        } else {
            Ok(None)
        }
    }

    /// 清除主题历史记录
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`, 失败返回错误信息
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 清除历史记录
    /// manager.clear_theme_history().unwrap();
    ///
    /// // 验证历史记录已清除
    /// if let Ok(history) = manager.get_theme_history().get_history() {
    ///     assert!(history.is_empty());
    /// }
    /// ```
    pub fn clear_theme_history(&self) -> Result<(), String> {
        self.theme_history.clear_history()
    }

    /// 切换到指定名称的主题
    ///
    /// # Arguments
    ///
    /// * `theme_name` - 主题名称
    ///
    /// # Returns
    ///
    /// 成功返回 `Ok(())`, 失败返回错误信息
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::Theme;
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 先设置一个主题
    /// manager.set_theme(Theme::new("dark")).unwrap();
    ///
    /// // 切换到另一个主题
    /// manager.switch_theme("light").unwrap();
    ///
    /// // 验证主题已切换
    /// if let Some(theme) = manager.get_current_theme() {
    ///     assert_eq!(theme.name, "light");
    /// }
    /// ```
    pub fn switch_theme(&self, theme_name: &str) -> Result<(), String> {
        // 创建新主题
        let theme = Theme::new(theme_name);

        // 设置主题
        self.set_theme(theme)
    }

    /// 根据主题模式查找主题
    ///
    /// # Arguments
    ///
    /// * `mode` - 主题模式
    ///
    /// # Returns
    ///
    /// 匹配的主题名称，如果没有找到则返回 `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    /// use css_in_rust::theme::theme_types::ThemeMode;
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 查找亮色主题
    /// if let Some(theme_name) = manager.find_theme_by_mode(ThemeMode::Light) {
    ///     println!("找到亮色主题: {}", theme_name);
    /// }
    ///
    /// // 查找暗色主题
    /// if let Some(theme_name) = manager.find_theme_by_mode(ThemeMode::Dark) {
    ///     println!("找到暗色主题: {}", theme_name);
    /// }
    /// ```
    pub fn find_theme_by_mode(&self, mode: ThemeMode) -> Option<String> {
        // 在实际实现中，这里应该查询可用的主题列表
        // 这里只是一个简化的实现
        match mode {
            ThemeMode::Light => Some("light".to_string()),
            ThemeMode::Dark => Some("dark".to_string()),
            ThemeMode::Auto => Some("auto".to_string()),
        }
    }

    /// 获取可用的主题列表
    ///
    /// # Returns
    ///
    /// 可用的主题名称列表
    ///
    /// # Examples
    ///
    /// ```
    /// use css_in_rust::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    ///
    /// let manager = ThemeManager::new(ThemeManagerConfig::default());
    ///
    /// // 获取可用主题
    /// let available_themes = manager.get_available_themes();
    /// for theme in available_themes {
    ///     println!("可用主题: {}", theme);
    /// }
    /// ```
    pub fn get_available_themes(&self) -> Vec<String> {
        // 在实际实现中，这里应该返回系统中可用的主题列表
        // 这里只是一个简化的实现
        vec!["light".to_string(), "dark".to_string()]
    }
}
