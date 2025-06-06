use std::fmt;

/// CSS 计算器，用于生成 CSS calc() 表达式
pub struct CssCalculator {
    expression: String,
}

impl CssCalculator {
    /// 创建新的 CSS 计算器
    pub fn new(initial_value: impl fmt::Display) -> Self {
        Self {
            expression: initial_value.to_string(),
        }
    }

    /// 添加值
    pub fn add(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} + {}", self.expression, value);
        self
    }

    /// 减去值
    pub fn subtract(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} - {}", self.expression, value);
        self
    }

    /// 乘以值
    pub fn multiply(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} * {}", self.expression, value);
        self
    }

    /// 除以值
    pub fn divide(mut self, value: impl fmt::Display) -> Self {
        self.expression = format!("{} / {}", self.expression, value);
        self
    }

    /// 添加自定义表达式
    pub fn add_expression(mut self, expr: impl fmt::Display) -> Self {
        self.expression = format!("{} + ({})", self.expression, expr);
        self
    }

    /// 减去自定义表达式
    pub fn subtract_expression(mut self, expr: impl fmt::Display) -> Self {
        self.expression = format!("{} - ({})", self.expression, expr);
        self
    }

    /// 生成 CSS calc() 函数
    pub fn calc(&self) -> String {
        format!("calc({})", self.expression)
    }

    /// 获取表达式字符串
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
