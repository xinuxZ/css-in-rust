use std::collections::HashMap;

/// CSS 单位类型
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
    pub fn default() -> Self {
        Self::new(16.0, 16.0, 1920.0, 1080.0)
    }

    /// 设置根字体大小
    pub fn with_root_font_size(mut self, size: f64) -> Self {
        self.root_font_size = size;
        self
    }

    /// 设置父元素字体大小
    pub fn with_parent_font_size(mut self, size: f64) -> Self {
        self.parent_font_size = size;
        self
    }

    /// 设置视口尺寸
    pub fn with_viewport_size(mut self, width: f64, height: f64) -> Self {
        self.viewport_width = width;
        self.viewport_height = height;
        self
    }

    /// 添加自定义转换比例
    pub fn add_custom_ratio(&mut self, from: CssUnit, to: CssUnit, ratio: f64) {
        self.custom_ratios.insert((from, to), ratio);
    }

    /// 转换单位
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
