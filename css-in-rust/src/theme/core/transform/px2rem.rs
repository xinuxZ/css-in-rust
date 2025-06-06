use crate::theme::core::css::{CssObject, CssValue};
use crate::theme::core::transform::Transformer;
use regex::Regex;
use std::collections::HashSet;

/// px 到 rem 的转换器
///
/// 将 CSS 中的像素（px）单位转换为 rem 单位
pub struct Px2RemTransformer {
    root_value: f32,
    precision: u32,
    media_query: bool,
    unitless_props: HashSet<String>,
    px_regex: Regex,
}

impl Px2RemTransformer {
    /// 创建新的 px2rem 转换器
    ///
    /// # 参数
    ///
    /// * `root_value` - 根字体大小，默认为 16
    /// * `precision` - 小数位数，默认为 5
    /// * `media_query` - 是否转换媒体查询中的 px 值，默认为 false
    pub fn new(root_value: f32, precision: u32, media_query: bool) -> Self {
        let mut unitless_props = HashSet::new();
        unitless_props.insert("zIndex".to_string());
        unitless_props.insert("fontWeight".to_string());
        unitless_props.insert("opacity".to_string());
        unitless_props.insert("flex".to_string());
        unitless_props.insert("flexGrow".to_string());
        unitless_props.insert("flexShrink".to_string());
        unitless_props.insert("order".to_string());
        unitless_props.insert("lineHeight".to_string());

        Self {
            root_value,
            precision,
            media_query,
            unitless_props,
            px_regex: Regex::new(r"(\d*\.?\d+)px").unwrap(),
        }
    }

    /// 使用默认配置创建 px2rem 转换器
    pub fn default() -> Self {
        Self::new(16.0, 5, false)
    }

    /// 将 px 值替换为 rem 值
    fn px_replace(&self, value: &str) -> String {
        self.px_regex
            .replace_all(value, |caps: &regex::Captures| {
                let px_value: f32 = caps[1].parse().unwrap_or(0.0);

                // 跳过小于等于 1px 的值
                if px_value <= 1.0 {
                    return format!("{}px", px_value);
                }

                let rem_value = px_value / self.root_value;
                format!("{:.1$}rem", rem_value, self.precision as usize)
            })
            .to_string()
    }
}

impl Transformer for Px2RemTransformer {
    fn visit(&self, css_obj: &mut CssObject) -> Result<(), String> {
        for (key, value) in css_obj.properties.iter_mut() {
            // 处理字符串类型的值
            if let CssValue::String(value_str) = value {
                if value_str.contains("px") {
                    *value = CssValue::String(self.px_replace(value_str));
                }
            }
            // 处理数字类型的值
            else if let CssValue::Number(num) = value {
                if !self.unitless_props.contains(key) {
                    // 对于非无单位属性，将数字转换为带 px 的字符串，然后转换为 rem
                    let px_str = format!("{}px", num);
                    *value = CssValue::String(self.px_replace(&px_str));
                }
            }
            // 处理嵌套对象
            else if let CssValue::Object(obj) = value {
                self.visit(obj)?;
            }
            // 处理数组
            else if let CssValue::Array(arr) = value {
                for item in arr.iter_mut() {
                    if let CssValue::Object(obj) = item {
                        self.visit(obj)?;
                    } else if let CssValue::String(str_val) = item {
                        if str_val.contains("px") {
                            *item = CssValue::String(self.px_replace(str_val));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
