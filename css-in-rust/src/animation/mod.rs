//! # 动画系统模块
//!
//! 提供完整的 CSS 动画支持，包括预定义动画、动画组合、性能优化等功能。
//! 支持 Ant Design 动画规范和自定义动画效果。

pub mod animation_engine;
pub mod animation_presets;
pub mod easing;
pub mod keyframes;

// 重新导出主要类型
pub use animation_engine::*;
pub use animation_presets::*;
pub use easing::*;
pub use keyframes::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// 动画配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationConfig {
    /// 动画名称
    pub name: String,
    /// 动画持续时间
    pub duration: Duration,
    /// 缓动函数
    pub easing: EasingFunction,
    /// 延迟时间
    pub delay: Duration,
    /// 重复次数
    pub iteration_count: AnimationIterationCount,
    /// 动画方向
    pub direction: AnimationDirection,
    /// 填充模式
    pub fill_mode: AnimationFillMode,
    /// 播放状态
    pub play_state: AnimationPlayState,
}

/// 动画重复次数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnimationIterationCount {
    /// 指定次数
    Count(u32),
    /// 无限循环
    Infinite,
}

/// 动画方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// 动画填充模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

/// 动画播放状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnimationPlayState {
    Running,
    Paused,
}

/// 动画管理器
///
/// 用于管理和组织应用程序中的动画，提供统一的接口访问自定义动画和预设动画。
#[derive(Clone)]
pub struct AnimationManager {
    /// 注册的动画
    animations: HashMap<String, AnimationConfig>,
    /// 动画引擎
    engine: AnimationEngine,
    /// 预设动画
    presets: AnimationPresets,
}

impl AnimationManager {
    /// 创建新的动画管理器
    ///
    /// 初始化一个新的动画管理器实例，包含标准预设动画和默认配置的动画引擎。
    ///
    /// # 返回值
    ///
    /// 返回一个新的`AnimationManager`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationManager;
    ///
    /// // 创建动画管理器
    /// let manager = AnimationManager::new();
    ///
    /// // 现在可以使用管理器注册和生成动画
    /// ```
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            engine: AnimationEngine::new(),
            presets: AnimationPresets::standard(),
        }
    }

    /// 注册动画
    ///
    /// 向管理器注册一个自定义动画配置，使其可以通过名称引用。
    ///
    /// # 参数
    ///
    /// * `config` - 要注册的动画配置
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationManager, AnimationConfig, EasingFactory};
    /// use css_in_rust::animation::{AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState};
    /// use std::time::Duration;
    ///
    /// let mut manager = AnimationManager::new();
    ///
    /// // 创建自定义动画配置
    /// let custom_animation = AnimationConfig {
    ///     name: "custom-fade".to_string(),
    ///     duration: Duration::from_millis(300),
    ///     easing: EasingFactory::standard(),
    ///     delay: Duration::from_millis(0),
    ///     iteration_count: AnimationIterationCount::Count(1),
    ///     direction: AnimationDirection::Normal,
    ///     fill_mode: AnimationFillMode::Both,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// // 注册动画
    /// manager.register_animation(custom_animation);
    /// ```
    pub fn register_animation(&mut self, config: AnimationConfig) {
        self.animations.insert(config.name.clone(), config);
    }

    /// 获取动画配置
    ///
    /// 通过名称获取已注册的动画配置。
    ///
    /// # 参数
    ///
    /// * `name` - 要获取的动画名称
    ///
    /// # 返回值
    ///
    /// 如果找到匹配的动画配置，则返回`Some(AnimationConfig)`；否则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationManager;
    ///
    /// let manager = AnimationManager::new();
    ///
    /// // 获取已注册的动画配置
    /// if let Some(config) = manager.get_animation("custom-fade") {
    ///     println!("找到动画: {}, 持续时间: {}毫秒",
    ///              config.name, config.duration.as_millis());
    /// } else {
    ///     println!("未找到动画");
    /// }
    /// ```
    pub fn get_animation(&self, name: &str) -> Option<&AnimationConfig> {
        self.animations.get(name)
    }

    /// 生成动画 CSS
    ///
    /// 根据动画名称生成对应的CSS代码。
    ///
    /// # 参数
    ///
    /// * `name` - 要生成CSS的动画名称
    ///
    /// # 返回值
    ///
    /// 如果找到匹配的动画，则返回`Some(String)`，包含生成的CSS代码；否则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationManager;
    ///
    /// let manager = AnimationManager::new();
    ///
    /// // 生成动画CSS
    /// if let Some(css) = manager.generate_css("fade-in") {
    ///     println!("生成的CSS: {}", css);
    ///
    ///     // 可以将CSS注入到文档中
    ///     // css_in_rust::runtime::inject_style(&css, "fade-in-animation");
    /// }
    /// ```
    pub fn generate_css(&self, name: &str) -> Option<String> {
        self.get_animation(name)
            .map(|config| self.engine.generate_css(config))
    }

    /// 获取预设动画
    ///
    /// 获取预定义的动画预设配置。
    ///
    /// # 参数
    ///
    /// * `preset` - 预设动画的名称
    ///
    /// # 返回值
    ///
    /// 如果找到匹配的预设，则返回`Some(AnimationConfig)`；否则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationManager;
    ///
    /// let manager = AnimationManager::new();
    ///
    /// // 获取预设动画配置
    /// if let Some(preset) = manager.get_preset("fade-in") {
    ///     // 使用预设创建自定义动画
    ///     let mut custom = preset.clone();
    ///     custom.name = "my-fade-in".to_string();
    ///
    ///     // 注册自定义动画
    ///     manager.register_animation(custom);
    /// }
    /// ```
    pub fn get_preset(&self, preset: &str) -> Option<AnimationConfig> {
        self.presets.get(preset)
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
