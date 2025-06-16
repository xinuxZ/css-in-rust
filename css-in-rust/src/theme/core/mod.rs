/// 主题系统核心模块
///
/// 本模块包含主题系统的核心功能，包括主题定义、样式生成、缓存管理、计算工具等。
/// 它是 CSS-in-Rust 主题系统的基础，提供了一套完整的主题管理和样式处理工具。
///
/// # 子模块
///
/// - `cache`: 提供主题和样式的缓存功能，优化性能
/// - `calc`: CSS 计算工具，处理单位转换和数值计算
/// - `css`: CSS 生成和处理工具，包括样式对象、管道和变量处理
/// - `manager`: 主题管理器，负责主题切换和状态管理
/// - `optimize`: 样式优化工具，减小生成的 CSS 体积
/// - `ssr`: 服务端渲染支持，提供样式提取和注水功能
/// - `token`: 设计令牌系统，定义和管理主题变量
/// - `transform`: CSS 转换工具，如逻辑属性转换和单位转换
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::ThemeManager;
/// use css_in_rust::theme::core::manager::ThemeManagerConfig;
/// use css_in_rust::theme::theme_types::{Theme, ThemeMode};
///
/// // 创建主题管理器
/// let config = ThemeManagerConfig::default();
/// let manager = ThemeManager::new(config);
///
/// // 创建并设置主题
/// let dark_theme = Theme::new("dark").with_mode(ThemeMode::Dark);
/// manager.set_theme(dark_theme).unwrap();
///
/// // 获取当前主题
/// if let Some(theme) = manager.get_current_theme() {
///     println!("当前主题: {}, 模式: {:?}", theme.name, theme.mode);
/// }
/// ```
pub mod cache;
pub mod calc;
pub mod css;
pub mod manager;
pub mod optimize;
pub mod ssr;
pub mod token;
pub mod transform;

// Re-exports
/// 从 token 模块重新导出的类型和函数
pub use token::{
    definitions::{
        DimensionUnit, DimensionValue, ThemeVariant, TokenDefinitions, TokenDefinitionsImpl,
        TokenMetadata,
    },
    values::TokenValues,
};

/// 从 css 模块重新导出的类型和函数
pub use css::{
    generator::CssGenerator,
    object::{CssObject, CssValue},
    pipeline::{ProcessedStyle, StylePipeline, StylePipelineBuilder},
    processor::StyleProcessor,
    variables::CssVariables,
};

/// 从 cache 模块重新导出的类型和函数
pub use cache::cache_manager::{CacheManager, MemoryUsage};
pub use cache::StyleCache;

/// 从 manager 模块重新导出的 ThemeManager
pub use manager::ThemeManager;

/// 从 optimize 模块重新导出的类型和函数
pub use optimize::{OptimizeConfig, StyleOptimizer};

/// 从 transform 模块重新导出的类型和函数
pub use transform::{
    LogicalPropertiesTransformer, Px2RemTransformer, Transformer, TransformerRegistry,
};

/// 从 calc 模块重新导出的类型和函数
pub use calc::{
    gen_calc, gen_var, gen_var_with_default, CssCalculator, NumCalculator, UnitConverter,
};

/// 从 ssr 模块重新导出的类型和函数
pub use ssr::{ServerStyleSheet, StyleExtractor, StyleHydration, StyleSheetManager};
