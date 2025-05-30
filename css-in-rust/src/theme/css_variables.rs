//! CSS 变量管理模块
//!
//! 负责 CSS 变量的生成、注入、更新和管理。
//! 提供高性能的 CSS 变量操作和主题切换支持。

use super::{DesignTokens, Theme};
use std::collections::HashMap;
use std::fmt::Write;

/// CSS 变量管理器
///
/// 负责管理 CSS 变量的生成、注入和更新
#[derive(Debug, Clone)]
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
/// 提供各种 CSS 变量生成策略
#[derive(Debug, Clone)]
pub struct CssVariableGenerator {
    /// 变量命名策略
    naming_strategy: VariableNamingStrategy,
    /// 值转换策略
    value_transform: ValueTransformStrategy,
    /// 输出格式
    output_format: OutputFormat,
}

/// 变量命名策略
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
/// 负责将 CSS 变量注入到 DOM 中
#[derive(Debug, Clone)]
pub struct CssVariableInjector {
    /// 目标选择器
    target_selector: String,
    /// 注入策略
    injection_strategy: InjectionStrategy,
    /// 是否启用批量更新
    batch_updates: bool,
}

/// 注入策略
#[derive(Debug, Clone, PartialEq)]
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

/// CSS 变量更新事件
#[derive(Debug, Clone)]
pub struct VariableUpdateEvent {
    /// 变更的变量
    pub changed_variables: HashMap<String, String>,
    /// 更新时间戳
    pub timestamp: u64,
    /// 更新原因
    pub reason: UpdateReason,
}

/// 更新原因
#[derive(Debug, Clone, PartialEq)]
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
            prefix: "css-in-rust".to_string(),
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

    /// 启用压缩
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.minify = minify;
        self
    }

    /// 从主题生成 CSS 变量
    pub fn generate_from_theme(&mut self, theme: &Theme) -> Result<(), String> {
        self.variables.clear();

        // 生成设计令牌变量
        self.generate_from_tokens(&theme.tokens)?;

        // 添加自定义变量
        for (key, value) in &theme.custom_variables {
            let var_name = self.format_variable_name(key);
            self.variables.insert(var_name, value.clone());
        }

        Ok(())
    }

    /// 从设计令牌生成变量
    fn generate_from_tokens(&mut self, tokens: &DesignTokens) -> Result<(), String> {
        // 颜色变量
        self.add_color_variables(&tokens.colors)?;

        // 字体变量
        self.add_typography_variables(&tokens.typography)?;

        // 间距变量
        self.add_spacing_variables(&tokens.spacing)?;

        // 边框变量
        self.add_border_variables(&tokens.borders)?;

        // 阴影变量
        self.add_shadow_variables(&tokens.shadows)?;

        // 动画变量
        self.add_motion_variables(&tokens.motion)?;

        // 断点变量
        self.add_breakpoint_variables(&tokens.breakpoints)?;

        Ok(())
    }

    /// 添加颜色变量
    fn add_color_variables(&mut self, colors: &super::ColorTokens) -> Result<(), String> {
        // 主要颜色
        self.add_variable("color-primary", &colors.primary);
        self.add_variable("color-success", &colors.success);
        self.add_variable("color-warning", &colors.warning);
        self.add_variable("color-error", &colors.error);
        self.add_variable("color-info", &colors.info);

        // 文本颜色
        self.add_variable("color-text-primary", &colors.text.primary);
        self.add_variable("color-text-secondary", &colors.text.secondary);
        self.add_variable("color-text-disabled", &colors.text.disabled);
        self.add_variable("color-text-inverse", &colors.text.inverse);

        // 背景颜色
        self.add_variable("color-bg-primary", &colors.background.primary);
        self.add_variable("color-bg-secondary", &colors.background.secondary);
        self.add_variable("color-bg-tertiary", &colors.background.tertiary);
        self.add_variable("color-bg-inverse", &colors.background.inverse);

        // 边框颜色
        self.add_variable("color-border-primary", &colors.border.primary);
        self.add_variable("color-border-secondary", &colors.border.secondary);
        self.add_variable("color-border-inverse", &colors.border.inverse);

        // 色阶
        self.add_color_scale("blue", &colors.blue);
        self.add_color_scale("green", &colors.green);
        self.add_color_scale("red", &colors.red);
        self.add_color_scale("orange", &colors.orange);
        self.add_color_scale("gray", &colors.gray);

        Ok(())
    }

    /// 添加色阶变量
    fn add_color_scale(&mut self, color_name: &str, scale: &super::ColorScale) {
        self.add_variable(&format!("color-{}-1", color_name), &scale.c1);
        self.add_variable(&format!("color-{}-2", color_name), &scale.c2);
        self.add_variable(&format!("color-{}-3", color_name), &scale.c3);
        self.add_variable(&format!("color-{}-4", color_name), &scale.c4);
        self.add_variable(&format!("color-{}-5", color_name), &scale.c5);
        self.add_variable(&format!("color-{}-6", color_name), &scale.c6);
        self.add_variable(&format!("color-{}-7", color_name), &scale.c7);
        self.add_variable(&format!("color-{}-8", color_name), &scale.c8);
        self.add_variable(&format!("color-{}-9", color_name), &scale.c9);
        self.add_variable(&format!("color-{}-10", color_name), &scale.c10);
    }

    /// 添加字体变量
    fn add_typography_variables(
        &mut self,
        typography: &super::TypographyTokens,
    ) -> Result<(), String> {
        // 字体族
        self.add_variable("font-family-sans", &typography.font_family.sans);
        self.add_variable("font-family-serif", &typography.font_family.serif);
        self.add_variable("font-family-mono", &typography.font_family.mono);

        // 字体大小
        self.add_variable("font-size-xs", &typography.font_size.xs);
        self.add_variable("font-size-sm", &typography.font_size.sm);
        self.add_variable("font-size-md", &typography.font_size.md);
        self.add_variable("font-size-lg", &typography.font_size.lg);
        self.add_variable("font-size-xl", &typography.font_size.xl);
        self.add_variable("font-size-xxl", &typography.font_size.xxl);
        self.add_variable("font-size-xxxl", &typography.font_size.xxxl);

        // 字重
        self.add_variable("font-weight-light", &typography.font_weight.light);
        self.add_variable("font-weight-normal", &typography.font_weight.normal);
        self.add_variable("font-weight-medium", &typography.font_weight.medium);
        self.add_variable("font-weight-semibold", &typography.font_weight.semibold);
        self.add_variable("font-weight-bold", &typography.font_weight.bold);

        // 行高
        self.add_variable("line-height-tight", &typography.line_height.tight);
        self.add_variable("line-height-normal", &typography.line_height.normal);
        self.add_variable("line-height-relaxed", &typography.line_height.relaxed);

        // 字间距
        self.add_variable("letter-spacing-tight", &typography.letter_spacing.tight);
        self.add_variable("letter-spacing-normal", &typography.letter_spacing.normal);
        self.add_variable("letter-spacing-wide", &typography.letter_spacing.wide);

        Ok(())
    }

    /// 添加间距变量
    fn add_spacing_variables(&mut self, spacing: &super::SpacingTokens) -> Result<(), String> {
        self.add_variable("spacing-xs", &spacing.xs);
        self.add_variable("spacing-sm", &spacing.sm);
        self.add_variable("spacing-md", &spacing.md);
        self.add_variable("spacing-lg", &spacing.lg);
        self.add_variable("spacing-xl", &spacing.xl);
        self.add_variable("spacing-xxl", &spacing.xxl);
        self.add_variable("spacing-xxxl", &spacing.xxxl);
        Ok(())
    }

    /// 添加边框变量
    fn add_border_variables(&mut self, borders: &super::BorderTokens) -> Result<(), String> {
        // 边框宽度
        self.add_variable("border-width-none", &borders.width.none);
        self.add_variable("border-width-thin", &borders.width.thin);
        self.add_variable("border-width-medium", &borders.width.medium);
        self.add_variable("border-width-thick", &borders.width.thick);

        // 边框圆角
        self.add_variable("border-radius-none", &borders.radius.none);
        self.add_variable("border-radius-sm", &borders.radius.sm);
        self.add_variable("border-radius-md", &borders.radius.md);
        self.add_variable("border-radius-lg", &borders.radius.lg);
        self.add_variable("border-radius-xl", &borders.radius.xl);
        self.add_variable("border-radius-full", &borders.radius.full);

        Ok(())
    }

    /// 添加阴影变量
    fn add_shadow_variables(&mut self, shadows: &super::ShadowTokens) -> Result<(), String> {
        self.add_variable("shadow-sm", &shadows.sm);
        self.add_variable("shadow-md", &shadows.md);
        self.add_variable("shadow-lg", &shadows.lg);
        self.add_variable("shadow-xl", &shadows.xl);
        self.add_variable("shadow-inner", &shadows.inner);
        Ok(())
    }

    /// 添加动画变量
    fn add_motion_variables(&mut self, motion: &super::MotionTokens) -> Result<(), String> {
        // 动画时长
        self.add_variable("motion-duration-fast", &motion.duration.fast);
        self.add_variable("motion-duration-normal", &motion.duration.normal);
        self.add_variable("motion-duration-slow", &motion.duration.slow);

        // 动画缓动
        self.add_variable("motion-easing-linear", &motion.easing.linear);
        self.add_variable("motion-easing-ease-in", &motion.easing.ease_in);
        self.add_variable("motion-easing-ease-out", &motion.easing.ease_out);
        self.add_variable("motion-easing-ease-in-out", &motion.easing.ease_in_out);

        Ok(())
    }

    /// 添加断点变量
    fn add_breakpoint_variables(
        &mut self,
        breakpoints: &super::BreakpointTokens,
    ) -> Result<(), String> {
        self.add_variable("breakpoint-xs", &breakpoints.xs);
        self.add_variable("breakpoint-sm", &breakpoints.sm);
        self.add_variable("breakpoint-md", &breakpoints.md);
        self.add_variable("breakpoint-lg", &breakpoints.lg);
        self.add_variable("breakpoint-xl", &breakpoints.xl);
        self.add_variable("breakpoint-xxl", &breakpoints.xxl);
        Ok(())
    }

    /// 添加单个变量
    fn add_variable(&mut self, name: &str, value: &str) {
        let var_name = self.format_variable_name(name);
        self.variables.insert(var_name, value.to_string());
    }

    /// 格式化变量名
    fn format_variable_name(&self, name: &str) -> String {
        format!("--{}-{}", self.prefix, name)
    }

    /// 更新变量值
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
        let selector = self.scope_selector.as_deref().unwrap_or(":root");

        let mut css = String::new();

        if self.minify {
            write!(&mut css, "{}{{", selector).unwrap();
            for (name, value) in &self.variables {
                write!(&mut css, "{}:{};", name, value).unwrap();
            }
            css.push('}');
        } else {
            writeln!(&mut css, "{} {{", selector).unwrap();
            for (name, value) in &self.variables {
                writeln!(&mut css, "  {}: {};", name, value).unwrap();
            }
            css.push_str("}\n");
        }

        css
    }

    /// 生成 JSON 格式
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.variables)
            .map_err(|e| format!("Failed to serialize variables to JSON: {}", e))
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
            batch_updates: true,
        }
    }

    /// 设置注入策略
    pub fn with_strategy(mut self, strategy: InjectionStrategy) -> Self {
        self.injection_strategy = strategy;
        self
    }

    /// 启用批量更新
    pub fn with_batch_updates(mut self, enabled: bool) -> Self {
        self.batch_updates = enabled;
        self
    }

    /// 注入 CSS 变量
    pub fn inject(&self, css: &str) -> Result<(), String> {
        // 在实际实现中，这里会将 CSS 注入到 DOM 中
        // 目前只是模拟实现
        println!(
            "Injecting CSS variables to {}: {}",
            self.target_selector, css
        );
        Ok(())
    }

    /// 更新特定变量
    pub fn update_variable(&self, name: &str, value: &str) -> Result<(), String> {
        // 在实际实现中，这里会更新 DOM 中的特定 CSS 变量
        println!("Updating variable {}: {}", name, value);
        Ok(())
    }

    /// 批量更新变量
    pub fn update_variables(&self, variables: &HashMap<String, String>) -> Result<(), String> {
        if self.batch_updates {
            // 批量更新
            println!("Batch updating {} variables", variables.len());
        } else {
            // 逐个更新
            for (name, value) in variables {
                self.update_variable(name, value)?;
            }
        }
        Ok(())
    }
}

/// CSS 变量工具函数
pub struct CssVariableUtils;

impl CssVariableUtils {
    /// 解析 CSS 变量引用
    pub fn parse_var_reference(css: &str) -> Vec<String> {
        let mut variables = Vec::new();
        let mut chars = css.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == 'v' && chars.peek() == Some(&'a') {
                // 可能是 var() 函数
                let remaining: String = chars.by_ref().collect();
                if remaining.starts_with("ar(") {
                    // 提取变量名
                    if let Some(start) = remaining.find("--") {
                        if let Some(end) = remaining[start..].find(')') {
                            let var_name = &remaining[start..start + end];
                            if let Some(comma_pos) = var_name.find(',') {
                                variables.push(var_name[..comma_pos].trim().to_string());
                            } else {
                                variables.push(var_name.trim().to_string());
                            }
                        }
                    }
                }
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
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn test_css_variable_manager_creation() {
        let manager = CssVariableManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.prefix, "css-in-rust");
    }

    #[test]
    fn test_variable_generation_from_theme() {
        let mut manager = CssVariableManager::new();
        let theme = Theme::ant_design();

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

        let theme = Theme::ant_design();
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

        let ref2 = CssVariableUtils::var_reference("--color-primary", Some("#1890ff"));
        assert_eq!(ref2, "var(--color-primary, #1890ff)");
    }

    #[test]
    fn test_parse_var_reference() {
        let css = "color: var(--primary-color); background: var(--bg-color, white);";
        let vars = CssVariableUtils::parse_var_reference(css);

        assert!(vars.contains(&"--primary-color".to_string()));
        assert!(vars.contains(&"--bg-color".to_string()));
    }
}
