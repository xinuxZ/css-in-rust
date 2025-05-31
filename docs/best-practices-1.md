# CSS-in-Rust 最佳实践指南（一）：样式组织与性能优化

本指南介绍 CSS-in-Rust 项目中样式组织和性能优化的最佳实践，帮助您构建高效、可维护的样式系统。

## 📁 样式组织最佳实践

### 1. 项目结构组织

#### 推荐的目录结构

```
src/
├── styles/
│   ├── base/           # 基础样式
│   │   ├── reset.rs    # CSS 重置
│   │   ├── typography.rs # 字体样式
│   │   └── variables.rs  # CSS 变量
│   ├── components/     # 组件样式
│   │   ├── button.rs   # 按钮组件
│   │   ├── card.rs     # 卡片组件
│   │   └── modal.rs    # 模态框组件
│   ├── layouts/        # 布局样式
│   │   ├── grid.rs     # 网格布局
│   │   ├── flex.rs     # 弹性布局
│   │   └── container.rs # 容器布局
│   ├── themes/         # 主题样式
│   │   ├── light.rs    # 浅色主题
│   │   ├── dark.rs     # 深色主题
│   │   └── custom.rs   # 自定义主题
│   ├── utilities/      # 工具类样式
│   │   ├── spacing.rs  # 间距工具
│   │   ├── colors.rs   # 颜色工具
│   │   └── responsive.rs # 响应式工具
│   └── mod.rs          # 样式模块导出
├── components/         # React/Yew 组件
└── lib.rs
```

#### 样式模块组织示例

```rust
// src/styles/mod.rs
/// 基础样式模块
pub mod base {
    pub mod reset;
    pub mod typography;
    pub mod variables;
}

/// 组件样式模块
pub mod components {
    pub mod button;
    pub mod card;
    pub mod modal;
}

/// 布局样式模块
pub mod layouts {
    pub mod grid;
    pub mod flex;
    pub mod container;
}

/// 主题样式模块
pub mod themes {
    pub mod light;
    pub mod dark;
    pub mod custom;
}

/// 工具类样式模块
pub mod utilities {
    pub mod spacing;
    pub mod colors;
    pub mod responsive;
}

// 重新导出常用样式
pub use base::variables::*;
pub use utilities::{spacing::*, colors::*, responsive::*};
```

### 2. 样式命名规范

#### BEM 命名约定

```rust
use css_in_rust::css;

/// 按钮组件样式 - 遵循 BEM 命名规范
pub fn button_styles() -> String {
    css! {
        // Block: button
        .button {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            padding: 0.5rem 1rem;
            border: none;
            border-radius: 0.25rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
        }

        // Element: button__icon
        .button__icon {
            margin-right: 0.5rem;
            width: 1rem;
            height: 1rem;
        }

        // Element: button__text
        .button__text {
            font-size: 0.875rem;
            line-height: 1.25;
        }

        // Modifier: button--primary
        .button--primary {
            background-color: #3b82f6;
            color: white;
        }

        .button--primary:hover {
            background-color: #2563eb;
        }

        // Modifier: button--secondary
        .button--secondary {
            background-color: #6b7280;
            color: white;
        }

        // Modifier: button--large
        .button--large {
            padding: 0.75rem 1.5rem;
            font-size: 1rem;
        }

        // State: button--disabled
        .button--disabled {
            opacity: 0.5;
            cursor: not-allowed;
            pointer-events: none;
        }
    }
}
```

#### 语义化命名

```rust
/// 语义化的样式命名
pub fn semantic_styles() -> String {
    css! {
        // 功能性命名
        .visually-hidden {
            position: absolute;
            width: 1px;
            height: 1px;
            padding: 0;
            margin: -1px;
            overflow: hidden;
            clip: rect(0, 0, 0, 0);
            white-space: nowrap;
            border: 0;
        }

        .sr-only {
            position: absolute;
            left: -10000px;
            width: 1px;
            height: 1px;
            overflow: hidden;
        }

        // 状态命名
        .is-active {
            background-color: #3b82f6;
            color: white;
        }

        .is-loading {
            opacity: 0.6;
            pointer-events: none;
        }

        .is-error {
            border-color: #ef4444;
            background-color: #fef2f2;
        }

        .is-success {
            border-color: #10b981;
            background-color: #f0fdf4;
        }

        // 布局命名
        .layout-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 1rem;
        }

        .layout-sidebar {
            width: 250px;
            flex-shrink: 0;
        }

        .layout-main {
            flex: 1;
            min-width: 0;
        }
    }
}
```

### 3. 样式复用策略

#### 创建可复用的样式函数

```rust
use css_in_rust::{css, CssStyle};

/// 可复用的间距工具函数
pub fn spacing_utilities() -> CssStyle {
    css! {
        // 外边距工具类
        .m-0 { margin: 0; }
        .m-1 { margin: 0.25rem; }
        .m-2 { margin: 0.5rem; }
        .m-3 { margin: 0.75rem; }
        .m-4 { margin: 1rem; }
        .m-5 { margin: 1.25rem; }
        .m-6 { margin: 1.5rem; }
        .m-8 { margin: 2rem; }
        .m-10 { margin: 2.5rem; }
        .m-12 { margin: 3rem; }

        // 内边距工具类
        .p-0 { padding: 0; }
        .p-1 { padding: 0.25rem; }
        .p-2 { padding: 0.5rem; }
        .p-3 { padding: 0.75rem; }
        .p-4 { padding: 1rem; }
        .p-5 { padding: 1.25rem; }
        .p-6 { padding: 1.5rem; }
        .p-8 { padding: 2rem; }
        .p-10 { padding: 2.5rem; }
        .p-12 { padding: 3rem; }

        // 方向性间距
        .mt-auto { margin-top: auto; }
        .mr-auto { margin-right: auto; }
        .mb-auto { margin-bottom: auto; }
        .ml-auto { margin-left: auto; }
        .mx-auto { margin-left: auto; margin-right: auto; }
        .my-auto { margin-top: auto; margin-bottom: auto; }
    }
}

/// 可复用的颜色工具函数
pub fn color_utilities() -> CssStyle {
    css! {
        // 文本颜色
        .text-primary { color: #3b82f6; }
        .text-secondary { color: #6b7280; }
        .text-success { color: #10b981; }
        .text-warning { color: #f59e0b; }
        .text-error { color: #ef4444; }
        .text-white { color: #ffffff; }
        .text-black { color: #000000; }

        // 背景颜色
        .bg-primary { background-color: #3b82f6; }
        .bg-secondary { background-color: #6b7280; }
        .bg-success { background-color: #10b981; }
        .bg-warning { background-color: #f59e0b; }
        .bg-error { background-color: #ef4444; }
        .bg-white { background-color: #ffffff; }
        .bg-gray-50 { background-color: #f9fafb; }
        .bg-gray-100 { background-color: #f3f4f6; }
        .bg-gray-200 { background-color: #e5e7eb; }

        // 边框颜色
        .border-primary { border-color: #3b82f6; }
        .border-secondary { border-color: #6b7280; }
        .border-success { border-color: #10b981; }
        .border-warning { border-color: #f59e0b; }
        .border-error { border-color: #ef4444; }
        .border-gray-200 { border-color: #e5e7eb; }
        .border-gray-300 { border-color: #d1d5db; }
    }
}

/// 可复用的布局工具函数
pub fn layout_utilities() -> CssStyle {
    css! {
        // Flexbox 工具
        .flex { display: flex; }
        .inline-flex { display: inline-flex; }
        .flex-col { flex-direction: column; }
        .flex-row { flex-direction: row; }
        .flex-wrap { flex-wrap: wrap; }
        .flex-nowrap { flex-wrap: nowrap; }

        .items-start { align-items: flex-start; }
        .items-center { align-items: center; }
        .items-end { align-items: flex-end; }
        .items-stretch { align-items: stretch; }

        .justify-start { justify-content: flex-start; }
        .justify-center { justify-content: center; }
        .justify-end { justify-content: flex-end; }
        .justify-between { justify-content: space-between; }
        .justify-around { justify-content: space-around; }
        .justify-evenly { justify-content: space-evenly; }

        .flex-1 { flex: 1 1 0%; }
        .flex-auto { flex: 1 1 auto; }
        .flex-initial { flex: 0 1 auto; }
        .flex-none { flex: none; }

        // Grid 工具
        .grid { display: grid; }
        .grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
        .grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
        .grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
        .grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
        .grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
        .grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }

        .gap-1 { gap: 0.25rem; }
        .gap-2 { gap: 0.5rem; }
        .gap-3 { gap: 0.75rem; }
        .gap-4 { gap: 1rem; }
        .gap-6 { gap: 1.5rem; }
        .gap-8 { gap: 2rem; }

        // 定位工具
        .relative { position: relative; }
        .absolute { position: absolute; }
        .fixed { position: fixed; }
        .sticky { position: sticky; }
        .static { position: static; }

        .top-0 { top: 0; }
        .right-0 { right: 0; }
        .bottom-0 { bottom: 0; }
        .left-0 { left: 0; }

        .inset-0 { top: 0; right: 0; bottom: 0; left: 0; }
    }
}
```

#### 样式组合模式

```rust
/// 样式组合器 - 将多个样式函数组合
pub struct StyleComposer {
    styles: Vec<CssStyle>,
}

impl StyleComposer {
    /// 创建新的样式组合器
    pub fn new() -> Self {
        Self {
            styles: Vec::new(),
        }
    }

    /// 添加样式
    pub fn add(mut self, style: CssStyle) -> Self {
        self.styles.push(style);
        self
    }

    /// 条件性添加样式
    pub fn add_if(self, condition: bool, style: CssStyle) -> Self {
        if condition {
            self.add(style)
        } else {
            self
        }
    }

    /// 组合所有样式
    pub fn compose(self) -> CssStyle {
        let mut combined = String::new();
        for style in self.styles {
            combined.push_str(&style.to_string());
            combined.push('\n');
        }
        CssStyle::from(combined)
    }
}

/// 使用样式组合器的示例
pub fn create_component_styles(is_dark_mode: bool, is_mobile: bool) -> CssStyle {
    StyleComposer::new()
        .add(spacing_utilities())
        .add(color_utilities())
        .add(layout_utilities())
        .add_if(is_dark_mode, dark_theme_styles())
        .add_if(is_mobile, mobile_responsive_styles())
        .compose()
}

/// 深色主题样式
fn dark_theme_styles() -> CssStyle {
    css! {
        .dark .text-primary { color: #60a5fa; }
        .dark .bg-white { background-color: #1f2937; }
        .dark .border-gray-200 { border-color: #374151; }
    }
}

/// 移动端响应式样式
fn mobile_responsive_styles() -> CssStyle {
    css! {
        @media (max-width: 768px) {
            .mobile-hidden { display: none; }
            .mobile-full { width: 100%; }
            .mobile-stack { flex-direction: column; }
        }
    }
}
```

## ⚡ 性能优化最佳实践

### 1. 编译时优化

#### 死代码消除配置

```rust
// build.rs
use css_in_rust::build_tools::{
    CssBuildProcessor, BuildConfig, DeadCodeEliminationConfig
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = BuildConfig {
        // 启用死代码消除
        dead_code_elimination: DeadCodeEliminationConfig {
            enabled: true,
            aggressive_mode: true,
            usage_threshold: 0.1, // 使用率低于 10% 的样式将被移除
            preserve_critical_css: true,
            preserve_patterns: vec![
                r"^\.(critical|important)-.*".to_string(),
                r"^\.(layout|grid|flex)-.*".to_string(),
            ],
            exclude_patterns: vec![
                r"^\.(test|debug)-.*".to_string(),
            ],
        },

        // 启用 CSS 压缩
        minification: true,

        // 启用源码映射（开发环境）
        source_maps: cfg!(debug_assertions),

        // 启用缓存
        enable_caching: true,
        cache_strategy: CacheStrategy::Aggressive,

        ..Default::default()
    };

    let mut processor = CssBuildProcessor::new(config)?;
    let result = processor.run()?;

    println!("构建完成:");
    println!("  - 处理文件: {} 个", result.files_processed);
    println!("  - 原始大小: {} KB", result.original_size / 1024);
    println!("  - 压缩后大小: {} KB", result.compressed_size / 1024);
    println!("  - 压缩率: {:.1}%", result.compression_ratio * 100.0);

    Ok(())
}
```

#### 样式分割策略

```rust
/// 样式分割管理器
pub struct StyleSplitter {
    critical_styles: Vec<CssStyle>,
    component_styles: HashMap<String, CssStyle>,
    utility_styles: CssStyle,
    theme_styles: HashMap<String, CssStyle>,
}

impl StyleSplitter {
    /// 创建新的样式分割器
    pub fn new() -> Self {
        Self {
            critical_styles: Vec::new(),
            component_styles: HashMap::new(),
            utility_styles: CssStyle::empty(),
            theme_styles: HashMap::new(),
        }
    }

    /// 添加关键样式（首屏渲染必需）
    pub fn add_critical(&mut self, style: CssStyle) {
        self.critical_styles.push(style);
    }

    /// 添加组件样式（按需加载）
    pub fn add_component(&mut self, component_name: &str, style: CssStyle) {
        self.component_styles.insert(component_name.to_string(), style);
    }

    /// 设置工具样式
    pub fn set_utilities(&mut self, style: CssStyle) {
        self.utility_styles = style;
    }

    /// 添加主题样式
    pub fn add_theme(&mut self, theme_name: &str, style: CssStyle) {
        self.theme_styles.insert(theme_name.to_string(), style);
    }

    /// 生成关键 CSS（内联到 HTML）
    pub fn generate_critical_css(&self) -> String {
        let mut critical = String::new();

        // 添加基础重置样式
        critical.push_str(&self.generate_reset_css());

        // 添加关键样式
        for style in &self.critical_styles {
            critical.push_str(&style.to_string());
            critical.push('\n');
        }

        // 添加基础工具类
        critical.push_str(&self.generate_essential_utilities());

        critical
    }

    /// 生成组件 CSS 文件
    pub fn generate_component_css(&self, component_name: &str) -> Option<String> {
        self.component_styles.get(component_name).map(|style| style.to_string())
    }

    /// 生成主题 CSS 文件
    pub fn generate_theme_css(&self, theme_name: &str) -> Option<String> {
        self.theme_styles.get(theme_name).map(|style| style.to_string())
    }

    /// 生成基础重置样式
    fn generate_reset_css(&self) -> String {
        css! {
            /* 关键重置样式 */
            *, *::before, *::after {
                box-sizing: border-box;
            }

            html {
                line-height: 1.15;
                -webkit-text-size-adjust: 100%;
            }

            body {
                margin: 0;
                font-family: system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
            }

            main {
                display: block;
            }

            h1 {
                font-size: 2em;
                margin: 0.67em 0;
            }
        }.to_string()
    }

    /// 生成基础工具类
    fn generate_essential_utilities(&self) -> String {
        css! {
            /* 关键工具类 */
            .sr-only {
                position: absolute;
                width: 1px;
                height: 1px;
                padding: 0;
                margin: -1px;
                overflow: hidden;
                clip: rect(0, 0, 0, 0);
                white-space: nowrap;
                border: 0;
            }

            .flex {
                display: flex;
            }

            .items-center {
                align-items: center;
            }

            .justify-center {
                justify-content: center;
            }

            .w-full {
                width: 100%;
            }

            .h-full {
                height: 100%;
            }
        }.to_string()
    }
}
```

### 2. 运行时优化

#### 样式缓存策略

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// 样式缓存管理器
pub struct StyleCacheManager {
    cache: Arc<RwLock<HashMap<String, CachedStyle>>>,
    max_size: usize,
    ttl: Duration,
}

/// 缓存的样式项
#[derive(Clone)]
struct CachedStyle {
    content: String,
    created_at: Instant,
    access_count: u32,
    last_accessed: Instant,
}

impl StyleCacheManager {
    /// 创建新的缓存管理器
    pub fn new(max_size: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// 获取缓存的样式
    pub fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.write().ok()?;

        if let Some(cached) = cache.get_mut(key) {
            // 检查是否过期
            if cached.created_at.elapsed() > self.ttl {
                cache.remove(key);
                return None;
            }

            // 更新访问信息
            cached.access_count += 1;
            cached.last_accessed = Instant::now();

            Some(cached.content.clone())
        } else {
            None
        }
    }

    /// 设置缓存
    pub fn set(&self, key: String, content: String) {
        if let Ok(mut cache) = self.cache.write() {
            // 检查缓存大小限制
            if cache.len() >= self.max_size {
                self.evict_lru(&mut cache);
            }

            let cached_style = CachedStyle {
                content,
                created_at: Instant::now(),
                access_count: 1,
                last_accessed: Instant::now(),
            };

            cache.insert(key, cached_style);
        }
    }

    /// 清理过期缓存
    pub fn cleanup_expired(&self) {
        if let Ok(mut cache) = self.cache.write() {
            let now = Instant::now();
            cache.retain(|_, cached| now.duration_since(cached.created_at) <= self.ttl);
        }
    }

    /// LRU 淘汰策略
    fn evict_lru(&self, cache: &mut HashMap<String, CachedStyle>) {
        if let Some((lru_key, _)) = cache
            .iter()
            .min_by_key(|(_, cached)| (cached.access_count, cached.last_accessed))
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            cache.remove(&lru_key);
        }
    }

    /// 获取缓存统计信息
    pub fn get_stats(&self) -> CacheStats {
        if let Ok(cache) = self.cache.read() {
            let total_entries = cache.len();
            let total_access_count: u32 = cache.values().map(|c| c.access_count).sum();
            let avg_access_count = if total_entries > 0 {
                total_access_count as f64 / total_entries as f64
            } else {
                0.0
            };

            CacheStats {
                total_entries,
                total_access_count,
                avg_access_count,
                hit_ratio: 0.0, // 需要在使用时计算
            }
        } else {
            CacheStats::default()
        }
    }
}

/// 缓存统计信息
#[derive(Debug, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_access_count: u32,
    pub avg_access_count: f64,
    pub hit_ratio: f64,
}
```

#### 懒加载样式系统

```rust
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// 懒加载样式管理器
pub struct LazyStyleLoader {
    loaded_styles: Arc<Mutex<HashSet<String>>>,
    style_registry: HashMap<String, Box<dyn Fn() -> CssStyle + Send + Sync>>,
    cache_manager: StyleCacheManager,
}

impl LazyStyleLoader {
    /// 创建新的懒加载管理器
    pub fn new() -> Self {
        Self {
            loaded_styles: Arc::new(Mutex::new(HashSet::new())),
            style_registry: HashMap::new(),
            cache_manager: StyleCacheManager::new(100, 3600), // 1小时 TTL
        }
    }

    /// 注册样式生成器
    pub fn register<F>(&mut self, name: &str, generator: F)
    where
        F: Fn() -> CssStyle + Send + Sync + 'static,
    {
        self.style_registry.insert(name.to_string(), Box::new(generator));
    }

    /// 懒加载样式
    pub fn load_style(&self, name: &str) -> Option<String> {
        // 检查是否已加载
        {
            let loaded = self.loaded_styles.lock().ok()?;
            if loaded.contains(name) {
                return self.cache_manager.get(name);
            }
        }

        // 检查缓存
        if let Some(cached) = self.cache_manager.get(name) {
            let mut loaded = self.loaded_styles.lock().ok()?;
            loaded.insert(name.to_string());
            return Some(cached);
        }

        // 生成样式
        if let Some(generator) = self.style_registry.get(name) {
            let style = generator();
            let content = style.to_string();

            // 缓存样式
            self.cache_manager.set(name.to_string(), content.clone());

            // 标记为已加载
            {
                let mut loaded = self.loaded_styles.lock().ok()?;
                loaded.insert(name.to_string());
            }

            Some(content)
        } else {
            None
        }
    }

    /// 预加载关键样式
    pub fn preload_critical(&self, style_names: &[&str]) {
        for name in style_names {
            self.load_style(name);
        }
    }

    /// 批量加载样式
    pub fn load_batch(&self, style_names: &[&str]) -> HashMap<String, String> {
        let mut results = HashMap::new();

        for name in style_names {
            if let Some(content) = self.load_style(name) {
                results.insert(name.to_string(), content);
            }
        }

        results
    }

    /// 卸载不需要的样式
    pub fn unload_style(&self, name: &str) {
        if let Ok(mut loaded) = self.loaded_styles.lock() {
            loaded.remove(name);
        }
    }

    /// 获取加载统计
    pub fn get_load_stats(&self) -> LoadStats {
        let loaded_count = self.loaded_styles.lock()
            .map(|loaded| loaded.len())
            .unwrap_or(0);

        let registered_count = self.style_registry.len();
        let cache_stats = self.cache_manager.get_stats();

        LoadStats {
            loaded_count,
            registered_count,
            load_ratio: if registered_count > 0 {
                loaded_count as f64 / registered_count as f64
            } else {
                0.0
            },
            cache_stats,
        }
    }
}

/// 加载统计信息
#[derive(Debug)]
pub struct LoadStats {
    pub loaded_count: usize,
    pub registered_count: usize,
    pub load_ratio: f64,
    pub cache_stats: CacheStats,
}
```

### 3. 构建优化配置

#### 生产环境优化配置

```toml
# css-in-rust.toml
[build]
# 优化级别
optimization_level = "aggressive"

# 死代码消除
[build.dead_code_elimination]
enabled = true
aggressive_mode = true
usage_threshold = 0.05
preserve_critical_css = true
preserve_patterns = [
    "^\\.(critical|important)-.*",
    "^\\.(layout|grid|flex)-.*"
]
exclude_patterns = [
    "^\\.(test|debug)-.*"
]

# CSS 压缩
[build.minification]
enabled = true
remove_comments = true
remove_whitespace = true
merge_duplicate_rules = true
optimize_selectors = true
simplify_colors = true
compress_numbers = true

# 缓存配置
[build.cache]
enabled = true
strategy = "aggressive"
max_size_mb = 500
ttl_hours = 24

# 并行处理
[build.parallel]
enabled = true
max_jobs = 8

# 输出配置
[build.output]
generate_source_maps = false
split_css_files = true
max_file_size_kb = 100
```

#### 开发环境优化配置

```toml
# css-in-rust.dev.toml
[build]
# 开发模式优化
optimization_level = "development"

# 禁用死代码消除（开发时保留所有样式）
[build.dead_code_elimination]
enabled = false

# 最小化压缩
[build.minification]
enabled = false
remove_comments = false
remove_whitespace = false

# 开发缓存
[build.cache]
enabled = true
strategy = "conservative"
max_size_mb = 100
ttl_hours = 1

# 适度并行
[build.parallel]
enabled = true
max_jobs = 4

# 开发输出
[build.output]
generate_source_maps = true
split_css_files = false
verbose_logging = true

# 热更新
[hot_reload]
enabled = true
port = 3001
watch_patterns = [
    "src/**/*.rs",
    "components/**/*.rs"
]
```

## 📊 性能监控与分析

### 性能指标收集

```rust
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// 性能指标收集器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub compilation_time: Duration,
    pub css_generation_time: Duration,
    pub optimization_time: Duration,
    pub cache_hit_ratio: f64,
    pub memory_usage: MemoryUsage,
    pub file_stats: FileStats,
}

/// 内存使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub current_memory_mb: f64,
}

/// 文件统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub total_files: usize,
    pub total_input_size_kb: usize,
    pub total_output_size_kb: usize,
    pub compression_ratio: f64,
}

/// 性能分析器
pub struct PerformanceAnalyzer {
    start_time: Option<Instant>,
    metrics: PerformanceMetrics,
}

impl PerformanceAnalyzer {
    /// 开始性能分析
    pub fn start() -> Self {
        Self {
            start_time: Some(Instant::now()),
            metrics: PerformanceMetrics::default(),
        }
    }

    /// 记录编译时间
    pub fn record_compilation_time(&mut self, duration: Duration) {
        self.metrics.compilation_time = duration;
    }

    /// 记录 CSS 生成时间
    pub fn record_css_generation_time(&mut self, duration: Duration) {
        self.metrics.css_generation_time = duration;
    }

    /// 记录优化时间
    pub fn record_optimization_time(&mut self, duration: Duration) {
        self.metrics.optimization_time = duration;
    }

    /// 生成性能报告
    pub fn generate_report(&self) -> PerformanceReport {
        let total_time = self.start_time
            .map(|start| start.elapsed())
            .unwrap_or_default();

        PerformanceReport {
            total_time,
            metrics: self.metrics.clone(),
            recommendations: self.generate_recommendations(),
        }
    }

    /// 生成优化建议
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        // 缓存命中率建议
        if self.metrics.cache_hit_ratio < 0.7 {
            recommendations.push(
                "缓存命中率较低，建议调整缓存策略或增加缓存大小".to_string()
            );
        }

        // 内存使用建议
        if self.metrics.memory_usage.peak_memory_mb > 512.0 {
            recommendations.push(
                "内存使用量较高，建议启用流式处理或减少并行度".to_string()
            );
        }

        // 压缩率建议
        if self.metrics.file_stats.compression_ratio < 0.3 {
            recommendations.push(
                "CSS 压缩率较低，建议启用更激进的优化选项".to_string()
            );
        }

        recommendations
    }
}

/// 性能报告
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub total_time: Duration,
    pub metrics: PerformanceMetrics,
    pub recommendations: Vec<String>,
}
```

## ✅ 性能优化检查清单

### 编译时优化
- [ ] 启用死代码消除
- [ ] 配置合适的使用率阈值
- [ ] 启用 CSS 压缩和混淆
- [ ] 使用样式分割策略
- [ ] 启用增量编译
- [ ] 配置并行处理

### 运行时优化
- [ ] 实现样式缓存机制
- [ ] 使用懒加载策略
- [ ] 优化样式查找算法
- [ ] 减少样式重复计算
- [ ] 实现样式去重

### 构建优化
- [ ] 配置生产环境优化选项
- [ ] 启用构建缓存
- [ ] 优化文件监控范围
- [ ] 使用适当的并行度
- [ ] 监控构建性能指标

### 代码组织
- [ ] 遵循样式命名规范
- [ ] 创建可复用的样式函数
- [ ] 合理组织目录结构
- [ ] 实现样式组合模式
- [ ] 使用语义化命名

通过遵循这些最佳实践，您可以构建高性能、可维护的 CSS-in-Rust 样式系统！🚀
