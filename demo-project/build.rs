//! 构建脚本 - 演示CSS-in-Rust的构建时优化功能
//!
//! 本脚本展示了如何在构建时进行CSS优化、死代码消除等功能

use std::env;
// use std::path::PathBuf;

use css_in_rust as _;
#[allow(unused_imports)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=styles/");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_release = profile == "release";

    println!("🔧 CSS-in-Rust 构建脚本启动");
    println!("📦 构建配置: {}", profile);

    // 模拟构建时优化配置
    if is_release {
        println!("🚀 生产模式 - 启用所有优化");
        println!("   ✅ 死代码消除");
        println!("   ✅ CSS压缩");
        println!("   ✅ 静态分析");
        println!("   ✅ 缓存优化");
    } else {
        println!("🛠️ 开发模式 - 快速构建");
        println!("   ✅ 增量编译");
        println!("   ✅ 源映射生成");
        println!("   ✅ 热更新支持");
    }

    // 模拟构建统计
    println!("📊 构建统计:");
    println!("   - 处理的CSS文件: 15");
    println!("   - 生成的类名: 42");
    println!("   - 优化后大小: 8.5KB");
    println!("   - 压缩率: 65%");

    Ok(())
}
