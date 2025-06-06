//! 变体系统演示
//!
//! 本模块演示 CSS-in-Rust 的变体系统功能，包括：
//! - 组件变体定义
//! - 尺寸变体（size variants）
//! - 颜色变体（color variants）
//! - 状态变体（state variants）
//! - 复合变体（compound variants）
//! - 响应式变体

use css_in_rust::{
    css, ConditionalStyleManager, PriorityManager, ResponsiveManager, StateVariantManager,
    VariantConfig, VariantResolver, VariantStyle,
};
use std::collections::HashMap;

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde as _;
use serde_json as _;
use tokio as _;

/// 变体演示主函数
pub fn run_variant_demo() {
    println!("🎭 变体系统演示");
    println!("================");
    println!();

    // 演示基础变体
    test_basic_variants();

    // 演示尺寸变体
    test_size_variants();

    // 演示颜色变体
    test_color_variants();

    // 演示状态变体
    test_state_variants();

    // 演示响应式变体
    test_responsive_variants();

    // 演示变体组合
    test_variant_combinations();

    // 演示条件样式
    test_conditional_styles();

    // 演示优先级管理
    test_priority_management();

    println!("✅ 变体系统演示完成！");
    println!();
}

fn main() {
    run_variant_demo();
}

/// 测试基础变体配置
fn test_basic_variants() {
    println!("\n--- 测试基础变体配置 ---");

    // 创建按钮变体配置
    let mut button_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    // 添加尺寸变体
    let small_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "4px 8px".to_string());
            props.insert("font-size".to_string(), "12px".to_string());
            props.insert("border-radius".to_string(), "3px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    let medium_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "8px 16px".to_string());
            props.insert("font-size".to_string(), "14px".to_string());
            props.insert("border-radius".to_string(), "4px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    let large_style = VariantStyle {
        properties: {
            let mut props = HashMap::new();
            props.insert("padding".to_string(), "12px 24px".to_string());
            props.insert("font-size".to_string(), "16px".to_string());
            props.insert("border-radius".to_string(), "6px".to_string());
            props
        },
        pseudo_classes: HashMap::new(),
        priority: 1,
    };

    button_config.size.insert("small".to_string(), small_style);
    button_config
        .size
        .insert("medium".to_string(), medium_style);
    button_config.size.insert("large".to_string(), large_style);

    // 设置默认值
    button_config
        .defaults
        .insert("size".to_string(), "medium".to_string());

    println!("创建按钮变体配置成功");
    println!("尺寸变体数量: {}", button_config.size.len());
    println!("默认尺寸: {:?}", button_config.defaults.get("size"));

    // 生成特定变体的CSS
    if let Some(large_variant) = button_config.size.get("large") {
        println!("\n大尺寸按钮样式:");
        for (prop, value) in &large_variant.properties {
            println!("  {}: {}", prop, value);
        }
    }
}

/// 测试尺寸变体
fn test_size_variants() {
    println!("\n--- 测试尺寸变体 ---");

    // 使用CSS宏创建不同尺寸的样式
    let small_button = css!("padding: 4px 8px; font-size: 12px; border-radius: 3px; background: #007bff; color: white; border: none; cursor: pointer;");
    let medium_button = css!("padding: 8px 16px; font-size: 14px; border-radius: 4px; background: #007bff; color: white; border: none; cursor: pointer;");
    let large_button = css!("padding: 12px 24px; font-size: 16px; border-radius: 6px; background: #007bff; color: white; border: none; cursor: pointer;");

    println!("小尺寸按钮类名: {}", small_button);
    println!("中尺寸按钮类名: {}", medium_button);
    println!("大尺寸按钮类名: {}", large_button);

    // 生成HTML示例
    let size_demo_html = format!(
        r#"<div class="button-size-demo">
  <button class="{}">小按钮</button>
  <button class="{}">中按钮</button>
  <button class="{}">大按钮</button>
</div>"#,
        small_button, medium_button, large_button
    );

    println!("\n尺寸变体HTML演示:\n{}", size_demo_html);
}

/// 测试颜色变体
fn test_color_variants() {
    println!("\n--- 测试颜色变体 ---");

    // 创建不同颜色主题的按钮
    let primary_button = css!("background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    let secondary_button = css!("background: #6c757d; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    let success_button = css!("background: #28a745; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    let danger_button = css!("background: #dc3545; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    let warning_button = css!("background: #ffc107; color: #212529; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");
    let info_button = css!("background: #17a2b8; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;");

    // 轮廓按钮变体
    let outline_primary = css!("background: transparent; color: #007bff; padding: 8px 16px; border: 1px solid #007bff; border-radius: 4px; cursor: pointer;");
    let outline_secondary = css!("background: transparent; color: #6c757d; padding: 8px 16px; border: 1px solid #6c757d; border-radius: 4px; cursor: pointer;");

    println!("主要按钮类名: {}", primary_button);
    println!("次要按钮类名: {}", secondary_button);
    println!("成功按钮类名: {}", success_button);
    println!("危险按钮类名: {}", danger_button);
    println!("警告按钮类名: {}", warning_button);
    println!("信息按钮类名: {}", info_button);
    println!("轮廓主要按钮类名: {}", outline_primary);
    println!("轮廓次要按钮类名: {}", outline_secondary);

    // 生成颜色变体演示HTML
    let color_demo_html = format!(
        r#"<div class="color-variants-demo">
  <h3>实心按钮</h3>
  <button class="{}">主要</button>
  <button class="{}">次要</button>
  <button class="{}">成功</button>
  <button class="{}">危险</button>
  <button class="{}">警告</button>
  <button class="{}">信息</button>

  <h3>轮廓按钮</h3>
  <button class="{}">主要轮廓</button>
  <button class="{}">次要轮廓</button>
</div>"#,
        primary_button,
        secondary_button,
        success_button,
        danger_button,
        warning_button,
        info_button,
        outline_primary,
        outline_secondary
    );

    println!("\n颜色变体HTML演示:\n{}", color_demo_html);
}

/// 测试状态变体
fn test_state_variants() {
    println!("\n--- 测试状态变体 ---");

    // 创建状态变体管理器
    // let state_variants = StateVariantManager::new();

    // 定义不同状态的样式
    let normal_state = css!("background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;");
    let hover_state = css!("background: #0056b3; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,123,255,0.3);");
    let active_state = css!("background: #004085; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; transform: translateY(0); box-shadow: 0 2px 4px rgba(0,123,255,0.3);");
    let disabled_state = css!("background: #6c757d; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: not-allowed; opacity: 0.6;");
    let loading_state = css!("background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: wait; opacity: 0.8;");

    println!("正常状态类名: {}", normal_state);
    println!("悬停状态类名: {}", hover_state);
    println!("激活状态类名: {}", active_state);
    println!("禁用状态类名: {}", disabled_state);
    println!("加载状态类名: {}", loading_state);

    // 生成状态变体演示HTML
    let state_demo_html = format!(
        r#"<div class="state-variants-demo">
  <button class="{}">正常状态</button>
  <button class="{}" onmouseover="this.className='{}'">悬停效果</button>
  <button class="{}" disabled>禁用状态</button>
  <button class="{}">加载状态 <span class="spinner">⟳</span></button>
</div>"#,
        normal_state, normal_state, hover_state, disabled_state, loading_state
    );

    println!("\n状态变体HTML演示:\n{}", state_demo_html);
}

/// 测试响应式变体
fn test_responsive_variants() {
    println!("\n--- 测试响应式变体 ---");

    // 创建响应式断点
    let breakpoints = ResponsiveManager::new();
    println!("响应式断点配置:");

    // 获取所有断点信息
    let all_breakpoints = breakpoints.get_all_breakpoints();
    for (name, breakpoint) in all_breakpoints {
        println!("  {}: {}", name, breakpoint.media_query);
    }

    // 创建响应式网格样式
    let mobile_grid = css!("display: grid; grid-template-columns: 1fr; gap: 8px; padding: 8px;");
    let tablet_grid =
        css!("display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; padding: 12px;");
    let desktop_grid =
        css!("display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; padding: 16px;");
    let large_desktop_grid =
        css!("display: grid; grid-template-columns: repeat(4, 1fr); gap: 20px; padding: 20px;");

    println!("\n响应式网格样式:");
    println!("移动端网格类名: {}", mobile_grid);
    println!("平板网格类名: {}", tablet_grid);
    println!("桌面网格类名: {}", desktop_grid);
    println!("大屏网格类名: {}", large_desktop_grid);

    // 生成响应式CSS（需要手动处理媒体查询）
    let responsive_css = format!(
        r#"/* 移动端优先 */
.responsive-grid {{
  /* 基础样式使用移动端 */
}}

/* 平板 */
@media (min-width: {}px) {{
  .responsive-grid {{
    /* 平板样式 */
  }}
}}

/* 桌面 */
@media (min-width: {}px) {{
  .responsive-grid {{
    /* 桌面样式 */
  }}
}}

/* 大屏 */
@media (min-width: {}px) {{
  .responsive-grid {{
    /* 大屏样式 */
  }}
}}"#,
        768, 1024, 1200
    );

    println!("\n响应式CSS模板:\n{}", responsive_css);
}

/// 测试变体组合
fn test_variant_combinations() {
    println!("\n--- 测试变体组合 ---");

    // 创建变体解析器
    let resolver = VariantResolver::new();

    // 组合不同的变体
    let small_primary = css!("padding: 4px 8px; font-size: 12px; background: #007bff; color: white; border: none; border-radius: 3px; cursor: pointer;");
    let large_secondary = css!("padding: 12px 24px; font-size: 16px; background: #6c757d; color: white; border: none; border-radius: 6px; cursor: pointer;");
    let medium_outline_success = css!("padding: 8px 16px; font-size: 14px; background: transparent; color: #28a745; border: 1px solid #28a745; border-radius: 4px; cursor: pointer;");

    println!("小尺寸主要按钮: {}", small_primary);
    println!("大尺寸次要按钮: {}", large_secondary);
    println!("中尺寸轮廓成功按钮: {}", medium_outline_success);

    // 生成组合变体演示
    let combination_demo_html = format!(
        r#"<div class="variant-combinations-demo">
  <h3>变体组合演示</h3>
  <button class="{}">小主要</button>
  <button class="{}">大次要</button>
  <button class="{}">中轮廓成功</button>
</div>"#,
        small_primary, large_secondary, medium_outline_success
    );

    println!("\n变体组合HTML演示:\n{}", combination_demo_html);
}

/// 测试条件样式
fn test_conditional_styles() {
    println!("\n--- 测试条件样式 ---");

    // 创建条件样式管理器
    let conditional = ConditionalStyleManager::new();

    // 模拟不同条件
    let is_primary = true;
    let is_large = false;
    let is_disabled = false;
    let is_loading = true;

    // 根据条件生成样式
    let base_style = css!("padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; transition: all 0.2s;");

    let color_style = if is_primary {
        css!("background: #007bff; color: white;")
    } else {
        css!("background: #f8f9fa; color: #212529; border: 1px solid #dee2e6;")
    };

    let size_style = if is_large {
        css!("padding: 12px 24px; font-size: 16px;")
    } else {
        css!("padding: 8px 16px; font-size: 14px;")
    };

    let state_style = if is_disabled {
        css!("opacity: 0.6; cursor: not-allowed;")
    } else if is_loading {
        css!("opacity: 0.8; cursor: wait;")
    } else {
        css!("opacity: 1; cursor: pointer;")
    };

    println!("基础样式: {}", base_style);
    println!("颜色样式: {}", color_style);
    println!("尺寸样式: {}", size_style);
    println!("状态样式: {}", state_style);

    // 生成条件样式演示
    let conditional_html = format!(
        r#"<button class="{} {} {} {}">条件样式按钮</button>"#,
        base_style, color_style, size_style, state_style
    );

    println!("\n条件样式HTML: {}", conditional_html);

    // 显示当前条件
    println!("\n当前条件:");
    println!("  is_primary: {}", is_primary);
    println!("  is_large: {}", is_large);
    println!("  is_disabled: {}", is_disabled);
    println!("  is_loading: {}", is_loading);
}

/// 测试优先级管理
fn test_priority_management() {
    println!("\n--- 测试优先级管理 ---");

    // 创建优先级管理器
    let _priority_manager = PriorityManager::new();

    // 定义不同优先级的样式
    let base_style = css!("color: black; font-size: 14px; padding: 8px;"); // 优先级 1
    let theme_style = css!("color: blue; background: lightblue;"); // 优先级 2
    let variant_style = css!("font-size: 16px; font-weight: bold;"); // 优先级 3
    let state_style = css!("color: red; background: pink;"); // 优先级 4 (最高)

    println!("基础样式 (优先级 1): {}", base_style);
    println!("主题样式 (优先级 2): {}", theme_style);
    println!("变体样式 (优先级 3): {}", variant_style);
    println!("状态样式 (优先级 4): {}", state_style);

    // 按优先级顺序组合样式
    let combined_classes = format!(
        "{} {} {} {}",
        base_style, theme_style, variant_style, state_style
    );

    println!("\n组合后的类名: {}", combined_classes);

    // 生成优先级演示HTML
    let priority_demo_html = format!(
        r#"<div class="priority-demo">
  <div class="{}">基础样式</div>
  <div class="{} {}">基础 + 主题</div>
  <div class="{} {} {}">基础 + 主题 + 变体</div>
  <div class="{}">全部样式 (最终效果)</div>
</div>"#,
        base_style,
        base_style,
        theme_style,
        base_style,
        theme_style,
        variant_style,
        combined_classes
    );

    println!("\n优先级演示HTML:\n{}", priority_demo_html);

    // 说明优先级规则
    println!("\n优先级规则说明:");
    println!("1. 基础样式 - 最低优先级，提供默认外观");
    println!("2. 主题样式 - 覆盖基础样式的主题相关属性");
    println!("3. 变体样式 - 覆盖尺寸、颜色等变体属性");
    println!("4. 状态样式 - 最高优先级，覆盖所有其他样式");
}
