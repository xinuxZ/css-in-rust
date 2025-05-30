# 阶段一：基础集成任务清单

## 概览

**阶段目标**：建立 `stylers + lightningcss` 基础集成环境
**时间安排**：第1-3周
**主要交付**：可工作的 CSS-in-Rust 原型

## 第1周：环境搭建

### 1.1 项目初始化

#### 任务 1.1.1：Fork stylers 项目
- [ ] **Fork stylers 仓库**
  ```bash
  git clone https://github.com/your-username/stylers.git
  cd stylers
  git remote add upstream https://github.com/original/stylers.git
  ```

- [ ] **创建开发分支**
  ```bash
  git checkout -b feature/lightningcss-integration
  git push -u origin feature/lightningcss-integration
  ```

- [ ] **设置项目结构**
  ```
  css-in-rust/
  ├── Cargo.toml
  ├── README.md
  ├── LICENSE
  ├── src/
  │   ├── lib.rs
  │   ├── core/
  │   │   ├── mod.rs
  │   │   ├── parser.rs
  │   │   └── optimizer.rs
  │   ├── macro/
  │   │   ├── mod.rs
  │   │   └── css_macro.rs
  │   ├── runtime/
  │   │   ├── mod.rs
  │   │   ├── provider.rs
  │   │   └── injector.rs
  │   └── adapters/
  │       ├── mod.rs
  │       └── dioxus.rs
  ├── tests/
  │   ├── integration/
  │   └── unit/
  ├── benches/
  ├── examples/
  └── docs/
  ```

#### 任务 1.1.2：配置 Cargo.toml
- [ ] **基础依赖配置**
  ```toml
  [package]
  name = "css-in-rust"
  version = "0.1.0"
  edition = "2021"
  authors = ["Your Name <your.email@example.com>"]
  description = "High-performance CSS-in-Rust solution"
  license = "MIT OR Apache-2.0"
  repository = "https://github.com/your-username/css-in-rust"

  [dependencies]
  lightningcss = "1.0"
  proc-macro2 = "1.0"
  quote = "1.0"
  syn = { version = "2.0", features = ["full"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  thiserror = "1.0"

  [dev-dependencies]
  criterion = "0.5"
  tokio-test = "0.4"

  [features]
  default = ["dioxus"]
  dioxus = ["dioxus-core"]

  [lib]
  proc-macro = true
  ```

#### 任务 1.1.3：CI/CD 配置
- [ ] **GitHub Actions 配置**
  ```yaml
  # .github/workflows/ci.yml
  name: CI

  on:
    push:
      branches: [ main, develop ]
    pull_request:
      branches: [ main, develop ]

  env:
    CARGO_TERM_COLOR: always

  jobs:
    test:
      runs-on: ubuntu-latest
      steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      - name: Format check
        run: cargo fmt --all -- --check
      - name: Clippy check
        run: cargo clippy --all-targets --all-features -- -D warnings
      - name: Run tests
        run: cargo test --all-features
      - name: Run benchmarks
        run: cargo bench
  ```

### 1.2 lightningcss 集成

#### 任务 1.2.1：依赖集成
- [ ] **添加 lightningcss 依赖**
  ```rust
  // src/core/parser.rs
  use lightningcss::{
      stylesheet::{StyleSheet, ParserOptions},
      targets::Browsers,
      printer::PrinterOptions,
  };

  pub struct CssParser {
      options: ParserOptions,
      targets: Option<Browsers>,
  }

  impl CssParser {
      pub fn new() -> Self {
          Self {
              options: ParserOptions::default(),
              targets: Some(Browsers::default()),
          }
      }

      pub fn parse(&self, css: &str) -> Result<StyleSheet, ParseError> {
          StyleSheet::parse(css, self.options.clone())
              .map_err(ParseError::from)
      }
  }
  ```

#### 任务 1.2.2：错误处理
- [ ] **定义错误类型**
  ```rust
  // src/core/mod.rs
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum CssError {
      #[error("CSS parsing failed: {0}")]
      ParseError(String),

      #[error("Style injection failed: {0}")]
      InjectionError(String),

      #[error("Theme error: {0}")]
      ThemeError(String),

      #[error("IO error: {0}")]
      IoError(#[from] std::io::Error),
  }

  pub type Result<T> = std::result::Result<T, CssError>;
  ```

### 1.3 基础测试框架

#### 任务 1.3.1：单元测试
- [ ] **解析器测试**
  ```rust
  // tests/unit/parser_tests.rs
  use css_in_rust::core::CssParser;

  #[test]
  fn test_basic_css_parsing() {
      let parser = CssParser::new();
      let css = ".button { background: red; }";

      let result = parser.parse(css);
      assert!(result.is_ok());
  }

  #[test]
  fn test_invalid_css_parsing() {
      let parser = CssParser::new();
      let css = ".button { background: ; }";

      let result = parser.parse(css);
      assert!(result.is_err());
  }
  ```

#### 任务 1.3.2：性能基准测试
- [ ] **基准测试配置**
  ```rust
  // benches/parsing_benchmark.rs
  use criterion::{black_box, criterion_group, criterion_main, Criterion};
  use css_in_rust::core::CssParser;

  fn parse_benchmark(c: &mut Criterion) {
      let parser = CssParser::new();
      let css = include_str!("../test_data/large.css");

      c.bench_function("parse large css", |b| {
          b.iter(|| parser.parse(black_box(css)))
      });
  }

  criterion_group!(benches, parse_benchmark);
  criterion_main!(benches);
  ```

## 第2周：核心集成

### 2.1 css! 宏实现

#### 任务 2.1.1：宏定义
- [ ] **基础宏结构**
  ```rust
  // src/macro/css_macro.rs
  use proc_macro::TokenStream;
  use quote::quote;
  use syn::{parse_macro_input, LitStr};

  #[proc_macro]
  pub fn css(input: TokenStream) -> TokenStream {
      let css_str = parse_macro_input!(input as LitStr);
      let css_content = css_str.value();

      // 解析和验证 CSS
      match parse_and_validate_css(&css_content) {
          Ok(processed_css) => {
              let class_name = generate_class_name(&css_content);

              quote! {
                  {
                      static CSS: &str = #processed_css;
                      static CLASS: &str = #class_name;

                      css_in_rust::runtime::inject_style(CSS, CLASS);
                      CLASS
                  }
              }.into()
          }
          Err(e) => {
              let error_msg = format!("CSS parsing error: {}", e);
              quote! {
                  compile_error!(#error_msg);
              }.into()
          }
      }
  }
  ```

#### 任务 2.1.2：CSS 处理逻辑
- [ ] **CSS 解析和优化**
  ```rust
  // src/macro/css_macro.rs
  use crate::core::{CssParser, CssOptimizer};
  use sha2::{Sha256, Digest};

  fn parse_and_validate_css(css: &str) -> Result<String, String> {
      let parser = CssParser::new();
      let optimizer = CssOptimizer::new();

      // 解析 CSS
      let stylesheet = parser.parse(css)
          .map_err(|e| format!("Parse error: {}", e))?;

      // 优化 CSS
      let optimized = optimizer.optimize(stylesheet)
          .map_err(|e| format!("Optimization error: {}", e))?;

      Ok(optimized)
  }

  fn generate_class_name(css: &str) -> String {
      let mut hasher = Sha256::new();
      hasher.update(css.as_bytes());
      let hash = hasher.finalize();
      format!("css-{:x}", &hash[..4].iter().fold(0u32, |acc, &b| acc << 8 | b as u32))
  }
  ```

### 2.2 运行时系统

#### 任务 2.2.1：样式注入器
- [ ] **Web 平台实现**
  ```rust
  // src/runtime/injector.rs
  use std::collections::HashMap;
  use std::sync::{Arc, Mutex};

  pub struct StyleInjector {
      injected_styles: Arc<Mutex<HashMap<String, bool>>>,
  }

  impl StyleInjector {
      pub fn new() -> Self {
          Self {
              injected_styles: Arc::new(Mutex::new(HashMap::new())),
          }
      }

      pub fn inject_style(&self, css: &str, class_name: &str) -> Result<(), CssError> {
          let mut styles = self.injected_styles.lock().unwrap();

          if styles.contains_key(class_name) {
              return Ok(()); // 已经注入过了
          }

          #[cfg(target_arch = "wasm32")]
          {
              self.inject_web_style(css, class_name)?;
          }

          #[cfg(not(target_arch = "wasm32"))]
          {
              // SSR 或其他平台的处理
              self.inject_server_style(css, class_name)?;
          }

          styles.insert(class_name.to_string(), true);
          Ok(())
      }

      #[cfg(target_arch = "wasm32")]
      fn inject_web_style(&self, css: &str, class_name: &str) -> Result<(), CssError> {
          use web_sys::{window, Document, HtmlStyleElement};

          let window = window().ok_or_else(|| CssError::InjectionError("No window".to_string()))?;
          let document = window.document().ok_or_else(|| CssError::InjectionError("No document".to_string()))?;

          let style_element = document.create_element("style")
              .map_err(|_| CssError::InjectionError("Failed to create style element".to_string()))?
              .dyn_into::<HtmlStyleElement>()
              .map_err(|_| CssError::InjectionError("Failed to cast to HtmlStyleElement".to_string()))?;

          style_element.set_text_content(Some(css));
          style_element.set_attribute("data-css-class", class_name)
              .map_err(|_| CssError::InjectionError("Failed to set attribute".to_string()))?;

          let head = document.head().ok_or_else(|| CssError::InjectionError("No head element".to_string()))?;
          head.append_child(&style_element)
              .map_err(|_| CssError::InjectionError("Failed to append style element".to_string()))?;

          Ok(())
      }
  }
  ```

#### 任务 2.2.2：全局样式管理
- [ ] **样式管理器**
  ```rust
  // src/runtime/provider.rs
  use std::sync::OnceLock;
  use crate::runtime::StyleInjector;

  static STYLE_INJECTOR: OnceLock<StyleInjector> = OnceLock::new();

  pub fn inject_style(css: &str, class_name: &str) -> &'static str {
      let injector = STYLE_INJECTOR.get_or_init(|| StyleInjector::new());

      if let Err(e) = injector.inject_style(css, class_name) {
          eprintln!("Failed to inject style: {}", e);
      }

      class_name
  }

  pub trait StyleProvider {
      fn inject_styles(&self, styles: &str) -> Result<(), CssError>;
      fn remove_styles(&self, id: &str) -> Result<(), CssError>;
      fn clear_all_styles(&self) -> Result<(), CssError>;
  }
  ```

## 第3周：基础测试

### 3.1 集成测试

#### 任务 3.1.1：端到端测试
- [ ] **宏集成测试**
  ```rust
  // tests/integration/macro_tests.rs
  use css_in_rust::css;

  #[test]
  fn test_basic_css_macro() {
      let class_name = css! {
          ".button {
              background: #007bff;
              color: white;
              padding: 8px 16px;
              border: none;
              border-radius: 4px;
              cursor: pointer;
          }"
      };

      assert!(class_name.starts_with("css-"));
      assert_eq!(class_name.len(), 12); // "css-" + 8位哈希
  }

  #[test]
  fn test_css_with_variables() {
      let class_name = css! {
          ".card {
              background: var(--bg-color, white);
              border: 1px solid var(--border-color, #ddd);
              border-radius: var(--border-radius, 4px);
          }"
      };

      assert!(class_name.starts_with("css-"));
  }
  ```

#### 任务 3.1.2：性能测试
- [ ] **编译时性能测试**
  ```rust
  // tests/integration/performance_tests.rs
  use std::time::Instant;
  use css_in_rust::core::CssParser;

  #[test]
  fn test_parsing_performance() {
      let parser = CssParser::new();
      let large_css = include_str!("../test_data/bootstrap.css");

      let start = Instant::now();
      let result = parser.parse(large_css);
      let duration = start.elapsed();

      assert!(result.is_ok());
      assert!(duration.as_millis() < 100, "Parsing took too long: {:?}", duration);
  }

  #[test]
  fn test_memory_usage() {
      // 内存使用测试
      let parser = CssParser::new();
      let css = ".test { color: red; }";

      for _ in 0..1000 {
          let _ = parser.parse(css);
      }

      // 这里可以添加内存使用检查
  }
  ```

### 3.2 文档和示例

#### 任务 3.2.1：基础文档
- [ ] **README.md**
  ```markdown
  # CSS-in-Rust

  High-performance CSS-in-Rust solution based on stylers + lightningcss.

  ## Features

  - 🚀 High-performance CSS parsing and optimization
  - 🔒 Type-safe CSS with compile-time validation
  - 🎨 Theme system with CSS variables
  - 📱 Responsive design support
  - 🔧 Framework adapters (Dioxus, Yew, Leptos)

  ## Quick Start

  ```rust
  use css_in_rust::css;

  fn Button() -> Element {
      let button_class = css! {
          ".button {
              background: #007bff;
              color: white;
              padding: 8px 16px;
              border: none;
              border-radius: 4px;
              cursor: pointer;
          }

          .button:hover {
              background: #0056b3;
          }"
      };

      rsx! {
          button { class: "{button_class}", "Click me" }
      }
  }
  ```
  ```

#### 任务 3.2.2：示例项目
- [ ] **基础示例**
  ```rust
  // examples/basic_usage.rs
  use css_in_rust::css;

  fn main() {
      let button_style = css! {
          ".btn {
              display: inline-block;
              padding: 6px 12px;
              margin-bottom: 0;
              font-size: 14px;
              font-weight: normal;
              line-height: 1.42857143;
              text-align: center;
              white-space: nowrap;
              vertical-align: middle;
              cursor: pointer;
              border: 1px solid transparent;
              border-radius: 4px;
          }"
      };

      println!("Generated class: {}", button_style);
  }
  ```

## 验收标准

### 功能验收
- [ ] `css!` 宏能够正确解析基础 CSS
- [ ] 样式能够正确注入到 DOM
- [ ] 生成的类名具有唯一性
- [ ] 错误的 CSS 能够在编译时被捕获

### 性能验收
- [ ] CSS 解析时间 < 100ms (单个文件)
- [ ] 宏展开时间 < 50ms (单个宏调用)
- [ ] 内存使用无明显泄漏

### 质量验收
- [ ] 单元测试覆盖率 > 80%
- [ ] 所有 clippy 检查通过
- [ ] 代码格式化检查通过
- [ ] 基础文档完整

## 下一步计划

完成阶段一后，将进入阶段二：核心功能实现，重点开发：
1. 主题系统
2. 变体系统
3. 样式优化引擎
4. 热更新支持

---

**负责人**：AI 架构师
**创建时间**：2024年
**预计完成时间**：第3周末



-================================ TODO =======================================-

### 部分实现但需要改进的任务 依赖配置差异
- 规划要求 ：使用 lightningcss 作为核心 CSS 处理引擎
- 当前实现 ：项目没有使用 lightningcss ，而是采用了自定义的 CSS 处理逻辑
- 影响 ：功能基本实现，但性能可能不如 lightningcss 优化 测试覆盖
- 规划要求 ：单元测试覆盖率 > 80%，性能基准测试
- 当前实现 ：有集成测试，但缺少详细的单元测试和性能基准测试
- 影响 ：基础功能验证充分，但测试覆盖度有待提升
### ❌ 未完成的任务 性能基准测试
- 缺失 ： benches/ 目录存在但为空，没有实现 Criterion 基准测试
- 影响 ：无法验证性能指标（CSS 解析时间 < 100ms，宏展开时间 < 50ms） Framework 适配器
- 缺失 ： adapters/ 目录存在但内容不完整，缺少 Dioxus、Yew、Leptos 等框架的具体适配器实现
- 影响 ：框架集成功能不完整
### 🎯 验收标准达成情况
