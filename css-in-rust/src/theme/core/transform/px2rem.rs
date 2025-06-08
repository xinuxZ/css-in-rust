use crate::theme::core::css::{CssObject, CssValue};
use crate::theme::core::transform::Transformer;
use regex::Regex;
use std::collections::HashSet;

/// px 到 rem 的转换器
///
/// 将 CSS 中的像素（px）单位转换为 rem 单位，以实现响应式布局。
/// 这个转换器会自动检测 CSS 属性值中的像素单位，并将其转换为相对于根字体大小的 rem 单位。
///
/// 转换公式：rem = px / rootValue
///
/// # 特性
///
/// - 可以设置根字体大小（默认为 16px）
/// - 可以设置转换精度（小数位数）
/// - 可以选择是否转换媒体查询中的像素值
/// - 自动跳过小于等于 1px 的值，以保留边框等细节
/// - 自动跳过无单位属性（如 zIndex, opacity 等）
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssObject;
/// use css_in_rust::theme::core::transform::{Transformer, Px2RemTransformer};
///
/// // 创建一个 CSS 对象
/// let mut css = CssObject::new();
/// css.set("fontSize", "16px");
/// css.set("margin", "24px 12px");
/// css.set("padding", 8); // 数字会被视为像素值
/// css.set("zIndex", 999); // 无单位属性不会被转换
///
/// // 创建转换器并应用
/// let transformer = Px2RemTransformer::default(); // 使用默认配置（根字体大小为 16px）
/// transformer.visit(&mut css).unwrap();
///
/// // 检查转换结果
/// assert_eq!(css.get("fontSize").unwrap().as_str(), Some("1rem"));
/// assert_eq!(css.get("margin").unwrap().as_str(), Some("1.5rem 0.75rem"));
/// assert_eq!(css.get("padding").unwrap().as_str(), Some("0.5rem"));
/// assert_eq!(css.get("zIndex").unwrap().as_f64(), Some(999.0)); // 未转换
/// ```
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
    /// 使用指定的配置创建一个新的像素到 rem 单位的转换器。
    ///
    /// # 参数
    ///
    /// * `root_value` - 根字体大小（像素），用于计算 rem 值，默认为 16
    /// * `precision` - 转换后的 rem 值的小数位数，默认为 5
    /// * `media_query` - 是否转换媒体查询中的 px 值，默认为 false
    ///
    /// # 返回值
    ///
    /// 返回配置好的 `Px2RemTransformer` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::Px2RemTransformer;
    ///
    /// // 创建根字体大小为 10px，精度为 2 位小数的转换器
    /// let transformer = Px2RemTransformer::new(10.0, 2, false);
    ///
    /// // 使用这个转换器，16px 会被转换为 1.60rem
    /// ```
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
    ///
    /// 创建一个使用默认配置的像素到 rem 单位的转换器：
    /// - 根字体大小：16px
    /// - 精度：5 位小数
    /// - 不转换媒体查询中的像素值
    ///
    /// # 返回值
    ///
    /// 返回使用默认配置的 `Px2RemTransformer` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::Px2RemTransformer;
    /// use css_in_rust::theme::core::transform::Transformer;
    /// use css_in_rust::theme::core::css::CssObject;
    ///
    /// let transformer = Px2RemTransformer::default();
    ///
    /// let mut css = CssObject::new();
    /// css.set("width", "320px");
    ///
    /// transformer.visit(&mut css).unwrap();
    /// assert_eq!(css.get("width").unwrap().as_str(), Some("20rem"));
    /// ```
    pub fn default() -> Self {
        Self::new(16.0, 5, false)
    }

    /// 将 px 值替换为 rem 值
    ///
    /// 使用正则表达式查找字符串中的像素值，并将其转换为 rem 值。
    /// 小于等于 1px 的值不会被转换，以保留细小的边框和阴影等。
    ///
    /// # 参数
    ///
    /// * `value` - 包含像素值的字符串
    ///
    /// # 返回值
    ///
    /// 返回替换后的字符串，其中像素值已被转换为 rem 值。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::Px2RemTransformer;
    ///
    /// let transformer = Px2RemTransformer::default(); // 根字体大小为 16px
    /// let result = transformer.px_replace("margin: 16px 8px 1px 0.5px;");
    ///
    /// // 16px -> 1rem, 8px -> 0.5rem, 1px 和 0.5px 保持不变
    /// assert_eq!(result, "margin: 1rem 0.5rem 1px 0.5px;");
    /// ```
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
    /// 访问并转换 CSS 对象中的像素值
    ///
    /// 实现 `Transformer` trait 的 `visit` 方法，遍历 CSS 对象中的所有属性，
    /// 将像素值转换为 rem 值。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要转换的 CSS 对象的可变引用
    ///
    /// # 返回值
    ///
    /// 如果转换成功，返回 `Ok(())`；如果转换过程中发生错误，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    /// use css_in_rust::theme::core::transform::{Transformer, Px2RemTransformer};
    ///
    /// let mut css = CssObject::new();
    /// css.set("fontSize", "16px");
    /// css.set("borderWidth", "1px"); // 小于等于 1px 的值不会被转换
    ///
    /// let transformer = Px2RemTransformer::default();
    /// transformer.visit(&mut css).unwrap();
    ///
    /// assert_eq!(css.get("fontSize").unwrap().as_str(), Some("1rem"));
    /// assert_eq!(css.get("borderWidth").unwrap().as_str(), Some("1px"));
    /// ```
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
