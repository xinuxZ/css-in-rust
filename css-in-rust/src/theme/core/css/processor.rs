use crate::theme::core::css::CssObject;
use crate::theme::core::transform::{Transformer, TransformerRegistry};

/// 样式处理器
///
/// 负责对 CSS 对象进行处理，包括应用转换器和生成最终的 CSS 字符串。
/// 样式处理器是样式管道中的核心组件，它管理一组转换器，并负责将CSS对象转换为CSS字符串。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{StyleProcessor, CssObject};
/// use css_in_rust::theme::core::transform::PrefixTransformer;
///
/// // 创建样式处理器
/// let mut processor = StyleProcessor::new();
///
/// // 注册转换器
/// processor.register_transformer(PrefixTransformer::new());
///
/// // 创建CSS对象
/// let mut css_obj = CssObject::new();
/// css_obj.set("display", "flex");
/// css_obj.set("user-select", "none");
///
/// // 处理CSS对象
/// processor.process(&mut css_obj).unwrap();
///
/// // 转换为CSS字符串
/// let css_string = processor.to_css_string(&css_obj);
/// println!("生成的CSS: {}", css_string);
/// ```
pub struct StyleProcessor {
    transformers: TransformerRegistry,
}

impl StyleProcessor {
    /// 创建新的样式处理器
    ///
    /// 初始化一个没有任何转换器的样式处理器。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `StyleProcessor` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StyleProcessor;
    ///
    /// let processor = StyleProcessor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            transformers: TransformerRegistry::new(),
        }
    }

    /// 注册转换器
    ///
    /// 向样式处理器中添加一个新的转换器，用于在处理过程中转换CSS。
    ///
    /// # 参数
    ///
    /// * `transformer` - 要注册的转换器，必须实现 `Transformer` trait
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StyleProcessor;
    /// use css_in_rust::theme::core::transform::{Transformer, PrefixTransformer};
    ///
    /// let mut processor = StyleProcessor::new();
    /// processor.register_transformer(PrefixTransformer::new());
    /// ```
    pub fn register_transformer<T: Transformer + 'static>(&mut self, transformer: T) {
        self.transformers.register(transformer);
    }

    /// 处理 CSS 对象
    ///
    /// 对CSS对象应用所有注册的转换器，修改原始对象。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要处理的CSS对象的可变引用
    ///
    /// # 返回值
    ///
    /// 如果处理成功，返回 `Ok(())`；如果处理失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{StyleProcessor, CssObject};
    /// use css_in_rust::theme::core::transform::PrefixTransformer;
    ///
    /// let mut processor = StyleProcessor::new();
    /// processor.register_transformer(PrefixTransformer::new());
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("user-select", "none");
    ///
    /// // 处理CSS对象，可能添加浏览器前缀
    /// processor.process(&mut css_obj).unwrap();
    /// ```
    pub fn process(&self, css_obj: &mut CssObject) -> Result<(), String> {
        // 应用所有转换器
        self.transformers.apply_all(css_obj)?;

        Ok(())
    }

    /// 将 CSS 对象转换为 CSS 字符串
    ///
    /// 将CSS对象序列化为CSS字符串表示形式。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要转换的CSS对象
    ///
    /// # 返回值
    ///
    /// 返回表示CSS的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{StyleProcessor, CssObject};
    ///
    /// let processor = StyleProcessor::new();
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("color", "red");
    /// css_obj.set("font-size", "16px");
    ///
    /// // 转换为CSS字符串
    /// let css_string = processor.to_css_string(&css_obj);
    /// println!("{}", css_string);
    /// // 输出类似于:
    /// // color: red;
    /// // font-size: 16px;
    /// ```
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
    ///
    /// 结合 `process` 和 `to_css_string` 方法的功能，处理CSS对象并直接返回CSS字符串。
    /// 这是一个便捷方法，适用于只需要最终CSS字符串而不需要保留处理后的CSS对象的情况。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要处理的CSS对象
    ///
    /// # 返回值
    ///
    /// 如果处理成功，返回 `Ok(String)`，包含生成的CSS；如果处理失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{StyleProcessor, CssObject};
    /// use css_in_rust::theme::core::transform::PrefixTransformer;
    ///
    /// let mut processor = StyleProcessor::new();
    /// processor.register_transformer(PrefixTransformer::new());
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("display", "flex");
    /// css_obj.set("color", "red");
    ///
    /// // 处理并直接获取CSS字符串
    /// let css_string = processor.process_to_css(css_obj).unwrap();
    /// println!("{}", css_string);
    /// ```
    pub fn process_to_css(&self, mut css_obj: CssObject) -> Result<String, String> {
        self.process(&mut css_obj)?;
        Ok(self.to_css_string(&css_obj))
    }
}
