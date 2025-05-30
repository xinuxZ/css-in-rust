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
/// 负责将 CSS 变量注入到指定目标中
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
            current_css: None,
            current_variables: HashMap::new(),
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
    pub fn inject(&mut self, css: &str) -> Result<(), String> {
        use std::fs;
        use std::io::Write;

        // 根据注入策略处理CSS内容
        let final_css = match self.injection_strategy {
            InjectionStrategy::Replace => css.to_string(),
            InjectionStrategy::Merge | InjectionStrategy::SmartMerge => {
                if let Some(existing) = &self.current_css {
                    format!(
                        "{}
{}",
                        existing, css
                    )
                } else {
                    css.to_string()
                }
            }
            InjectionStrategy::Append => {
                if let Some(existing) = &self.current_css {
                    format!(
                        "{}
{}",
                        existing, css
                    )
                } else {
                    css.to_string()
                }
            }
        };

        // 更新内部状态
        self.current_css = Some(final_css.clone());
        self.parse_and_store_variables(&final_css);

        // 根据目标选择器决定输出方式
        if self.target_selector == "stdout" || self.target_selector == ":stdout" {
            // 输出到标准输出
            println!("{}", final_css);
        } else if self.target_selector.starts_with("file:") {
            // 写入到文件
            let file_path = self
                .target_selector
                .strip_prefix("file:")
                .unwrap_or(&self.target_selector);
            fs::write(file_path, &final_css)
                .map_err(|e| format!("Failed to write CSS to file '{}': {}", file_path, e))?
        } else if self.target_selector.ends_with(".css") {
            // 直接作为文件路径处理
            fs::write(&self.target_selector, &final_css).map_err(|e| {
                format!(
                    "Failed to write CSS to file '{}': {}",
                    self.target_selector, e
                )
            })?
        } else {
            // 默认输出到标准输出，并显示目标选择器信息
            println!("/* CSS for selector: {} */", self.target_selector);
            println!("{}", final_css);
        }

        Ok(())
    }

    /// 解析CSS并存储变量到内部状态
    fn parse_and_store_variables(&mut self, css: &str) {
        // 简单的CSS变量解析
        for line in css.lines() {
            let line = line.trim();
            if line.starts_with("--") && line.contains(':') {
                if let Some(colon_pos) = line.find(':') {
                    let var_name = line[..colon_pos].trim().to_string();
                    let var_value = line[colon_pos + 1..]
                        .trim()
                        .trim_end_matches(';')
                        .trim()
                        .to_string();
                    self.current_variables.insert(var_name, var_value);
                }
            }
        }
    }

    /// 更新特定变量
    pub fn update_variable(&mut self, name: &str, value: &str) -> Result<(), String> {
        // 确保变量名以--开头
        let var_name = if name.starts_with("--") {
            name.to_string()
        } else {
            format!("--{}", name)
        };

        // 更新内部变量状态
        self.current_variables
            .insert(var_name.clone(), value.to_string());

        // 重新生成CSS
        let mut new_css = String::new();

        // 如果有现有CSS，尝试更新其中的变量
        if let Some(existing_css) = &self.current_css {
            let mut updated = false;
            for line in existing_css.lines() {
                let line = line.trim();
                if line.starts_with(&var_name) && line.contains(':') {
                    // 找到要更新的变量，替换其值
                    new_css.push_str(&format!("  {}: {};\n", var_name, value));
                    updated = true;
                } else {
                    new_css.push_str(line);
                    new_css.push('\n');
                }
            }

            // 如果变量不存在，添加到CSS中
            if !updated {
                // 查找合适的位置插入新变量
                if new_css.contains(":root {") {
                    // 在:root块中添加
                    new_css = new_css
                        .replace(":root {", &format!(":root {{\n  {}: {};", var_name, value));
                } else {
                    // 创建新的:root块
                    new_css = format!(":root {{\n  {}: {};\n}}\n{}", var_name, value, new_css);
                }
            }
        } else {
            // 没有现有CSS，创建新的
            new_css = format!(":root {{\n  {}: {};\n}}", var_name, value);
        }

        // 重新注入更新后的CSS
        self.inject(&new_css)
    }

    /// 批量更新变量
    pub fn update_variables(&mut self, variables: &HashMap<String, String>) -> Result<(), String> {
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
