//! 主题桥接模块
//!
//! 负责 ant-design-dioxus 主题与 css-in-rust 主题系统的双向同步
//! 提供主题变量注入、动态切换和类型安全的主题 API

use super::theme::{CssVariableInjector, DesignTokens, InjectionStrategy, Theme, ThemeMode};
use std::collections::HashMap;
use std::sync::Arc;

/// 主题桥接器
///
/// 负责在 ant-design-dioxus 和 css-in-rust 之间同步主题状态
#[derive(Debug, Clone)]
pub struct ThemeBridge {
    /// 当前主题
    current_theme: Arc<Theme>,
    /// CSS 变量注入器
    css_injector: CssVariableInjector,
    /// 变量缓存
    variable_cache: HashMap<String, String>,
    /// 是否启用自动同步
    auto_sync: bool,
}

impl ThemeBridge {
    /// 创建新的主题桥接器
    ///
    /// # 参数
    ///
    /// * `initial_theme` - 初始主题
    /// * `injection_strategy` - CSS 注入策略
    /// * `auto_sync` - 是否启用自动同步
    ///
    /// # 示例
    ///
    /// ```rust
    /// use css_in_rust::theme_bridge::ThemeBridge;
    /// use css_in_rust::backup::theme_v2::{Theme, InjectionStrategy};
    ///
    /// let bridge = ThemeBridge::new(
    ///     Theme::default(),
    ///     InjectionStrategy::Replace,
    ///     true
    /// );
    /// ```
    pub fn new(
        initial_theme: Theme,
        injection_strategy: InjectionStrategy,
        auto_sync: bool,
    ) -> Self {
        let css_injector = CssVariableInjector::new(":root").with_strategy(injection_strategy);
        let mut bridge = Self {
            current_theme: Arc::new(initial_theme),
            css_injector,
            variable_cache: HashMap::new(),
            auto_sync,
        };

        // 初始化时同步主题变量
        if auto_sync {
            let _ = bridge.sync_theme_variables();
        }

        bridge
    }

    /// 获取当前主题
    pub fn current_theme(&self) -> &Theme {
        &self.current_theme
    }

    /// 设置新主题
    ///
    /// # 参数
    ///
    /// * `theme` - 新主题
    ///
    /// # 返回值
    ///
    /// 返回是否成功切换主题
    pub fn set_theme(&mut self, theme: Theme) -> Result<(), ThemeBridgeError> {
        self.current_theme = Arc::new(theme);

        if self.auto_sync {
            self.sync_theme_variables()?;
        }

        Ok(())
    }

    /// 切换主题模式（亮色/暗色）
    pub fn toggle_mode(&mut self) -> Result<(), ThemeBridgeError> {
        let mut theme = (*self.current_theme).clone();
        theme.mode = match theme.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Auto => ThemeMode::Light,
        };

        self.set_theme(theme)
    }

    /// 同步主题变量到 CSS
    ///
    /// 将当前主题的设计令牌转换为 CSS 变量并注入到文档中
    pub fn sync_theme_variables(&mut self) -> Result<(), ThemeBridgeError> {
        let design_tokens = DesignTokens::new();

        let css_variables = self
            .current_theme
            .generate_design_tokens_css(&design_tokens);

        // 只有变量发生变化时才重新注入
        if css_variables != self.variable_cache {
            self.css_injector
                .inject_css_variables(&css_variables)
                .map_err(|e| ThemeBridgeError::InjectionFailed(e.to_string()))?;

            self.variable_cache = css_variables.clone();
        }

        Ok(())
    }

    /// 获取当前主题的 CSS 变量
    pub fn get_css_variables(&self) -> String {
        let design_tokens = DesignTokens::new();

        let variables = self
            .current_theme
            .generate_design_tokens_css(&design_tokens);

        // 将 HashMap 转换为 CSS 字符串
        let mut css = String::from(":root {\n");
        for (name, value) in &variables {
            let var_name = if name.starts_with("--") {
                name.clone()
            } else {
                format!("--{}", name)
            };
            css.push_str(&format!("  {}: {};\n", var_name, value));
        }
        css.push_str("}\n");
        css
    }

    /// 获取特定变量的值
    ///
    /// # 参数
    ///
    /// * `variable_name` - 变量名（不包含 `--` 前缀）
    ///
    /// # 示例
    ///
    /// ```rust
    /// let primary_color = bridge.get_variable("primary-color");
    /// ```
    pub fn get_variable(&self, variable_name: &str) -> Option<String> {
        self.variable_cache.get(variable_name).cloned()
    }

    /// 设置自定义变量
    ///
    /// # 参数
    ///
    /// * `variable_name` - 变量名（不包含 `--` 前缀）
    /// * `value` - 变量值
    pub fn set_custom_variable(
        &mut self,
        variable_name: &str,
        value: &str,
    ) -> Result<(), ThemeBridgeError> {
        let mut css_variables = HashMap::new();
        css_variables.insert(variable_name.to_string(), value.to_string());

        self.css_injector
            .inject_css_variables(&css_variables)
            .map_err(|e| ThemeBridgeError::InjectionFailed(e.to_string()))?;

        self.variable_cache
            .insert(variable_name.to_string(), value.to_string());

        Ok(())
    }

    /// 批量设置自定义变量
    ///
    /// # 参数
    ///
    /// * `variables` - 变量映射表
    pub fn set_custom_variables(
        &mut self,
        variables: HashMap<String, String>,
    ) -> Result<(), ThemeBridgeError> {
        let mut css_vars = String::from(":root {");

        for (name, value) in &variables {
            css_vars.push_str(&format!(" --{}: {};", name, value));
        }

        css_vars.push_str(" }");

        let mut css_variables = HashMap::new();
        for (name, value) in &variables {
            css_variables.insert(name.clone(), value.clone());
        }

        self.css_injector
            .inject_css_variables(&css_variables)
            .map_err(|e| ThemeBridgeError::InjectionFailed(e.to_string()))?;

        self.variable_cache.extend(variables);

        Ok(())
    }

    /// 清除所有自定义变量
    pub fn clear_custom_variables(&mut self) -> Result<(), ThemeBridgeError> {
        // 重新同步主题变量，这会覆盖所有自定义变量
        self.sync_theme_variables()
    }

    /// 检查是否为暗色模式
    pub fn is_dark_mode(&self) -> bool {
        matches!(self.current_theme.mode, ThemeMode::Dark)
    }

    /// 检查是否为亮色模式
    pub fn is_light_mode(&self) -> bool {
        matches!(self.current_theme.mode, ThemeMode::Light)
    }

    /// 获取主题名称
    pub fn theme_name(&self) -> &str {
        &self.current_theme.name
    }

    /// 解析 CSS 变量字符串为映射表
    fn parse_css_variables(&self, css: &str) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        // 简单的 CSS 变量解析
        for line in css.lines() {
            let line = line.trim();
            if line.starts_with("--") && line.contains(':') {
                if let Some((name, value)) = line.split_once(':') {
                    let name = name.trim().trim_start_matches("--");
                    let value = value.trim().trim_end_matches(';');
                    variables.insert(name.to_string(), value.to_string());
                }
            }
        }

        variables
    }
}

/// 主题桥接错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeBridgeError {
    /// CSS 注入失败
    InjectionFailed(String),
    /// 主题解析失败
    ThemeParsingFailed(String),
    /// 变量不存在
    VariableNotFound(String),
    /// 无效的变量值
    InvalidVariableValue(String),
}

impl std::fmt::Display for ThemeBridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeBridgeError::InjectionFailed(msg) => {
                write!(f, "CSS injection failed: {}", msg)
            }
            ThemeBridgeError::ThemeParsingFailed(msg) => {
                write!(f, "Theme parsing failed: {}", msg)
            }
            ThemeBridgeError::VariableNotFound(name) => {
                write!(f, "Variable not found: {}", name)
            }
            ThemeBridgeError::InvalidVariableValue(value) => {
                write!(f, "Invalid variable value: {}", value)
            }
        }
    }
}

impl std::error::Error for ThemeBridgeError {}

/// 全局主题桥接器实例
///
/// 提供全局访问主题桥接器的便捷方法
pub struct GlobalThemeBridge {
    bridge: std::sync::Mutex<Option<ThemeBridge>>,
}

impl GlobalThemeBridge {
    /// 创建新的全局主题桥接器
    pub const fn new() -> Self {
        Self {
            bridge: std::sync::Mutex::new(None),
        }
    }

    /// 初始化全局主题桥接器
    pub fn initialize(
        &self,
        theme: Theme,
        injection_strategy: InjectionStrategy,
        auto_sync: bool,
    ) -> Result<(), ThemeBridgeError> {
        let mut bridge_guard = self.bridge.lock().unwrap();
        *bridge_guard = Some(ThemeBridge::new(theme, injection_strategy, auto_sync));
        Ok(())
    }

    /// 执行主题桥接器操作
    pub fn with_bridge<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut ThemeBridge) -> R,
    {
        let mut bridge_guard = self.bridge.lock().unwrap();
        if let Some(ref mut bridge) = *bridge_guard {
            Some(f(bridge))
        } else {
            None
        }
    }
}

/// 全局主题桥接器实例
static GLOBAL_THEME_BRIDGE: GlobalThemeBridge = GlobalThemeBridge::new();

/// 初始化全局主题桥接器
///
/// # 参数
///
/// * `theme` - 初始主题
/// * `injection_strategy` - CSS 注入策略
/// * `auto_sync` - 是否启用自动同步
///
/// # 示例
///
/// ```rust
/// use css_in_rust::theme_bridge::init_global_theme_bridge;
/// use css_in_rust::backup::theme_v2::{Theme, InjectionStrategy};
///
/// init_global_theme_bridge(
///     Theme::default(),
///     InjectionStrategy::Replace,
///     true
/// ).unwrap();
/// ```
pub fn init_global_theme_bridge(
    theme: Theme,
    injection_strategy: InjectionStrategy,
    auto_sync: bool,
) -> Result<(), ThemeBridgeError> {
    GLOBAL_THEME_BRIDGE.initialize(theme, injection_strategy, auto_sync)
}

/// 使用全局主题桥接器
///
/// # 示例
///
/// ```rust
/// use css_in_rust::theme_bridge::with_global_theme_bridge;
///
/// with_global_theme_bridge(|bridge| {
///     bridge.toggle_mode().unwrap();
///     println!("Current theme: {}", bridge.theme_name());
/// });
/// ```
pub fn with_global_theme_bridge<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut ThemeBridge) -> R,
{
    GLOBAL_THEME_BRIDGE.with_bridge(f)
}

/// 获取当前主题的 CSS 变量
///
/// # 示例
///
/// ```rust
/// use css_in_rust::theme_bridge::get_current_css_variables;
///
/// if let Some(css_vars) = get_current_css_variables() {
///     println!("Current CSS variables: {}", css_vars);
/// }
/// ```
pub fn get_current_css_variables() -> Option<String> {
    with_global_theme_bridge(|bridge| bridge.get_css_variables())
}

/// 切换全局主题模式
///
/// # 示例
///
/// ```rust
/// use css_in_rust::theme_bridge::toggle_global_theme_mode;
///
/// if let Some(result) = toggle_global_theme_mode() {
///     match result {
///         Ok(_) => println!("Theme mode toggled successfully"),
///         Err(e) => eprintln!("Failed to toggle theme mode: {}", e),
///     }
/// }
/// ```
pub fn toggle_global_theme_mode() -> Option<Result<(), ThemeBridgeError>> {
    with_global_theme_bridge(|bridge| bridge.toggle_mode())
}

/// 设置全局自定义变量
///
/// # 参数
///
/// * `variable_name` - 变量名（不包含 `--` 前缀）
/// * `value` - 变量值
///
/// # 示例
///
/// ```rust
/// use css_in_rust::theme_bridge::set_global_custom_variable;
///
/// if let Some(result) = set_global_custom_variable("custom-color", "#ff0000") {
///     match result {
///         Ok(_) => println!("Custom variable set successfully"),
///         Err(e) => eprintln!("Failed to set custom variable: {}", e),
///     }
/// }
/// ```
pub fn set_global_custom_variable(
    variable_name: &str,
    value: &str,
) -> Option<Result<(), ThemeBridgeError>> {
    with_global_theme_bridge(|bridge| bridge.set_custom_variable(variable_name, value))
}
