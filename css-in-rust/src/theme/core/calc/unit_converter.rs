use std::collections::HashMap;

/// CSS 单位类型
///
/// 表示CSS中常用的长度和尺寸单位，用于单位转换和计算。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::CssUnit;
///
/// let px_unit = CssUnit::Px;
/// assert_eq!(px_unit.to_str(), "px");
///
/// let rem_unit = CssUnit::from_str("rem");
/// assert_eq!(rem_unit, CssUnit::Rem);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssUnit {
    /// 像素
    Px,
    /// 相对于根元素的字体大小
    Rem,
    /// 相对于父元素的字体大小
    Em,
    /// 视口宽度的百分比
    Vw,
    /// 视口高度的百分比
    Vh,
    /// 百分比
    Percent,
    /// 无单位
    None,
}

impl CssUnit {
    /// 从字符串解析单位
    ///
    /// 将字符串表示的单位转换为 `CssUnit` 枚举值。
    /// 不区分大小写，支持常见的CSS单位。
    ///
    /// # 参数
    ///
    /// * `s` - 单位字符串，如 "px", "rem", "em", "vw", "vh", "%"
    ///
    /// # 返回值
    ///
    /// 返回对应的 `CssUnit` 枚举值，如果无法识别则返回 `CssUnit::None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssUnit;
    ///
    /// assert_eq!(CssUnit::from_str("px"), CssUnit::Px);
    /// assert_eq!(CssUnit::from_str("REM"), CssUnit::Rem);
    /// assert_eq!(CssUnit::from_str("%"), CssUnit::Percent);
    /// assert_eq!(CssUnit::from_str("unknown"), CssUnit::None);
    /// ```
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "px" => CssUnit::Px,
            "rem" => CssUnit::Rem,
            "em" => CssUnit::Em,
            "vw" => CssUnit::Vw,
            "vh" => CssUnit::Vh,
            "%" => CssUnit::Percent,
            _ => CssUnit::None,
        }
    }

    /// 转换为字符串
    ///
    /// 将 `CssUnit` 枚举值转换为对应的字符串表示。
    ///
    /// # 返回值
    ///
    /// 返回单位的字符串表示，如 "px", "rem", "em", "vw", "vh", "%"。
    /// 对于 `CssUnit::None`，返回空字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::CssUnit;
    ///
    /// assert_eq!(CssUnit::Px.to_str(), "px");
    /// assert_eq!(CssUnit::Rem.to_str(), "rem");
    /// assert_eq!(CssUnit::Percent.to_str(), "%");
    /// assert_eq!(CssUnit::None.to_str(), "");
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            CssUnit::Px => "px",
            CssUnit::Rem => "rem",
            CssUnit::Em => "em",
            CssUnit::Vw => "vw",
            CssUnit::Vh => "vh",
            CssUnit::Percent => "%",
            CssUnit::None => "",
        }
    }
}

/// 单位转换器
///
/// 用于在不同CSS单位之间进行值的转换，支持像素、rem、em、视口单位和百分比等。
/// 转换基于当前的上下文设置，如根字体大小、父元素字体大小和视口尺寸。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
///
/// // 创建默认的单位转换器
/// let converter = UnitConverter::default();
///
/// // 将16px转换为rem
/// let rem_value = converter.convert(16.0, CssUnit::Px, CssUnit::Rem);
/// assert_eq!(rem_value, 1.0);
///
/// // 将1.5rem转换为像素
/// let px_value = converter.convert(1.5, CssUnit::Rem, CssUnit::Px);
/// assert_eq!(px_value, 24.0);
/// ```
pub struct UnitConverter {
    /// 根字体大小（px）
    root_font_size: f64,
    /// 父元素字体大小（px）
    parent_font_size: f64,
    /// 视口宽度（px）
    viewport_width: f64,
    /// 视口高度（px）
    viewport_height: f64,
    /// 自定义转换比例
    custom_ratios: HashMap<(CssUnit, CssUnit), f64>,
}

impl UnitConverter {
    /// 创建新的单位转换器
    ///
    /// 使用指定的参数初始化单位转换器。
    ///
    /// # 参数
    ///
    /// * `root_font_size` - 根元素字体大小，单位为像素
    /// * `parent_font_size` - 父元素字体大小，单位为像素
    /// * `viewport_width` - 视口宽度，单位为像素
    /// * `viewport_height` - 视口高度，单位为像素
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `UnitConverter` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::UnitConverter;
    ///
    /// // 创建自定义设置的单位转换器
    /// let converter = UnitConverter::new(
    ///     18.0,  // 根字体大小为18px
    ///     16.0,  // 父元素字体大小为16px
    ///     1440.0, // 视口宽度为1440px
    ///     900.0   // 视口高度为900px
    /// );
    /// ```
    pub fn new(
        root_font_size: f64,
        parent_font_size: f64,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Self {
        Self {
            root_font_size,
            parent_font_size,
            viewport_width,
            viewport_height,
            custom_ratios: HashMap::new(),
        }
    }

    /// 使用默认值创建单位转换器
    ///
    /// 创建一个使用默认设置的单位转换器：
    /// - 根字体大小：16px
    /// - 父元素字体大小：16px
    /// - 视口宽度：1920px
    /// - 视口高度：1080px
    ///
    /// # 返回值
    ///
    /// 返回一个使用默认设置的 `UnitConverter` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::UnitConverter;
    ///
    /// let converter = UnitConverter::default();
    /// ```
    pub fn default() -> Self {
        Self::new(16.0, 16.0, 1920.0, 1080.0)
    }

    /// 设置根字体大小
    ///
    /// 更新单位转换器的根字体大小设置。
    ///
    /// # 参数
    ///
    /// * `size` - 新的根字体大小，单位为像素
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `UnitConverter` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default()
    ///     .with_root_font_size(20.0);
    ///
    /// // 使用新的根字体大小进行转换
    /// let rem_value = converter.convert(20.0, CssUnit::Px, CssUnit::Rem);
    /// assert_eq!(rem_value, 1.0);
    /// ```
    pub fn with_root_font_size(mut self, size: f64) -> Self {
        self.root_font_size = size;
        self
    }

    /// 设置父元素字体大小
    ///
    /// 更新单位转换器的父元素字体大小设置。
    ///
    /// # 参数
    ///
    /// * `size` - 新的父元素字体大小，单位为像素
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `UnitConverter` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default()
    ///     .with_parent_font_size(18.0);
    ///
    /// // 使用新的父元素字体大小进行转换
    /// let em_value = converter.convert(18.0, CssUnit::Px, CssUnit::Em);
    /// assert_eq!(em_value, 1.0);
    /// ```
    pub fn with_parent_font_size(mut self, size: f64) -> Self {
        self.parent_font_size = size;
        self
    }

    /// 设置视口尺寸
    ///
    /// 更新单位转换器的视口宽度和高度设置。
    ///
    /// # 参数
    ///
    /// * `width` - 新的视口宽度，单位为像素
    /// * `height` - 新的视口高度，单位为像素
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `UnitConverter` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default()
    ///     .with_viewport_size(1000.0, 800.0);
    ///
    /// // 使用新的视口尺寸进行转换
    /// let vw_value = converter.convert(100.0, CssUnit::Px, CssUnit::Vw);
    /// assert_eq!(vw_value, 10.0); // 100px = 10vw (在1000px宽的视口中)
    /// ```
    pub fn with_viewport_size(mut self, width: f64, height: f64) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self
    }

    /// 添加自定义转换比例
    ///
    /// 为特定的单位对添加自定义转换比例，覆盖默认的转换逻辑。
    ///
    /// # 参数
    ///
    /// * `from` - 源单位
    /// * `to` - 目标单位
    /// * `ratio` - 转换比例
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let mut converter = UnitConverter::default();
    ///
    /// // 添加自定义转换比例：1rem = 10px (而不是默认的16px)
    /// converter.add_custom_ratio(CssUnit::Rem, CssUnit::Px, 10.0);
    ///
    /// // 使用自定义比例进行转换
    /// let px_value = converter.convert(2.0, CssUnit::Rem, CssUnit::Px);
    /// assert_eq!(px_value, 20.0);
    /// ```
    pub fn add_custom_ratio(&mut self, from: CssUnit, to: CssUnit, ratio: f64) {
        self.custom_ratios.insert((from, to), ratio);
    }

    /// 转换单位
    ///
    /// 将值从一个单位转换为另一个单位。
    ///
    /// # 参数
    ///
    /// * `value` - 要转换的值
    /// * `from` - 源单位
    /// * `to` - 目标单位
    ///
    /// # 返回值
    ///
    /// 返回转换后的值。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default();
    ///
    /// // px 到 rem
    /// assert_eq!(converter.convert(16.0, CssUnit::Px, CssUnit::Rem), 1.0);
    ///
    /// // rem 到 px
    /// assert_eq!(converter.convert(1.0, CssUnit::Rem, CssUnit::Px), 16.0);
    ///
    /// // vw 到 px (在1920px宽的视口中)
    /// assert_eq!(converter.convert(10.0, CssUnit::Vw, CssUnit::Px), 192.0);
    /// ```
    pub fn convert(&self, value: f64, from: CssUnit, to: CssUnit) -> f64 {
        if from == to {
            return value;
        }

        // 检查自定义转换比例
        if let Some(ratio) = self.custom_ratios.get(&(from, to)) {
            return value * ratio;
        }

        // 先转换为像素
        let px_value = match from {
            CssUnit::Px => value,
            CssUnit::Rem => value * self.root_font_size,
            CssUnit::Em => value * self.parent_font_size,
            CssUnit::Vw => value * self.viewport_width / 100.0,
            CssUnit::Vh => value * self.viewport_height / 100.0,
            CssUnit::Percent => value, // 百分比需要上下文，这里简单处理
            CssUnit::None => value,
        };

        // 从像素转换为目标单位
        match to {
            CssUnit::Px => px_value,
            CssUnit::Rem => px_value / self.root_font_size,
            CssUnit::Em => px_value / self.parent_font_size,
            CssUnit::Vw => px_value * 100.0 / self.viewport_width,
            CssUnit::Vh => px_value * 100.0 / self.viewport_height,
            CssUnit::Percent => px_value, // 百分比需要上下文，这里简单处理
            CssUnit::None => px_value,
        }
    }

    /// 解析带单位的值
    ///
    /// 从字符串中解析数值和单位。
    ///
    /// # 参数
    ///
    /// * `value_str` - 要解析的字符串，如 "16px", "1.5rem", "100%"
    ///
    /// # 返回值
    ///
    /// 如果解析成功，返回包含数值和单位的元组 `(f64, CssUnit)`。
    /// 如果解析失败，返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default();
    ///
    /// assert_eq!(converter.parse_value("16px"), Some((16.0, CssUnit::Px)));
    /// assert_eq!(converter.parse_value("1.5rem"), Some((1.5, CssUnit::Rem)));
    /// assert_eq!(converter.parse_value("100%"), Some((100.0, CssUnit::Percent)));
    /// assert_eq!(converter.parse_value("42"), Some((42.0, CssUnit::None)));
    /// assert_eq!(converter.parse_value("invalid"), None);
    /// ```
    pub fn parse_value(&self, value_str: &str) -> Option<(f64, CssUnit)> {
        let value_str = value_str.trim();

        // 处理百分比
        if value_str.ends_with('%') {
            if let Ok(value) = value_str[..value_str.len() - 1].parse::<f64>() {
                return Some((value, CssUnit::Percent));
            }
            return None;
        }

        // 处理其他单位
        for unit in &[
            CssUnit::Px,
            CssUnit::Rem,
            CssUnit::Em,
            CssUnit::Vw,
            CssUnit::Vh,
        ] {
            let unit_str = unit.to_str();
            if value_str.ends_with(unit_str) {
                if let Ok(value) = value_str[..value_str.len() - unit_str.len()].parse::<f64>() {
                    return Some((value, *unit));
                }
                return None;
            }
        }

        // 尝试解析无单位值
        if let Ok(value) = value_str.parse::<f64>() {
            return Some((value, CssUnit::None));
        }

        None
    }

    /// 转换带单位的值
    ///
    /// 将带单位的字符串值转换为另一个单位的字符串值。
    ///
    /// # 参数
    ///
    /// * `value_str` - 要转换的带单位字符串，如 "16px", "1.5rem"
    /// * `to` - 目标单位
    ///
    /// # 返回值
    ///
    /// 如果转换成功，返回转换后的带单位字符串。
    /// 如果转换失败，返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::calc::{UnitConverter, CssUnit};
    ///
    /// let converter = UnitConverter::default();
    ///
    /// assert_eq!(
    ///     converter.convert_value_str("16px", CssUnit::Rem),
    ///     Some("1rem".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     converter.convert_value_str("1.5rem", CssUnit::Px),
    ///     Some("24px".to_string())
    /// );
    ///
    /// assert_eq!(
    ///     converter.convert_value_str("invalid", CssUnit::Px),
    ///     None
    /// );
    /// ```
    pub fn convert_value_str(&self, value_str: &str, to: CssUnit) -> Option<String> {
        if let Some((value, from)) = self.parse_value(value_str) {
            let converted = self.convert(value, from, to);
            Some(format!("{}{}", converted, to.to_str()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversion() {
        let converter = UnitConverter::default();

        // px to rem
        assert_eq!(converter.convert(16.0, CssUnit::Px, CssUnit::Rem), 1.0);

        // rem to px
        assert_eq!(converter.convert(1.0, CssUnit::Rem, CssUnit::Px), 16.0);

        // vw to px
        assert_eq!(converter.convert(10.0, CssUnit::Vw, CssUnit::Px), 192.0);
    }

    #[test]
    fn test_parse_value() {
        let converter = UnitConverter::default();

        assert_eq!(converter.parse_value("16px"), Some((16.0, CssUnit::Px)));
        assert_eq!(converter.parse_value("1.5rem"), Some((1.5, CssUnit::Rem)));
        assert_eq!(
            converter.parse_value("100%"),
            Some((100.0, CssUnit::Percent))
        );
        assert_eq!(converter.parse_value("42"), Some((42.0, CssUnit::None)));
    }

    #[test]
    fn test_convert_value_str() {
        let converter = UnitConverter::default();

        assert_eq!(
            converter.convert_value_str("16px", CssUnit::Rem),
            Some("1rem".to_string())
        );
        assert_eq!(
            converter.convert_value_str("1rem", CssUnit::Px),
            Some("16px".to_string())
        );
    }
}
