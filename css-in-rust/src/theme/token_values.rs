//! 设计令牌值存储模块
//!
//! 本模块负责存储和管理具体的令牌值，支持多主题变体。
//! 职责：令牌值的存储、检索和主题切换

// 移除对design_tokens的依赖，将相关类型定义在本文件中
use super::token_definitions::{ThemeVariant, TokenMetadata, TokenPath, TokenValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 边框颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BorderColors {
    pub primary: String,
    pub secondary: String,
    pub inverse: String,
}

impl Default for BorderColors {
    fn default() -> Self {
        Self {
            primary: "#dddddd".to_string(),
            secondary: "#eeeeee".to_string(),
            inverse: "#434343".to_string(),
        }
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

impl Default for ColorScale {
    fn default() -> Self {
        Self::blue()
    }
}

impl ColorScale {
    /// 蓝色色阶（通用蓝色方案）
    pub fn blue() -> Self {
        Self {
            c1: "#e6f3ff".to_string(),
            c2: "#b3d9ff".to_string(),
            c3: "#80bfff".to_string(),
            c4: "#4da6ff".to_string(),
            c5: "#1a8cff".to_string(),
            c6: "#0066cc".to_string(),
            c7: "#0052a3".to_string(),
            c8: "#003d7a".to_string(),
            c9: "#002952".to_string(),
            c10: "#001429".to_string(),
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

    /// 灰色色阶（通用灰色方案）
    pub fn gray() -> Self {
        Self {
            c1: "#ffffff".to_string(),
            c2: "#fafafa".to_string(),
            c3: "#f5f5f5".to_string(),
            c4: "#eeeeee".to_string(),
            c5: "#dddddd".to_string(),
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
            c9: "#dddddd".to_string(),
            c10: "#eeeeee".to_string(),
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

/// 文本颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
    pub inverse: String,
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

/// 背景颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackgroundColors {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub inverse: String,
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

impl Default for BackgroundColors {
    fn default() -> Self {
        Self {
            primary: "#ffffff".to_string(),
            secondary: "#fafafa".to_string(),
            tertiary: "#f5f5f5".to_string(),
            inverse: "#000000".to_string(),
        }
    }
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

impl ColorTokens {
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

/// 设计令牌集合
///
/// 包含通用设计体系的所有令牌定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignTokens {
    /// 颜色令牌
    pub colors: Colors,
    /// 字体令牌
    pub typography: Typography,
    /// 间距令牌
    pub spacing: Spacing,
    /// 边框令牌
    pub borders: Borders,
    /// 阴影令牌
    pub shadows: Shadows,
    /// 动画令牌
    pub motion: Motion,
    /// 断点令牌
    pub breakpoints: Breakpoints,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignTokens {
    /// 创建新的设计令牌集合
    pub fn new() -> Self {
        Self {
            colors: Colors::default(),
            typography: Typography::default(),
            spacing: Spacing::default(),
            borders: Borders::default(),
            shadows: Shadows::default(),
            motion: Motion::default(),
            breakpoints: Breakpoints::default(),
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

    /// 列出所有可用的令牌路径
    pub fn list_paths(&self, _theme: &str) -> Vec<String> {
        vec![
            "colors.primary".to_string(),
            "colors.secondary".to_string(),
            "typography.font_size.md".to_string(),
            "spacing.md".to_string(),
            "borders.width.thin".to_string(),
            "shadows.sm".to_string(),
            "motion.duration.fast".to_string(),
            "breakpoints.md".to_string(),
        ]
    }

    /// 获取亮色主题的令牌值
    pub fn get_light_theme_values(&self) -> Self {
        // 返回当前实例的克隆，因为默认就是亮色主题
        self.clone()
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

    /// 创建令牌存储
    pub fn create_store(self) -> Self {
        self
    }

    /// 获取支持的主题列表
    pub fn get_supported_themes(&self) -> Vec<crate::theme::ThemeVariant> {
        vec![
            crate::theme::ThemeVariant::Light,
            crate::theme::ThemeVariant::Dark,
        ]
    }

    /// 清空指定主题的令牌（重置为默认值）
    pub fn clear_theme(&mut self, _theme: crate::theme::ThemeVariant) {
        *self = Self::default();
    }

    /// 设置令牌值
    pub fn set_value(&mut self, path: &str, value: String) -> Result<(), String> {
        let parts: Vec<&str> = path.split('.').collect();

        match parts.as_slice() {
            ["colors", "primary"] => {
                self.colors.primary = value;
                Ok(())
            }
            ["colors", "success"] => {
                self.colors.success = value;
                Ok(())
            }
            ["colors", "warning"] => {
                self.colors.warning = value;
                Ok(())
            }
            ["colors", "error"] => {
                self.colors.error = value;
                Ok(())
            }
            ["colors", "info"] => {
                self.colors.info = value;
                Ok(())
            }
            _ => Err(format!("Unsupported token path: {}", path)),
        }
    }

    /// 获取令牌的元数据
    pub fn get_metadata(&self, path: &str) -> Option<super::token_definitions::TokenMetadata> {
        if self.get_value(path).is_some() {
            Some(super::token_definitions::TokenMetadata {
                description: Some(format!("Design token at path: {}", path)),
                token_type: "token".to_string(),
                deprecated: false,
                aliases: Vec::new(),
                tags: Vec::new(),
            })
        } else {
            None
        }
    }

    /// 复制主题
    pub fn copy_theme(&mut self, _base_theme: ThemeVariant, _new_theme: ThemeVariant) {
        // 简单实现，实际应该复制主题相关的令牌值
        // 这里暂时留空，后续可以根据需要实现具体逻辑
    }
}

/// 为 DesignTokens 实现 Iterator trait
/// 允许遍历所有的设计令牌
impl IntoIterator for DesignTokens {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = Vec::new();

        // 添加基础颜色
        items.push(("colors.primary".to_string(), self.colors.primary));
        items.push(("colors.success".to_string(), self.colors.success));
        items.push(("colors.warning".to_string(), self.colors.warning));
        items.push(("colors.error".to_string(), self.colors.error));
        items.push(("colors.info".to_string(), self.colors.info));

        // 添加间距
        items.push(("spacing.xs".to_string(), self.spacing.xs));
        items.push(("spacing.sm".to_string(), self.spacing.sm));
        items.push(("spacing.md".to_string(), self.spacing.md));
        items.push(("spacing.lg".to_string(), self.spacing.lg));
        items.push(("spacing.xl".to_string(), self.spacing.xl));

        items.into_iter()
    }
}

impl<'a> IntoIterator for &'a DesignTokens {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = Vec::new();

        // 添加基础颜色
        items.push(("colors.primary".to_string(), self.colors.primary.clone()));
        items.push(("colors.success".to_string(), self.colors.success.clone()));
        items.push(("colors.warning".to_string(), self.colors.warning.clone()));
        items.push(("colors.error".to_string(), self.colors.error.clone()));
        items.push(("colors.info".to_string(), self.colors.info.clone()));

        // 添加间距
        items.push(("spacing.xs".to_string(), self.spacing.xs.clone()));
        items.push(("spacing.sm".to_string(), self.spacing.sm.clone()));
        items.push(("spacing.md".to_string(), self.spacing.md.clone()));
        items.push(("spacing.lg".to_string(), self.spacing.lg.clone()));
        items.push(("spacing.xl".to_string(), self.spacing.xl.clone()));

        items.into_iter()
    }
}

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

/// 字体权重配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontWeight {
    pub light: String,
    pub normal: String,
    pub medium: String,
    pub semibold: String,
    pub bold: String,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self {
            light: "300".to_string(),
            normal: "400".to_string(),
            medium: "500".to_string(),
            semibold: "600".to_string(),
            bold: "700".to_string(),
        }
    }
}

impl std::fmt::Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "normal: {}, bold: {}", self.normal, self.bold)
    }
}

/// 行高配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineHeight {
    pub tight: String,
    pub normal: String,
    pub relaxed: String,
}

impl Default for LineHeight {
    fn default() -> Self {
        Self {
            tight: "1.25".to_string(),
            normal: "1.5".to_string(),
            relaxed: "1.75".to_string(),
        }
    }
}

impl std::fmt::Display for LineHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "normal: {}", self.normal)
    }
}

/// 字母间距配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LetterSpacing {
    pub tight: String,
    pub normal: String,
    pub wide: String,
}

impl Default for LetterSpacing {
    fn default() -> Self {
        Self {
            tight: "-0.025em".to_string(),
            normal: "0".to_string(),
            wide: "0.025em".to_string(),
        }
    }
}

impl std::fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "normal: {}", self.normal)
    }
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
    pub font_weight: FontWeight,
    pub line_height: LineHeight,
    pub letter_spacing: LetterSpacing,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_size: FontSize::default(),
            font_family: FontFamily::default(),
            font_weight: FontWeight::default(),
            line_height: LineHeight::default(),
            letter_spacing: LetterSpacing::default(),
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
            "300",
            self.font_weight,
            "500",
            "600",
            "700",
            "1.2",
            self.line_height,
            "1.8",
            "-0.05em",
            self.letter_spacing,
            "0.1em"
        )
    }
}

// TextColors已在上面定义，此处删除重复定义

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

// TextColors和BackgroundColors已在上面定义，此处删除重复定义

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
            primary: "#0066cc".to_string(),
            success: "#00aa00".to_string(),
            warning: "#ff9900".to_string(),
            error: "#cc0000".to_string(),
            info: "#0066cc".to_string(),
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

// TokenValueStore 已合并到 DesignTokens 中
