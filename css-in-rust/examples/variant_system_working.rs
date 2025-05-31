//! CSS-in-Rust 变体系统工作示例
//!
//! 本示例展示了一个完整的变体系统，包括:
//! - 基础变体管理
//! - 多种变体类型
//! - 变体组合
//! - 样式应用

use css_in_rust::variants::minimal::{SimpleVariantManager, SimpleVariantStyle};
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
use tempfile as _;

fn main() {
    println!("=== CSS-in-Rust 变体系统完整示例 ===");

    // 创建变体管理器
    let mut manager = SimpleVariantManager::new();

    // 演示1: 尺寸变体
    demo_size_variants(&mut manager);

    // 演示2: 颜色变体
    demo_color_variants(&mut manager);

    // 演示3: 状态变体
    demo_state_variants(&mut manager);

    // 演示4: 变体组合
    demo_variant_combinations(&mut manager);

    println!("=== 示例完成 ===");
}

/// 演示尺寸变体
fn demo_size_variants(manager: &mut SimpleVariantManager) {
    println!("\n--- 尺寸变体演示 ---");

    // 注册小尺寸变体
    let size_small = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("width".to_string(), "80px".to_string());
            props.insert("height".to_string(), "40px".to_string());
            props.insert("padding".to_string(), "8px".to_string());
            props.insert("font-size".to_string(), "12px".to_string());
            props
        },
        priority: 1,
    };

    // 注册中等尺寸变体
    let size_medium = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("width".to_string(), "120px".to_string());
            props.insert("height".to_string(), "60px".to_string());
            props.insert("padding".to_string(), "12px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props
        },
        priority: 1,
    };

    // 注册大尺寸变体
    let size_large = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("width".to_string(), "160px".to_string());
            props.insert("height".to_string(), "80px".to_string());
            props.insert("padding".to_string(), "16px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props
        },
        priority: 1,
    };

    manager.register_variant("size-small".to_string(), size_small);
    manager.register_variant("size-medium".to_string(), size_medium);
    manager.register_variant("size-large".to_string(), size_large);

    println!("已注册尺寸变体: small, medium, large");

    // 测试不同尺寸
    for size in ["size-small", "size-medium", "size-large"] {
        manager.activate_variant(size.to_string());
        let styles = manager.get_current_styles();
        println!("激活 {}: {:?}", size, styles);
        manager.deactivate_variant(size);
    }
}

/// 演示颜色变体
fn demo_color_variants(manager: &mut SimpleVariantManager) {
    println!("\n--- 颜色变体演示 ---");

    // 主要颜色
    let color_primary = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#007bff".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border-color".to_string(), "#0056b3".to_string());
            props
        },
        priority: 2,
    };

    // 次要颜色
    let color_secondary = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#6c757d".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border-color".to_string(), "#545b62".to_string());
            props
        },
        priority: 2,
    };

    // 成功颜色
    let color_success = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#28a745".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border-color".to_string(), "#1e7e34".to_string());
            props
        },
        priority: 2,
    };

    // 危险颜色
    let color_danger = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#dc3545".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border-color".to_string(), "#bd2130".to_string());
            props
        },
        priority: 2,
    };

    manager.register_variant("color-primary".to_string(), color_primary);
    manager.register_variant("color-secondary".to_string(), color_secondary);
    manager.register_variant("color-success".to_string(), color_success);
    manager.register_variant("color-danger".to_string(), color_danger);

    println!("已注册颜色变体: primary, secondary, success, danger");

    // 测试不同颜色
    for color in [
        "color-primary",
        "color-secondary",
        "color-success",
        "color-danger",
    ] {
        manager.activate_variant(color.to_string());
        let styles = manager.get_current_styles();
        println!("激活 {}: {:?}", color, styles);
        manager.deactivate_variant(color);
    }
}

/// 演示状态变体
fn demo_state_variants(manager: &mut SimpleVariantManager) {
    println!("\n--- 状态变体演示 ---");

    // 悬停状态
    let state_hover = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("transform".to_string(), "scale(1.05)".to_string());
            props.insert(
                "box-shadow".to_string(),
                "0 4px 8px rgba(0,0,0,0.2)".to_string(),
            );
            props.insert("transition".to_string(), "all 0.2s ease".to_string());
            props
        },
        priority: 3,
    };

    // 焦点状态
    let state_focus = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("outline".to_string(), "2px solid #007bff".to_string());
            props.insert("outline-offset".to_string(), "2px".to_string());
            props
        },
        priority: 3,
    };

    // 禁用状态
    let state_disabled = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("opacity".to_string(), "0.5".to_string());
            props.insert("cursor".to_string(), "not-allowed".to_string());
            props.insert("pointer-events".to_string(), "none".to_string());
            props
        },
        priority: 4,
    };

    manager.register_variant("state-hover".to_string(), state_hover);
    manager.register_variant("state-focus".to_string(), state_focus);
    manager.register_variant("state-disabled".to_string(), state_disabled);

    println!("已注册状态变体: hover, focus, disabled");

    // 测试不同状态
    for state in ["state-hover", "state-focus", "state-disabled"] {
        manager.activate_variant(state.to_string());
        let styles = manager.get_current_styles();
        println!("激活 {}: {:?}", state, styles);
        manager.deactivate_variant(state);
    }
}

/// 演示变体组合
fn demo_variant_combinations(manager: &mut SimpleVariantManager) {
    println!("\n--- 变体组合演示 ---");

    // 组合1: 大尺寸 + 主要颜色
    manager.activate_variant("size-large".to_string());
    manager.activate_variant("color-primary".to_string());
    let styles1 = manager.get_current_styles();
    println!("组合1 (大尺寸 + 主要颜色): {:?}", styles1);

    // 组合2: 中等尺寸 + 成功颜色 + 悬停状态
    manager.deactivate_variant("size-large");
    manager.deactivate_variant("color-primary");
    manager.activate_variant("size-medium".to_string());
    manager.activate_variant("color-success".to_string());
    manager.activate_variant("state-hover".to_string());
    let styles2 = manager.get_current_styles();
    println!("组合2 (中等尺寸 + 成功颜色 + 悬停): {:?}", styles2);

    // 组合3: 小尺寸 + 危险颜色 + 禁用状态
    manager.deactivate_variant("size-medium");
    manager.deactivate_variant("color-success");
    manager.deactivate_variant("state-hover");
    manager.activate_variant("size-small".to_string());
    manager.activate_variant("color-danger".to_string());
    manager.activate_variant("state-disabled".to_string());
    let styles3 = manager.get_current_styles();
    println!("组合3 (小尺寸 + 危险颜色 + 禁用): {:?}", styles3);

    // 清理
    manager.deactivate_variant("size-small");
    manager.deactivate_variant("color-danger");
    manager.deactivate_variant("state-disabled");

    println!("\n变体组合演示完成，展示了不同优先级的样式如何合并");
}
