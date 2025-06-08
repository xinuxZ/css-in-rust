//! # 动画引擎模块
//!
//! 负责动画的生成、优化和管理，提供高性能的动画处理能力。

use super::*;
use crate::performance::cache::CacheManager;
use crate::performance::CacheConfig;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 动画引擎
#[derive(Clone)]
pub struct AnimationEngine {
    /// 缓存管理器
    cache: Arc<Mutex<CacheManager>>,
    /// 关键帧注册表
    keyframes_registry: Arc<Mutex<HashMap<String, Keyframes>>>,
    /// 性能配置
    performance_config: AnimationPerformanceConfig,
}

/// 动画性能配置
#[derive(Debug, Clone)]
pub struct AnimationPerformanceConfig {
    /// 是否启用硬件加速
    pub enable_hardware_acceleration: bool,
    /// 是否启用 CSS 优化
    pub enable_css_optimization: bool,
    /// 最大并发动画数量
    pub max_concurrent_animations: usize,
    /// 是否启用缓存
    pub enable_caching: bool,
}

impl Default for AnimationPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_hardware_acceleration: true,
            enable_css_optimization: true,
            max_concurrent_animations: 50,
            enable_caching: true,
        }
    }
}

impl AnimationEngine {
    /// 创建新的动画引擎
    ///
    /// 初始化一个新的动画引擎实例，使用默认的性能配置。
    ///
    /// # 返回值
    ///
    /// 返回一个新的`AnimationEngine`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationEngine;
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 现在可以使用引擎生成动画CSS
    /// ```
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(CacheManager::new(CacheConfig::default()))),
            keyframes_registry: Arc::new(Mutex::new(HashMap::new())),
            performance_config: AnimationPerformanceConfig::default(),
        }
    }

    /// 使用自定义性能配置创建引擎
    ///
    /// 初始化一个新的动画引擎实例，使用自定义的性能配置。
    ///
    /// # 参数
    ///
    /// * `config` - 自定义的动画性能配置
    ///
    /// # 返回值
    ///
    /// 返回一个新的`AnimationEngine`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationPerformanceConfig};
    ///
    /// // 创建自定义性能配置
    /// let mut config = AnimationPerformanceConfig::default();
    /// config.enable_hardware_acceleration = false;
    /// config.enable_caching = true;
    /// config.max_concurrent_animations = 100;
    ///
    /// // 使用自定义配置创建动画引擎
    /// let engine = AnimationEngine::with_config(config);
    /// ```
    pub fn with_config(config: AnimationPerformanceConfig) -> Self {
        Self {
            cache: Arc::new(Mutex::new(CacheManager::new(CacheConfig::default()))),
            keyframes_registry: Arc::new(Mutex::new(HashMap::new())),
            performance_config: config,
        }
    }

    /// 生成动画 CSS
    ///
    /// 根据提供的动画配置生成CSS代码。如果启用了缓存，会尝试从缓存中获取结果。
    ///
    /// # 参数
    ///
    /// * `config` - 动画配置
    ///
    /// # 返回值
    ///
    /// 返回生成的CSS字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationConfig, EasingFactory};
    /// use css_in_rust::animation::{AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState};
    /// use std::time::Duration;
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 创建动画配置
    /// let config = AnimationConfig {
    ///     name: "fade-in".to_string(),
    ///     duration: Duration::from_millis(300),
    ///     easing: EasingFactory::standard(),
    ///     delay: Duration::from_millis(0),
    ///     iteration_count: AnimationIterationCount::Count(1),
    ///     direction: AnimationDirection::Normal,
    ///     fill_mode: AnimationFillMode::Both,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// // 生成CSS
    /// let css = engine.generate_css(&config);
    /// println!("生成的CSS: {}", css);
    /// ```
    pub fn generate_css(&self, config: &AnimationConfig) -> String {
        let cache_key = self.generate_cache_key(config);

        // 尝试从缓存获取
        if self.performance_config.enable_caching {
            if let Ok(mut cache) = self.cache.lock() {
                // 为缓存键生成哈希值
                let source_hash = format!("{:?}", config);
                let config_hash = "animation_config".to_string();
                if let Some(cached_css) = cache.get(&cache_key, &source_hash, &config_hash) {
                    return cached_css.clone();
                }
            }
        }

        // 生成 CSS
        let css = self.generate_animation_css(config);

        // 缓存结果
        if self.performance_config.enable_caching {
            if let Ok(mut cache) = self.cache.lock() {
                let source_hash = format!("{:?}", config);
                let config_hash = "animation_config".to_string();
                cache.set(cache_key, css.clone(), source_hash, config_hash);
            }
        }

        css
    }

    /// 注册关键帧
    ///
    /// 将关键帧定义注册到动画引擎中，使其可以被动画使用。
    ///
    /// # 参数
    ///
    /// * `keyframes` - 要注册的关键帧定义
    ///
    /// # 返回值
    ///
    /// 成功时返回`Ok(())`，失败时返回包含错误信息的`Err(String)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, KeyframesBuilder};
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 创建关键帧定义
    /// let keyframes = KeyframesBuilder::new("bounce")
    ///     .step(0)
    ///     .property("transform", "scale(0.3)")
    ///     .property("opacity", "0")
    ///     .step(50)
    ///     .property("transform", "scale(1.05)")
    ///     .property("opacity", "1")
    ///     .step(100)
    ///     .property("transform", "scale(1)")
    ///     .build();
    ///
    /// // 注册关键帧
    /// match engine.register_keyframes(keyframes) {
    ///     Ok(_) => println!("关键帧注册成功"),
    ///     Err(e) => println!("关键帧注册失败: {}", e),
    /// }
    /// ```
    pub fn register_keyframes(&self, keyframes: Keyframes) -> Result<(), String> {
        keyframes.validate()?;

        if let Ok(mut registry) = self.keyframes_registry.lock() {
            registry.insert(keyframes.name.clone(), keyframes);
            Ok(())
        } else {
            Err("无法获取关键帧注册表锁".to_string())
        }
    }

    /// 获取关键帧
    ///
    /// 通过名称获取已注册的关键帧定义。
    ///
    /// # 参数
    ///
    /// * `name` - 要获取的关键帧名称
    ///
    /// # 返回值
    ///
    /// 如果找到匹配的关键帧，则返回`Some(Keyframes)`；否则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, PredefinedKeyframes};
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 注册预定义关键帧
    /// let fade_in = PredefinedKeyframes::fade_in();
    /// engine.register_keyframes(fade_in).unwrap();
    ///
    /// // 获取已注册的关键帧
    /// if let Some(keyframes) = engine.get_keyframes("fade-in") {
    ///     println!("找到关键帧: {}", keyframes.name);
    ///
    ///     // 可以生成关键帧的CSS
    ///     let css = keyframes.to_css();
    ///     println!("关键帧CSS: {}", css);
    /// }
    /// ```
    pub fn get_keyframes(&self, name: &str) -> Option<Keyframes> {
        if let Ok(registry) = self.keyframes_registry.lock() {
            registry.get(name).cloned()
        } else {
            None
        }
    }

    /// 生成完整的动画样式表
    ///
    /// 根据提供的多个动画配置生成完整的CSS样式表，包括所有注册的关键帧和动画类。
    ///
    /// # 参数
    ///
    /// * `animations` - 动画配置数组
    ///
    /// # 返回值
    ///
    /// 返回生成的完整CSS样式表字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationConfig, EasingFactory};
    /// use css_in_rust::animation::{AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState};
    /// use std::time::Duration;
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 创建多个动画配置
    /// let fade_in = AnimationConfig {
    ///     name: "fade-in".to_string(),
    ///     duration: Duration::from_millis(300),
    ///     easing: EasingFactory::standard(),
    ///     delay: Duration::from_millis(0),
    ///     iteration_count: AnimationIterationCount::Count(1),
    ///     direction: AnimationDirection::Normal,
    ///     fill_mode: AnimationFillMode::Both,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// let fade_out = AnimationConfig {
    ///     name: "fade-out".to_string(),
    ///     duration: Duration::from_millis(200),
    ///     easing: EasingFactory::standard(),
    ///     delay: Duration::from_millis(0),
    ///     iteration_count: AnimationIterationCount::Count(1),
    ///     direction: AnimationDirection::Normal,
    ///     fill_mode: AnimationFillMode::Both,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// // 生成完整样式表
    /// let stylesheet = engine.generate_stylesheet(&[fade_in, fade_out]);
    /// println!("生成的样式表: {}", stylesheet);
    ///
    /// // 可以将样式表注入到文档中
    /// // css_in_rust::runtime::inject_style(&stylesheet, "animations");
    /// ```
    pub fn generate_stylesheet(&self, animations: &[AnimationConfig]) -> String {
        let mut stylesheet = String::new();

        // 生成关键帧
        if let Ok(registry) = self.keyframes_registry.lock() {
            for keyframes in registry.values() {
                stylesheet.push_str(&keyframes.to_css());
                stylesheet.push('\n');
            }
        }

        // 生成动画类
        for config in animations {
            let class_name = format!(".{}", config.name);
            let animation_css = self.generate_animation_css(config);
            stylesheet.push_str(&format!("{}{{{}}}\n", class_name, animation_css));
        }

        if self.performance_config.enable_css_optimization {
            self.optimize_css(&stylesheet)
        } else {
            stylesheet
        }
    }

    /// 生成动画 CSS 属性
    fn generate_animation_css(&self, config: &AnimationConfig) -> String {
        let mut properties = Vec::new();

        // animation-name
        properties.push(format!("animation-name: {}", config.name));

        // animation-duration
        properties.push(format!(
            "animation-duration: {}ms",
            config.duration.as_millis()
        ));

        // animation-timing-function
        properties.push(format!(
            "animation-timing-function: {}",
            config.easing.to_css()
        ));

        // animation-delay
        if config.delay.as_millis() > 0 {
            properties.push(format!("animation-delay: {}ms", config.delay.as_millis()));
        }

        // animation-iteration-count
        let iteration_count = match &config.iteration_count {
            AnimationIterationCount::Count(n) => n.to_string(),
            AnimationIterationCount::Infinite => "infinite".to_string(),
        };
        properties.push(format!("animation-iteration-count: {}", iteration_count));

        // animation-direction
        let direction = match config.direction {
            AnimationDirection::Normal => "normal",
            AnimationDirection::Reverse => "reverse",
            AnimationDirection::Alternate => "alternate",
            AnimationDirection::AlternateReverse => "alternate-reverse",
        };
        properties.push(format!("animation-direction: {}", direction));

        // animation-fill-mode
        let fill_mode = match config.fill_mode {
            AnimationFillMode::None => "none",
            AnimationFillMode::Forwards => "forwards",
            AnimationFillMode::Backwards => "backwards",
            AnimationFillMode::Both => "both",
        };
        properties.push(format!("animation-fill-mode: {}", fill_mode));

        // animation-play-state
        let play_state = match config.play_state {
            AnimationPlayState::Running => "running",
            AnimationPlayState::Paused => "paused",
        };
        properties.push(format!("animation-play-state: {}", play_state));

        // 硬件加速优化
        if self.performance_config.enable_hardware_acceleration {
            properties.push("will-change: transform, opacity".to_string());
            properties.push("transform: translateZ(0)".to_string());
        }

        properties.join("; ")
    }

    /// 生成缓存键
    fn generate_cache_key(&self, config: &AnimationConfig) -> String {
        format!(
            "{}:{}:{}:{}:{}:{:?}:{:?}:{:?}:{:?}",
            config.name,
            config.duration.as_millis(),
            config.easing.to_css(),
            config.delay.as_millis(),
            match &config.iteration_count {
                AnimationIterationCount::Count(n) => n.to_string(),
                AnimationIterationCount::Infinite => "infinite".to_string(),
            },
            config.direction,
            config.fill_mode,
            config.play_state,
            self.performance_config.enable_hardware_acceleration
        )
    }

    /// 优化 CSS
    fn optimize_css(&self, css: &str) -> String {
        // 简单的 CSS 优化：移除多余空白和注释
        css.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("/*"))
            .collect::<Vec<_>>()
            .join("")
    }

    /// 清除缓存
    ///
    /// 清除动画引擎的内部缓存，释放内存并确保下次生成CSS时重新计算。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationEngine;
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 生成一些动画CSS后，可能需要清除缓存
    /// // ...
    ///
    /// // 清除缓存
    /// engine.clear_cache();
    /// println!("缓存已清除");
    /// ```
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// 获取缓存统计
    ///
    /// 获取当前缓存的使用情况统计，包括缓存项数量和容量。
    ///
    /// # 返回值
    ///
    /// 如果成功获取缓存锁，返回`Some((size, capacity))`，其中`size`是当前缓存项数量，
    /// `capacity`是缓存容量；如果获取锁失败，则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationEngine;
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 检查缓存状态
    /// if let Some((size, capacity)) = engine.get_cache_stats() {
    ///     println!("缓存统计: {} 项 / {} 容量", size, capacity);
    ///
    ///     // 如果缓存使用率过高，可以清除缓存
    ///     if size > capacity * 80 / 100 {  // 超过80%使用率
    ///         engine.clear_cache();
    ///         println!("缓存使用率过高，已清除");
    ///     }
    /// }
    /// ```
    pub fn get_cache_stats(&self) -> Option<(usize, usize)> {
        if let Ok(cache) = self.cache.lock() {
            Some((cache.len(), cache.capacity()))
        } else {
            None
        }
    }

    /// 预加载动画
    ///
    /// 预先生成并缓存多个动画的CSS，以提高后续使用时的性能。
    ///
    /// # 参数
    ///
    /// * `configs` - 要预加载的动画配置数组
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationPresets};
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 获取标准预设
    /// let presets = AnimationPresets::standard();
    ///
    /// // 预加载常用动画
    /// let animations = vec![
    ///     presets.get("fade-in").unwrap(),
    ///     presets.get("fade-out").unwrap(),
    ///     presets.get("slide-up").unwrap(),
    ///     presets.get("slide-down").unwrap(),
    /// ];
    ///
    /// // 预加载动画
    /// engine.preload_animations(&animations);
    /// println!("常用动画已预加载");
    /// ```
    pub fn preload_animations(&self, configs: &[AnimationConfig]) {
        for config in configs {
            self.generate_css(config);
        }
    }
}

/// 动画批处理器
///
/// 用于批量处理多个动画，提高性能并简化动画管理。
/// 可以一次性生成多个动画的CSS，避免多次调用引擎的开销。
pub struct AnimationBatch {
    animations: Vec<AnimationConfig>,
    engine: AnimationEngine,
}

impl AnimationBatch {
    /// 创建新的批处理器
    ///
    /// 使用指定的动画引擎初始化一个新的批处理器实例。
    ///
    /// # 参数
    ///
    /// * `engine` - 用于生成动画CSS的动画引擎
    ///
    /// # 返回值
    ///
    /// 返回一个新的`AnimationBatch`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch};
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 创建批处理器
    /// let batch = AnimationBatch::new(engine);
    ///
    /// // 现在可以向批处理器添加动画
    /// ```
    pub fn new(engine: AnimationEngine) -> Self {
        Self {
            animations: Vec::new(),
            engine,
        }
    }

    /// 添加动画
    ///
    /// 向批处理器添加一个动画配置。
    ///
    /// # 参数
    ///
    /// * `config` - 要添加的动画配置
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch, AnimationConfig, EasingFactory};
    /// use css_in_rust::animation::{AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState};
    /// use std::time::Duration;
    ///
    /// // 创建批处理器
    /// let mut batch = AnimationBatch::new(AnimationEngine::new());
    ///
    /// // 创建动画配置
    /// let fade_in = AnimationConfig {
    ///     name: "fade-in".to_string(),
    ///     duration: Duration::from_millis(300),
    ///     easing: EasingFactory::standard(),
    ///     delay: Duration::from_millis(0),
    ///     iteration_count: AnimationIterationCount::Count(1),
    ///     direction: AnimationDirection::Normal,
    ///     fill_mode: AnimationFillMode::Both,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// // 添加动画到批处理器
    /// batch.add_animation(fade_in);
    /// ```
    pub fn add_animation(&mut self, config: AnimationConfig) {
        self.animations.push(config);
    }

    /// 批量生成 CSS
    ///
    /// 为所有添加的动画生成完整的CSS样式表。
    ///
    /// # 返回值
    ///
    /// 返回包含所有动画的CSS样式表字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch, AnimationPresets};
    ///
    /// // 创建批处理器
    /// let mut batch = AnimationBatch::new(AnimationEngine::new());
    ///
    /// // 获取预设动画
    /// let presets = AnimationPresets::standard();
    ///
    /// // 添加多个动画
    /// batch.add_animation(presets.get("fade-in").unwrap());
    /// batch.add_animation(presets.get("slide-up").unwrap());
    /// batch.add_animation(presets.get("zoom-in").unwrap());
    ///
    /// // 生成所有动画的CSS
    /// let css = batch.generate_css();
    /// println!("生成的CSS样式表: {}", css);
    ///
    /// // 可以将CSS注入到文档中
    /// // css_in_rust::runtime::inject_style(&css, "animations-bundle");
    /// ```
    pub fn generate_css(&self) -> String {
        self.engine.generate_stylesheet(&self.animations)
    }

    /// 清空批处理
    ///
    /// 移除批处理器中的所有动画配置。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch, AnimationPresets};
    ///
    /// // 创建批处理器并添加动画
    /// let mut batch = AnimationBatch::new(AnimationEngine::new());
    /// let presets = AnimationPresets::standard();
    /// batch.add_animation(presets.get("fade-in").unwrap());
    ///
    /// // 清空批处理器
    /// batch.clear();
    /// assert_eq!(batch.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.animations.clear();
    }

    /// 获取动画数量
    ///
    /// 返回批处理器中当前的动画配置数量。
    ///
    /// # 返回值
    ///
    /// 返回批处理器中的动画数量。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch, AnimationPresets};
    ///
    /// // 创建批处理器
    /// let mut batch = AnimationBatch::new(AnimationEngine::new());
    /// assert_eq!(batch.len(), 0);
    ///
    /// // 添加动画
    /// let presets = AnimationPresets::standard();
    /// batch.add_animation(presets.get("fade-in").unwrap());
    /// assert_eq!(batch.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.animations.len()
    }

    /// 检查是否为空
    ///
    /// 检查批处理器是否不包含任何动画配置。
    ///
    /// # 返回值
    ///
    /// 如果批处理器不包含任何动画，则返回`true`；否则返回`false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationEngine, AnimationBatch};
    ///
    /// // 创建批处理器
    /// let mut batch = AnimationBatch::new(AnimationEngine::new());
    /// assert!(batch.is_empty());
    ///
    /// // 添加动画后不再为空
    /// // batch.add_animation(...);
    /// // assert!(!batch.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }
}

impl Default for AnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_animation_engine_creation() {
        let engine = AnimationEngine::new();
        assert!(engine.get_cache_stats().is_some());
    }

    #[test]
    fn test_css_generation() {
        let engine = AnimationEngine::new();
        let config = AnimationConfig {
            name: "test".to_string(),
            duration: Duration::from_millis(300),
            easing: EasingFunction::Css("linear".to_string()),
            delay: Duration::from_millis(0),
            iteration_count: AnimationIterationCount::Count(1),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::Both,
            play_state: AnimationPlayState::Running,
        };

        let css = engine.generate_css(&config);
        assert!(css.contains("animation-name: test"));
        assert!(css.contains("animation-duration: 300ms"));
    }

    #[test]
    fn test_keyframes_registration() {
        let engine = AnimationEngine::new();
        let keyframes = PredefinedKeyframes::fade_in();

        assert!(engine.register_keyframes(keyframes.clone()).is_ok());
        assert!(engine.get_keyframes("fade-in").is_some());
    }

    #[test]
    fn test_animation_batch() {
        let engine = AnimationEngine::new();
        let mut batch = AnimationBatch::new(engine);

        assert!(batch.is_empty());

        let config = AnimationConfig {
            name: "test".to_string(),
            duration: Duration::from_millis(300),
            easing: EasingFunction::Css("linear".to_string()),
            delay: Duration::from_millis(0),
            iteration_count: AnimationIterationCount::Count(1),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::Both,
            play_state: AnimationPlayState::Running,
        };

        batch.add_animation(config);
        assert_eq!(batch.len(), 1);
        assert!(!batch.is_empty());
    }
}
