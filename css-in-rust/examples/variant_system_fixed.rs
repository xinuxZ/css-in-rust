//! 修复后的变体系统演示
//!
//! 这个示例展示了简化后的变体系统如何工作，
//! 包括变体配置、注册和应用功能。

use css_in_rust::variants::{VariantConfig, VariantManager, VariantStyle};

// Suppress unused dependency warnings
#[allow(unused_extern_crates)]
extern crate serde;
#[allow(unused_extern_crates)]
extern crate serde_json;
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
    println!("=== CSS in Rust 变体系统演示 ===");

    // 创建变体管理器
    let mut manager = VariantManager::new();
    println!("✅ 变体管理器创建成功");

    // 创建按钮变体配置
    let mut button_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 配置尺寸变体
    let small_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "8px 16px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    let large_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "16px 32px".to_string());
            props.insert("font-size".to_string(), "18px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    button_config.size.insert("small".to_string(), small_style);
    button_config.size.insert("large".to_string(), large_style);

    // 配置颜色变体
    let primary_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#1890ff".to_string());
            props.insert("color".to_string(), "white".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    let secondary_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#f5f5f5".to_string());
            props.insert("color".to_string(), "#333".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    button_config
        .color
        .insert("primary".to_string(), primary_style);
    button_config
        .color
        .insert("secondary".to_string(), secondary_style);

    // 设置默认变体
    button_config
        .defaults
        .insert("size".to_string(), "small".to_string());
    button_config
        .defaults
        .insert("color".to_string(), "primary".to_string());

    // 注册按钮配置
    manager.register_variant_config("button", button_config);
    println!("✅ 按钮变体配置注册成功");

    // 应用变体
    let mut variants = HashMap::new();
    variants.insert("size".to_string(), "large".to_string());
    variants.insert("color".to_string(), "secondary".to_string());

    let props = HashMap::new();

    match manager.apply_variants("button", &variants, &props) {
        Ok(result) => {
            println!("✅ 变体应用成功");
            println!("应用的变体: {:?}", result.applied_variants);
            println!("优先级分数: {}", result.priority_score);
            println!("CSS 类名: {}", result.class_name);
            println!("CSS 规则:");
            println!("  {}", result.css_rules);
        }
        Err(e) => {
            println!("❌ 变体应用失败: {}", e);
        }
    }

    println!("\n=== 演示完成 ===");
}
