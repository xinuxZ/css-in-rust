//! 最简单的 CSS-in-Rust 示例
//!
//! 本示例展示最基础的功能，避免复杂的依赖问题。

use css_in_rust::variants::minimal::{SimpleVariantManager, SimpleVariantStyle};
use std::collections::HashMap;

fn main() {
    println!("=== CSS-in-Rust 最简示例 ===");

    // 创建简单变体管理器
    let mut manager = SimpleVariantManager::new();

    // 注册一些基础变体
    let size_small = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("width".to_string(), "100px".to_string());
            props.insert("height".to_string(), "50px".to_string());
            props
        },
        priority: 1,
    };

    let size_large = SimpleVariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("width".to_string(), "200px".to_string());
            props.insert("height".to_string(), "100px".to_string());
            props
        },
        priority: 1,
    };

    // 注册变体
    manager.register_variant("size-small".to_string(), size_small);
    manager.register_variant("size-large".to_string(), size_large);

    println!("已注册变体: size-small, size-large");

    // 激活变体
    manager.activate_variant("size-small".to_string());
    println!("激活变体: size-small");

    // 获取当前样式
    let styles = manager.get_current_styles();
    println!("当前样式: {:?}", styles);

    // 切换到大尺寸
    manager.deactivate_variant("size-small");
    manager.activate_variant("size-large".to_string());
    println!("切换到变体: size-large");

    let styles = manager.get_current_styles();
    println!("当前样式: {:?}", styles);

    println!("=== 示例完成 ===");
}
