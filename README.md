# CSS-in-Rust

**高性能的 Rust CSS-in-JS 解决方案，专为 Dioxus 框架设计**

CSS-in-Rust 是一个专注于 Dioxus 框架的样式解决方案，提供类似于 styled-components 和 emotion 的 CSS-in-JS 体验，同时利用 Rust 的性能和类型安全特性。

## 特性

- **强大的 CSS 处理**：基于 lightningcss 引擎实现高性能的 CSS 解析和处理
- **主题系统**：灵活的主题支持，包括动态主题切换和主题变量
- **编译时优化**：支持在编译时处理和优化 CSS，减少运行时开销
- **类型安全**：与 Rust 类型系统无缝集成
- **Dioxus 框架支持**：专为 Dioxus 框架设计的适配器和工具
- **零运行时开销**：通过编译时计算和优化最小化运行时开销

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
css-in-rust = "0.1.0"
```

## 基本用法

```rust
use css_in_rust::css;
use dioxus::prelude::*;

fn App() -> Element {
    let button_style = css!("
        background-color: blue;
            color: white;
            padding: 8px 16px;
            border-radius: 4px;

        &:hover {
            background-color: darkblue;
        }
    ");

    rsx! {
        button {
            class: button_style,
            "Click me"
        }
    }
}
```

## 高级特性

### 主题支持

```rust
use css_in_rust::theme::{DioxusAdapter, ThemeProvider};
use dioxus::prelude::*;

fn App() -> Element {
    // 设置主题提供者
    let theme_provider = use_context::<ThemeProvider>().unwrap();

    let button_style = css!("
        background-color: var(--primary-color);
        color: var(--text-color);
        padding: var(--spacing-md);
    ");

    rsx! {
        button {
            class: button_style,
            "主题化按钮"
        }
    }
}
```

## 版权和许可

本项目采用 MIT 或 Apache-2.0 许可证。

## 开发

### 构建

```bash
cargo build
```

### 测试

```bash
cargo test
```

### 运行示例

```bash
cargo run --example basic
```

## 路线图

### 阶段 1 (当前) ✅
- [x] 基本 CSS 解析与 lightningcss
- [x] `css!` 宏实现
- [x] 样式注入系统
- [x] 平台特定提供者 (Web/Server)
- [x] 基本优化
- [x] 文档和示例

### 阶段 2 (计划中)
- [x] 主题系统与 CSS 变量
- [x] 变体系统 (hover, focus, etc.)
- [x] 样式优化引擎
- [x] 热重载支持
- [·] 高级缓存

### 阶段 3 (计划中)
- [ ] SSR/SSG 支持
- [ ] 关键 CSS 提取

### 阶段 4 (计划中)
- [ ] VS Code 插件
- [ ] CLI 工具
- [ ] 其他框架适配器
- [ ] 性能监控

## 贡献

我们欢迎贡献！请参阅我们的 [贡献指南](CONTRIBUTING.md) 了解更多详细信息。

### 开发设置

1. 克隆仓库：
   ```bash
   git clone https://github.com/xinuxZ/css-in-rust.git
   cd css-in-rust
   ```

2. 安装依赖：
   ```bash
   cargo build
   ```

3. 运行测试：
   ```bash
   cargo test
   ```

4. 运行示例：
   ```bash
   cargo run --example basic
   ```

## 支持

如果您有任何问题或需要帮助，请：
1. 创建一个 [新问题](https://github.com/xinuxZ/css-in-rust/issues/new)

---

**由 CSS-in-Rust 团队制作**
