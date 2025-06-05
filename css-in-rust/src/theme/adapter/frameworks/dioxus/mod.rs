use crate::theme::{
    adapter::provider::ThemeProviderAdapter, core::css::CssGenerator, theme_types::Theme,
};

/// Dioxus 框架适配器
///
/// 提供与 Dioxus 框架的集成
#[derive(Debug)]
pub struct DioxusAdapter {
    /// 主题提供者
    provider: ThemeProviderAdapter,
    /// CSS 生成器
    css_generator: CssGenerator,

    /// 是否启用自动注入
    auto_inject: bool,
}

/// Dioxus 组件样式
#[derive(Debug, Clone)]
pub struct DioxusComponentStyle {
    /// 类名
    pub class_name: String,
    /// CSS 内容
    pub css: String,
    /// 组件名称
    pub component_name: String,
}

impl DioxusAdapter {
    /// 创建新的 Dioxus 适配器
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
    pub fn style_component(&self, component_name: &str, css_rules: &str) -> DioxusComponentStyle {
        // 生成唯一类名
        let class_name = format!("css-{}", self.generate_hash(component_name, css_rules));

        // 处理CSS规则
        let processed_css = self.process_css_rules(css_rules, &class_name);

        // 如果启用了自动注入，则注入样式
        if self.auto_inject {
            // 注入样式的逻辑（在实际实现中需要根据Dioxus的API进行适配）
            // 这里只是一个占位符
        }

        DioxusComponentStyle {
            class_name,
            css: processed_css,
            component_name: component_name.to_string(),
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
}

/// Dioxus 主题钩子
///
/// 提供在 Dioxus 组件中使用主题的功能
pub mod hooks {
    use super::*;

    /// 使用主题
    ///
    /// 在 Dioxus 组件中获取当前主题
    pub fn use_theme(adapter: &DioxusAdapter) -> Option<Theme> {
        adapter.get_current_theme()
    }

    /// 使用主题切换
    ///
    /// 在 Dioxus 组件中切换主题
    pub fn use_theme_toggle(adapter: &mut DioxusAdapter) -> impl FnMut() + '_ {
        move || {
            adapter.toggle_theme();
        }
    }

    /// 使用组件样式
    ///
    /// 在 Dioxus 组件中使用样式
    pub fn use_styled(adapter: &DioxusAdapter, component_name: &str, css_rules: &str) -> String {
        adapter
            .style_component(component_name, css_rules)
            .class_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::adapter::provider::{ThemeProviderAdapter, ThemeProviderConfig};
    use crate::theme::core::manager::{ThemeManager, ThemeManagerConfig};
    use std::sync::Arc;

    #[test]
    fn test_dioxus_adapter_basic() {
        // 创建适配器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));
        let provider = ThemeProviderAdapter::new(manager, ThemeProviderConfig::default());
        let adapter = DioxusAdapter::new(provider);

        // 生成组件样式
        let style = adapter.style_component("button", "color: blue;");

        // 验证样式
        assert!(!style.class_name.is_empty());
        assert!(style.css.contains("color: blue;"));
        assert_eq!(style.component_name, "button");
    }
}
