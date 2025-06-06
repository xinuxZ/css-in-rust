#[cfg(test)]
mod tests {
    use crate::theme::{
        adapter::{
            frameworks::DioxusAdapter,
            provider::{ThemeProviderAdapter, ThemeProviderConfig},
            ssr::SsrSupport,
        },
        core::manager::{ThemeManager, ThemeManagerConfig},
        theme_types::{Theme, ThemeMode},
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn test_theme_manager() {
        let config = crate::theme::core::manager::ThemeManagerConfig::default();
        let manager = ThemeManager::new(config);

        // 创建主题
        let mut theme = Theme::default();
        theme.name = "test-theme".to_string();
        theme.mode = ThemeMode::Dark;

        // 设置主题
        manager.set_theme(theme.clone()).unwrap();

        // 获取主题
        let current_theme = manager.get_current_theme().unwrap();
        assert_eq!(current_theme.name, "test-theme");
        assert_eq!(current_theme.mode, ThemeMode::Dark);

        // 切换主题模式
        manager.toggle_theme_mode();
        let current_theme = manager.get_current_theme().unwrap();
        assert_eq!(current_theme.mode, ThemeMode::Light);
    }

    #[test]
    fn test_ssr_support() {
        let ssr = SsrSupport::new();

        // 创建样式
        let mut styles = HashMap::new();
        styles.insert("button".to_string(), ".button { color: blue; }".to_string());
        styles.insert(
            "input".to_string(),
            ".input { border: 1px solid gray; }".to_string(),
        );

        // 渲染样式
        let result = ssr.render_styles(styles);

        // 验证结果
        assert!(!result.css.is_empty());
        assert!(!result.hash.is_empty());
        assert!(result.style_id.starts_with("ssr-styles-"));

        // 生成样式标签
        let tag = ssr.generate_style_tag(&result);
        assert!(tag.starts_with("<style"));
        assert!(tag.contains("data-ssr=\"true\""));
        assert!(tag.ends_with("</style>"));
    }

    #[test]
    fn test_framework_adapters() {
        // 创建主题提供者适配器
        let manager = Arc::new(ThemeManager::new(ThemeManagerConfig::default()));
        let provider = ThemeProviderAdapter::new(manager, ThemeProviderConfig::default());

        // 测试 Dioxus 适配器
        let dioxus_adapter = DioxusAdapter::new(provider.clone());
        let dioxus_style = dioxus_adapter.style_component("button", "color: blue;");
        assert!(!dioxus_style.class_name.is_empty());
        assert!(dioxus_style.css.contains("color: blue;"));
    }
}
