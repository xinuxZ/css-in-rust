//! 设计令牌值存储模块
//!
//! 本模块负责存储和管理具体的令牌值，支持多主题变体。
//! 职责：令牌值的存储、检索和主题切换

use super::design_tokens::{BorderColors, ColorScale};
use super::token_definitions::{ThemeVariant, TokenMetadata, TokenPath, TokenValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 响应式断点配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Breakpoints {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            xs: "0px".to_string(),
            sm: "576px".to_string(),
            md: "768px".to_string(),
            lg: "992px".to_string(),
            xl: "1200px".to_string(),
            xxl: "1600px".to_string(),
        }
    }
}

impl Breakpoints {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "xs" => Some(self.xs.clone()),
            "sm" => Some(self.sm.clone()),
            "md" => Some(self.md.clone()),
            "lg" => Some(self.lg.clone()),
            "xl" => Some(self.xl.clone()),
            "xxl" => Some(self.xxl.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --breakpoint-xs: {};\n\
             --breakpoint-sm: {};\n\
             --breakpoint-md: {};\n\
             --breakpoint-lg: {};\n\
             --breakpoint-xl: {};\n\
             --breakpoint-xxl: {};\n",
            self.xs, self.sm, self.md, self.lg, self.xl, self.xxl
        )
    }
}

/// 动画缓动函数配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Easing {
    pub linear: String,
    pub ease_in: String,
    pub ease_out: String,
    pub ease_in_out: String,
}

impl Default for Easing {
    fn default() -> Self {
        Self {
            linear: "linear".to_string(),
            ease_in: "cubic-bezier(0.55, 0.055, 0.675, 0.19)".to_string(),
            ease_out: "cubic-bezier(0.215, 0.61, 0.355, 1)".to_string(),
            ease_in_out: "cubic-bezier(0.645, 0.045, 0.355, 1)".to_string(),
        }
    }
}

impl Easing {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "linear" => Some(self.linear.clone()),
            "ease_in" => Some(self.ease_in.clone()),
            "ease_out" => Some(self.ease_out.clone()),
            "ease_in_out" => Some(self.ease_in_out.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --motion-easing-linear: {};\n\
             --motion-easing-ease-in: {};\n\
             --motion-easing-ease-out: {};\n\
             --motion-easing-ease-in-out: {};\n",
            self.linear, self.ease_in, self.ease_out, self.ease_in_out
        )
    }
}

/// 动画持续时间配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub fast: String,
    pub normal: String,
    pub slow: String,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            fast: "0.1s".to_string(),
            normal: "0.2s".to_string(),
            slow: "0.3s".to_string(),
        }
    }
}

impl Duration {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "fast" => Some(self.fast.clone()),
            "normal" => Some(self.normal.clone()),
            "slow" => Some(self.slow.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --motion-duration-fast: {};\n\
             --motion-duration-normal: {};\n\
             --motion-duration-slow: {};\n",
            self.fast, self.normal, self.slow
        )
    }
}

/// 动画配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Motion {
    pub duration: Duration,
    pub easing: Easing,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            duration: Duration::default(),
            easing: Easing::default(),
        }
    }
}

impl Motion {
    pub fn get_value(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();
        match parts.as_slice() {
            ["duration", duration_path] => self.duration.get_value(duration_path),
            ["easing", easing_path] => self.easing.get_value(easing_path),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "{}{}\n",
            self.duration.to_css_variables(),
            self.easing.to_css_variables()
        )
    }
}

/// 阴影配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shadows {
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub inner: String,
}

impl Default for Shadows {
    fn default() -> Self {
        Self {
            sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)".to_string(),
            md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".to_string(),
            lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)"
                .to_string(),
            xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)"
                .to_string(),
            inner: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)".to_string(),
        }
    }
}

impl Shadows {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "sm" => Some(self.sm.clone()),
            "md" => Some(self.md.clone()),
            "lg" => Some(self.lg.clone()),
            "xl" => Some(self.xl.clone()),
            "inner" => Some(self.inner.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --shadow-sm: {};\n\
             --shadow-md: {};\n\
             --shadow-lg: {};\n\
             --shadow-xl: {};\n\
             --shadow-inner: {};\n",
            self.sm, self.md, self.lg, self.xl, self.inner
        )
    }
}

/// 边框圆角配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderRadius {
    pub none: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub full: String,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            none: "0px".to_string(),
            sm: "2px".to_string(),
            md: "6px".to_string(),
            lg: "8px".to_string(),
            xl: "12px".to_string(),
            full: "9999px".to_string(),
        }
    }
}

impl BorderRadius {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "none" => Some(self.none.clone()),
            "sm" => Some(self.sm.clone()),
            "md" => Some(self.md.clone()),
            "lg" => Some(self.lg.clone()),
            "xl" => Some(self.xl.clone()),
            "full" => Some(self.full.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --border-radius-none: {};\n\
             --border-radius-sm: {};\n\
             --border-radius-md: {};\n\
             --border-radius-lg: {};\n\
             --border-radius-xl: {};\n\
             --border-radius-full: {};\n",
            self.none, self.sm, self.md, self.lg, self.xl, self.full
        )
    }
}

/// 边框宽度配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderWidth {
    pub none: String,
    pub thin: String,
    pub medium: String,
    pub thick: String,
}

impl Default for BorderWidth {
    fn default() -> Self {
        Self {
            none: "0px".to_string(),
            thin: "1px".to_string(),
            medium: "2px".to_string(),
            thick: "4px".to_string(),
        }
    }
}

impl BorderWidth {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "none" => Some(self.none.clone()),
            "thin" => Some(self.thin.clone()),
            "medium" => Some(self.medium.clone()),
            "thick" => Some(self.thick.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --border-width-none: {};\n\
             --border-width-thin: {};\n\
             --border-width-medium: {};\n\
             --border-width-thick: {};\n",
            self.none, self.thin, self.medium, self.thick
        )
    }
}

/// 边框配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Borders {
    pub radius: BorderRadius,
    pub width: BorderWidth,
}

impl Default for Borders {
    fn default() -> Self {
        Self {
            radius: BorderRadius::default(),
            width: BorderWidth::default(),
        }
    }
}

impl Borders {
    pub fn get_value(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();
        match parts.as_slice() {
            ["radius", radius_path] => self.radius.get_value(radius_path),
            ["width", width_path] => self.width.get_value(width_path),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "{}{}",
            self.radius.to_css_variables(),
            self.width.to_css_variables()
        )
    }
}

/// 间距配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spacing {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
    pub xxxl: String,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: "4px".to_string(),
            sm: "8px".to_string(),
            md: "16px".to_string(),
            lg: "24px".to_string(),
            xl: "32px".to_string(),
            xxl: "48px".to_string(),
            xxxl: "64px".to_string(),
        }
    }
}

impl Spacing {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "xs" => Some(self.xs.clone()),
            "sm" => Some(self.sm.clone()),
            "md" => Some(self.md.clone()),
            "lg" => Some(self.lg.clone()),
            "xl" => Some(self.xl.clone()),
            "xxl" => Some(self.xxl.clone()),
            "xxxl" => Some(self.xxxl.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --spacing-xs: {};\n\
             --spacing-sm: {};\n\
             --spacing-md: {};\n\
             --spacing-lg: {};\n\
             --spacing-xl: {};\n\
             --spacing-xxl: {};\n\
             --spacing-xxxl: {};\n",
            self.xs, self.sm, self.md, self.lg, self.xl, self.xxl, self.xxxl
        )
    }
}

/// 字体大小配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontSize {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
    pub xxxl: String,
}

impl Default for FontSize {
    fn default() -> Self {
        Self {
            xs: "12px".to_string(),
            sm: "14px".to_string(),
            md: "16px".to_string(),
            lg: "18px".to_string(),
            xl: "20px".to_string(),
            xxl: "24px".to_string(),
            xxxl: "30px".to_string(),
        }
    }
}

/// 字体族配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontFamily {
    pub sans: String,
    pub serif: String,
    pub mono: String,
}

impl Default for FontFamily {
    fn default() -> Self {
        Self {
            sans: "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            serif: "Georgia, Cambria, 'Times New Roman', Times, serif".to_string(),
            mono: "ui-monospace, SFMono-Regular, 'SF Mono', Consolas, 'Liberation Mono', Menlo, monospace".to_string(),
        }
    }
}

/// 排版配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Typography {
    pub font_size: FontSize,
    pub font_family: FontFamily,
    pub font_weight: super::design_tokens::FontWeights,
    pub line_height: super::design_tokens::LineHeights,
    pub letter_spacing: super::design_tokens::LetterSpacing,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_size: FontSize::default(),
            font_family: FontFamily::default(),
            font_weight: super::design_tokens::FontWeights::default(),
            line_height: super::design_tokens::LineHeights::default(),
            letter_spacing: super::design_tokens::LetterSpacing::default(),
        }
    }
}

impl Typography {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            path if path.starts_with("font_family.") => {
                let sub_path = &path[12..];
                match sub_path {
                    "sans" => Some(self.font_family.sans.clone()),
                    "serif" => Some(self.font_family.serif.clone()),
                    "mono" => Some(self.font_family.mono.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("font_size.") => {
                let sub_path = &path[10..];
                match sub_path {
                    "xs" => Some(self.font_size.xs.clone()),
                    "sm" => Some(self.font_size.sm.clone()),
                    "md" => Some(self.font_size.md.clone()),
                    "lg" => Some(self.font_size.lg.clone()),
                    "xl" => Some(self.font_size.xl.clone()),
                    "xxl" => Some(self.font_size.xxl.clone()),
                    "xxxl" => Some(self.font_size.xxxl.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("font_weight.") => {
                let sub_path = &path[12..];
                match sub_path {
                    "light" => Some(self.font_weight.light.clone()),
                    "normal" => Some(self.font_weight.normal.clone()),
                    "medium" => Some(self.font_weight.medium.clone()),
                    "semibold" => Some(self.font_weight.semibold.clone()),
                    "bold" => Some(self.font_weight.bold.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("line_height.") => {
                let sub_path = &path[12..];
                match sub_path {
                    "tight" => Some(self.line_height.tight.clone()),
                    "normal" => Some(self.line_height.normal.clone()),
                    "relaxed" => Some(self.line_height.relaxed.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("letter_spacing.") => {
                let sub_path = &path[15..];
                match sub_path {
                    "tight" => Some(self.letter_spacing.tight.clone()),
                    "normal" => Some(self.letter_spacing.normal.clone()),
                    "wide" => Some(self.letter_spacing.wide.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --font-size-xs: {};\n\
             --font-size-sm: {};\n\
             --font-size-md: {};\n\
             --font-size-lg: {};\n\
             --font-size-xl: {};\n\
             --font-size-xxl: {};\n\
             --font-size-xxxl: {};\n\
             --font-family-sans: {};\n\
             --font-family-serif: {};\n\
             --font-family-mono: {};\n\
             --font-weight-light: {};\n\
             --font-weight-normal: {};\n\
             --font-weight-medium: {};\n\
             --font-weight-semibold: {};\n\
             --font-weight-bold: {};\n\
             --line-height-tight: {};\n\
             --line-height-normal: {};\n\
             --line-height-relaxed: {};\n\
             --letter-spacing-tight: {};\n\
             --letter-spacing-normal: {};\n\
             --letter-spacing-wide: {};\n",
            self.font_size.xs,
            self.font_size.sm,
            self.font_size.md,
            self.font_size.lg,
            self.font_size.xl,
            self.font_size.xxl,
            self.font_size.xxxl,
            self.font_family.sans,
            self.font_family.serif,
            self.font_family.mono,
            self.font_weight.light,
            self.font_weight.normal,
            self.font_weight.medium,
            self.font_weight.semibold,
            self.font_weight.bold,
            self.line_height.tight,
            self.line_height.normal,
            self.line_height.relaxed,
            self.letter_spacing.tight,
            self.letter_spacing.normal,
            self.letter_spacing.wide
        )
    }
}

/// 文本颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
    pub inverse: String,
}

impl Default for TextColors {
    fn default() -> Self {
        Self {
            primary: "rgba(0, 0, 0, 0.88)".to_string(),
            secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            inverse: "rgba(255, 255, 255, 0.88)".to_string(),
        }
    }
}

impl TextColors {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "primary" => Some(self.primary.clone()),
            "secondary" => Some(self.secondary.clone()),
            "disabled" => Some(self.disabled.clone()),
            "inverse" => Some(self.inverse.clone()),
            _ => None,
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-text-primary: {};\n\
             --color-text-secondary: {};\n\
             --color-text-disabled: {};\n\
             --color-text-inverse: {};\n",
            self.primary, self.secondary, self.disabled, self.inverse
        )
    }
}

/// 背景颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub inverse: String,
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            primary: "#ffffff".to_string(),
            secondary: "#fafafa".to_string(),
            tertiary: "#f5f5f5".to_string(),
            inverse: "#141414".to_string(),
        }
    }
}

impl BackgroundColors {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "primary" => Some(self.primary.clone()),
            "secondary" => Some(self.secondary.clone()),
            "tertiary" => Some(self.tertiary.clone()),
            "inverse" => Some(self.inverse.clone()),
            _ => None,
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-background-primary: {};\n\
             --color-background-secondary: {};\n\
             --color-background-tertiary: {};\n\
             --color-background-inverse: {};\n",
            self.primary, self.secondary, self.tertiary, self.inverse
        )
    }
}

/// 颜色配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Colors {
    pub primary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub text: TextColors,
    pub background: BackgroundColors,
    pub border: BorderColors,
    pub blue: ColorScale,
    pub green: ColorScale,
    pub red: ColorScale,
    pub orange: ColorScale,
    pub gray: ColorScale,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: "#1677ff".to_string(),
            success: "#52c41a".to_string(),
            warning: "#faad14".to_string(),
            error: "#ff4d4f".to_string(),
            info: "#1677ff".to_string(),
            text: TextColors::default(),
            background: BackgroundColors::default(),
            border: BorderColors::default(),
            blue: ColorScale::default(),
            green: ColorScale::default(),
            red: ColorScale::default(),
            orange: ColorScale::default(),
            gray: ColorScale::default(),
        }
    }
}

impl Colors {
    /// 根据路径获取颜色值
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "primary" => Some(self.primary.clone()),
            "success" => Some(self.success.clone()),
            "warning" => Some(self.warning.clone()),
            "error" => Some(self.error.clone()),
            "info" => Some(self.info.clone()),
            path if path.starts_with("text.") => {
                let sub_path = &path[5..];
                self.text.get_value(sub_path)
            }
            path if path.starts_with("background.") => {
                let sub_path = &path[11..];
                self.background.get_value(sub_path)
            }
            path if path.starts_with("border.") => {
                let sub_path = &path[7..];
                self.border.get_value(sub_path)
            }
            path if path.starts_with("blue.") => {
                let sub_path = &path[5..];
                self.blue.get_value(sub_path)
            }
            path if path.starts_with("green.") => {
                let sub_path = &path[6..];
                self.green.get_value(sub_path)
            }
            path if path.starts_with("red.") => {
                let sub_path = &path[4..];
                self.red.get_value(sub_path)
            }
            path if path.starts_with("orange.") => {
                let sub_path = &path[7..];
                self.orange.get_value(sub_path)
            }
            path if path.starts_with("gray.") => {
                let sub_path = &path[5..];
                self.gray.get_value(sub_path)
            }
            _ => None,
        }
    }

    /// 生成 CSS 变量
    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-primary: {};\n\
             --color-success: {};\n\
             --color-warning: {};\n\
             --color-error: {};\n\
             --color-info: {};\n\
             {}{}{}{}{}{}{}{}\n",
            self.primary,
            self.success,
            self.warning,
            self.error,
            self.info,
            self.text.to_css_variables(),
            self.background.to_css_variables(),
            self.border.to_css_variables(),
            self.blue.to_css_variables("blue"),
            self.green.to_css_variables("green"),
            self.red.to_css_variables("red"),
            self.orange.to_css_variables("orange"),
            self.gray.to_css_variables("gray")
        )
    }
}

/// 令牌值存储
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenValueStore {
    /// 存储不同主题变体的令牌值
    values: HashMap<(TokenPath, ThemeVariant), TokenValue>,
    /// 令牌元数据
    metadata: HashMap<TokenPath, TokenMetadata>,
    /// 响应式断点配置
    pub breakpoints: Breakpoints,
    /// 动画配置
    pub motion: Motion,
    /// 阴影配置
    pub shadows: Shadows,
    /// 边框配置
    pub borders: Borders,
    /// 间距配置
    pub spacing: Spacing,
    /// 排版配置
    pub typography: Typography,
    /// 颜色配置
    pub colors: Colors,
}

impl Default for TokenValueStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenValueStore {
    /// 创建新的令牌值存储
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            metadata: HashMap::new(),
            breakpoints: Breakpoints::default(),
            motion: Motion::default(),
            shadows: Shadows::default(),
            borders: Borders::default(),
            spacing: Spacing::default(),
            typography: Typography::default(),
            colors: Colors::default(),
        }
    }

    /// 生成 CSS 变量声明
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 生成颜色变量
        css.push_str(&format!("  --color-primary: {};\n", self.colors.primary));
        css.push_str(&format!("  --color-success: {};\n", self.colors.success));
        css.push_str(&format!("  --color-warning: {};\n", self.colors.warning));
        css.push_str(&format!("  --color-error: {};\n", self.colors.error));
        css.push_str(&format!("  --color-info: {};\n", self.colors.info));

        // 生成文本颜色变量
        css.push_str(&format!(
            "  --color-text-primary: {};\n",
            self.colors.text.primary
        ));
        css.push_str(&format!(
            "  --color-text-secondary: {};\n",
            self.colors.text.secondary
        ));
        css.push_str(&format!(
            "  --color-text-disabled: {};\n",
            self.colors.text.disabled
        ));
        css.push_str(&format!(
            "  --color-text-inverse: {};\n",
            self.colors.text.inverse
        ));

        // 生成背景颜色变量
        css.push_str(&format!(
            "  --bg-primary: {};\n",
            self.colors.background.primary
        ));
        css.push_str(&format!(
            "  --bg-secondary: {};\n",
            self.colors.background.secondary
        ));
        css.push_str(&format!(
            "  --bg-tertiary: {};\n",
            self.colors.background.tertiary
        ));
        css.push_str(&format!(
            "  --bg-inverse: {};\n",
            self.colors.background.inverse
        ));

        // 生成字体变量
        css.push_str(&format!(
            "  --font-family-sans: {};\n",
            self.typography.font_family.sans
        ));
        css.push_str(&format!(
            "  --font-family-serif: {};\n",
            self.typography.font_family.serif
        ));
        css.push_str(&format!(
            "  --font-family-mono: {};\n",
            self.typography.font_family.mono
        ));

        // 生成字体大小变量
        css.push_str(&format!(
            "  --font-size-xs: {};\n",
            self.typography.font_size.xs
        ));
        css.push_str(&format!(
            "  --font-size-sm: {};\n",
            self.typography.font_size.sm
        ));
        css.push_str(&format!(
            "  --font-size-md: {};\n",
            self.typography.font_size.md
        ));
        css.push_str(&format!(
            "  --font-size-lg: {};\n",
            self.typography.font_size.lg
        ));
        css.push_str(&format!(
            "  --font-size-xl: {};\n",
            self.typography.font_size.xl
        ));
        css.push_str(&format!(
            "  --font-size-xxl: {};\n",
            self.typography.font_size.xxl
        ));
        css.push_str(&format!(
            "  --font-size-xxxl: {};\n",
            self.typography.font_size.xxxl
        ));

        // 生成间距变量
        css.push_str(&format!("  --spacing-xs: {};\n", self.spacing.xs));
        css.push_str(&format!("  --spacing-sm: {};\n", self.spacing.sm));
        css.push_str(&format!("  --spacing-md: {};\n", self.spacing.md));
        css.push_str(&format!("  --spacing-lg: {};\n", self.spacing.lg));
        css.push_str(&format!("  --spacing-xl: {};\n", self.spacing.xl));
        css.push_str(&format!("  --spacing-xxl: {};\n", self.spacing.xxl));
        css.push_str(&format!("  --spacing-xxxl: {};\n", self.spacing.xxxl));

        // 生成边框变量
        css.push_str(&format!(
            "  --border-width-none: {};\n",
            self.borders.width.none
        ));
        css.push_str(&format!(
            "  --border-width-thin: {};\n",
            self.borders.width.thin
        ));
        css.push_str(&format!(
            "  --border-width-medium: {};\n",
            self.borders.width.medium
        ));
        css.push_str(&format!(
            "  --border-width-thick: {};\n",
            self.borders.width.thick
        ));

        css.push_str(&format!(
            "  --border-radius-none: {};\n",
            self.borders.radius.none
        ));
        css.push_str(&format!(
            "  --border-radius-sm: {};\n",
            self.borders.radius.sm
        ));
        css.push_str(&format!(
            "  --border-radius-md: {};\n",
            self.borders.radius.md
        ));
        css.push_str(&format!(
            "  --border-radius-lg: {};\n",
            self.borders.radius.lg
        ));
        css.push_str(&format!(
            "  --border-radius-xl: {};\n",
            self.borders.radius.xl
        ));
        css.push_str(&format!(
            "  --border-radius-full: {};\n",
            self.borders.radius.full
        ));

        // 生成阴影变量
        css.push_str(&format!("  --shadow-sm: {};\n", self.shadows.sm));
        css.push_str(&format!("  --shadow-md: {};\n", self.shadows.md));
        css.push_str(&format!("  --shadow-lg: {};\n", self.shadows.lg));
        css.push_str(&format!("  --shadow-xl: {};\n", self.shadows.xl));
        css.push_str(&format!("  --shadow-inner: {};\n", self.shadows.inner));

        // 生成动画变量
        css.push_str(&format!(
            "  --motion-easing-linear: {};\n",
            self.motion.easing.linear
        ));
        css.push_str(&format!(
            "  --motion-easing-ease-in: {};\n",
            self.motion.easing.ease_in
        ));
        css.push_str(&format!(
            "  --motion-easing-ease-out: {};\n",
            self.motion.easing.ease_out
        ));
        css.push_str(&format!(
            "  --motion-easing-ease-in-out: {};\n",
            self.motion.easing.ease_in_out
        ));

        css.push_str(&format!(
            "  --motion-duration-fast: {};\n",
            self.motion.duration.fast
        ));
        css.push_str(&format!(
            "  --motion-duration-normal: {};\n",
            self.motion.duration.normal
        ));
        css.push_str(&format!(
            "  --motion-duration-slow: {};\n",
            self.motion.duration.slow
        ));

        css
    }

    /// 创建Ant Design默认主题的令牌存储
    pub fn ant_design_default() -> Self {
        AntDesignTokenValues::create_default_store()
    }

    /// 创建Ant Design暗色主题的令牌存储
    pub fn ant_design_dark() -> Self {
        AntDesignTokenValues::create_dark_store()
    }

    /// 获取令牌值
    pub fn get_value(&self, path: &TokenPath, theme: ThemeVariant) -> Option<&TokenValue> {
        self.values.get(&(path.clone(), theme))
    }

    /// 设置令牌值
    pub fn set_value(&mut self, path: TokenPath, value: TokenValue, theme: ThemeVariant) {
        self.values.insert((path, theme), value);
    }

    /// 获取令牌元数据
    pub fn get_metadata(&self, path: &TokenPath) -> Option<&TokenMetadata> {
        self.metadata.get(path)
    }

    /// 设置令牌元数据
    pub fn set_metadata(&mut self, path: TokenPath, metadata: TokenMetadata) {
        self.metadata.insert(path, metadata);
    }

    /// 列出指定主题的所有令牌路径
    pub fn list_paths(&self, theme: ThemeVariant) -> Vec<TokenPath> {
        self.values
            .keys()
            .filter(|(_, t)| *t == theme)
            .map(|(path, _)| path.clone())
            .collect()
    }

    /// 检查令牌是否存在
    pub fn has_token(&self, path: &TokenPath, theme: ThemeVariant) -> bool {
        self.values.contains_key(&(path.clone(), theme))
    }

    /// 批量设置令牌值
    pub fn set_values_batch(
        &mut self,
        values: HashMap<TokenPath, TokenValue>,
        theme: ThemeVariant,
    ) {
        for (path, value) in values {
            self.values.insert((path, theme), value);
        }
    }

    /// 检查主题是否存在
    pub fn has_theme(&self, theme: ThemeVariant) -> bool {
        self.values.keys().any(|(_, t)| *t == theme)
    }

    /// 获取所有主题变体
    pub fn get_themes(&self) -> Vec<ThemeVariant> {
        let mut themes: Vec<ThemeVariant> = self.values.keys().map(|(_, theme)| *theme).collect();
        themes.sort();
        themes.dedup();
        themes
    }

    /// 获取所有支持的主题变体
    pub fn get_supported_themes(&self) -> Vec<ThemeVariant> {
        let mut themes: Vec<ThemeVariant> = self.values.keys().map(|(_, theme)| *theme).collect();
        themes.sort();
        themes.dedup();
        themes
    }

    /// 清空指定主题的所有令牌
    pub fn clear_theme(&mut self, theme: ThemeVariant) {
        self.values.retain(|(_, t), _| *t != theme);
    }

    /// 复制主题
    pub fn copy_theme(&mut self, from: ThemeVariant, to: ThemeVariant) {
        let source_values: Vec<_> = self
            .values
            .iter()
            .filter(|((_, t), _)| *t == from)
            .map(|((path, _), value)| (path.clone(), value.clone()))
            .collect();

        for (path, value) in source_values {
            self.values.insert((path, to), value);
        }
    }
}

/// Ant Design 默认令牌值
pub struct AntDesignTokenValues;

impl AntDesignTokenValues {
    /// 获取默认的浅色主题令牌值
    pub fn get_light_theme_values() -> HashMap<TokenPath, TokenValue> {
        let mut values = HashMap::new();

        // 基础颜色
        values.insert(
            TokenPath::from_str("color.primary.50"),
            TokenValue::String("#e6f4ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.100"),
            TokenValue::String("#bae0ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.200"),
            TokenValue::String("#91caff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.300"),
            TokenValue::String("#69b1ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.400"),
            TokenValue::String("#4096ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.500"),
            TokenValue::String("#1677ff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.600"),
            TokenValue::String("#0958d9".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.700"),
            TokenValue::String("#003eb3".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.800"),
            TokenValue::String("#002c8c".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.primary.900"),
            TokenValue::String("#001d66".to_string()),
        );

        // 成功色
        values.insert(
            TokenPath::from_str("color.success.500"),
            TokenValue::String("#52c41a".to_string()),
        );

        // 警告色
        values.insert(
            TokenPath::from_str("color.warning.500"),
            TokenValue::String("#faad14".to_string()),
        );

        // 错误色
        values.insert(
            TokenPath::from_str("color.error.500"),
            TokenValue::String("#ff4d4f".to_string()),
        );

        // 信息色
        values.insert(
            TokenPath::from_str("color.info.500"),
            TokenValue::String("#1677ff".to_string()),
        );

        // 文本颜色
        values.insert(
            TokenPath::from_str("color.text.primary"),
            TokenValue::String("rgba(0, 0, 0, 0.88)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.secondary"),
            TokenValue::String("rgba(0, 0, 0, 0.65)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.tertiary"),
            TokenValue::String("rgba(0, 0, 0, 0.45)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.quaternary"),
            TokenValue::String("rgba(0, 0, 0, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            TokenPath::from_str("color.background.default"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.container"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.elevated"),
            TokenValue::String("#ffffff".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.layout"),
            TokenValue::String("#f5f5f5".to_string()),
        );

        // 边框颜色
        values.insert(
            TokenPath::from_str("color.border.default"),
            TokenValue::String("#d9d9d9".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.border.secondary"),
            TokenValue::String("#f0f0f0".to_string()),
        );

        // 字体大小
        values.insert(
            TokenPath::from_str("typography.fontSize.xs"),
            TokenValue::Number(12.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.sm"),
            TokenValue::Number(14.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.base"),
            TokenValue::Number(14.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.lg"),
            TokenValue::Number(16.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.xl"),
            TokenValue::Number(20.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.2xl"),
            TokenValue::Number(24.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.3xl"),
            TokenValue::Number(30.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontSize.4xl"),
            TokenValue::Number(38.0),
        );

        // 字体权重
        values.insert(
            TokenPath::from_str("typography.fontWeight.normal"),
            TokenValue::Number(400.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontWeight.medium"),
            TokenValue::Number(500.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontWeight.semibold"),
            TokenValue::Number(600.0),
        );
        values.insert(
            TokenPath::from_str("typography.fontWeight.bold"),
            TokenValue::Number(700.0),
        );

        // 行高
        values.insert(
            TokenPath::from_str("typography.lineHeight.tight"),
            TokenValue::Number(1.2),
        );
        values.insert(
            TokenPath::from_str("typography.lineHeight.normal"),
            TokenValue::Number(1.5),
        );
        values.insert(
            TokenPath::from_str("typography.lineHeight.relaxed"),
            TokenValue::Number(1.75),
        );

        // 间距
        values.insert(TokenPath::from_str("spacing.xs"), TokenValue::Number(4.0));
        values.insert(TokenPath::from_str("spacing.sm"), TokenValue::Number(8.0));
        values.insert(TokenPath::from_str("spacing.md"), TokenValue::Number(16.0));
        values.insert(TokenPath::from_str("spacing.lg"), TokenValue::Number(24.0));
        values.insert(TokenPath::from_str("spacing.xl"), TokenValue::Number(32.0));
        values.insert(TokenPath::from_str("spacing.2xl"), TokenValue::Number(48.0));
        values.insert(TokenPath::from_str("spacing.3xl"), TokenValue::Number(64.0));

        // 边框半径
        values.insert(
            TokenPath::from_str("border.radius.none"),
            TokenValue::Number(0.0),
        );
        values.insert(
            TokenPath::from_str("border.radius.sm"),
            TokenValue::Number(2.0),
        );
        values.insert(
            TokenPath::from_str("border.radius.base"),
            TokenValue::Number(6.0),
        );
        values.insert(
            TokenPath::from_str("border.radius.lg"),
            TokenValue::Number(8.0),
        );
        values.insert(
            TokenPath::from_str("border.radius.xl"),
            TokenValue::Number(12.0),
        );
        values.insert(
            TokenPath::from_str("border.radius.full"),
            TokenValue::String("50%".to_string()),
        );

        // 边框宽度
        values.insert(
            TokenPath::from_str("border.width.none"),
            TokenValue::Number(0.0),
        );
        values.insert(
            TokenPath::from_str("border.width.thin"),
            TokenValue::Number(1.0),
        );
        values.insert(
            TokenPath::from_str("border.width.thick"),
            TokenValue::Number(2.0),
        );

        values
    }

    /// 获取默认的深色主题令牌值
    pub fn get_dark_theme_values() -> HashMap<TokenPath, TokenValue> {
        let mut values = Self::get_light_theme_values();

        // 覆盖深色主题特定的值

        // 文本颜色
        values.insert(
            TokenPath::from_str("color.text.primary"),
            TokenValue::String("rgba(255, 255, 255, 0.85)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.secondary"),
            TokenValue::String("rgba(255, 255, 255, 0.65)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.tertiary"),
            TokenValue::String("rgba(255, 255, 255, 0.45)".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.text.quaternary"),
            TokenValue::String("rgba(255, 255, 255, 0.25)".to_string()),
        );

        // 背景颜色
        values.insert(
            TokenPath::from_str("color.background.default"),
            TokenValue::String("#141414".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.container"),
            TokenValue::String("#1f1f1f".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.elevated"),
            TokenValue::String("#262626".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.background.layout"),
            TokenValue::String("#000000".to_string()),
        );

        // 边框颜色
        values.insert(
            TokenPath::from_str("color.border.default"),
            TokenValue::String("#434343".to_string()),
        );
        values.insert(
            TokenPath::from_str("color.border.secondary"),
            TokenValue::String("#303030".to_string()),
        );

        values
    }

    /// 创建预配置的令牌值存储
    pub fn create_default_store() -> TokenValueStore {
        let mut store = TokenValueStore::new();

        // 设置浅色主题值
        store.set_values_batch(Self::get_light_theme_values(), ThemeVariant::Light);

        // 设置深色主题值
        store.set_values_batch(Self::get_dark_theme_values(), ThemeVariant::Dark);

        store
    }

    /// 创建暗色主题的令牌值存储
    pub fn create_dark_store() -> TokenValueStore {
        let mut store = TokenValueStore::new();

        // 只设置深色主题值
        store.set_values_batch(Self::get_dark_theme_values(), ThemeVariant::Dark);

        store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_value_store() {
        let mut store = TokenValueStore::new();
        let path = TokenPath::from_str("color.primary.500");
        let value = TokenValue::String("#1677ff".to_string());

        store.set_value(path.clone(), value.clone(), ThemeVariant::Light);

        assert_eq!(store.get_value(&path, ThemeVariant::Light), Some(&value));
        assert_eq!(store.get_value(&path, ThemeVariant::Dark), None);
        assert!(store.has_token(&path, ThemeVariant::Light));
        assert!(!store.has_token(&path, ThemeVariant::Dark));
    }

    #[test]
    fn test_ant_design_default_values() {
        let store = AntDesignTokenValues::create_default_store();

        let primary_path = TokenPath::from_str("color.primary.500");
        assert!(store.has_token(&primary_path, ThemeVariant::Light));
        assert!(store.has_token(&primary_path, ThemeVariant::Dark));

        let text_path = TokenPath::from_str("color.text.primary");
        let light_text = store.get_value(&text_path, ThemeVariant::Light);
        let dark_text = store.get_value(&text_path, ThemeVariant::Dark);

        assert!(light_text.is_some());
        assert!(dark_text.is_some());
        assert_ne!(light_text, dark_text); // 浅色和深色主题的文本颜色应该不同
    }
}
