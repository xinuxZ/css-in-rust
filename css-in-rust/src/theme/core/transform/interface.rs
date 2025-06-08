use crate::theme::core::css::CssObject;

/// CSS 样式转换器接口
///
/// 这个 trait 定义了 CSS 转换器的基本行为，所有的转换器都需要实现这个接口。
/// 转换器用于访问和修改 CSS 对象，例如添加浏览器前缀、转换单位或处理逻辑属性等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssObject;
/// use css_in_rust::theme::core::transform::Transformer;
///
/// // 创建一个简单的转换器，将所有颜色属性转换为红色
/// struct RedColorTransformer;
///
/// impl Transformer for RedColorTransformer {
///     fn visit(&self, css_obj: &mut CssObject) -> Result<(), String> {
///         if let Some(color) = css_obj.get("color") {
///             css_obj.set("color", "red");
///         }
///         Ok(())
///     }
/// }
///
/// // 使用这个转换器
/// let mut css = CssObject::new();
/// css.set("color", "blue");
///
/// let transformer = RedColorTransformer;
/// transformer.visit(&mut css).unwrap();
///
/// assert_eq!(css.get("color").unwrap().as_str(), Some("red"));
/// ```
pub trait Transformer {
    /// 访问并转换 CSS 对象
    ///
    /// 这个方法接收一个 CSS 对象的可变引用，并对其进行转换操作。
    /// 转换器可以添加、修改或删除 CSS 对象中的属性。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要转换的 CSS 对象的可变引用
    ///
    /// # 返回值
    ///
    /// 如果转换成功，返回 `Ok(())`；如果转换过程中发生错误，返回包含错误信息的 `Err(String)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    /// use css_in_rust::theme::core::transform::Transformer;
    ///
    /// struct PrefixTransformer;
    ///
    /// impl Transformer for PrefixTransformer {
    ///     fn visit(&self, css_obj: &mut CssObject) -> Result<(), String> {
    ///         // 为 CSS 对象添加浏览器前缀
    ///         if let Some(display) = css_obj.get("display") {
    ///             if display.as_str() == Some("flex") {
    ///                 css_obj.set("-webkit-display", "flex");
    ///                 css_obj.set("-moz-display", "flex");
    ///             }
    ///         }
    ///         Ok(())
    ///     }
    /// }
    /// ```
    fn visit(&self, css_obj: &mut CssObject) -> Result<(), String>;
}

/// 转换器集合，用于管理多个转换器
///
/// 这个结构体用于注册和管理多个 CSS 转换器，并提供批量应用所有转换器的功能。
/// 它是样式处理管道中的关键组件，允许将多个转换操作组合在一起。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::CssObject;
/// use css_in_rust::theme::core::transform::{Transformer, TransformerRegistry};
/// use css_in_rust::theme::core::transform::{Px2RemTransformer, LogicalPropertiesTransformer};
///
/// // 创建转换器注册表
/// let mut registry = TransformerRegistry::new();
///
/// // 注册多个转换器
/// registry.register(Px2RemTransformer::default());
/// registry.register(LogicalPropertiesTransformer::new());
///
/// // 创建 CSS 对象
/// let mut css = CssObject::new();
/// css.set("marginBlock", "20px");
/// css.set("fontSize", "16px");
///
/// // 应用所有转换器
/// registry.apply_all(&mut css).unwrap();
///
/// // 检查转换结果
/// assert!(css.get("marginTop").is_some());
/// assert!(css.get("marginBottom").is_some());
/// assert_eq!(css.get("fontSize").unwrap().as_str(), Some("1rem"));
/// ```
pub struct TransformerRegistry {
    transformers: Vec<Box<dyn Transformer>>,
}

impl TransformerRegistry {
    /// 创建新的转换器注册表
    ///
    /// 初始化一个空的转换器注册表，用于后续注册和管理转换器。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `TransformerRegistry` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::TransformerRegistry;
    ///
    /// let registry = TransformerRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            transformers: Vec::new(),
        }
    }

    /// 注册一个转换器
    ///
    /// 将实现了 `Transformer` trait 的对象添加到注册表中。
    /// 注册的转换器将在调用 `apply_all` 方法时按照注册顺序被应用。
    ///
    /// # 参数
    ///
    /// * `transformer` - 要注册的转换器，必须实现 `Transformer` trait
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::transform::{TransformerRegistry, Px2RemTransformer};
    ///
    /// let mut registry = TransformerRegistry::new();
    ///
    /// // 注册 px 到 rem 的转换器
    /// registry.register(Px2RemTransformer::default());
    /// ```
    pub fn register<T: Transformer + 'static>(&mut self, transformer: T) {
        self.transformers.push(Box::new(transformer));
    }

    /// 应用所有注册的转换器
    ///
    /// 按照注册顺序依次应用所有转换器到给定的 CSS 对象上。
    /// 如果任何转换器返回错误，处理将立即停止并返回该错误。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要应用转换器的 CSS 对象的可变引用
    ///
    /// # 返回值
    ///
    /// 如果所有转换器都成功应用，返回 `Ok(())`；如果任何转换器返回错误，返回该错误。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::CssObject;
    /// use css_in_rust::theme::core::transform::{TransformerRegistry, Px2RemTransformer, LogicalPropertiesTransformer};
    ///
    /// let mut registry = TransformerRegistry::new();
    /// registry.register(Px2RemTransformer::default());
    /// registry.register(LogicalPropertiesTransformer::new());
    ///
    /// let mut css = CssObject::new();
    /// css.set("marginBlock", "20px");
    ///
    /// // 应用所有转换器
    /// registry.apply_all(&mut css).unwrap();
    ///
    /// // 转换后，marginBlock 被转换为 marginTop 和 marginBottom，
    /// // 并且 20px 被转换为 rem 单位
    /// ```
    pub fn apply_all(&self, css_obj: &mut CssObject) -> Result<(), String> {
        for transformer in &self.transformers {
            transformer.visit(css_obj)?;
        }
        Ok(())
    }
}
