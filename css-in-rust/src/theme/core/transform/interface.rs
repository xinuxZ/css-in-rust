use crate::theme::core::css::CssObject;

/// CSS 样式转换器接口
pub trait Transformer {
    /// 访问并转换 CSS 对象
    fn visit(&self, css_obj: &mut CssObject) -> Result<(), String>;
}

/// 转换器集合，用于管理多个转换器
pub struct TransformerRegistry {
    transformers: Vec<Box<dyn Transformer>>,
}

impl TransformerRegistry {
    /// 创建新的转换器注册表
    pub fn new() -> Self {
        Self {
            transformers: Vec::new(),
        }
    }

    /// 注册一个转换器
    pub fn register<T: Transformer + 'static>(&mut self, transformer: T) {
        self.transformers.push(Box::new(transformer));
    }

    /// 应用所有注册的转换器
    pub fn apply_all(&self, css_obj: &mut CssObject) -> Result<(), String> {
        for transformer in &self.transformers {
            transformer.visit(css_obj)?;
        }
        Ok(())
    }
}
