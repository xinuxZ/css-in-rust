//! CSS-in-Rust 变体系统简单示例
//!
//! 本示例展示如何使用 CSS-in-Rust 的变体系统来创建可复用的样式组件。
//! 包括基础变体管理、简单变体应用等功能。

use css_in_rust::variants::{
    SimpleVariantManager, SimpleVariantStyle, VariantConfig, VariantStyle,
};
use std::collections::HashMap;

// 只保留实际需要的导入
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
    println!("=== CSS-in-Rust 变体系统简单示例 ===");

    // 1. 基础变体管理示例
    demo_simple_variant_manager();

    // 2. 变体配置示例
    demo_variant_config();

    // 3. 综合应用示例
    demo_comprehensive_usage();
}

/// 演示简单变体管理器的使用
fn demo_simple_variant_manager() {
    println!("\n--- 简单变体管理器示例 ---");

    let mut manager = SimpleVariantManager::new();

    // 注册尺寸变体
    let small_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "8px 12px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props.insert("border-radius".to_string(), "4px".to_string());
            props
        },
        priority: 1,
    };

    let medium_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "12px 16px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props.insert("border-radius".to_string(), "6px".to_string());
            props
        },
        priority: 1,
    };

    let large_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "16px 24px".to_string());
            props.insert("font-size".to_string(), "18px".to_string());
            props.insert("border-radius".to_string(), "8px".to_string());
            props
        },
        priority: 1,
    };

    // 注册变体
    manager.register_variant("size-small".to_string(), small_style);
    manager.register_variant("size-medium".to_string(), medium_style);
    manager.register_variant("size-large".to_string(), large_style);

    // 注册颜色变体
    let primary_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#3b82f6".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border".to_string(), "1px solid #3b82f6".to_string());
            props
        },
        priority: 2,
    };

    let secondary_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#6b7280".to_string());
            props.insert("color".to_string(), "white".to_string());
            props.insert("border".to_string(), "1px solid #6b7280".to_string());
            props
        },
        priority: 2,
    };

    manager.register_variant("color-primary".to_string(), primary_style);
    manager.register_variant("color-secondary".to_string(), secondary_style);

    // 激活变体
    manager.activate_variant("size-medium".to_string());
    manager.activate_variant("color-primary".to_string());

    // 获取当前样式
    let current_styles = manager.get_current_styles();
    println!("当前激活的样式:");
    for (property, value) in &current_styles {
        println!("  {}: {}", property, value);
    }

    // 切换到不同的变体组合
    manager.deactivate_variant("size-medium");
    manager.deactivate_variant("color-primary");
    manager.activate_variant("size-large".to_string());
    manager.activate_variant("color-secondary".to_string());

    let updated_styles = manager.get_current_styles();
    println!("\n更新后的样式:");
    for (property, value) in &updated_styles {
        println!("  {}: {}", property, value);
    }
}

/// 演示变体配置的使用
fn demo_variant_config() {
    println!("\n--- 变体配置示例 ---");

    // 创建变体配置
    let mut config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 配置尺寸变体
    config.size.insert(
        "xs".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("width".to_string(), "20px".to_string());
                props.insert("height".to_string(), "20px".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 1,
        },
    );

    config.size.insert(
        "sm".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("width".to_string(), "24px".to_string());
                props.insert("height".to_string(), "24px".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 1,
        },
    );

    config.size.insert(
        "md".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("width".to_string(), "32px".to_string());
                props.insert("height".to_string(), "32px".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 1,
        },
    );

    // 配置颜色变体
    config.color.insert(
        "red".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("color".to_string(), "#ef4444".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 2,
        },
    );

    config.color.insert(
        "green".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("color".to_string(), "#10b981".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 2,
        },
    );

    config.color.insert(
        "blue".to_string(),
        VariantStyle {
            properties: {
                let mut props = HashMap::new();
                props.insert("color".to_string(), "#3b82f6".to_string());
                props
            },
            pseudo_classes: HashMap::new(),
            priority: 2,
        },
    );

    // 使用简单变体管理器应用配置
    let manager = SimpleVariantManager::new();

    let mut test_variants = HashMap::new();
    test_variants.insert("size".to_string(), "md".to_string());
    test_variants.insert("color".to_string(), "blue".to_string());

    match manager.apply_simple_variants(&config, &test_variants) {
        Ok(styles) => {
            println!("应用变体配置后的样式:");
            for (property, value) in &styles {
                println!("  {}: {}", property, value);
            }
        }
        Err(e) => {
            println!("应用变体配置时出错: {}", e);
        }
    }
}

/// 演示综合使用场景
fn demo_comprehensive_usage() {
    println!("\n--- 综合应用示例 ---");

    // 创建按钮组件的变体系统
    let mut button_manager = SimpleVariantManager::new();

    // 定义按钮的基础样式变体
    let base_button_style = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("display".to_string(), "inline-flex".to_string());
            props.insert("align-items".to_string(), "center".to_string());
            props.insert("justify-content".to_string(), "center".to_string());
            props.insert("border".to_string(), "none".to_string());
            props.insert("cursor".to_string(), "pointer".to_string());
            props.insert("transition".to_string(), "all 0.2s ease-in-out".to_string());
            props
        },
        priority: 0,
    };

    // 尺寸变体
    let button_sm = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "6px 12px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props.insert("border-radius".to_string(), "4px".to_string());
            props
        },
        priority: 1,
    };

    let button_md = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "8px 16px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props.insert("border-radius".to_string(), "6px".to_string());
            props
        },
        priority: 1,
    };

    let button_lg = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "12px 24px".to_string());
            props.insert("font-size".to_string(), "18px".to_string());
            props.insert("border-radius".to_string(), "8px".to_string());
            props
        },
        priority: 1,
    };

    // 颜色变体
    let button_primary = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#3b82f6".to_string());
            props.insert("color".to_string(), "white".to_string());
            props
        },
        priority: 2,
    };

    let button_secondary = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#f3f4f6".to_string());
            props.insert("color".to_string(), "#374151".to_string());
            props
        },
        priority: 2,
    };

    let button_danger = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("background-color".to_string(), "#ef4444".to_string());
            props.insert("color".to_string(), "white".to_string());
            props
        },
        priority: 2,
    };

    // 注册所有变体
    button_manager.register_variant("base".to_string(), base_button_style);
    button_manager.register_variant("size-sm".to_string(), button_sm);
    button_manager.register_variant("size-md".to_string(), button_md);
    button_manager.register_variant("size-lg".to_string(), button_lg);
    button_manager.register_variant("color-primary".to_string(), button_primary);
    button_manager.register_variant("color-secondary".to_string(), button_secondary);
    button_manager.register_variant("color-danger".to_string(), button_danger);

    // 演示不同的按钮组合
    let button_combinations = vec![
        ("小型主要按钮", vec!["base", "size-sm", "color-primary"]),
        ("中型次要按钮", vec!["base", "size-md", "color-secondary"]),
        ("大型危险按钮", vec!["base", "size-lg", "color-danger"]),
    ];

    for (name, variants) in button_combinations {
        println!("\n{}:", name);

        // 清除之前的激活状态
        let mut fresh_manager = SimpleVariantManager::new();
        fresh_manager.register_variant(
            "base".to_string(),
            SimpleVariantStyle {
                properties: {
                    let mut props = HashMap::new();
                    props.insert("display".to_string(), "inline-flex".to_string());
                    props.insert("align-items".to_string(), "center".to_string());
                    props.insert("justify-content".to_string(), "center".to_string());
                    props.insert("border".to_string(), "none".to_string());
                    props.insert("cursor".to_string(), "pointer".to_string());
                    props.insert("transition".to_string(), "all 0.2s ease-in-out".to_string());
                    props
                },
                priority: 0,
            },
        );

        // 重新注册需要的变体
        if variants.contains(&"size-sm") {
            fresh_manager.register_variant(
                "size-sm".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("padding".to_string(), "6px 12px".to_string());
                        props.insert("font-size".to_string(), "14px".to_string());
                        props.insert("border-radius".to_string(), "4px".to_string());
                        props
                    },
                    priority: 1,
                },
            );
        }

        if variants.contains(&"size-md") {
            fresh_manager.register_variant(
                "size-md".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("padding".to_string(), "8px 16px".to_string());
                        props.insert("font-size".to_string(), "16px".to_string());
                        props.insert("border-radius".to_string(), "6px".to_string());
                        props
                    },
                    priority: 1,
                },
            );
        }

        if variants.contains(&"size-lg") {
            fresh_manager.register_variant(
                "size-lg".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("padding".to_string(), "12px 24px".to_string());
                        props.insert("font-size".to_string(), "18px".to_string());
                        props.insert("border-radius".to_string(), "8px".to_string());
                        props
                    },
                    priority: 1,
                },
            );
        }

        if variants.contains(&"color-primary") {
            fresh_manager.register_variant(
                "color-primary".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("background-color".to_string(), "#3b82f6".to_string());
                        props.insert("color".to_string(), "white".to_string());
                        props
                    },
                    priority: 2,
                },
            );
        }

        if variants.contains(&"color-secondary") {
            fresh_manager.register_variant(
                "color-secondary".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("background-color".to_string(), "#f3f4f6".to_string());
                        props.insert("color".to_string(), "#374151".to_string());
                        props
                    },
                    priority: 2,
                },
            );
        }

        if variants.contains(&"color-danger") {
            fresh_manager.register_variant(
                "color-danger".to_string(),
                SimpleVariantStyle {
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("background-color".to_string(), "#ef4444".to_string());
                        props.insert("color".to_string(), "white".to_string());
                        props
                    },
                    priority: 2,
                },
            );
        }

        // 激活变体
        for variant in variants {
            fresh_manager.activate_variant(variant.to_string());
        }

        // 获取最终样式
        let final_styles = fresh_manager.get_current_styles();
        for (property, value) in &final_styles {
            println!("  {}: {}", property, value);
        }
    }

    println!("\n=== 示例完成 ===");
}
