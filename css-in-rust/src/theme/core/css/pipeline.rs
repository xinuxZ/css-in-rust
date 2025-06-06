use crate::theme::core::cache::CacheManager;
use crate::theme::core::css::{CssObject, StyleProcessor};
use crate::theme::core::optimize::{OptimizeConfig, StyleOptimizer};
use crate::theme::core::transform::{Transformer, TransformerRegistry};
use std::sync::Arc;

/// 样式处理管道
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
    pub fn new() -> Self {
        Self {
            processor: StyleProcessor::new(),
            optimizer: None,
            cache_manager: None,
        }
    }

    /// 设置样式处理器
    pub fn with_processor(mut self, processor: StyleProcessor) -> Self {
        self.processor = processor;
        self
    }

    /// 设置样式优化器
    pub fn with_optimizer(mut self, optimizer: StyleOptimizer) -> Self {
        self.optimizer = Some(optimizer);
        self
    }

    /// 设置缓存管理器
    pub fn with_cache_manager(mut self, cache_manager: Arc<CacheManager>) -> Self {
        self.cache_manager = Some(cache_manager);
        self
    }

    /// 注册转换器
    pub fn register_transformer<T: Transformer + 'static>(&mut self, transformer: T) {
        self.processor.register_transformer(transformer);
    }

    /// 处理 CSS 对象
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
    fn generate_class_name(&self, css: &str) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(css.as_bytes());
        let hash = hasher.finalize();

        format!("css-{}", hex::encode(&hash[..4]))
    }
}

/// 处理后的样式
pub struct ProcessedStyle {
    /// CSS 类名
    pub class_name: String,
    /// CSS 字符串
    pub css: String,
    /// CSS 对象
    pub css_object: CssObject,
}

/// 样式处理管道构建器
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
    pub fn new() -> Self {
        Self {
            transformers: TransformerRegistry::new(),
            enable_optimization: true,
            enable_caching: true,
        }
    }

    /// 添加转换器
    pub fn add_transformer<T: Transformer + 'static>(mut self, transformer: T) -> Self {
        self.transformers.register(transformer);
        self
    }

    /// 启用或禁用优化
    pub fn with_optimization(mut self, enable: bool) -> Self {
        self.enable_optimization = enable;
        self
    }

    /// 启用或禁用缓存
    pub fn with_caching(mut self, enable: bool) -> Self {
        self.enable_caching = enable;
        self
    }

    /// 构建样式处理管道
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
