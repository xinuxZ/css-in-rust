



我需要深入分析 Ant Design 的 CSS-in-JS 方案特点，并探讨如何在 CSS-in-Rust 中实现对应功能。


toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 1,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 53\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 2,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 54\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 3,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 55\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 4,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 56\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 5,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 57\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 6,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 58\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 7,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": true,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 59\n}"}],"isError":null}




toolName: run_mcp

status: success

mcp: mcp.config.usrlocalmcp.sequential-thinking sequentialthinking [object Object]
 {"content":[{"type":"text","text":"{\n  \"thoughtNumber\": 8,\n  \"totalThoughts\": 8,\n  \"nextThoughtNeeded\": false,\n  \"branches\": [],\n  \"thoughtHistoryLength\": 60\n}"}],"isError":null}

## Ant Design CSS-in-JS 方案特点分析

### 核心特点

#### 1. 运行时样式生成
- 使用 `@ant-design/cssinjs` 库在运行时动态生成 CSS 规则
- 样式通过 JavaScript 对象定义，支持函数式样式计算
- 动态注入到 DOM 中的 `<style>` 标签

#### 2. 主题系统
- 基于 Design Token 的主题架构
- 通过 `ConfigProvider` 提供主题上下文
- 支持动态主题切换和主题变量访问
- 组件可以根据主题变量进行样式计算

#### 3. 样式隔离
- 自动生成 hash 类名实现样式隔离
- 避免全局样式污染
- 每个组件都有独特的类名前缀

#### 4. 动态样式计算
- 支持基于 props、状态和主题的动态样式
- 样式函数可以接收组件属性和主题变量
- 实时响应状态变化

#### 5. 性能优化机制
- **样式缓存**：相同样式只生成一次
- **按需注入**：只注入被使用的样式
- **样式合并**：合并相似的样式规则
- **SSR 支持**：服务端样式预生成

#### 6. 开发体验
- TypeScript 支持，提供类型检查和智能提示
- 良好的调试和开发工具支持

## CSS-in-Rust 对应功能实现方案

### 1. 运行时样式生成

```rust
// 样式宏定义
style! {
    .button {
        background: {theme.primary_color};
        padding: {size.padding}px;
        border-radius: {theme.border_radius}px;
    }
}

// 运行时样式注入
use web_sys::window;
let style_element = document.create_element("style")?;
style_element.set_text_content(Some(&css_string));
```

**实现要点**：
- 使用 Rust 宏系统在编译时生成样式定义代码
- 运行时通过 `web-sys` 操作 DOM，动态创建和注入样式
- 利用 Dioxus 响应式系统自动重新计算样式

### 2. 主题系统

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub primary_color: String,
    pub border_radius: u32,
    pub font_size: FontSize,
    pub spacing: Spacing,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontSize {
    pub small: u32,
    pub medium: u32,
    pub large: u32,
}

// 主题上下文提供
fn App() -> Element {
    let theme = Theme::default();
    rsx! {
        ThemeProvider { theme,
            Button { "Hello World" }
        }
    }
}

// 组件中使用主题
fn Button() -> Element {
    let theme = use_context::<Theme>();
    let styles = use_memo(move || {
        format!(
            "background: {}; border-radius: {}px;",
            theme.primary_color, theme.border_radius
        )
    });

    rsx! {
        button { style: "{styles}", "Click me" }
    }
}
```

**实现要点**：
- 使用 Rust 结构体定义 Design Token，利用类型系统保证一致性
- 通过 Dioxus Context API 提供主题上下文
- 支持主题继承和覆盖机制
- 实现主题切换的平滑过渡

### 3. 样式隔离

```rust
// 自动生成唯一类名
macro_rules! style_hash {
    ($component:expr, $styles:expr) => {
        {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            $component.hash(&mut hasher);
            $styles.hash(&mut hasher);
            let hash = hasher.finish();
            format!("{}__{:x}", $component, hash)
        }
    };
}

// 样式作用域
mod button_styles {
    use super::*;

    pub fn get_styles(theme: &Theme) -> String {
        let class_name = style_hash!("button", "primary-styles");
        format!(
            ".{} {{ background: {}; padding: 8px 16px; }}",
            class_name, theme.primary_color
        )
    }
}
```

**实现要点**：
- 编译时自动生成唯一类名（组件名 + 样式内容哈希）
- 使用 Rust 模块系统实现样式作用域
- 支持 CSS Modules 风格的样式导入
- 可选 Shadow DOM 实现更强隔离

### 4. 动态样式计算

```rust
#[derive(Props, Clone, PartialEq)]
struct ButtonProps {
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
}

#[derive(Clone, PartialEq)]
enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

fn Button(props: ButtonProps) -> Element {
    let theme = use_context::<Theme>();

    // 动态样式计算
    let styles = use_memo(move || {
        let base_color = match props.variant {
            ButtonVariant::Primary => &theme.primary_color,
            ButtonVariant::Secondary => &theme.secondary_color,
            ButtonVariant::Danger => &theme.danger_color,
        };

        let opacity = if props.disabled { "0.6" } else { "1.0" };
        let padding = match props.size {
            ButtonSize::Small => "4px 8px",
            ButtonSize::Medium => "8px 16px",
            ButtonSize::Large => "12px 24px",
        };

        format!(
            "background: {}; opacity: {}; padding: {};",
            base_color, opacity, padding
        )
    });

    rsx! {
        button {
            style: "{styles}",
            disabled: props.disabled,
            "Button"
        }
    }
}
```

**实现要点**：
- 利用 Rust 函数式编程特性，样式作为参数函数
- 使用 `use_memo` 缓存样式计算结果
- 支持条件样式、响应式样式等动态特性
- 类型安全的样式属性定义

### 5. 性能优化

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

// 全局样式缓存
static STYLE_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// 样式缓存管理器
struct StyleManager {
    cache: HashMap<String, String>,
    injected_styles: HashSet<String>,
}

impl StyleManager {
    /// 获取或生成样式
    fn get_or_create_style(&mut self, key: &str, generator: impl Fn() -> String) -> &str {
        self.cache.entry(key.to_string()).or_insert_with(generator)
    }

    /// 增量样式更新
    fn update_styles(&mut self, new_styles: HashMap<String, String>) {
        for (key, style) in new_styles {
            if !self.injected_styles.contains(&key) {
                self.inject_style(&key, &style);
                self.injected_styles.insert(key);
            }
        }
    }

    /// 注入样式到 DOM
    fn inject_style(&self, key: &str, css: &str) {
        // 使用 web-sys 注入样式
    }
}

// 编译时样式优化宏
macro_rules! optimized_styles {
    ($($rule:expr),*) => {
        {
            // 编译时合并和优化样式规则
            let mut merged = String::new();
            $(
                merged.push_str($rule);
            )*
            // 移除重复规则和空白字符
            optimize_css(&merged)
        }
    };
}
```

**实现要点**：
- **编译时优化**：零成本抽象，样式预计算
- **样式缓存**：`lazy_static`/`once_cell` 全局缓存
- **增量更新**：只更新变化的样式规则
- **Tree Shaking**：编译时移除未使用样式
- **样式压缩**：自动优化 CSS 规则

### 6. 开发体验增强

```rust
// 类型安全的样式属性
#[derive(Debug, Clone, PartialEq)]
pub struct StyleProps {
    pub color: Color,
    pub font_size: FontSize,
    pub margin: Spacing,
    pub padding: Spacing,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
    Named(NamedColor),
}

// 样式验证宏
macro_rules! validate_style {
    ($style:expr) => {
        {
            // 编译时样式验证
            compile_error_if_invalid_css!($style);
            $style
        }
    };
}

// 调试支持
#[cfg(debug_assertions)]
fn inject_style_with_source_map(css: &str, source_info: &str) {
    let css_with_map = format!(
        "{}\n/*# sourceURL={} */",
        css, source_info
    );
    // 注入带 source map 的样式
}
```

**实现要点**：
- **类型安全**：Rust 强类型系统的样式属性检查
- **IDE 支持**：rust-analyzer 智能提示和错误检查
- **调试支持**：source map 生成，支持热重载
- **文档生成**：自动生成样式文档

## CSS-in-Rust 相比 CSS-in-JS 的优势

### 1. 编译时优化
- 更多优化在编译时完成，运行时性能更好
- 静态样式可以完全在编译时生成
- 死代码消除更彻底

### 2. 类型安全
- Rust 类型系统提供编译时样式检查
- 防止样式属性拼写错误和类型不匹配
- 更好的重构支持

### 3. 零运行时开销
- 静态样式零运行时计算成本
- 更高效的内存使用
- 更小的包体积

### 4. 更好的 Tree Shaking
- 未使用的样式代码可以被完全移除
- 编译时依赖分析更精确
- 按需加载更高效

### 5. 原生性能
- Rust 的原生性能优势
- 更快的样式计算和处理
- 更低的 CPU 和内存占用

这样的 CSS-in-Rust 实现方案既保持了 CSS-in-JS 的灵活性和动态特性，又充分利用了 Rust 的性能、安全性和编译时优化优势，为 Ant Design Dioxus 提供了一个强大而高效的样式解决方案。
