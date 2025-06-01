//! Ant Design 预设主题
//!
//! 提供 Ant Design 官方的颜色、字体、间距等设计令牌预设值。
//! 包括亮色主题和暗色主题的完整实现。

use super::design_tokens::*;
use super::token_value::TokenValue;

impl ColorTokens {
    /// 创建 Ant Design 亮色主题颜色令牌
    pub fn ant_design_light() -> Self {
        Self {
            primary: ColorScale::ant_design_blue(),
            success: ColorScale::ant_design_green(),
            warning: ColorScale::ant_design_gold(),
            error: ColorScale::ant_design_red(),
            info: ColorScale::ant_design_blue(),
            neutral: ColorScale::ant_design_gray(),
            text: TextColors::ant_design_light(),
            background: BackgroundColors::ant_design_light(),
            border: BorderColors::ant_design_light(),
        }
    }

    /// 创建 Ant Design 暗色主题颜色令牌
    pub fn ant_design_dark() -> Self {
        Self {
            primary: ColorScale::ant_design_blue(),
            success: ColorScale::ant_design_green(),
            warning: ColorScale::ant_design_gold(),
            error: ColorScale::ant_design_red(),
            info: ColorScale::ant_design_blue(),
            neutral: ColorScale::ant_design_gray_dark(),
            text: TextColors::ant_design_dark(),
            background: BackgroundColors::ant_design_dark(),
            border: BorderColors::ant_design_dark(),
        }
    }
}

impl ColorScale {
    /// Ant Design 蓝色系（主色调）
    pub fn ant_design_blue() -> Self {
        Self {
            _50: TokenValue::color("#e6f4ff"),
            _100: TokenValue::color("#bae0ff"),
            _200: TokenValue::color("#91caff"),
            _300: TokenValue::color("#69b1ff"),
            _400: TokenValue::color("#4096ff"),
            _500: TokenValue::color("#1677ff"), // 主色调
            _600: TokenValue::color("#0958d9"),
            _700: TokenValue::color("#003eb3"),
            _800: TokenValue::color("#002c8c"),
            _900: TokenValue::color("#001d66"),
            _950: TokenValue::color("#001529"),
        }
    }

    /// Ant Design 绿色系（成功色）
    pub fn ant_design_green() -> Self {
        Self {
            _50: TokenValue::color("#f6ffed"),
            _100: TokenValue::color("#d9f7be"),
            _200: TokenValue::color("#b7eb8f"),
            _300: TokenValue::color("#95de64"),
            _400: TokenValue::color("#73d13d"),
            _500: TokenValue::color("#52c41a"), // 成功色
            _600: TokenValue::color("#389e0d"),
            _700: TokenValue::color("#237804"),
            _800: TokenValue::color("#135200"),
            _900: TokenValue::color("#092b00"),
            _950: TokenValue::color("#051b00"),
        }
    }

    /// Ant Design 金色系（警告色）
    pub fn ant_design_gold() -> Self {
        Self {
            _50: TokenValue::color("#fffbe6"),
            _100: TokenValue::color("#fff1b8"),
            _200: TokenValue::color("#ffe58f"),
            _300: TokenValue::color("#ffd666"),
            _400: TokenValue::color("#ffc53d"),
            _500: TokenValue::color("#faad14"), // 警告色
            _600: TokenValue::color("#d48806"),
            _700: TokenValue::color("#ad6800"),
            _800: TokenValue::color("#874d00"),
            _900: TokenValue::color("#613400"),
            _950: TokenValue::color("#3d2100"),
        }
    }

    /// Ant Design 红色系（错误色）
    pub fn ant_design_red() -> Self {
        Self {
            _50: TokenValue::color("#fff2f0"),
            _100: TokenValue::color("#ffccc7"),
            _200: TokenValue::color("#ffa39e"),
            _300: TokenValue::color("#ff7875"),
            _400: TokenValue::color("#ff4d4f"),
            _500: TokenValue::color("#f5222d"), // 错误色
            _600: TokenValue::color("#cf1322"),
            _700: TokenValue::color("#a8071a"),
            _800: TokenValue::color("#820014"),
            _900: TokenValue::color("#5c0011"),
            _950: TokenValue::color("#2a0009"),
        }
    }

    /// Ant Design 灰色系（中性色）
    pub fn ant_design_gray() -> Self {
        Self {
            _50: TokenValue::color("#fafafa"),
            _100: TokenValue::color("#f5f5f5"),
            _200: TokenValue::color("#f0f0f0"),
            _300: TokenValue::color("#d9d9d9"),
            _400: TokenValue::color("#bfbfbf"),
            _500: TokenValue::color("#8c8c8c"),
            _600: TokenValue::color("#595959"),
            _700: TokenValue::color("#434343"),
            _800: TokenValue::color("#262626"),
            _900: TokenValue::color("#1f1f1f"),
            _950: TokenValue::color("#141414"),
        }
    }

    /// Ant Design 暗色主题灰色系
    pub fn ant_design_gray_dark() -> Self {
        Self {
            _50: TokenValue::color("#141414"),
            _100: TokenValue::color("#1f1f1f"),
            _200: TokenValue::color("#262626"),
            _300: TokenValue::color("#434343"),
            _400: TokenValue::color("#595959"),
            _500: TokenValue::color("#8c8c8c"),
            _600: TokenValue::color("#bfbfbf"),
            _700: TokenValue::color("#d9d9d9"),
            _800: TokenValue::color("#f0f0f0"),
            _900: TokenValue::color("#f5f5f5"),
            _950: TokenValue::color("#fafafa"),
        }
    }
}

impl TextColors {
    /// Ant Design 亮色主题文本颜色
    pub fn ant_design_light() -> Self {
        Self {
            primary: TokenValue::color("rgba(0, 0, 0, 0.88)"),
            secondary: TokenValue::color("rgba(0, 0, 0, 0.65)"),
            disabled: TokenValue::color("rgba(0, 0, 0, 0.25)"),
            inverse: TokenValue::color("rgba(255, 255, 255, 0.85)"),
        }
    }

    /// Ant Design 暗色主题文本颜色
    pub fn ant_design_dark() -> Self {
        Self {
            primary: TokenValue::color("rgba(255, 255, 255, 0.85)"),
            secondary: TokenValue::color("rgba(255, 255, 255, 0.65)"),
            disabled: TokenValue::color("rgba(255, 255, 255, 0.25)"),
            inverse: TokenValue::color("rgba(0, 0, 0, 0.88)"),
        }
    }
}

impl BackgroundColors {
    /// Ant Design 亮色主题背景颜色
    pub fn ant_design_light() -> Self {
        Self {
            primary: TokenValue::color("#ffffff"),
            secondary: TokenValue::color("#fafafa"),
            container: TokenValue::color("#ffffff"),
            elevated: TokenValue::color("#ffffff"),
            overlay: TokenValue::color("rgba(0, 0, 0, 0.45)"),
        }
    }

    /// Ant Design 暗色主题背景颜色
    pub fn ant_design_dark() -> Self {
        Self {
            primary: TokenValue::color("#141414"),
            secondary: TokenValue::color("#1f1f1f"),
            container: TokenValue::color("#1f1f1f"),
            elevated: TokenValue::color("#262626"),
            overlay: TokenValue::color("rgba(0, 0, 0, 0.65)"),
        }
    }
}

impl BorderColors {
    /// Ant Design 亮色主题边框颜色
    pub fn ant_design_light() -> Self {
        Self {
            default: TokenValue::color("#d9d9d9"),
            divider: TokenValue::color("#f0f0f0"),
            focus: TokenValue::color("#1677ff"),
            error: TokenValue::color("#f5222d"),
        }
    }

    /// Ant Design 暗色主题边框颜色
    pub fn ant_design_dark() -> Self {
        Self {
            default: TokenValue::color("#434343"),
            divider: TokenValue::color("#262626"),
            focus: TokenValue::color("#1677ff"),
            error: TokenValue::color("#f5222d"),
        }
    }
}

impl TypographyTokens {
    /// 创建 Ant Design 默认字体令牌
    pub fn ant_design_default() -> Self {
        Self {
            font_family: FontFamilyTokens::ant_design_default(),
            font_size: FontSizeTokens::ant_design_default(),
            font_weight: FontWeightTokens::ant_design_default(),
            line_height: LineHeightTokens::ant_design_default(),
            letter_spacing: LetterSpacingTokens::ant_design_default(),
        }
    }
}

impl FontFamilyTokens {
    /// Ant Design 默认字体族
    pub fn ant_design_default() -> Self {
        Self {
            sans: TokenValue::string("-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'"),
            serif: TokenValue::string("Georgia, Cambria, 'Times New Roman', Times, serif"),
            mono: TokenValue::string("'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace"),
        }
    }
}

impl FontSizeTokens {
    /// Ant Design 默认字体大小
    pub fn ant_design_default() -> Self {
        Self {
            xs: TokenValue::size("12px"),
            sm: TokenValue::size("14px"),
            base: TokenValue::size("14px"), // Ant Design 基础字体大小
            lg: TokenValue::size("16px"),
            xl: TokenValue::size("20px"),
            _2xl: TokenValue::size("24px"),
            _3xl: TokenValue::size("30px"),
            _4xl: TokenValue::size("36px"),
            _5xl: TokenValue::size("48px"),
            _6xl: TokenValue::size("60px"),
        }
    }
}

impl FontWeightTokens {
    /// Ant Design 默认字体粗细
    pub fn ant_design_default() -> Self {
        Self {
            thin: TokenValue::number(100.0),
            light: TokenValue::number(300.0),
            normal: TokenValue::number(400.0),
            medium: TokenValue::number(500.0),
            semibold: TokenValue::number(600.0),
            bold: TokenValue::number(700.0),
            extrabold: TokenValue::number(800.0),
        }
    }
}

impl LineHeightTokens {
    /// Ant Design 默认行高
    pub fn ant_design_default() -> Self {
        Self {
            tight: TokenValue::number(1.2),
            normal: TokenValue::number(1.5715), // Ant Design 默认行高
            relaxed: TokenValue::number(1.75),
            loose: TokenValue::number(2.0),
        }
    }
}

impl LetterSpacingTokens {
    /// Ant Design 默认字间距
    pub fn ant_design_default() -> Self {
        Self {
            tight: TokenValue::size("-0.025em"),
            normal: TokenValue::size("0em"),
            wide: TokenValue::size("0.025em"),
        }
    }
}

impl SpacingTokens {
    /// 创建 Ant Design 默认间距令牌
    pub fn ant_design_default() -> Self {
        Self {
            _0: TokenValue::size("0px"),
            _1: TokenValue::size("1px"),
            _2: TokenValue::size("2px"),
            _3: TokenValue::size("4px"),
            _4: TokenValue::size("4px"),
            _5: TokenValue::size("6px"),
            _6: TokenValue::size("8px"),
            _8: TokenValue::size("8px"),
            _10: TokenValue::size("12px"),
            _12: TokenValue::size("12px"),
            _16: TokenValue::size("16px"),
            _20: TokenValue::size("20px"),
            _24: TokenValue::size("24px"),
            _32: TokenValue::size("32px"),
            _40: TokenValue::size("40px"),
            _48: TokenValue::size("48px"),
            _56: TokenValue::size("56px"),
            _64: TokenValue::size("64px"),
        }
    }
}

impl ShadowTokens {
    /// 创建 Ant Design 默认阴影令牌
    pub fn ant_design_default() -> Self {
        Self {
            none: TokenValue::string("none"),
            sm: TokenValue::string("0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)"),
            base: TokenValue::string("0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)"),
            md: TokenValue::string("0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12), 0 9px 28px 8px rgba(0, 0, 0, 0.05)"),
            lg: TokenValue::string("0 9px 28px 8px rgba(0, 0, 0, 0.05), 0 6px 16px 0 rgba(0, 0, 0, 0.08), 0 3px 6px -4px rgba(0, 0, 0, 0.12)"),
            xl: TokenValue::string("0 16px 32px 16px rgba(0, 0, 0, 0.04), 0 10px 24px 0 rgba(0, 0, 0, 0.08), 0 6px 16px -4px rgba(0, 0, 0, 0.12)"),
            _2xl: TokenValue::string("0 25px 50px -12px rgba(0, 0, 0, 0.25)"),
            inner: TokenValue::string("inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)"),
        }
    }

    /// 创建 Ant Design 暗色主题阴影令牌
    pub fn ant_design_dark() -> Self {
        Self {
            none: TokenValue::string("none"),
            sm: TokenValue::string("0 1px 2px 0 rgba(0, 0, 0, 0.16), 0 1px 6px -1px rgba(0, 0, 0, 0.12), 0 2px 4px 0 rgba(0, 0, 0, 0.09)"),
            base: TokenValue::string("0 1px 2px 0 rgba(0, 0, 0, 0.16), 0 1px 6px -1px rgba(0, 0, 0, 0.12), 0 2px 4px 0 rgba(0, 0, 0, 0.09)"),
            md: TokenValue::string("0 6px 16px 0 rgba(0, 0, 0, 0.32), 0 3px 6px -4px rgba(0, 0, 0, 0.32), 0 9px 28px 8px rgba(0, 0, 0, 0.20)"),
            lg: TokenValue::string("0 9px 28px 8px rgba(0, 0, 0, 0.20), 0 6px 16px 0 rgba(0, 0, 0, 0.32), 0 3px 6px -4px rgba(0, 0, 0, 0.32)"),
            xl: TokenValue::string("0 16px 32px 16px rgba(0, 0, 0, 0.24), 0 10px 24px 0 rgba(0, 0, 0, 0.32), 0 6px 16px -4px rgba(0, 0, 0, 0.32)"),
            _2xl: TokenValue::string("0 25px 50px -12px rgba(0, 0, 0, 0.50)"),
            inner: TokenValue::string("inset 0 2px 4px 0 rgba(0, 0, 0, 0.16)"),
        }
    }
}

impl BorderTokens {
    /// 创建 Ant Design 默认边框令牌
    pub fn ant_design_default() -> Self {
        Self {
            width: BorderWidthTokens::ant_design_default(),
            style: BorderStyleTokens::ant_design_default(),
        }
    }
}

impl BorderWidthTokens {
    /// Ant Design 默认边框宽度
    pub fn ant_design_default() -> Self {
        Self {
            none: TokenValue::size("0px"),
            thin: TokenValue::size("1px"),
            base: TokenValue::size("1px"),
            thick: TokenValue::size("2px"),
        }
    }
}

impl BorderStyleTokens {
    /// Ant Design 默认边框样式
    pub fn ant_design_default() -> Self {
        Self {
            solid: TokenValue::string("solid"),
            dashed: TokenValue::string("dashed"),
            dotted: TokenValue::string("dotted"),
            double: TokenValue::string("double"),
        }
    }
}

impl RadiusTokens {
    /// 创建 Ant Design 默认圆角令牌
    pub fn ant_design_default() -> Self {
        Self {
            none: TokenValue::size("0px"),
            sm: TokenValue::size("2px"),
            base: TokenValue::size("6px"), // Ant Design 默认圆角
            md: TokenValue::size("8px"),
            lg: TokenValue::size("12px"),
            xl: TokenValue::size("16px"),
            _2xl: TokenValue::size("24px"),
            _3xl: TokenValue::size("32px"),
            full: TokenValue::size("9999px"),
        }
    }
}

impl MotionTokens {
    /// 创建 Ant Design 默认动画令牌
    pub fn ant_design_default() -> Self {
        Self {
            duration: DurationTokens::ant_design_default(),
            easing: EasingTokens::ant_design_default(),
        }
    }
}

impl DurationTokens {
    /// Ant Design 默认动画时长
    pub fn ant_design_default() -> Self {
        Self {
            fast: TokenValue::string("0.1s"),
            base: TokenValue::string("0.2s"),
            slow: TokenValue::string("0.3s"),
        }
    }
}

impl EasingTokens {
    /// Ant Design 默认缓动函数
    pub fn ant_design_default() -> Self {
        Self {
            linear: TokenValue::string("linear"),
            ease_in: TokenValue::string("cubic-bezier(0.55, 0.055, 0.675, 0.19)"),
            ease_out: TokenValue::string("cubic-bezier(0.215, 0.61, 0.355, 1)"),
            ease_in_out: TokenValue::string("cubic-bezier(0.645, 0.045, 0.355, 1)"),
        }
    }
}

impl ZIndexTokens {
    /// 创建 Ant Design 默认层级令牌
    pub fn ant_design_default() -> Self {
        Self {
            hide: TokenValue::number(-1.0),
            auto: TokenValue::string("auto"),
            base: TokenValue::number(0.0),
            docked: TokenValue::number(10.0),
            dropdown: TokenValue::number(1050.0),
            sticky: TokenValue::number(1020.0),
            banner: TokenValue::number(1030.0),
            overlay: TokenValue::number(1040.0),
            modal: TokenValue::number(1050.0),
            popover: TokenValue::number(1060.0),
            skiplink: TokenValue::number(1070.0),
            toast: TokenValue::number(1080.0),
            tooltip: TokenValue::number(1090.0),
        }
    }
}

impl BreakpointTokens {
    /// 创建 Ant Design 默认断点令牌
    pub fn ant_design_default() -> Self {
        Self {
            xs: TokenValue::size("480px"),
            sm: TokenValue::size("576px"),
            md: TokenValue::size("768px"),
            lg: TokenValue::size("992px"),
            xl: TokenValue::size("1200px"),
            _2xl: TokenValue::size("1600px"),
        }
    }
}

// 为其他令牌类型实现 CSS 变量生成
impl TypographyTokens {
    /// 生成字体相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 字体族
        css.push_str(&format!(
            "  --font-family-sans: {};\n",
            self.font_family.sans.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-family-serif: {};\n",
            self.font_family.serif.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-family-mono: {};\n",
            self.font_family.mono.to_css_value()
        ));

        // 字体大小
        css.push_str(&format!(
            "  --font-size-xs: {};\n",
            self.font_size.xs.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-sm: {};\n",
            self.font_size.sm.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-base: {};\n",
            self.font_size.base.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-lg: {};\n",
            self.font_size.lg.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-xl: {};\n",
            self.font_size.xl.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-2xl: {};\n",
            self.font_size._2xl.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-3xl: {};\n",
            self.font_size._3xl.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-4xl: {};\n",
            self.font_size._4xl.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-5xl: {};\n",
            self.font_size._5xl.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-size-6xl: {};\n",
            self.font_size._6xl.to_css_value()
        ));

        // 字体粗细
        css.push_str(&format!(
            "  --font-weight-thin: {};\n",
            self.font_weight.thin.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-light: {};\n",
            self.font_weight.light.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-normal: {};\n",
            self.font_weight.normal.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-medium: {};\n",
            self.font_weight.medium.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-semibold: {};\n",
            self.font_weight.semibold.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-bold: {};\n",
            self.font_weight.bold.to_css_value()
        ));
        css.push_str(&format!(
            "  --font-weight-extrabold: {};\n",
            self.font_weight.extrabold.to_css_value()
        ));

        // 行高
        css.push_str(&format!(
            "  --line-height-tight: {};\n",
            self.line_height.tight.to_css_value()
        ));
        css.push_str(&format!(
            "  --line-height-normal: {};\n",
            self.line_height.normal.to_css_value()
        ));
        css.push_str(&format!(
            "  --line-height-relaxed: {};\n",
            self.line_height.relaxed.to_css_value()
        ));
        css.push_str(&format!(
            "  --line-height-loose: {};\n",
            self.line_height.loose.to_css_value()
        ));

        // 字间距
        css.push_str(&format!(
            "  --letter-spacing-tight: {};\n",
            self.letter_spacing.tight.to_css_value()
        ));
        css.push_str(&format!(
            "  --letter-spacing-normal: {};\n",
            self.letter_spacing.normal.to_css_value()
        ));
        css.push_str(&format!(
            "  --letter-spacing-wide: {};\n",
            self.letter_spacing.wide.to_css_value()
        ));

        css
    }
}

impl SpacingTokens {
    /// 生成间距相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&format!("  --spacing-0: {};\n", self._0.to_css_value()));
        css.push_str(&format!("  --spacing-1: {};\n", self._1.to_css_value()));
        css.push_str(&format!("  --spacing-2: {};\n", self._2.to_css_value()));
        css.push_str(&format!("  --spacing-3: {};\n", self._3.to_css_value()));
        css.push_str(&format!("  --spacing-4: {};\n", self._4.to_css_value()));
        css.push_str(&format!("  --spacing-5: {};\n", self._5.to_css_value()));
        css.push_str(&format!("  --spacing-6: {};\n", self._6.to_css_value()));
        css.push_str(&format!("  --spacing-8: {};\n", self._8.to_css_value()));
        css.push_str(&format!("  --spacing-10: {};\n", self._10.to_css_value()));
        css.push_str(&format!("  --spacing-12: {};\n", self._12.to_css_value()));
        css.push_str(&format!("  --spacing-16: {};\n", self._16.to_css_value()));
        css.push_str(&format!("  --spacing-20: {};\n", self._20.to_css_value()));
        css.push_str(&format!("  --spacing-24: {};\n", self._24.to_css_value()));
        css.push_str(&format!("  --spacing-32: {};\n", self._32.to_css_value()));
        css.push_str(&format!("  --spacing-40: {};\n", self._40.to_css_value()));
        css.push_str(&format!("  --spacing-48: {};\n", self._48.to_css_value()));
        css.push_str(&format!("  --spacing-56: {};\n", self._56.to_css_value()));
        css.push_str(&format!("  --spacing-64: {};\n", self._64.to_css_value()));

        css
    }
}

impl ShadowTokens {
    /// 生成阴影相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&format!("  --shadow-none: {};\n", self.none.to_css_value()));
        css.push_str(&format!("  --shadow-sm: {};\n", self.sm.to_css_value()));
        css.push_str(&format!("  --shadow-base: {};\n", self.base.to_css_value()));
        css.push_str(&format!("  --shadow-md: {};\n", self.md.to_css_value()));
        css.push_str(&format!("  --shadow-lg: {};\n", self.lg.to_css_value()));
        css.push_str(&format!("  --shadow-xl: {};\n", self.xl.to_css_value()));
        css.push_str(&format!("  --shadow-2xl: {};\n", self._2xl.to_css_value()));
        css.push_str(&format!(
            "  --shadow-inner: {};\n",
            self.inner.to_css_value()
        ));

        css
    }
}

impl BorderTokens {
    /// 生成边框相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 边框宽度
        css.push_str(&format!(
            "  --border-width-none: {};\n",
            self.width.none.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-width-thin: {};\n",
            self.width.thin.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-width-base: {};\n",
            self.width.base.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-width-thick: {};\n",
            self.width.thick.to_css_value()
        ));

        // 边框样式
        css.push_str(&format!(
            "  --border-style-solid: {};\n",
            self.style.solid.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-style-dashed: {};\n",
            self.style.dashed.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-style-dotted: {};\n",
            self.style.dotted.to_css_value()
        ));
        css.push_str(&format!(
            "  --border-style-double: {};\n",
            self.style.double.to_css_value()
        ));

        css
    }
}

impl RadiusTokens {
    /// 生成圆角相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&format!("  --radius-none: {};\n", self.none.to_css_value()));
        css.push_str(&format!("  --radius-sm: {};\n", self.sm.to_css_value()));
        css.push_str(&format!("  --radius-base: {};\n", self.base.to_css_value()));
        css.push_str(&format!("  --radius-md: {};\n", self.md.to_css_value()));
        css.push_str(&format!("  --radius-lg: {};\n", self.lg.to_css_value()));
        css.push_str(&format!("  --radius-xl: {};\n", self.xl.to_css_value()));
        css.push_str(&format!("  --radius-2xl: {};\n", self._2xl.to_css_value()));
        css.push_str(&format!("  --radius-3xl: {};\n", self._3xl.to_css_value()));
        css.push_str(&format!("  --radius-full: {};\n", self.full.to_css_value()));

        css
    }
}

impl MotionTokens {
    /// 生成动画相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        // 动画时长
        css.push_str(&format!(
            "  --duration-fast: {};\n",
            self.duration.fast.to_css_value()
        ));
        css.push_str(&format!(
            "  --duration-base: {};\n",
            self.duration.base.to_css_value()
        ));
        css.push_str(&format!(
            "  --duration-slow: {};\n",
            self.duration.slow.to_css_value()
        ));

        // 缓动函数
        css.push_str(&format!(
            "  --easing-linear: {};\n",
            self.easing.linear.to_css_value()
        ));
        css.push_str(&format!(
            "  --easing-ease-in: {};\n",
            self.easing.ease_in.to_css_value()
        ));
        css.push_str(&format!(
            "  --easing-ease-out: {};\n",
            self.easing.ease_out.to_css_value()
        ));
        css.push_str(&format!(
            "  --easing-ease-in-out: {};\n",
            self.easing.ease_in_out.to_css_value()
        ));

        css
    }
}

impl ZIndexTokens {
    /// 生成层级相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&format!(
            "  --z-index-hide: {};\n",
            self.hide.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-auto: {};\n",
            self.auto.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-base: {};\n",
            self.base.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-docked: {};\n",
            self.docked.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-dropdown: {};\n",
            self.dropdown.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-sticky: {};\n",
            self.sticky.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-banner: {};\n",
            self.banner.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-overlay: {};\n",
            self.overlay.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-modal: {};\n",
            self.modal.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-popover: {};\n",
            self.popover.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-skiplink: {};\n",
            self.skiplink.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-toast: {};\n",
            self.toast.to_css_value()
        ));
        css.push_str(&format!(
            "  --z-index-tooltip: {};\n",
            self.tooltip.to_css_value()
        ));

        css
    }
}

impl BreakpointTokens {
    /// 生成断点相关的 CSS 变量
    pub fn to_css_variables(&self) -> String {
        let mut css = String::new();

        css.push_str(&format!("  --breakpoint-xs: {};\n", self.xs.to_css_value()));
        css.push_str(&format!("  --breakpoint-sm: {};\n", self.sm.to_css_value()));
        css.push_str(&format!("  --breakpoint-md: {};\n", self.md.to_css_value()));
        css.push_str(&format!("  --breakpoint-lg: {};\n", self.lg.to_css_value()));
        css.push_str(&format!("  --breakpoint-xl: {};\n", self.xl.to_css_value()));
        css.push_str(&format!(
            "  --breakpoint-2xl: {};\n",
            self._2xl.to_css_value()
        ));

        css
    }
}
