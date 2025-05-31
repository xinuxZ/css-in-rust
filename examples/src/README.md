# CSS-in-Rust 示例集合

本目录包含了 CSS-in-Rust 项目的各种示例，展示了不同功能和使用场景。

## 可运行的示例

### 1. minimal_demo.rs - 最简示例
**状态**: ✅ 可运行

展示最基础的变体系统功能：
- 创建简单变体管理器
- 注册和激活变体
- 获取当前样式

```bash
# 运行命令
RUSTFLAGS='-A unused-crate-dependencies' cargo run --example minimal_demo --no-default-features
```

### 2. variant_system_working.rs - 完整变体系统示例
**状态**: ✅ 可运行

展示完整的变体系统功能：
- 尺寸变体（small, medium, large）
- 颜色变体（primary, secondary, success, danger）
- 状态变体（hover, focus, disabled）
- 变体组合和优先级处理

```bash
# 运行命令
RUSTFLAGS='-A unused-crate-dependencies' cargo run --example variant_system_working --no-default-features
```

## 需要修复的示例

### 3. basic.rs
**状态**: ❌ 编译错误
**问题**: 未使用的外部依赖错误

### 4. variant_system_simple.rs
**状态**: ❌ 编译错误
**问题**: 未使用的外部依赖错误

### 5. variant_system_comprehensive.rs
**状态**: ❌ 编译错误
**问题**: 导入路径错误和未使用的依赖

### 6. variant_system_demo.rs
**状态**: ❌ 编译错误
**问题**: 导入路径错误

## 变体系统核心功能

### SimpleVariantManager
变体管理器提供以下核心功能：

```rust
// 创建管理器
let mut manager = SimpleVariantManager::new();

// 注册变体
let style = SimpleVariantStyle {
    properties: HashMap::new(),
    priority: 1,
};
manager.register_variant("variant-name".to_string(), style);

// 激活/停用变体
manager.activate_variant("variant-name".to_string());
manager.deactivate_variant("variant-name");

// 获取当前样式
let styles = manager.get_current_styles();
```

### 变体优先级
变体系统支持优先级处理：
- 优先级数字越高，样式优先级越高
- 相同属性的样式会被高优先级覆盖
- 不同属性的样式会合并

### 变体类型示例

#### 尺寸变体
```rust
let size_large = SimpleVariantStyle {
    properties: {
        let mut props = HashMap::new();
        props.insert("width".to_string(), "160px".to_string());
        props.insert("height".to_string(), "80px".to_string());
        props.insert("padding".to_string(), "16px".to_string());
        props
    },
    priority: 1,
};
```

#### 颜色变体
```rust
let color_primary = SimpleVariantStyle {
    properties: {
        let mut props = HashMap::new();
        props.insert("background-color".to_string(), "#007bff".to_string());
        props.insert("color".to_string(), "white".to_string());
        props
    },
    priority: 2,
};
```

#### 状态变体
```rust
let state_hover = SimpleVariantStyle {
    properties: {
        let mut props = HashMap::new();
        props.insert("transform".to_string(), "scale(1.05)".to_string());
        props.insert("box-shadow".to_string(), "0 4px 8px rgba(0,0,0,0.2)".to_string());
        props
    },
    priority: 3,
};
```

## 编译注意事项

由于项目包含多个可选依赖，在编译示例时建议使用以下命令格式：

```bash
# 忽略未使用依赖警告
RUSTFLAGS='-A unused-crate-dependencies' cargo run --example <example_name> --no-default-features
```

## 项目架构

```
css-in-rust/
├── src/
│   ├── variants/
│   │   ├── minimal.rs          # 简化变体系统
│   │   ├── mod.rs             # 变体模块导出
│   │   ├── state_variants.rs  # 状态变体
│   │   └── ...
│   └── ...
├── examples/
│   ├── minimal_demo.rs        # ✅ 最简示例
│   ├── variant_system_working.rs # ✅ 完整示例
│   └── ...
└── Cargo.toml
```

## 下一步开发建议

1. **修复现有示例**: 解决编译错误，统一依赖管理
2. **扩展变体类型**: 添加响应式变体、动画变体等
3. **性能优化**: 优化样式合并算法
4. **文档完善**: 添加更多使用场景和最佳实践
5. **测试覆盖**: 增加单元测试和集成测试
