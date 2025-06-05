//! CSS 变量管理模块
//!
//! 负责 CSS 变量的生成、注入、更新和管理。
//! 提供高性能的 CSS 变量操作和主题切换支持。

use crate::theme::{core::token::TokenSystem, Theme};
use std::collections::HashMap;
use std::fmt::Write;

/// CSS 变量特征
pub trait CssVariables {
    /// 生成 CSS 变量声明
    fn to_css_variables(&self) -> String;
    /// 获取 CSS 变量映射
    fn get_variables(&self) -> HashMap<String, String>;
}

/// CSS 变量管理器
///
/// 管理和生成 CSS 变量，支持前缀、作用域和压缩选项。
pub struct CssVariableManager {
    /// 当前变量集合
    variables: HashMap<String, String>,
    /// 变量前缀
    prefix: String,
    /// 作用域选择器
    scope_selector: Option<String>,
    /// 是否启用压缩
    minify: bool,
}

/// CSS 变量生成器
///
/// 根据不同策略生成 CSS 变量。
pub struct CssVariableGenerator {
    /// 变量命名策略
    naming_strategy: VariableNamingStrategy,
    /// 值转换策略
    value_transform: ValueTransformStrategy,
    /// 输出格式
    output_format: OutputFormat,
    variables: HashMap<String, String>,
}

/// 变量命名策略
pub enum VariableNamingStrategy {
    /// 扁平化命名：--color-primary
    Flat,
    /// 层级命名：--color-primary-500
    Hierarchical,
    /// 语义化命名：--primary-color
    Semantic,
    /// 自定义前缀：--custom-color-primary
    CustomPrefix(String),
}

/// 值转换策略
pub enum ValueTransformStrategy {
    /// 保持原值
    None,
    /// 转换为 RGB
    ToRgb,
    /// 转换为 HSL
    ToHsl,
    /// 添加回退值
    WithFallback(String),
}

/// 输出格式
pub enum OutputFormat {
    /// 标准 CSS
    Standard,
    /// 压缩 CSS
    Minified,
    /// 带注释的 CSS
    WithComments,
    /// JSON 格式
    Json,
}

/// CSS 变量注入器
///
/// 负责将 CSS 变量注入到 DOM 或 CSS 文件中。
#[derive(Debug, Clone)]
pub struct CssVariableInjector {
    /// 目标选择器（可以是文件路径、选择器等）
    target_selector: String,
    /// 注入策略
    injection_strategy: InjectionStrategy,
    /// 是否启用批量更新
    batch_updates: bool,
    /// 当前CSS内容缓存
    current_css: Option<String>,
    /// 当前变量状态
    current_variables: HashMap<String, String>,
}

/// 注入策略
#[derive(Debug, Clone)]
pub enum InjectionStrategy {
    /// 替换现有样式
    Replace,
    /// 合并到现有样式
    Merge,
    /// 追加到现有样式
    Append,
    /// 智能合并（检测冲突）
    SmartMerge,
}

/// 变量更新事件
///
/// 描述变量更新的事件信息。
pub struct VariableUpdateEvent {
    /// 变更的变量
    pub changed_variables: HashMap<String, String>,
    /// 更新时间戳
    pub timestamp: u64,
    /// 更新原因
    pub reason: UpdateReason,
}

/// 更新原因
pub enum UpdateReason {
    /// 主题切换
    ThemeSwitch,
    /// 令牌更新
    TokenUpdate,
    /// 用户自定义
    UserCustom,
    /// 响应式变化
    ResponsiveChange,
}

impl CssVariableManager {
    /// 创建新的 CSS 变量管理器
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            prefix: String::new(),
            scope_selector: None,
            minify: false,
        }
    }

    /// 设置变量前缀
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// 设置作用域选择器
    pub fn with_scope(mut self, selector: impl Into<String>) -> Self {
        self.scope_selector = Some(selector.into());
        self
    }

    /// 设置是否压缩
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.minify = minify;
        self
    }

    /// 从主题生成 CSS 变量
    pub fn generate_from_theme(&mut self, theme: &Theme) -> Result<(), String> {
        self.variables.clear();

        // 生成设计令牌变量
        self.generate_from_token_system(&theme.token_system)?;

        // 添加自定义变量
        for (key, value) in &theme.custom_variables {
            let var_name = self.format_variable_name(key);
            self.variables.insert(var_name, value.clone());
        }

        Ok(())
    }

    /// 从设计令牌系统生成变量
    fn generate_from_token_system(&mut self, token_system: &TokenSystem) -> Result<(), String> {
        // 使用 token_system 的 to_css_variables 方法获取 CSS 变量字符串
        let css = token_system.to_css_variables();

        // 解析 CSS 变量并添加到变量集合
        self.parse_and_add_css_variables(&css);

        Ok(())
    }

    /// 解析 CSS 变量字符串并添加到变量集合
    fn parse_and_add_css_variables(&mut self, css: &str) {
        for line in css.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with("--") {
                continue;
            }

            if let Some((name, value)) = line.split_once(':') {
                let name = name.trim();
                let value = value.trim().trim_end_matches(';').trim();
                self.variables.insert(name.to_string(), value.to_string());
            }
        }
    }

    /// 添加变量
    pub fn add_variable(&mut self, name: &str, value: &str) {
        let var_name = self.format_variable_name(name);
        self.variables.insert(var_name, value.to_string());
    }

    /// 格式化变量名
    fn format_variable_name(&self, name: &str) -> String {
        if name.starts_with("--") {
            name.to_string()
        } else {
            format!("--{}{}", self.prefix, name)
        }
    }

    /// 更新变量
    pub fn update_variable(&mut self, name: &str, value: impl Into<String>) {
        let var_name = self.format_variable_name(name);
        self.variables.insert(var_name, value.into());
    }

    /// 批量更新变量
    pub fn update_variables(&mut self, updates: HashMap<String, String>) {
        for (name, value) in updates {
            self.update_variable(&name, value);
        }
    }

    /// 获取变量值
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        let var_name = self.format_variable_name(name);
        self.variables.get(&var_name)
    }

    /// 获取所有变量
    pub fn get_all_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// 生成 CSS 字符串
    pub fn to_css(&self) -> String {
        let mut css = String::new();

        let scope = if let Some(selector) = &self.scope_selector {
            selector.clone()
        } else {
            ":root".to_string()
        };

        if !self.minify {
            css.push_str(&format!("{} {{\n", scope));
        } else {
            css.push_str(&format!("{}{{", scope));
        }

        for (name, value) in &self.variables {
            if !self.minify {
                css.push_str(&format!("  {}: {};\n", name, value));
            } else {
                css.push_str(&format!("{}:{};", name, value));
            }
        }

        if !self.minify {
            css.push_str("}\n");
        } else {
            css.push('}');
        }

        css
    }

    /// 生成 JSON 字符串
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.variables).map_err(|e| e.to_string())
    }

    /// 清空所有变量
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    /// 获取变量数量
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Default for CssVariableManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CssVariableGenerator {
    /// 创建新的 CSS 变量生成器
    pub fn new() -> Self {
        Self {
            naming_strategy: VariableNamingStrategy::Flat,
            value_transform: ValueTransformStrategy::None,
            output_format: OutputFormat::Standard,
            variables: HashMap::new(),
        }
    }

    /// 设置命名策略
    pub fn with_naming_strategy(mut self, strategy: VariableNamingStrategy) -> Self {
        self.naming_strategy = strategy;
        self
    }

    /// 设置值转换策略
    pub fn with_value_transform(mut self, transform: ValueTransformStrategy) -> Self {
        self.value_transform = transform;
        self
    }

    /// 设置输出格式
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }

    /// 从主题生成 CSS 变量
    pub fn generate(&self, theme: &Theme) -> Result<String, String> {
        let mut manager = CssVariableManager::new();

        // 根据命名策略设置前缀
        match &self.naming_strategy {
            VariableNamingStrategy::CustomPrefix(prefix) => {
                manager = manager.with_prefix(prefix);
            }
            _ => {}
        }

        // 根据输出格式设置压缩
        match self.output_format {
            OutputFormat::Minified => {
                manager = manager.with_minify(true);
            }
            _ => {}
        }

        manager.generate_from_theme(theme)?;

        match self.output_format {
            OutputFormat::Json => manager.to_json(),
            _ => Ok(manager.to_css()),
        }
    }
}

impl Default for CssVariableGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CssVariableInjector {
    /// 创建新的 CSS 变量注入器
    pub fn new(target_selector: impl Into<String>) -> Self {
        Self {
            target_selector: target_selector.into(),
            injection_strategy: InjectionStrategy::Replace,
            batch_updates: false,
            current_css: None,
            current_variables: HashMap::new(),
        }
    }

    /// 设置注入策略
    pub fn with_strategy(mut self, strategy: InjectionStrategy) -> Self {
        self.injection_strategy = strategy;
        self
    }

    /// 设置是否启用批量更新
    pub fn with_batch_updates(mut self, batch_updates: bool) -> Self {
        self.batch_updates = batch_updates;
        self
    }

    /// 注入 CSS 变量
    pub fn inject_css_variables(
        &mut self,
        variables: &HashMap<String, String>,
    ) -> Result<(), String> {
        // 检查是否有变化
        if &self.current_variables == variables {
            return Ok(());
        }

        // 生成 CSS 字符串
        let mut css = format!("{} {{\n", self.target_selector);

        for (name, value) in variables {
            css.push_str(&format!("  {}: {};\n", name, value));
        }

        css.push_str("}\n");

        // 保存当前状态
        self.current_variables = variables.clone();
        self.current_css = Some(css.clone());

        // 注入代码的实现将因环境而异，这里只是一个存根
        Ok(())
    }

    /// 获取当前 CSS 内容
    pub fn get_current_css(&self) -> Option<&String> {
        self.current_css.as_ref()
    }

    /// 获取当前变量
    pub fn get_current_variables(&self) -> &HashMap<String, String> {
        &self.current_variables
    }

    /// 获取目标选择器
    pub fn get_target_selector(&self) -> &str {
        &self.target_selector
    }

    /// 获取注入策略
    pub fn get_injection_strategy(&self) -> &InjectionStrategy {
        &self.injection_strategy
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.current_css = None;
        self.current_variables.clear();
    }
}

/// CSS 变量工具函数
pub struct CssVariableUtils;

impl CssVariableUtils {
    /// 解析 CSS 变量引用
    pub fn parse_var_reference(css: &str) -> Vec<String> {
        let mut variables = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = css.chars().collect();

        while i < chars.len() {
            if i + 4 < chars.len() && chars[i..i + 4].iter().collect::<String>() == "var(" {
                // 找到 var( 开始
                i += 4; // 跳过 "var("

                // 查找变量名开始位置（--）
                while i < chars.len() && chars[i] != '-' {
                    i += 1;
                }

                if i + 1 < chars.len() && chars[i] == '-' && chars[i + 1] == '-' {
                    let start = i;
                    // 查找变量名结束位置（, 或 )）
                    while i < chars.len() && chars[i] != ',' && chars[i] != ')' {
                        i += 1;
                    }

                    if i > start {
                        let var_name: String = chars[start..i]
                            .iter()
                            .collect::<String>()
                            .trim()
                            .to_string();
                        if !var_name.is_empty() {
                            variables.push(var_name);
                        }
                    }
                }
            } else {
                i += 1;
            }
        }

        variables
    }

    /// 验证 CSS 变量名
    pub fn validate_variable_name(name: &str) -> bool {
        name.starts_with("--")
            && name.len() > 2
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    /// 格式化 CSS 变量值
    pub fn format_variable_value(value: &str, transform: &ValueTransformStrategy) -> String {
        match transform {
            ValueTransformStrategy::None => value.to_string(),
            ValueTransformStrategy::ToRgb => {
                // 简单的颜色转换示例
                if value.starts_with('#') {
                    // 这里应该实现完整的颜色转换逻辑
                    format!("rgb({})", value)
                } else {
                    value.to_string()
                }
            }
            ValueTransformStrategy::ToHsl => {
                // 简单的颜色转换示例
                if value.starts_with('#') {
                    // 这里应该实现完整的颜色转换逻辑
                    format!("hsl({})", value)
                } else {
                    value.to_string()
                }
            }
            ValueTransformStrategy::WithFallback(fallback) => {
                format!("{}, {}", value, fallback)
            }
        }
    }

    /// 生成 CSS 变量引用
    pub fn var_reference(name: &str, fallback: Option<&str>) -> String {
        match fallback {
            Some(fb) => format!("var({}, {})", name, fb),
            None => format!("var({})", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;
    use super::*;

    #[test]
    fn test_css_variable_manager_creation() {
        let manager = CssVariableManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.prefix, "");
    }

    #[test]
    fn test_variable_generation_from_theme() {
        let mut manager = CssVariableManager::new();
        let theme = Theme::default();

        assert!(manager.generate_from_theme(&theme).is_ok());
        assert!(!manager.is_empty());

        let primary_var = manager.get_variable("color-primary");
        assert!(primary_var.is_some());
    }

    #[test]
    fn test_css_output() {
        let mut manager = CssVariableManager::new().with_prefix("test");
        manager.add_variable("color", "#ff0000");

        let css = manager.to_css();
        assert!(css.contains(":root {"));
        assert!(css.contains("--test-color: #ff0000;"));
    }

    #[test]
    fn test_minified_output() {
        let mut manager = CssVariableManager::new().with_minify(true);
        manager.add_variable("color", "#ff0000");

        let css = manager.to_css();
        assert!(!css.contains("\n"));
        assert!(css.contains(":root{--css-in-rust-color:#ff0000;}"));
    }

    #[test]
    fn test_variable_updates() {
        let mut manager = CssVariableManager::new();
        manager.update_variable("test", "value1");

        assert_eq!(manager.get_variable("test"), Some(&"value1".to_string()));

        manager.update_variable("test", "value2");
        assert_eq!(manager.get_variable("test"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_css_variable_generator() {
        let generator = CssVariableGenerator::new()
            .with_naming_strategy(VariableNamingStrategy::CustomPrefix("custom".to_string()))
            .with_output_format(OutputFormat::Minified);

        let theme = Theme::default();
        let css = generator.generate(&theme).unwrap();

        assert!(css.contains("--custom-"));
        assert!(!css.contains("\n"));
    }

    #[test]
    fn test_variable_name_validation() {
        assert!(CssVariableUtils::validate_variable_name("--valid-name"));
        assert!(CssVariableUtils::validate_variable_name("--valid_name_123"));
        assert!(!CssVariableUtils::validate_variable_name("invalid-name"));
        assert!(!CssVariableUtils::validate_variable_name("--"));
        assert!(!CssVariableUtils::validate_variable_name("--invalid name"));
    }

    #[test]
    fn test_var_reference_generation() {
        let ref1 = CssVariableUtils::var_reference("--color-primary", None);
        assert_eq!(ref1, "var(--color-primary)");

        let ref2 = CssVariableUtils::var_reference("--color-primary", Some("#0066cc"));
        assert_eq!(ref2, "var(--color-primary, #0066cc)");
    }

    #[test]
    fn test_parse_var_reference() {
        let css = "color: var(--primary-color); background: var(--bg-color, white);";
        let vars = CssVariableUtils::parse_var_reference(css);

        assert!(vars.contains(&"--primary-color".to_string()));
        assert!(vars.contains(&"--bg-color".to_string()));
    }
}
