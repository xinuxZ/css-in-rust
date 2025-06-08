use crate::theme::core::css::{CssObject, CssValue};
use crate::theme::core::transform::Transformer;
use std::collections::HashMap;

/// 逻辑属性转换器
///
/// 将 CSS 逻辑属性（如 marginBlock）转换为传统的物理属性（如 marginTop 和 marginBottom）。
/// 逻辑属性是与书写模式和方向无关的 CSS 属性，它们使得 CSS 布局在不同书写模式（如从左到右、从右到左）下更一致。
/// 然而，并非所有浏览器都完全支持逻辑属性，因此这个转换器可以确保更好的兼容性。
///
/// # 支持的转换
///
/// 这个转换器支持以下逻辑属性的转换：
///
/// - margin 系列：marginBlock, marginBlockStart, marginBlockEnd, marginInline, marginInlineStart, marginInlineEnd
/// - padding 系列：paddingBlock, paddingBlockStart, paddingBlockEnd, paddingInline, paddingInlineStart, paddingInlineEnd
/// - border 系列：borderBlock, borderBlockStart, borderBlockEnd, borderInline, borderInlineStart, borderInlineEnd
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssObject;
/// use css_in_rust::theme::core::transform::{Transformer, LogicalPropertiesTransformer};
///
/// // 创建 CSS 对象
/// let mut css = CssObject::new();
/// css.set("marginBlock", "10px 20px"); // 上边距 10px，下边距 20px
/// css.set("paddingInline", "15px"); // 左右内边距均为 15px
///
/// // 创建并应用转换器
/// let transformer = LogicalPropertiesTransformer::new();
/// transformer.visit(&mut css).unwrap();
///
/// // 检查转换结果
/// assert_eq!(css.get("marginTop").unwrap().as_str(), Some("10px"));
/// assert_eq!(css.get("marginBottom").unwrap().as_str(), Some("20px"));
/// assert_eq!(css.get("paddingLeft").unwrap().as_str(), Some("15px"));
/// assert_eq!(css.get("paddingRight").unwrap().as_str(), Some("15px"));
/// assert!(css.get("marginBlock").is_none()); // 原始逻辑属性已被移除
/// assert!(css.get("paddingInline").is_none()); // 原始逻辑属性已被移除
/// ```
pub struct LogicalPropertiesTransformer {
    key_map: HashMap<String, Vec<String>>,
}

impl LogicalPropertiesTransformer {
    /// 创建新的逻辑属性转换器
    ///
    /// 初始化一个新的逻辑属性转换器，并设置逻辑属性到物理属性的映射关系。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `LogicalPropertiesTransformer` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::LogicalPropertiesTransformer;
    ///
    /// let transformer = LogicalPropertiesTransformer::new();
    /// ```
    pub fn new() -> Self {
        let mut key_map = HashMap::new();

        // 映射逻辑属性到物理属性
        key_map.insert(
            "marginBlock".to_string(),
            vec!["marginTop".to_string(), "marginBottom".to_string()],
        );
        key_map.insert(
            "marginBlockStart".to_string(),
            vec!["marginTop".to_string()],
        );
        key_map.insert(
            "marginBlockEnd".to_string(),
            vec!["marginBottom".to_string()],
        );
        key_map.insert(
            "marginInline".to_string(),
            vec!["marginLeft".to_string(), "marginRight".to_string()],
        );
        key_map.insert(
            "marginInlineStart".to_string(),
            vec!["marginLeft".to_string()],
        );
        key_map.insert(
            "marginInlineEnd".to_string(),
            vec!["marginRight".to_string()],
        );

        key_map.insert(
            "paddingBlock".to_string(),
            vec!["paddingTop".to_string(), "paddingBottom".to_string()],
        );
        key_map.insert(
            "paddingBlockStart".to_string(),
            vec!["paddingTop".to_string()],
        );
        key_map.insert(
            "paddingBlockEnd".to_string(),
            vec!["paddingBottom".to_string()],
        );
        key_map.insert(
            "paddingInline".to_string(),
            vec!["paddingLeft".to_string(), "paddingRight".to_string()],
        );
        key_map.insert(
            "paddingInlineStart".to_string(),
            vec!["paddingLeft".to_string()],
        );
        key_map.insert(
            "paddingInlineEnd".to_string(),
            vec!["paddingRight".to_string()],
        );

        key_map.insert(
            "borderBlock".to_string(),
            vec!["borderTop".to_string(), "borderBottom".to_string()],
        );
        key_map.insert(
            "borderBlockStart".to_string(),
            vec!["borderTop".to_string()],
        );
        key_map.insert(
            "borderBlockEnd".to_string(),
            vec!["borderBottom".to_string()],
        );
        key_map.insert(
            "borderInline".to_string(),
            vec!["borderLeft".to_string(), "borderRight".to_string()],
        );
        key_map.insert(
            "borderInlineStart".to_string(),
            vec!["borderLeft".to_string()],
        );
        key_map.insert(
            "borderInlineEnd".to_string(),
            vec!["borderRight".to_string()],
        );

        Self { key_map }
    }

    /// 解析属性值，处理多个值和 !important 标记
    ///
    /// 将 CSS 属性值字符串拆分为多个值，并保留 !important 标记（如果有）。
    /// 例如，"10px 20px !important" 会被解析为 ["10px !important", "20px !important"]。
    ///
    /// # 参数
    ///
    /// * `value` - 要解析的 CSS 属性值字符串
    ///
    /// # 返回值
    ///
    /// 返回解析后的值数组。如果输入是单个值，返回包含一个元素的数组；
    /// 如果输入是空格分隔的多个值，返回包含多个元素的数组。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::LogicalPropertiesTransformer;
    ///
    /// let transformer = LogicalPropertiesTransformer::new();
    ///
    /// // 单个值
    /// let values = transformer.split_values("10px");
    /// assert_eq!(values, vec!["10px"]);
    ///
    /// // 多个值
    /// let values = transformer.split_values("10px 20px");
    /// assert_eq!(values, vec!["10px", "20px"]);
    ///
    /// // 带 !important 的值
    /// let values = transformer.split_values("10px !important");
    /// assert_eq!(values, vec!["10px !important"]);
    ///
    /// // 带 !important 的多个值
    /// let values = transformer.split_values("10px 20px !important");
    /// assert_eq!(values, vec!["10px !important", "20px !important"]);
    /// ```
    fn split_values(&self, value: &str) -> Vec<String> {
        // 移除 !important 标记并保存
        let (value, has_important) = if value.ends_with("!important") {
            let v = value.trim_end_matches("!important").trim();
            (v, true)
        } else {
            (value, false)
        };

        // 分割值
        let values: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();

        // 如果有 !important，添加回去
        if has_important {
            values
                .into_iter()
                .map(|v| format!("{} !important", v))
                .collect()
        } else {
            values
        }
    }
}

impl Transformer for LogicalPropertiesTransformer {
    /// 访问并转换 CSS 对象中的逻辑属性
    ///
    /// 实现 `Transformer` trait 的 `visit` 方法，将 CSS 对象中的逻辑属性转换为物理属性。
    /// 这个方法会遍历 CSS 对象中的所有属性，检查是否有逻辑属性，并将其转换为对应的物理属性。
    ///
    /// # 转换规则
    ///
    /// - 如果逻辑属性值是单个值（如 "10px"），则该值会应用到所有对应的物理属性。
    /// - 如果逻辑属性值是两个值（如 "10px 20px"），且对应两个物理属性，则第一个值应用到第一个物理属性，第二个值应用到第二个物理属性。
    /// - 如果值的数量与物理属性的数量不匹配，则保留原始的逻辑属性。
    /// - 转换后，原始的逻辑属性会被移除。
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
    /// use css_in_rust::theme::core::transform::{Transformer, LogicalPropertiesTransformer};
    ///
    /// let mut css = CssObject::new();
    /// css.set("marginBlock", "10px"); // 上下边距均为 10px
    /// css.set("paddingInline", "5px 15px"); // 左内边距 5px，右内边距 15px
    ///
    /// let transformer = LogicalPropertiesTransformer::new();
    /// transformer.visit(&mut css).unwrap();
    ///
    /// // 检查转换结果
    /// assert_eq!(css.get("marginTop").unwrap().as_str(), Some("10px"));
    /// assert_eq!(css.get("marginBottom").unwrap().as_str(), Some("10px"));
    /// assert_eq!(css.get("paddingLeft").unwrap().as_str(), Some("5px"));
    /// assert_eq!(css.get("paddingRight").unwrap().as_str(), Some("15px"));
    /// ```
    fn visit(&self, css_obj: &mut CssObject) -> Result<(), String> {
        let mut new_properties = HashMap::new();
        let mut remove_keys = Vec::new();

        for (key, value) in &css_obj.properties {
            if let Some(mappings) = self.key_map.get(key) {
                match value {
                    CssValue::String(value_str) => {
                        let values = self.split_values(value_str);

                        // 根据值的数量和映射关系转换属性
                        if values.len() == 1 {
                            // 单个值应用到所有映射的属性
                            for mapped_key in mappings {
                                new_properties.insert(
                                    mapped_key.clone(),
                                    CssValue::String(values[0].clone()),
                                );
                            }
                        } else if values.len() == 2 && mappings.len() == 2 {
                            // 两个值分别应用到两个映射的属性
                            new_properties
                                .insert(mappings[0].clone(), CssValue::String(values[0].clone()));
                            new_properties
                                .insert(mappings[1].clone(), CssValue::String(values[1].clone()));
                        } else {
                            // 值的数量与映射的属性数量不匹配，保留原始属性
                            continue;
                        }

                        // 标记原始属性待移除
                        remove_keys.push(key.clone());
                    }
                    CssValue::Number(num) => {
                        // 数字值应用到所有映射的属性
                        for mapped_key in mappings {
                            new_properties.insert(mapped_key.clone(), CssValue::Number(*num));
                        }

                        // 标记原始属性待移除
                        remove_keys.push(key.clone());
                    }
                    _ => {
                        // 其他类型的值不处理
                        continue;
                    }
                }
            }
        }

        // 移除已转换的逻辑属性
        for key in remove_keys {
            css_obj.properties.remove(&key);
        }

        // 添加转换后的物理属性
        for (key, value) in new_properties {
            css_obj.properties.insert(key, value);
        }

        Ok(())
    }
}
