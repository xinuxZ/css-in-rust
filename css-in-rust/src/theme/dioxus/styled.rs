use super::hooks::use_style;
use dioxus::prelude::*;

/// 样式化元素属性
#[derive(Props, PartialEq)]
pub struct StyledProps<P: PartialEq> {
    /// 基础组件属性
    #[props(flatten)]
    pub props: P,

    /// CSS样式
    pub css: String,

    /// 条件样式列表
    #[props(default)]
    pub conditional_styles: Vec<(bool, String)>,

    /// 子元素
    #[props(default)]
    pub children: Element,
}

/// 样式化div组件
#[component]
pub fn StyledDiv(cx: Scope<StyledProps<DivProps>>) -> Element {
    // 应用基本样式
    let mut class_name = use_style(cx, &cx.props.css);

    // 应用条件样式
    for (condition, style) in &cx.props.conditional_styles {
        if *condition {
            let condition_class = use_style(cx, style);
            class_name = format!("{} {}", class_name, condition_class);
        }
    }

    // 获取原始属性
    let mut props = cx.props.props.clone();

    // 合并类名
    if let Some(existing_class) = props.class.clone() {
        props.class = Some(format!("{} {}", existing_class, class_name));
    } else {
        props.class = Some(class_name);
    }

    cx.render(rsx! {
        div {
            ..props,
            &cx.props.children
        }
    })
}

/// 样式化按钮组件
#[component]
pub fn StyledButton(cx: Scope<StyledProps<ButtonProps>>) -> Element {
    // 应用基本样式
    let mut class_name = use_style(cx, &cx.props.css);

    // 应用条件样式
    for (condition, style) in &cx.props.conditional_styles {
        if *condition {
            let condition_class = use_style(cx, style);
            class_name = format!("{} {}", class_name, condition_class);
        }
    }

    // 获取原始属性
    let mut props = cx.props.props.clone();

    // 合并类名
    if let Some(existing_class) = props.class.clone() {
        props.class = Some(format!("{} {}", existing_class, class_name));
    } else {
        props.class = Some(class_name);
    }

    cx.render(rsx! {
        button {
            ..props,
            &cx.props.children
        }
    })
}

/// 样式化span组件
#[component]
pub fn StyledSpan(cx: Scope<StyledProps<SpanProps>>) -> Element {
    // 应用基本样式
    let mut class_name = use_style(cx, &cx.props.css);

    // 应用条件样式
    for (condition, style) in &cx.props.conditional_styles {
        if *condition {
            let condition_class = use_style(cx, style);
            class_name = format!("{} {}", class_name, condition_class);
        }
    }

    // 获取原始属性
    let mut props = cx.props.props.clone();

    // 合并类名
    if let Some(existing_class) = props.class.clone() {
        props.class = Some(format!("{} {}", existing_class, class_name));
    } else {
        props.class = Some(class_name);
    }

    cx.render(rsx! {
        span {
            ..props,
            &cx.props.children
        }
    })
}

/// 创建样式化组件
///
/// 用于创建自定义样式化组件
#[macro_export]
macro_rules! styled_component {
    ($name:ident, $tag:ident, $props_type:ty) => {
        #[component]
        pub fn $name(cx: Scope<StyledProps<$props_type>>) -> Element {
            // 应用基本样式
            let mut class_name = use_style(cx, &cx.props.css);

            // 应用条件样式
            for (condition, style) in &cx.props.conditional_styles {
                if *condition {
                    let condition_class = use_style(cx, style);
                    class_name = format!("{} {}", class_name, condition_class);
                }
            }

            // 获取原始属性
            let mut props = cx.props.props.clone();

            // 合并类名
            if let Some(existing_class) = props.class.clone() {
                props.class = Some(format!("{} {}", existing_class, class_name));
            } else {
                props.class = Some(class_name);
            }

            cx.render(rsx! {
                $tag {
                    ..props,
                    &cx.props.children
                }
            })
        }
    };
}
