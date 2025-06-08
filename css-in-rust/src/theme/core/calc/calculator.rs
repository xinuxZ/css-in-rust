use std::fmt;

/// CSS 计算器，用于生成 CSS calc() 表达式
///
/// 该结构体提供了一个流畅的API，用于构建CSS计算表达式，
/// 可以进行加、减、乘、除等操作，并最终生成标准的CSS calc()函数。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::CssCalculator;
///
/// // 创建一个基本的计算表达式
/// let calc = CssCalculator::new("100%").subtract("20px");
/// assert_eq!(calc.calc(), "calc(100% - 20px)");
///
/// // 链式调用多个操作
/// let complex_calc = CssCalculator::new("100%")
///     .subtract("20px")
///     .divide("2")
///     .add("10px");
/// assert_eq!(complex_calc.calc(), "calc(100% - 20px / 2 + 10px)");
/// ```
pub struct CssCalculator {
    expression: String,
}

impl CssCalculator {
    /// 创建新的 CSS 计算器
    ///
    /// 初始化一个CSS计算器，设置表达式的起始值。
    ///
    /// # 参数
    ///
    /// * `initial_value` - 初始值，可以是任何实现了 `Display` trait 的类型，
    ///   如数字、字符串或其他可显示类型。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `CssCalculator` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// // 使用像素值初始化
    /// let calc1 = CssCalculator::new("100px");
    ///
    /// // 使用百分比初始化
    /// let calc2 = CssCalculator::new("50%");
    ///
    /// // 使用变量初始化
    /// let calc3 = CssCalculator::new("var(--spacing)");
    /// ```
    pub fn new(initial_value: impl fmt::Display) -> Self {
        Self {
            expression: initial_value.to_string(),
        }
    }

    /// 添加值
    ///
    /// 向当前表达式添加一个值。
    ///
    /// # 参数
    ///
    /// * `value` - 要添加的值，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100px").add("20px");
    /// assert_eq!(calc.calc(), "calc(100px + 20px)");
    ///
    /// // 链式调用
    /// let calc = CssCalculator::new("100px").add("20px").add("5px");
    /// assert_eq!(calc.calc(), "calc(100px + 20px + 5px)");
    /// ```
    pub fn add(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} + {}", self.expression, value);
        self
    }

    /// 减去值
    ///
    /// 从当前表达式中减去一个值。
    ///
    /// # 参数
    ///
    /// * `value` - 要减去的值，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100px").subtract("20px");
    /// assert_eq!(calc.calc(), "calc(100px - 20px)");
    ///
    /// // 链式调用
    /// let calc = CssCalculator::new("100%").subtract("20px").subtract("5px");
    /// assert_eq!(calc.calc(), "calc(100% - 20px - 5px)");
    /// ```
    pub fn subtract(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} - {}", self.expression, value);
        self
    }

    /// 乘以值
    ///
    /// 将当前表达式乘以一个值。
    ///
    /// # 参数
    ///
    /// * `value` - 乘数，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("10px").multiply("2");
    /// assert_eq!(calc.calc(), "calc(10px * 2)");
    ///
    /// // 与变量相乘
    /// let calc = CssCalculator::new("var(--spacing)").multiply("3");
    /// assert_eq!(calc.calc(), "calc(var(--spacing) * 3)");
    /// ```
    pub fn multiply(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} * {}", self.expression, value);
        self
    }

    /// 除以值
    ///
    /// 将当前表达式除以一个值。
    ///
    /// # 参数
    ///
    /// * `value` - 除数，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100px").divide("2");
    /// assert_eq!(calc.calc(), "calc(100px / 2)");
    ///
    /// // 复杂表达式
    /// let calc = CssCalculator::new("var(--container-width)").divide("3");
    /// assert_eq!(calc.calc(), "calc(var(--container-width) / 3)");
    /// ```
    pub fn divide(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} / {}", self.expression, value);
        self
    }

    /// 添加自定义表达式
    ///
    /// 向当前表达式添加一个括号包围的自定义表达式。
    /// 这对于添加复杂的子表达式很有用。
    ///
    /// # 参数
    ///
    /// * `expr` - 要添加的表达式，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100%").add_expression("var(--margin) * 2");
    /// assert_eq!(calc.calc(), "calc(100% + (var(--margin) * 2))");
    /// ```
    pub fn add_expression(mut self, expr: impl fmt::Display) -> Self {
        self.expression = format!("{} + ({})", self.expression, expr);
        self
    }

    /// 减去自定义表达式
    ///
    /// 从当前表达式中减去一个括号包围的自定义表达式。
    /// 这对于减去复杂的子表达式很有用。
    ///
    /// # 参数
    ///
    /// * `expr` - 要减去的表达式，可以是任何实现了 `Display` trait 的类型。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `CssCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100%").subtract_expression("var(--margin) * 2");
    /// assert_eq!(calc.calc(), "calc(100% - (var(--margin) * 2))");
    /// ```
    pub fn subtract_expression(mut self, expr: impl fmt::Display) -> Self {
        self.expression = format!("{} - ({})", self.expression, expr);
        self
    }

    /// 生成 CSS calc() 函数
    ///
    /// 将当前表达式包装在CSS calc()函数中，生成完整的CSS计算表达式。
    ///
    /// # 返回值
    ///
    /// 返回格式化的CSS calc()函数字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100%").subtract("20px");
    /// assert_eq!(calc.calc(), "calc(100% - 20px)");
    /// ```
    pub fn calc(&self) -> String {
        format!("calc({})", self.expression)
    }

    /// 获取表达式字符串
    ///
    /// 返回当前表达式的原始字符串，不包含calc()函数包装。
    ///
    /// # 返回值
    ///
    /// 返回当前表达式的字符串引用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssCalculator;
    ///
    /// let calc = CssCalculator::new("100%").subtract("20px");
    /// assert_eq!(calc.expression(), "100% - 20px");
    /// ```
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

impl fmt::Display for CssCalculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "calc({})", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_calculator() {
        let calc = CssCalculator::new("100%").subtract("20px").add("10px");

        assert_eq!(calc.calc(), "calc(100% - 20px + 10px)");
    }

    #[test]
    fn test_complex_expression() {
        let calc = CssCalculator::new("100%")
            .subtract_expression("var(--margin) * 2")
            .add("10px");

        assert_eq!(calc.calc(), "calc(100% - (var(--margin) * 2) + 10px)");
    }
}
