/// CSS 生成和处理模块
///
/// 本模块提供 CSS 生成、处理和管理的核心功能，包括组件样式生成、CSS 对象表示、
/// 样式处理管道和变量管理等。它是主题系统中负责将设计令牌转换为实际 CSS 的关键部分。
///
/// # 子模块
///
/// - `component_style`: 组件样式生成器，用于创建组件特定的样式
/// - `dependency`: 样式依赖跟踪器，管理样式之间的依赖关系
/// - `generator`: CSS 生成器，将设计令牌转换为 CSS
/// - `object`: CSS 对象模型，提供 CSS 的结构化表示
/// - `pipeline`: 样式处理管道，定义样式处理流程
/// - `processor`: 样式处理器，处理和转换 CSS
/// - `variables`: CSS 变量管理，处理 CSS 自定义属性
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{CssGenerator, CssObject};
/// use css_in_rust::theme::core::token::simple_system::ThemeVariant;
///
/// // 创建 CSS 生成器
/// let mut generator = CssGenerator::new();
///
/// // 生成主题 CSS
/// let theme_css = generator.generate_theme_css().unwrap();
/// println!("生成的主题 CSS: {}", theme_css);
///
/// // 创建 CSS 对象
/// let mut css_object = CssObject::new();
/// css_object.add_property("color", "red");
/// css_object.add_property("font-size", "16px");
///
/// // 转换为 CSS 字符串
/// let css_string = css_object.to_string();
/// println!("CSS 字符串: {}", css_string);
/// ```
pub mod component_style;
pub mod dependency;
pub mod generator;
pub mod object;
pub mod pipeline;
pub mod processor;
pub mod variables;

// Re-exports
/// 从 component_style 模块重新导出的类型和函数
pub use component_style::{
    create_button_style_generator, ComponentStyleConfig, ComponentStyleGenerator,
    DefaultComponentStyleGenerator,
};
/// 从 dependency 模块重新导出的 DependencyTracker
pub use dependency::DependencyTracker;
/// 从 generator 模块重新导出的 CssGenerator
pub use generator::CssGenerator;
/// 从 object 模块重新导出的类型
pub use object::{CssObject, CssValue};
/// 从 pipeline 模块重新导出的类型
pub use pipeline::{ProcessedStyle, StylePipeline, StylePipelineBuilder};
/// 从 processor 模块重新导出的 StyleProcessor
pub use processor::StyleProcessor;
/// 从 variables 模块重新导出的 CssVariables
pub use variables::CssVariables;
