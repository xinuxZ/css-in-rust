//! 设计令牌（Design Tokens）模块
//!
//! 定义 Ant Design 设计体系的所有设计令牌，包括颜色、字体、间距、边框等。
//! 提供类型安全的令牌访问和 CSS 变量生成功能。

use serde::{Deserialize, Serialize};

/// 设计令牌集合
///
/// 包含 Ant Design 设计体系的所有令牌定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignTokens {
    /// 颜色令牌
    pub colors: ColorTokens,
    /// 字体令牌
    pub typography: TypographyTokens,
    /// 间距令牌
    pub spacing: SpacingTokens,
    /// 边框令牌
    pub borders: BorderTokens,
    /// 阴影令牌
    pub shadows: ShadowTokens,
    /// 动画令牌
    pub motion: MotionTokens,
    /// 断点令牌
    pub breakpoints: BreakpointTokens,
}

/// 颜色令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorTokens {
    /// 主色调
    pub primary: String,
    /// 成功色
    pub success: String,
    /// 警告色
    pub warning: String,
    /// 错误色
    pub error: String,
    /// 信息色
    pub info: String,
    /// 文本颜色
    pub text: TextColors,
    /// 背景颜色
    pub background: BackgroundColors,
    /// 边框颜色
    pub border: BorderColors,
    /// 蓝色色阶
    pub blue: ColorScale,
    /// 绿色色阶
    pub green: ColorScale,
    /// 红色色阶
    pub red: ColorScale,
    /// 橙色色阶
    pub orange: ColorScale,
    /// 灰色色阶
    pub gray: ColorScale,
}

/// 文本颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
    pub inverse: String,
}

/// 背景颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub inverse: String,
}

/// 边框颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderColors {
    pub primary: String,
    pub secondary: String,
    pub inverse: String,
}

/// 颜色色阶（1-10级）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScale {
    pub c1: String,
    pub c2: String,
    pub c3: String,
    pub c4: String,
    pub c5: String,
    pub c6: String,
    pub c7: String,
    pub c8: String,
    pub c9: String,
    pub c10: String,
}

/// 字体令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypographyTokens {
    /// 字体族
    pub font_family: FontFamily,
    /// 字体大小
    pub font_size: FontSizes,
    /// 字重
    pub font_weight: FontWeights,
    /// 行高
    pub line_height: LineHeights,
    /// 字间距
    pub letter_spacing: LetterSpacing,
}

/// 字体族
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontFamily {
    pub sans: String,
    pub serif: String,
    pub mono: String,
}

/// 字体大小
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontSizes {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
    pub xxxl: String,
}

/// 字重
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontWeights {
    pub light: String,
    pub normal: String,
    pub medium: String,
    pub semibold: String,
    pub bold: String,
}

/// 行高
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineHeights {
    pub tight: String,
    pub normal: String,
    pub relaxed: String,
}

/// 字间距
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LetterSpacing {
    pub tight: String,
    pub normal: String,
    pub wide: String,
}

/// 间距令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpacingTokens {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
    pub xxxl: String,
}

/// 边框令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderTokens {
    pub width: BorderWidths,
    pub radius: BorderRadius,
    pub style: BorderStyles,
}

/// 边框宽度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderWidths {
    pub none: String,
    pub thin: String,
    pub medium: String,
    pub thick: String,
}

/// 边框圆角
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderRadius {
    pub none: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub full: String,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderStyles {
    pub solid: String,
    pub dashed: String,
    pub dotted: String,
}

/// 阴影令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShadowTokens {
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub inner: String,
}

/// 动画令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MotionTokens {
    pub duration: MotionDuration,
    pub easing: MotionEasing,
}

/// 动画时长
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MotionDuration {
    pub fast: String,
    pub normal: String,
    pub slow: String,
}

/// 动画缓动
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MotionEasing {
    pub linear: String,
    pub ease_in: String,
    pub ease_out: String,
    pub ease_in_out: String,
}

/// 断点令牌
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BreakpointTokens {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
}

impl DesignTokens {
    /// 创建 Ant Design 默认设计令牌
    pub fn ant_design_default() -> Self {
        Self {
            colors: ColorTokens::ant_design_light(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            borders: BorderTokens::default(),
            shadows: ShadowTokens::default(),
            motion: MotionTokens::default(),
            breakpoints: BreakpointTokens::default(),
        }
    }

    /// 创建 Ant Design 暗色主题设计令牌
    pub fn ant_design_dark() -> Self {
        Self {
            colors: ColorTokens::ant_design_dark(),
            typography: TypographyTokens::default(),
            spacing: SpacingTokens::default(),
            borders: BorderTokens::default(),
            shadows: ShadowTokens::dark(),
            motion: MotionTokens::default(),
            breakpoints: BreakpointTokens::default(),
        }
    }

    /// 根据路径获取令牌值
    ///
    /// 支持点分路径，如 "colors.primary"、"spacing.md"、"typography.font_size.lg"
    pub fn get_value(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();

        match parts.as_slice() {
            ["colors", color_path @ ..] => self.colors.get_value(&color_path.join(".")),
            ["typography", typo_path @ ..] => self.typography.get_value(&typo_path.join(".")),
            ["spacing", spacing] => self.spacing.get_value(spacing),
            ["borders", border_path @ ..] => self.borders.get_value(&border_path.join(".")),
            ["shadows", shadow] => self.shadows.get_value(shadow),
            ["motion", motion_path @ ..] => self.motion.get_value(&motion_path.join(".")),
            ["breakpoints", breakpoint] => self.breakpoints.get_value(breakpoint),
            _ => None,
        }
    }

    /// 生成 CSS 变量声明
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&self.colors.to_css_variables());
        css.push_str(&self.typography.to_css_variables());
        css.push_str(&self.spacing.to_css_variables());
        css.push_str(&self.borders.to_css_variables());
        css.push_str(&self.shadows.to_css_variables());
        css.push_str(&self.motion.to_css_variables());
        css.push_str(&self.breakpoints.to_css_variables());

        css
    }
}

impl ColorTokens {
    /// Ant Design 亮色主题颜色
    pub fn ant_design_light() -> Self {
        Self {
            primary: "#1890ff".to_string(),
            success: "#52c41a".to_string(),
            warning: "#faad14".to_string(),
            error: "#f5222d".to_string(),
            info: "#1890ff".to_string(),
            text: TextColors {
                primary: "rgba(0, 0, 0, 0.88)".to_string(),
                secondary: "rgba(0, 0, 0, 0.65)".to_string(),
                disabled: "rgba(0, 0, 0, 0.25)".to_string(),
                inverse: "#ffffff".to_string(),
            },
            background: BackgroundColors {
                primary: "#ffffff".to_string(),
                secondary: "#fafafa".to_string(),
                tertiary: "#f5f5f5".to_string(),
                inverse: "#001529".to_string(),
            },
            border: BorderColors {
                primary: "#d9d9d9".to_string(),
                secondary: "#f0f0f0".to_string(),
                inverse: "#434343".to_string(),
            },
            blue: ColorScale::blue(),
            green: ColorScale::green(),
            red: ColorScale::red(),
            orange: ColorScale::orange(),
            gray: ColorScale::gray(),
        }
    }

    /// Ant Design 暗色主题颜色
    pub fn ant_design_dark() -> Self {
        Self {
            primary: "#1890ff".to_string(),
            success: "#52c41a".to_string(),
            warning: "#faad14".to_string(),
            error: "#f5222d".to_string(),
            info: "#1890ff".to_string(),
            text: TextColors {
                primary: "rgba(255, 255, 255, 0.88)".to_string(),
                secondary: "rgba(255, 255, 255, 0.65)".to_string(),
                disabled: "rgba(255, 255, 255, 0.25)".to_string(),
                inverse: "rgba(0, 0, 0, 0.88)".to_string(),
            },
            background: BackgroundColors {
                primary: "#141414".to_string(),
                secondary: "#1f1f1f".to_string(),
                tertiary: "#262626".to_string(),
                inverse: "#ffffff".to_string(),
            },
            border: BorderColors {
                primary: "#434343".to_string(),
                secondary: "#303030".to_string(),
                inverse: "#d9d9d9".to_string(),
            },
            blue: ColorScale::blue(),
            green: ColorScale::green(),
            red: ColorScale::red(),
            orange: ColorScale::orange(),
            gray: ColorScale::gray_dark(),
        }
    }

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

    /// 生成颜色相关的 CSS 变量
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

// 为各个子结构实现 get_value 和 to_css_variables 方法
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

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-bg-primary: {};\n\
             --color-bg-secondary: {};\n\
             --color-bg-tertiary: {};\n\
             --color-bg-inverse: {};\n",
            self.primary, self.secondary, self.tertiary, self.inverse
        )
    }
}

impl BorderColors {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "primary" => Some(self.primary.clone()),
            "secondary" => Some(self.secondary.clone()),
            "inverse" => Some(self.inverse.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --color-border-primary: {};\n\
             --color-border-secondary: {};\n\
             --color-border-inverse: {};\n",
            self.primary, self.secondary, self.inverse
        )
    }
}

impl ColorScale {
    /// 蓝色色阶
    pub fn blue() -> Self {
        Self {
            c1: "#e6f7ff".to_string(),
            c2: "#bae7ff".to_string(),
            c3: "#91d5ff".to_string(),
            c4: "#69c0ff".to_string(),
            c5: "#40a9ff".to_string(),
            c6: "#1890ff".to_string(),
            c7: "#096dd9".to_string(),
            c8: "#0050b3".to_string(),
            c9: "#003a8c".to_string(),
            c10: "#002766".to_string(),
        }
    }

    /// 绿色色阶
    pub fn green() -> Self {
        Self {
            c1: "#f6ffed".to_string(),
            c2: "#d9f7be".to_string(),
            c3: "#b7eb8f".to_string(),
            c4: "#95de64".to_string(),
            c5: "#73d13d".to_string(),
            c6: "#52c41a".to_string(),
            c7: "#389e0d".to_string(),
            c8: "#237804".to_string(),
            c9: "#135200".to_string(),
            c10: "#092b00".to_string(),
        }
    }

    /// 红色色阶
    pub fn red() -> Self {
        Self {
            c1: "#fff2f0".to_string(),
            c2: "#ffccc7".to_string(),
            c3: "#ffa39e".to_string(),
            c4: "#ff7875".to_string(),
            c5: "#ff4d4f".to_string(),
            c6: "#f5222d".to_string(),
            c7: "#cf1322".to_string(),
            c8: "#a8071a".to_string(),
            c9: "#820014".to_string(),
            c10: "#5c0011".to_string(),
        }
    }

    /// 橙色色阶
    pub fn orange() -> Self {
        Self {
            c1: "#fff7e6".to_string(),
            c2: "#ffe7ba".to_string(),
            c3: "#ffd591".to_string(),
            c4: "#ffc069".to_string(),
            c5: "#ffa940".to_string(),
            c6: "#fa8c16".to_string(),
            c7: "#d46b08".to_string(),
            c8: "#ad4e00".to_string(),
            c9: "#873800".to_string(),
            c10: "#612500".to_string(),
        }
    }

    /// 灰色色阶（亮色主题）
    pub fn gray() -> Self {
        Self {
            c1: "#ffffff".to_string(),
            c2: "#fafafa".to_string(),
            c3: "#f5f5f5".to_string(),
            c4: "#f0f0f0".to_string(),
            c5: "#d9d9d9".to_string(),
            c6: "#bfbfbf".to_string(),
            c7: "#8c8c8c".to_string(),
            c8: "#595959".to_string(),
            c9: "#434343".to_string(),
            c10: "#262626".to_string(),
        }
    }

    /// 灰色色阶（暗色主题）
    pub fn gray_dark() -> Self {
        Self {
            c1: "#141414".to_string(),
            c2: "#1f1f1f".to_string(),
            c3: "#262626".to_string(),
            c4: "#303030".to_string(),
            c5: "#434343".to_string(),
            c6: "#595959".to_string(),
            c7: "#8c8c8c".to_string(),
            c8: "#bfbfbf".to_string(),
            c9: "#d9d9d9".to_string(),
            c10: "#f0f0f0".to_string(),
        }
    }

    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            "1" | "c1" => Some(self.c1.clone()),
            "2" | "c2" => Some(self.c2.clone()),
            "3" | "c3" => Some(self.c3.clone()),
            "4" | "c4" => Some(self.c4.clone()),
            "5" | "c5" => Some(self.c5.clone()),
            "6" | "c6" => Some(self.c6.clone()),
            "7" | "c7" => Some(self.c7.clone()),
            "8" | "c8" => Some(self.c8.clone()),
            "9" | "c9" => Some(self.c9.clone()),
            "10" | "c10" => Some(self.c10.clone()),
            _ => None,
        }
    }

    pub fn to_css_variables(&self, prefix: &str) -> String {
        format!(
            "  --color-{}-1: {};\n\
             --color-{}-2: {};\n\
             --color-{}-3: {};\n\
             --color-{}-4: {};\n\
             --color-{}-5: {};\n\
             --color-{}-6: {};\n\
             --color-{}-7: {};\n\
             --color-{}-8: {};\n\
             --color-{}-9: {};\n\
             --color-{}-10: {};\n",
            prefix,
            self.c1,
            prefix,
            self.c2,
            prefix,
            self.c3,
            prefix,
            self.c4,
            prefix,
            self.c5,
            prefix,
            self.c6,
            prefix,
            self.c7,
            prefix,
            self.c8,
            prefix,
            self.c9,
            prefix,
            self.c10
        )
    }
}

// 为其他令牌类型实现默认值和方法
impl Default for TypographyTokens {
    fn default() -> Self {
        Self {
            font_family: FontFamily {
                sans: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
                serif: "'Times New Roman', Times, serif".to_string(),
                mono: "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
            },
            font_size: FontSizes {
                xs: "12px".to_string(),
                sm: "14px".to_string(),
                md: "16px".to_string(),
                lg: "18px".to_string(),
                xl: "20px".to_string(),
                xxl: "24px".to_string(),
                xxxl: "32px".to_string(),
            },
            font_weight: FontWeights {
                light: "300".to_string(),
                normal: "400".to_string(),
                medium: "500".to_string(),
                semibold: "600".to_string(),
                bold: "700".to_string(),
            },
            line_height: LineHeights {
                tight: "1.2".to_string(),
                normal: "1.5".to_string(),
                relaxed: "1.8".to_string(),
            },
            letter_spacing: LetterSpacing {
                tight: "-0.025em".to_string(),
                normal: "0".to_string(),
                wide: "0.025em".to_string(),
            },
        }
    }
}

impl TypographyTokens {
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
            "  --font-family-sans: {};\n\
             --font-family-serif: {};\n\
             --font-family-mono: {};\n\
             --font-size-xs: {};\n\
             --font-size-sm: {};\n\
             --font-size-md: {};\n\
             --font-size-lg: {};\n\
             --font-size-xl: {};\n\
             --font-size-xxl: {};\n\
             --font-size-xxxl: {};\n\
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
            self.font_family.sans,
            self.font_family.serif,
            self.font_family.mono,
            self.font_size.xs,
            self.font_size.sm,
            self.font_size.md,
            self.font_size.lg,
            self.font_size.xl,
            self.font_size.xxl,
            self.font_size.xxxl,
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

// 为其他令牌类型实现类似的方法...
// 由于篇幅限制，这里只展示核心实现，其他类型的实现类似

impl Default for SpacingTokens {
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

impl SpacingTokens {
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

// 为简化代码，其他类型的实现省略，但结构类似
// 包括 BorderTokens, ShadowTokens, MotionTokens, BreakpointTokens

impl Default for BorderTokens {
    fn default() -> Self {
        Self {
            width: BorderWidths {
                none: "0".to_string(),
                thin: "1px".to_string(),
                medium: "2px".to_string(),
                thick: "4px".to_string(),
            },
            radius: BorderRadius {
                none: "0".to_string(),
                sm: "2px".to_string(),
                md: "6px".to_string(),
                lg: "8px".to_string(),
                xl: "12px".to_string(),
                full: "9999px".to_string(),
            },
            style: BorderStyles {
                solid: "solid".to_string(),
                dashed: "dashed".to_string(),
                dotted: "dotted".to_string(),
            },
        }
    }
}

impl BorderTokens {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            path if path.starts_with("width.") => {
                let sub_path = &path[6..];
                match sub_path {
                    "none" => Some(self.width.none.clone()),
                    "thin" => Some(self.width.thin.clone()),
                    "medium" => Some(self.width.medium.clone()),
                    "thick" => Some(self.width.thick.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("radius.") => {
                let sub_path = &path[7..];
                match sub_path {
                    "none" => Some(self.radius.none.clone()),
                    "sm" => Some(self.radius.sm.clone()),
                    "md" => Some(self.radius.md.clone()),
                    "lg" => Some(self.radius.lg.clone()),
                    "xl" => Some(self.radius.xl.clone()),
                    "full" => Some(self.radius.full.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("style.") => {
                let sub_path = &path[6..];
                match sub_path {
                    "solid" => Some(self.style.solid.clone()),
                    "dashed" => Some(self.style.dashed.clone()),
                    "dotted" => Some(self.style.dotted.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --border-width-none: {};\n\
             --border-width-thin: {};\n\
             --border-width-medium: {};\n\
             --border-width-thick: {};\n\
             --border-radius-none: {};\n\
             --border-radius-sm: {};\n\
             --border-radius-md: {};\n\
             --border-radius-lg: {};\n\
             --border-radius-xl: {};\n\
             --border-radius-full: {};\n",
            self.width.none,
            self.width.thin,
            self.width.medium,
            self.width.thick,
            self.radius.none,
            self.radius.sm,
            self.radius.md,
            self.radius.lg,
            self.radius.xl,
            self.radius.full
        )
    }
}

impl Default for ShadowTokens {
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

impl ShadowTokens {
    /// 暗色主题阴影
    pub fn dark() -> Self {
        Self {
            sm: "0 1px 2px 0 rgba(0, 0, 0, 0.3)".to_string(),
            md: "0 4px 6px -1px rgba(0, 0, 0, 0.4), 0 2px 4px -1px rgba(0, 0, 0, 0.3)".to_string(),
            lg: "0 10px 15px -3px rgba(0, 0, 0, 0.4), 0 4px 6px -2px rgba(0, 0, 0, 0.3)"
                .to_string(),
            xl: "0 20px 25px -5px rgba(0, 0, 0, 0.4), 0 10px 10px -5px rgba(0, 0, 0, 0.2)"
                .to_string(),
            inner: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.3)".to_string(),
        }
    }

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

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration: MotionDuration {
                fast: "150ms".to_string(),
                normal: "300ms".to_string(),
                slow: "500ms".to_string(),
            },
            easing: MotionEasing {
                linear: "linear".to_string(),
                ease_in: "cubic-bezier(0.4, 0, 1, 1)".to_string(),
                ease_out: "cubic-bezier(0, 0, 0.2, 1)".to_string(),
                ease_in_out: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            },
        }
    }
}

impl MotionTokens {
    pub fn get_value(&self, path: &str) -> Option<String> {
        match path {
            path if path.starts_with("duration.") => {
                let sub_path = &path[9..];
                match sub_path {
                    "fast" => Some(self.duration.fast.clone()),
                    "normal" => Some(self.duration.normal.clone()),
                    "slow" => Some(self.duration.slow.clone()),
                    _ => None,
                }
            }
            path if path.starts_with("easing.") => {
                let sub_path = &path[7..];
                match sub_path {
                    "linear" => Some(self.easing.linear.clone()),
                    "ease_in" => Some(self.easing.ease_in.clone()),
                    "ease_out" => Some(self.easing.ease_out.clone()),
                    "ease_in_out" => Some(self.easing.ease_in_out.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            "  --motion-duration-fast: {};\n\
             --motion-duration-normal: {};\n\
             --motion-duration-slow: {};\n\
             --motion-easing-linear: {};\n\
             --motion-easing-ease-in: {};\n\
             --motion-easing-ease-out: {};\n\
             --motion-easing-ease-in-out: {};\n",
            self.duration.fast,
            self.duration.normal,
            self.duration.slow,
            self.easing.linear,
            self.easing.ease_in,
            self.easing.ease_out,
            self.easing.ease_in_out
        )
    }
}

impl Default for BreakpointTokens {
    fn default() -> Self {
        Self {
            xs: "480px".to_string(),
            sm: "576px".to_string(),
            md: "768px".to_string(),
            lg: "992px".to_string(),
            xl: "1200px".to_string(),
            xxl: "1600px".to_string(),
        }
    }
}

impl BreakpointTokens {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_tokens_creation() {
        let tokens = DesignTokens::ant_design_default();
        assert_eq!(tokens.colors.primary, "#1890ff");
        assert_eq!(tokens.spacing.md, "16px");
    }

    #[test]
    fn test_token_path_access() {
        let tokens = DesignTokens::ant_design_default();

        assert_eq!(
            tokens.get_value("colors.primary"),
            Some("#1890ff".to_string())
        );
        assert_eq!(tokens.get_value("spacing.md"), Some("16px".to_string()));
        assert_eq!(
            tokens.get_value("typography.font_size.lg"),
            Some("18px".to_string())
        );
        assert_eq!(
            tokens.get_value("colors.blue.6"),
            Some("#1890ff".to_string())
        );
    }

    #[test]
    fn test_css_variables_generation() {
        let tokens = DesignTokens::ant_design_default();
        let css = tokens.to_css_variables();

        assert!(css.contains("--color-primary: #1890ff;"));
        assert!(css.contains("--spacing-md: 16px;"));
        assert!(css.contains("--font-size-lg: 18px;"));
    }

    #[test]
    fn test_color_scale_access() {
        let blue = ColorScale::blue();
        assert_eq!(blue.get_value("6"), Some("#1890ff".to_string()));
        assert_eq!(blue.get_value("c6"), Some("#1890ff".to_string()));
    }

    #[test]
    fn test_dark_theme_tokens() {
        let dark_tokens = DesignTokens::ant_design_dark();
        assert_eq!(dark_tokens.colors.background.primary, "#141414");
        assert_eq!(dark_tokens.colors.text.primary, "rgba(255, 255, 255, 0.88)");
    }
}
