/// 数值计算器，用于主题中的数值计算
pub struct NumCalculator {
    value: f64,
}

impl NumCalculator {
    /// 创建新的数值计算器
    pub fn new(initial_value: f64) -> Self {
        Self {
            value: initial_value,
        }
    }

    /// 添加值
    pub fn add(mut self, value: f64) -> Self {
        self.value += value;
        self
    }

    /// 减去值
    pub fn subtract(mut self, value: f64) -> Self {
        self.value -= value;
        self
    }

    /// 乘以值
    pub fn multiply(mut self, value: f64) -> Self {
        self.value *= value;
        self
    }

    /// 除以值
    pub fn divide(mut self, value: f64) -> Self {
        if value != 0.0 {
            self.value /= value;
        }
        self
    }

    /// 取最小值
    pub fn min(mut self, value: f64) -> Self {
        self.value = self.value.min(value);
        self
    }

    /// 取最大值
    pub fn max(mut self, value: f64) -> Self {
        self.value = self.value.max(value);
        self
    }

    /// 四舍五入
    pub fn round(mut self) -> Self {
        self.value = self.value.round();
        self
    }

    /// 向上取整
    pub fn ceil(mut self) -> Self {
        self.value = self.value.ceil();
        self
    }

    /// 向下取整
    pub fn floor(mut self) -> Self {
        self.value = self.value.floor();
        self
    }

    /// 设置精度
    pub fn precision(mut self, precision: u32) -> Self {
        let factor = 10.0_f64.powi(precision as i32);
        self.value = (self.value * factor).round() / factor;
        self
    }

    /// 获取计算结果
    pub fn value(&self) -> f64 {
        self.value
    }

    /// 转换为整数
    pub fn to_int(&self) -> i32 {
        self.value as i32
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    /// 转换为带单位的字符串
    pub fn to_string_with_unit(&self, unit: &str) -> String {
        format!("{}{}", self.value, unit)
    }
}

impl From<NumCalculator> for f64 {
    fn from(calc: NumCalculator) -> Self {
        calc.value
    }
}

impl From<NumCalculator> for String {
    fn from(calc: NumCalculator) -> Self {
        calc.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_calculator() {
        let calc = NumCalculator::new(10.0)
            .add(5.0)
            .multiply(2.0)
            .subtract(3.0);

        assert_eq!(calc.value(), 27.0);
    }

    #[test]
    fn test_precision() {
        let calc = NumCalculator::new(10.0).divide(3.0).precision(2);

        assert_eq!(calc.value(), 3.33);
    }

    #[test]
    fn test_unit_conversion() {
        let calc = NumCalculator::new(10.0).multiply(1.5).precision(1);

        assert_eq!(calc.to_string_with_unit("px"), "15.0px");
    }
}
