//! 变体系统演示示例
//!
//! 展示 CSS-in-Rust 变体系统的完整功能，包括：
//! - 结构化变体系统
//! - 响应式断点系统
//! - 状态变体
//! - 条件样式
//! - 样式优先级管理

use css_in_rust::{
    theme::Theme,
    variants::{
        conditional_styles::{ConditionType, ConditionValue, ConditionalStyleManager},
        priority_manager::{PriorityManager, PriorityType, StyleSource},
        responsive::{responsive_variant, Breakpoint, ResponsiveManager},
        state_variants::{StateType, StateVariantManager},
        variant_types::{
            ColorVariant, SizeVariant, StateVariant, VariantCombination, VariantValue,
        },
        VariantResolutionContext, VariantResolutionOptions, VariantResolver,
    },
};
use std::collections::HashMap;

#[allow(unused_imports)]
use css_in_rust_macros as _;
#[allow(unused_imports)]
use lightningcss as _;
#[allow(unused_imports)]
use proc_macro2 as _;
#[allow(unused_imports)]
use quote as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
#[allow(unused_imports)]
use sha2 as _;
#[allow(unused_imports)]
use syn as _;

fn main() {
    println!("=== CSS-in-Rust 变体系统演示 ===");

    // 1. 基础变体系统演示
    demo_basic_variants();

    // 2. 响应式断点系统演示
    demo_responsive_system();

    // 3. 状态变体演示
    demo_state_variants();

    // 4. 条件样式演示
    demo_conditional_styles();

    // 5. 优先级管理演示
    demo_priority_management();

    // 6. 完整变体解析演示
    demo_complete_variant_resolution();
}

/// 演示基础变体系统
fn demo_basic_variants() {
    println!("\n--- 基础变体系统演示 ---");

    // 创建变体组合
    let mut variant_combination = VariantCombination {
        name: Some("button".to_string()),
        variants: HashMap::new(),
        priority: 10,
    };

    // 添加不同类型的变体
    variant_combination
        .variants
        .insert("size".to_string(), VariantValue::Size(SizeVariant::Large));
    variant_combination.variants.insert(
        "color".to_string(),
        VariantValue::Color(ColorVariant::Primary),
    );
    variant_combination.variants.insert(
        "state".to_string(),
        VariantValue::State(StateVariant::Hover),
    );

    println!("创建的变体组合: {:?}", variant_combination);

    // 检查变体兼容性
    let size_variant = VariantValue::Size(SizeVariant::Small);
    let color_variant = VariantValue::Color(ColorVariant::Secondary);

    println!(
        "尺寸变体与颜色变体兼容性: {}",
        size_variant.is_compatible(&color_variant)
    );

    // 合并变体
    if let Some(merged) = size_variant.merge(&color_variant) {
        println!("合并后的变体: {:?}", merged);
    }
}

/// 演示响应式断点系统
fn demo_responsive_system() {
    println!("\n--- 响应式断点系统演示 ---");

    let mut responsive_manager = ResponsiveManager::new();

    // 添加断点
    responsive_manager.add_breakpoint(Breakpoint {
        name: "mobile".to_string(),
        min_width: Some(0),
        max_width: Some(767),
        media_query: "(max-width: 767px)".to_string(),
        priority: 1,
    });

    responsive_manager.add_breakpoint(Breakpoint {
        name: "tablet".to_string(),
        min_width: Some(768),
        max_width: Some(1023),
        media_query: "(min-width: 768px) and (max-width: 1023px)".to_string(),
        priority: 2,
    });

    responsive_manager.add_breakpoint(Breakpoint {
        name: "desktop".to_string(),
        min_width: Some(1024),
        max_width: None,
        media_query: "(min-width: 1024px)".to_string(),
        priority: 3,
    });

    // 设置当前活动断点
    responsive_manager.set_active_breakpoints(vec!["tablet".to_string()]);

    // 创建响应式变体
    let responsive_styles = HashMap::from([
        (
            "mobile".to_string(),
            HashMap::from([
                ("font-size".to_string(), "14px".to_string()),
                ("padding".to_string(), "8px".to_string()),
            ]),
        ),
        (
            "tablet".to_string(),
            HashMap::from([
                ("font-size".to_string(), "16px".to_string()),
                ("padding".to_string(), "12px".to_string()),
            ]),
        ),
        (
            "desktop".to_string(),
            HashMap::from([
                ("font-size".to_string(), "18px".to_string()),
                ("padding".to_string(), "16px".to_string()),
            ]),
        ),
    ]);

    let responsive_variant = responsive_variant("button", responsive_styles);
    println!("创建的响应式变体: {:?}", responsive_variant);

    // 获取当前断点的样式
    if let Some(current_styles) =
        responsive_manager.get_current_styles(&responsive_variant.breakpoint_styles)
    {
        println!("当前断点 (tablet) 的样式: {:?}", current_styles);
    }
}

/// 演示状态变体
fn demo_state_variants() {
    println!("\n--- 状态变体演示 ---");

    let mut state_manager = StateVariantManager::new();

    // 注册状态变体
    let hover_styles = HashMap::from([
        ("background-color".to_string(), "#007bff".to_string()),
        ("transform".to_string(), "scale(1.05)".to_string()),
    ]);

    state_manager.register_state_variant("button".to_string(), StateType::Hover, hover_styles);

    let focus_styles = HashMap::from([
        ("outline".to_string(), "2px solid #007bff".to_string()),
        (
            "box-shadow".to_string(),
            "0 0 0 3px rgba(0, 123, 255, 0.25)".to_string(),
        ),
    ]);

    state_manager.register_state_variant("button".to_string(), StateType::Focus, focus_styles);

    // 设置活动状态
    state_manager.set_active_states(vec![StateType::Hover, StateType::Focus]);

    // 生成状态样式
    let state_result = state_manager.generate_state_styles("button");
    println!("生成的状态样式: {:?}", state_result);

    // 检查状态组合
    let combination =
        state_manager.create_state_combination(vec![StateType::Hover, StateType::Active]);
    println!("状态组合: {:?}", combination);
}

/// 演示条件样式
fn demo_conditional_styles() {
    println!("\n--- 条件样式演示 ---");

    let mut conditional_manager = ConditionalStyleManager::new();

    // 设置 props
    let props = HashMap::from([
        (
            "variant".to_string(),
            ConditionValue::String("primary".to_string()),
        ),
        (
            "size".to_string(),
            ConditionValue::String("large".to_string()),
        ),
        ("disabled".to_string(), ConditionValue::Boolean(false)),
        ("width".to_string(), ConditionValue::Number(200.0)),
    ]);

    conditional_manager.set_props(props);

    // 添加条件样式规则
    conditional_manager.add_conditional_style(
        "button-primary".to_string(),
        ConditionType::Equals,
        "variant".to_string(),
        ConditionValue::String("primary".to_string()),
        HashMap::from([
            ("background-color".to_string(), "#007bff".to_string()),
            ("color".to_string(), "white".to_string()),
        ]),
    );

    conditional_manager.add_conditional_style(
        "button-large".to_string(),
        ConditionType::Equals,
        "size".to_string(),
        ConditionValue::String("large".to_string()),
        HashMap::from([
            ("padding".to_string(), "12px 24px".to_string()),
            ("font-size".to_string(), "18px".to_string()),
        ]),
    );

    // 添加动态样式规则
    conditional_manager.add_dynamic_style_rule(
        "dynamic-width".to_string(),
        "width".to_string(),
        "width".to_string(),
        "px".to_string(),
    );

    // 评估条件样式
    let conditional_result = conditional_manager.evaluate_conditional_styles();
    println!("条件样式评估结果: {:?}", conditional_result);

    // 计算动态样式
    let dynamic_result = conditional_manager.compute_dynamic_styles();
    println!("动态样式计算结果: {:?}", dynamic_result);
}

/// 演示优先级管理
fn demo_priority_management() {
    println!("\n--- 优先级管理演示 ---");

    let mut priority_manager = PriorityManager::new();

    // 添加不同优先级的样式规则
    priority_manager.add_style_rule(
        "background-color".to_string(),
        "blue".to_string(),
        PriorityType::Base,
        StyleSource::Theme,
        None,
    );

    priority_manager.add_style_rule(
        "background-color".to_string(),
        "red".to_string(),
        PriorityType::Variant,
        StyleSource::Variant,
        None,
    );

    priority_manager.add_style_rule(
        "background-color".to_string(),
        "green".to_string(),
        PriorityType::State,
        StyleSource::State,
        None,
    );

    priority_manager.add_style_rule(
        "color".to_string(),
        "white".to_string(),
        PriorityType::Base,
        StyleSource::Theme,
        None,
    );

    // 解析样式优先级
    let resolution_result = priority_manager.resolve_styles();
    println!("优先级解析结果: {:?}", resolution_result);

    // 检查冲突
    let conflicts = priority_manager.detect_conflicts();
    if !conflicts.is_empty() {
        println!("检测到的样式冲突: {:?}", conflicts);
    }
}

/// 演示完整的变体解析流程
fn demo_complete_variant_resolution() {
    println!("\n--- 完整变体解析演示 ---");

    let mut resolver = VariantResolver::new();

    // 配置解析选项
    let options = VariantResolutionOptions {
        enable_responsive: true,
        enable_state_variants: true,
        enable_conditional_styles: true,
        enable_priority_management: true,
        enable_caching: true,
        generate_source_maps: true,
    };

    resolver.set_options(options);

    // 准备变体配置
    let variants = HashMap::from([
        ("size".to_string(), "large".to_string()),
        ("color".to_string(), "primary".to_string()),
        ("variant".to_string(), "solid".to_string()),
    ]);

    // 准备解析上下文
    let context = VariantResolutionContext {
        current_breakpoints: vec!["desktop".to_string()],
        current_states: vec![StateType::Hover],
        current_props: HashMap::from([
            ("disabled".to_string(), ConditionValue::Boolean(false)),
            ("loading".to_string(), ConditionValue::Boolean(false)),
        ]),
        theme_context: Some("light".to_string()),
        strict_mode: false,
    };

    // 执行变体解析
    match resolver.resolve_variants(&variants, &context) {
        Ok(result) => {
            println!("\n=== 变体解析成功 ===");
            println!("最终样式: {:?}", result.final_styles);
            println!("应用的变体: {:?}", result.applied_variants);
            println!("生成的 CSS:\n{}", result.generated_css);
            println!("解析统计: {:?}", result.resolution_stats);

            if let Some(source_maps) = result.source_maps {
                println!("源映射: {:?}", source_maps);
            }
        }
        Err(error) => {
            println!("变体解析失败: {}", error);
        }
    }

    // 演示缓存效果
    println!("\n--- 缓存效果演示 ---");
    let start_time = std::time::Instant::now();

    // 第二次解析（应该使用缓存）
    match resolver.resolve_variants(&variants, &context) {
        Ok(result) => {
            let duration = start_time.elapsed();
            println!("第二次解析耗时: {:?}", duration);
            println!("缓存命中: {}", result.resolution_stats.cache_hits > 0);
        }
        Err(error) => {
            println!("第二次解析失败: {}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_system_integration() {
        let mut resolver = VariantResolver::new();
        let variants = HashMap::from([("size".to_string(), "medium".to_string())]);
        let context = VariantResolutionContext::default();

        let result = resolver.resolve_variants(&variants, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_responsive_breakpoints() {
        let mut manager = ResponsiveManager::new();
        manager.add_breakpoint(Breakpoint {
            name: "test".to_string(),
            min_width: Some(768),
            max_width: Some(1023),
            media_query: "(min-width: 768px)".to_string(),
            priority: 1,
        });

        assert!(manager.get_breakpoint("test").is_some());
    }

    #[test]
    fn test_state_variants() {
        let mut manager = StateVariantManager::new();
        let styles = HashMap::from([("color".to_string(), "blue".to_string())]);

        manager.register_state_variant("test".to_string(), StateType::Hover, styles);

        let result = manager.generate_state_styles("test");
        assert!(!result.state_styles.is_empty());
    }

    #[test]
    fn test_conditional_styles() {
        let mut manager = ConditionalStyleManager::new();
        let props = HashMap::from([(
            "variant".to_string(),
            ConditionValue::String("primary".to_string()),
        )]);

        manager.set_props(props);
        manager.add_conditional_style(
            "test".to_string(),
            ConditionType::Equals,
            "variant".to_string(),
            ConditionValue::String("primary".to_string()),
            HashMap::from([("color".to_string(), "blue".to_string())]),
        );

        let result = manager.evaluate_conditional_styles();
        assert!(!result.applied_styles.is_empty());
    }

    #[test]
    fn test_priority_management() {
        let mut manager = PriorityManager::new();

        manager.add_style_rule(
            "color".to_string(),
            "red".to_string(),
            PriorityType::Base,
            StyleSource::Theme,
            None,
        );

        manager.add_style_rule(
            "color".to_string(),
            "blue".to_string(),
            PriorityType::Variant,
            StyleSource::Variant,
            None,
        );

        let result = manager.resolve_styles();
        assert_eq!(result.final_styles.get("color"), Some(&"blue".to_string()));
    }
}
