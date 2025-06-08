use crate::theme::core::cache::CacheManager;
use crate::theme::core::css::{CssObject, StyleProcessor};
use crate::theme::core::optimize::{OptimizeConfig, StyleOptimizer};
use crate::theme::core::transform::{Transformer, TransformerRegistry};
use std::sync::Arc;

/// 样式处理管道
///
/// 样式处理管道负责将CSS对象通过一系列处理步骤转换为最终的CSS输出。
/// 处理步骤包括应用转换器、优化CSS和缓存结果等。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{StylePipeline, CssObject};
/// use css_in_rust::theme::core::transform::PrefixTransformer;
///
/// // 创建样式处理管道
/// let mut pipeline = StylePipeline::new();
///
/// // 注册转换器
/// pipeline.register_transformer(PrefixTransformer::new());
///
/// // 创建CSS对象
/// let mut css_obj = CssObject::new();
/// css_obj.set("color", "red");
/// css_obj.set("display", "flex");
///
/// // 处理CSS对象
/// let result = pipeline.process(css_obj).unwrap();
///
/// // 使用处理结果
/// println!("生成的类名: {}", result.class_name);
/// println!("生成的CSS: {}", result.css);
/// ```
pub struct StylePipeline {
    /// 样式处理器
    processor: StyleProcessor,
    /// 样式优化器
    optimizer: Option<StyleOptimizer>,
    /// 缓存管理器
    cache_manager: Option<Arc<CacheManager>>,
}

impl StylePipeline {
    /// 创建新的样式处理管道
    ///
    /// 初始化一个基本的样式处理管道，包含默认的样式处理器，但没有优化器和缓存管理器。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `StylePipeline` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipeline;
    ///
    /// let pipeline = StylePipeline::new();
    /// ```
    pub fn new() -> Self {
        Self {
            processor: StyleProcessor::new(),
            optimizer: None,
            cache_manager: None,
        }
    }

    /// 设置样式处理器
    ///
    /// 替换默认的样式处理器为指定的处理器。
    ///
    /// # 参数
    ///
    /// * `processor` - 要使用的样式处理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipeline` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{StylePipeline, StyleProcessor};
    ///
    /// let custom_processor = StyleProcessor::new();
    /// let pipeline = StylePipeline::new().with_processor(custom_processor);
    /// ```
    pub fn with_processor(mut self, processor: StyleProcessor) -> Self {
        self.processor = processor;
        self
    }

    /// 设置样式优化器
    ///
    /// 添加样式优化器到处理管道中，用于优化生成的CSS。
    ///
    /// # 参数
    ///
    /// * `optimizer` - 要使用的样式优化器
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipeline` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipeline;
    /// use css_in_rust::theme::core::optimize::{StyleOptimizer, OptimizeConfig};
    ///
    /// let optimizer = StyleOptimizer::new(OptimizeConfig::default());
    /// let pipeline = StylePipeline::new().with_optimizer(optimizer);
    /// ```
    pub fn with_optimizer(mut self, optimizer: StyleOptimizer) -> Self {
        self.optimizer = Some(optimizer);
        self
    }

    /// 设置缓存管理器
    ///
    /// 添加缓存管理器到处理管道中，用于缓存处理结果以提高性能。
    ///
    /// # 参数
    ///
    /// * `cache_manager` - 要使用的缓存管理器
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipeline` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use std::sync::Arc;
    /// use css_in_rust::theme::core::css::StylePipeline;
    /// use css_in_rust::theme::core::cache::CacheManager;
    ///
    /// let cache_manager = Arc::new(CacheManager::new());
    /// let pipeline = StylePipeline::new().with_cache_manager(cache_manager);
    /// ```
    pub fn with_cache_manager(mut self, cache_manager: Arc<CacheManager>) -> Self {
        self.cache_manager = Some(cache_manager);
        self
    }

    /// 注册转换器
    ///
    /// 向样式处理器中注册一个新的转换器，用于在处理过程中转换CSS。
    ///
    /// # 参数
    ///
    /// * `transformer` - 要注册的转换器，必须实现 `Transformer` trait
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipeline;
    /// use css_in_rust::theme::core::transform::{Transformer, PrefixTransformer};
    ///
    /// let mut pipeline = StylePipeline::new();
    /// pipeline.register_transformer(PrefixTransformer::new());
    /// ```
    pub fn register_transformer<T: Transformer + 'static>(&mut self, transformer: T) {
        self.processor.register_transformer(transformer);
    }

    /// 处理 CSS 对象
    ///
    /// 将CSS对象通过处理管道进行处理，包括应用转换器、优化CSS和缓存结果等步骤。
    ///
    /// # 参数
    ///
    /// * `css_obj` - 要处理的CSS对象
    ///
    /// # 返回值
    ///
    /// 如果处理成功，返回 `Ok(ProcessedStyle)`；如果处理失败，返回错误信息。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::{StylePipeline, CssObject};
    ///
    /// let pipeline = StylePipeline::new();
    ///
    /// let mut css_obj = CssObject::new();
    /// css_obj.set("color", "red");
    /// css_obj.set("font-size", "16px");
    ///
    /// let result = pipeline.process(css_obj).unwrap();
    /// println!("生成的类名: {}", result.class_name);
    /// println!("生成的CSS: {}", result.css);
    /// ```
    pub fn process(&self, mut css_obj: CssObject) -> Result<ProcessedStyle, String> {
        // 1. 应用转换器
        self.processor.process(&mut css_obj)?;

        // 2. 优化 CSS
        let optimized_css = if let Some(optimizer) = &self.optimizer {
            let css_string = self.processor.to_css_string(&css_obj);
            optimizer.optimize(&css_string)
        } else {
            self.processor.to_css_string(&css_obj)
        };

        // 3. 生成类名
        let class_name = self.generate_class_name(&optimized_css);

        // 4. 缓存处理结果
        if let Some(cache_manager) = &self.cache_manager {
            // 这里可以实现缓存逻辑
        }

        Ok(ProcessedStyle {
            class_name,
            css: optimized_css,
            css_object: css_obj,
        })
    }

    /// 生成类名
    ///
    /// 根据CSS内容生成唯一的类名，使用SHA-256哈希算法。
    ///
    /// # 参数
    ///
    /// * `css` - CSS字符串
    ///
    /// # 返回值
    ///
    /// 返回生成的类名，格式为 "css-" 前缀加上哈希值的前8个字符。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipeline;
    ///
    /// let pipeline = StylePipeline::new();
    /// let class_name = pipeline.generate_class_name("color: red; font-size: 16px;");
    /// assert!(class_name.starts_with("css-"));
    /// assert_eq!(class_name.len(), 12); // "css-" + 8个字符
    /// ```
    fn generate_class_name(&self, css: &str) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        let hash = hasher.finalize();

        format!("css-{}", hex::encode(&hash[..4]))
    }
}

/// 处理后的样式
///
/// 表示经过样式处理管道处理后的结果，包含生成的类名、CSS字符串和原始CSS对象。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::{StylePipeline, CssObject, ProcessedStyle};
///
/// let pipeline = StylePipeline::new();
///
/// let mut css_obj = CssObject::new();
/// css_obj.set("color", "red");
///
/// let processed: ProcessedStyle = pipeline.process(css_obj).unwrap();
///
/// // 使用处理结果
/// println!("类名: {}", processed.class_name);
/// println!("CSS: {}", processed.css);
/// println!("属性数量: {}", processed.css_object.len());
/// ```
pub struct ProcessedStyle {
    /// CSS 类名
    pub class_name: String,
    /// CSS 字符串
    pub css: String,
    /// CSS 对象
    pub css_object: CssObject,
}

/// 样式处理管道构建器
///
/// 用于构建样式处理管道的构建器模式实现。
/// 提供了流畅的API来配置和创建样式处理管道。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::core::css::StylePipelineBuilder;
/// use css_in_rust::theme::core::transform::PrefixTransformer;
///
/// // 创建并配置管道构建器
/// let pipeline = StylePipelineBuilder::new()
///     .add_transformer(PrefixTransformer::new())
///     .with_optimization(true)
///     .with_caching(true)
///     .build();
///
/// // 使用构建的管道处理CSS
/// let mut css_obj = css_in_rust::theme::core::css::CssObject::new();
/// css_obj.set("color", "red");
///
/// let result = pipeline.process(css_obj).unwrap();
/// ```
pub struct StylePipelineBuilder {
    /// 转换器注册表
    transformers: TransformerRegistry,
    /// 是否启用优化
    enable_optimization: bool,
    /// 是否启用缓存
    enable_caching: bool,
}

impl StylePipelineBuilder {
    /// 创建新的样式处理管道构建器
    ///
    /// 初始化一个样式处理管道构建器，默认启用优化和缓存。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `StylePipelineBuilder` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipelineBuilder;
    ///
    /// let builder = StylePipelineBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            transformers: TransformerRegistry::new(),
            enable_optimization: true,
            enable_caching: true,
        }
    }

    /// 添加转换器
    ///
    /// 向构建器中添加一个转换器，该转换器将在构建的管道中使用。
    ///
    /// # 参数
    ///
    /// * `transformer` - 要添加的转换器，必须实现 `Transformer` trait
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipelineBuilder` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipelineBuilder;
    /// use css_in_rust::theme::core::transform::{Transformer, PrefixTransformer};
    ///
    /// let builder = StylePipelineBuilder::new()
    ///     .add_transformer(PrefixTransformer::new());
    /// ```
    pub fn add_transformer<T: Transformer + 'static>(mut self, transformer: T) -> Self {
        self.transformers.register(transformer);
        self
    }

    /// 启用或禁用优化
    ///
    /// 配置是否在构建的管道中启用CSS优化。
    ///
    /// # 参数
    ///
    /// * `enable` - 是否启用优化
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipelineBuilder` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipelineBuilder;
    ///
    /// // 禁用优化
    /// let builder = StylePipelineBuilder::new().with_optimization(false);
    ///
    /// // 启用优化
    /// let builder = StylePipelineBuilder::new().with_optimization(true);
    /// ```
    pub fn with_optimization(mut self, enable: bool) -> Self {
        self.enable_optimization = enable;
        self
    }

    /// 启用或禁用缓存
    ///
    /// 配置是否在构建的管道中启用结果缓存。
    ///
    /// # 参数
    ///
    /// * `enable` - 是否启用缓存
    ///
    /// # 返回值
    ///
    /// 返回修改后的 `StylePipelineBuilder` 实例，支持链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipelineBuilder;
    ///
    /// // 禁用缓存
    /// let builder = StylePipelineBuilder::new().with_caching(false);
    ///
    /// // 启用缓存
    /// let builder = StylePipelineBuilder::new().with_caching(true);
    /// ```
    pub fn with_caching(mut self, enable: bool) -> Self {
        self.enable_caching = enable;
        self
    }

    /// 构建样式处理管道
    ///
    /// 根据当前配置构建一个样式处理管道。
    ///
    /// # 返回值
    ///
    /// 返回构建好的 `StylePipeline` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::core::css::StylePipelineBuilder;
    /// use css_in_rust::theme::core::transform::PrefixTransformer;
    ///
    /// // 创建完整配置的管道
    /// let pipeline = StylePipelineBuilder::new()
    ///     .add_transformer(PrefixTransformer::new())
    ///     .with_optimization(true)
    ///     .with_caching(true)
    ///     .build();
    /// ```
    pub fn build(self) -> StylePipeline {
        let mut processor = StyleProcessor::new();

        // TODO: 将转换器从注册表移动到处理器

        let mut pipeline = StylePipeline::new().with_processor(processor);

        if self.enable_optimization {
            pipeline = pipeline.with_optimizer(StyleOptimizer::new(OptimizeConfig::default()));
        }

        if self.enable_caching {
            // TODO: 创建缓存管理器
        }

        pipeline
    }
}
