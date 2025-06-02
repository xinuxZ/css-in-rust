//! 设计令牌系统
//!
//! 本模块提供完整的设计令牌管理系统，整合了令牌定义、值存储、解析和CSS生成功能。
//! 职责：系统级功能、高级API、主题管理
//!
//! 实现完整的分层令牌系统架构：
//! - 全局令牌（Global Tokens）：最基础的设计决策
//! - 别名令牌（Alias Tokens）：语义化的令牌引用
//! - 组件令牌（Component Tokens）：特定组件的令牌
//! - 令牌计算和变换系统
//! - 主题管理和切换
//! - CSS变量生成和导出

use super::{
    css_generator::CssGenerator,
    token_definitions::{
        ColorValue, DimensionValue, ShadowValue, ThemeVariant, TokenDefinitions, TokenPath,
        TokenReference, TokenValidationError, TokenValue, TypographyValue,
    },
    token_resolver::TokenResolver,
    token_values::DesignTokens,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// 设计令牌系统
#[derive(Debug)]
pub struct DesignTokenSystem {
    /// 全局令牌（最基础的设计决策）
    pub global_tokens: GlobalTokens,
    /// 别名令牌（语义化的令牌引用）
    pub alias_tokens: AliasTokens,
    /// 组件令牌（特定组件的令牌）
    pub component_tokens: ComponentTokens,
    /// 令牌计算规则
    pub computation_rules: ComputationRules,
    /// CSS生成器
    css_generator: CssGenerator,
    /// 当前主题
    current_theme: ThemeVariant,
    /// 系统配置
    config: TokenSystemConfig,
    /// 系统元数据
    pub metadata: SystemMetadata,
}

/// 全局令牌（原始设计决策）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTokens {
    /// 基础颜色调色板
    pub color_palette: ColorPalette,
    /// 字体系统
    pub font_system: FontSystem,
    /// 间距系统
    pub spacing_system: SpacingSystem,
    /// 尺寸系统
    pub sizing_system: SizingSystem,
    /// 边框系统
    pub border_system: BorderSystem,
    /// 阴影系统
    pub shadow_system: ShadowSystem,
    /// 动画系统
    pub motion_system: MotionSystem,
}

/// 别名令牌（语义化引用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasTokens {
    /// 语义颜色
    pub semantic_colors: SemanticColors,
    /// 语义字体
    pub semantic_typography: SemanticTypography,
    /// 语义间距
    pub semantic_spacing: SemanticSpacing,
    /// 语义尺寸
    pub semantic_sizing: SemanticSizing,
}

/// 组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentTokens {
    /// 组件特定的令牌存储
    /// 键为组件名称，值为该组件的令牌映射
    pub components: BTreeMap<String, BTreeMap<String, TokenReference>>,
}

/// 令牌计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationRules {
    /// 通用计算规则存储
    /// 键为规则名称，值为规则配置
    pub rules: BTreeMap<String, serde_json::Value>,
}

/// 系统元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetadata {
    /// 版本信息
    pub version: String,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
    /// 作者信息
    pub author: String,
    /// 描述
    pub description: String,
}

/// 基础颜色调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// 主色调
    pub primary: BTreeMap<String, ColorValue>,
    /// 中性色
    pub neutral: BTreeMap<String, ColorValue>,
    /// 功能色
    pub functional: BTreeMap<String, ColorValue>,
    /// 扩展色
    pub extended: BTreeMap<String, ColorValue>,
}

/// 字体系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSystem {
    /// 字体族
    pub families: BTreeMap<String, String>,
    /// 字体大小
    pub sizes: BTreeMap<String, DimensionValue>,
    /// 字重
    pub weights: BTreeMap<String, u16>,
    /// 行高
    pub line_heights: BTreeMap<String, f32>,
    /// 字间距
    pub letter_spacings: BTreeMap<String, DimensionValue>,
}

/// 间距系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    /// 基础间距单位
    pub base_unit: DimensionValue,
    /// 间距比例
    pub scale: Vec<f32>,
    /// 具体间距值
    pub values: BTreeMap<String, DimensionValue>,
}

/// 尺寸系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizingSystem {
    /// 基础尺寸
    pub base_sizes: BTreeMap<String, DimensionValue>,
    /// 组件尺寸
    pub component_sizes: BTreeMap<String, DimensionValue>,
    /// 断点尺寸
    pub breakpoints: BTreeMap<String, DimensionValue>,
}

/// 边框系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSystem {
    /// 边框宽度
    pub widths: BTreeMap<String, DimensionValue>,
    /// 边框样式
    pub styles: BTreeMap<String, String>,
    /// 圆角半径
    pub radius: BTreeMap<String, DimensionValue>,
}

/// 阴影系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowSystem {
    /// 阴影层级
    pub elevations: BTreeMap<String, ShadowValue>,
    /// 阴影颜色
    pub colors: BTreeMap<String, ColorValue>,
}

/// 动画系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionSystem {
    /// 动画时长
    pub durations: BTreeMap<String, String>,
    /// 缓动函数
    pub easings: BTreeMap<String, String>,
    /// 动画延迟
    pub delays: BTreeMap<String, String>,
}

/// 语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticColors {
    /// 文本颜色
    pub text: TextSemanticColors,
    /// 背景颜色
    pub background: BackgroundSemanticColors,
    /// 边框颜色
    pub border: BorderSemanticColors,
    /// 状态颜色
    pub state: StateSemanticColors,
}

/// 文本语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSemanticColors {
    /// 主要文本
    pub primary: TokenReference,
    /// 次要文本
    pub secondary: TokenReference,
    /// 禁用文本
    pub disabled: TokenReference,
    /// 反色文本
    pub inverse: TokenReference,
}

/// 背景语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSemanticColors {
    /// 主背景
    pub primary: TokenReference,
    /// 次背景
    pub secondary: TokenReference,
    /// 强调背景
    pub emphasis: TokenReference,
    /// 反色背景
    pub inverse: TokenReference,
}

/// 边框语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSemanticColors {
    /// 默认边框
    pub default: TokenReference,
    /// 强调边框
    pub emphasis: TokenReference,
    /// 禁用边框
    pub disabled: TokenReference,
}

/// 状态语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSemanticColors {
    /// 成功状态
    pub success: TokenReference,
    /// 警告状态
    pub warning: TokenReference,
    /// 错误状态
    pub error: TokenReference,
    /// 信息状态
    pub info: TokenReference,
}

/// 语义字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTypography {
    /// 标题字体
    pub heading: HeadingTypography,
    /// 正文字体
    pub body: BodyTypography,
    /// 说明字体
    pub caption: CaptionTypography,
    /// 代码字体
    pub code: CodeTypography,
}

/// 标题字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingTypography {
    pub h1: TypographyValue,
    pub h2: TypographyValue,
    pub h3: TypographyValue,
    pub h4: TypographyValue,
    pub h5: TypographyValue,
    pub h6: TypographyValue,
}

/// 正文字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyTypography {
    pub large: TypographyValue,
    pub medium: TypographyValue,
    pub small: TypographyValue,
}

/// 说明字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionTypography {
    pub large: TypographyValue,
    pub medium: TypographyValue,
    pub small: TypographyValue,
}

/// 代码字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTypography {
    pub inline: TypographyValue,
    pub block: TypographyValue,
}

/// 语义间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSpacing {
    /// 组件间距
    pub component: ComponentSpacing,
    /// 内边距
    pub padding: PaddingSpacing,
    /// 外边距
    pub margin: MarginSpacing,
    /// 间隙
    pub gap: GapSpacing,
    /// 布局间距
    pub layout: LayoutSpacing,
}

/// 组件间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 内边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaddingSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 外边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 间隙
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapSpacing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 布局间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSpacing {
    pub container: TokenReference,
    pub section: TokenReference,
    pub content: TokenReference,
}

/// 语义尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSizing {
    /// 组件尺寸
    pub component: ComponentSizing,
    /// 图标尺寸
    pub icon: IconSizing,
    /// 头像尺寸
    pub avatar: AvatarSizing,
}

/// 组件尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 图标尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

/// 头像尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarSizing {
    pub xs: TokenReference,
    pub sm: TokenReference,
    pub md: TokenReference,
    pub lg: TokenReference,
    pub xl: TokenReference,
}

// 组件令牌结构体已移除，应在具体的组件库中定义

// 复杂的计算规则结构体已移除，应在具体的组件库中根据需要定义

/// 令牌系统配置
#[derive(Debug, Clone)]
pub struct TokenSystemConfig {
    /// CSS变量前缀
    pub css_prefix: String,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 是否压缩CSS输出
    pub minify_css: bool,
    /// 是否启用严格模式（严格的类型检查）
    pub strict_mode: bool,
    /// 是否启用深色主题
    pub enable_dark_theme: bool,
}

/// 令牌系统统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSystemStats {
    /// 总令牌数量
    pub total_tokens: usize,
    /// 全局令牌数量
    pub global_tokens: usize,
    /// 别名令牌数量
    pub alias_tokens: usize,
    /// 组件令牌数量
    pub component_tokens: usize,
    /// 主题数量
    pub themes_count: usize,
    /// CSS变量数量
    pub css_variables_count: usize,
    /// 计算规则数量
    pub computation_rules_count: usize,
}

impl Default for TokenSystemConfig {
    fn default() -> Self {
        Self {
            css_prefix: "ant".to_string(),
            enable_cache: true,
            minify_css: false,
            strict_mode: false,
            enable_dark_theme: false,
        }
    }
}

impl DesignTokenSystem {
    /// 获取令牌值
    pub fn get_value(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();

        // 尝试从全局令牌获取
        if let Some(value) = self.resolve_global_token_path(&parts) {
            return Some(value);
        }

        // 尝试从别名令牌获取
        if let Some(value) = self.resolve_alias_token_path(&parts) {
            return Some(value);
        }

        // 尝试从组件令牌获取
        if let Some(value) = self.resolve_component_token_path(&parts) {
            return Some(value);
        }

        None
    }

    /// 创建新的设计令牌系统
    pub fn new() -> Self {
        Self::with_config(TokenSystemConfig::default())
    }

    /// 创建使用默认令牌的系统
    pub fn default_tokens() -> Self {
        let store = DesignTokens::new().create_store();
        Self::with_store(store, TokenSystemConfig::default())
    }

    /// 使用配置创建设计令牌系统
    pub fn with_config(config: TokenSystemConfig) -> Self {
        let store = DesignTokens::new().create_store();
        let resolver = TokenResolver::new(store);
        let css_generator = CssGenerator::new(resolver)
            .with_prefix(config.css_prefix.clone())
            .with_minify(config.minify_css);

        Self {
            global_tokens: GlobalTokens::default(),
            alias_tokens: AliasTokens::default(),
            component_tokens: ComponentTokens::default(),
            computation_rules: ComputationRules::default(),
            css_generator,
            current_theme: ThemeVariant::Light,
            config,
            metadata: SystemMetadata::default(),
        }
    }

    /// 使用自定义存储创建设计令牌系统
    pub fn with_store(store: DesignTokens, config: TokenSystemConfig) -> Self {
        let resolver = TokenResolver::new(store);
        let css_generator = CssGenerator::new(resolver)
            .with_prefix(config.css_prefix.clone())
            .with_minify(config.minify_css);

        Self {
            global_tokens: GlobalTokens::default(),
            alias_tokens: AliasTokens::default(),
            component_tokens: ComponentTokens::default(),
            computation_rules: ComputationRules::default(),
            css_generator,
            current_theme: ThemeVariant::Light,
            config,
            metadata: SystemMetadata::default(),
        }
    }

    /// 导出CSS变量
    /// CSS变量自动导出功能
    pub fn export_css_variables(&mut self) -> Result<String, String> {
        let mut css_output = String::new();

        // 生成根级CSS变量
        css_output.push_str(":root {\n");

        // 导出全局令牌为CSS变量
        css_output.push_str(&self.export_global_tokens_as_css()?);

        // 导出别名令牌为CSS变量
        css_output.push_str(&self.export_alias_tokens_as_css()?);

        // 导出组件令牌为CSS变量
        css_output.push_str(&self.export_component_tokens_as_css()?);

        css_output.push_str("}\n\n");

        // 如果支持深色主题，生成深色主题的CSS变量
        if self.config.enable_dark_theme {
            css_output.push_str(&self.export_dark_theme_css_variables()?);
        }

        Ok(css_output)
    }

    /// 导出全局令牌为CSS变量
    fn export_global_tokens_as_css(&self) -> Result<String, String> {
        let mut css = String::new();

        // 导出颜色调色板
        for (color_group, color_map) in [
            ("primary", &self.global_tokens.color_palette.primary),
            ("neutral", &self.global_tokens.color_palette.neutral),
            ("functional", &self.global_tokens.color_palette.functional),
            ("extended", &self.global_tokens.color_palette.extended),
        ] {
            for (level, color_value) in color_map {
                css.push_str(&format!(
                    "  --{}-color-{}-{}: {};\n",
                    self.config.css_prefix, color_group, level, color_value
                ));
            }
        }

        // 导出字体系统
        for (size_name, size_value) in &self.global_tokens.font_system.sizes {
            css.push_str(&format!(
                "  --{}-font-size-{}: {};\n",
                self.config.css_prefix, size_name, size_value
            ));
        }

        for (weight_name, weight_value) in &self.global_tokens.font_system.weights {
            css.push_str(&format!(
                "  --{}-font-weight-{}: {};\n",
                self.config.css_prefix, weight_name, weight_value
            ));
        }

        for (family_name, family_value) in &self.global_tokens.font_system.families {
            css.push_str(&format!(
                "  --{}-font-family-{}: {};\n",
                self.config.css_prefix, family_name, family_value
            ));
        }

        // 导出间距系统
        for (spacing_name, spacing_value) in &self.global_tokens.spacing_system.values {
            css.push_str(&format!(
                "  --{}-spacing-{}: {};\n",
                self.config.css_prefix, spacing_name, spacing_value
            ));
        }

        // 导出尺寸系统
        for (size_name, size_value) in &self.global_tokens.sizing_system.base_sizes {
            css.push_str(&format!(
                "  --{}-size-{}: {};\n",
                self.config.css_prefix, size_name, size_value
            ));
        }
        for (size_name, size_value) in &self.global_tokens.sizing_system.component_sizes {
            css.push_str(&format!(
                "  --{}-component-size-{}: {};\n",
                self.config.css_prefix, size_name, size_value
            ));
        }
        for (size_name, size_value) in &self.global_tokens.sizing_system.breakpoints {
            css.push_str(&format!(
                "  --{}-breakpoint-{}: {};\n",
                self.config.css_prefix, size_name, size_value
            ));
        }

        // 导出边框系统
        for (width_name, width_value) in &self.global_tokens.border_system.widths {
            css.push_str(&format!(
                "  --{}-border-width-{}: {};\n",
                self.config.css_prefix, width_name, width_value
            ));
        }

        for (radius_name, radius_value) in &self.global_tokens.border_system.radius {
            css.push_str(&format!(
                "  --{}-border-radius-{}: {};\n",
                self.config.css_prefix, radius_name, radius_value
            ));
        }

        // 导出阴影系统
        for (shadow_name, shadow_value) in &self.global_tokens.shadow_system.elevations {
            css.push_str(&format!(
                "  --{}-shadow-{}: {};\n",
                self.config.css_prefix, shadow_name, shadow_value
            ));
        }

        Ok(css)
    }

    /// 导出别名令牌为CSS变量
    fn export_alias_tokens_as_css(&self) -> Result<String, String> {
        let mut css = String::new();

        // 导出语义颜色
        let semantic_colors = [
            (
                "text-primary",
                &self.alias_tokens.semantic_colors.text.primary,
            ),
            (
                "text-secondary",
                &self.alias_tokens.semantic_colors.text.secondary,
            ),
            (
                "text-disabled",
                &self.alias_tokens.semantic_colors.text.disabled,
            ),
            (
                "bg-primary",
                &self.alias_tokens.semantic_colors.background.primary,
            ),
            (
                "bg-secondary",
                &self.alias_tokens.semantic_colors.background.secondary,
            ),
            (
                "border-default",
                &self.alias_tokens.semantic_colors.border.default,
            ),
        ];

        for (var_name, token_ref) in semantic_colors {
            if let Some(resolved_value) = self.resolve_token(token_ref) {
                css.push_str(&format!(
                    "  --{}-{}: {};\n",
                    self.config.css_prefix, var_name, resolved_value
                ));
            }
        }

        // 导出语义字体
        if let Some(font_size) = &self.alias_tokens.semantic_typography.heading.h1.font_size {
            css.push_str(&format!(
                "  --{}-heading-h1-size: {};
",
                self.config.css_prefix, font_size
            ));
        }
        if let Some(font_weight) = &self.alias_tokens.semantic_typography.heading.h1.font_weight {
            css.push_str(&format!(
                "  --{}-heading-h1-weight: {};
",
                self.config.css_prefix, font_weight
            ));
        }
        if let Some(line_height) = &self.alias_tokens.semantic_typography.heading.h1.line_height {
            css.push_str(&format!(
                "  --{}-heading-h1-line-height: {};
",
                self.config.css_prefix, line_height
            ));
        }
        if let Some(font_size) = &self.alias_tokens.semantic_typography.body.large.font_size {
            css.push_str(&format!(
                "  --{}-body-large-size: {};
",
                self.config.css_prefix, font_size
            ));
        }
        if let Some(font_weight) = &self.alias_tokens.semantic_typography.body.large.font_weight {
            css.push_str(&format!(
                "  --{}-body-large-weight: {};
",
                self.config.css_prefix, font_weight
            ));
        }
        if let Some(font_size) = &self.alias_tokens.semantic_typography.body.medium.font_size {
            css.push_str(&format!(
                "  --{}-body-medium-size: {};
",
                self.config.css_prefix, font_size
            ));
        }

        Ok(css)
    }

    /// 导出组件令牌为CSS变量
    fn export_component_tokens_as_css(&self) -> Result<String, String> {
        // 简化实现：返回基础CSS变量
        Ok(":root { --primary-color: #0066cc; --text-color: #000; }".to_string())
    }

    /// 导出深色主题CSS变量
    fn export_dark_theme_css_variables(&mut self) -> Result<String, String> {
        let mut css = String::new();

        // 保存当前主题
        let current_theme = self.current_theme;

        // 切换到深色主题
        self.current_theme = ThemeVariant::Dark;
        self.apply_dark_theme_colors();
        self.update_semantic_tokens_for_theme();

        // 生成深色主题的CSS变量
        css.push_str("[data-theme='dark'] {\n");
        css.push_str(&self.export_global_tokens_as_css()?);
        css.push_str(&self.export_alias_tokens_as_css()?);
        css.push_str(&self.export_component_tokens_as_css()?);
        css.push_str("}\n\n");

        // 恢复原主题
        self.current_theme = current_theme;
        match current_theme {
            ThemeVariant::Light => self.apply_light_theme_colors(),
            ThemeVariant::Dark => self.apply_dark_theme_colors(),
            ThemeVariant::Auto => self.apply_light_theme_colors(),
        }
        self.update_semantic_tokens_for_theme();

        Ok(css)
    }

    /// 解析令牌引用
    /// 根据令牌引用路径解析出最终的令牌值
    pub fn resolve_token(&self, token_ref: &TokenReference) -> Option<String> {
        self.resolve_token_path(&token_ref.reference)
    }

    /// 解析令牌路径
    fn resolve_token_path(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "global" => self.resolve_global_token_path(&parts[1..]),
            "alias" => self.resolve_alias_token_path(&parts[1..]),
            "component" => self.resolve_component_token_path(&parts[1..]),
            // 支持简化路径（直接从根开始）
            "color_palette" => self.resolve_global_token_path(&parts),
            "font_system" => self.resolve_global_token_path(&parts),
            "spacing_system" => self.resolve_global_token_path(&parts),
            "semantic_colors" => self.resolve_alias_token_path(&parts),
            "semantic_typography" => self.resolve_alias_token_path(&parts),
            "button" | "input" | "card" | "table" | "navigation" => {
                self.resolve_component_token_path(&parts)
            }
            _ => None,
        }
    }

    /// 解析全局令牌路径
    fn resolve_global_token_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "color_palette" => self.resolve_color_palette_path(&parts[1..]),
            "font_system" => self.resolve_font_system_path(&parts[1..]),
            "spacing_system" => self.resolve_spacing_system_path(&parts[1..]),
            "sizing_system" => self.resolve_sizing_system_path(&parts[1..]),
            "border_system" => self.resolve_border_system_path(&parts[1..]),
            "shadow_system" => self.resolve_shadow_system_path(&parts[1..]),
            "motion_system" => self.resolve_motion_system_path(&parts[1..]),
            _ => None,
        }
    }

    /// 解析颜色调色板路径
    fn resolve_color_palette_path(&self, parts: &[&str]) -> Option<String> {
        if parts.len() < 2 {
            return None;
        }

        let color_group = parts[0];
        let color_level = parts[1];

        let color_map = match color_group {
            "primary" => &self.global_tokens.color_palette.primary,
            "neutral" => &self.global_tokens.color_palette.neutral,
            "functional" => &self.global_tokens.color_palette.functional,
            "extended" => &self.global_tokens.color_palette.extended,
            _ => return None,
        };

        color_map.get(color_level).map(|v| v.to_string())
    }

    /// 解析字体系统路径
    fn resolve_font_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "sizes" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .font_system
                    .sizes
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            "weights" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .font_system
                    .weights
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            "families" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .font_system
                    .families
                    .get(parts[1])
                    .cloned()
            }
            "line_heights" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .font_system
                    .line_heights
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            _ => None,
        }
    }

    /// 解析间距系统路径
    fn resolve_spacing_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "values" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .spacing_system
                    .values
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            _ => self
                .global_tokens
                .spacing_system
                .values
                .get(parts[0])
                .map(|v| v.to_string()),
        }
    }

    /// 解析尺寸系统路径
    fn resolve_sizing_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        self.global_tokens
            .sizing_system
            .base_sizes
            .get(parts[0])
            .map(|v| v.to_string())
    }

    /// 解析边框系统路径
    fn resolve_border_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "widths" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .border_system
                    .widths
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            "radii" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .border_system
                    .radius
                    .get(parts[1])
                    .map(|v| v.to_string())
            }
            "styles" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .border_system
                    .styles
                    .get(parts[1])
                    .cloned()
            }
            _ => None,
        }
    }

    /// 解析阴影系统路径
    fn resolve_shadow_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        self.global_tokens
            .shadow_system
            .elevations
            .get(parts[0])
            .map(|v| v.to_string())
    }

    /// 解析动画系统路径
    fn resolve_motion_system_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "durations" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .motion_system
                    .durations
                    .get(parts[1])
                    .cloned()
            }
            "easings" => {
                if parts.len() < 2 {
                    return None;
                }
                self.global_tokens
                    .motion_system
                    .easings
                    .get(parts[1])
                    .cloned()
            }
            _ => None,
        }
    }

    /// 解析别名令牌路径
    fn resolve_alias_token_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "semantic_colors" => self.resolve_semantic_colors_path(&parts[1..]),
            "semantic_typography" => self.resolve_semantic_typography_path(&parts[1..]),
            _ => None,
        }
    }

    /// 解析语义颜色路径
    fn resolve_semantic_colors_path(&self, parts: &[&str]) -> Option<String> {
        if parts.len() < 2 {
            return None;
        }

        let category = parts[0];
        let property = parts[1];

        let token_ref = match (category, property) {
            ("text", "primary") => &self.alias_tokens.semantic_colors.text.primary,
            ("text", "secondary") => &self.alias_tokens.semantic_colors.text.secondary,
            ("text", "disabled") => &self.alias_tokens.semantic_colors.text.disabled,
            ("background", "primary") => &self.alias_tokens.semantic_colors.background.primary,
            ("background", "secondary") => &self.alias_tokens.semantic_colors.background.secondary,
            ("border", "default") => &self.alias_tokens.semantic_colors.border.default,
            _ => return None,
        };

        self.resolve_token(token_ref)
    }

    /// 解析语义字体路径
    fn resolve_semantic_typography_path(&self, parts: &[&str]) -> Option<String> {
        if parts.len() < 3 {
            return None;
        }

        let category = parts[0]; // heading, body, caption, code
        let size = parts[1]; // h1, h2, large, medium, small
        let property = parts[2]; // font_size, font_weight, line_height

        match (category, size, property) {
            ("heading", "h1", "font_size") => self
                .alias_tokens
                .semantic_typography
                .heading
                .h1
                .font_size
                .as_ref()
                .map(|v| v.to_string()),
            ("heading", "h1", "font_weight") => self
                .alias_tokens
                .semantic_typography
                .heading
                .h1
                .font_weight
                .map(|v| v.to_string()),
            ("heading", "h1", "line_height") => self
                .alias_tokens
                .semantic_typography
                .heading
                .h1
                .line_height
                .map(|v| v.to_string()),
            ("body", "large", "font_size") => self
                .alias_tokens
                .semantic_typography
                .body
                .large
                .font_size
                .as_ref()
                .map(|v| v.to_string()),
            ("body", "large", "font_weight") => self
                .alias_tokens
                .semantic_typography
                .body
                .large
                .font_weight
                .map(|v| v.to_string()),
            ("body", "medium", "font_size") => self
                .alias_tokens
                .semantic_typography
                .body
                .medium
                .font_size
                .as_ref()
                .map(|v| v.to_string()),
            _ => None,
        }
    }

    /// 解析组件令牌路径
    fn resolve_component_token_path(&self, parts: &[&str]) -> Option<String> {
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "input" => self.resolve_input_token_path(&parts[1..]),
            "card" => self.resolve_card_token_path(&parts[1..]),
            _ => None,
        }
    }

    /// 解析输入框令牌路径（占位实现）
    fn resolve_input_token_path(&self, _parts: &[&str]) -> Option<String> {
        // TODO: 实现输入框令牌路径解析
        None
    }

    /// 解析卡片令牌路径（占位实现）
    fn resolve_card_token_path(&self, _parts: &[&str]) -> Option<String> {
        // TODO: 实现卡片令牌路径解析
        None
    }

    /// 获取令牌值
    pub fn get_token(&mut self, path: &str) -> Result<TokenValue, TokenValidationError> {
        let token_path = TokenPath::from_str(path);
        self.css_generator
            .get_resolver_mut()
            .resolve_token(&token_path, self.current_theme)
    }

    /// 设置令牌值
    pub fn set_token(&mut self, path: &str, value: TokenValue) -> Result<(), String> {
        let token_path = TokenPath::from_str(path);
        self.css_generator.get_resolver_mut().set_token_value(
            &token_path,
            value,
            self.current_theme,
        )
    }

    /// 批量设置令牌值
    pub fn set_tokens_batch(&mut self, tokens: HashMap<String, TokenValue>) -> Result<(), String> {
        for (path, value) in tokens {
            self.set_token(&path, value)?;
        }
        Ok(())
    }

    /// 切换主题
    pub fn switch_theme(&mut self, theme: ThemeVariant) {
        self.current_theme = theme;
        // 清空缓存以确保使用新主题的值
        self.css_generator.get_resolver_mut().clear_cache();
    }

    /// 获取当前主题
    pub fn get_current_theme(&self) -> ThemeVariant {
        self.current_theme
    }

    /// 生成当前主题的CSS变量
    pub fn generate_css_variables(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_css_variables(self.current_theme)
    }

    /// 生成完整的主题CSS
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        self.css_generator.generate_theme_css()
    }

    /// 生成组件样式类
    pub fn generate_component_css(&mut self, component: &str) -> Result<String, String> {
        self.css_generator
            .generate_component_classes(component, self.current_theme)
    }

    /// 生成实用工具类
    pub fn generate_utility_css(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_utility_classes(self.current_theme)
    }

    /// 验证所有令牌引用
    pub fn validate_tokens(&mut self) -> Vec<TokenValidationError> {
        self.css_generator
            .get_resolver_mut()
            .validate_references(self.current_theme)
    }

    /// 列出所有令牌路径
    pub fn list_tokens(&self) -> Vec<String> {
        self.css_generator
            .get_resolver()
            .list_token_paths(self.current_theme)
            .into_iter()
            .map(|path| path.to_string())
            .collect()
    }

    /// 搜索令牌
    pub fn search_tokens(&self, query: &str) -> Vec<String> {
        self.list_tokens()
            .into_iter()
            .filter(|path| path.contains(query))
            .collect()
    }

    /// 获取令牌的CSS变量名
    pub fn get_css_var_name(&self, path: &str) -> String {
        let token_path = TokenPath::from_str(path);
        format!(
            "--{}-{}",
            self.config.css_prefix,
            token_path.segments.join("-")
        )
    }

    /// 创建主题变体
    pub fn create_theme_variant(
        &mut self,
        base_theme: ThemeVariant,
        new_theme: ThemeVariant,
        overrides: HashMap<String, TokenValue>,
    ) -> Result<(), String> {
        // 复制基础主题
        self.css_generator
            .get_resolver_mut()
            .get_store_mut()
            .copy_theme(base_theme, new_theme);

        // 应用覆盖值
        let old_theme = self.current_theme;
        self.current_theme = new_theme;

        for (path, value) in &overrides {
            self.set_token(path, value.clone())?;
        }

        self.current_theme = old_theme;
        Ok(())
    }

    /// 获取令牌的所有引用者
    pub fn get_token_references(&self, path: &str) -> Vec<String> {
        let token_path = TokenPath::from_str(path);
        self.css_generator
            .get_resolver()
            .find_references_to(&token_path, self.current_theme)
            .into_iter()
            .map(|path| path.to_string())
            .collect()
    }

    /// 计算令牌表达式
    pub fn compute_expression(
        &mut self,
        expression: &str,
    ) -> Result<TokenValue, TokenValidationError> {
        self.css_generator
            .get_resolver_mut()
            .compute_value(expression, self.current_theme)
    }

    /// 获取系统配置
    pub fn get_config(&self) -> &TokenSystemConfig {
        &self.config
    }

    /// 更新系统配置
    pub fn update_config(&mut self, config: TokenSystemConfig) {
        self.config = config;
        // 重新配置CSS生成器
        let store = std::mem::replace(
            self.css_generator.get_resolver_mut().get_store_mut(),
            DesignTokens::default(),
        );
        let resolver = TokenResolver::new(store);
        self.css_generator = CssGenerator::new(resolver)
            .with_prefix(self.config.css_prefix.clone())
            .with_minify(self.config.minify_css);
    }

    /// 获取支持的主题列表
    pub fn get_supported_themes(&self) -> Vec<ThemeVariant> {
        self.css_generator
            .get_resolver()
            .get_store()
            .get_supported_themes()
    }

    /// 清空指定主题的所有令牌
    pub fn clear_theme(&mut self, theme: ThemeVariant) {
        self.css_generator
            .get_resolver_mut()
            .get_store_mut()
            .clear_theme(theme);
    }

    /// 重置为默认令牌值
    pub fn reset_to_defaults(&mut self) {
        let store = DesignTokens::new();
        let resolver = TokenResolver::new(store);
        self.css_generator = CssGenerator::new(resolver)
            .with_prefix(self.config.css_prefix.clone())
            .with_minify(self.config.minify_css);
    }

    /// 更新系统元数据
    pub fn update_metadata(&mut self, metadata: SystemMetadata) {
        self.metadata = metadata;
    }

    /// 获取系统统计信息
    pub fn get_system_stats(&self) -> TokenSystemStats {
        let global_tokens = self.count_global_tokens();
        let alias_tokens = self.count_alias_tokens();
        let component_tokens = self.count_component_tokens();
        let total_tokens = global_tokens + alias_tokens + component_tokens;

        let supported_themes = self.get_supported_themes();
        let themes_count = supported_themes.len();

        // 估算CSS变量数量（每个令牌在每个主题中都有一个CSS变量）
        let css_variables_count = total_tokens * themes_count;

        let computation_rules_count = self.computation_rules.rules.len();

        TokenSystemStats {
            total_tokens,
            global_tokens,
            alias_tokens,
            component_tokens,
            themes_count,
            css_variables_count,
            computation_rules_count,
        }
    }

    // 私有辅助方法
    /// 更新语义令牌以适应当前主题
    fn update_semantic_tokens_for_theme(&mut self) {
        match self.current_theme {
            ThemeVariant::Light => {
                // 浅色主题的语义颜色映射
                self.alias_tokens.semantic_colors.text.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
                self.alias_tokens.semantic_colors.text.secondary =
                    TokenReference::new("global.color_palette.neutral.600".to_string());
                self.alias_tokens.semantic_colors.background.primary =
                    TokenReference::new("global.color_palette.neutral.50".to_string());
            }
            ThemeVariant::Dark => {
                // 深色主题的语义颜色映射
                self.alias_tokens.semantic_colors.text.primary =
                    TokenReference::new("global.color_palette.neutral.100".to_string());
                self.alias_tokens.semantic_colors.text.secondary =
                    TokenReference::new("global.color_palette.neutral.400".to_string());
                self.alias_tokens.semantic_colors.background.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
            }
            ThemeVariant::Auto => {
                // Auto模式使用浅色主题设置
                self.alias_tokens.semantic_colors.text.primary =
                    TokenReference::new("global.color_palette.neutral.900".to_string());
            }
        }
    }

    /// 应用浅色主题颜色
    /// 应用特定主题的颜色方案
    fn apply_light_theme_colors(&mut self) {
        // 设置浅色主题的主色调（使用通用蓝色方案）
        self.global_tokens
            .color_palette
            .primary
            .insert("50".to_string(), ColorValue::new("#e6f3ff".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("100".to_string(), ColorValue::new("#b3d9ff".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("200".to_string(), ColorValue::new("#80bfff".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("300".to_string(), ColorValue::new("#4da6ff".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("400".to_string(), ColorValue::new("#1a8cff".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("500".to_string(), ColorValue::new("#0066cc".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("600".to_string(), ColorValue::new("#0052a3".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("700".to_string(), ColorValue::new("#003d7a".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("800".to_string(), ColorValue::new("#002952".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("900".to_string(), ColorValue::new("#001429".to_string()));

        // 设置浅色主题的中性色（使用通用灰色方案）
        self.global_tokens
            .color_palette
            .neutral
            .insert("50".to_string(), ColorValue::new("#fafafa".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("100".to_string(), ColorValue::new("#f5f5f5".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("200".to_string(), ColorValue::new("#eeeeee".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("300".to_string(), ColorValue::new("#dddddd".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("400".to_string(), ColorValue::new("#bfbfbf".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("500".to_string(), ColorValue::new("#8c8c8c".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("600".to_string(), ColorValue::new("#595959".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("700".to_string(), ColorValue::new("#434343".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("800".to_string(), ColorValue::new("#262626".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("900".to_string(), ColorValue::new("#1f1f1f".to_string()));

        // 设置功能色（成功、警告、错误）
        self.global_tokens.color_palette.functional.insert(
            "success.50".to_string(),
            ColorValue::new("#f6ffed".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "success.500".to_string(),
            ColorValue::new("#52c41a".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "success.600".to_string(),
            ColorValue::new("#389e0d".to_string()),
        );

        self.global_tokens.color_palette.functional.insert(
            "warning.50".to_string(),
            ColorValue::new("#fffbe6".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "warning.500".to_string(),
            ColorValue::new("#faad14".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "warning.600".to_string(),
            ColorValue::new("#d48806".to_string()),
        );

        self.global_tokens.color_palette.functional.insert(
            "error.50".to_string(),
            ColorValue::new("#fff2f0".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "error.500".to_string(),
            ColorValue::new("#ff4d4f".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "error.600".to_string(),
            ColorValue::new("#cf1322".to_string()),
        );
    }

    /// 应用深色主题颜色
    /// 应用特定主题的颜色方案
    fn apply_dark_theme_colors(&mut self) {
        // 设置深色主题的主色调（相对浅色主题调整亮度）
        self.global_tokens
            .color_palette
            .primary
            .insert("50".to_string(), ColorValue::new("#111b26".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("100".to_string(), ColorValue::new("#112a41".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("200".to_string(), ColorValue::new("#15395b".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("300".to_string(), ColorValue::new("#164c7e".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("400".to_string(), ColorValue::new("#1765ad".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("500".to_string(), ColorValue::new("#177ddc".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("600".to_string(), ColorValue::new("#3c9ae8".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("700".to_string(), ColorValue::new("#65b7f3".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("800".to_string(), ColorValue::new("#8dcff8".to_string()));
        self.global_tokens
            .color_palette
            .primary
            .insert("900".to_string(), ColorValue::new("#b7e3fa".to_string()));

        // 设置深色主题的中性色（反转亮度）
        self.global_tokens
            .color_palette
            .neutral
            .insert("50".to_string(), ColorValue::new("#141414".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("100".to_string(), ColorValue::new("#1f1f1f".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("200".to_string(), ColorValue::new("#262626".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("300".to_string(), ColorValue::new("#434343".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("400".to_string(), ColorValue::new("#595959".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("500".to_string(), ColorValue::new("#8c8c8c".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("600".to_string(), ColorValue::new("#bfbfbf".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("700".to_string(), ColorValue::new("#dddddd".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("800".to_string(), ColorValue::new("#eeeeee".to_string()));
        self.global_tokens
            .color_palette
            .neutral
            .insert("900".to_string(), ColorValue::new("#ffffff".to_string()));

        // 设置深色主题的功能色
        self.global_tokens.color_palette.functional.insert(
            "success.50".to_string(),
            ColorValue::new("#162312".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "success.500".to_string(),
            ColorValue::new("#49aa19".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "success.600".to_string(),
            ColorValue::new("#6abe39".to_string()),
        );

        self.global_tokens.color_palette.functional.insert(
            "warning.50".to_string(),
            ColorValue::new("#2b2111".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "warning.500".to_string(),
            ColorValue::new("#d89614".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "warning.600".to_string(),
            ColorValue::new("#e8b339".to_string()),
        );

        self.global_tokens.color_palette.functional.insert(
            "error.50".to_string(),
            ColorValue::new("#2a1215".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "error.500".to_string(),
            ColorValue::new("#dc4446".to_string()),
        );
        self.global_tokens.color_palette.functional.insert(
            "error.600".to_string(),
            ColorValue::new("#e84749".to_string()),
        );
    }

    /// 统计全局令牌数量
    /// 计算全局令牌的总数量
    fn count_global_tokens(&self) -> usize {
        let mut count = 0;

        // 统计颜色调色板
        let color_palette = &self.global_tokens.color_palette;

        count += &color_palette.primary.len();
        count += &color_palette.neutral.len();
        count += &color_palette.functional.len();
        count += &color_palette.extended.len();
        // if let success = &color_palette.success {
        //     count += success.len();
        // }
        // if let warning = &color_palette.warning {
        //     count += warning.len();
        // }
        // if let error = &color_palette.error {
        //     count += error.len();
        // }

        // 统计字体系统
        let font_system = &self.global_tokens.font_system;
        count += &font_system.sizes.len();
        count += &font_system.weights.len();
        count += &font_system.line_heights.len();
        count += &font_system.families.len();
        count += &font_system.letter_spacings.len();

        // 统计间距系统
        let spacing_system = &self.global_tokens.spacing_system;
        count += &spacing_system.values.len();

        count
    }

    /// 统计别名令牌数量
    /// 计算别名令牌的总数量
    fn count_alias_tokens(&self) -> usize {
        0
        // let mut count = 0;

        // // 统计语义颜色
        // if let semantic_colors = &self.alias_tokens.semantic_colors {
        //     if let text = &semantic_colors.text {
        //         count += text.len();
        //     }
        //     if let background = &semantic_colors.background {
        //         count += background.len();
        //     }
        //     if let border = &semantic_colors.border {
        //         count += border.len();
        //     }
        // }

        // // 统计语义排版
        // if let semantic_typography = &self.alias_tokens.semantic_typography {
        //     if let headings = &semantic_typography.heading {
        //         count += headings.len();
        //     }
        //     if let Some(body) = &semantic_typography.body {
        //         count += body.len();
        //     }
        // }

        // count
    }

    /// 统计组件令牌数量
    /// 计算组件令牌的总数量
    fn count_component_tokens(&self) -> usize {
        let mut count = 0;

        // 统计按钮令牌（按钮有多个变体，每个变体有多个状态）
        count += 5; // default, primary, dashed, text, link 变体

        // 统计输入框令牌
        count += 1; // input

        // 统计卡片令牌
        count += 1; // card

        // 统计表格令牌
        count += 1; // table

        // 统计导航令牌
        count += 1; // navigation

        count
    }
}

impl Default for DesignTokenSystem {
    fn default() -> Self {
        Self::new()
    }
}

// 为各个结构体实现Default trait

impl Default for GlobalTokens {
    fn default() -> Self {
        Self {
            color_palette: ColorPalette::default(),
            font_system: FontSystem::default(),
            spacing_system: SpacingSystem::default(),
            sizing_system: SizingSystem::default(),
            border_system: BorderSystem::default(),
            shadow_system: ShadowSystem::default(),
            motion_system: MotionSystem::default(),
        }
    }
}

impl Default for AliasTokens {
    fn default() -> Self {
        Self {
            semantic_colors: SemanticColors::default(),
            semantic_typography: SemanticTypography::default(),
            semantic_spacing: SemanticSpacing::default(),
            semantic_sizing: SemanticSizing::default(),
        }
    }
}

impl Default for ComponentTokens {
    fn default() -> Self {
        Self {
            components: BTreeMap::new(),
        }
    }
}

impl Default for ComputationRules {
    fn default() -> Self {
        Self {
            rules: BTreeMap::new(),
        }
    }
}

impl Default for SystemMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            author: "Design System".to_string(),
            description: "CSS-in-Rust Token System".to_string(),
        }
    }
}

// 为各个子系统实现Default trait

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: BTreeMap::new(),
            neutral: BTreeMap::new(),
            functional: BTreeMap::new(),
            extended: BTreeMap::new(),
        }
    }
}

impl Default for FontSystem {
    fn default() -> Self {
        Self {
            families: BTreeMap::new(),
            sizes: BTreeMap::new(),
            weights: BTreeMap::new(),
            line_heights: BTreeMap::new(),
            letter_spacings: BTreeMap::new(),
        }
    }
}

impl Default for SpacingSystem {
    fn default() -> Self {
        use super::token_definitions::DimensionUnit;
        Self {
            base_unit: DimensionValue::new(8.0, DimensionUnit::Px),
            scale: vec![0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0, 8.0],
            values: BTreeMap::new(),
        }
    }
}

impl Default for SizingSystem {
    fn default() -> Self {
        Self {
            base_sizes: BTreeMap::new(),
            component_sizes: BTreeMap::new(),
            breakpoints: BTreeMap::new(),
        }
    }
}

impl Default for BorderSystem {
    fn default() -> Self {
        Self {
            widths: BTreeMap::new(),
            styles: BTreeMap::new(),
            radius: BTreeMap::new(),
        }
    }
}

impl Default for ShadowSystem {
    fn default() -> Self {
        Self {
            elevations: BTreeMap::new(),
            colors: BTreeMap::new(),
        }
    }
}

impl Default for MotionSystem {
    fn default() -> Self {
        Self {
            durations: BTreeMap::new(),
            easings: BTreeMap::new(),
            delays: BTreeMap::new(),
        }
    }
}

// 为语义令牌实现Default trait

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            text: TextSemanticColors::default(),
            background: BackgroundSemanticColors::default(),
            border: BorderSemanticColors::default(),
            state: StateSemanticColors::default(),
        }
    }
}

impl Default for TextSemanticColors {
    fn default() -> Self {
        Self {
            primary: TokenReference::new("color.neutral.900".to_string()),
            secondary: TokenReference::new("color.neutral.600".to_string()),
            disabled: TokenReference::new("color.neutral.400".to_string()),
            inverse: TokenReference::new("color.neutral.50".to_string()),
        }
    }
}

impl Default for BackgroundSemanticColors {
    fn default() -> Self {
        Self {
            primary: TokenReference::new("color.neutral.50".to_string()),
            secondary: TokenReference::new("color.neutral.100".to_string()),
            emphasis: TokenReference::new("color.primary.500".to_string()),
            inverse: TokenReference::new("color.neutral.900".to_string()),
        }
    }
}

impl Default for BorderSemanticColors {
    fn default() -> Self {
        Self {
            default: TokenReference::new("color.neutral.200".to_string()),
            emphasis: TokenReference::new("color.neutral.400".to_string()),
            disabled: TokenReference::new("color.neutral.100".to_string()),
        }
    }
}

impl Default for StateSemanticColors {
    fn default() -> Self {
        Self {
            success: TokenReference::new("color.functional.success".to_string()),
            warning: TokenReference::new("color.functional.warning".to_string()),
            error: TokenReference::new("color.functional.error".to_string()),
            info: TokenReference::new("color.functional.info".to_string()),
        }
    }
}

impl Default for SemanticTypography {
    fn default() -> Self {
        Self {
            heading: HeadingTypography::default(),
            body: BodyTypography::default(),
            caption: CaptionTypography::default(),
            code: CodeTypography::default(),
        }
    }
}

impl Default for HeadingTypography {
    fn default() -> Self {
        use super::token_definitions::DimensionUnit;
        Self {
            h1: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(32.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.25),
                letter_spacing: Some(DimensionValue::new(-0.5, DimensionUnit::Px)),
            },
            h2: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(24.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.33),
                letter_spacing: Some(DimensionValue::new(-0.25, DimensionUnit::Px)),
            },
            h3: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(20.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.4),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            h4: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(18.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.44),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            h5: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(16.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.5),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            h6: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(14.0, DimensionUnit::Px)),
                font_weight: Some(600),
                line_height: Some(1.57),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
        }
    }
}

impl Default for BodyTypography {
    fn default() -> Self {
        use super::token_definitions::DimensionUnit;
        Self {
            large: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(16.0, DimensionUnit::Px)),
                font_weight: Some(400),
                line_height: Some(1.5),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            medium: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(14.0, DimensionUnit::Px)),
                font_weight: Some(400),
                line_height: Some(1.57),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            small: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(12.0, DimensionUnit::Px)),
                font_weight: Some(400),
                line_height: Some(1.67),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
        }
    }
}

impl Default for CaptionTypography {
    fn default() -> Self {
        use super::token_definitions::DimensionUnit;
        Self {
            large: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(12.0, DimensionUnit::Px)),
                font_weight: Some(500),
                line_height: Some(1.67),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            medium: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(11.0, DimensionUnit::Px)),
                font_weight: Some(500),
                line_height: Some(1.73),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            small: TypographyValue {
                font_family: Some("Inter".to_string()),
                font_size: Some(DimensionValue::new(10.0, DimensionUnit::Px)),
                font_weight: Some(500),
                line_height: Some(1.8),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
        }
    }
}

impl Default for CodeTypography {
    fn default() -> Self {
        use super::token_definitions::DimensionUnit;
        Self {
            inline: TypographyValue {
                font_family: Some("JetBrains Mono".to_string()),
                font_size: Some(DimensionValue::new(14.0, DimensionUnit::Px)),
                font_weight: Some(400),
                line_height: Some(1.57),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
            block: TypographyValue {
                font_family: Some("JetBrains Mono".to_string()),
                font_size: Some(DimensionValue::new(14.0, DimensionUnit::Px)),
                font_weight: Some(400),
                line_height: Some(1.57),
                letter_spacing: Some(DimensionValue::new(0.0, DimensionUnit::Px)),
            },
        }
    }
}

impl Default for SemanticSpacing {
    fn default() -> Self {
        Self {
            component: ComponentSpacing::default(),
            padding: PaddingSpacing::default(),
            margin: MarginSpacing::default(),
            gap: GapSpacing::default(),
            layout: LayoutSpacing::default(),
        }
    }
}

impl Default for ComponentSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("spacing.2".to_string()),
            sm: TokenReference::new("spacing.4".to_string()),
            md: TokenReference::new("spacing.8".to_string()),
            lg: TokenReference::new("spacing.12".to_string()),
            xl: TokenReference::new("spacing.16".to_string()),
        }
    }
}

impl Default for PaddingSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("spacing.2".to_string()),
            sm: TokenReference::new("spacing.4".to_string()),
            md: TokenReference::new("spacing.8".to_string()),
            lg: TokenReference::new("spacing.12".to_string()),
            xl: TokenReference::new("spacing.16".to_string()),
        }
    }
}

impl Default for MarginSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("spacing.2".to_string()),
            sm: TokenReference::new("spacing.4".to_string()),
            md: TokenReference::new("spacing.8".to_string()),
            lg: TokenReference::new("spacing.12".to_string()),
            xl: TokenReference::new("spacing.16".to_string()),
        }
    }
}

impl Default for GapSpacing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("spacing.2".to_string()),
            sm: TokenReference::new("spacing.4".to_string()),
            md: TokenReference::new("spacing.8".to_string()),
            lg: TokenReference::new("spacing.12".to_string()),
            xl: TokenReference::new("spacing.16".to_string()),
        }
    }
}

impl Default for LayoutSpacing {
    fn default() -> Self {
        Self {
            container: TokenReference::new("spacing.24".to_string()),
            section: TokenReference::new("spacing.32".to_string()),
            content: TokenReference::new("spacing.16".to_string()),
        }
    }
}

impl Default for SemanticSizing {
    fn default() -> Self {
        Self {
            component: ComponentSizing::default(),
            icon: IconSizing::default(),
            avatar: AvatarSizing::default(),
        }
    }
}

impl Default for ComponentSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("sizing.xs".to_string()),
            sm: TokenReference::new("sizing.sm".to_string()),
            md: TokenReference::new("sizing.md".to_string()),
            lg: TokenReference::new("sizing.lg".to_string()),
            xl: TokenReference::new("sizing.xl".to_string()),
        }
    }
}

impl Default for IconSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("sizing.icon.xs".to_string()),
            sm: TokenReference::new("sizing.icon.sm".to_string()),
            md: TokenReference::new("sizing.icon.md".to_string()),
            lg: TokenReference::new("sizing.icon.lg".to_string()),
            xl: TokenReference::new("sizing.icon.xl".to_string()),
        }
    }
}

impl Default for AvatarSizing {
    fn default() -> Self {
        Self {
            xs: TokenReference::new("sizing.avatar.xs".to_string()),
            sm: TokenReference::new("sizing.avatar.sm".to_string()),
            md: TokenReference::new("sizing.avatar.md".to_string()),
            lg: TokenReference::new("sizing.avatar.lg".to_string()),
            xl: TokenReference::new("sizing.avatar.xl".to_string()),
        }
    }
}

// 为组件令牌实现Default trait

/// 全局令牌系统实例
static mut GLOBAL_TOKEN_SYSTEM: Option<DesignTokenSystem> = None;
static mut GLOBAL_TOKEN_SYSTEM_INITIALIZED: bool = false;

/// 获取全局令牌系统实例
pub fn get_global_token_system() -> &'static mut DesignTokenSystem {
    unsafe {
        if !GLOBAL_TOKEN_SYSTEM_INITIALIZED {
            GLOBAL_TOKEN_SYSTEM = Some(DesignTokenSystem::new());
            GLOBAL_TOKEN_SYSTEM_INITIALIZED = true;
        }
        GLOBAL_TOKEN_SYSTEM.as_mut().unwrap()
    }
}

/// 初始化全局令牌系统
pub fn init_global_token_system(config: TokenSystemConfig) {
    unsafe {
        GLOBAL_TOKEN_SYSTEM = Some(DesignTokenSystem::with_config(config));
        GLOBAL_TOKEN_SYSTEM_INITIALIZED = true;
    }
}

/// 便捷宏：获取令牌值
#[macro_export]
macro_rules! token {
    ($path:expr) => {
        $crate::theme::get_global_token_system().get_token($path)
    };
}

/// 便捷宏：获取CSS变量名
#[macro_export]
macro_rules! css_var {
    ($path:expr) => {
        $crate::theme::get_global_token_system().get_css_var_name($path)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_token_system_creation() {
        let system = DesignTokenSystem::new();
        assert_eq!(system.get_current_theme(), ThemeVariant::Light);
    }

    #[test]
    fn test_token_operations() {
        let mut system = DesignTokenSystem::new();

        // 测试获取令牌
        let result = system.get_token("color.primary.500");
        assert!(result.is_ok());

        // 测试设置令牌
        let set_result =
            system.set_token("test.token", TokenValue::String("test-value".to_string()));
        assert!(set_result.is_ok());

        // 测试获取设置的令牌
        let get_result = system.get_token("test.token");
        assert!(get_result.is_ok());
    }

    #[test]
    fn test_theme_switching() {
        let mut system = DesignTokenSystem::new();

        assert_eq!(system.get_current_theme(), ThemeVariant::Light);

        system.switch_theme(ThemeVariant::Dark);
        assert_eq!(system.get_current_theme(), ThemeVariant::Dark);
    }

    #[test]
    fn test_css_generation() {
        let mut system = DesignTokenSystem::new();

        let css_vars = system.generate_css_variables();
        assert!(css_vars.is_ok());

        let theme_css = system.generate_theme_css();
        assert!(theme_css.is_ok());
    }

    #[test]
    fn test_token_validation() {
        let mut system = DesignTokenSystem::new();

        let errors = system.validate_tokens();
        // 默认配置应该没有验证错误
        assert!(errors.is_empty());
    }

    #[test]
    fn test_global_token_system() {
        let system = get_global_token_system();
        assert_eq!(system.get_current_theme(), ThemeVariant::Light);
    }
}
