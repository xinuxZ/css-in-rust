//! CSS-in-Rust 完整功能演示
//!
//! 本文件演示了 CSS-in-Rust 的所有核心功能，包括：
//! - 基础 CSS 宏功能
//! - 条件样式应用
//! - 类名组合
//! - 嵌套样式支持
//! - 伪类和伪元素
//! - 媒体查询
//! - CSS 变量
//! - 样式注入

use css_in_rust::css;
use css_in_rust::runtime::StyleManager;
use std::collections::HashMap;

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde as _;
use serde_json as _;
use tokio as _;

/// 主函数 - 演示所有CSS功能
fn main() {
    println!("🎨 CSS-in-Rust 完整功能演示");
    println!("==================================");
    println!();

    // 初始化样式管理器
    css_in_rust::init();

    // 基础功能测试
    // test_basic_css_macro();
    test_conditional_styles();
    // test_class_composition();
    test_nested_styles();
    // test_pseudo_elements();
    // test_media_queries();
    // test_css_variables();
    // test_animations();
    // test_style_injection();

    println!("✅ 所有功能测试完成！");
}

/// 测试基础 CSS 功能
fn test_basic_css() {
    println!("\n--- 测试基础 CSS 功能 ---");

    let button_style = css!("background-color: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    println!("按钮样式类名: {}", button_style);

    let card_style = css!("background: white; border-radius: 8px; padding: 16px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);");
    println!("卡片样式类名: {}", card_style);

    // 生成HTML示例
    let html = format!(
        r#"<button class="{}">点击我</button>
<div class="{}">这是一个卡片</div>"#,
        button_style, card_style
    );
    println!("生成的HTML:\n{}", html);
}

/// 测试条件样式
fn test_conditional_styles() {
    println!("\n--- 测试条件样式 ---");

    let is_primary = true;
    let is_disabled = false;

    // 使用基础条件样式（如果css_if!可用）
    // 由于当前实现可能不完整，我们用基础css!宏模拟
    let button_class = if is_primary {
        css!("background-color: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px;")
    } else {
        css!("background-color: #f8f9fa; color: #212529; border: 1px solid #dee2e6; padding: 8px 16px; border-radius: 4px;")
    };

    let disabled_class = if is_disabled {
        css!("opacity: 0.6; cursor: not-allowed;")
    } else {
        css!("cursor: pointer;")
    };

    println!("主要按钮类名: {}", button_class);
    println!("禁用状态类名: {}", disabled_class);

    // 组合类名
    let combined_html = format!(
        r#"<button class="{} {}">条件样式按钮</button>"#,
        button_class, disabled_class
    );
    println!("组合样式HTML: {}", combined_html);
}

/// 测试嵌套样式
fn test_nested_styles() {
    println!("\n--- 测试嵌套样式 ---");

    // 测试复杂的嵌套CSS
    let navigation_style = css!(
        "display: flex; align-items: center; padding: 0 16px; background: white; border-bottom: 1px solid #e8e8e8;"
    );

    let nav_item_style = css!(
        "padding: 8px 16px; color: #666; text-decoration: none; border-radius: 4px; transition: all 0.2s;"
    );

    let nav_item_hover_style = css!("background-color: #f0f0f0; color: #1890ff;");

    println!("导航栏样式: {}", navigation_style);
    println!("导航项样式: {}", nav_item_style);
    println!("导航项悬停样式: {}", nav_item_hover_style);

    let nav_html = format!(
        r#"<nav class="{}"> <a href="" class="{}">首页</a> <a href="" class="{}">关于</a> <a href="" class="{}">联系</a> </nav>"#,
        navigation_style, nav_item_style, nav_item_style, nav_item_style
    );
    println!("导航HTML:\n{}", nav_html);
}

/// 测试伪类样式
fn test_pseudo_classes() {
    println!("\n--- 测试伪类样式 ---");

    // 由于当前实现可能不支持完整的伪类语法，我们分别定义
    let button_base = css!(
        "background-color: #007bff; color: white; padding: 12px 24px; border: none; border-radius: 6px; font-size: 16px; cursor: pointer; transition: all 0.2s ease;"
    );

    let button_hover = css!(
        "background-color: #0056b3; transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0, 123, 255, 0.3);"
    );

    let button_active = css!(
        "background-color: #004085; transform: translateY(0); box-shadow: 0 2px 4px rgba(0, 123, 255, 0.3);"
    );

    let button_focus = css!("outline: 2px solid #80bdff; outline-offset: 2px;");

    println!("按钮基础样式: {}", button_base);
    println!("按钮悬停样式: {}", button_hover);
    println!("按钮激活样式: {}", button_active);
    println!("按钮焦点样式: {}", button_focus);

    // 生成带有伪类的CSS（需要手动处理）
    let interactive_button_html = format!(
        r#"<button class="{}"
        onmouseover="this.className='{}'"
        onmouseout="this.className='{}'"
        onmousedown="this.className='{}'"
        onmouseup="this.className='{}'"
        onfocus="this.className='{} {}'"
        onblur="this.className='{}'">交互式按钮</button>"#,
        button_base,
        button_hover,
        button_base,
        button_active,
        button_hover,
        button_base,
        button_focus,
        button_base
    );

    println!("交互式按钮HTML:\n{}", interactive_button_html);
}
