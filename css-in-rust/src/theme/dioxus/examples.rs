use crate::theme::core::{
    CssCalculator, CssObject, LogicalPropertiesTransformer, NumCalculator, Px2RemTransformer,
    StylePipeline, UnitConverter,
};
use crate::theme::dioxus::{use_style, use_theme, ThemeProvider};
use dioxus::prelude::*;

/// 使用转换器的示例
#[component]
pub fn TransformerExample() -> Element {
    // 创建样式处理管道
    let mut pipeline = StylePipeline::new();

    // 注册转换器
    pipeline.register_transformer(LogicalPropertiesTransformer::new());
    pipeline.register_transformer(Px2RemTransformer::new(16.0, 5, false));

    // 创建 CSS 对象
    let mut css_obj = CssObject::new();
    css_obj.insert("marginInline", "16px");
    css_obj.insert("paddingBlock", "1rem");
    css_obj.insert("color", "var(--primary-color)");

    // 处理 CSS 对象
    let processed = pipeline.process(css_obj).unwrap();

    // 使用处理后的 CSS
    let class_name = processed.class_name.clone();
    let css = processed.css.clone();

    // 使用 use_style 钩子注入样式
    let style_class = use_style(&css);

    render! {
        div {
            class: "{style_class}",
            "使用转换器的示例"
        }
    }
}

/// 使用计算器的示例
#[component]
pub fn CalculatorExample() -> Element {
    // 使用 CSS 计算器
    let calc = CssCalculator::new("100%").subtract("20px").add("10px");

    let width = calc.calc();

    // 使用数值计算器
    let num_calc = NumCalculator::new(16.0).multiply(1.5).precision(1);

    let font_size = num_calc.to_string_with_unit("px");

    // 使用单位转换器
    let converter = UnitConverter::default();
    let margin = converter
        .convert_value_str(
            "16px",
            crate::theme::core::calc::unit_converter::CssUnit::Rem,
        )
        .unwrap();

    // 创建样式
    let css = format!(
        r#"
        width: {};
        font-size: {};
        margin: {};
        "#,
        width, font_size, margin
    );

    let style_class = use_style(&css);

    render! {
        div {
            class: "{style_class}",
            "使用计算器的示例"
        }
    }
}

/// SSR 示例（需要在服务端使用）
#[component]
pub fn SsrExample() -> Element {
    // 获取主题
    let theme = use_theme();

    // 创建样式
    let css = r#"
        color: var(--primary-color);
        background-color: var(--background-color);
        padding: 16px;
        border-radius: 4px;
        margin: 8px 0;
    "#;

    let style_class = use_style(css);

    render! {
        div {
            class: "{style_class}",
            "SSR 示例"
        }
    }
}

/// 完整示例，结合所有功能
#[component]
pub fn CompleteExample() -> Element {
    render! {
        ThemeProvider {
            TransformerExample {}
            CalculatorExample {}
            SsrExample {}
        }
    }
}
