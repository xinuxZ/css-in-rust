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
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(CacheManager::new(CacheConfig::default()))),
            keyframes_registry: Arc::new(Mutex::new(HashMap::new())),
            performance_config: AnimationPerformanceConfig::default(),
        }
    }

    /// 使用自定义性能配置创建引擎
    pub fn with_config(config: AnimationPerformanceConfig) -> Self {
        Self {
            cache: Arc::new(Mutex::new(CacheManager::new(CacheConfig::default()))),
            keyframes_registry: Arc::new(Mutex::new(HashMap::new())),
            performance_config: config,
        }
    }

    /// 生成动画 CSS
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
    pub fn get_keyframes(&self, name: &str) -> Option<Keyframes> {
        if let Ok(registry) = self.keyframes_registry.lock() {
            registry.get(name).cloned()
        } else {
            None
        }
    }

    /// 生成完整的动画样式表
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
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// 获取缓存统计
    pub fn get_cache_stats(&self) -> Option<(usize, usize)> {
        if let Ok(cache) = self.cache.lock() {
            Some((cache.len(), cache.capacity()))
        } else {
            None
        }
    }

    /// 预加载动画
    pub fn preload_animations(&self, configs: &[AnimationConfig]) {
        for config in configs {
            self.generate_css(config);
        }
    }
}

/// 动画批处理器
pub struct AnimationBatch {
    animations: Vec<AnimationConfig>,
    engine: AnimationEngine,
}

impl AnimationBatch {
    /// 创建新的批处理器
    pub fn new(engine: AnimationEngine) -> Self {
        Self {
            animations: Vec::new(),
            engine,
        }
    }

    /// 添加动画
    pub fn add_animation(&mut self, config: AnimationConfig) {
        self.animations.push(config);
    }

    /// 批量生成 CSS
    pub fn generate_css(&self) -> String {
        self.engine.generate_stylesheet(&self.animations)
    }

    /// 清空批处理
    pub fn clear(&mut self) {
        self.animations.clear();
    }

    /// 获取动画数量
    pub fn len(&self) -> usize {
        self.animations.len()
    }

    /// 检查是否为空
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
            easing: EasingFunction::Linear,
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
            easing: EasingFunction::Linear,
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
