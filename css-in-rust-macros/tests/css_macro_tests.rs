//! CSS 宏单元测试
//!
//! 测试 css! 宏的各种使用场景
use lightningcss as _;
use proc_macro2 as _;
use quote as _;
use sha2 as _;
use syn as _;

use css_in_rust_macros::{css, css_class, css_if, css_multi_if};

#[cfg(test)]
mod css_macro_tests {
    use super::*;

    /// 测试基础 CSS 样式
    #[test]
    fn test_basic_css() {
        let class_name = css!("color: red; font-size: 16px;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试空 CSS 样式
    #[test]
    fn test_empty_css() {
        let class_name = css!("");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试复杂 CSS 样式
    #[test]
    fn test_complex_css() {
        let class_name = css!(
            r#"
            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
            border-radius: 8px;
            padding: 16px 24px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            transition: all 0.3s ease;
        "#
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试主题变量
    #[test]
    fn test_theme_variables() {
        let class_name = css!("color: var(--primary-color); font-size: var(--font-size-base);");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试变体语法 - hover 状态
    #[test]
    fn test_variant_syntax_hover() {
        let class_name = css!("hover:bg-primary-500 hover:text-white");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试变体语法 - 响应式
    #[test]
    fn test_variant_syntax_responsive() {
        let class_name = css!("sm:text-lg md:text-xl lg:text-2xl");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试变体语法 - 暗色模式
    #[test]
    fn test_variant_syntax_dark_mode() {
        let class_name = css!("dark:bg-gray-800 dark:text-white");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试混合语法 - 传统 CSS + 变体
    #[test]
    fn test_mixed_syntax() {
        let class_name = css!("color: blue; hover:bg-red-500 sm:text-lg");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试 Flexbox 布局
    #[test]
    fn test_flexbox_layout() {
        let class_name = css!(
            r#"
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            gap: 16px;
        "#
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试 Grid 布局
    #[test]
    fn test_grid_layout() {
        let class_name = css!(
            r#"
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            grid-gap: 16px;
            grid-auto-rows: minmax(100px, auto);
        "#
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试相同 CSS 内容的缓存机制
    #[test]
    fn test_css_caching() {
        let class_name1 = css!("color: red; font-size: 16px;");
        let class_name2 = css!("color: red; font-size: 16px;");

        // 相同的 CSS 内容应该生成相同的类名（缓存机制）
        assert_eq!(class_name1, class_name2);
    }

    /// 测试不同 CSS 内容生成不同类名
    #[test]
    fn test_different_css_different_classes() {
        let class_name1 = css!("color: red;");
        let class_name2 = css!("color: blue;");

        // 不同的 CSS 内容应该生成不同的类名
        assert_ne!(class_name1, class_name2);
    }
}

#[cfg(test)]
mod css_if_macro_tests {
    use super::*;

    /// 测试条件为真的情况
    #[test]
    fn test_css_if_true_condition() {
        let is_active = true;
        let class_name = css_if!(is_active, "background-color: blue;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试条件为假的情况
    #[test]
    fn test_css_if_false_condition() {
        let is_active = false;
        let class_name = css_if!(is_active, "background-color: blue;");
        assert_eq!(class_name, "");
    }

    /// 测试复杂条件表达式
    #[test]
    fn test_css_if_complex_condition() {
        let count = 5;
        let class_name = css_if!(count > 3, "color: green; font-weight: bold;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试布尔变量条件
    #[test]
    fn test_css_if_boolean_variable() {
        let is_highlighted = true;
        let is_disabled = false;

        let class_name1 = css_if!(is_highlighted, "background-color: yellow;");
        let class_name2 = css_if!(is_disabled, "opacity: 0.5;");

        assert!(!class_name1.is_empty());
        assert_eq!(class_name2, "");
    }
}

#[cfg(test)]
mod css_class_macro_tests {
    use super::*;

    /// 测试基础类名生成
    #[test]
    fn test_css_class_basic() {
        let class_name = css_class!("my-component");
        assert!(class_name.contains("my-component"));
        assert!(class_name.len() > "my-component".len()); // 应该包含哈希后缀
    }

    /// 测试相同类名生成相同结果
    #[test]
    fn test_css_class_consistency() {
        let class_name1 = css_class!("button");
        let class_name2 = css_class!("button");
        assert_eq!(class_name1, class_name2);
    }

    /// 测试不同类名生成不同结果
    #[test]
    fn test_css_class_uniqueness() {
        let class_name1 = css_class!("button");
        let class_name2 = css_class!("input");
        assert_ne!(class_name1, class_name2);
    }

    /// 测试复杂类名
    #[test]
    fn test_css_class_complex_name() {
        let class_name = css_class!("ant-design-button-primary");
        assert!(class_name.contains("ant-design-button-primary"));
    }
}

#[cfg(test)]
mod css_multi_if_macro_tests {
    use super::*;

    /// 测试 AND 条件 - 两个条件都为真
    #[test]
    fn test_css_multi_if_and_both_true() {
        let is_active = true;
        let is_large = true;
        let class_name = css_multi_if!(
            is_active & is_large,
            "background-color: blue; font-size: 18px;"
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-multi-"));
    }

    /// 测试 AND 条件 - 一个条件为假
    #[test]
    fn test_css_multi_if_and_one_false() {
        let is_active = true;
        let is_large = false;
        let class_name = css_multi_if!(is_active & is_large, "background-color: blue;");
        assert_eq!(class_name, "");
    }

    /// 测试 OR 条件 - 一个条件为真
    #[test]
    fn test_css_multi_if_or_one_true() {
        let is_active = true;
        let is_large = false;
        let class_name = css_multi_if!(is_active | is_large, "background-color: green;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-multi-"));
    }

    /// 测试 OR 条件 - 两个条件都为假
    #[test]
    fn test_css_multi_if_or_both_false() {
        let is_active = false;
        let is_large = false;
        let class_name = css_multi_if!(is_active | is_large, "background-color: green;");
        assert_eq!(class_name, "");
    }

    /// 测试复杂条件表达式
    #[test]
    fn test_css_multi_if_complex_condition() {
        let is_active = true;
        let is_large = false;
        let is_primary = true;

        let class_name = css_multi_if!(
            (is_active & !is_large) | (is_large & is_primary),
            "background-color: yellow; border: 2px solid red;"
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-multi-"));
    }

    /// 测试否定条件
    #[test]
    fn test_css_multi_if_negation() {
        let is_disabled = false;
        let is_hidden = false;

        let class_name = css_multi_if!(!is_disabled & !is_hidden, "display: block; opacity: 1;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-multi-"));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// 测试多个宏组合使用
    #[test]
    fn test_macro_combination() {
        let is_primary = true;
        let is_large = false;

        let base_class = css_class!("button");
        let style_class = css!("padding: 8px 16px; border-radius: 4px;");
        let conditional_class = css_if!(is_primary, "background-color: #007bff; color: white;");
        let multi_conditional_class =
            css_multi_if!(is_primary & !is_large, "font-size: 14px; font-weight: 500;");

        assert!(!base_class.is_empty());
        assert!(!style_class.is_empty());
        assert!(!conditional_class.is_empty());
        assert!(!multi_conditional_class.is_empty());

        // 所有类名都应该是不同的
        assert_ne!(base_class, style_class);
        assert_ne!(style_class, conditional_class);
        assert_ne!(conditional_class, multi_conditional_class);
    }

    /// 测试性能 - 大量相同 CSS 的缓存效果
    #[test]
    fn test_performance_caching() {
        // 生成多个相同的 CSS 类名，测试缓存效果
        let mut class_names = Vec::new();
        for _ in 0..10 {
            class_names.push(css!("color: red; font-size: 16px; padding: 8px;"));
        }

        // 所有类名应该相同（缓存生效）
        let first_class = &class_names[0];
        for class_name in &class_names {
            assert_eq!(class_name, first_class);
        }
    }

    /// 测试边界情况 - 特殊字符
    #[test]
    fn test_special_characters() {
        let class_name = css!(
            r#"
            content: "Hello, World!";
            font-family: "Helvetica Neue", Arial, sans-serif;
        "#
        );
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }

    /// 测试边界情况 - 长 CSS
    #[test]
    fn test_long_css() {
        let class_name = css!("--custom-property-0: value-0; --custom-property-1: value-1; --custom-property-2: value-2;");
        assert!(!class_name.is_empty());
        assert!(class_name.starts_with("css-"));
    }
}
