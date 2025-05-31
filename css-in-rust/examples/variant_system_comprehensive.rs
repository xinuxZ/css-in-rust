//! CSS-in-Rust 变体系统综合示例
//!
//! 本示例展示了如何使用 CSS-in-Rust 的完整变体系统，包括：
//! - 基础变体管理
//! - 响应式变体
//! - 状态变体
//! - 条件样式
//! - 优先级管理
//! - 变体解析器

use css_in_rust::variants::{
    Breakpoint,
    ColorVariant,
    ConditionRule,
    ConditionType,
    ConditionValue,
    ConditionalStyle,
    // 条件样式
    ConditionalStyleManager,
    DynamicStyleRule,
    // 优先级管理
    PriorityManager,
    PriorityType,
    // 响应式
    ResponsiveManager,
    ResponsiveStyleResult,
    ShapeVariant,
    SimpleVariant,
    SimpleVariantManager,
    SimpleVariantStyle,
    // 变体类型
    SizeVariant,
    Specificity,
    StateCombination,
    StateType,
    // 状态变体
    StateVariant,
    StateVariantManager,
    StateVariantResult,
    StyleCalculator,
    StyleRule,
    StyleSource,
    // 基础变体
    VariantConfig,
    VariantManager,
    VariantResolutionContext,
    VariantResolutionOptions,
    // 变体解析器
    VariantResolver,
    VariantStyle,
};
use std::collections::HashMap;

use base64 as _;
use chrono as _;
#[allow(unused_imports)]
use css_in_rust_macros as _;
use lazy_static as _;
#[allow(unused_imports)]
use lightningcss as _;
use num_cpus as _;
#[allow(unused_imports)]
use proc_macro2 as _;
#[allow(unused_imports)]
use quote as _;
use regex as _;
#[allow(unused_imports)]
use serde as _;
#[allow(unused_imports)]
use serde_json as _;
use sha1 as _;
#[allow(unused_imports)]
use sha2 as _;
#[allow(unused_imports)]
use syn as _;
use tempfile as _;

fn main() {
    println!("=== CSS-in-Rust 变体系统综合示例 ===");

    // 1. 基础变体管理示例
    demo_basic_variants();

    // 2. 响应式变体示例
    demo_responsive_variants();

    // 3. 状态变体示例
    demo_state_variants();

    // 4. 条件样式示例
    demo_conditional_styles();

    // 5. 优先级管理示例
    demo_priority_management();

    // 6. 变体解析器综合示例
    demo_variant_resolver();

    println!("\n=== 示例完成 ===");
}

/// 基础变体管理示例
fn demo_basic_variants() {
    println!("\n--- 基础变体管理示例 ---");

    // 创建变体管理器
    let mut variant_manager = VariantManager::new();

    // 创建按钮组件的变体配置
    let mut button_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 定义尺寸变体
    let small_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "4px 8px".to_string());
            props.insert("font-size".to_string(), "12px".to_string());
            props.insert("height".to_string(), "24px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 10,
    };

    let medium_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "8px 16px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props.insert("height".to_string(), "32px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 10,
    };

    let large_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "12px 24px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props.insert("height".to_string(), "40px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 10,
    };

    // 定义颜色变体
    let primary_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#007bff".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border".to_string(), "1px solid #007bff".to_string());
            props
        },
        pseudo_classes: {
            let mut pseudo = HashMap::new();
            let mut hover_props = HashMap::new();
            hover_props.insert("background-color".to_string(), "#0056b3".to_string());
            pseudo.insert("hover".to_string(), hover_props);
            pseudo
        },
        priority: 20,
    };

    let secondary_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#6c757d".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border".to_string(), "1px solid #6c757d".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 20,
    };

    // 添加变体到配置
    button_config.size.insert("sm".to_string(), small_style);
    button_config.size.insert("md".to_string(), medium_style);
    button_config.size.insert("lg".to_string(), large_style);
    button_config
        .color
        .insert("primary".to_string(), primary_style);
    button_config
        .color
        .insert("secondary".to_string(), secondary_style);

    // 设置默认值
    button_config
        .defaults
        .insert("size".to_string(), "md".to_string());
    button_config
        .defaults
        .insert("color".to_string(), "primary".to_string());

    // 注册配置
    variant_manager.register_variant_config("button", button_config);

    // 应用变体
    let mut variants = HashMap::new();
    variants.insert("size".to_string(), "lg".to_string());
    variants.insert("color".to_string(), "secondary".to_string());

    match variant_manager.apply_variants("button", &variants) {
        Ok(result) => {
            println!("生成的类名: {}", result.class_name);
            println!("CSS 规则: {}", result.css_rules);
            println!("应用的变体: {:?}", result.applied_variants);
        }
        Err(e) => println!("应用变体失败: {}", e),
    }
}

/// 响应式变体示例
fn demo_responsive_variants() {
    println!("\n--- 响应式变体示例 ---");

    let mut responsive_manager = ResponsiveManager::new();

    // 定义断点
    let breakpoints = vec![
        Breakpoint {
            name: "mobile".to_string(),
            min_width: Some(0),
            max_width: Some(767),
            media_query: "(max-width: 767px)".to_string(),
            priority: 10,
        },
        Breakpoint {
            name: "tablet".to_string(),
            min_width: Some(768),
            max_width: Some(1023),
            media_query: "(min-width: 768px) and (max-width: 1023px)".to_string(),
            priority: 10,
        },
        Breakpoint {
            name: "desktop".to_string(),
            min_width: Some(1024),
            max_width: None,
            media_query: "(min-width: 1024px)".to_string(),
            priority: 10,
        },
    ];

    // 注册断点
    for breakpoint in breakpoints {
        responsive_manager.register_breakpoint(breakpoint);
    }

    // 创建响应式变体配置
    let mut responsive_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 定义响应式样式
    let mobile_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("font-size".to_string(), "14px".to_string());
            props.insert("padding".to_string(), "8px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 10,
    };

    let tablet_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("font-size".to_string(), "16px".to_string());
            props.insert("padding".to_string(), "12px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 20,
    };

    let desktop_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("font-size".to_string(), "18px".to_string());
            props.insert("padding".to_string(), "16px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 30,
    };

    // 添加响应式变体
    responsive_config
        .responsive
        .insert("mobile".to_string(), mobile_style);
    responsive_config
        .responsive
        .insert("tablet".to_string(), tablet_style);
    responsive_config
        .responsive
        .insert("desktop".to_string(), desktop_style);

    // 设置当前断点
    responsive_manager.set_active_breakpoints(vec!["tablet".to_string()]);

    // 应用响应式样式
    match responsive_manager.apply_responsive_variants(&responsive_config) {
        Ok(result) => {
            println!("基础样式: {:?}", result.base_styles);
            println!("媒体查询样式: {:?}", result.media_queries);
            println!("生成的 CSS: {}", result.css);
        }
        Err(e) => println!("应用响应式样式失败: {}", e),
    }
}

/// 状态变体示例
fn demo_state_variants() {
    println!("\n--- 状态变体示例 ---");

    let mut state_manager = StateVariantManager::new();

    // 创建状态变体
    let hover_variant = StateVariant {
        state_type: StateType::Hover,
        style: VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("background-color".to_string(), "#f0f0f0".to_string());
                props.insert("transform".to_string(), "scale(1.05)".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 100,
        },
        combinable: true,
        priority: 100,
    };

    let active_variant = StateVariant {
        state_type: StateType::Active,
        style: VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("background-color".to_string(), "#d0d0d0".to_string());
                props.insert("transform".to_string(), "scale(0.95)".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 110,
        },
        combinable: true,
        priority: 110,
    };

    let disabled_variant = StateVariant {
        state_type: StateType::Disabled,
        style: VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("opacity".to_string(), "0.5".to_string());
                props.insert("cursor".to_string(), "not-allowed".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 120,
        },
        combinable: false,
        priority: 120,
    };

    // 注册状态变体
    state_manager.register_variant(hover_variant);
    state_manager.register_variant(active_variant);
    state_manager.register_variant(disabled_variant);

    // 创建状态组合
    let hover_active_combination = StateCombination {
        states: vec![StateType::Hover, StateType::Active],
        style: VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("background-color".to_string(), "#e0e0e0".to_string());
                props.insert("transform".to_string(), "scale(1.02)".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 150,
        },
        priority: 150,
    };

    state_manager.register_combination(hover_active_combination);

    // 设置活动状态
    state_manager.set_active_states(vec![StateType::Hover]);

    // 应用状态变体
    match state_manager.apply_state_variants() {
        Ok(result) => {
            println!("基础样式: {:?}", result.base_styles);
            println!("状态样式: {:?}", result.state_styles);
            println!("生成的 CSS: {}", result.css);
        }
        Err(e) => println!("应用状态变体失败: {}", e),
    }
}

/// 条件样式示例
fn demo_conditional_styles() {
    println!("\n--- 条件样式示例 ---");

    let mut conditional_manager = ConditionalStyleManager::new();

    // 设置 props
    let mut props = HashMap::new();
    props.insert(
        "theme".to_string(),
        ConditionValue::String("dark".to_string()),
    );
    props.insert(
        "size".to_string(),
        ConditionValue::String("large".to_string()),
    );
    props.insert("count".to_string(), ConditionValue::Number(42.0));
    props.insert("isVisible".to_string(), ConditionValue::Boolean(true));

    conditional_manager.set_props(props);

    // 创建条件规则
    let theme_condition = ConditionRule {
        condition_type: ConditionType::Equals,
        prop_name: "theme".to_string(),
        expected_value: ConditionValue::String("dark".to_string()),
    };

    let size_condition = ConditionRule {
        condition_type: ConditionType::Equals,
        prop_name: "size".to_string(),
        expected_value: ConditionValue::String("large".to_string()),
    };

    // 创建条件样式
    let dark_theme_style = ConditionalStyle {
        condition: theme_condition,
        styles: {
            let mut styles = HashMap::new();
            styles.insert("background-color".to_string(), "#333".to_string());
            styles.insert("color".to_string(), "#fff".to_string());
            styles
        },
        priority: 50,
    };

    let large_size_style = ConditionalStyle {
        condition: size_condition,
        styles: {
            let mut styles = HashMap::new();
            styles.insert("font-size".to_string(), "18px".to_string());
            styles.insert("padding".to_string(), "16px".to_string());
            styles
        },
        priority: 60,
    };

    // 注册条件样式
    conditional_manager.register_conditional_style(dark_theme_style);
    conditional_manager.register_conditional_style(large_size_style);

    // 创建动态样式规则
    let dynamic_rule = DynamicStyleRule {
        prop_name: "count".to_string(),
        calculator: StyleCalculator::Linear {
            base_value: 10.0,
            multiplier: 0.5,
            unit: "px".to_string(),
        },
        target_property: "margin-top".to_string(),
        priority: 70,
    };

    conditional_manager.register_dynamic_rule(dynamic_rule);

    // 评估条件样式
    match conditional_manager.evaluate_conditional_styles() {
        Ok(result) => {
            println!("匹配的条件: {:?}", result.matched_conditions);
            println!("应用的样式: {:?}", result.applied_styles);
        }
        Err(e) => println!("评估条件样式失败: {}", e),
    }

    // 计算动态样式
    match conditional_manager.compute_dynamic_styles() {
        Ok(result) => {
            println!("动态样式: {:?}", result.computed_styles);
            println!("计算详情: {:?}", result.calculation_details);
        }
        Err(e) => println!("计算动态样式失败: {}", e),
    }
}

/// 优先级管理示例
fn demo_priority_management() {
    println!("\n--- 优先级管理示例 ---");

    let mut priority_manager = PriorityManager::new();

    // 创建样式规则
    let base_rule = StyleRule {
        property: "background-color".to_string(),
        value: "blue".to_string(),
        priority_type: PriorityType::Base,
        priority_value: 10,
        source: StyleSource::Base,
        important: false,
        specificity: Specificity {
            inline: 0,
            ids: 0,
            classes: 1,
            elements: 1,
        },
        timestamp: 1,
    };

    let hover_rule = StyleRule {
        property: "background-color".to_string(),
        value: "darkblue".to_string(),
        priority_type: PriorityType::State,
        priority_value: 20,
        source: StyleSource::State("hover".to_string()),
        important: false,
        specificity: Specificity {
            inline: 0,
            ids: 0,
            classes: 1,
            elements: 1,
        },
        timestamp: 2,
    };

    let important_rule = StyleRule {
        property: "background-color".to_string(),
        value: "red".to_string(),
        priority_type: PriorityType::Important,
        priority_value: 30,
        source: StyleSource::Custom("important".to_string()),
        important: true,
        specificity: Specificity {
            inline: 0,
            ids: 0,
            classes: 2,
            elements: 1,
        },
        timestamp: 3,
    };

    // 添加规则
    priority_manager.add_rule(base_rule);
    priority_manager.add_rule(hover_rule);
    priority_manager.add_rule(important_rule);

    // 解析样式
    match priority_manager.resolve_property_conflicts("background-color") {
        Ok(result) => {
            println!("最终样式值: {:?}", result.final_value);
            println!("获胜规则: {:?}", result.winning_rule);
            println!("冲突规则: {:?}", result.conflicting_rules);
        }
        Err(e) => println!("解析样式失败: {}", e),
    }
}

/// 变体解析器综合示例
fn demo_variant_resolver() {
    println!("\n--- 变体解析器综合示例 ---");

    // 创建各个管理器的实例
    let mut variant_manager = VariantManager::new();
    let mut responsive_manager = ResponsiveManager::new();
    let mut state_manager = StateVariantManager::new();
    let mut conditional_manager = ConditionalStyleManager::new();
    let mut priority_manager = PriorityManager::new();

    // 创建综合的按钮配置
    let mut button_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 添加基础变体
    let large_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "12px 24px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 10,
    };

    let primary_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#007bff".to_string());
            props.insert("color".to_string(), "white".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 20,
    };

    button_config.size.insert("large".to_string(), large_style);
    button_config
        .color
        .insert("primary".to_string(), primary_style);

    variant_manager.register_variant_config("button", button_config);

    // 应用基础变体
    let mut variants = HashMap::new();
    variants.insert("size".to_string(), "large".to_string());
    variants.insert("color".to_string(), "primary".to_string());

    match variant_manager.apply_variants("button", &variants) {
        Ok(result) => {
            println!("基础变体结果:");
            println!("  类名: {}", result.class_name);
            println!("  CSS: {}", result.css_rules);
            println!("  应用的变体: {:?}", result.applied_variants);
        }
        Err(e) => println!("应用基础变体失败: {}", e),
    }

    println!("\n变体解析器演示完成 - 各个管理器可以独立使用或组合使用");
}
