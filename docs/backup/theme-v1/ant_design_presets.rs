//! Ant Design 主题预设配置
//!
//! 提供 Ant Design 设计系统的完整预设配置，包括亮色和暗色主题。
//! 这个模块集中管理所有 Ant Design 相关的默认值，避免在其他模块中重复定义。

use crate::theme::{design_token_system::*, design_tokens::*};

/// 创建 Ant Design 亮色主题的设计令牌
pub fn create_light_design_tokens() -> DesignTokens {
    DesignTokens {
        colors: create_light_color_tokens(),
        typography: create_typography_tokens(),
        spacing: create_spacing_tokens(),
        borders: create_border_tokens(),
        shadows: create_shadow_tokens(),
        motion: create_motion_tokens(),
        breakpoints: create_breakpoint_tokens(),
    }
}

/// 创建 Ant Design 暗色主题的设计令牌
pub fn create_dark_design_tokens() -> DesignTokens {
    DesignTokens {
        colors: create_dark_color_tokens(),
        typography: create_typography_tokens(),
        spacing: create_spacing_tokens(),
        borders: create_border_tokens(),
        shadows: create_shadow_tokens(),
        motion: create_motion_tokens(),
        breakpoints: create_breakpoint_tokens(),
    }
}

/// 创建 Ant Design 亮色主题的完整令牌系统
pub fn create_light_token_system() -> DesignTokenSystem {
    DesignTokenSystem {
        base_tokens: create_light_design_tokens(),
        alias_tokens: create_alias_tokens(),
        component_tokens: create_component_tokens(),
        computation_rules: create_computation_rules(),
        token_references: std::collections::HashMap::new(),
        metadata: TokenMetadata {
            version: "1.0.0".to_string(),
            description: "Ant Design Light Theme".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            author: "Ant Design Team".to_string(),
        },
    }
}

/// 创建 Ant Design 暗色主题的完整令牌系统
pub fn create_dark_token_system() -> DesignTokenSystem {
    DesignTokenSystem {
        base_tokens: create_dark_design_tokens(),
        alias_tokens: create_alias_tokens(),
        component_tokens: create_component_tokens(),
        computation_rules: create_computation_rules(),
        token_references: std::collections::HashMap::new(),
        metadata: TokenMetadata {
            version: "1.0.0".to_string(),
            description: "Ant Design Dark Theme".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
            author: "Ant Design Team".to_string(),
        },
    }
}

// ============================================================================
// 颜色令牌配置
// ============================================================================

/// 创建亮色主题的颜色令牌
fn create_light_color_tokens() -> ColorTokens {
    ColorTokens {
        primary: "#1890ff".to_string(),
        success: "#52c41a".to_string(),
        warning: "#faad14".to_string(),
        error: "#f5222d".to_string(),
        info: "#1890ff".to_string(),
        text: TextColors {
            primary: "rgba(0, 0, 0, 0.88)".to_string(),
            secondary: "rgba(0, 0, 0, 0.65)".to_string(),
            disabled: "rgba(0, 0, 0, 0.25)".to_string(),
            inverse: "rgba(255, 255, 255, 0.88)".to_string(),
        },
        background: BackgroundColors {
            primary: "#ffffff".to_string(),
            secondary: "#fafafa".to_string(),
            tertiary: "#f5f5f5".to_string(),
            inverse: "#141414".to_string(),
        },
        border: BorderColors {
            primary: "#d9d9d9".to_string(),
            secondary: "#f0f0f0".to_string(),
            inverse: "#434343".to_string(),
        },
        blue: create_blue_color_scale(),
        green: create_green_color_scale(),
        red: create_red_color_scale(),
        orange: create_orange_color_scale(),
        gray: create_gray_color_scale(),
    }
}

/// 创建暗色主题的颜色令牌
fn create_dark_color_tokens() -> ColorTokens {
    ColorTokens {
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
        blue: create_blue_color_scale(),
        green: create_green_color_scale(),
        red: create_red_color_scale(),
        orange: create_orange_color_scale(),
        gray: create_gray_color_scale(),
    }
}

// 颜色色阶定义
fn create_blue_color_scale() -> ColorScale {
    ColorScale {
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

fn create_green_color_scale() -> ColorScale {
    ColorScale {
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

fn create_red_color_scale() -> ColorScale {
    ColorScale {
        c1: "#fff2e8".to_string(),
        c2: "#ffd8bf".to_string(),
        c3: "#ffbb96".to_string(),
        c4: "#ff9c6e".to_string(),
        c5: "#ff7a45".to_string(),
        c6: "#fa541c".to_string(),
        c7: "#d4380d".to_string(),
        c8: "#ad2102".to_string(),
        c9: "#871400".to_string(),
        c10: "#610b00".to_string(),
    }
}

fn create_orange_color_scale() -> ColorScale {
    ColorScale {
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

fn create_gray_color_scale() -> ColorScale {
    ColorScale {
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

// ============================================================================
// 其他令牌配置
// ============================================================================

/// 创建字体令牌
fn create_typography_tokens() -> TypographyTokens {
    TypographyTokens {
        font_family: FontFamily {
            primary: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif".to_string(),
            monospace: "'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace".to_string(),
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
            normal: "0em".to_string(),
            wide: "0.025em".to_string(),
        },
    }
}

/// 创建间距令牌
fn create_spacing_tokens() -> SpacingTokens {
    SpacingTokens {
        xs: "4px".to_string(),
        sm: "8px".to_string(),
        md: "16px".to_string(),
        lg: "24px".to_string(),
        xl: "32px".to_string(),
        xxl: "48px".to_string(),
        xxxl: "64px".to_string(),
    }
}

/// 创建边框令牌
fn create_border_tokens() -> BorderTokens {
    BorderTokens {
        width: BorderWidths {
            none: "0px".to_string(),
            thin: "1px".to_string(),
            medium: "2px".to_string(),
            thick: "4px".to_string(),
        },
        style: BorderStyle {
            solid: "solid".to_string(),
            dashed: "dashed".to_string(),
            dotted: "dotted".to_string(),
        },
        radius: BorderRadius {
            none: "0".to_string(),
            sm: "2px".to_string(),
            md: "6px".to_string(),
            lg: "8px".to_string(),
            xl: "12px".to_string(),
            full: "9999px".to_string(),
        },
    }
}

/// 创建阴影令牌
fn create_shadow_tokens() -> ShadowTokens {
    ShadowTokens {
        sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)".to_string(),
        md: "0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)".to_string(),
        lg: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".to_string(),
        xl: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)".to_string(),
        inner: "inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)".to_string(),
    }
}

/// 创建动画令牌
fn create_motion_tokens() -> MotionTokens {
    MotionTokens {
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

/// 创建断点令牌
fn create_breakpoint_tokens() -> BreakpointTokens {
    BreakpointTokens {
        xs: "480px".to_string(),
        sm: "576px".to_string(),
        md: "768px".to_string(),
        lg: "992px".to_string(),
        xl: "1200px".to_string(),
        xxl: "1600px".to_string(),
    }
}

// ============================================================================
// 高级令牌配置
// ============================================================================

/// 创建别名令牌
fn create_alias_tokens() -> AliasTokens {
    AliasTokens {
        semantic_colors: create_semantic_colors(),
        semantic_typography: create_semantic_typography(),
        semantic_spacing: create_semantic_spacing(),
        semantic_sizing: create_semantic_sizing(),
    }
}

/// 创建组件令牌
fn create_component_tokens() -> ComponentTokens {
    ComponentTokens {
        button: create_button_tokens(),
        input: create_input_tokens(),
        card: create_card_tokens(),
        table: create_table_tokens(),
        navigation: create_navigation_tokens(),
    }
}

/// 创建计算规则
fn create_computation_rules() -> ComputationRules {
    ComputationRules {
        color_rules: vec![],
        spacing_rules: vec![],
        typography_rules: vec![],
    }
}

// ============================================================================
// 语义化令牌配置
// ============================================================================

fn create_semantic_colors() -> SemanticColors {
    SemanticColors {
        text: create_text_semantic_colors(),
        background: create_background_semantic_colors(),
        border: create_border_semantic_colors(),
        state: create_state_semantic_colors(),
    }
}

fn create_text_semantic_colors() -> TextSemanticColors {
    TextSemanticColors {
        primary: TokenReference::Base("colors.text.primary".to_string()),
        secondary: TokenReference::Base("colors.text.secondary".to_string()),
        disabled: TokenReference::Base("colors.text.disabled".to_string()),
        inverse: TokenReference::Base("colors.text.inverse".to_string()),
        link: TokenReference::Base("colors.primary".to_string()),
        link_hover: TokenReference::Base("colors.primary.hover".to_string()),
        tertiary: TokenReference::Base("colors.background.tertiary".to_string()),
    }
}

fn create_background_semantic_colors() -> BackgroundSemanticColors {
    BackgroundSemanticColors {
        primary: TokenReference::Base("colors.background.primary".to_string()),
        secondary: TokenReference::Base("colors.background.secondary".to_string()),
        tertiary: TokenReference::Base("colors.background.tertiary".to_string()),
        inverse: TokenReference::Base("colors.background.inverse".to_string()),
        overlay: TokenReference::Transform {
            base: "colors.gray.c10".to_string(),
            transform: TokenTransform::Alpha(0.5),
        },
    }
}

fn create_border_semantic_colors() -> BorderSemanticColors {
    BorderSemanticColors {
        primary: TokenReference::Base("colors.border.primary".to_string()),
        secondary: TokenReference::Base("colors.border.secondary".to_string()),
        focus: TokenReference::Base("colors.primary".to_string()),
        error: TokenReference::Base("colors.error".to_string()),
        success: TokenReference::Base("colors.success".to_string()),
    }
}

fn create_state_semantic_colors() -> StateSemanticColors {
    StateSemanticColors {
        hover: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        active: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        focus: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.05),
        },
        disabled: TokenReference::Base("colors.text.disabled".to_string()),
        selected: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.2),
        },
    }
}

fn create_semantic_typography() -> SemanticTypography {
    SemanticTypography {
        heading: create_heading_typography(),
        body: create_body_typography(),
        caption: create_caption_typography(),
        code: create_code_typography(),
    }
}

fn create_heading_typography() -> HeadingTypography {
    HeadingTypography {
        h1: create_h1_typography(),
        h2: create_h2_typography(),
        h3: create_h3_typography(),
        h4: create_h4_typography(),
        h5: create_h5_typography(),
        h6: create_h6_typography(),
    }
}

fn create_body_typography() -> BodyTypography {
    BodyTypography {
        large: create_body_large_typography(),
        medium: create_body_medium_typography(),
        small: create_body_small_typography(),
    }
}

fn create_caption_typography() -> CaptionTypography {
    CaptionTypography {
        large: create_caption_large_typography(),
        medium: create_caption_medium_typography(),
        small: create_caption_small_typography(),
    }
}

fn create_code_typography() -> CodeTypography {
    CodeTypography {
        inline: create_code_inline_typography(),
        block: create_code_block_typography(),
    }
}

// 具体的字体令牌配置
fn create_h1_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Transform {
            base: "typography.font_size.xxl".to_string(),
            transform: TokenTransform::Scale(1.5),
        },
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.tight".to_string()),
        letter_spacing: TokenReference::Literal("-0.025em".to_string()),
        text_transform: None,
        text_decoration: None,
    }
}

fn create_h2_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Transform {
            base: "typography.font_size.xxl".to_string(),
            transform: TokenTransform::Scale(1.25),
        },
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.tight".to_string()),
        letter_spacing: TokenReference::Literal("-0.025em".to_string()),
        text_transform: None,
        text_decoration: None,
    }
}

fn create_h3_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.xxl".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.tight".to_string()),
        letter_spacing: TokenReference::Literal("-0.025em".to_string()),
        text_transform: None,
        text_decoration: None,
    }
}

fn create_h4_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.xl".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: TokenReference::Literal("-0.025em".to_string()),
        text_transform: None,
        text_decoration: None,
    }
}

fn create_h5_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.lg".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_h6_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.md".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_body_large_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.lg".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_body_medium_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.md".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_body_small_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.sm".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_caption_large_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.sm".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.medium".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_caption_medium_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Base("typography.font_size.xs".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.medium".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_caption_small_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.primary".to_string()),
        font_size: TokenReference::Transform {
            base: "typography.font_size.xs".to_string(),
            transform: TokenTransform::Scale(0.875),
        },
        font_weight: TokenReference::Base("typography.font_weight.medium".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_code_inline_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.monospace".to_string()),
        font_size: TokenReference::Transform {
            base: "typography.font_size.sm".to_string(),
            transform: TokenTransform::Scale(0.875),
        },
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
        line_height: TokenReference::Base("typography.line_height.normal".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_code_block_typography() -> TypographyToken {
    TypographyToken {
        font_family: TokenReference::Base("typography.font_family.monospace".to_string()),
        font_size: TokenReference::Base("typography.font_size.sm".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
        line_height: TokenReference::Base("typography.line_height.relaxed".to_string()),
        letter_spacing: None,
        text_transform: None,
        text_decoration: None,
    }
}

fn create_semantic_spacing() -> SemanticSpacing {
    SemanticSpacing {
        component: create_component_spacing(),
        layout: create_layout_spacing(),
    }
}

fn create_component_spacing() -> ComponentSpacing {
    ComponentSpacing {
        padding: create_padding_spacing(),
        margin: create_margin_spacing(),
        gap: create_gap_spacing(),
    }
}

fn create_layout_spacing() -> LayoutSpacing {
    LayoutSpacing {
        section: TokenReference::Base("spacing.xxl".to_string()),
        container: TokenReference::Base("spacing.xl".to_string()),
        content: TokenReference::Base("spacing.lg".to_string()),
    }
}

fn create_padding_spacing() -> PaddingSpacing {
    PaddingSpacing {
        xs: TokenReference::Base("spacing.xs".to_string()),
        sm: TokenReference::Base("spacing.sm".to_string()),
        md: TokenReference::Base("spacing.md".to_string()),
        lg: TokenReference::Base("spacing.lg".to_string()),
        xl: TokenReference::Base("spacing.xl".to_string()),
    }
}

fn create_margin_spacing() -> MarginSpacing {
    MarginSpacing {
        xs: TokenReference::Base("spacing.xs".to_string()),
        sm: TokenReference::Base("spacing.sm".to_string()),
        md: TokenReference::Base("spacing.md".to_string()),
        lg: TokenReference::Base("spacing.lg".to_string()),
        xl: TokenReference::Base("spacing.xl".to_string()),
    }
}

fn create_gap_spacing() -> GapSpacing {
    GapSpacing {
        xs: TokenReference::Base("spacing.xs".to_string()),
        sm: TokenReference::Base("spacing.sm".to_string()),
        md: TokenReference::Base("spacing.md".to_string()),
        lg: TokenReference::Base("spacing.lg".to_string()),
        xl: TokenReference::Base("spacing.xl".to_string()),
    }
}

fn create_semantic_sizing() -> SemanticSizing {
    SemanticSizing {
        icon: create_icon_sizing(),
        button: create_button_sizing(),
        input: create_input_sizing(),
        avatar: create_avatar_sizing(),
    }
}

fn create_icon_sizing() -> IconSizing {
    IconSizing {
        xs: TokenReference::Literal("12px".to_string()),
        sm: TokenReference::Literal("16px".to_string()),
        md: TokenReference::Literal("20px".to_string()),
        lg: TokenReference::Literal("24px".to_string()),
        xl: TokenReference::Literal("32px".to_string()),
    }
}

fn create_button_sizing() -> ComponentSizing {
    ComponentSizing {
        xs: TokenReference::Literal("20px".to_string()),
        sm: TokenReference::Literal("24px".to_string()),
        md: TokenReference::Literal("32px".to_string()),
        lg: TokenReference::Literal("40px".to_string()),
        xl: TokenReference::Literal("48px".to_string()),
    }
}

fn create_input_sizing() -> ComponentSizing {
    ComponentSizing {
        xs: TokenReference::Literal("20px".to_string()),
        sm: TokenReference::Literal("24px".to_string()),
        md: TokenReference::Literal("32px".to_string()),
        lg: TokenReference::Literal("40px".to_string()),
        xl: TokenReference::Literal("48px".to_string()),
    }
}

fn create_avatar_sizing() -> AvatarSizing {
    AvatarSizing {
        xs: TokenReference::Literal("24px".to_string()),
        sm: TokenReference::Literal("32px".to_string()),
        md: TokenReference::Literal("40px".to_string()),
        lg: TokenReference::Literal("64px".to_string()),
        xl: TokenReference::Literal("80px".to_string()),
    }
}

// ============================================================================
// 组件令牌配置
// ============================================================================

fn create_button_tokens() -> ButtonTokens {
    ButtonTokens {
        primary: create_primary_button_tokens(),
        secondary: create_secondary_button_tokens(),
        ghost: create_ghost_button_tokens(),
        link: create_link_button_tokens(),
        text: create_text_button_tokens(),
        danger: create_danger_button_tokens(),
    }
}

fn create_primary_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Base("colors.primary".to_string()),
        color: TokenReference::Literal("#ffffff".to_string()),
        border: TokenReference::Base("colors.primary".to_string()),
        hover_background: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        hover_color: TokenReference::Literal("#ffffff".to_string()),
        hover_border: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        active_background: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        active_color: TokenReference::Literal("#ffffff".to_string()),
        active_border: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        disabled_background: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_color: TokenReference::Literal("#ffffff".to_string()),
        disabled_border: TokenReference::Base("colors.text.disabled".to_string()),
    }
}

fn create_secondary_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Base("colors.background.primary".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Base("colors.border.primary".to_string()),
        hover_background: TokenReference::Base("colors.background.secondary".to_string()),
        hover_color: TokenReference::Base("colors.primary".to_string()),
        hover_border: TokenReference::Base("colors.primary".to_string()),
        active_background: TokenReference::Base("colors.background.tertiary".to_string()),
        active_color: TokenReference::Base("colors.primary".to_string()),
        active_border: TokenReference::Base("colors.primary".to_string()),
        disabled_background: TokenReference::Base("colors.background.secondary".to_string()),
        disabled_color: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_border: TokenReference::Base("colors.border.secondary".to_string()),
    }
}

fn create_ghost_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Literal("transparent".to_string()),
        color: TokenReference::Base("colors.primary".to_string()),
        border: TokenReference::Base("colors.primary".to_string()),
        hover_background: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Alpha(0.1),
        },
        hover_color: TokenReference::Base("colors.primary".to_string()),
        hover_border: TokenReference::Base("colors.primary".to_string()),
        active_background: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Alpha(0.2),
        },
        active_color: TokenReference::Base("colors.primary".to_string()),
        active_border: TokenReference::Base("colors.primary".to_string()),
        disabled_background: TokenReference::Literal("transparent".to_string()),
        disabled_color: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_border: TokenReference::Base("colors.border.secondary".to_string()),
    }
}

fn create_link_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Literal("transparent".to_string()),
        color: TokenReference::Base("colors.primary".to_string()),
        border: TokenReference::Literal("transparent".to_string()),
        hover_background: TokenReference::Literal("transparent".to_string()),
        hover_color: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        hover_border: TokenReference::Literal("transparent".to_string()),
        active_background: TokenReference::Literal("transparent".to_string()),
        active_color: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        active_border: TokenReference::Literal("transparent".to_string()),
        disabled_background: TokenReference::Literal("transparent".to_string()),
        disabled_color: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_border: TokenReference::Literal("transparent".to_string()),
    }
}

fn create_text_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Literal("transparent".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Literal("transparent".to_string()),
        hover_background: TokenReference::Transform {
            base: "colors.gray.c5".to_string(),
            transform: TokenTransform::Alpha(0.1),
        },
        hover_color: TokenReference::Base("colors.text.primary".to_string()),
        hover_border: TokenReference::Literal("transparent".to_string()),
        active_background: TokenReference::Transform {
            base: "colors.gray.c5".to_string(),
            transform: TokenTransform::Alpha(0.2),
        },
        active_color: TokenReference::Base("colors.text.primary".to_string()),
        active_border: TokenReference::Literal("transparent".to_string()),
        disabled_background: TokenReference::Literal("transparent".to_string()),
        disabled_color: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_border: TokenReference::Literal("transparent".to_string()),
    }
}

fn create_danger_button_tokens() -> ButtonVariantTokens {
    ButtonVariantTokens {
        background: TokenReference::Base("colors.error".to_string()),
        color: TokenReference::Literal("#ffffff".to_string()),
        border: TokenReference::Base("colors.error".to_string()),
        hover_background: TokenReference::Transform {
            base: "colors.error".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        hover_color: TokenReference::Literal("#ffffff".to_string()),
        hover_border: TokenReference::Transform {
            base: "colors.error".to_string(),
            transform: TokenTransform::Lighten(0.1),
        },
        active_background: TokenReference::Transform {
            base: "colors.error".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        active_color: TokenReference::Literal("#ffffff".to_string()),
        active_border: TokenReference::Transform {
            base: "colors.error".to_string(),
            transform: TokenTransform::Darken(0.1),
        },
        disabled_background: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_color: TokenReference::Literal("#ffffff".to_string()),
        disabled_border: TokenReference::Base("colors.text.disabled".to_string()),
    }
}

fn create_input_tokens() -> InputTokens {
    InputTokens {
        background: TokenReference::Base("colors.background.primary".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Base("colors.border.primary".to_string()),
        placeholder_color: TokenReference::Base("colors.text.secondary".to_string()),
        focus_border: TokenReference::Base("colors.primary".to_string()),
        focus_shadow: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Alpha(0.2),
        },
        error_border: TokenReference::Base("colors.error".to_string()),
        error_shadow: TokenReference::Transform {
            base: "colors.error".to_string(),
            transform: TokenTransform::Alpha(0.2),
        },
        disabled_background: TokenReference::Base("colors.background.secondary".to_string()),
        disabled_color: TokenReference::Base("colors.text.disabled".to_string()),
        disabled_border: TokenReference::Base("colors.border.secondary".to_string()),
    }
}

fn create_card_tokens() -> CardTokens {
    CardTokens {
        background: TokenReference::Base("colors.background.primary".to_string()),
        border: TokenReference::Base("colors.border.secondary".to_string()),
        shadow: TokenReference::Base("shadows.sm".to_string()),
        hover_shadow: TokenReference::Base("shadows.md".to_string()),
        header_background: TokenReference::Base("colors.background.secondary".to_string()),
        header_border: TokenReference::Base("colors.border.secondary".to_string()),
    }
}

fn create_table_tokens() -> TableTokens {
    TableTokens {
        header: create_table_header_tokens(),
        body: create_table_body_tokens(),
        footer: create_table_footer_tokens(),
        border: TokenReference::Base("colors.border.primary".to_string()),
        stripe: TokenReference::Base("colors.background.secondary".to_string()),
    }
}

fn create_table_header_tokens() -> TableSectionTokens {
    TableSectionTokens {
        background: TokenReference::Base("colors.background.secondary".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Base("colors.border.primary".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.semibold".to_string()),
    }
}

fn create_table_body_tokens() -> TableSectionTokens {
    TableSectionTokens {
        background: TokenReference::Base("colors.background.primary".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Base("colors.border.secondary".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
    }
}

fn create_table_footer_tokens() -> TableSectionTokens {
    TableSectionTokens {
        background: TokenReference::Base("colors.background.secondary".to_string()),
        color: TokenReference::Base("colors.text.secondary".to_string()),
        border: TokenReference::Base("colors.border.primary".to_string()),
        font_weight: TokenReference::Base("typography.font_weight.normal".to_string()),
    }
}

fn create_navigation_tokens() -> NavigationTokens {
    NavigationTokens {
        background: TokenReference::Base("colors.background.primary".to_string()),
        color: TokenReference::Base("colors.text.primary".to_string()),
        border: TokenReference::Base("colors.border.secondary".to_string()),
        active_background: TokenReference::Transform {
            base: "colors.primary".to_string(),
            transform: TokenTransform::Alpha(0.1),
        },
        active_color: TokenReference::Base("colors.primary".to_string()),
        hover_background: TokenReference::Transform {
            base: "colors.gray.c5".to_string(),
            transform: TokenTransform::Alpha(0.1),
        },
        hover_color: TokenReference::Base("colors.text.primary".to_string()),
    }
}
