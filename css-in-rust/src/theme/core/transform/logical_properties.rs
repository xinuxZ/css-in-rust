use crate::theme::core::css::{CssObject, CssValue};
use crate::theme::core::transform::Transformer;
use std::collections::HashMap;

/// 逻辑属性转换器
///
/// 将 CSS 逻辑属性（如 marginBlock）转换为传统的物理属性（如 marginTop 和 marginBottom）
pub struct LogicalPropertiesTransformer {
    key_map: HashMap<String, Vec<String>>,
}

impl LogicalPropertiesTransformer {
    /// 创建新的逻辑属性转换器
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
