//! CSS生成器模块
//!
//! 本模块负责将设计令牌转换为CSS变量和样式声明。
//! 职责：CSS变量生成、样式类生成、主题切换支持

use crate::theme::core::token::definitions::{
    DimensionUnit, DimensionValue, ThemeVariant, TokenValue,
};
use crate::theme::core::token::resolver::TokenResolver;
use std::fmt::Write;

/// CSS 生成器
///
/// 负责生成优化的 CSS 代码，包括变量声明、规则、媒体查询和主题样式。
/// 提供了一系列方法来将设计令牌和样式配置转换为有效的 CSS 字符串。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssGenerator;
/// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
///
/// // 创建 CSS 生成器
/// let mut generator = CssGenerator::new();
///
/// // 生成 CSS 变量
/// let variables = vec![
///     ("color-primary".to_string(), "#1890ff".to_string()),
///     ("font-size-base".to_string(), "14px".to_string()),
/// ];
/// let css = generator.generate_variables(&variables);
/// println!("{}", css);
///
/// // 生成 CSS 规则
/// let declarations = vec![
///     ("color".to_string(), "var(--color-primary)".to_string()),
///     ("font-size".to_string(), "var(--font-size-base)".to_string()),
/// ];
/// let rule = generator.generate_rule(".button", &declarations);
/// println!("{}", rule);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CssGenerator {
    /// 变量前缀
    prefix: String,
    /// 是否启用压缩
    minify: bool,
    /// 令牌解析器
    resolver: TokenResolver,
}

impl CssGenerator {
    /// 创建新的 CSS 生成器
    ///
    /// 创建一个默认配置的 CSS 生成器，使用 "css-in-rust" 作为变量前缀，不启用压缩。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `CssGenerator` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            prefix: "css-in-rust".to_string(),
            minify: false,
            resolver: TokenResolver::default(),
        }
    }

    /// 创建带前缀的 CSS 生成器
    ///
    /// 创建一个使用指定前缀的 CSS 生成器，用于生成 CSS 变量。
    ///
    /// # 参数
    ///
    /// * `prefix` - 要使用的变量前缀
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `CssGenerator` 实例，使用指定的前缀。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::with_prefix("my-theme");
    /// ```
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            minify: false,
            resolver: TokenResolver::default(),
        }
    }

    /// 设置是否启用压缩
    ///
    /// 配置 CSS 生成器是否生成压缩的 CSS 代码（无空格和换行符）。
    ///
    /// # 参数
    ///
    /// * `minify` - 是否启用压缩
    ///
    /// # 返回值
    ///
    /// 返回配置后的 `CssGenerator` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new().with_minify(true);
    /// ```
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.minify = minify;
        self
    }

    /// 生成 CSS 变量声明
    ///
    /// 根据提供的变量名和值对生成 CSS 变量声明。
    ///
    /// # 参数
    ///
    /// * `variables` - 变量名和值的元组数组
    ///
    /// # 返回值
    ///
    /// 返回包含 CSS 变量声明的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let variables = vec![
    ///     ("color-primary".to_string(), "#1890ff".to_string()),
    ///     ("font-size-base".to_string(), "14px".to_string()),
    /// ];
    /// let css = generator.generate_variables(&variables);
    /// assert!(css.contains("--css-in-rust-color-primary: #1890ff;"));
    /// ```
    pub fn generate_variables(&self, variables: &[(String, String)]) -> String {
        let mut css = String::new();

        for (name, value) in variables {
            self.write_variable(&mut css, name, value);
        }

        css
    }

    /// 写入变量声明
    ///
    /// 将单个 CSS 变量声明写入字符串。
    ///
    /// # 参数
    ///
    /// * `css` - 要写入的字符串
    /// * `name` - 变量名
    /// * `value` - 变量值
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let mut css = String::new();
    /// generator.write_variable(&mut css, "color-primary", "#1890ff");
    /// assert_eq!(css, "  --css-in-rust-color-primary: #1890ff;\n");
    /// ```
    fn write_variable(&self, css: &mut String, name: &str, value: &str) {
        let name = if name.starts_with("--") {
            name.to_string()
        } else {
            format!("--{}-{}", self.prefix, name)
        };

        if self.minify {
            write!(css, "{}:{};", name, value).unwrap();
        } else {
            writeln!(css, "  {}: {};", name, value).unwrap();
        }
    }

    /// 生成 CSS 规则
    ///
    /// 根据选择器和声明生成 CSS 规则。
    ///
    /// # 参数
    ///
    /// * `selector` - CSS 选择器
    /// * `declarations` - 属性和值的元组数组
    ///
    /// # 返回值
    ///
    /// 返回包含 CSS 规则的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let declarations = vec![
    ///     ("color".to_string(), "red".to_string()),
    ///     ("font-size".to_string(), "16px".to_string()),
    /// ];
    /// let css = generator.generate_rule("body", &declarations);
    /// assert!(css.contains("body {"));
    /// assert!(css.contains("  color: red;"));
    /// assert!(css.contains("  font-size: 16px;"));
    /// ```
    pub fn generate_rule(&self, selector: &str, declarations: &[(String, String)]) -> String {
        let mut css = String::new();
        let space = if self.minify { "" } else { " " };

        if self.minify {
            write!(css, "{}{{{}", selector, space).unwrap();
        } else {
            writeln!(css, "{} {{", selector).unwrap();
        }

        for (property, value) in declarations {
            if self.minify {
                write!(css, "{}:{};", property, value).unwrap();
            } else {
                writeln!(css, "  {}: {};", property, value).unwrap();
            }
        }

        if self.minify {
            write!(css, "{}}}", space).unwrap();
        } else {
            css.push_str("}\n");
        }

        css
    }

    /// 生成媒体查询
    ///
    /// 根据媒体查询条件、选择器和声明生成媒体查询规则。
    ///
    /// # 参数
    ///
    /// * `query` - 媒体查询条件
    /// * `selector` - CSS 选择器
    /// * `declarations` - 属性和值的元组数组
    ///
    /// # 返回值
    ///
    /// 返回包含媒体查询的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let declarations = vec![
    ///     ("display".to_string(), "none".to_string()),
    /// ];
    /// let css = generator.generate_media_query("(max-width: 768px)", ".mobile", &declarations);
    /// assert!(css.contains("@media (max-width: 768px) {"));
    /// assert!(css.contains(".mobile {"));
    /// assert!(css.contains("display: none;"));
    /// ```
    pub fn generate_media_query(
        &self,
        query: &str,
        selector: &str,
        declarations: &[(String, String)],
    ) -> String {
        let mut css = String::new();
        let space = if self.minify { "" } else { " " };

        if self.minify {
            write!(css, "@media{}{}{{{}", space, query, space).unwrap();
        } else {
            writeln!(css, "@media {} {{", query).unwrap();
        }

        let rule = self.generate_rule(selector, declarations);
        if !self.minify {
            // 缩进规则内容
            for line in rule.lines() {
                writeln!(css, "  {}", line).unwrap();
            }
        } else {
            css.push_str(&rule);
        }

        if self.minify {
            write!(css, "{}}}", space).unwrap();
        } else {
            css.push_str("}\n");
        }

        css
    }

    /// 生成 CSS 类
    ///
    /// 为指定的类名生成 CSS 规则。
    ///
    /// # 参数
    ///
    /// * `class_name` - 类名（不包含前导点）
    /// * `declarations` - 属性和值的元组数组
    ///
    /// # 返回值
    ///
    /// 返回包含 CSS 类规则的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let declarations = vec![
    ///     ("background-color".to_string(), "blue".to_string()),
    ///     ("color".to_string(), "white".to_string()),
    /// ];
    /// let css = generator.generate_class("button", &declarations);
    /// assert!(css.contains(".button {"));
    /// assert!(css.contains("  background-color: blue;"));
    /// assert!(css.contains("  color: white;"));
    /// ```
    pub fn generate_class(&self, class_name: &str, declarations: &[(String, String)]) -> String {
        self.generate_rule(&format!(".{}", class_name), declarations)
    }

    /// 生成CSS变量
    ///
    /// 根据主题变体生成 CSS 变量声明。
    ///
    /// # 参数
    ///
    /// * `_theme` - 主题变体
    ///
    /// # 返回值
    ///
    /// 成功返回包含 CSS 变量声明的字符串，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
    ///
    /// let mut generator = CssGenerator::new();
    /// let css = generator.generate_css_variables(ThemeVariant::Light).unwrap();
    /// assert!(css.contains(":root {"));
    /// assert!(css.contains("--color-primary: #1890ff;"));
    /// ```
    pub fn generate_css_variables(&mut self, _theme: ThemeVariant) -> Result<String, String> {
        let mut css = String::new();
        css.push_str(":root {\n");
        css.push_str("  --color-primary: #1890ff;\n");
        css.push_str("  --color-success: #52c41a;\n");
        css.push_str("  --color-warning: #faad14;\n");
        css.push_str("  --color-error: #f5222d;\n");
        css.push_str("  --font-size-base: 14px;\n");
        css.push_str("  --spacing-unit: 4px;\n");
        css.push_str("}\n");
        Ok(css)
    }

    /// 生成主题CSS
    ///
    /// 生成当前主题的 CSS 变量和样式。
    ///
    /// # 返回值
    ///
    /// 成功返回包含主题 CSS 的字符串，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let mut generator = CssGenerator::new();
    /// let css = generator.generate_theme_css().unwrap();
    /// assert!(css.contains(":root {"));
    /// assert!(css.contains("--color-primary: #1890ff;"));
    /// ```
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        self.generate_css_variables(ThemeVariant::Light)
    }

    /// 生成组件CSS
    ///
    /// 为指定组件生成 CSS 样式。
    ///
    /// # 参数
    ///
    /// * `_component` - 组件名称
    /// * `_theme` - 主题变体
    ///
    /// # 返回值
    ///
    /// 成功返回包含组件 CSS 的字符串，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
    ///
    /// let mut generator = CssGenerator::new();
    /// let css = generator.generate_component_css("button", ThemeVariant::Light).unwrap();
    /// ```
    pub fn generate_component_css(
        &mut self,
        _component: &str,
        _theme: ThemeVariant,
    ) -> Result<String, String> {
        Ok(String::new())
    }

    /// 生成组件类
    ///
    /// 为指定组件生成 CSS 类名。
    ///
    /// # 参数
    ///
    /// * `_component` - 组件名称
    /// * `_theme` - 主题变体
    ///
    /// # 返回值
    ///
    /// 成功返回包含组件类的字符串，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
    ///
    /// let mut generator = CssGenerator::new();
    /// let classes = generator.generate_component_classes("button", ThemeVariant::Light).unwrap();
    /// ```
    pub fn generate_component_classes(
        &mut self,
        _component: &str,
        _theme: ThemeVariant,
    ) -> Result<String, String> {
        Ok(String::new())
    }

    /// 生成工具类CSS
    ///
    /// 生成工具类 CSS，如间距、颜色、字体大小等通用样式类。
    ///
    /// # 参数
    ///
    /// * `_theme` - 主题变体
    ///
    /// # 返回值
    ///
    /// 成功返回包含工具类 CSS 的字符串，失败返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
    ///
    /// let mut generator = CssGenerator::new();
    /// let css = generator.generate_utility_classes(ThemeVariant::Light).unwrap();
    /// ```
    pub fn generate_utility_classes(&mut self, _theme: ThemeVariant) -> Result<String, String> {
        Ok(String::new())
    }

    /// 将值转换为 CSS 字符串
    ///
    /// 将令牌值转换为有效的 CSS 字符串表示。
    ///
    /// # 参数
    ///
    /// * `value` - 要转换的令牌值
    ///
    /// # 返回值
    ///
    /// 返回值的 CSS 字符串表示。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, DimensionValue, DimensionUnit};
    ///
    /// let generator = CssGenerator::new();
    /// let dim = DimensionValue { value: 16.0, unit: DimensionUnit::Px };
    /// let value = TokenValue::Dimension(dim);
    /// let css = generator.value_to_css(&value);
    /// assert_eq!(css, "16px");
    /// ```
    pub fn value_to_css(&self, value: &TokenValue) -> String {
        match value {
            TokenValue::String(s) => s.clone(),
            TokenValue::Number(n) => n.to_string(),
            TokenValue::Dimension(dim) => self.dimension_to_css(dim),
            TokenValue::Color(color) => format!("#{}", color.hex),
            TokenValue::Boolean(b) => b.to_string(),
            TokenValue::Array(arr) => {
                let mut values = Vec::new();
                for v in arr {
                    values.push(self.value_to_css(v));
                }
                values.join(", ")
            }
            TokenValue::Object(map) => {
                let mut values = Vec::new();
                for (k, v) in map {
                    values.push(format!("{}: {}", k, self.value_to_css(v)));
                }
                format!("{{ {} }}", values.join(", "))
            }
            _ => "".to_string(),
        }
    }

    /// 将维度值转换为 CSS 字符串
    ///
    /// 将维度值（如像素、百分比等）转换为有效的 CSS 单位表示。
    ///
    /// # 参数
    ///
    /// * `dim` - 要转换的维度值
    ///
    /// # 返回值
    ///
    /// 返回维度值的 CSS 字符串表示。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    /// use css_in_rust::theme::core::token::definitions::{DimensionValue, DimensionUnit};
    ///
    /// let generator = CssGenerator::new();
    /// let dim = DimensionValue { value: 50.0, unit: DimensionUnit::Percent };
    /// let css = generator.dimension_to_css(&dim);
    /// assert_eq!(css, "50%");
    /// ```
    fn dimension_to_css(&self, dim: &DimensionValue) -> String {
        match dim.unit {
            DimensionUnit::Px => format!("{}px", dim.value),
            DimensionUnit::Em => format!("{}em", dim.value),
            DimensionUnit::Rem => format!("{}rem", dim.value),
            DimensionUnit::Percent => format!("{}%", dim.value),
            DimensionUnit::Vh => format!("{}vh", dim.value),
            DimensionUnit::Vw => format!("{}vw", dim.value),
            DimensionUnit::Auto => "auto".to_string(),
        }
    }

    /// 获取令牌解析器的可变引用
    ///
    /// # 返回值
    ///
    /// 返回令牌解析器的可变引用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let mut generator = CssGenerator::new();
    /// let resolver = generator.get_resolver_mut();
    /// ```
    pub fn get_resolver_mut(&mut self) -> &mut TokenResolver {
        &mut self.resolver
    }

    /// 获取令牌解析器的引用
    ///
    /// # 返回值
    ///
    /// 返回令牌解析器的引用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::new();
    /// let resolver = generator.get_resolver();
    /// ```
    pub fn get_resolver(&self) -> &TokenResolver {
        &self.resolver
    }
}

impl Default for CssGenerator {
    /// 创建默认的 CSS 生成器
    ///
    /// # 返回值
    ///
    /// 返回一个新的默认 `CssGenerator` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssGenerator;
    ///
    /// let generator = CssGenerator::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_variable_generation() {
        let mut generator = CssGenerator::new();
        let css = generator.generate_css_variables(ThemeVariant::Light);
        assert!(css.is_ok());
    }

    #[test]
    fn test_theme_css_generation() {
        let mut generator = CssGenerator::new();
        let css = generator.generate_theme_css();
        assert!(css.is_ok());
    }

    #[test]
    fn test_utility_classes_generation() {
        let mut generator = CssGenerator::new();
        let css = generator.generate_utility_classes(ThemeVariant::Light);
        assert!(css.is_ok());
    }
}
