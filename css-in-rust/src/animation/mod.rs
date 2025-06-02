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
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            engine: AnimationEngine::new(),
            presets: AnimationPresets::standard(),
        }
    }

    /// 注册动画
    pub fn register_animation(&mut self, config: AnimationConfig) {
        self.animations.insert(config.name.clone(), config);
    }

    /// 获取动画配置
    pub fn get_animation(&self, name: &str) -> Option<&AnimationConfig> {
        self.animations.get(name)
    }

    /// 生成动画 CSS
    pub fn generate_css(&self, name: &str) -> Option<String> {
        self.get_animation(name)
            .map(|config| self.engine.generate_css(config))
    }

    /// 获取预设动画
    pub fn get_preset(&self, preset: &str) -> Option<AnimationConfig> {
        self.presets.get(preset)
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
