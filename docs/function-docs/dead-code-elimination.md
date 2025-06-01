# CSS Dead Code Elimination

本文档详细介绍了 CSS-in-Rust 库中的死代码消除功能，包括其工作原理、配置选项和使用方法。

## 概述

CSS 死代码消除是一个强大的优化功能，它可以在构建时自动检测和移除未使用的 CSS 规则，从而减少最终的 CSS 文件大小，提高应用性能。

### 主要特性

- 🔍 **静态分析**: 自动扫描 Rust 源代码，检测 `css!` 宏的使用情况
- 🗑️ **死代码消除**: 移除未使用的 CSS 规则和选择器
- 📊 **使用情况报告**: 生成详细的 CSS 使用情况分析报告
- ⚡ **构建时优化**: 在编译期间进行优化，不影响运行时性能
- 🎯 **精确控制**: 支持多种配置选项，满足不同的优化需求
- 🔧 **CLI 工具**: 提供命令行工具，方便集成到构建流程中

## 工作原理

### 1. 静态分析阶段

静态分析器会扫描项目中的所有 Rust 文件，查找 `css!` 和 `css_if!` 宏的使用：

```rust
// 这些 CSS 类会被标记为"已使用"
let button_style = css!("
    .btn { padding: 10px; }
    .btn-primary { background: blue; }
");

let conditional_style = css_if!(is_active, "
    .active { color: red; }
");
```

### 2. 使用情况跟踪

系统会提取并跟踪以下信息：
- CSS 类选择器 (`.class-name`)
- CSS ID 选择器 (`#id-name`)
- 宏调用位置和上下文
- 条件使用情况

### 3. 死代码检测

优化器会分析 CSS 规则，识别未被任何代码引用的选择器：

```css
/* 这些规则会被保留 */
.btn { padding: 10px; }
.btn-primary { background: blue; }

/* 这些规则会被移除 */
.unused-class { color: red; }
#never-referenced { display: none; }
```

### 4. 优化输出

最终输出只包含被实际使用的 CSS 规则，大大减少文件大小。

## 配置选项

### OptimizerConfig

```rust
use css_in_rust::core::optimizer::OptimizerConfig;

let config = OptimizerConfig {
    // 基础优化选项
    minify: true,                    // 启用 CSS 压缩（包含颜色、字体等优化）
    vendor_prefix: true,             // 启用厂商前缀处理
    analyze_dependencies: true,      // 分析依赖关系

    // 死代码消除选项
    enable_dead_code_elimination: true,  // 启用死代码消除（替代 remove_unused）
    source_paths: vec![              // 源代码路径
        PathBuf::from("src"),
        PathBuf::from("examples"),
    ],
    aggressive_elimination: false,   // 激进消除模式
    usage_threshold: 0.0,           // 使用阈值 (0.0-1.0)

    analyze_dependencies: true,      // 分析依赖关系
};
```

### BuildConfig

```rust
use css_in_rust::build_tools::BuildConfig;

let config = BuildConfig {
    project_root: PathBuf::from("."),              // 项目根目录
    output_dir: PathBuf::from("target/css-optimized"), // 输出目录
    enable_dead_code_elimination: true,             // 启用死代码消除
    generate_reports: true,                         // 生成分析报告
    usage_threshold: 0.0,                          // 使用阈值
    aggressive_elimination: false,                  // 激进模式
};
```

## 使用方法

### 1. 编程接口

#### 基础优化

```rust
use css_in_rust::core::optimizer::{CssOptimizer, OptimizerConfig};

// 创建优化器
let config = OptimizerConfig {
    enable_dead_code_elimination: true,
    ..Default::default()
};
let mut optimizer = CssOptimizer::with_config(config);

// 跟踪 CSS 使用情况
optimizer.track_css_usage(".btn", None);
optimizer.track_css_usage(".btn-primary", None);
optimizer.track_css_usage("#main-header", None);

// 优化 CSS
let css = "
    .btn { padding: 10px; }
    .btn-primary { background: blue; }
    .btn-secondary { background: gray; }
    .unused { color: red; }
    #main-header { font-size: 24px; }
    #unused-id { display: none; }
";

let optimized = optimizer.optimize(css)?;
print!("Optimized CSS: {}", optimized);
```

#### 构建时处理

```rust
use css_in_rust::build_tools::{CssBuildProcessor, BuildConfig};

// 配置构建处理器
let config = BuildConfig {
    project_root: std::env::current_dir()?,
    enable_dead_code_elimination: true,
    aggressive_elimination: false,
    ..Default::default()
};

// 运行构建过程
let processor = CssBuildProcessor::with_config(config);
let result = processor.run()?;

println!("Files processed: {}", result.processed_files.len());
println!("Total savings: {} bytes ({:.2}%)",
    result.savings_bytes, result.savings_percentage);
```

#### 静态分析

```rust
use css_in_rust::build_tools::StaticAnalyzer;

// 创建分析器
let analyzer = StaticAnalyzer::new(std::env::current_dir()?)
    .with_include_patterns(vec![
        "**/*.rs".to_string(),
        "**/*.html".to_string(),
    ])
    .with_exclude_patterns(vec![
        "target/**".to_string(),
        "**/.git/**".to_string(),
    ]);

// 执行分析
let report = analyzer.analyze()?;

println!("Files analyzed: {}", report.analyzed_files.len());
println!("CSS classes found: {}", report.used_classes.len());
println!("CSS IDs found: {}", report.used_ids.len());

// 保存报告
analyzer.save_report(&report, &PathBuf::from("analysis-report.json"))?;
```

### 2. 命令行工具

#### 构建整个项目

```bash
# 基础构建
css-optimizer build

# 自定义配置
css-optimizer build \
    --project-root ./my-project \
    --output-dir ./dist/css \
    --aggressive \
    --usage-threshold 0.1

# 禁用某些功能
css-optimizer build \
    --no-dead-code-elimination \
    --no-reports
```

#### 优化单个文件

```bash
# 优化单个 CSS 文件
css-optimizer optimize styles.css --output optimized.css

# 启用死代码消除
css-optimizer optimize styles.css \
    --output optimized.css \
    --enable-dead-code-elimination \
    --aggressive

# 禁用某些优化
css-optimizer optimize styles.css \
    --no-minify \
    --no-merge-rules
```

#### 静态分析

```bash
# 分析项目
css-optimizer analyze

# 保存分析报告
css-optimizer analyze --output analysis-report.json

# 指定项目根目录
css-optimizer analyze --project-root ./my-project
```

### 3. 构建脚本集成

#### Cargo.toml 配置

```toml
[dependencies]
css-in-rust = { version = "0.1", features = ["build-tools"] }

[[bin]]
name = "css-build"
path = "build/css_build.rs"
```

#### 构建脚本示例

```rust
// build/css_build.rs
use css_in_rust::build_tools::build_script;

fn main() {
    build_script::main();
}
```

#### 在 build.rs 中使用

```rust
// build.rs
use css_in_rust::build_tools::{CssBuildProcessor, BuildConfig};

fn main() {
    let config = BuildConfig {
        project_root: std::env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(".")),
        output_dir: PathBuf::from("target/css"),
        enable_dead_code_elimination: true,
        ..Default::default()
    };

    let processor = CssBuildProcessor::with_config(config);

    if let Err(e) = processor.run() {
        panic!("CSS build failed: {}", e);
    }

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=examples/");
}
```

## 高级功能

### 1. 激进消除模式

激进模式会更积极地移除可能未使用的 CSS 规则：

```rust
let config = OptimizerConfig {
    aggressive_elimination: true,
    usage_threshold: 0.1,  // 使用率低于 10% 的规则会被移除
    ..Default::default()
};
```

### 2. 使用阈值

设置最小使用阈值，只有使用频率超过阈值的规则才会被保留：

```rust
let config = OptimizerConfig {
    usage_threshold: 0.05,  // 只保留使用率超过 5% 的规则
    ..Default::default()
};
```

### 3. 自定义分析模式

```rust
let analyzer = StaticAnalyzer::new(project_root)
    .with_include_patterns(vec![
        "src/**/*.rs".to_string(),
        "templates/**/*.html".to_string(),
        "assets/**/*.css".to_string(),
    ])
    .with_exclude_patterns(vec![
        "target/**".to_string(),
        "tests/**".to_string(),
        "benches/**".to_string(),
    ])
    .with_dependency_analysis(true);
```

### 4. 条件编译支持

系统能够理解条件编译和动态 CSS：

```rust
// 条件 CSS 会被正确跟踪
let style = css_if!(is_mobile, "
    .mobile-only { display: block; }
");

// 特性门控的 CSS
#[cfg(feature = "dark-theme")]
let dark_style = css!("
    .dark-theme { background: black; }
");
```

## 性能优化建议

### 1. 合理设置包含/排除模式

```rust
// 只分析必要的文件
let analyzer = StaticAnalyzer::new(project_root)
    .with_include_patterns(vec![
        "src/**/*.rs".to_string(),      // 主要源代码
        "examples/**/*.rs".to_string(),  // 示例代码
    ])
    .with_exclude_patterns(vec![
        "target/**".to_string(),        // 构建输出
        "**/.git/**".to_string(),       // Git 文件
        "**/node_modules/**".to_string(), // Node 模块
        "tests/**".to_string(),         // 测试文件（如果不需要）
    ]);
```

### 2. 增量分析

```rust
// 只在源文件改变时重新分析
if source_files_changed() {
    let report = analyzer.analyze()?;
    cache_report(&report);
} else {
    let report = load_cached_report()?;
}
```

### 3. 并行处理

大型项目可以考虑并行处理多个文件：

```rust
use rayon::prelude::*;

// 并行处理 CSS 文件
let results: Vec<_> = css_files
    .par_iter()
    .map(|file| process_css_file(file, &usage_report))
    .collect();
```

## 故障排除

### 常见问题

#### 1. CSS 规则被错误移除

**问题**: 某些实际使用的 CSS 规则被移除了。

**解决方案**:
- 检查 `css!` 宏的使用是否正确
- 确保所有源文件都被包含在分析范围内
- 考虑禁用激进模式
- 降低使用阈值

```rust
let config = OptimizerConfig {
    aggressive_elimination: false,
    usage_threshold: 0.0,
    ..Default::default()
};
```

#### 2. 分析速度过慢

**问题**: 静态分析耗时过长。

**解决方案**:
- 优化包含/排除模式
- 减少分析的文件数量
- 使用增量分析
- 考虑并行处理

#### 3. 动态 CSS 未被识别

**问题**: 运行时生成的 CSS 类名未被正确跟踪。

**解决方案**:
- 手动调用 `track_css_usage()`
- 使用更宽松的匹配模式
- 在代码中添加注释标记

```rust
// 手动跟踪动态生成的类名
optimizer.track_css_usage(&format!(".dynamic-{}", id), None);

// 或者在代码中添加标记注释
// css-usage: .dynamic-class-prefix
```

### 调试技巧

#### 1. 启用详细日志

```rust
env_logger::init();

// 设置日志级别
std::env::set_var("RUST_LOG", "css_in_rust=debug");
```

#### 2. 生成详细报告

```rust
let config = BuildConfig {
    generate_reports: true,
    ..Default::default()
};

// 报告会包含详细的分析信息
let result = processor.run()?;
analyzer.save_report(&result.usage_report, &PathBuf::from("debug-report.json"))?;
```

#### 3. 逐步验证

```rust
// 先禁用死代码消除，确保基础功能正常
let config = OptimizerConfig {
    enable_dead_code_elimination: false,
    ..Default::default()
};

// 然后逐步启用各项功能
let config = OptimizerConfig {
    enable_dead_code_elimination: true,
    aggressive_elimination: false,
    usage_threshold: 0.0,
    ..Default::default()
};
```

## 最佳实践

### 1. 项目结构

```
project/
├── src/
│   ├── components/     # 组件代码
│   ├── styles/         # 共享样式
│   └── lib.rs
├── build/
│   └── css_build.rs    # CSS 构建脚本
├── target/
│   └── css-optimized/  # 优化后的 CSS
└── Cargo.toml
```

### 2. 命名约定

```rust
// 使用一致的 CSS 类命名
let button_style = css!("
    .btn { /* 基础样式 */ }
    .btn--primary { /* 主要变体 */ }
    .btn--secondary { /* 次要变体 */ }
    .btn__icon { /* 子元素 */ }
");
```

### 3. 模块化设计

```rust
// 将样式组织到模块中
mod button {
    use css_in_rust::css;

    pub fn base_style() -> String {
        css!("/* 基础按钮样式 */")
    }

    pub fn primary_style() -> String {
        css!("/* 主要按钮样式 */")
    }
}
```

### 4. 测试策略

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_optimization() {
        let optimizer = CssOptimizer::default();
        let result = optimizer.optimize("/* test css */");
        assert!(result.is_ok());
    }

    #[test]
    fn test_dead_code_elimination() {
        let mut optimizer = CssOptimizer::with_config(OptimizerConfig {
            enable_dead_code_elimination: true,
            ..Default::default()
        });

        optimizer.track_css_usage(".used-class", None);

        let css = ".used-class { color: red; } .unused-class { color: blue; }";
        let result = optimizer.optimize(css).unwrap();

        assert!(result.contains(".used-class"));
        assert!(!result.contains(".unused-class"));
    }
}
```

## 总结

CSS 死代码消除功能为 CSS-in-Rust 库提供了强大的优化能力，通过静态分析和智能优化，可以显著减少 CSS 文件大小，提高应用性能。合理配置和使用这些功能，可以在保证功能完整性的同时，获得最佳的优化效果。

关键要点：
- 🎯 **精确配置**: 根据项目需求调整优化参数
- 📊 **监控分析**: 定期检查优化报告和效果
- 🔧 **渐进优化**: 从保守设置开始，逐步调整到最佳状态
- 🧪 **充分测试**: 确保优化后的 CSS 功能正常
- 📚 **文档记录**: 记录配置决策和优化策略

通过遵循这些指导原则，您可以充分利用 CSS 死代码消除功能，构建更高效、更优化的 Web 应用。
