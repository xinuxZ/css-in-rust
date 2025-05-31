//! # 动画预设模块
//!
//! 提供 Ant Design 标准动画预设，包括淡入淡出、滑动、缩放等常用动画效果。

use super::*;
use std::collections::HashMap;
use std::time::Duration;

/// 动画预设集合
#[derive(Debug, Clone)]
pub struct AnimationPresets {
    presets: HashMap<String, AnimationConfig>,
}

impl AnimationPresets {
    /// 创建 Ant Design 标准预设
    pub fn ant_design() -> Self {
        let mut presets = HashMap::new();

        // 淡入淡出动画
        presets.insert(
            "fade-in".to_string(),
            AnimationConfig {
                name: "fade-in".to_string(),
                duration: Duration::from_millis(200),
                easing: EasingFunction::AntDesign(AntDesignEasing::Decelerated),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        presets.insert(
            "fade-out".to_string(),
            AnimationConfig {
                name: "fade-out".to_string(),
                duration: Duration::from_millis(150),
                easing: EasingFunction::AntDesign(AntDesignEasing::Accelerated),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 滑动动画
        presets.insert(
            "slide-up".to_string(),
            AnimationConfig {
                name: "slide-up".to_string(),
                duration: Duration::from_millis(300),
                easing: EasingFunction::AntDesign(AntDesignEasing::Standard),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        presets.insert(
            "slide-down".to_string(),
            AnimationConfig {
                name: "slide-down".to_string(),
                duration: Duration::from_millis(300),
                easing: EasingFunction::AntDesign(AntDesignEasing::Standard),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        presets.insert(
            "slide-left".to_string(),
            AnimationConfig {
                name: "slide-left".to_string(),
                duration: Duration::from_millis(300),
                easing: EasingFunction::AntDesign(AntDesignEasing::Standard),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        presets.insert(
            "slide-right".to_string(),
            AnimationConfig {
                name: "slide-right".to_string(),
                duration: Duration::from_millis(300),
                easing: EasingFunction::AntDesign(AntDesignEasing::Standard),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 缩放动画
        presets.insert(
            "zoom-in".to_string(),
            AnimationConfig {
                name: "zoom-in".to_string(),
                duration: Duration::from_millis(200),
                easing: EasingFunction::AntDesign(AntDesignEasing::Decelerated),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        presets.insert(
            "zoom-out".to_string(),
            AnimationConfig {
                name: "zoom-out".to_string(),
                duration: Duration::from_millis(150),
                easing: EasingFunction::AntDesign(AntDesignEasing::Accelerated),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 弹性动画
        presets.insert(
            "bounce-in".to_string(),
            AnimationConfig {
                name: "bounce-in".to_string(),
                duration: Duration::from_millis(400),
                easing: EasingFunction::AntDesign(AntDesignEasing::Bounce),
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 摇摆动画
        presets.insert(
            "shake".to_string(),
            AnimationConfig {
                name: "shake".to_string(),
                duration: Duration::from_millis(500),
                easing: EasingFunction::EaseInOut,
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Count(1),
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 脉冲动画
        presets.insert(
            "pulse".to_string(),
            AnimationConfig {
                name: "pulse".to_string(),
                duration: Duration::from_millis(1000),
                easing: EasingFunction::EaseInOut,
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Infinite,
                direction: AnimationDirection::Alternate,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        // 旋转动画
        presets.insert(
            "rotate".to_string(),
            AnimationConfig {
                name: "rotate".to_string(),
                duration: Duration::from_millis(1000),
                easing: EasingFunction::Linear,
                delay: Duration::from_millis(0),
                iteration_count: AnimationIterationCount::Infinite,
                direction: AnimationDirection::Normal,
                fill_mode: AnimationFillMode::Both,
                play_state: AnimationPlayState::Running,
            },
        );

        Self { presets }
    }

    /// 获取预设动画
    pub fn get(&self, name: &str) -> Option<AnimationConfig> {
        self.presets.get(name).cloned()
    }

    /// 获取所有预设名称
    pub fn list_presets(&self) -> Vec<String> {
        self.presets.keys().cloned().collect()
    }

    /// 添加自定义预设
    pub fn add_preset(&mut self, config: AnimationConfig) {
        self.presets.insert(config.name.clone(), config);
    }

    /// 移除预设
    pub fn remove_preset(&mut self, name: &str) -> Option<AnimationConfig> {
        self.presets.remove(name)
    }

    /// 检查预设是否存在
    pub fn has_preset(&self, name: &str) -> bool {
        self.presets.contains_key(name)
    }
}

/// 动画预设类别
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationCategory {
    /// 淡入淡出
    Fade,
    /// 滑动
    Slide,
    /// 缩放
    Zoom,
    /// 弹性
    Bounce,
    /// 旋转
    Rotate,
    /// 摇摆
    Shake,
    /// 脉冲
    Pulse,
    /// 自定义
    Custom,
}

impl AnimationCategory {
    /// 获取类别下的预设动画名称
    pub fn get_presets(&self) -> Vec<&'static str> {
        match self {
            AnimationCategory::Fade => vec!["fade-in", "fade-out"],
            AnimationCategory::Slide => vec!["slide-up", "slide-down", "slide-left", "slide-right"],
            AnimationCategory::Zoom => vec!["zoom-in", "zoom-out"],
            AnimationCategory::Bounce => vec!["bounce-in"],
            AnimationCategory::Rotate => vec!["rotate"],
            AnimationCategory::Shake => vec!["shake"],
            AnimationCategory::Pulse => vec!["pulse"],
            AnimationCategory::Custom => vec![],
        }
    }

    /// 获取类别描述
    pub fn description(&self) -> &'static str {
        match self {
            AnimationCategory::Fade => "淡入淡出动画，适用于元素的显示和隐藏",
            AnimationCategory::Slide => "滑动动画，适用于元素的进入和退出",
            AnimationCategory::Zoom => "缩放动画，适用于元素的放大和缩小",
            AnimationCategory::Bounce => "弹性动画，适用于强调和反馈",
            AnimationCategory::Rotate => "旋转动画，适用于加载和状态指示",
            AnimationCategory::Shake => "摇摆动画，适用于错误提示和注意力吸引",
            AnimationCategory::Pulse => "脉冲动画，适用于状态指示和强调",
            AnimationCategory::Custom => "自定义动画",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ant_design_presets() {
        let presets = AnimationPresets::ant_design();
        assert!(presets.has_preset("fade-in"));
        assert!(presets.has_preset("slide-up"));
        assert!(presets.has_preset("zoom-in"));
        assert!(!presets.has_preset("non-existent"));
    }

    #[test]
    fn test_get_preset() {
        let presets = AnimationPresets::ant_design();
        let fade_in = presets.get("fade-in").unwrap();
        assert_eq!(fade_in.name, "fade-in");
        assert_eq!(fade_in.duration, Duration::from_millis(200));
    }

    #[test]
    fn test_animation_categories() {
        let fade_presets = AnimationCategory::Fade.get_presets();
        assert!(fade_presets.contains(&"fade-in"));
        assert!(fade_presets.contains(&"fade-out"));
    }
}
