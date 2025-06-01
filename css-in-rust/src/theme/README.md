# 设计令牌系统重构文档

## 概述

本文档描述了设计令牌系统的重构方案，该重构基于**单一职责原则**对原有的 `design_token_system.rs` 和 `design_tokens.rs` 进行了模块化拆分和优化。

## 重构目标

### 问题分析

原有系统存在以下问题：

1. **职责混乱**：`design_token_system.rs` 和 `design_tokens.rs` 存在功能重叠
2. **代码重复**：令牌定义、值获取、CSS生成等功能在两个文件中都有实现
3. **耦合度高**：模块间依赖关系复杂，难以独立测试和维护
4. **扩展性差**：添加新功能需要修改多个文件
5. **缺乏抽象**：没有清晰的接口定义

### 重构原则

- **单一职责原则 (SRP)**：每个模块只负责一个明确的功能
- **开闭原则 (OCP)**：对扩展开放，对修改封闭
- **依赖倒置原则 (DIP)**：依赖抽象而非具体实现
- **接口隔离原则 (ISP)**：使用专门的接口而非大而全的接口

## 新架构设计

### 模块结构

```
src/theme/
├── token_definitions.rs    # 令牌定义和抽象接口
├── token_values.rs         # 令牌值存储和管理
├── token_resolver.rs       # 令牌解析和引用处理
├── css_generator.rs        # CSS生成和样式输出
├── token_system.rs         # 系统级API和高级功能
└── mod.rs                  # 模块导出和公共API
```

### 职责分工

#### 1. `token_definitions.rs` - 令牌定义层

**职责**：定义令牌的基础类型和抽象接口

- `TokenValue` 枚举：令牌值类型定义
- `TokenPath` 结构：令牌路径表示
- `ThemeVariant` 枚举：主题变体定义
- `TokenDefinitions` trait：令牌操作抽象接口
- `TokenValidationError` 枚举：验证错误类型

```rust
pub enum TokenValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Reference(String),
    Array(Vec<TokenValue>),
    Object(HashMap<String, TokenValue>),
}

pub trait TokenDefinitions {
    fn get_token_value(&self, path: &TokenPath, theme: ThemeVariant) -> Result<TokenValue, TokenValidationError>;
    fn set_token_value(&mut self, path: &TokenPath, value: TokenValue, theme: ThemeVariant) -> Result<(), String>;
    // ... 其他方法
}
```

#### 2. `token_values.rs` - 令牌值存储层

**职责**：管理令牌值的存储和主题变体

- `TokenValueStore`：令牌值存储容器
- `AntDesignTokenValues`：Ant Design 默认令牌值
- 主题管理：支持多主题切换和自定义主题

```rust
pub struct TokenValueStore {
    values: HashMap<ThemeVariant, HashMap<String, TokenValue>>,
    metadata: HashMap<String, TokenMetadata>,
}

pub struct AntDesignTokenValues;

impl AntDesignTokenValues {
    pub fn create_default_store() -> TokenValueStore {
        // 创建包含默认 Ant Design 令牌的存储
    }
}
```

#### 3. `token_resolver.rs` - 令牌解析层

**职责**：处理令牌引用解析和值计算

- 引用解析：处理 `{color.primary.500}` 形式的引用
- 循环引用检测：防止无限递归
- 表达式计算：支持基础数学运算
- 缓存机制：提高解析性能

```rust
pub struct TokenResolver {
    store: TokenValueStore,
    cache: HashMap<String, TokenValue>,
    resolution_stack: Vec<String>,
}

impl TokenResolver {
    pub fn resolve_token(&mut self, path: &TokenPath, theme: ThemeVariant) -> Result<TokenValue, TokenValidationError> {
        // 解析令牌，处理引用和计算
    }
}
```

#### 4. `css_generator.rs` - CSS生成层

**职责**：将令牌转换为CSS变量和样式

- CSS变量生成：`--ant-color-primary-500: #1890ff`
- 主题CSS生成：完整的主题样式表
- 组件样式类：特定组件的样式类
- 实用工具类：通用的工具样式类

```rust
pub struct CssGenerator {
    resolver: TokenResolver,
    prefix: String,
    minify: bool,
}

impl CssGenerator {
    pub fn generate_css_variables(&mut self, theme: ThemeVariant) -> Result<String, String> {
        // 生成CSS变量
    }

    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        // 生成完整主题CSS
    }
}
```

#### 5. `token_system.rs` - 系统集成层

**职责**：提供高级API和系统级功能

- 系统配置管理
- 全局令牌系统
- 批量操作
- 主题变体创建
- JSON导入导出

```rust
pub struct DesignTokenSystem {
    css_generator: CssGenerator,
    current_theme: ThemeVariant,
    config: TokenSystemConfig,
}

impl DesignTokenSystem {
    pub fn new() -> Self { /* ... */ }
    pub fn get_token(&mut self, path: &str) -> Result<TokenValue, TokenValidationError> { /* ... */ }
    pub fn generate_css_variables(&mut self) -> Result<String, String> { /* ... */ }
    // ... 其他高级API
}
```

## 使用方式

### 基本使用

```rust
use ant_design_dioxus::theme::{
    DesignTokenSystem, ThemeVariant, TokenValue
};

// 创建令牌系统
let mut system = DesignTokenSystem::new();

// 获取令牌值
let primary_color = system.get_token("color.primary.500")?;

// 设置自定义令牌
system.set_token("custom.brand.color", TokenValue::String("#1890ff".to_string()))?;

// 切换主题
system.switch_theme(ThemeVariant::Dark);

// 生成CSS
let css = system.generate_css_variables()?;
```

### 全局令牌系统

```rust
use ant_design_dioxus::theme::{
    init_global_token_system, get_global_token_system, TokenSystemConfig
};

// 初始化全局系统
let config = TokenSystemConfig {
    css_prefix: "my-app".to_string(),
    enable_cache: true,
    minify_css: true,
    strict_mode: false,
};
init_global_token_system(config);

// 使用全局系统
let system = get_global_token_system();
let token_value = system.get_token("color.primary.500")?;
```

### 便捷宏

```rust
use ant_design_dioxus::{token, css_var};

// 获取令牌值
let primary_color = token!("color.primary.500")?;

// 获取CSS变量名
let css_var_name = css_var!("color.primary.500"); // "--ant-color-primary-500"
```

## 优势

### 1. 清晰的职责分离

- 每个模块都有明确的单一职责
- 模块间依赖关系清晰
- 易于理解和维护

### 2. 高度可扩展

- 新功能可以通过实现trait来添加
- 支持自定义令牌存储后端
- 支持自定义CSS生成策略

### 3. 强类型安全

- 使用Rust的类型系统确保安全性
- 编译时错误检查
- 清晰的错误处理

### 4. 高性能

- 智能缓存机制
- 惰性计算
- 最小化重复计算

### 5. 易于测试

- 每个模块可以独立测试
- 清晰的接口便于mock
- 全面的单元测试覆盖

## 迁移指南

### 从旧系统迁移

1. **更新导入**：
   ```rust
   // 旧的导入
   use crate::theme::{DesignTokens, DesignTokenSystem};

   // 新的导入
   use crate::theme::{
       DesignTokenSystem, TokenValue, ThemeVariant
   };
   ```

2. **API变更**：
   ```rust
   // 旧的API
   let color = design_tokens.get_color("primary");

   // 新的API
   let color = system.get_token("color.primary.500")?;
   ```

3. **配置更新**：
   ```rust
   // 新的配置方式
   let config = TokenSystemConfig {
       css_prefix: "ant".to_string(),
       enable_cache: true,
       minify_css: false,
       strict_mode: false,
   };
   let system = DesignTokenSystem::with_config(config);
   ```

## 最佳实践

### 1. 令牌命名规范

使用层次化的命名结构：
```
color.primary.500
color.background.default
spacing.margin.large
typography.font.size.heading1
```

### 2. 主题管理

- 为每个主题变体提供完整的令牌定义
- 使用引用来保持一致性
- 避免硬编码值

### 3. 性能优化

- 启用缓存以提高性能
- 批量操作而非单个操作
- 在生产环境中启用CSS压缩

### 4. 错误处理

- 始终处理令牌解析错误
- 提供有意义的错误信息
- 使用验证功能检查令牌完整性

## 总结

通过这次重构，我们实现了：

1. **模块化设计**：清晰的职责分离和模块边界
2. **类型安全**：强类型系统和编译时检查
3. **高性能**：智能缓存和优化的算法
4. **易扩展**：基于trait的设计支持自定义扩展
5. **易测试**：独立的模块便于单元测试

新的设计令牌系统为Ant Design Dioxus提供了强大、灵活、高性能的主题管理能力，同时保持了简洁易用的API。
