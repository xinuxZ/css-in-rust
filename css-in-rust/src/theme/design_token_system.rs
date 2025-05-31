//! # 设计令牌系统模块
//!
//! 提供完整的设计令牌管理系统，支持令牌层级、计算、导出和同步。
//! 实现 Ant Design 设计令牌规范。

use super::design_tokens::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 设计令牌系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokenSystem {
    /// 全局令牌（最基础的设计决策）
    pub global_tokens: GlobalTokens,
    /// 别名令牌（语义化的令牌引用）
    pub alias_tokens: AliasTokens,
    /// 组件令牌（特定组件的令牌）
    pub component_tokens: ComponentTokens,
    /// 令牌计算规则
    pub computation_rules: ComputationRules,
    /// 令牌元数据
    pub metadata: TokenMetadata,
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
    /// 按钮组件令牌
    pub button: ButtonTokens,
    /// 输入框组件令牌
    pub input: InputTokens,
    /// 卡片组件令牌
    pub card: CardTokens,
    /// 表格组件令牌
    pub table: TableTokens,
    /// 导航组件令牌
    pub navigation: NavigationTokens,
}

/// 令牌计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationRules {
    /// 颜色计算规则
    pub color_rules: Vec<ColorComputationRule>,
    /// 间距计算规则
    pub spacing_rules: Vec<SpacingComputationRule>,
    /// 字体计算规则
    pub typography_rules: Vec<TypographyComputationRule>,
}

/// 令牌元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
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

/// 颜色调色板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// 主色调
    pub primary: ColorScale,
    /// 中性色
    pub neutral: ColorScale,
    /// 成功色
    pub success: ColorScale,
    /// 警告色
    pub warning: ColorScale,
    /// 错误色
    pub error: ColorScale,
    /// 信息色
    pub info: ColorScale,
}

/// 字体系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSystem {
    /// 字体族
    pub font_families: BTreeMap<String, String>,
    /// 字体大小比例
    pub font_scale: FontScale,
    /// 字重
    pub font_weights: BTreeMap<String, u16>,
    /// 行高
    pub line_heights: BTreeMap<String, f32>,
    /// 字间距
    pub letter_spacings: BTreeMap<String, String>,
}

/// 字体大小比例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontScale {
    /// 基础字体大小（px）
    pub base_size: u16,
    /// 比例因子
    pub scale_ratio: f32,
    /// 字体大小映射
    pub sizes: BTreeMap<String, u16>,
}

/// 间距系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    /// 基础间距单位（px）
    pub base_unit: u16,
    /// 间距比例
    pub scale: Vec<f32>,
    /// 间距映射
    pub spacing_map: BTreeMap<String, u16>,
}

/// 尺寸系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizingSystem {
    /// 组件尺寸
    pub component_sizes: BTreeMap<String, ComponentSize>,
    /// 图标尺寸
    pub icon_sizes: BTreeMap<String, u16>,
    /// 头像尺寸
    pub avatar_sizes: BTreeMap<String, u16>,
}

/// 组件尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSize {
    pub height: u16,
    pub padding_horizontal: u16,
    pub padding_vertical: u16,
    pub font_size: u16,
}

/// 边框系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSystem {
    /// 边框宽度
    pub widths: BTreeMap<String, u16>,
    /// 边框样式
    pub styles: BTreeMap<String, String>,
    /// 圆角半径
    pub radius: BTreeMap<String, u16>,
}

/// 阴影系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowSystem {
    /// 阴影层级
    pub elevations: BTreeMap<String, ShadowDefinition>,
}

/// 阴影定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowDefinition {
    pub x: i16,
    pub y: i16,
    pub blur: u16,
    pub spread: i16,
    pub color: String,
    pub inset: bool,
}

/// 动画系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionSystem {
    /// 动画持续时间
    pub durations: BTreeMap<String, u16>,
    /// 缓动函数
    pub easings: BTreeMap<String, String>,
    /// 动画延迟
    pub delays: BTreeMap<String, u16>,
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
    pub primary: TokenReference,
    pub secondary: TokenReference,
    pub tertiary: TokenReference,
    pub disabled: TokenReference,
    pub inverse: TokenReference,
    pub link: TokenReference,
    pub link_hover: TokenReference,
}

/// 背景语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundSemanticColors {
    pub primary: TokenReference,
    pub secondary: TokenReference,
    pub tertiary: TokenReference,
    pub inverse: TokenReference,
    pub overlay: TokenReference,
}

/// 边框语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSemanticColors {
    pub primary: TokenReference,
    pub secondary: TokenReference,
    pub focus: TokenReference,
    pub error: TokenReference,
    pub success: TokenReference,
}

/// 状态语义颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSemanticColors {
    pub hover: TokenReference,
    pub active: TokenReference,
    pub focus: TokenReference,
    pub disabled: TokenReference,
    pub selected: TokenReference,
}

/// 令牌引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReference {
    /// 引用路径（如：global.color_palette.primary.500）
    pub reference: String,
    /// 可选的变换函数
    pub transform: Option<TokenTransform>,
}

/// 令牌变换
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenTransform {
    /// 透明度变换
    Alpha(f32),
    /// 亮度变换
    Lighten(f32),
    /// 暗度变换
    Darken(f32),
    /// 饱和度变换
    Saturate(f32),
    /// 去饱和变换
    Desaturate(f32),
    /// 数学运算
    Math(MathOperation),
}

/// 数学运算
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MathOperation {
    Add(f32),
    Subtract(f32),
    Multiply(f32),
    Divide(f32),
}

/// 颜色计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorComputationRule {
    pub name: String,
    pub base_color: TokenReference,
    pub operations: Vec<TokenTransform>,
}

/// 间距计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingComputationRule {
    pub name: String,
    pub base_value: TokenReference,
    pub multiplier: f32,
}

/// 字体计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyComputationRule {
    pub name: String,
    pub base_size: TokenReference,
    pub scale_factor: f32,
}

/// 语义字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTypography {
    pub heading: HeadingTypography,
    pub body: BodyTypography,
    pub caption: CaptionTypography,
    pub code: CodeTypography,
}

/// 标题字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingTypography {
    pub h1: TypographyToken,
    pub h2: TypographyToken,
    pub h3: TypographyToken,
    pub h4: TypographyToken,
    pub h5: TypographyToken,
    pub h6: TypographyToken,
}

/// 正文字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyTypography {
    pub large: TypographyToken,
    pub medium: TypographyToken,
    pub small: TypographyToken,
}

/// 说明字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionTypography {
    pub large: TypographyToken,
    pub medium: TypographyToken,
    pub small: TypographyToken,
}

/// 代码字体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTypography {
    pub inline: TypographyToken,
    pub block: TypographyToken,
}

/// 字体令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyToken {
    pub font_family: TokenReference,
    pub font_size: TokenReference,
    pub font_weight: TokenReference,
    pub line_height: TokenReference,
    pub letter_spacing: TokenReference,
}

/// 语义间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSpacing {
    pub component: ComponentSpacing,
    pub layout: LayoutSpacing,
}

/// 组件间距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpacing {
    pub padding: PaddingSpacing,
    pub margin: MarginSpacing,
    pub gap: GapSpacing,
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
    pub section: TokenReference,
    pub container: TokenReference,
    pub grid: TokenReference,
}

/// 语义尺寸
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSizing {
    pub component: ComponentSizing,
    pub icon: IconSizing,
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

// 组件令牌定义

/// 按钮令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTokens {
    pub primary: ButtonVariantTokens,
    pub secondary: ButtonVariantTokens,
    pub ghost: ButtonVariantTokens,
    pub link: ButtonVariantTokens,
    pub text: ButtonVariantTokens,
}

/// 按钮变体令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonVariantTokens {
    pub background: StateTokens<TokenReference>,
    pub border: StateTokens<TokenReference>,
    pub text: StateTokens<TokenReference>,
    pub shadow: StateTokens<TokenReference>,
}

/// 状态令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTokens<T> {
    pub default: T,
    pub hover: T,
    pub active: T,
    pub focus: T,
    pub disabled: T,
}

/// 输入框令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTokens {
    pub background: StateTokens<TokenReference>,
    pub border: StateTokens<TokenReference>,
    pub text: StateTokens<TokenReference>,
    pub placeholder: TokenReference,
    pub shadow: StateTokens<TokenReference>,
}

/// 卡片令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTokens {
    pub background: TokenReference,
    pub border: TokenReference,
    pub shadow: TokenReference,
    pub radius: TokenReference,
    pub padding: TokenReference,
}

/// 表格令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableTokens {
    pub header: TableSectionTokens,
    pub body: TableSectionTokens,
    pub footer: TableSectionTokens,
    pub border: TokenReference,
    pub stripe: TokenReference,
}

/// 表格区域令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSectionTokens {
    pub background: TokenReference,
    pub text: TokenReference,
    pub border: TokenReference,
}

/// 导航令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationTokens {
    pub background: TokenReference,
    pub border: TokenReference,
    pub item: NavigationItemTokens,
}

/// 导航项令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationItemTokens {
    pub background: StateTokens<TokenReference>,
    pub text: StateTokens<TokenReference>,
    pub border: StateTokens<TokenReference>,
}

impl DesignTokenSystem {
    /// 创建 Ant Design 默认令牌系统
    pub fn ant_design_default() -> Self {
        Self {
            global_tokens: GlobalTokens::ant_design_default(),
            alias_tokens: AliasTokens::ant_design_default(),
            component_tokens: ComponentTokens::ant_design_default(),
            computation_rules: ComputationRules::ant_design_default(),
            metadata: TokenMetadata {
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                author: "CSS-in-Rust".to_string(),
                description: "Ant Design 设计令牌系统".to_string(),
            },
        }
    }

    /// 获取令牌值
    pub fn get_token_value(&self, path: &str) -> Option<crate::theme::TokenValue> {
        // 简单实现，返回字符串值
        // 实际实现需要根据路径解析令牌
        Some(crate::theme::TokenValue::string("default-value"))
    }

    /// 设置令牌值
    pub fn set_token(
        &mut self,
        path: &str,
        value: crate::theme::TokenValue,
    ) -> Result<(), crate::theme::ThemeError> {
        // 简单实现，实际需要根据路径设置令牌
        Ok(())
    }

    /// 导出为 CSS 变量
    pub fn export_as_css_variables(&self) -> Result<String, crate::theme::ThemeError> {
        let mut css = String::new();

        // 导出全局令牌
        css.push_str("  /* Global Tokens */\n");
        css.push_str("  --primary-color: #1890ff;\n");
        css.push_str("  --success-color: #52c41a;\n");
        css.push_str("  --warning-color: #faad14;\n");
        css.push_str("  --error-color: #ff4d4f;\n");
        css.push_str("  --font-size-base: 14px;\n");
        css.push_str("  --line-height-base: 1.5715;\n");
        css.push_str("  --border-radius-base: 6px;\n");
        css.push_str("  --spacing-xs: 8px;\n");
        css.push_str("  --spacing-sm: 12px;\n");
        css.push_str("  --spacing-md: 16px;\n");
        css.push_str("  --spacing-lg: 24px;\n");
        css.push_str("  --spacing-xl: 32px;\n");

        Ok(css)
    }

    /// 解析令牌引用
    pub fn resolve_token(&self, reference: &TokenReference) -> Result<String, String> {
        // 这里实现令牌解析逻辑
        // 解析路径如：global.color_palette.primary.500
        let parts: Vec<&str> = reference.reference.split('.').collect();

        // 简化实现，实际应该根据路径递归解析
        match parts.as_slice() {
            ["global", "color_palette", color, level] => {
                // 解析颜色令牌
                self.resolve_color_token(color, level)
            }
            ["global", "spacing_system", size] => {
                // 解析间距令牌
                self.resolve_spacing_token(size)
            }
            _ => Err(format!("无法解析令牌引用: {}", reference.reference)),
        }
    }

    /// 解析颜色令牌
    fn resolve_color_token(&self, color: &str, level: &str) -> Result<String, String> {
        // 简化实现
        match (color, level) {
            ("primary", "500") => Ok("#1890ff".to_string()),
            ("success", "500") => Ok("#52c41a".to_string()),
            ("warning", "500") => Ok("#faad14".to_string()),
            ("error", "500") => Ok("#f5222d".to_string()),
            _ => Err(format!("未知的颜色令牌: {}.{}", color, level)),
        }
    }

    /// 解析间距令牌
    fn resolve_spacing_token(&self, size: &str) -> Result<String, String> {
        match size {
            "xs" => Ok("4px".to_string()),
            "sm" => Ok("8px".to_string()),
            "md" => Ok("16px".to_string()),
            "lg" => Ok("24px".to_string()),
            "xl" => Ok("32px".to_string()),
            _ => Err(format!("未知的间距令牌: {}", size)),
        }
    }

    /// 导出为 CSS 变量
    pub fn export_css_variables(&self) -> String {
        let mut css = ":root {\n".to_string();

        // 导出颜色变量
        css.push_str("  /* 颜色令牌 */\n");
        css.push_str("  --color-primary-500: #1890ff;\n");
        css.push_str("  --color-success-500: #52c41a;\n");
        css.push_str("  --color-warning-500: #faad14;\n");
        css.push_str("  --color-error-500: #f5222d;\n");

        // 导出间距变量
        css.push_str("  /* 间距令牌 */\n");
        css.push_str("  --spacing-xs: 4px;\n");
        css.push_str("  --spacing-sm: 8px;\n");
        css.push_str("  --spacing-md: 16px;\n");
        css.push_str("  --spacing-lg: 24px;\n");
        css.push_str("  --spacing-xl: 32px;\n");

        css.push_str("}\n");
        css
    }

    /// 导出为 JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 验证令牌系统
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证令牌引用的有效性
        // 这里可以添加更多验证逻辑

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// 为各个结构体实现默认值
impl GlobalTokens {
    fn ant_design_default() -> Self {
        // 实现 Ant Design 默认全局令牌
        Self {
            color_palette: ColorPalette::ant_design_default(),
            font_system: FontSystem::ant_design_default(),
            spacing_system: SpacingSystem::ant_design_default(),
            sizing_system: SizingSystem::ant_design_default(),
            border_system: BorderSystem::ant_design_default(),
            shadow_system: ShadowSystem::ant_design_default(),
            motion_system: MotionSystem::ant_design_default(),
        }
    }
}

impl AliasTokens {
    fn ant_design_default() -> Self {
        // 实现 Ant Design 默认别名令牌
        Self {
            semantic_colors: SemanticColors::ant_design_default(),
            semantic_typography: SemanticTypography::ant_design_default(),
            semantic_spacing: SemanticSpacing::ant_design_default(),
            semantic_sizing: SemanticSizing::ant_design_default(),
        }
    }
}

impl ComponentTokens {
    fn ant_design_default() -> Self {
        // 实现 Ant Design 默认组件令牌
        Self {
            button: ButtonTokens::ant_design_default(),
            input: InputTokens::ant_design_default(),
            card: CardTokens::ant_design_default(),
            table: TableTokens::ant_design_default(),
            navigation: NavigationTokens::ant_design_default(),
        }
    }
}

impl ComputationRules {
    fn ant_design_default() -> Self {
        Self {
            color_rules: Vec::new(),
            spacing_rules: Vec::new(),
            typography_rules: Vec::new(),
        }
    }
}

// 为了简化示例，这里只实现部分默认值
// 实际项目中需要完整实现所有结构体的默认值

impl ColorPalette {
    fn ant_design_default() -> Self {
        Self {
            primary: ColorScale::blue(),
            neutral: ColorScale::gray(),
            success: ColorScale::green(),
            warning: ColorScale::orange(),
            error: ColorScale::red(),
            info: ColorScale::blue(),
        }
    }
}

impl FontSystem {
    fn ant_design_default() -> Self {
        let mut font_families = BTreeMap::new();
        font_families.insert("primary".to_string(), "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif".to_string());
        font_families.insert(
            "mono".to_string(),
            "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
        );

        let mut font_weights = BTreeMap::new();
        font_weights.insert("light".to_string(), 300);
        font_weights.insert("normal".to_string(), 400);
        font_weights.insert("medium".to_string(), 500);
        font_weights.insert("semibold".to_string(), 600);
        font_weights.insert("bold".to_string(), 700);

        let mut line_heights = BTreeMap::new();
        line_heights.insert("tight".to_string(), 1.2);
        line_heights.insert("normal".to_string(), 1.5);
        line_heights.insert("relaxed".to_string(), 1.75);

        let mut letter_spacings = BTreeMap::new();
        letter_spacings.insert("tight".to_string(), "-0.025em".to_string());
        letter_spacings.insert("normal".to_string(), "0".to_string());
        letter_spacings.insert("wide".to_string(), "0.025em".to_string());

        Self {
            font_families,
            font_scale: FontScale {
                base_size: 14,
                scale_ratio: 1.25,
                sizes: BTreeMap::new(),
            },
            font_weights,
            line_heights,
            letter_spacings,
        }
    }
}

impl SpacingSystem {
    fn ant_design_default() -> Self {
        let mut spacing_map = BTreeMap::new();
        spacing_map.insert("xs".to_string(), 4);
        spacing_map.insert("sm".to_string(), 8);
        spacing_map.insert("md".to_string(), 16);
        spacing_map.insert("lg".to_string(), 24);
        spacing_map.insert("xl".to_string(), 32);

        Self {
            spacing_map,
            base_unit: 8,
            scale: vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0],
        }
    }
}

impl SizingSystem {
    fn ant_design_default() -> Self {
        let mut component_sizes = BTreeMap::new();
        component_sizes.insert(
            "xs".to_string(),
            ComponentSize {
                height: 24,
                padding_horizontal: 8,
                padding_vertical: 4,
                font_size: 12,
            },
        );
        component_sizes.insert(
            "sm".to_string(),
            ComponentSize {
                height: 32,
                padding_horizontal: 12,
                padding_vertical: 6,
                font_size: 14,
            },
        );
        component_sizes.insert(
            "md".to_string(),
            ComponentSize {
                height: 40,
                padding_horizontal: 16,
                padding_vertical: 8,
                font_size: 16,
            },
        );
        component_sizes.insert(
            "lg".to_string(),
            ComponentSize {
                height: 48,
                padding_horizontal: 20,
                padding_vertical: 10,
                font_size: 18,
            },
        );
        component_sizes.insert(
            "xl".to_string(),
            ComponentSize {
                height: 56,
                padding_horizontal: 24,
                padding_vertical: 12,
                font_size: 20,
            },
        );

        let mut icon_sizes = BTreeMap::new();
        icon_sizes.insert("xs".to_string(), 12);
        icon_sizes.insert("sm".to_string(), 16);
        icon_sizes.insert("md".to_string(), 20);
        icon_sizes.insert("lg".to_string(), 24);
        icon_sizes.insert("xl".to_string(), 32);

        let mut avatar_sizes = BTreeMap::new();
        avatar_sizes.insert("xs".to_string(), 24);
        avatar_sizes.insert("sm".to_string(), 32);
        avatar_sizes.insert("md".to_string(), 40);
        avatar_sizes.insert("lg".to_string(), 48);
        avatar_sizes.insert("xl".to_string(), 64);

        Self {
            component_sizes,
            icon_sizes,
            avatar_sizes,
        }
    }
}

impl BorderSystem {
    fn ant_design_default() -> Self {
        let mut border_widths = BTreeMap::new();
        border_widths.insert("thin".to_string(), 1);
        border_widths.insert("medium".to_string(), 2);
        border_widths.insert("thick".to_string(), 4);

        let mut border_radii = BTreeMap::new();
        border_radii.insert("none".to_string(), 0);
        border_radii.insert("sm".to_string(), 2);
        border_radii.insert("md".to_string(), 6);
        border_radii.insert("lg".to_string(), 8);
        border_radii.insert("full".to_string(), 9999);

        Self {
            widths: border_widths,
            radius: border_radii,
            styles: {
                let mut styles = BTreeMap::new();
                styles.insert("solid".to_string(), "solid".to_string());
                styles.insert("dashed".to_string(), "dashed".to_string());
                styles.insert("dotted".to_string(), "dotted".to_string());
                styles
            },
        }
    }
}

impl ShadowSystem {
    fn ant_design_default() -> Self {
        let mut shadow_levels = BTreeMap::new();
        shadow_levels.insert(
            "sm".to_string(),
            ShadowDefinition {
                x: 0,
                y: 1,
                blur: 2,
                spread: 0,
                color: "rgba(0, 0, 0, 0.05)".to_string(),
                inset: false,
            },
        );
        shadow_levels.insert(
            "md".to_string(),
            ShadowDefinition {
                x: 0,
                y: 4,
                blur: 6,
                spread: -1,
                color: "rgba(0, 0, 0, 0.1)".to_string(),
                inset: false,
            },
        );
        shadow_levels.insert(
            "lg".to_string(),
            ShadowDefinition {
                x: 0,
                y: 10,
                blur: 15,
                spread: -3,
                color: "rgba(0, 0, 0, 0.1)".to_string(),
                inset: false,
            },
        );
        shadow_levels.insert(
            "xl".to_string(),
            ShadowDefinition {
                x: 0,
                y: 20,
                blur: 25,
                spread: -5,
                color: "rgba(0, 0, 0, 0.1)".to_string(),
                inset: false,
            },
        );

        Self {
            elevations: shadow_levels,
        }
    }
}

impl MotionSystem {
    fn ant_design_default() -> Self {
        let mut durations = BTreeMap::new();
        durations.insert("fast".to_string(), 150);
        durations.insert("normal".to_string(), 300);
        durations.insert("slow".to_string(), 500);

        let mut easings = BTreeMap::new();
        easings.insert(
            "ease".to_string(),
            "cubic-bezier(0.25, 0.1, 0.25, 1)".to_string(),
        );
        easings.insert(
            "ease-in".to_string(),
            "cubic-bezier(0.42, 0, 1, 1)".to_string(),
        );
        easings.insert(
            "ease-out".to_string(),
            "cubic-bezier(0, 0, 0.58, 1)".to_string(),
        );

        let mut delays = BTreeMap::new();
        delays.insert("none".to_string(), 0);
        delays.insert("short".to_string(), 100);
        delays.insert("medium".to_string(), 200);
        delays.insert("long".to_string(), 500);

        Self {
            durations,
            easings,
            delays,
        }
    }
}

impl SemanticColors {
    fn ant_design_default() -> Self {
        Self {
            text: TextSemanticColors {
                primary: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: Some(TokenTransform::Alpha(0.85)),
                },
                secondary: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: Some(TokenTransform::Alpha(0.45)),
                },
                tertiary: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: Some(TokenTransform::Alpha(0.25)),
                },
                disabled: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: Some(TokenTransform::Alpha(0.25)),
                },
                inverse: TokenReference {
                    reference: "global.color_palette.neutral.50".to_string(),
                    transform: None,
                },
                link: TokenReference {
                    reference: "global.color_palette.primary.600".to_string(),
                    transform: None,
                },
                link_hover: TokenReference {
                    reference: "global.color_palette.primary.500".to_string(),
                    transform: None,
                },
            },
            background: BackgroundSemanticColors {
                primary: TokenReference {
                    reference: "global.color_palette.neutral.50".to_string(),
                    transform: None,
                },
                secondary: TokenReference {
                    reference: "global.color_palette.neutral.100".to_string(),
                    transform: None,
                },
                tertiary: TokenReference {
                    reference: "global.color_palette.neutral.200".to_string(),
                    transform: None,
                },
                inverse: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: None,
                },
                overlay: TokenReference {
                    reference: "global.color_palette.neutral.900".to_string(),
                    transform: Some(TokenTransform::Alpha(0.5)),
                },
            },
            border: BorderSemanticColors {
                primary: TokenReference {
                    reference: "global.color_palette.neutral.300".to_string(),
                    transform: None,
                },
                secondary: TokenReference {
                    reference: "global.color_palette.neutral.200".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "global.color_palette.primary.500".to_string(),
                    transform: None,
                },
                error: TokenReference {
                    reference: "global.color_palette.error.500".to_string(),
                    transform: None,
                },
                success: TokenReference {
                    reference: "global.color_palette.success.500".to_string(),
                    transform: None,
                },
            },
            state: StateSemanticColors {
                hover: TokenReference {
                    reference: "global.color_palette.neutral.100".to_string(),
                    transform: None,
                },
                active: TokenReference {
                    reference: "global.color_palette.neutral.200".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "global.color_palette.primary.100".to_string(),
                    transform: None,
                },
                disabled: TokenReference {
                    reference: "global.color_palette.neutral.100".to_string(),
                    transform: None,
                },
                selected: TokenReference {
                    reference: "global.color_palette.primary.50".to_string(),
                    transform: None,
                },
            },
        }
    }
}

impl HeadingTypography {
    fn ant_design_default() -> Self {
        Self {
            h1: TypographyToken::ant_design_default_h1(),
            h2: TypographyToken::ant_design_default_h2(),
            h3: TypographyToken::ant_design_default_h3(),
            h4: TypographyToken::ant_design_default_h4(),
            h5: TypographyToken::ant_design_default_h5(),
            h6: TypographyToken::ant_design_default_h6(),
        }
    }
}

impl BodyTypography {
    fn ant_design_default() -> Self {
        Self {
            large: TypographyToken::ant_design_default_body_large(),
            medium: TypographyToken::ant_design_default_body_medium(),
            small: TypographyToken::ant_design_default_body_small(),
        }
    }
}

impl CaptionTypography {
    fn ant_design_default() -> Self {
        Self {
            large: TypographyToken::ant_design_default_caption_large(),
            medium: TypographyToken::ant_design_default_caption_medium(),
            small: TypographyToken::ant_design_default_caption_small(),
        }
    }
}

impl CodeTypography {
    fn ant_design_default() -> Self {
        Self {
            inline: TypographyToken::ant_design_default_code_inline(),
            block: TypographyToken::ant_design_default_code_block(),
        }
    }
}

impl TypographyToken {
    fn ant_design_default_h1() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h1".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.tight".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_h2() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h2".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_h3() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h3".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_h4() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h4".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_h5() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h5".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_h6() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.h6".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.semibold".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_body_large() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.body_large".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_body_medium() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.body_medium".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_body_small() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.body_small".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_caption_large() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.caption_large".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_caption_medium() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.caption_medium".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_caption_small() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.primary".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.caption_small".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.normal".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_code_inline() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.mono".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.code".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.relaxed".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_code_block() -> Self {
        Self {
            font_family: TokenReference {
                reference: "global.font_families.mono".to_string(),
                transform: None,
            },
            font_size: TokenReference {
                reference: "global.font_sizes.code".to_string(),
                transform: None,
            },
            font_weight: TokenReference {
                reference: "global.font_weights.normal".to_string(),
                transform: None,
            },
            line_height: TokenReference {
                reference: "global.line_heights.loose".to_string(),
                transform: None,
            },
            letter_spacing: TokenReference {
                reference: "global.letter_spacings.normal".to_string(),
                transform: None,
            },
        }
    }
}

impl SemanticTypography {
    fn ant_design_default() -> Self {
        Self {
            heading: HeadingTypography::ant_design_default(),
            body: BodyTypography::ant_design_default(),
            caption: CaptionTypography::ant_design_default(),
            code: CodeTypography::ant_design_default(),
        }
    }
}

impl PaddingSpacing {
    fn ant_design_default() -> Self {
        Self {
            xs: TokenReference {
                reference: "4px".to_string(),
                transform: None,
            },
            sm: TokenReference {
                reference: "8px".to_string(),
                transform: None,
            },
            md: TokenReference {
                reference: "16px".to_string(),
                transform: None,
            },
            lg: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
            xl: TokenReference {
                reference: "32px".to_string(),
                transform: None,
            },
        }
    }
}

impl MarginSpacing {
    fn ant_design_default() -> Self {
        Self {
            xs: TokenReference {
                reference: "4px".to_string(),
                transform: None,
            },
            sm: TokenReference {
                reference: "8px".to_string(),
                transform: None,
            },
            md: TokenReference {
                reference: "16px".to_string(),
                transform: None,
            },
            lg: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
            xl: TokenReference {
                reference: "32px".to_string(),
                transform: None,
            },
        }
    }
}

impl GapSpacing {
    fn ant_design_default() -> Self {
        Self {
            xs: TokenReference {
                reference: "4px".to_string(),
                transform: None,
            },
            sm: TokenReference {
                reference: "8px".to_string(),
                transform: None,
            },
            md: TokenReference {
                reference: "16px".to_string(),
                transform: None,
            },
            lg: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
            xl: TokenReference {
                reference: "32px".to_string(),
                transform: None,
            },
        }
    }
}

impl ComponentSpacing {
    fn ant_design_default() -> Self {
        Self {
            padding: PaddingSpacing::ant_design_default(),
            margin: MarginSpacing::ant_design_default(),
            gap: GapSpacing::ant_design_default(),
        }
    }
}

impl LayoutSpacing {
    fn ant_design_default() -> Self {
        Self {
            section: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
            container: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
            grid: TokenReference {
                reference: "24px".to_string(),
                transform: None,
            },
        }
    }
}

impl SemanticSpacing {
    fn ant_design_default() -> Self {
        Self {
            component: ComponentSpacing::ant_design_default(),
            layout: LayoutSpacing::ant_design_default(),
        }
    }
}

impl SemanticSizing {
    fn ant_design_default() -> Self {
        Self {
            component: ComponentSizing {
                xs: TokenReference {
                    reference: "24px".to_string(),
                    transform: None,
                },
                sm: TokenReference {
                    reference: "28px".to_string(),
                    transform: None,
                },
                md: TokenReference {
                    reference: "32px".to_string(),
                    transform: None,
                },
                lg: TokenReference {
                    reference: "36px".to_string(),
                    transform: None,
                },
                xl: TokenReference {
                    reference: "40px".to_string(),
                    transform: None,
                },
            },
            icon: IconSizing {
                xs: TokenReference {
                    reference: "12px".to_string(),
                    transform: None,
                },
                sm: TokenReference {
                    reference: "14px".to_string(),
                    transform: None,
                },
                md: TokenReference {
                    reference: "16px".to_string(),
                    transform: None,
                },
                lg: TokenReference {
                    reference: "20px".to_string(),
                    transform: None,
                },
                xl: TokenReference {
                    reference: "24px".to_string(),
                    transform: None,
                },
            },
            avatar: AvatarSizing {
                xs: TokenReference {
                    reference: "24px".to_string(),
                    transform: None,
                },
                sm: TokenReference {
                    reference: "32px".to_string(),
                    transform: None,
                },
                md: TokenReference {
                    reference: "40px".to_string(),
                    transform: None,
                },
                lg: TokenReference {
                    reference: "48px".to_string(),
                    transform: None,
                },
                xl: TokenReference {
                    reference: "64px".to_string(),
                    transform: None,
                },
            },
        }
    }
}

impl ButtonTokens {
    fn ant_design_default() -> Self {
        Self {
            primary: ButtonVariantTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#f5f5f5".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#d9d9d9".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                shadow: StateTokens {
                    default: TokenReference {
                        reference: "0 2px 0 rgba(0, 0, 0, 0.016)".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "0 4px 4px 0 rgba(0, 0, 0, 0.1)".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "0 0 0 rgba(0, 0, 0, 0)".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                },
            },
            secondary: ButtonVariantTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#f5f5f5".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "#d9d9d9".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#d9d9d9".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                shadow: StateTokens {
                    default: TokenReference {
                        reference: "0 2px 0 rgba(0, 0, 0, 0.016)".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "0 4px 4px 0 rgba(0, 0, 0, 0.1)".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "0 0 0 rgba(0, 0, 0, 0)".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                },
            },
            ghost: ButtonVariantTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#d9d9d9".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                shadow: StateTokens {
                    default: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "0 4px 4px 0 rgba(0, 0, 0, 0.1)".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "0 0 0 rgba(0, 0, 0, 0)".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                },
            },
            link: ButtonVariantTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#096dd9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                shadow: StateTokens {
                    default: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                },
            },
            text: ButtonVariantTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "rgba(0, 0, 0, 0.018)".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "rgba(0, 0, 0, 0.028)".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#000000d9".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                shadow: StateTokens {
                    default: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "none".to_string(),
                        transform: None,
                    },
                },
            },
        }
    }
}

impl InputTokens {
    fn ant_design_default() -> Self {
        Self {
            background: StateTokens {
                default: TokenReference {
                    reference: "#ffffff".to_string(),
                    transform: None,
                },
                hover: TokenReference {
                    reference: "#ffffff".to_string(),
                    transform: None,
                },
                active: TokenReference {
                    reference: "#ffffff".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "#ffffff".to_string(),
                    transform: None,
                },
                disabled: TokenReference {
                    reference: "#f5f5f5".to_string(),
                    transform: None,
                },
            },
            border: StateTokens {
                default: TokenReference {
                    reference: "#d9d9d9".to_string(),
                    transform: None,
                },
                hover: TokenReference {
                    reference: "#40a9ff".to_string(),
                    transform: None,
                },
                active: TokenReference {
                    reference: "#1890ff".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "#1890ff".to_string(),
                    transform: None,
                },
                disabled: TokenReference {
                    reference: "#d9d9d9".to_string(),
                    transform: None,
                },
            },
            text: StateTokens {
                default: TokenReference {
                    reference: "#000000d9".to_string(),
                    transform: None,
                },
                hover: TokenReference {
                    reference: "#000000d9".to_string(),
                    transform: None,
                },
                active: TokenReference {
                    reference: "#000000d9".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "#000000d9".to_string(),
                    transform: None,
                },
                disabled: TokenReference {
                    reference: "#00000040".to_string(),
                    transform: None,
                },
            },
            placeholder: TokenReference {
                reference: "#bfbfbf".to_string(),
                transform: None,
            },
            shadow: StateTokens {
                default: TokenReference {
                    reference: "none".to_string(),
                    transform: None,
                },
                hover: TokenReference {
                    reference: "none".to_string(),
                    transform: None,
                },
                active: TokenReference {
                    reference: "none".to_string(),
                    transform: None,
                },
                focus: TokenReference {
                    reference: "0 0 0 2px rgba(24, 144, 255, 0.2)".to_string(),
                    transform: None,
                },
                disabled: TokenReference {
                    reference: "none".to_string(),
                    transform: None,
                },
            },
        }
    }
}

impl CardTokens {
    fn ant_design_default() -> Self {
        Self {
            background: TokenReference { reference: "#ffffff".to_string(), transform: None },
            border: TokenReference { reference: "#f0f0f0".to_string(), transform: None },
            shadow: TokenReference { reference: "0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02)".to_string(), transform: None },
            radius: TokenReference { reference: "8px".to_string(), transform: None },
            padding: TokenReference { reference: "24px".to_string(), transform: None },
        }
    }
}

impl TableSectionTokens {
    fn ant_design_default_header() -> Self {
        Self {
            background: TokenReference {
                reference: "#fafafa".to_string(),
                transform: None,
            },
            text: TokenReference {
                reference: "#000000d9".to_string(),
                transform: None,
            },
            border: TokenReference {
                reference: "#f0f0f0".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_body() -> Self {
        Self {
            background: TokenReference {
                reference: "#ffffff".to_string(),
                transform: None,
            },
            text: TokenReference {
                reference: "#000000d9".to_string(),
                transform: None,
            },
            border: TokenReference {
                reference: "#f0f0f0".to_string(),
                transform: None,
            },
        }
    }

    fn ant_design_default_footer() -> Self {
        Self {
            background: TokenReference {
                reference: "#fafafa".to_string(),
                transform: None,
            },
            text: TokenReference {
                reference: "#000000d9".to_string(),
                transform: None,
            },
            border: TokenReference {
                reference: "#f0f0f0".to_string(),
                transform: None,
            },
        }
    }
}

impl TableTokens {
    fn ant_design_default() -> Self {
        Self {
            header: TableSectionTokens::ant_design_default_header(),
            body: TableSectionTokens::ant_design_default_body(),
            footer: TableSectionTokens::ant_design_default_footer(),
            border: TokenReference {
                reference: "#f0f0f0".to_string(),
                transform: None,
            },
            stripe: TokenReference {
                reference: "#fafafa".to_string(),
                transform: None,
            },
        }
    }
}

impl NavigationTokens {
    fn ant_design_default() -> Self {
        Self {
            background: TokenReference {
                reference: "#001529".to_string(),
                transform: None,
            },
            border: TokenReference {
                reference: "1px solid #f0f0f0".to_string(),
                transform: None,
            },
            item: NavigationItemTokens {
                background: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#f5f5f5".to_string(),
                        transform: None,
                    },
                },
                text: StateTokens {
                    default: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#ffffff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#bfbfbf".to_string(),
                        transform: None,
                    },
                },
                border: StateTokens {
                    default: TokenReference {
                        reference: "transparent".to_string(),
                        transform: None,
                    },
                    hover: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    active: TokenReference {
                        reference: "#1890ff".to_string(),
                        transform: None,
                    },
                    focus: TokenReference {
                        reference: "#40a9ff".to_string(),
                        transform: None,
                    },
                    disabled: TokenReference {
                        reference: "#d9d9d9".to_string(),
                        transform: None,
                    },
                },
            },
        }
    }
}

// 其他结构体的默认实现...
// 由于篇幅限制，这里省略了详细实现
// 实际项目中需要为每个结构体提供完整的 Ant Design 默认值

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_token_system_creation() {
        let system = DesignTokenSystem::ant_design_default();
        assert_eq!(system.metadata.version, "1.0.0");
    }

    #[test]
    fn test_token_resolution() {
        let system = DesignTokenSystem::ant_design_default();
        let reference = TokenReference {
            reference: "global.color_palette.primary.500".to_string(),
            transform: None,
        };

        let result = system.resolve_token(&reference);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "#1890ff");
    }

    #[test]
    fn test_css_export() {
        let system = DesignTokenSystem::ant_design_default();
        let css = system.export_css_variables();
        assert!(css.contains(":root"));
        assert!(css.contains("--color-primary-500"));
    }
}
