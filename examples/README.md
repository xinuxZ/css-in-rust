# CSS-in-Rust Examples

这个目录包含了 CSS-in-Rust 库的各种使用示例。

## 运行示例

### 基础示例

```bash
# 基础 CSS 宏使用
cargo run --bin basic

# 最简变体系统演示
cargo run --bin minimal_demo

# 死代码消除演示
cargo run --bin dead_code_elimination

# 简单变体演示
cargo run --bin simple_variant_demo
```

### 高级示例

```bash
# 完整项目示例
cargo run --bin complete_project_example

# 变体系统演示
cargo run --bin variant_system_demo
cargo run --bin variant_system_simple
cargo run --bin variant_system_working
cargo run --bin variant_system_fixed

# 综合变体系统演示（可能需要修复）
cargo run --bin variant_system_comprehensive
```

## 示例说明

- **basic.rs**: 展示基础的 `css!` 宏使用方法
- **minimal_demo.rs**: 最简单的变体系统使用示例
- **dead_code_elimination.rs**: 演示 CSS 死代码消除功能
- **simple_variant_demo.rs**: 简单的变体系统演示
- **complete_project_example.rs**: 完整的项目使用示例
- **variant_system_*.rs**: 各种变体系统的使用方法和演示

## 编译和测试

```bash
# 检查所有示例的编译
cargo check

# 编译所有示例
cargo build

# 运行特定示例
cargo run --bin <example_name>
```

## 注意事项

- 确保主项目 `css-in-rust` 已经正确编译
- 某些高级示例可能需要特定的功能特性
- 如果遇到编译错误，请检查依赖版本和 API 兼容性
