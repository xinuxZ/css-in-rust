//! CSS生成器模块
//!
//! 本模块负责将设计令牌转换为CSS变量和样式声明。
//! 职责：CSS变量生成、样式类生成、主题切换支持

use super::token_definitions::{ThemeVariant, TokenDefinitions, TokenPath, TokenValue};
use super::token_resolver::TokenResolver;
use std::collections::HashMap;

/// CSS生成器
#[derive(Debug)]
pub struct CssGenerator {
    /// 令牌解析器
    resolver: TokenResolver,
    /// CSS变量前缀
    prefix: String,
    /// 是否压缩输出
    minify: bool,
}

impl CssGenerator {
    /// 创建新的CSS生成器
    pub fn new(resolver: TokenResolver) -> Self {
        Self {
            resolver,
            prefix: "ant".to_string(),
            minify: false,
        }
    }

    /// 获取解析器引用
    pub fn get_resolver(&self) -> &TokenResolver {
        &self.resolver
    }

    /// 获取解析器可变引用
    pub fn get_resolver_mut(&mut self) -> &mut TokenResolver {
        &mut self.resolver
    }

    /// 设置CSS变量前缀
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    /// 设置是否压缩输出
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.minify = minify;
        self
    }

    /// 生成主题的CSS变量
    pub fn generate_css_variables(&mut self, theme: ThemeVariant) -> Result<String, String> {
        let paths = self.resolver.list_token_paths(theme);
        let mut css_vars = Vec::new();

        for path in paths {
            match self.resolver.resolve_token(&path, theme) {
                Ok(value) => {
                    let var_name = self.generate_css_var_name(&path);
                    let css_value = value.to_css_value();
                    css_vars.push(format!("  {}: {};", var_name, css_value));
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to resolve token {}: {}",
                        path.to_string(),
                        e
                    ));
                }
            }
        }

        let selector = match theme {
            ThemeVariant::Light => ":root",
            ThemeVariant::Dark => "[data-theme='dark']",
            ThemeVariant::Auto => ":root",
        };

        let newline = if self.minify { "" } else { "\n" };
        let space = if self.minify { "" } else { " " };

        Ok(format!(
            "{}{{{}{}{}}}",
            selector,
            newline,
            css_vars.join(newline),
            newline
        ))
    }

    /// 生成完整的主题CSS
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        let mut css_parts = Vec::new();

        // 生成浅色主题CSS
        let light_css = self.generate_css_variables(ThemeVariant::Light)?;
        css_parts.push(light_css);

        // 生成深色主题CSS
        let dark_css = self.generate_css_variables(ThemeVariant::Dark)?;
        css_parts.push(dark_css);

        // 添加主题切换逻辑
        let theme_switch_css = self.generate_theme_switch_css();
        css_parts.push(theme_switch_css);

        let separator = if self.minify { "" } else { "\n\n" };
        Ok(css_parts.join(separator))
    }

    /// 生成主题切换CSS
    fn generate_theme_switch_css(&self) -> String {
        let newline = if self.minify { "" } else { "\n" };
        let space = if self.minify { "" } else { " " };

        format!(
            "@media (prefers-color-scheme: dark) {{{newline}{}:root {{{newline}{}color-scheme: dark;{newline}{}}}{newline}}}",
            space, space, space
        )
    }

    /// 生成CSS变量名
    fn generate_css_var_name(&self, path: &TokenPath) -> String {
        format!("--{}-{}", self.prefix, path.segments.join("-"))
    }

    /// 生成组件样式类
    pub fn generate_component_classes(
        &mut self,
        component: &str,
        theme: ThemeVariant,
    ) -> Result<String, String> {
        let component_prefix = format!("component.{}", component);
        let paths = self.resolver.list_token_paths(theme);
        let component_paths: Vec<_> = paths
            .into_iter()
            .filter(|path| path.to_string().starts_with(&component_prefix))
            .collect();

        if component_paths.is_empty() {
            return Ok(String::new());
        }

        let mut css_rules = Vec::new();
        let mut current_selector = String::new();
        let mut current_properties = Vec::new();

        for path in component_paths {
            match self.resolver.resolve_token(&path, theme) {
                Ok(value) => {
                    let path_str = path.to_string();
                    let parts: Vec<&str> = path_str.split('.').collect();

                    if parts.len() >= 3 {
                        let selector = format!(".{}-{}", self.prefix, parts[2]);
                        let property = self.path_to_css_property(&path);
                        let css_value = value.to_css_value();

                        if selector != current_selector {
                            if !current_selector.is_empty() {
                                css_rules.push(
                                    self.format_css_rule(&current_selector, &current_properties),
                                );
                            }
                            current_selector = selector;
                            current_properties.clear();
                        }

                        current_properties.push(format!("{}: {}", property, css_value));
                    }
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to resolve component token {}: {}",
                        path.to_string(),
                        e
                    ));
                }
            }
        }

        if !current_selector.is_empty() {
            css_rules.push(self.format_css_rule(&current_selector, &current_properties));
        }

        let separator = if self.minify { "" } else { "\n\n" };
        Ok(css_rules.join(separator))
    }

    /// 将令牌路径转换为CSS属性名
    fn path_to_css_property(&self, path: &TokenPath) -> String {
        let segments = &path.segments;
        if segments.len() < 4 {
            return "color".to_string();
        }

        match segments[3].as_str() {
            "backgroundColor" => "background-color".to_string(),
            "borderColor" => "border-color".to_string(),
            "borderRadius" => "border-radius".to_string(),
            "borderWidth" => "border-width".to_string(),
            "fontSize" => "font-size".to_string(),
            "fontWeight" => "font-weight".to_string(),
            "lineHeight" => "line-height".to_string(),
            "padding" => "padding".to_string(),
            "margin" => "margin".to_string(),
            "width" => "width".to_string(),
            "height" => "height".to_string(),
            _ => segments[3].replace('_', "-"),
        }
    }

    /// 格式化CSS规则
    fn format_css_rule(&self, selector: &str, properties: &[String]) -> String {
        let newline = if self.minify { "" } else { "\n" };
        let space = if self.minify { "" } else { " " };
        let indent = if self.minify { "" } else { "  " };

        format!(
            "{}{{{}{}{}{}}}",
            selector,
            newline,
            properties
                .iter()
                .map(|prop| format!("{}{};{}", indent, prop, newline))
                .collect::<String>(),
            if self.minify { "" } else { "" },
            if properties.is_empty() { "" } else { "" }
        )
    }

    /// 生成实用工具类
    pub fn generate_utility_classes(&mut self, theme: ThemeVariant) -> Result<String, String> {
        let mut utilities = Vec::new();

        // 生成颜色实用工具类
        utilities.push(self.generate_color_utilities(theme)?);

        // 生成间距实用工具类
        utilities.push(self.generate_spacing_utilities(theme)?);

        // 生成字体实用工具类
        utilities.push(self.generate_typography_utilities(theme)?);

        let separator = if self.minify { "" } else { "\n\n" };
        Ok(utilities.join(separator))
    }

    /// 生成颜色实用工具类
    fn generate_color_utilities(&mut self, theme: ThemeVariant) -> Result<String, String> {
        let color_paths = self.get_paths_by_category("color", theme);
        let mut utilities = Vec::new();

        for path in color_paths {
            if let Ok(value) = self.resolver.resolve_token(&path, theme) {
                let path_str = path.to_string();
                let parts: Vec<&str> = path_str.split('.').collect();

                if parts.len() >= 3 {
                    let class_name = format!(".{}-{}-{}", self.prefix, parts[1], parts[2]);
                    let bg_class_name = format!(".{}-bg-{}-{}", self.prefix, parts[1], parts[2]);
                    let border_class_name =
                        format!(".{}-border-{}-{}", self.prefix, parts[1], parts[2]);

                    utilities.push(self.format_css_rule(
                        &class_name,
                        &[format!("color: {}", value.to_css_value())],
                    ));
                    utilities.push(self.format_css_rule(
                        &bg_class_name,
                        &[format!("background-color: {}", value.to_css_value())],
                    ));
                    utilities.push(self.format_css_rule(
                        &border_class_name,
                        &[format!("border-color: {}", value.to_css_value())],
                    ));
                }
            }
        }

        let separator = if self.minify { "" } else { "\n" };
        Ok(utilities.join(separator))
    }

    /// 生成间距实用工具类
    fn generate_spacing_utilities(&mut self, theme: ThemeVariant) -> Result<String, String> {
        let spacing_paths = self.get_paths_by_category("spacing", theme);
        let mut utilities = Vec::new();

        for path in spacing_paths {
            if let Ok(value) = self.resolver.resolve_token(&path, theme) {
                let path_str = path.to_string();
                let parts: Vec<&str> = path_str.split('.').collect();

                if parts.len() >= 2 {
                    let size = parts[1];
                    let css_value = if let TokenValue::Number(num) = value {
                        format!("{}px", num)
                    } else {
                        value.to_css_value()
                    };

                    // 生成 padding 和 margin 类
                    utilities.push(self.format_css_rule(
                        &format!(".{}-p-{}", self.prefix, size),
                        &[format!("padding: {}", css_value)],
                    ));
                    utilities.push(self.format_css_rule(
                        &format!(".{}-m-{}", self.prefix, size),
                        &[format!("margin: {}", css_value)],
                    ));
                    utilities.push(self.format_css_rule(
                        &format!(".{}-px-{}", self.prefix, size),
                        &[format!(
                            "padding-left: {}; padding-right: {}",
                            css_value, css_value
                        )],
                    ));
                    utilities.push(self.format_css_rule(
                        &format!(".{}-py-{}", self.prefix, size),
                        &[format!(
                            "padding-top: {}; padding-bottom: {}",
                            css_value, css_value
                        )],
                    ));
                    utilities.push(self.format_css_rule(
                        &format!(".{}-mx-{}", self.prefix, size),
                        &[format!(
                            "margin-left: {}; margin-right: {}",
                            css_value, css_value
                        )],
                    ));
                    utilities.push(self.format_css_rule(
                        &format!(".{}-my-{}", self.prefix, size),
                        &[format!(
                            "margin-top: {}; margin-bottom: {}",
                            css_value, css_value
                        )],
                    ));
                }
            }
        }

        let separator = if self.minify { "" } else { "\n" };
        Ok(utilities.join(separator))
    }

    /// 生成字体实用工具类
    fn generate_typography_utilities(&mut self, theme: ThemeVariant) -> Result<String, String> {
        let typography_paths = self.get_paths_by_category("typography", theme);
        let mut utilities = Vec::new();

        for path in typography_paths {
            if let Ok(value) = self.resolver.resolve_token(&path, theme) {
                let path_str = path.to_string();
                let parts: Vec<&str> = path_str.split('.').collect();

                if parts.len() >= 3 {
                    let property = parts[1];
                    let size = parts[2];

                    let (css_property, css_value) = match property {
                        "fontSize" => {
                            let val = if let TokenValue::Number(num) = value {
                                format!("{}px", num)
                            } else {
                                value.to_css_value()
                            };
                            ("font-size", val)
                        }
                        "fontWeight" => ("font-weight", value.to_css_value()),
                        "lineHeight" => ("line-height", value.to_css_value()),
                        _ => continue,
                    };

                    let class_name = format!(
                        ".{}-{}-{}",
                        self.prefix,
                        property.replace("font", "text").to_lowercase(),
                        size
                    );
                    utilities.push(self.format_css_rule(
                        &class_name,
                        &[format!("{}: {}", css_property, css_value)],
                    ));
                }
            }
        }

        let separator = if self.minify { "" } else { "\n" };
        Ok(utilities.join(separator))
    }

    /// 根据类别获取令牌路径
    fn get_paths_by_category(&self, category: &str, theme: ThemeVariant) -> Vec<TokenPath> {
        self.resolver
            .list_token_paths(theme)
            .into_iter()
            .filter(|path| path.segments.first().map(|s| s.as_str()) == Some(category))
            .collect()
    }

    // /// 获取解析器的引用
    // pub fn get_resolver(&self) -> &TokenResolver {
    //     &self.resolver
    // }

    // /// 获取解析器的可变引用
    // pub fn get_resolver_mut(&mut self) -> &mut TokenResolver {
    //     &mut self.resolver
    // }
}

#[cfg(test)]
mod tests {
    use super::super::token_values::DesignTokens;
    use super::*;
    use crate::theme::token_values::DesignTokens;

    #[test]
    fn test_css_variable_generation() {
        let mut store = DesignTokens::default();
        // 添加一些测试令牌
        let light_values = DesignTokens::new().get_light_theme_values();
        for (path, value) in light_values {
            store.set_value(&path.to_string(), value.to_string());
        }
        let resolver = TokenResolver::new(store);
        let mut generator = CssGenerator::new(resolver);

        let css = generator.generate_css_variables(ThemeVariant::Light);
        assert!(css.is_ok());

        let css_content = css.unwrap();
        assert!(css_content.contains(":root"));
        assert!(css_content.contains("--ant-color-primary-500"));
    }

    #[test]
    fn test_theme_css_generation() {
        let store = DesignTokens::new().create_store();
        let resolver = TokenResolver::new(store);
        let mut generator = CssGenerator::new(resolver);

        let css = generator.generate_theme_css();
        assert!(css.is_ok());

        let css_content = css.unwrap();
        assert!(css_content.contains(":root"));
        assert!(css_content.contains("[data-theme='dark']"));
        assert!(css_content.contains("prefers-color-scheme"));
    }

    #[test]
    fn test_utility_classes_generation() {
        let store = DesignTokens::new().create_store();
        let resolver = TokenResolver::new(store);
        let mut generator = CssGenerator::new(resolver);

        let css = generator.generate_utility_classes(ThemeVariant::Light);
        assert!(css.is_ok());

        let css_content = css.unwrap();
        // 应该包含颜色、间距和字体实用工具类
        assert!(css_content.len() > 0);
    }
}
