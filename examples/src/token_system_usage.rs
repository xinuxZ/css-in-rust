//! 设计令牌系统使用示例
//!
//! 本示例展示如何使用重构后的设计令牌系统

use ant_design_dioxus::theme::{
    get_global_token_system, init_global_token_system, DesignTokenSystem, ThemeVariant,
    TokenSystemConfig, TokenValue,
};
use std::collections::HashMap;

fn main() {
    println!("=== 设计令牌系统使用示例 ===");

    // 示例1: 基本使用
    basic_usage_example();

    // 示例2: 主题切换
    theme_switching_example();

    // 示例3: 自定义令牌
    custom_tokens_example();

    // 示例4: CSS生成
    css_generation_example();

    // 示例5: 全局令牌系统
    global_system_example();

    // 示例6: 主题变体创建
    theme_variant_example();
}

/// 基本使用示例
fn basic_usage_example() {
    println!("\n--- 基本使用示例 ---");

    let mut system = DesignTokenSystem::new();

    // 获取令牌值
    match system.get_token("color.primary.500") {
        Ok(value) => println!("Primary color: {:?}", value),
        Err(e) => println!("Error getting token: {:?}", e),
    }

    // 设置自定义令牌
    let _ = system.set_token(
        "custom.brand.color",
        TokenValue::String("#1890ff".to_string()),
    );

    // 获取自定义令牌
    match system.get_token("custom.brand.color") {
        Ok(value) => println!("Custom brand color: {:?}", value),
        Err(e) => println!("Error getting custom token: {:?}", e),
    }
}

/// 主题切换示例
fn theme_switching_example() {
    println!("\n--- 主题切换示例 ---");

    let mut system = DesignTokenSystem::new();

    println!("当前主题: {:?}", system.get_current_theme());

    // 获取浅色主题的背景色
    match system.get_token("color.background.default") {
        Ok(value) => println!("浅色主题背景色: {:?}", value),
        Err(e) => println!("Error: {:?}", e),
    }

    // 切换到深色主题
    system.switch_theme(ThemeVariant::Dark);
    println!("切换后主题: {:?}", system.get_current_theme());

    // 获取深色主题的背景色
    match system.get_token("color.background.default") {
        Ok(value) => println!("深色主题背景色: {:?}", value),
        Err(e) => println!("Error: {:?}", e),
    }
}

/// 自定义令牌示例
fn custom_tokens_example() {
    println!("\n--- 自定义令牌示例 ---");

    let mut system = DesignTokenSystem::new();

    // 批量设置令牌
    let mut custom_tokens = HashMap::new();
    custom_tokens.insert(
        "brand.primary".to_string(),
        TokenValue::String("#1890ff".to_string()),
    );
    custom_tokens.insert(
        "brand.secondary".to_string(),
        TokenValue::String("#722ed1".to_string()),
    );
    custom_tokens.insert("spacing.custom".to_string(), TokenValue::Number(24.0));

    match system.set_tokens_batch(custom_tokens) {
        Ok(_) => println!("批量设置令牌成功"),
        Err(e) => println!("批量设置令牌失败: {}", e),
    }

    // 列出所有令牌
    let tokens = system.list_tokens();
    println!("令牌总数: {}", tokens.len());

    // 搜索令牌
    let brand_tokens = system.search_tokens("brand");
    println!("品牌相关令牌: {:?}", brand_tokens);
}

/// CSS生成示例
fn css_generation_example() {
    println!("\n--- CSS生成示例 ---");

    let mut system = DesignTokenSystem::new();

    // 生成CSS变量
    match system.generate_css_variables() {
        Ok(css) => {
            println!("CSS变量 (前100字符):");
            let preview = if css.len() > 100 { &css[..100] } else { &css };
            println!("{}", preview);
        }
        Err(e) => println!("生成CSS变量失败: {}", e),
    }

    // 生成主题CSS
    match system.generate_theme_css() {
        Ok(css) => {
            println!("\n主题CSS (前100字符):");
            let preview = if css.len() > 100 { &css[..100] } else { &css };
            println!("{}", preview);
        }
        Err(e) => println!("生成主题CSS失败: {}", e),
    }

    // 生成组件CSS
    match system.generate_component_css("button") {
        Ok(css) => {
            println!("\n按钮组件CSS (前100字符):");
            let preview = if css.len() > 100 { &css[..100] } else { &css };
            println!("{}", preview);
        }
        Err(e) => println!("生成组件CSS失败: {}", e),
    }
}

/// 全局令牌系统示例
fn global_system_example() {
    println!("\n--- 全局令牌系统示例 ---");

    // 初始化全局系统
    let config = TokenSystemConfig {
        css_prefix: "my-app".to_string(),
        enable_cache: true,
        minify_css: true,
        strict_mode: true,
    };
    init_global_token_system(config);

    // 使用全局系统
    let global_system = get_global_token_system();
    println!("全局系统当前主题: {:?}", global_system.get_current_theme());

    // 获取CSS变量名
    let css_var_name = global_system.get_css_var_name("color.primary.500");
    println!("CSS变量名: {}", css_var_name);
}

/// 主题变体创建示例
fn theme_variant_example() {
    println!("\n--- 主题变体创建示例 ---");

    let mut system = DesignTokenSystem::new();

    // 创建自定义主题变体
    let mut overrides = HashMap::new();
    overrides.insert(
        "color.primary.500".to_string(),
        TokenValue::String("#ff4d4f".to_string()),
    );
    overrides.insert(
        "color.background.default".to_string(),
        TokenValue::String("#f0f0f0".to_string()),
    );

    match system.create_theme_variant(
        ThemeVariant::Light,
        ThemeVariant::Custom("red-theme".to_string()),
        overrides,
    ) {
        Ok(_) => {
            println!("创建自定义主题变体成功");

            // 切换到自定义主题
            system.switch_theme(ThemeVariant::Custom("red-theme".to_string()));

            // 验证自定义主题的值
            match system.get_token("color.primary.500") {
                Ok(value) => println!("自定义主题主色: {:?}", value),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        Err(e) => println!("创建自定义主题变体失败: {}", e),
    }

    // 列出支持的主题
    let themes = system.get_supported_themes();
    println!("支持的主题: {:?}", themes);
}

/// 高级功能示例
#[allow(dead_code)]
fn advanced_features_example() {
    println!("\n--- 高级功能示例 ---");

    let mut system = DesignTokenSystem::new();

    // 令牌验证
    let errors = system.validate_tokens();
    if errors.is_empty() {
        println!("所有令牌验证通过");
    } else {
        println!("发现 {} 个验证错误", errors.len());
    }

    // 导出令牌为JSON
    match system.export_tokens_json() {
        Ok(json) => {
            println!("令牌JSON导出成功 (前100字符):");
            let preview = if json.len() > 100 {
                &json[..100]
            } else {
                &json
            };
            println!("{}", preview);
        }
        Err(e) => println!("导出JSON失败: {}", e),
    }

    // 计算表达式
    match system.compute_expression("16 * 1.5") {
        Ok(result) => println!("表达式计算结果: {:?}", result),
        Err(e) => println!("表达式计算失败: {:?}", e),
    }

    // 获取令牌引用
    let references = system.get_token_references("color.primary.500");
    println!("引用 color.primary.500 的令牌: {:?}", references);
}
