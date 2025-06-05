use crate::theme::{
    adapter::provider::ThemeProviderAdapter, core::css::CssGenerator, theme_types::Theme,
};

/// React 框架适配器
///
/// 提供与 React 框架的集成
#[derive(Debug)]
pub struct ReactAdapter {
    /// 主题提供者
    provider: ThemeProviderAdapter,
    /// CSS 生成器
    css_generator: CssGenerator,
    /// 是否启用自动注入
    auto_inject: bool,
}

/// React 组件样式
#[derive(Debug, Clone)]
pub struct ReactComponentStyle {
    /// 类名
    pub class_name: String,
    /// CSS 内容
    pub css: String,
    /// 组件名称
    pub component_name: String,
    /// 是否已注入
    pub injected: bool,
}

impl ReactAdapter {
    /// 创建新的 React 适配器
    pub fn new(provider: ThemeProviderAdapter) -> Self {
        Self {
            provider,
            css_generator: CssGenerator::new(),
            auto_inject: true,
        }
    }

    /// 设置是否启用自动注入
    pub fn with_auto_inject(mut self, auto_inject: bool) -> Self {
        self.auto_inject = auto_inject;
        self
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<Theme> {
        self.provider.get_theme()
    }

    /// 生成组件样式
    pub fn style_component(&self, component_name: &str, css_rules: &str) -> ReactComponentStyle {
        // 生成唯一类名
        let class_name = format!("css-{}", self.generate_hash(component_name, css_rules));

        // 处理CSS规则
        let processed_css = self.process_css_rules(css_rules, &class_name);

        // 如果启用了自动注入，则注入样式
        let injected = if self.auto_inject {
            // 注入样式的逻辑（在实际实现中需要根据React的API进行适配）
            // 这里只是一个占位符
            true
        } else {
            false
        };

        ReactComponentStyle {
            class_name,
            css: processed_css,
            component_name: component_name.to_string(),
            injected,
        }
    }

    /// 处理CSS规则
    fn process_css_rules(&self, css_rules: &str, class_name: &str) -> String {
        // 将CSS规则包装在类选择器中
        format!(".{} {{ {} }}", class_name, css_rules)
    }

    /// 生成哈希
    fn generate_hash(&self, component_name: &str, css_rules: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        component_name.hash(&mut hasher);
        css_rules.hash(&mut hasher);

        if let Some(theme) = self.get_current_theme() {
            theme.name.hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    /// 切换主题
    pub fn toggle_theme(&mut self) {
        if let Err(e) = self.provider.toggle_theme_mode() {
            eprintln!("切换主题失败: {}", e);
        }
    }

    /// 设置主题
    pub fn set_theme(&mut self, theme: Theme) -> Result<(), String> {
        self.provider
            .switch_theme(&theme.name)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    /// 生成 React 样式对象
    pub fn create_style_object(&self, css_rules: &str) -> String {
        // 将CSS规则转换为React样式对象
        // 这里只是一个简单的实现，实际应该使用CSS解析器
        let mut style_object = String::from("{");

        for rule in css_rules.split(';') {
            let rule = rule.trim();
            if rule.is_empty() {
                continue;
            }

            if let Some((property, value)) = rule.split_once(':') {
                let property = property.trim();
                let value = value.trim();

                // 转换CSS属性为驼峰式
                let property = self.to_camel_case(property);

                style_object.push_str(&format!("{}: \"{}\", ", property, value));
            }
        }

        style_object.push('}');
        style_object
    }

    /// 转换为驼峰式命名
    fn to_camel_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for c in s.chars() {
            if c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

/// React 钩子
///
/// 提供在 React 组件中使用主题的功能
pub mod hooks {
    use super::*;

    /// 使用主题
    ///
    /// 在 React 组件中获取当前主题
    pub fn use_theme(adapter: &ReactAdapter) -> Option<Theme> {
        adapter.get_current_theme()
    }

    /// 使用主题切换
    ///
    /// 在 React 组件中切换主题
    pub fn use_theme_toggle(adapter: &mut ReactAdapter) -> impl FnMut() + '_ {
        move || {
            adapter.toggle_theme();
        }
    }

    /// 使用样式化组件
    ///
    /// 在 React 组件中使用样式
    pub fn use_styled(adapter: &ReactAdapter, component_name: &str, css_rules: &str) -> String {
        adapter
            .style_component(component_name, css_rules)
            .class_name
    }

    /// 使用样式对象
    ///
    /// 在 React 组件中使用样式对象
    pub fn use_style_object(adapter: &ReactAdapter, css_rules: &str) -> String {
        adapter.create_style_object(css_rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::adapter::provider::{ThemeProviderAdapter, ThemeProviderConfig};
    use crate::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    use std::sync::Arc;

    #[test]
    fn test_react_adapter_basic() {
        // 创建适配器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));
        let provider = ThemeProviderAdapter::new(manager, ThemeProviderConfig::default());
        let adapter = ReactAdapter::new(provider);

        // 生成组件样式
        let style = adapter.style_component("button", "color: blue;");

        // 验证样式
        assert!(!style.class_name.is_empty());
        assert!(style.css.contains("color: blue;"));
        assert_eq!(style.component_name, "button");
    }

    #[test]
    fn test_create_style_object() {
        // 创建适配器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));
        let provider = ThemeProviderAdapter::new(manager, ThemeProviderConfig::default());
        let adapter = ReactAdapter::new(provider);

        // 生成样式对象
        let style_object = adapter.create_style_object("color: blue; margin-top: 10px;");

        // 验证样式对象
        assert!(style_object.contains("color: \"blue\""));
        assert!(style_object.contains("marginTop: \"10px\""));
    }
}
