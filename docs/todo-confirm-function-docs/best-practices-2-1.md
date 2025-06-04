# CSS-in-Rust 最佳实践指南（二-1）：主题设计与响应式开发

本指南介绍 CSS-in-Rust 项目中主题设计、响应式开发和组件设计的最佳实践。

## 🎨 主题设计最佳实践

### 1. 主题系统架构

#### 主题结构设计

```rust
use css_in_rust::{theme, css};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 主题配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ColorPalette,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub shadows: ShadowSystem,
    pub borders: BorderSystem,
    pub animations: AnimationConfig,
    pub breakpoints: BreakpointConfig,
}

/// 颜色调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // 主色调
    pub primary: ColorScale,
    pub secondary: ColorScale,
    pub accent: ColorScale,

    // 语义色彩
    pub success: ColorScale,
    pub warning: ColorScale,
    pub error: ColorScale,
    pub info: ColorScale,

    // 中性色
    pub gray: ColorScale,

    // 背景和文本
    pub background: BackgroundColors,
    pub text: TextColors,

    // 边框和分割线
    pub border: BorderColors,
}

/// 颜色等级系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScale {
    pub _50: String,   // 最浅
    pub _100: String,
    pub _200: String,
    pub _300: String,
    pub _400: String,
    pub _500: String,  // 基础色
    pub _600: String,
    pub _700: String,
    pub _800: String,
    pub _900: String,  // 最深
    pub _950: String,
}

/// 背景颜色系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub surface: String,
    pub overlay: String,
}

/// 文本颜色系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub disabled: String,
    pub inverse: String,
}

/// 边框颜色系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderColors {
    pub default: String,
    pub muted: String,
    pub strong: String,
    pub focus: String,
}
```

#### 字体系统设计

```rust
/// 字体系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_families: FontFamilies,
    pub font_sizes: FontSizes,
    pub font_weights: FontWeights,
    pub line_heights: LineHeights,
    pub letter_spacings: LetterSpacings,
}

/// 字体族配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamilies {
    pub sans: String,
    pub serif: String,
    pub mono: String,
    pub display: String,
}

/// 字体大小系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSizes {
    pub xs: String,    // 0.75rem
    pub sm: String,    // 0.875rem
    pub base: String,  // 1rem
    pub lg: String,    // 1.125rem
    pub xl: String,    // 1.25rem
    pub _2xl: String,  // 1.5rem
    pub _3xl: String,  // 1.875rem
    pub _4xl: String,  // 2.25rem
    pub _5xl: String,  // 3rem
    pub _6xl: String,  // 3.75rem
    pub _7xl: String,  // 4.5rem
    pub _8xl: String,  // 6rem
    pub _9xl: String,  // 8rem
}

/// 字体粗细系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeights {
    pub thin: u16,        // 100
    pub extralight: u16,  // 200
    pub light: u16,       // 300
    pub normal: u16,      // 400
    pub medium: u16,      // 500
    pub semibold: u16,    // 600
    pub bold: u16,        // 700
    pub extrabold: u16,   // 800
    pub black: u16,       // 900
}

/// 行高系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineHeights {
    pub none: String,     // 1
    pub tight: String,    // 1.25
    pub snug: String,     // 1.375
    pub normal: String,   // 1.5
    pub relaxed: String,  // 1.625
    pub loose: String,    // 2
}

/// 字母间距系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterSpacings {
    pub tighter: String,  // -0.05em
    pub tight: String,    // -0.025em
    pub normal: String,   // 0em
    pub wide: String,     // 0.025em
    pub wider: String,    // 0.05em
    pub widest: String,   // 0.1em
}
```

#### 间距系统设计

```rust
/// 间距比例系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingScale {
    pub _0: String,    // 0
    pub px: String,    // 1px
    pub _0_5: String,  // 0.125rem
    pub _1: String,    // 0.25rem
    pub _1_5: String,  // 0.375rem
    pub _2: String,    // 0.5rem
    pub _2_5: String,  // 0.625rem
    pub _3: String,    // 0.75rem
    pub _3_5: String,  // 0.875rem
    pub _4: String,    // 1rem
    pub _5: String,    // 1.25rem
    pub _6: String,    // 1.5rem
    pub _7: String,    // 1.75rem
    pub _8: String,    // 2rem
    pub _9: String,    // 2.25rem
    pub _10: String,   // 2.5rem
    pub _11: String,   // 2.75rem
    pub _12: String,   // 3rem
    pub _14: String,   // 3.5rem
    pub _16: String,   // 4rem
    pub _20: String,   // 5rem
    pub _24: String,   // 6rem
    pub _28: String,   // 7rem
    pub _32: String,   // 8rem
    pub _36: String,   // 9rem
    pub _40: String,   // 10rem
    pub _44: String,   // 11rem
    pub _48: String,   // 12rem
    pub _52: String,   // 13rem
    pub _56: String,   // 14rem
    pub _60: String,   // 15rem
    pub _64: String,   // 16rem
    pub _72: String,   // 18rem
    pub _80: String,   // 20rem
    pub _96: String,   // 24rem
}
```

### 2. 主题实现模式

#### 主题提供者模式

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/// 主题提供者
pub struct ThemeProvider {
    themes: HashMap<String, ThemeConfig>,
    current_theme: Arc<RwLock<String>>,
    fallback_theme: String,
}

impl ThemeProvider {
    /// 创建新的主题提供者
    pub fn new(default_theme: &str) -> Self {
        let mut provider = Self {
            themes: HashMap::new(),
            current_theme: Arc::new(RwLock::new(default_theme.to_string())),
            fallback_theme: default_theme.to_string(),
        };

        // 注册默认主题
        provider.register_theme("light", create_light_theme());
        provider.register_theme("dark", create_dark_theme());

        provider
    }

    /// 注册主题
    pub fn register_theme(&mut self, name: &str, theme: ThemeConfig) {
        self.themes.insert(name.to_string(), theme);
    }

    /// 切换主题
    pub fn switch_theme(&self, theme_name: &str) -> Result<(), ThemeError> {
        if !self.themes.contains_key(theme_name) {
            return Err(ThemeError::ThemeNotFound(theme_name.to_string()));
        }

        if let Ok(mut current) = self.current_theme.write() {
            *current = theme_name.to_string();
            Ok(())
        } else {
            Err(ThemeError::LockError)
        }
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> Option<ThemeConfig> {
        let current_name = self.current_theme.read().ok()?.clone();
        self.themes.get(&current_name).cloned()
    }

    /// 获取主题变量
    pub fn get_theme_variable(&self, path: &str) -> Option<String> {
        let theme = self.get_current_theme()?;
        self.resolve_theme_path(&theme, path)
    }

    /// 解析主题路径
    fn resolve_theme_path(&self, theme: &ThemeConfig, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();

        match parts.as_slice() {
            ["colors", "primary", scale] => {
                self.get_color_scale_value(&theme.colors.primary, scale)
            },
            ["colors", "secondary", scale] => {
                self.get_color_scale_value(&theme.colors.secondary, scale)
            },
            ["typography", "font_sizes", size] => {
                self.get_font_size_value(&theme.typography.font_sizes, size)
            },
            ["spacing", value] => {
                self.get_spacing_value(&theme.spacing, value)
            },
            _ => None,
        }
    }

    /// 获取颜色等级值
    fn get_color_scale_value(&self, scale: &ColorScale, level: &str) -> Option<String> {
        match level {
            "50" => Some(scale._50.clone()),
            "100" => Some(scale._100.clone()),
            "200" => Some(scale._200.clone()),
            "300" => Some(scale._300.clone()),
            "400" => Some(scale._400.clone()),
            "500" => Some(scale._500.clone()),
            "600" => Some(scale._600.clone()),
            "700" => Some(scale._700.clone()),
            "800" => Some(scale._800.clone()),
            "900" => Some(scale._900.clone()),
            "950" => Some(scale._950.clone()),
            _ => None,
        }
    }

    /// 获取字体大小值
    fn get_font_size_value(&self, sizes: &FontSizes, size: &str) -> Option<String> {
        match size {
            "xs" => Some(sizes.xs.clone()),
            "sm" => Some(sizes.sm.clone()),
            "base" => Some(sizes.base.clone()),
            "lg" => Some(sizes.lg.clone()),
            "xl" => Some(sizes.xl.clone()),
            "2xl" => Some(sizes._2xl.clone()),
            "3xl" => Some(sizes._3xl.clone()),
            "4xl" => Some(sizes._4xl.clone()),
            "5xl" => Some(sizes._5xl.clone()),
            "6xl" => Some(sizes._6xl.clone()),
            "7xl" => Some(sizes._7xl.clone()),
            "8xl" => Some(sizes._8xl.clone()),
            "9xl" => Some(sizes._9xl.clone()),
            _ => None,
        }
    }

    /// 获取间距值
    fn get_spacing_value(&self, spacing: &SpacingScale, value: &str) -> Option<String> {
        match value {
            "0" => Some(spacing._0.clone()),
            "px" => Some(spacing.px.clone()),
            "0.5" => Some(spacing._0_5.clone()),
            "1" => Some(spacing._1.clone()),
            "1.5" => Some(spacing._1_5.clone()),
            "2" => Some(spacing._2.clone()),
            "2.5" => Some(spacing._2_5.clone()),
            "3" => Some(spacing._3.clone()),
            "3.5" => Some(spacing._3_5.clone()),
            "4" => Some(spacing._4.clone()),
            "5" => Some(spacing._5.clone()),
            "6" => Some(spacing._6.clone()),
            "7" => Some(spacing._7.clone()),
            "8" => Some(spacing._8.clone()),
            "9" => Some(spacing._9.clone()),
            "10" => Some(spacing._10.clone()),
            "11" => Some(spacing._11.clone()),
            "12" => Some(spacing._12.clone()),
            "14" => Some(spacing._14.clone()),
            "16" => Some(spacing._16.clone()),
            "20" => Some(spacing._20.clone()),
            "24" => Some(spacing._24.clone()),
            "28" => Some(spacing._28.clone()),
            "32" => Some(spacing._32.clone()),
            "36" => Some(spacing._36.clone()),
            "40" => Some(spacing._40.clone()),
            "44" => Some(spacing._44.clone()),
            "48" => Some(spacing._48.clone()),
            "52" => Some(spacing._52.clone()),
            "56" => Some(spacing._56.clone()),
            "60" => Some(spacing._60.clone()),
            "64" => Some(spacing._64.clone()),
            "72" => Some(spacing._72.clone()),
            "80" => Some(spacing._80.clone()),
            "96" => Some(spacing._96.clone()),
            _ => None,
        }
    }
}

/// 主题错误类型
#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("主题 '{0}' 未找到")]
    ThemeNotFound(String),

    #[error("锁定错误")]
    LockError,

    #[error("主题变量路径 '{0}' 无效")]
    InvalidPath(String),
}
```

#### 主题宏系统

```rust
/// 主题变量宏
#[macro_export]
macro_rules! theme_var {
    ($path:expr) => {
        {
            use $crate::theme::THEME_PROVIDER;
            THEME_PROVIDER.get_theme_variable($path)
                .unwrap_or_else(|| {
                    eprintln!("警告: 主题变量 '{}' 未找到，使用默认值", $path);
                    "inherit".to_string()
                })
        }
    };

    ($path:expr, $default:expr) => {
        {
            use $crate::theme::THEME_PROVIDER;
            THEME_PROVIDER.get_theme_variable($path)
                .unwrap_or_else(|| $default.to_string())
        }
    };
}

/// 主题样式宏
#[macro_export]
macro_rules! themed_css {
    ($($css:tt)*) => {
        {
            use $crate::{css, theme_var};

            // 解析 CSS 中的主题变量
            let css_content = stringify!($($css)*);
            let processed_css = process_theme_variables(css_content);

            css! { $processed_css }
        }
    };
}

/// 处理主题变量的函数
fn process_theme_variables(css: &str) -> String {
    use regex::Regex;

    let theme_var_regex = Regex::new(r"\$theme\(([^)]+)\)").unwrap();

    theme_var_regex.replace_all(css, |caps: &regex::Captures| {
        let path = &caps[1];
        theme_var!(path)
    }).to_string()
}
```

### 3. 预定义主题

#### 浅色主题

```rust
/// 创建浅色主题
pub fn create_light_theme() -> ThemeConfig {
    ThemeConfig {
        name: "light".to_string(),
        colors: ColorPalette {
            primary: ColorScale {
                _50: "#eff6ff".to_string(),
                _100: "#dbeafe".to_string(),
                _200: "#bfdbfe".to_string(),
                _300: "#93c5fd".to_string(),
                _400: "#60a5fa".to_string(),
                _500: "#3b82f6".to_string(),
                _600: "#2563eb".to_string(),
                _700: "#1d4ed8".to_string(),
                _800: "#1e40af".to_string(),
                _900: "#1e3a8a".to_string(),
                _950: "#172554".to_string(),
            },
            secondary: ColorScale {
                _50: "#f8fafc".to_string(),
                _100: "#f1f5f9".to_string(),
                _200: "#e2e8f0".to_string(),
                _300: "#cbd5e1".to_string(),
                _400: "#94a3b8".to_string(),
                _500: "#64748b".to_string(),
                _600: "#475569".to_string(),
                _700: "#334155".to_string(),
                _800: "#1e293b".to_string(),
                _900: "#0f172a".to_string(),
                _950: "#020617".to_string(),
            },
            accent: ColorScale {
                _50: "#fdf4ff".to_string(),
                _100: "#fae8ff".to_string(),
                _200: "#f5d0fe".to_string(),
                _300: "#f0abfc".to_string(),
                _400: "#e879f9".to_string(),
                _500: "#d946ef".to_string(),
                _600: "#c026d3".to_string(),
                _700: "#a21caf".to_string(),
                _800: "#86198f".to_string(),
                _900: "#701a75".to_string(),
                _950: "#4a044e".to_string(),
            },
            success: ColorScale {
                _50: "#f0fdf4".to_string(),
                _100: "#dcfce7".to_string(),
                _200: "#bbf7d0".to_string(),
                _300: "#86efac".to_string(),
                _400: "#4ade80".to_string(),
                _500: "#22c55e".to_string(),
                _600: "#16a34a".to_string(),
                _700: "#15803d".to_string(),
                _800: "#166534".to_string(),
                _900: "#14532d".to_string(),
                _950: "#052e16".to_string(),
            },
            warning: ColorScale {
                _50: "#fffbeb".to_string(),
                _100: "#fef3c7".to_string(),
                _200: "#fde68a".to_string(),
                _300: "#fcd34d".to_string(),
                _400: "#fbbf24".to_string(),
                _500: "#f59e0b".to_string(),
                _600: "#d97706".to_string(),
                _700: "#b45309".to_string(),
                _800: "#92400e".to_string(),
                _900: "#78350f".to_string(),
                _950: "#451a03".to_string(),
            },
            error: ColorScale {
                _50: "#fef2f2".to_string(),
                _100: "#fee2e2".to_string(),
                _200: "#fecaca".to_string(),
                _300: "#fca5a5".to_string(),
                _400: "#f87171".to_string(),
                _500: "#ef4444".to_string(),
                _600: "#dc2626".to_string(),
                _700: "#b91c1c".to_string(),
                _800: "#991b1b".to_string(),
                _900: "#7f1d1d".to_string(),
                _950: "#450a0a".to_string(),
            },
            info: ColorScale {
                _50: "#ecfeff".to_string(),
                _100: "#cffafe".to_string(),
                _200: "#a5f3fc".to_string(),
                _300: "#67e8f9".to_string(),
                _400: "#22d3ee".to_string(),
                _500: "#06b6d4".to_string(),
                _600: "#0891b2".to_string(),
                _700: "#0e7490".to_string(),
                _800: "#155e75".to_string(),
                _900: "#164e63".to_string(),
                _950: "#083344".to_string(),
            },
            gray: ColorScale {
                _50: "#f9fafb".to_string(),
                _100: "#f3f4f6".to_string(),
                _200: "#e5e7eb".to_string(),
                _300: "#d1d5db".to_string(),
                _400: "#9ca3af".to_string(),
                _500: "#6b7280".to_string(),
                _600: "#4b5563".to_string(),
                _700: "#374151".to_string(),
                _800: "#1f2937".to_string(),
                _900: "#111827".to_string(),
                _950: "#030712".to_string(),
            },
            background: BackgroundColors {
                primary: "#ffffff".to_string(),
                secondary: "#f9fafb".to_string(),
                tertiary: "#f3f4f6".to_string(),
                surface: "#ffffff".to_string(),
                overlay: "rgba(0, 0, 0, 0.5)".to_string(),
            },
            text: TextColors {
                primary: "#111827".to_string(),
                secondary: "#6b7280".to_string(),
                tertiary: "#9ca3af".to_string(),
                disabled: "#d1d5db".to_string(),
                inverse: "#ffffff".to_string(),
            },
            border: BorderColors {
                default: "#e5e7eb".to_string(),
                muted: "#f3f4f6".to_string(),
                strong: "#d1d5db".to_string(),
                focus: "#3b82f6".to_string(),
            },
        },
        typography: Typography {
            font_families: FontFamilies {
                sans: "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'".to_string(),
                serif: "ui-serif, Georgia, Cambria, 'Times New Roman', Times, serif".to_string(),
                mono: "ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace".to_string(),
                display: "'Inter', ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
            },
            font_sizes: FontSizes {
                xs: "0.75rem".to_string(),
                sm: "0.875rem".to_string(),
                base: "1rem".to_string(),
                lg: "1.125rem".to_string(),
                xl: "1.25rem".to_string(),
                _2xl: "1.5rem".to_string(),
                _3xl: "1.875rem".to_string(),
                _4xl: "2.25rem".to_string(),
                _5xl: "3rem".to_string(),
                _6xl: "3.75rem".to_string(),
                _7xl: "4.5rem".to_string(),
                _8xl: "6rem".to_string(),
                _9xl: "8rem".to_string(),
            },
            font_weights: FontWeights {
                thin: 100,
                extralight: 200,
                light: 300,
                normal: 400,
                medium: 500,
                semibold: 600,
                bold: 700,
                extrabold: 800,
                black: 900,
            },
            line_heights: LineHeights {
                none: "1".to_string(),
                tight: "1.25".to_string(),
                snug: "1.375".to_string(),
                normal: "1.5".to_string(),
                relaxed: "1.625".to_string(),
                loose: "2".to_string(),
            },
            letter_spacings: LetterSpacings {
                tighter: "-0.05em".to_string(),
                tight: "-0.025em".to_string(),
                normal: "0em".to_string(),
                wide: "0.025em".to_string(),
                wider: "0.05em".to_string(),
                widest: "0.1em".to_string(),
            },
        },
        spacing: SpacingScale {
            _0: "0".to_string(),
            px: "1px".to_string(),
            _0_5: "0.125rem".to_string(),
            _1: "0.25rem".to_string(),
            _1_5: "0.375rem".to_string(),
            _2: "0.5rem".to_string(),
            _2_5: "0.625rem".to_string(),
            _3: "0.75rem".to_string(),
            _3_5: "0.875rem".to_string(),
            _4: "1rem".to_string(),
            _5: "1.25rem".to_string(),
            _6: "1.5rem".to_string(),
            _7: "1.75rem".to_string(),
            _8: "2rem".to_string(),
            _9: "2.25rem".to_string(),
            _10: "2.5rem".to_string(),
            _11: "2.75rem".to_string(),
            _12: "3rem".to_string(),
            _14: "3.5rem".to_string(),
            _16: "4rem".to_string(),
            _20: "5rem".to_string(),
            _24: "6rem".to_string(),
            _28: "7rem".to_string(),
            _32: "8rem".to_string(),
            _36: "9rem".to_string(),
            _40: "10rem".to_string(),
            _44: "11rem".to_string(),
            _48: "12rem".to_string(),
            _52: "13rem".to_string(),
            _56: "14rem".to_string(),
            _60: "15rem".to_string(),
            _64: "16rem".to_string(),
            _72: "18rem".to_string(),
            _80: "20rem".to_string(),
            _96: "24rem".to_string(),
        },
        shadows: ShadowSystem {
            sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            base: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            _2xl: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            inner: "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
            none: "none".to_string(),
        },
        borders: BorderSystem {
            widths: BorderWidths {
                none: "0".to_string(),
                thin: "1px".to_string(),
                base: "1px".to_string(),
                thick: "2px".to_string(),
                _2: "2px".to_string(),
                _4: "4px".to_string(),
                _8: "8px".to_string(),
            },
            radius: BorderRadius {
                none: "0".to_string(),
                sm: "0.125rem".to_string(),
                base: "0.25rem".to_string(),
                md: "0.375rem".to_string(),
                lg: "0.5rem".to_string(),
                xl: "0.75rem".to_string(),
                _2xl: "1rem".to_string(),
                _3xl: "1.5rem".to_string(),
                full: "9999px".to_string(),
            },
        },
        animations: AnimationConfig {
            durations: AnimationDurations {
                fast: "150ms".to_string(),
                normal: "300ms".to_string(),
                slow: "500ms".to_string(),
                slower: "750ms".to_string(),
            },
            easings: AnimationEasings {
                linear: "linear".to_string(),
                ease: "ease".to_string(),
                ease_in: "ease-in".to_string(),
                ease_out: "ease-out".to_string(),
                ease_in_out: "ease-in-out".to_string(),
                bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            },
        },
        breakpoints: BreakpointConfig {
            xs: "475px".to_string(),
            sm: "640px".to_string(),
            md: "768px".to_string(),
            lg: "1024px".to_string(),
            xl: "1280px".to_string(),
            _2xl: "1536px".to_string(),
        },
    }
}
```

## 📱 响应式设计最佳实践

### 1. 断点系统设计

#### 断点配置

```rust
/// 断点配置系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointConfig {
    pub xs: String,   // 475px - 超小屏幕
    pub sm: String,   // 640px - 小屏幕
    pub md: String,   // 768px - 中等屏幕
    pub lg: String,   // 1024px - 大屏幕
    pub xl: String,   // 1280px - 超大屏幕
    pub _2xl: String, // 1536px - 2倍超大屏幕
}

/// 响应式工具管理器
pub struct ResponsiveManager {
    breakpoints: BreakpointConfig,
    current_breakpoint: Arc<RwLock<String>>,
}

impl ResponsiveManager {
    /// 创建响应式管理器
    pub fn new(breakpoints: BreakpointConfig) -> Self {
        Self {
            breakpoints,
            current_breakpoint: Arc::new(RwLock::new("md".to_string())),
        }
    }

    /// 获取断点媒体查询
    pub fn get_media_query(&self, breakpoint: &str) -> Option<String> {
        let width = match breakpoint {
            "xs" => &self.breakpoints.xs,
            "sm" => &self.breakpoints.sm,
            "md" => &self.breakpoints.md,
            "lg" => &self.breakpoints.lg,
            "xl" => &self.breakpoints.xl,
            "2xl" => &self.breakpoints._2xl,
            _ => return None,
        };

        Some(format!("@media (min-width: {})", width))
    }

    /// 获取断点范围媒体查询
    pub fn get_range_media_query(&self, min: &str, max: &str) -> Option<String> {
        let min_width = self.get_breakpoint_value(min)?;
        let max_width = self.get_breakpoint_value(max)?;

        Some(format!(
            "@media (min-width: {}) and (max-width: {})",
            min_width,
            max_width
        ))
    }

    /// 获取断点值
    fn get_breakpoint_value(&self, breakpoint: &str) -> Option<&String> {
        match breakpoint {
            "xs" => Some(&self.breakpoints.xs),
            "sm" => Some(&self.breakpoints.sm),
            "md" => Some(&self.breakpoints.md),
            "lg" => Some(&self.breakpoints.lg),
            "xl" => Some(&self.breakpoints.xl),
            "2xl" => Some(&self.breakpoints._2xl),
            _ => None,
        }
    }
}
```

#### 响应式宏系统

```rust
/// 响应式样式宏
#[macro_export]
macro_rules! responsive_css {
    (
        $(
            $breakpoint:ident: {
                $($css:tt)*
            }
        ),* $(,)?
    ) => {
        {
            use $crate::{css, responsive::RESPONSIVE_MANAGER};

            let mut styles = String::new();

            $(
                if let Some(media_query) = RESPONSIVE_MANAGER.get_media_query(stringify!($breakpoint)) {
                    styles.push_str(&format!(
                        "{} {{ {} }}",
                        media_query,
                        stringify!($($css)*)
                    ));
                }
            )*

            css! { $styles }
        }
    };
}

/// 移动优先响应式宏
#[macro_export]
macro_rules! mobile_first_css {
    (
        base: { $($base_css:tt)* },
        $(
            $breakpoint:ident: {
                $($css:tt)*
            }
        ),* $(,)?
    ) => {
        {
            use $crate::{css, responsive::RESPONSIVE_MANAGER};

            let mut styles = String::new();

            // 基础样式（移动端）
            styles.push_str(stringify!($($base_css)*));

            // 响应式样式
            $(
                if let Some(media_query) = RESPONSIVE_MANAGER.get_media_query(stringify!($breakpoint)) {
                    styles.push_str(&format!(
                        " {} {{ {} }}",
                        media_query,
                        stringify!($($css)*)
                    ));
                }
            )*

            css! { $styles }
        }
    };
}
```

### 2. 响应式组件模式

#### 响应式容器组件

```rust
use css_in_rust::{css, mobile_first_css, theme_var};

/// 响应式容器样式
pub fn responsive_container_styles() -> String {
    mobile_first_css! {
        base: {
            .container {
                width: 100%;
                margin-left: auto;
                margin-right: auto;
                padding-left: $theme(spacing.4);
                padding-right: $theme(spacing.4);
            }
        },
        sm: {
            .container {
                max-width: 640px;
                padding-left: $theme(spacing.6);
                padding-right: $theme(spacing.6);
            }
        },
        md: {
            .container {
                max-width: 768px;
            }
        },
        lg: {
            .container {
                max-width: 1024px;
                padding-left: $theme(spacing.8);
                padding-right: $theme(spacing.8);
            }
        },
        xl: {
            .container {
                max-width: 1280px;
            }
        },
        _2xl: {
            .container {
                max-width: 1536px;
            }
        },
    }
}

/// 响应式网格系统
pub fn responsive_grid_styles() -> String {
    mobile_first_css! {
        base: {
            .grid {
                display: grid;
                gap: $theme(spacing.4);
                grid-template-columns: 1fr;
            }

            .grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
            .grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
            .grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
            .grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
            .grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
            .grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
        },
        sm: {
            .sm\:grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
            .sm\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
            .sm\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
            .sm\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
            .sm\:grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
            .sm\:grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
        },
        md: {
            .md\:grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
            .md\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
            .md\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
            .md\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
            .md\:grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
            .md\:grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
        },
        lg: {
            .lg\:grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
            .lg\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
            .lg\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
            .lg\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
            .lg\:grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
            .lg\:grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
        },
        xl: {
            .xl\:grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
            .xl\:grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
            .xl\:grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
            .xl\:grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
            .xl\:grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
            .xl\:grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }
        },
    }
}
```

#### 响应式字体系统

```rust
/// 响应式字体样式
pub fn responsive_typography_styles() -> String {
    mobile_first_css! {
        base: {
            .text-xs { font-size: $theme(typography.font_sizes.xs); }
            .text-sm { font-size: $theme(typography.font_sizes.sm); }
            .text-base { font-size: $theme(typography.font_sizes.base); }
            .text-lg { font-size: $theme(typography.font_sizes.lg); }
            .text-xl { font-size: $theme(typography.font_sizes.xl); }
            .text-2xl { font-size: $theme(typography.font_sizes._2xl); }
            .text-3xl { font-size: $theme(typography.font_sizes._3xl); }
            .text-4xl { font-size: $theme(typography.font_sizes._4xl); }
            .text-5xl { font-size: $theme(typography.font_sizes._5xl); }
            .text-6xl { font-size: $theme(typography.font_sizes._6xl); }

            // 响应式标题
            .heading-1 {
                font-size: $theme(typography.font_sizes._2xl);
                font-weight: $theme(typography.font_weights.bold);
                line-height: $theme(typography.line_heights.tight);
            }

            .heading-2 {
                font-size: $theme(typography.font_sizes.xl);
                font-weight: $theme(typography.font_weights.semibold);
                line-height: $theme(typography.line_heights.tight);
            }

            .heading-3 {
                font-size: $theme(typography.font_sizes.lg);
                font-weight: $theme(typography.font_weights.semibold);
                line-height: $theme(typography.line_heights.snug);
            }
        },
        sm: {
            .heading-1 {
                font-size: $theme(typography.font_sizes._3xl);
            }

            .heading-2 {
                font-size: $theme(typography.font_sizes._2xl);
            }

            .heading-3 {
                font-size: $theme(typography.font_sizes.xl);
            }
        },
        md: {
            .heading-1 {
                font-size: $theme(typography.font_sizes._4xl);
            }

            .heading-2 {
                font-size: $theme(typography.font_sizes._3xl);
            }

            .heading-3 {
                font-size: $theme(typography.font_sizes._2xl);
            }
        },
        lg: {
            .heading-1 {
                font-size: $theme(typography.font_sizes._5xl);
            }

            .heading-2 {
                font-size: $theme(typography.font_sizes._4xl);
            }

            .heading-3 {
                font-size: $theme(typography.font_sizes._3xl);
            }
        },
        xl: {
            .heading-1 {
                font-size: $theme(typography.font_sizes._6xl);
            }

            .heading-2 {
                font-size: $theme(typography.font_sizes._5xl);
            }

            .heading-3 {
                font-size: $theme(typography.font_sizes._4xl);
            }
        },
    }
}
```

### 3. 响应式工具类

#### 显示和隐藏工具

```rust
/// 响应式显示工具
pub fn responsive_display_utilities() -> String {
    mobile_first_css! {
        base: {
            .block { display: block; }
            .inline-block { display: inline-block; }
            .inline { display: inline; }
            .flex { display: flex; }
            .inline-flex { display: inline-flex; }
            .grid { display: grid; }
            .inline-grid { display: inline-grid; }
            .hidden { display: none; }

            // 移动端专用
            .mobile-only { display: block; }
            .desktop-only { display: none; }
        },
        sm: {
            .sm\:block { display: block; }
            .sm\:inline-block { display: inline-block; }
            .sm\:inline { display: inline; }
            .sm\:flex { display: flex; }
            .sm\:inline-flex { display: inline-flex; }
            .sm\:grid { display: grid; }
            .sm\:inline-grid { display: inline-grid; }
            .sm\:hidden { display: none; }
        },
        md: {
            .md\:block { display: block; }
            .md\:inline-block { display: inline-block; }
            .md\:inline { display: inline; }
            .md\:flex { display: flex; }
            .md\:inline-flex { display: inline-flex; }
            .md\:grid { display: grid; }
            .md\:inline-grid { display: inline-grid; }
            .md\:hidden { display: none; }

            // 桌面端显示
            .mobile-only { display: none; }
            .desktop-only { display: block; }
        },
        lg: {
            .lg\:block { display: block; }
            .lg\:inline-block { display: inline-block; }
            .lg\:inline { display: inline; }
            .lg\:flex { display: flex; }
            .lg\:inline-flex { display: inline-flex; }
            .lg\:grid { display: grid; }
            .lg\:inline-grid { display: inline-grid; }
            .lg\:hidden { display: none; }
        },
        xl: {
            .xl\:block { display: block; }
            .xl\:inline-block { display: inline-block; }
            .xl\:inline { display: inline; }
            .xl\:flex { display: flex; }
            .xl\:inline-flex { display: inline-flex; }
            .xl\:grid { display: grid; }
            .xl\:inline-grid { display: inline-grid; }
            .xl\:hidden { display: none; }
        },
    }
}
```

#### 响应式间距工具

```rust
/// 响应式间距工具
pub fn responsive_spacing_utilities() -> String {
    mobile_first_css! {
        base: {
            // 基础间距
            .p-2 { padding: $theme(spacing.2); }
            .p-4 { padding: $theme(spacing.4); }
            .p-6 { padding: $theme(spacing.6); }
            .p-8 { padding: $theme(spacing.8); }

            .m-2 { margin: $theme(spacing.2); }
            .m-4 { margin: $theme(spacing.4); }
            .m-6 { margin: $theme(spacing.6); }
            .m-8 { margin: $theme(spacing.8); }

            // 方向性间距
            .px-4 { padding-left: $theme(spacing.4); padding-right: $theme(spacing.4); }
            .py-4 { padding-top: $theme(spacing.4); padding-bottom: $theme(spacing.4); }
            .mx-4 { margin-left: $theme(spacing.4); margin-right: $theme(spacing.4); }
            .my-4 { margin-top: $theme(spacing.4); margin-bottom: $theme(spacing.4); }
        },
        sm: {
            .sm\:p-2 { padding: $theme(spacing.2); }
            .sm\:p-4 { padding: $theme(spacing.4); }
            .sm\:p-6 { padding: $theme(spacing.6); }
            .sm\:p-8 { padding: $theme(spacing.8); }
            .sm\:p-12 { padding: $theme(spacing.12); }

            .sm\:m-2 { margin: $theme(spacing.2); }
            .sm\:m-4 { margin: $theme(spacing.4); }
            .sm\:m-6 { margin: $theme(spacing.6); }
            .sm\:m-8 { margin: $theme(spacing.8); }
            .sm\:m-12 { margin: $theme(spacing.12); }

            .sm\:px-6 { padding-left: $theme(spacing.6); padding-right: $theme(spacing.6); }
            .sm\:py-6 { padding-top: $theme(spacing.6); padding-bottom: $theme(spacing.6); }
            .sm\:mx-6 { margin-left: $theme(spacing.6); margin-right: $theme(spacing.6); }
            .sm\:my-6 { margin-top: $theme(spacing.6); margin-bottom: $theme(spacing.6); }
        },
        md: {
            .md\:p-8 { padding: $theme(spacing.8); }
            .md\:p-12 { padding: $theme(spacing.12); }
            .md\:p-16 { padding: $theme(spacing.16); }

            .md\:m-8 { margin: $theme(spacing.8); }
            .md\:m-12 { margin: $theme(spacing.12); }
            .md\:m-16 { margin: $theme(spacing.16); }

            .md\:px-8 { padding-left: $theme(spacing.8); padding-right: $theme(spacing.8); }
            .md\:py-8 { padding-top: $theme(spacing.8); padding-bottom: $theme(spacing.8); }
            .md\:mx-8 { margin-left: $theme(spacing.8); margin-right: $theme(spacing.8); }
            .md\:my-8 { margin-top: $theme(spacing.8); margin-bottom: $theme(spacing.8); }
        },
        lg: {
            .lg\:p-12 { padding: $theme(spacing.12); }
            .lg\:p-16 { padding: $theme(spacing.16); }
            .lg\:p-20 { padding: $theme(spacing.20); }

            .lg\:m-12 { margin: $theme(spacing.12); }
            .lg\:m-16 { margin: $theme(spacing.16); }
            .lg\:m-20 { margin: $theme(spacing.20); }

            .lg\:px-12 { padding-left: $theme(spacing.12); padding-right: $theme(spacing.12); }
            .lg\:py-12 { padding-top: $theme(spacing.12); padding-bottom: $theme(spacing.12); }
            .lg\:mx-12 { margin-left: $theme(spacing.12); margin-right: $theme(spacing.12); }
            .lg\:my-12 { margin-top: $theme(spacing.12); margin-bottom: $theme(spacing.12); }
        },
        xl: {
            .xl\:p-16 { padding: $theme(spacing.16); }
            .xl\:p-20 { padding: $theme(spacing.20); }
            .xl\:p-24 { padding: $theme(spacing.24); }

            .xl\:m-16 { margin: $theme(spacing.16); }
            .xl\:m-20 { margin: $theme(spacing.20); }
            .xl\:m-24 { margin: $theme(spacing.24); }

            .xl\:px-16 { padding-left: $theme(spacing.16); padding-right: $theme(spacing.16); }
            .xl\:py-16 { padding-top: $theme(spacing.16); padding-bottom: $theme(spacing.16); }
            .xl\:mx-16 { margin-left: $theme(spacing.16); margin-right: $theme(spacing.16); }
            .xl\:my-16 { margin-top: $theme(spacing.16); margin-bottom: $theme(spacing.16); }
        },
    }
}
```

通过这些主题设计和响应式开发的最佳实践，您可以构建出既美观又适配各种设备的现代化 Web 应用！🎨📱
