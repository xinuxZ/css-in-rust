use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    Expr, Ident, LitStr,
};

/// 样式化组件输入
struct StyledComponentInput {
    component_name: Ident,
    tag_name: Ident,
    css: LitStr,
}

impl Parse for StyledComponentInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name: Ident = input.parse()?;
        input.parse::<Comma>()?;

        let tag_name: Ident = input.parse()?;
        input.parse::<Comma>()?;

        let css: LitStr = input.parse()?;

        Ok(StyledComponentInput {
            component_name,
            tag_name,
            css,
        })
    }
}

/// 创建样式化组件宏
///
/// 使用方式：
/// ```ignore
/// styled_component!(MyButton, button, "background-color: blue; color: white;");
/// ```
pub fn styled_component_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentInput);

    let component_name = &input.component_name;
    let tag_name = &input.tag_name;
    let css = &input.css;

    let output = quote! {
        #[component]
        pub fn #component_name(cx: Scope<dioxus::prelude::Props>) -> dioxus::prelude::Element {
            use css_in_rust::theme::dioxus::use_style;

            let class_name = use_style(cx, #css);

            cx.render(rsx! {
                #tag_name {
                    class: class_name,
                    &cx.props.children
                }
            })
        }
    };

    output.into()
}

/// 样式化组件输入（带属性）
struct StyledComponentWithPropsInput {
    component_name: Ident,
    tag_name: Ident,
    props_type: syn::Type,
    css: LitStr,
}

impl Parse for StyledComponentWithPropsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name: Ident = input.parse()?;
        input.parse::<Comma>()?;

        let tag_name: Ident = input.parse()?;
        input.parse::<Comma>()?;

        let props_type: syn::Type = input.parse()?;
        input.parse::<Comma>()?;

        let css: LitStr = input.parse()?;

        Ok(StyledComponentWithPropsInput {
            component_name,
            tag_name,
            props_type,
            css,
        })
    }
}

/// 创建带属性的样式化组件宏
///
/// 使用方式：
/// ```ignore
/// styled_component_with_props!(MyButton, button, ButtonProps, "background-color: blue; color: white;");
/// ```
pub fn styled_component_with_props_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentWithPropsInput);

    let component_name = &input.component_name;
    let tag_name = &input.tag_name;
    let props_type = &input.props_type;
    let css = &input.css;

    let output = quote! {
        #[component]
        pub fn #component_name(cx: Scope<css_in_rust::theme::dioxus::styled::StyledProps<#props_type>>) -> dioxus::prelude::Element {
            use css_in_rust::theme::dioxus::use_style;

            // 应用基本样式
            let mut class_name = use_style(cx, #css);

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
                #tag_name {
                    ..props,
                    &cx.props.children
                }
            })
        }
    };

    output.into()
}

/// 主题化样式输入
struct ThemedStyleInput {
    css_expr: Expr,
}

impl Parse for ThemedStyleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let css_expr = input.parse()?;
        Ok(ThemedStyleInput { css_expr })
    }
}

/// 主题化样式宏
///
/// 使用方式：
/// ```ignore
/// themed_style!(|theme| format!("color: {};", theme.colors.primary))
/// ```
pub fn themed_style_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ThemedStyleInput);
    let css_expr = &input.css_expr;

    let output = quote! {
        {
            use css_in_rust::theme::dioxus::use_themed_style;
            use_themed_style(cx, #css_expr)
        }
    };

    output.into()
}
