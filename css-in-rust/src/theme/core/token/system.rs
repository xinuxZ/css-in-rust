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

use super::css_generator::CssGenerator;
use super::definitions::{
    ColorValue, DimensionValue, ThemeVariant, TokenPath, TokenValue, TypographyValue,
};
use std::collections::HashMap;

/// 设计令牌系统配置
///
/// 用于配置设计令牌系统的行为，如CSS变量前缀和是否启用压缩。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::TokenSystemConfig;
///
/// let config = TokenSystemConfig {
///     prefix: "app".to_string(),
///     minify: true,
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct TokenSystemConfig {
    /// 前缀
    ///
    /// 用于生成CSS变量时的前缀，例如 `--prefix-color-primary`
    pub prefix: String,
    /// 是否启用压缩
    ///
    /// 如果为true，生成的CSS将被压缩以减小体积
    pub minify: bool,
}

/// 系统元数据
///
/// 包含设计令牌系统的描述性信息，如名称、版本和描述。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::SystemMetadata;
///
/// let metadata = SystemMetadata {
///     name: "企业主题".to_string(),
///     version: "1.0.0".to_string(),
///     description: "公司标准设计系统".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct SystemMetadata {
    /// 系统名称
    pub name: String,
    /// 系统版本
    pub version: String,
    /// 系统描述
    pub description: String,
}

/// 全局令牌
///
/// 存储最基础的设计决策，如原始颜色、尺寸和排版值。
/// 这些是设计系统中最原子的单元，通常由设计师直接定义。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::GlobalTokens;
/// use css_in_rust::theme::core::token::{ColorValue, DimensionValue};
/// use std::collections::HashMap;
///
/// let mut global_tokens = GlobalTokens::default();
///
/// // 添加颜色令牌
/// let mut colors = HashMap::new();
/// colors.insert("blue-500".to_string(), ColorValue::hex("#1890ff"));
/// colors.insert("red-500".to_string(), ColorValue::hex("#f5222d"));
/// global_tokens.colors = colors;
///
/// // 添加尺寸令牌
/// let mut dimensions = HashMap::new();
/// dimensions.insert("spacing-md".to_string(), DimensionValue::px(16.0));
/// global_tokens.dimensions = dimensions;
/// ```
#[derive(Debug, Clone, Default)]
pub struct GlobalTokens {
    /// 颜色
    ///
    /// 存储原始颜色值，如品牌色、中性色等
    pub colors: HashMap<String, ColorValue>,
    /// 尺寸
    ///
    /// 存储原始尺寸值，如间距、边框半径等
    pub dimensions: HashMap<String, DimensionValue>,
    /// 排版
    ///
    /// 存储原始排版值，如字体大小、行高等
    pub typography: HashMap<String, TypographyValue>,
}

/// 别名令牌
///
/// 提供语义化的令牌引用，将全局令牌映射到有意义的名称上。
/// 例如，将 `blue-500` 映射为 `primary`，使设计意图更加明确。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::AliasTokens;
/// use std::collections::HashMap;
///
/// let mut alias_tokens = AliasTokens::default();
///
/// // 创建颜色别名
/// let mut colors = HashMap::new();
/// colors.insert("primary".to_string(), "blue-500".to_string());
/// colors.insert("danger".to_string(), "red-500".to_string());
/// alias_tokens.colors = colors;
///
/// // 创建尺寸别名
/// let mut dimensions = HashMap::new();
/// dimensions.insert("spacing-standard".to_string(), "spacing-md".to_string());
/// alias_tokens.dimensions = dimensions;
/// ```
#[derive(Debug, Clone, Default)]
pub struct AliasTokens {
    /// 颜色别名
    ///
    /// 将全局颜色令牌映射到语义化名称
    pub colors: HashMap<String, String>,
    /// 尺寸别名
    ///
    /// 将全局尺寸令牌映射到语义化名称
    pub dimensions: HashMap<String, String>,
    /// 排版别名
    ///
    /// 将全局排版令牌映射到语义化名称
    pub typography: HashMap<String, String>,
}

/// 组件令牌
///
/// 存储特定组件的令牌，这些令牌通常引用全局或别名令牌。
/// 组件令牌使设计系统能够为不同组件提供一致但独特的外观。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::ComponentTokens;
/// use std::collections::HashMap;
///
/// let mut component_tokens = ComponentTokens::default();
///
/// // 创建按钮组件的令牌
/// let mut button_tokens = HashMap::new();
/// button_tokens.insert("background".to_string(), "colors.primary".to_string());
/// button_tokens.insert("padding".to_string(), "dimensions.spacing-standard".to_string());
///
/// // 添加到组件令牌中
/// component_tokens.components.insert("button".to_string(), button_tokens);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ComponentTokens {
    /// 组件特定令牌
    ///
    /// 按组件名称组织的令牌映射
    pub components: HashMap<String, HashMap<String, String>>,
}

/// 计算规则
///
/// 定义令牌值之间的计算关系，允许基于其他令牌动态生成值。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::ComputationRules;
/// use std::collections::HashMap;
///
/// let mut computation_rules = ComputationRules::default();
///
/// // 添加计算规则
/// let mut rules = HashMap::new();
/// rules.insert(
///     "button.hover.background".to_string(),
///     "darken(colors.primary, 10%)".to_string()
/// );
/// computation_rules.rules = rules;
/// ```
#[derive(Debug, Clone, Default)]
pub struct ComputationRules {
    /// 规则映射
    ///
    /// 键是目标令牌路径，值是计算表达式
    pub rules: HashMap<String, String>,
}

/// 设计令牌系统
///
/// 完整的设计令牌管理系统，整合了令牌定义、存储、解析和CSS生成功能。
/// 提供了一个统一的接口来管理设计系统中的所有令牌，并生成相应的CSS变量和类。
///
/// 设计令牌系统采用分层架构：
/// - 全局令牌：定义基础设计决策
/// - 别名令牌：提供语义化引用
/// - 组件令牌：定义组件特定样式
///
/// 此外，系统还支持：
/// - 多主题管理和切换
/// - 令牌值的计算和转换
/// - 生成CSS变量和工具类
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::token::system::DesignTokenSystem;
/// use css_in_rust::theme::core::token::definitions::{TokenValue, ThemeVariant};
///
/// // 创建设计令牌系统
/// let mut system = DesignTokenSystem::new()
///     .with_prefix("app")
///     .with_name("企业主题")
///     .with_version("1.0.0");
///
/// // 设置令牌值
/// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
/// system.set_token("spacing.md", TokenValue::Dimension(16.0, "px".to_string())).unwrap();
///
/// // 生成CSS变量
/// let css = system.generate_css_variables().unwrap();
/// println!("CSS变量: {}", css);
///
/// // 切换主题
/// system.switch_theme(ThemeVariant::Dark);
///
/// // 为暗色主题设置不同的值
/// system.set_token("colors.primary", TokenValue::Color("#177ddc".to_string())).unwrap();
///
/// // 生成暗色主题的CSS
/// let dark_css = system.generate_css_variables().unwrap();
/// println!("暗色主题CSS变量: {}", dark_css);
/// ```
#[derive(Debug, Clone)]
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
    pub current_theme: ThemeVariant,
    /// 系统配置
    pub config: TokenSystemConfig,
    /// 系统元数据
    pub metadata: SystemMetadata,
}

impl Default for DesignTokenSystem {
    fn default() -> Self {
        Self {
            global_tokens: GlobalTokens::default(),
            alias_tokens: AliasTokens::default(),
            component_tokens: ComponentTokens::default(),
            computation_rules: ComputationRules::default(),
            css_generator: CssGenerator::default(),
            current_theme: ThemeVariant::Light,
            config: TokenSystemConfig::default(),
            metadata: SystemMetadata::default(),
        }
    }
}

impl DesignTokenSystem {
    /// 创建新的设计令牌系统
    ///
    /// 初始化一个带有默认配置的设计令牌系统。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `DesignTokenSystem` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置前缀
    ///
    /// 设置生成CSS变量时使用的前缀。
    ///
    /// # 参数
    ///
    /// * `prefix` - CSS变量的前缀，如 "app" 将生成 "--app-color-primary" 这样的变量
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_prefix("app");
    /// ```
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.prefix = prefix.into();
        self
    }

    /// 设置是否启用压缩
    ///
    /// 配置生成的CSS是否应该被压缩。
    ///
    /// # 参数
    ///
    /// * `minify` - 是否启用CSS压缩，`true` 表示启用，`false` 表示不启用
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_minify(true); // 启用CSS压缩
    /// ```
    pub fn with_minify(mut self, minify: bool) -> Self {
        self.config.minify = minify;
        self
    }

    /// 设置系统名称
    ///
    /// 设置设计令牌系统的名称。
    ///
    /// # 参数
    ///
    /// * `name` - 系统名称，如 "企业主题" 或 "产品设计系统"
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_name("企业主题");
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.metadata.name = name.into();
        self
    }

    /// 设置系统版本
    ///
    /// 设置设计令牌系统的版本号。
    ///
    /// # 参数
    ///
    /// * `version` - 系统版本号，如 "1.0.0" 或 "2.3.1"
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_version("1.0.0");
    /// ```
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.metadata.version = version.into();
        self
    }

    /// 设置系统描述
    ///
    /// 设置设计令牌系统的描述信息。
    ///
    /// # 参数
    ///
    /// * `description` - 系统描述，如 "公司标准设计系统，包含颜色、排版和间距等基础设计元素"
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_description("公司标准设计系统，包含颜色、排版和间距等基础设计元素");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = description.into();
        self
    }

    /// 设置当前主题
    ///
    /// 设置设计令牌系统的当前主题变体。
    ///
    /// # 参数
    ///
    /// * `theme` - 主题变体，如 `ThemeVariant::Light` 或 `ThemeVariant::Dark`
    ///
    /// # 返回值
    ///
    /// 返回配置更新后的 `DesignTokenSystem` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::ThemeVariant;
    ///
    /// let system = DesignTokenSystem::new()
    ///     .with_theme(ThemeVariant::Dark); // 初始化为暗色主题
    /// ```
    pub fn with_theme(mut self, theme: ThemeVariant) -> Self {
        self.current_theme = theme;
        self
    }

    /// 生成CSS变量
    ///
    /// 根据当前主题生成CSS自定义属性（变量）。
    ///
    /// # 返回值
    ///
    /// 如果生成成功，返回包含CSS变量定义的字符串；如果生成失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = DesignTokenSystem::new();
    /// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
    ///
    /// // 生成CSS变量
    /// let css = system.generate_css_variables().unwrap();
    /// // 输出类似：":root { --color-primary: #1890ff; }"
    /// ```
    pub fn generate_css_variables(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_css_variables(self.current_theme)
    }

    /// 生成主题CSS
    ///
    /// 生成完整的主题CSS，包括CSS变量和基础样式。
    ///
    /// # 返回值
    ///
    /// 如果生成成功，返回包含主题CSS的字符串；如果生成失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = DesignTokenSystem::new();
    /// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
    /// system.set_token("colors.text", TokenValue::Color("#333333".to_string())).unwrap();
    ///
    /// // 生成主题CSS
    /// let css = system.generate_theme_css().unwrap();
    /// // 输出包含CSS变量和基础样式的完整CSS
    /// ```
    pub fn generate_theme_css(&mut self) -> Result<String, String> {
        self.css_generator.generate_theme_css()
    }

    /// 生成组件CSS
    ///
    /// 为指定组件生成CSS类。
    ///
    /// # 参数
    ///
    /// * `component` - 组件名称，如 "button" 或 "card"
    ///
    /// # 返回值
    ///
    /// 如果生成成功，返回包含组件CSS类的字符串；如果生成失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    /// use std::collections::HashMap;
    ///
    /// let mut system = DesignTokenSystem::new();
    ///
    /// // 设置按钮组件的令牌
    /// let mut button_tokens = HashMap::new();
    /// button_tokens.insert("background".to_string(), "colors.primary".to_string());
    /// button_tokens.insert("padding".to_string(), "8px 16px".to_string());
    /// system.component_tokens.components.insert("button".to_string(), button_tokens);
    ///
    /// // 生成按钮组件的CSS
    /// let css = system.generate_component_css("button").unwrap();
    /// // 输出按钮组件的CSS类
    /// ```
    pub fn generate_component_css(&mut self, component: &str) -> Result<String, String> {
        self.css_generator
            .generate_component_classes(component, self.current_theme)
    }

    /// 生成工具类CSS
    ///
    /// 生成基于设计令牌的工具类CSS，如颜色、间距和排版工具类。
    ///
    /// # 返回值
    ///
    /// 如果生成成功，返回包含工具类CSS的字符串；如果生成失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = DesignTokenSystem::new();
    /// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
    /// system.set_token("spacing.md", TokenValue::Dimension(16.0, "px".to_string())).unwrap();
    ///
    /// // 生成工具类CSS
    /// let css = system.generate_utility_classes().unwrap();
    /// // 输出工具类CSS，如 .text-primary, .bg-primary, .p-md 等
    /// ```
    pub fn generate_utility_classes(&mut self) -> Result<String, String> {
        self.css_generator
            .generate_utility_classes(self.current_theme)
    }

    /// 解析令牌
    ///
    /// 解析指定路径的令牌值，处理引用和计算。
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，如 "colors.primary" 或 "spacing.md"
    ///
    /// # 返回值
    ///
    /// 如果解析成功，返回解析后的令牌值；如果解析失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::TokenValue;
    ///
    /// let mut system = DesignTokenSystem::new();
    /// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
    /// system.set_token("colors.link", TokenValue::Reference("colors.primary".to_string())).unwrap();
    ///
    /// // 解析引用令牌
    /// let value = system.resolve_token("colors.link").unwrap();
    /// // value 应该是 TokenValue::Color("#1890ff")
    /// ```
    pub fn resolve_token(&self, path: &str) -> Result<TokenValue, String> {
        match self
            .css_generator
            .get_resolver()
            .resolve_token(path, self.current_theme)
        {
            Ok(value) => Ok(value),
            Err(e) => Err(e.to_string()),
        }
    }

    /// 设置令牌值
    ///
    /// 设置指定路径的令牌值。
    ///
    /// # 参数
    ///
    /// * `path` - 令牌路径，如 "colors.primary" 或 "spacing.md"
    /// * `value` - 要设置的令牌值
    ///
    /// # 返回值
    ///
    /// 如果设置成功，返回 `Ok(())`；如果设置失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, ThemeVariant};
    ///
    /// let mut system = DesignTokenSystem::new();
    ///
    /// // 设置亮色主题的主色
    /// system.set_token("colors.primary", TokenValue::Color("#1890ff".to_string())).unwrap();
    ///
    /// // 切换到暗色主题
    /// system.switch_theme(ThemeVariant::Dark);
    ///
    /// // 设置暗色主题的主色
    /// system.set_token("colors.primary", TokenValue::Color("#177ddc".to_string())).unwrap();
    /// ```
    pub fn set_token(&mut self, path: &str, value: TokenValue) -> Result<(), String> {
        let token_path = TokenPath::from_str(path);
        match self.css_generator.get_resolver_mut().set_token(
            &token_path,
            value,
            self.current_theme,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// 切换主题
    ///
    /// 切换设计令牌系统的当前主题。
    ///
    /// # 参数
    ///
    /// * `theme` - 要切换到的主题变体
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::token::system::DesignTokenSystem;
    /// use css_in_rust::theme::core::token::definitions::{TokenValue, ThemeVariant};
    ///
    /// let mut system = DesignTokenSystem::new();
    ///
    /// // 设置亮色主题的值
    /// system.set_token("colors.background", TokenValue::Color("#ffffff".to_string())).unwrap();
    ///
    /// // 切换到暗色主题
    /// system.switch_theme(ThemeVariant::Dark);
    ///
    /// // 设置暗色主题的值
    /// system.set_token("colors.background", TokenValue::Color("#121212".to_string())).unwrap();
    ///
    /// // 生成当前主题（暗色）的CSS
    /// let css = system.generate_css_variables().unwrap();
    /// // css 应该包含 "--color-background: #121212;"
    /// ```
    pub fn switch_theme(&mut self, theme: ThemeVariant) {
        self.current_theme = theme;
        // 清空缓存以确保使用新主题的值
        self.css_generator.get_resolver_mut().clear_cache();
    }
}
