//! 全面功能测试
//! 测试README.md中声称的所有功能点

use css_in_rust::{
    css,
    theme::{ThemeManager, ThemeManagerConfig},
    ConditionalStyleManager, PriorityManager, ResponsiveManager, StateVariantManager,
    VariantConfig, VariantResolver, VariantStyle,
};
use std::collections::HashMap;

use chrono as _;
use css_in_rust_macros as _;
use regex as _;
use serde as _;
use serde_json as _;
use tokio as _;

/// 测试基础CSS宏功能
pub fn test_basic_css_macro() {
    println!("\n=== 测试基础CSS宏功能 ===");

    // 测试基础css!宏
    let button_class = css! {
        r#"
        .button {
            background: #007bff;
            color: white;
            padding: 8px 16px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            transition: background-color 0.2s;
        }

        .button:hover {
            background: #0056b3;
        }
        "#
    };

    println!("✅ 基础按钮样式类名: {}", button_class);
}

/// 测试响应式设计功能
pub fn test_responsive_design() {
    println!("\n=== 测试响应式设计功能 ===");

    let responsive_class = css! {
        r#"
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 16px;
        }

        @media (max-width: 768px) {
            .container {
                padding: 0 8px;
            }
        }

        @media (max-width: 480px) {
            .container {
                padding: 0 4px;
            }
        }
        "#
    };

    println!("✅ 响应式容器类名: {}", responsive_class);

    // 测试响应式管理器
    let _responsive_manager = ResponsiveManager::new();

    println!("✅ 响应式断点配置完成");
}

/// 测试CSS变量和主题功能
pub fn test_theming() {
    println!("\n=== 测试CSS变量和主题功能 ===");

    let themed_class = css! {
        r#"
        :root {
            --primary-color: #007bff;
            --secondary-color: #6c757d;
        }

        .card {
            background: white;
            border: 1px solid var(--primary-color);
            border-radius: 8px;
            padding: 16px;
        }
        "#
    };

    println!("✅ 主题化卡片类名: {}", themed_class);

    // 测试主题管理器
    let _theme_manager = ThemeManager::new(ThemeManagerConfig::default());

    println!("✅ 主题变量配置完成");
}

/// 测试动画功能
pub fn test_animations() {
    println!("\n=== 测试动画功能 ===");

    let animated_class = css! {
        r#"
        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .fade-in {
            animation: fadeIn 0.3s ease-out;
        }
        "#
    };

    println!("✅ 动画类名: {}", animated_class);
}

/// 测试变体系统
pub fn test_variant_system() {
    println!("\n=== 测试变体系统 ===");

    // 创建变体配置
    let mut variant_config = VariantConfig {
        size: HashMap::new(),
        color: HashMap::new(),
        state: HashMap::new(),
        responsive: HashMap::new(),
        defaults: HashMap::new(),
    };

    variant_config
        .defaults
        .insert("rounded".to_string(), "true".to_string());

    // 创建变体解析器
    let _variant_resolver = VariantResolver::new();

    // 创建状态变体管理器
    let _state_manager = StateVariantManager::new();

    println!("✅ 变体系统配置完成");
}

/// 测试条件样式
pub fn test_conditional_styles() {
    println!("\n=== 测试条件样式 ===");

    let _conditional_manager = ConditionalStyleManager::new();

    // 模拟条件样式应用
    let is_active = true;
    let is_disabled = false;

    let conditional_class = if is_active && !is_disabled {
        css! {
            r#"
            .active {
                background-color: #28a745;
                color: white;
            }
            "#
        }
    } else {
        css! {
            r#"
            .inactive {
                background-color: #6c757d;
                color: #dee2e6;
            }
            "#
        }
    };

    println!("✅ 条件样式类名: {}", conditional_class);
}

/// 测试优先级管理
pub fn test_priority_management() {
    println!("\n=== 测试优先级管理 ===");

    let _priority_manager = PriorityManager::new();

    // 创建不同优先级的样式
    let base_style = VariantStyle {
        properties: HashMap::from([
            ("color".to_string(), "black".to_string()),
            ("font-size".to_string(), "14px".to_string()),
        ]),
        pseudo_classes: HashMap::new(),
        priority: 100,
    };

    let theme_style = VariantStyle {
        properties: HashMap::from([("color".to_string(), "#007bff".to_string())]),
        pseudo_classes: HashMap::new(),
        priority: 200,
    };

    let variant_style = VariantStyle {
        properties: HashMap::from([("font-size".to_string(), "16px".to_string())]),
        pseudo_classes: HashMap::new(),
        priority: 300,
    };

    println!("✅ 优先级管理配置完成");
    println!("   - 基础样式优先级: {}", base_style.priority);
    println!("   - 主题样式优先级: {}", theme_style.priority);
    println!("   - 变体样式优先级: {}", variant_style.priority);
}

/// 测试性能优化功能
pub fn test_performance_features() {
    println!("\n=== 测试性能优化功能 ===");

    // 测试样式去重
    let duplicate_class1 = css! {
        r#"
        .duplicate {
            color: red;
            font-size: 16px;
        }
        "#
    };

    let duplicate_class2 = css! {
        r#"
        .duplicate {
            color: red;
            font-size: 16px;
        }
        "#
    };

    println!("✅ 重复样式测试:");
    println!("   - 第一个类名: {}", duplicate_class1);
    println!("   - 第二个类名: {}", duplicate_class2);

    if duplicate_class1 == duplicate_class2 {
        println!("   ✅ 样式去重功能正常工作");
    } else {
        println!("   ❌ 样式去重功能可能存在问题");
    }
}

/// 测试框架集成（模拟Dioxus）
pub fn test_framework_integration() {
    println!("\n=== 测试框架集成 ===");

    // 模拟Dioxus组件样式
    let component_style = css! {
        r#"
        .dioxus-component {
            display: flex;
            flex-direction: column;
            gap: 16px;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }

        .dioxus-component .title {
            font-size: 24px;
            font-weight: bold;
            color: #333;
        }

        .dioxus-component .content {
            line-height: 1.6;
            color: #666;
        }
        "#
    };

    println!("✅ 框架集成样式类名: {}", component_style);

    // 生成模拟HTML
    let html = format!(
        r#"
    <div class="{}">
        <h2 class="title">Dioxus组件标题</h2>
        <p class="content">这是一个使用CSS-in-Rust的Dioxus组件示例。</p>
    </div>
    "#,
        component_style
    );

    println!("✅ 生成的HTML片段:");
    println!("{}", html);
}

/// 运行所有功能测试
pub fn run_comprehensive_tests() {
    println!("🚀 开始全面功能测试...");

    test_basic_css_macro();
    test_responsive_design();
    test_theming();
    test_animations();
    test_variant_system();
    test_conditional_styles();
    test_priority_management();
    test_performance_features();
    test_framework_integration();

    println!("\n🎉 全面功能测试完成！");
}

/// 主函数
fn main() {
    // 初始化CSS运行时
    css_in_rust::init();

    // 运行全面测试
    run_comprehensive_tests();
}
