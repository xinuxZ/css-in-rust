/// CSS 计算工具模块
///
/// 本模块提供 CSS 计算相关的工具函数和类型，包括生成 CSS `calc()` 表达式、
/// 变量引用、数值计算和单位转换等功能。
///
/// # 子模块
///
/// - `calculator`: CSS 计算器，用于处理 CSS 表达式
/// - `num_calculator`: 数值计算器，处理数值运算
/// - `unit_converter`: 单位转换器，处理不同 CSS 单位之间的转换
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::{gen_calc, gen_var, gen_var_with_default};
///
/// // 生成 calc() 表达式
/// let width = gen_calc("100% - 20px");
/// assert_eq!(width, "calc(100% - 20px)");
///
/// // 生成变量引用
/// let color = gen_var("primary-color");
/// assert_eq!(color, "var(--primary-color)");
///
/// // 生成带默认值的变量引用
/// let fallback = gen_var_with_default("font-size", "16px");
/// assert_eq!(fallback, "var(--font-size, 16px)");
/// ```
mod calculator;
mod num_calculator;
pub mod unit_converter;

/// 从 calculator 模块导出的 CssCalculator
pub use calculator::CssCalculator;
/// 从 num_calculator 模块导出的 NumCalculator
pub use num_calculator::NumCalculator;
/// 从 unit_converter 模块导出的 UnitConverter
pub use unit_converter::UnitConverter;

/// 生成 CSS 计算表达式
///
/// 创建一个 CSS `calc()` 函数表达式，用于在 CSS 中进行数学计算。
///
/// # 参数
///
/// * `expr` - 计算表达式，如 "100% - 20px" 或 "2 * var(--spacing)"
///
/// # 返回值
///
/// 返回格式化的 CSS `calc()` 表达式字符串。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::gen_calc;
///
/// // 基本计算
/// let width = gen_calc("100% - 20px");
/// assert_eq!(width, "calc(100% - 20px)");
///
/// // 复杂计算
/// let margin = gen_calc("var(--spacing) * 2 + 10px");
/// assert_eq!(margin, "calc(var(--spacing) * 2 + 10px)");
///
/// // 嵌套计算
/// let padding = gen_calc("var(--base-spacing) / 2");
/// assert_eq!(padding, "calc(var(--base-spacing) / 2)");
/// ```
pub fn gen_calc(expr: &str) -> String {
    format!("calc({})", expr)
}

/// 生成 CSS 变量引用
///
/// 创建一个 CSS 自定义属性（变量）引用，用于在 CSS 中使用变量。
///
/// # 参数
///
/// * `name` - 变量名称（不包含 `--` 前缀）
///
/// # 返回值
///
/// 返回格式化的 CSS `var()` 函数字符串。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::gen_var;
///
/// // 颜色变量
/// let primary_color = gen_var("primary-color");
/// assert_eq!(primary_color, "var(--primary-color)");
///
/// // 尺寸变量
/// let font_size = gen_var("font-size-base");
/// assert_eq!(font_size, "var(--font-size-base)");
///
/// // 在样式中使用
/// let button_style = format!("color: {}; font-size: {};",
///     gen_var("primary-color"),
///     gen_var("font-size-base")
/// );
/// ```
pub fn gen_var(name: &str) -> String {
    format!("var(--{})", name)
}

/// 生成带默认值的 CSS 变量引用
///
/// 创建一个带有回退值的 CSS 自定义属性（变量）引用，
/// 当变量未定义时将使用提供的默认值。
///
/// # 参数
///
/// * `name` - 变量名称（不包含 `--` 前缀）
/// * `default_value` - 当变量未定义时使用的默认值
///
/// # 返回值
///
/// 返回格式化的带默认值的 CSS `var()` 函数字符串。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::gen_var_with_default;
///
/// // 带默认颜色的变量
/// let color = gen_var_with_default("primary-color", "#1890ff");
/// assert_eq!(color, "var(--primary-color, #1890ff)");
///
/// // 带默认尺寸的变量
/// let spacing = gen_var_with_default("spacing", "8px");
/// assert_eq!(spacing, "var(--spacing, 8px)");
///
/// // 在样式中使用
/// let text_style = format!("color: {}; margin: {};",
///     gen_var_with_default("text-color", "#333"),
///     gen_var_with_default("text-margin", "16px")
/// );
/// ```
pub fn gen_var_with_default(name: &str, default_value: &str) -> String {
    format!("var(--{}, {})", name, default_value)
}
