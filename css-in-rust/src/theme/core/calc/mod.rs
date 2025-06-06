mod calculator;
mod num_calculator;
pub mod unit_converter;

pub use calculator::CssCalculator;
pub use num_calculator::NumCalculator;
pub use unit_converter::UnitConverter;

/// 生成CSS计算表达式
///
/// 例如：`calc(100% - 20px)`
pub fn gen_calc(expr: &str) -> String {
    format!("calc({})", expr)
}

/// 生成CSS变量引用
///
/// 例如：`var(--primary-color)`
pub fn gen_var(name: &str) -> String {
    format!("var(--{})", name)
}

/// 生成CSS变量引用，带默认值
///
/// 例如：`var(--primary-color, #1890ff)`
pub fn gen_var_with_default(name: &str, default_value: &str) -> String {
    format!("var(--{}, {})", name, default_value)
}
