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
        ColorValue, DimensionValue, MathOperation, ShadowValue, ThemeVariant, TokenDefinitions,
        TokenPath, TokenReference, TokenTier, TokenTransform, TokenType, TokenValidationError,
        TokenValue, TypographyValue,
    },
    token_resolver::TokenResolver,
    token_values::{AntDesignTokenValues, DesignTokens},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt;

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

/// 按钮组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonTokens {
    /// 默认变体
    pub default: ButtonVariantTokens,
    /// 主要变体
    pub primary: ButtonVariantTokens,
    /// 虚线变体
    pub dashed: ButtonVariantTokens,
    /// 文本变体
    pub text: ButtonVariantTokens,
    /// 链接变体
    pub link: ButtonVariantTokens,
}

/// 按钮变体令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonVariantTokens {
    /// 默认状态
    pub default: StateTokens,
    /// 悬停状态
    pub hover: StateTokens,
    /// 激活状态
    pub active: StateTokens,
    /// 禁用状态
    pub disabled: StateTokens,
    /// 聚焦状态
    pub focus: StateTokens,
}

/// 状态令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTokens {
    /// 背景色
    pub background_color: TokenReference,
    /// 文本颜色
    pub text_color: TokenReference,
    /// 边框颜色
    pub border_color: TokenReference,
    /// 边框宽度
    pub border_width: TokenReference,
    /// 边框样式
    pub border_style: TokenReference,
    /// 圆角半径
    pub border_radius: TokenReference,
    /// 内边距
    pub padding: TokenReference,
    /// 阴影
    pub shadow: TokenReference,
}

/// 输入框组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTokens {
    /// 默认状态
    pub default: StateTokens,
    /// 悬停状态
    pub hover: StateTokens,
    /// 聚焦状态
    pub focus: StateTokens,
    /// 禁用状态
    pub disabled: StateTokens,
    /// 错误状态
    pub error: StateTokens,
}

/// 卡片组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTokens {
    /// 默认状态
    pub default: StateTokens,
    /// 悬停状态
    pub hover: StateTokens,
    /// 头部令牌
    pub header: CardSectionTokens,
    /// 主体令牌
    pub body: CardSectionTokens,
    /// 底部令牌
    pub footer: CardSectionTokens,
}

/// 卡片区域令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSectionTokens {
    /// 背景色
    pub background_color: TokenReference,
    /// 内边距
    pub padding: TokenReference,
    /// 边框
    pub border: TokenReference,
}

/// 表格组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableTokens {
    /// 表头令牌
    pub header: TableSectionTokens,
    /// 表体令牌
    pub body: TableSectionTokens,
    /// 表尾令牌
    pub footer: TableSectionTokens,
    /// 行令牌
    pub row: TableRowTokens,
}

/// 表格区域令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSectionTokens {
    /// 背景色
    pub background_color: TokenReference,
    /// 文本颜色
    pub text_color: TokenReference,
    /// 边框颜色
    pub border_color: TokenReference,
}

/// 表格行令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRowTokens {
    /// 默认状态
    pub default: StateTokens,
    /// 悬停状态
    pub hover: StateTokens,
    /// 选中状态
    pub selected: StateTokens,
    /// 条纹状态
    pub striped: StateTokens,
}

/// 导航组件令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationTokens {
    /// 导航项令牌
    pub item: NavigationItemTokens,
    /// 子菜单令牌
    pub submenu: NavigationSubmenuTokens,
}

/// 导航项令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationItemTokens {
    /// 默认状态
    pub default: StateTokens,
    /// 悬停状态
    pub hover: StateTokens,
    /// 激活状态
    pub active: StateTokens,
    /// 选中状态
    pub selected: StateTokens,
}

/// 导航子菜单令牌
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationSubmenuTokens {
    /// 背景色
    pub background_color: TokenReference,
    /// 边框颜色
    pub border_color: TokenReference,
    /// 阴影
    pub shadow: TokenReference,
}

/// 颜色计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorComputationRule {
    /// 规则名称
    pub name: String,
    /// 输入令牌路径
    pub input_token: TokenPath,
    /// 输出令牌路径
    pub output_token: TokenPath,
    /// 变换函数
    pub transform: TokenTransform,
    /// 条件
    pub condition: Option<String>,
}

/// 间距计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingComputationRule {
    /// 规则名称
    pub name: String,
    /// 基础令牌
    pub base_token: TokenPath,
    /// 比例因子
    pub scale_factor: f32,
    /// 输出令牌路径
    pub output_token: TokenPath,
    /// 最小值
    pub min_value: Option<DimensionValue>,
    /// 最大值
    pub max_value: Option<DimensionValue>,
}

/// 字体计算规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyComputationRule {
    /// 规则名称
    pub name: String,
    /// 基础字体大小
    pub base_font_size: TokenPath,
    /// 比例因子
    pub scale_factor: f32,
    /// 输出令牌路径
    pub output_token: TokenPath,
    /// 行高比例
    pub line_height_ratio: Option<f32>,
}

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
        }
    }
}

impl DesignTokenSystem {
    /// 创建新的设计令牌系统
    pub fn new() -> Self {
        Self::with_config(TokenSystemConfig::default())
    }

    /// 创建使用Ant Design默认令牌的系统
    pub fn ant_design_default() -> Self {
        let store = DesignTokens::ant_design_default();
        Self::with_store(store, TokenSystemConfig::default())
    }

    /// 使用配置创建设计令牌系统
    pub fn with_config(config: TokenSystemConfig) -> Self {
        let store = AntDesignTokenValues::create_default_store();
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

    /// 导出令牌为JSON
    pub fn export_tokens_json(&mut self) -> Result<String, String> {
        let paths = self
            .css_generator
            .get_resolver()
            .list_token_paths(self.current_theme);
        let mut tokens = HashMap::new();

        for path in paths {
            match self
                .css_generator
                .get_resolver_mut()
                .resolve_token(&path, self.current_theme)
            {
                Ok(value) => {
                    tokens.insert(path.to_string(), value);
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to resolve token {}: {}",
                        path.to_string(),
                        e
                    ));
                }
            }
        }

        serde_json::to_string_pretty(&tokens)
            .map_err(|e| format!("Failed to serialize tokens: {}", e))
    }

    /// 从JSON导入令牌
    pub fn import_tokens_json(&mut self, json: &str) -> Result<(), String> {
        let tokens: HashMap<String, TokenValue> =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        self.set_tokens_batch(tokens)
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
        let store = AntDesignTokenValues::create_default_store();
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

        let computation_rules_count = self.computation_rules.color_rules.len()
            + self.computation_rules.spacing_rules.len()
            + self.computation_rules.typography_rules.len();

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

    /// 创建令牌值存储
    fn create_token_value_store(&self) -> DesignTokens {
        // 这里需要将分层令牌转换为扁平的令牌值存储
        // 实际实现需要根据TokenValueStore的具体结构来调整
        AntDesignTokenValues::create_default_store()
    }

    /// 更新主题相关令牌
    fn update_theme_dependent_tokens(&mut self) {
        // 根据当前主题更新颜色令牌
        match self.current_theme {
            ThemeVariant::Light => self.apply_light_theme_colors(),
            ThemeVariant::Dark => self.apply_dark_theme_colors(),
            ThemeVariant::Auto => {
                // Auto模式下根据系统设置或默认使用浅色主题
                self.apply_light_theme_colors();
            }
        }
    }

    /// 应用浅色主题颜色
    fn apply_light_theme_colors(&mut self) {
        // 实现浅色主题的颜色应用逻辑
    }

    /// 应用深色主题颜色
    fn apply_dark_theme_colors(&mut self) {
        // 实现深色主题的颜色应用逻辑
    }

    /// 获取全局令牌值
    fn get_global_token_value(&self, path: &TokenPath) -> Option<TokenValue> {
        // 根据路径从全局令牌中获取值
        None // 占位实现
    }

    /// 获取别名令牌值
    fn get_alias_token_value(&self, path: &TokenPath) -> Option<TokenValue> {
        // 根据路径从别名令牌中获取值
        None // 占位实现
    }

    /// 获取组件令牌值
    fn get_component_token_value(&self, path: &TokenPath) -> Option<TokenValue> {
        // 根据路径从组件令牌中获取值
        None // 占位实现
    }

    /// 应用颜色计算规则
    fn apply_color_computation_rule(
        &mut self,
        rule: &ColorComputationRule,
    ) -> Result<(), TokenValidationError> {
        // 实现颜色计算规则的应用逻辑
        Ok(())
    }

    /// 应用间距计算规则
    fn apply_spacing_computation_rule(
        &mut self,
        rule: &SpacingComputationRule,
    ) -> Result<(), TokenValidationError> {
        // 实现间距计算规则的应用逻辑
        Ok(())
    }

    /// 应用字体计算规则
    fn apply_typography_computation_rule(
        &mut self,
        rule: &TypographyComputationRule,
    ) -> Result<(), TokenValidationError> {
        // 实现字体计算规则的应用逻辑
        Ok(())
    }

    /// 验证全局令牌
    fn validate_global_tokens(&self) -> Result<(), Vec<TokenValidationError>> {
        // 实现全局令牌验证逻辑
        Ok(())
    }

    /// 验证别名令牌
    fn validate_alias_tokens(&self) -> Result<(), Vec<TokenValidationError>> {
        // 实现别名令牌验证逻辑
        Ok(())
    }

    /// 验证组件令牌
    fn validate_component_tokens(&self) -> Result<(), Vec<TokenValidationError>> {
        // 实现组件令牌验证逻辑
        Ok(())
    }

    /// 验证计算规则
    fn validate_computation_rules(&self) -> Result<(), Vec<TokenValidationError>> {
        // 实现计算规则验证逻辑
        Ok(())
    }

    /// 统计全局令牌数量
    fn count_global_tokens(&self) -> usize {
        // 实现全局令牌计数逻辑
        0
    }

    /// 统计别名令牌数量
    fn count_alias_tokens(&self) -> usize {
        // 实现别名令牌计数逻辑
        0
    }

    /// 统计组件令牌数量
    fn count_component_tokens(&self) -> usize {
        // 实现组件令牌计数逻辑
        0
    }

    /// 创建Ant Design全局令牌
    fn create_ant_design_global_tokens(&self) -> GlobalTokens {
        GlobalTokens::default()
    }

    /// 创建Ant Design别名令牌
    fn create_ant_design_alias_tokens(&self) -> AliasTokens {
        AliasTokens::default()
    }

    /// 创建Ant Design组件令牌
    fn create_ant_design_component_tokens(&self) -> ComponentTokens {
        ComponentTokens::default()
    }

    /// 创建Ant Design计算规则
    fn create_ant_design_computation_rules(&self) -> ComputationRules {
        ComputationRules::default()
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
            button: ButtonTokens::default(),
            input: InputTokens::default(),
            card: CardTokens::default(),
            table: TableTokens::default(),
            navigation: NavigationTokens::default(),
        }
    }
}

impl Default for ComputationRules {
    fn default() -> Self {
        Self {
            color_rules: Vec::new(),
            spacing_rules: Vec::new(),
            typography_rules: Vec::new(),
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
            description: "Ant Design Token System".to_string(),
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

impl Default for ButtonTokens {
    fn default() -> Self {
        Self {
            default: ButtonVariantTokens::default(),
            primary: ButtonVariantTokens::default(),
            dashed: ButtonVariantTokens::default(),
            text: ButtonVariantTokens::default(),
            link: ButtonVariantTokens::default(),
        }
    }
}

impl Default for ButtonVariantTokens {
    fn default() -> Self {
        Self {
            default: StateTokens::default(),
            hover: StateTokens::default(),
            active: StateTokens::default(),
            disabled: StateTokens::default(),
            focus: StateTokens::default(),
        }
    }
}

impl Default for StateTokens {
    fn default() -> Self {
        Self {
            background_color: TokenReference::new("color.background.primary".to_string()),
            text_color: TokenReference::new("color.text.primary".to_string()),
            border_color: TokenReference::new("color.border.default".to_string()),
            border_width: TokenReference::new("border.width.default".to_string()),
            border_style: TokenReference::new("border.style.solid".to_string()),
            border_radius: TokenReference::new("border.radius.default".to_string()),
            padding: TokenReference::new("spacing.padding.md".to_string()),
            shadow: TokenReference::new("shadow.none".to_string()),
        }
    }
}

impl Default for InputTokens {
    fn default() -> Self {
        Self {
            default: StateTokens::default(),
            hover: StateTokens::default(),
            focus: StateTokens::default(),
            disabled: StateTokens::default(),
            error: StateTokens::default(),
        }
    }
}

impl Default for CardTokens {
    fn default() -> Self {
        Self {
            default: StateTokens::default(),
            hover: StateTokens::default(),
            header: CardSectionTokens::default(),
            body: CardSectionTokens::default(),
            footer: CardSectionTokens::default(),
        }
    }
}

impl Default for CardSectionTokens {
    fn default() -> Self {
        Self {
            background_color: TokenReference::new("color.background.primary".to_string()),
            padding: TokenReference::new("spacing.padding.md".to_string()),
            border: TokenReference::new("border.default".to_string()),
        }
    }
}

impl Default for TableTokens {
    fn default() -> Self {
        Self {
            header: TableSectionTokens::default(),
            body: TableSectionTokens::default(),
            footer: TableSectionTokens::default(),
            row: TableRowTokens::default(),
        }
    }
}

impl Default for TableSectionTokens {
    fn default() -> Self {
        Self {
            background_color: TokenReference::new("color.background.primary".to_string()),
            text_color: TokenReference::new("color.text.primary".to_string()),
            border_color: TokenReference::new("color.border.default".to_string()),
        }
    }
}

impl Default for TableRowTokens {
    fn default() -> Self {
        Self {
            default: StateTokens::default(),
            hover: StateTokens::default(),
            selected: StateTokens::default(),
            striped: StateTokens::default(),
        }
    }
}

impl Default for NavigationTokens {
    fn default() -> Self {
        Self {
            item: NavigationItemTokens::default(),
            submenu: NavigationSubmenuTokens::default(),
        }
    }
}

impl Default for NavigationItemTokens {
    fn default() -> Self {
        Self {
            default: StateTokens::default(),
            hover: StateTokens::default(),
            active: StateTokens::default(),
            selected: StateTokens::default(),
        }
    }
}

impl Default for NavigationSubmenuTokens {
    fn default() -> Self {
        Self {
            background_color: TokenReference::new("color.background.secondary".to_string()),
            border_color: TokenReference::new("color.border.default".to_string()),
            shadow: TokenReference::new("shadow.dropdown".to_string()),
        }
    }
}

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
