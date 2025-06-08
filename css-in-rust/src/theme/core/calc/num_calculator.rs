/// 数值计算器，用于主题中的数值计算
///
/// 该结构体提供了一个流畅的API，用于进行数值计算操作，
/// 如加、减、乘、除、最大值、最小值、四舍五入等，
/// 并可以将结果转换为不同格式。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::NumCalculator;
///
/// // 基本计算
/// let result = NumCalculator::new(10.0)
///     .add(5.0)
///     .multiply(2.0)
///     .value();
/// assert_eq!(result, 30.0);
///
/// // 转换为带单位的字符串
/// let size = NumCalculator::new(10.0).multiply(1.5).to_string_with_unit("px");
/// assert_eq!(size, "15px");
/// ```
pub struct NumCalculator {
    value: f64,
}

impl NumCalculator {
    /// 创建新的数值计算器
    ///
    /// 初始化一个数值计算器，设置初始值。
    ///
    /// # 参数
    ///
    /// * `initial_value` - 初始数值
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `NumCalculator` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// // 创建初始值为10的计算器
    /// let calc = NumCalculator::new(10.0);
    ///
    /// // 创建初始值为0的计算器
    /// let zero_calc = NumCalculator::new(0.0);
    /// ```
    pub fn new(initial_value: f64) -> Self {
        Self {
            value: initial_value,
        }
    }

    /// 添加值
    ///
    /// 向当前值添加一个数值。
    ///
    /// # 参数
    ///
    /// * `value` - 要添加的数值
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).add(5.0);
    /// assert_eq!(calc.value(), 15.0);
    ///
    /// // 链式调用
    /// let calc = NumCalculator::new(10.0).add(5.0).add(3.0);
    /// assert_eq!(calc.value(), 18.0);
    /// ```
    pub fn add(mut self, value: f64) -> Self {
        self.value += value;
        self
    }

    /// 减去值
    ///
    /// 从当前值中减去一个数值。
    ///
    /// # 参数
    ///
    /// * `value` - 要减去的数值
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).subtract(3.0);
    /// assert_eq!(calc.value(), 7.0);
    ///
    /// // 链式调用
    /// let calc = NumCalculator::new(20.0).subtract(5.0).subtract(3.0);
    /// assert_eq!(calc.value(), 12.0);
    /// ```
    pub fn subtract(mut self, value: f64) -> Self {
        self.value -= value;
        self
    }

    /// 乘以值
    ///
    /// 将当前值乘以一个数值。
    ///
    /// # 参数
    ///
    /// * `value` - 乘数
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).multiply(2.0);
    /// assert_eq!(calc.value(), 20.0);
    ///
    /// // 链式调用
    /// let calc = NumCalculator::new(5.0).multiply(2.0).multiply(3.0);
    /// assert_eq!(calc.value(), 30.0);
    /// ```
    pub fn multiply(mut self, value: f64) -> Self {
        self.value *= value;
        self
    }

    /// 除以值
    ///
    /// 将当前值除以一个数值。如果除数为零，则不进行操作。
    ///
    /// # 参数
    ///
    /// * `value` - 除数
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).divide(2.0);
    /// assert_eq!(calc.value(), 5.0);
    ///
    /// // 除以零不会改变值
    /// let calc = NumCalculator::new(10.0).divide(0.0);
    /// assert_eq!(calc.value(), 10.0);
    /// ```
    pub fn divide(mut self, value: f64) -> Self {
        if value != 0.0 {
            self.value /= value;
        }
        self
    }

    /// 取最小值
    ///
    /// 将当前值与给定值比较，取较小的一个。
    ///
    /// # 参数
    ///
    /// * `value` - 比较值
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).min(5.0);
    /// assert_eq!(calc.value(), 5.0);
    ///
    /// // 当前值已经更小
    /// let calc = NumCalculator::new(3.0).min(8.0);
    /// assert_eq!(calc.value(), 3.0);
    /// ```
    pub fn min(mut self, value: f64) -> Self {
        self.value = self.value.min(value);
        self
    }

    /// 取最大值
    ///
    /// 将当前值与给定值比较，取较大的一个。
    ///
    /// # 参数
    ///
    /// * `value` - 比较值
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).max(15.0);
    /// assert_eq!(calc.value(), 15.0);
    ///
    /// // 当前值已经更大
    /// let calc = NumCalculator::new(12.0).max(8.0);
    /// assert_eq!(calc.value(), 12.0);
    /// ```
    pub fn max(mut self, value: f64) -> Self {
        self.value = self.value.max(value);
        self
    }

    /// 四舍五入
    ///
    /// 将当前值四舍五入到最接近的整数。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.4).round();
    /// assert_eq!(calc.value(), 10.0);
    ///
    /// let calc = NumCalculator::new(10.6).round();
    /// assert_eq!(calc.value(), 11.0);
    /// ```
    pub fn round(mut self) -> Self {
        self.value = self.value.round();
        self
    }

    /// 向上取整
    ///
    /// 将当前值向上取整到最接近的整数。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.1).ceil();
    /// assert_eq!(calc.value(), 11.0);
    ///
    /// let calc = NumCalculator::new(10.9).ceil();
    /// assert_eq!(calc.value(), 11.0);
    /// ```
    pub fn ceil(mut self) -> Self {
        self.value = self.value.ceil();
        self
    }

    /// 向下取整
    ///
    /// 将当前值向下取整到最接近的整数。
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.1).floor();
    /// assert_eq!(calc.value(), 10.0);
    ///
    /// let calc = NumCalculator::new(10.9).floor();
    /// assert_eq!(calc.value(), 10.0);
    /// ```
    pub fn floor(mut self) -> Self {
        self.value = self.value.floor();
        self
    }

    /// 设置精度
    ///
    /// 将当前值四舍五入到指定的小数位数。
    ///
    /// # 参数
    ///
    /// * `precision` - 小数位数
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `NumCalculator` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.123456).precision(2);
    /// assert_eq!(calc.value(), 10.12);
    ///
    /// let calc = NumCalculator::new(10.678).precision(1);
    /// assert_eq!(calc.value(), 10.7);
    /// ```
    pub fn precision(mut self, precision: u32) -> Self {
        let factor = 10.0_f64.powi(precision as i32);
        self.value = (self.value * factor).round() / factor;
        self
    }

    /// 获取计算结果
    ///
    /// 返回当前计算的结果值。
    ///
    /// # 返回值
    ///
    /// 返回当前的数值结果。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.0).add(5.0).multiply(2.0);
    /// assert_eq!(calc.value(), 30.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// 转换为整数
    ///
    /// 将当前值转换为32位整数，截断小数部分。
    ///
    /// # 返回值
    ///
    /// 返回转换后的整数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.7);
    /// assert_eq!(calc.to_int(), 10);
    ///
    /// let calc = NumCalculator::new(-5.3);
    /// assert_eq!(calc.to_int(), -5);
    /// ```
    pub fn to_int(&self) -> i32 {
        self.value as i32
    }

    /// 转换为字符串
    ///
    /// 将当前值转换为字符串表示。
    ///
    /// # 返回值
    ///
    /// 返回表示当前值的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// let calc = NumCalculator::new(10.5);
    /// assert_eq!(calc.to_string(), "10.5");
    /// ```
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    /// 转换为带单位的字符串
    ///
    /// 将当前值转换为带指定单位的字符串。
    /// 如果值是整数，则不显示小数点。
    ///
    /// # 参数
    ///
    /// * `unit` - 要附加的单位字符串，如 "px", "em", "%"
    ///
    /// # 返回值
    ///
    /// 返回带单位的字符串表示。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::NumCalculator;
    ///
    /// // 整数值
    /// let calc = NumCalculator::new(10.0);
    /// assert_eq!(calc.to_string_with_unit("px"), "10px");
    ///
    /// // 小数值
    /// let calc = NumCalculator::new(10.5);
    /// assert_eq!(calc.to_string_with_unit("em"), "10.5em");
    ///
    /// // 百分比
    /// let calc = NumCalculator::new(50.0);
    /// assert_eq!(calc.to_string_with_unit("%"), "50%");
    /// ```
    pub fn to_string_with_unit(&self, unit: &str) -> String {
        // 如果值是整数，则不显示小数点
        if self.value == (self.value as i64) as f64 {
            return format!("{}{}", self.value as i64, unit);
        }
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

        assert_eq!(calc.to_string_with_unit("px"), "15px");
    }
}
