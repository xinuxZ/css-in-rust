use crate::theme::core::css::CssObject;
use crate::theme::core::transform::{Transformer, TransformerRegistry};

/// 样式处理器
///
/// 负责对 CSS 对象进行处理，包括应用转换器和生成最终的 CSS 字符串
pub struct StyleProcessor {
    transformers: TransformerRegistry,
}

impl StyleProcessor {
    /// 创建新的样式处理器
    pub fn new() -> Self {
        Self {
            transformers: TransformerRegistry::new(),
        }
    }

    /// 注册转换器
    pub fn register_transformer<T: Transformer + 'static>(&mut self, transformer: T) {
        self.transformers.register(transformer);
    }

    /// 处理 CSS 对象
    pub fn process(&self, css_obj: &mut CssObject) -> Result<(), String> {
        // 应用所有转换器
        self.transformers.apply_all(css_obj)?;

        Ok(())
    }

    /// 将 CSS 对象转换为 CSS 字符串
    pub fn to_css_string(&self, css_obj: &CssObject) -> String {
        let mut css = String::new();

        for (key, value) in &css_obj.properties {
            match value {
                crate::theme::core::css::CssValue::String(s) => {
                    css.push_str(&format!("{}: {};\n", key, s));
                }
                crate::theme::core::css::CssValue::Number(n) => {
                    css.push_str(&format!("{}: {};\n", key, n));
                }
                crate::theme::core::css::CssValue::Bool(b) => {
                    css.push_str(&format!("{}: {};\n", key, b));
                }
                crate::theme::core::css::CssValue::Object(obj) => {
                    css.push_str(&format!("{} {{\n", key));
                    css.push_str(&self.to_css_string(obj));
                    css.push_str("}\n");
                }
                crate::theme::core::css::CssValue::Array(arr) => {
                    let values: Vec<String> = arr
                        .iter()
                        .map(|v| match v {
                            crate::theme::core::css::CssValue::String(s) => s.clone(),
                            crate::theme::core::css::CssValue::Number(n) => n.to_string(),
                            crate::theme::core::css::CssValue::Bool(b) => b.to_string(),
                            _ => "".to_string(),
                        })
                        .collect();
                    css.push_str(&format!("{}: {};\n", key, values.join(", ")));
                }
                crate::theme::core::css::CssValue::Null => {
                    // 跳过空值
                }
            }
        }

        css
    }

    /// 处理 CSS 对象并转换为 CSS 字符串
    pub fn process_to_css(&self, mut css_obj: CssObject) -> Result<String, String> {
        self.process(&mut css_obj)?;
        Ok(self.to_css_string(&css_obj))
    }
}
