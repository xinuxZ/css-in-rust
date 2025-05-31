//! 简化的变体系统演示
//!
//! 展示基础的变体系统功能

use css_in_rust::variants::{
    conditional_styles::ConditionValue,
    responsive::Breakpoint,
    state_variants::StateType,
    variant_types::{ColorVariant, SizeVariant, StateVariant, VariantValue},
};
use std::collections::HashMap;

#[allow(unused_imports)]
use chrono as _;
#[allow(unused_imports)]
use css_in_rust_macros as _;
#[allow(unused_imports)]
use lazy_static as _;
#[allow(unused_imports)]
use lightningcss as _;
#[allow(unused_imports)]
use proc_macro2 as _;
#[allow(unused_imports)]
use quote as _;
#[allow(unused_imports)]
use regex as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use sha2 as _;
#[allow(unused_imports)]
use syn as _;
#[allow(unused_imports)]
use tempfile as _;

fn main() {
    println!("=== CSS-in-Rust 变体系统简化演示 ===");

    // 1. 基础变体类型演示
    demo_basic_variant_types();

    // 2. 断点系统演示
    demo_breakpoint_system();

    // 3. 状态类型演示
    demo_state_types();

    // 4. 条件值演示
    demo_condition_values();
}

/// 演示基础变体类型
fn demo_basic_variant_types() {
    println!("\n--- 基础变体类型演示 ---");

    // 尺寸变体
    let size_variants = vec![
        SizeVariant::XS,
        SizeVariant::SM,
        SizeVariant::MD,
        SizeVariant::LG,
        SizeVariant::XL,
    ];

    println!("可用的尺寸变体:");
    for variant in &size_variants {
        println!("  - {:?}", variant);
    }

    // 颜色变体
    let color_variants = vec![
        ColorVariant::Primary,
        ColorVariant::Secondary,
        ColorVariant::Success,
        ColorVariant::Warning,
        ColorVariant::Danger,
        ColorVariant::Info,
    ];

    println!("\n可用的颜色变体:");
    for variant in &color_variants {
        println!("  - {:?}", variant);
    }

    // 状态变体
    let state_variants = vec![
        StateVariant::Hover,
        StateVariant::Focus,
        StateVariant::Active,
        StateVariant::Disabled,
        StateVariant::Loading,
    ];

    println!("\n可用的状态变体:");
    for variant in &state_variants {
        println!("  - {:?}", variant);
    }

    // 变体值包装
    let variant_values = vec![
        VariantValue::Size(SizeVariant::LG),
        VariantValue::Color(ColorVariant::Primary),
        VariantValue::State(StateVariant::Hover),
        VariantValue::String("custom-variant".to_string()),
    ];

    println!("\n变体值示例:");
    for value in &variant_values {
        println!("  - {:?}", value);
    }

    // 测试变体兼容性
    let size_value = VariantValue::Size(SizeVariant::MD);
    let color_value = VariantValue::Color(ColorVariant::Success);

    println!("\n变体兼容性测试:");
    println!(
        "  尺寸变体与颜色变体兼容: {}",
        size_value.is_compatible(&color_value)
    );

    let state1 = VariantValue::State(StateVariant::Hover);
    let state2 = VariantValue::State(StateVariant::Focus);
    println!(
        "  悬停状态与焦点状态兼容: {}",
        state1.is_compatible(&state2)
    );
}

/// 演示断点系统
fn demo_breakpoint_system() {
    println!("\n--- 断点系统演示 ---");

    // 创建标准断点
    let breakpoints = vec![
        Breakpoint {
            name: "mobile".to_string(),
            min_width: Some(0),
            max_width: Some(767),
            media_query: "(max-width: 767px)".to_string(),
            priority: 1,
        },
        Breakpoint {
            name: "tablet".to_string(),
            min_width: Some(768),
            max_width: Some(1023),
            media_query: "(min-width: 768px) and (max-width: 1023px)".to_string(),
            priority: 2,
        },
        Breakpoint {
            name: "desktop".to_string(),
            min_width: Some(1024),
            max_width: Some(1439),
            media_query: "(min-width: 1024px) and (max-width: 1439px)".to_string(),
            priority: 3,
        },
        Breakpoint {
            name: "wide".to_string(),
            min_width: Some(1440),
            max_width: None,
            media_query: "(min-width: 1440px)".to_string(),
            priority: 4,
        },
    ];

    println!("定义的响应式断点:");
    for breakpoint in &breakpoints {
        println!(
            "  - {}: {} (优先级: {})",
            breakpoint.name, breakpoint.media_query, breakpoint.priority
        );
    }

    // 演示断点匹配
    println!("\n断点匹配示例:");
    let test_widths = vec![320, 768, 1024, 1920];

    for width in test_widths {
        println!("  屏幕宽度 {}px 匹配的断点:", width);
        for breakpoint in &breakpoints {
            let matches = match (breakpoint.min_width, breakpoint.max_width) {
                (Some(min), Some(max)) => width >= min && width <= max,
                (Some(min), None) => width >= min,
                (None, Some(max)) => width <= max,
                (None, None) => true,
            };

            if matches {
                println!("    ✓ {}", breakpoint.name);
            }
        }
    }
}

/// 演示状态类型
fn demo_state_types() {
    println!("\n--- 状态类型演示 ---");

    // 标准伪类状态
    let pseudo_states = vec![
        StateType::Hover,
        StateType::Focus,
        StateType::Active,
        StateType::Visited,
        StateType::Disabled,
        StateType::Checked,
    ];

    println!("标准伪类状态:");
    for state in &pseudo_states {
        println!("  - {:?}", state);
    }

    // 自定义状态
    let custom_states = vec![
        StateType::Custom("loading".to_string()),
        StateType::Custom("error".to_string()),
        StateType::Custom("success".to_string()),
        StateType::Custom("pending".to_string()),
    ];

    println!("\n自定义状态:");
    for state in &custom_states {
        println!("  - {:?}", state);
    }

    // 状态组合示例
    println!("\n状态组合示例:");
    let combinations = vec![
        vec![StateType::Hover, StateType::Focus],
        vec![StateType::Active, StateType::Disabled],
        vec![
            StateType::Custom("loading".to_string()),
            StateType::Disabled,
        ],
    ];

    for (i, combination) in combinations.iter().enumerate() {
        println!("  组合 {}: {:?}", i + 1, combination);
    }
}

/// 演示条件值
fn demo_condition_values() {
    println!("\n--- 条件值演示 ---");

    // 基础条件值类型
    let condition_values = vec![
        ConditionValue::String("primary".to_string()),
        ConditionValue::Number(42.0),
        ConditionValue::Boolean(true),
        ConditionValue::Array(vec![
            ConditionValue::String("red".to_string()),
            ConditionValue::String("blue".to_string()),
            ConditionValue::String("green".to_string()),
        ]),
    ];

    println!("基础条件值类型:");
    for (i, value) in condition_values.iter().enumerate() {
        println!("  {}: {:?}", i + 1, value);
    }

    // 对象类型条件值
    let mut object_value = HashMap::new();
    object_value.insert(
        "variant".to_string(),
        ConditionValue::String("primary".to_string()),
    );
    object_value.insert(
        "size".to_string(),
        ConditionValue::String("large".to_string()),
    );
    object_value.insert("disabled".to_string(), ConditionValue::Boolean(false));
    object_value.insert("width".to_string(), ConditionValue::Number(300.0));

    let complex_condition = ConditionValue::Object(object_value);
    println!("\n复杂对象条件值:");
    println!("  {:?}", complex_condition);

    // 嵌套数组示例
    let nested_array = ConditionValue::Array(vec![
        ConditionValue::Object({
            let mut obj = HashMap::new();
            obj.insert(
                "type".to_string(),
                ConditionValue::String("button".to_string()),
            );
            obj.insert(
                "variant".to_string(),
                ConditionValue::String("primary".to_string()),
            );
            obj
        }),
        ConditionValue::Object({
            let mut obj = HashMap::new();
            obj.insert(
                "type".to_string(),
                ConditionValue::String("input".to_string()),
            );
            obj.insert(
                "variant".to_string(),
                ConditionValue::String("outlined".to_string()),
            );
            obj
        }),
    ]);

    println!("\n嵌套数组条件值:");
    println!("  {:?}", nested_array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_types() {
        let size = VariantValue::Size(SizeVariant::Large);
        let color = VariantValue::Color(ColorVariant::Primary);

        assert!(size.is_compatible(&color));
    }

    #[test]
    fn test_breakpoint_creation() {
        let breakpoint = Breakpoint {
            name: "test".to_string(),
            min_width: Some(768),
            max_width: Some(1023),
            media_query: "(min-width: 768px)".to_string(),
            priority: 1,
        };

        assert_eq!(breakpoint.name, "test");
        assert_eq!(breakpoint.priority, 1);
    }

    #[test]
    fn test_state_types() {
        let hover = StateType::Hover;
        let custom = StateType::Custom("loading".to_string());

        assert_ne!(hover, custom);
    }

    #[test]
    fn test_condition_values() {
        let string_val = ConditionValue::String("test".to_string());
        let number_val = ConditionValue::Number(42.0);
        let bool_val = ConditionValue::Boolean(true);

        // 测试不同类型的条件值
        match string_val {
            ConditionValue::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected string value"),
        }

        match number_val {
            ConditionValue::Number(n) => assert_eq!(n, 42.0),
            _ => panic!("Expected number value"),
        }

        match bool_val {
            ConditionValue::Boolean(b) => assert!(b),
            _ => panic!("Expected boolean value"),
        }
    }
}
