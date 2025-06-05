# Theme 系统

Theme 系统是 css-in-rust 的核心组件，提供完整的主题管理功能，包括设计令牌、主题上下文、CSS 变量管理等。

## 架构概述

Theme 系统采用分层架构设计，分为三层：

1. **核心层 (core)**：提供基础的 Token 系统、CSS 生成、缓存机制等核心功能
2. **适配层 (adapter)**：连接核心层和框架层，提供通用的适配接口
3. **系统层 (systems)**：提供特定领域的系统实现，如颜色系统、排版系统等

## 核心层 (core)

核心层包含以下模块：

- **token**：Token 系统的核心实现
  - `definitions.rs`：Token 的基础类型和接口
  - `values.rs`：Token 值的存储和管理
  - `resolver.rs`：Token 解析和引用处理
  - `css_generator.rs`：Token 到 CSS 的转换
  - `system.rs`：Token 系统的基础实现
  - `simple_system.rs`：简化版 Token 系统

- **css**：CSS 生成和管理
  - `generator.rs`：CSS 生成和样式输出
  - `variables.rs`：CSS 变量管理
  - `dependency.rs`：样式依赖追踪

- **cache**：缓存系统
  - `component_cache.rs`：组件级样式缓存

- **optimize**：优化引擎
  - `mod.rs`：样式优化配置和实现

- **manager**：主题管理
  - `mod.rs`：主题管理器
  - `theme_history.rs`：主题历史记录

- **provider**：主题提供者
  - `mod.rs`：主题提供者实现

## 适配层 (adapter)

适配层包含以下模块：

- **provider**：主题提供者适配器
  - `mod.rs`：扩展核心主题提供者，添加高级功能

- **injection**：样式注入
  - `mod.rs`：样式注入器，负责将生成的CSS注入到不同平台

- **ssr**：服务端渲染支持
  - `mod.rs`：SSR 支持
  - `hydration.rs`：客户端水合

- **frameworks**：框架适配器
  - `dioxus`：Dioxus 框架适配器
  - `react`：React 框架适配器

## 系统层 (systems)

系统层包含以下模块：

- **color**：颜色系统
  - `mod.rs`：颜色系统实现

- **typography**：排版系统
  - `mod.rs`：排版系统实现

- **spacing**：间距系统
  - `mod.rs`：间距系统实现

- **semantic**：语义系统
  - `mod.rs`：语义系统实现

## 使用示例

### 基础使用

```rust
use css_in_rust::theme::{Theme, ThemeMode, ThemeManager};

// 创建主题管理器
let manager = ThemeManager::default();

// 创建主题
let mut theme = Theme::default();
theme.name = "my-theme".to_string();
theme.mode = ThemeMode::Dark;

// 设置主题
manager.set_theme(theme).unwrap();

// 获取当前主题
let current_theme = manager.get_current_theme().unwrap();
```

### 使用框架适配器

```rust
use css_in_rust::theme::{
    adapter::{ThemeProviderAdapter, DioxusAdapter},
    Theme, ThemeMode,
};

// 创建主题提供者适配器
let provider = ThemeProviderAdapter::new();

// 创建 Dioxus 适配器
let dioxus_adapter = DioxusAdapter::new(provider);

// 生成组件样式
let style = dioxus_adapter.style_component("button", "color: blue;");
```

### 使用 SSR 支持

```rust
use css_in_rust::theme::adapter::ssr::SsrSupport;
use std::collections::HashMap;

// 创建 SSR 支持
let ssr = SsrSupport::new();

// 创建样式
let mut styles = HashMap::new();
styles.insert("button".to_string(), ".button { color: blue; }".to_string());
styles.insert("input".to_string(), ".input { border: 1px solid gray; }".to_string());

// 渲染样式
let result = ssr.render_styles(styles);

// 生成样式标签
let tag = ssr.generate_style_tag(&result);
```

## 性能优化

Theme 系统提供了多种性能优化机制：

1. **组件级样式缓存**：使用 `ComponentStyleCache` 缓存组件样式
2. **样式依赖追踪**：使用 `DependencyTracker` 追踪样式依赖关系
3. **样式优化**：使用 `StyleOptimizer` 优化生成的 CSS
4. **SSR 支持**：使用 `SsrSupport` 和 `HydrationEngine` 优化服务端渲染

## 扩展

Theme 系统设计为易于扩展，可以通过以下方式进行扩展：

1. 实现自定义 Token 系统
2. 添加新的框架适配器
3. 扩展现有系统（如颜色系统、排版系统等）
4. 添加新的优化策略
