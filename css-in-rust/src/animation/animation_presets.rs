//! # 动画预设模块
//!
//! 提供 Ant Design 标准动画预设，包括淡入淡出、滑动、缩放等常用动画效果。

use crate::animation::{
    AnimationConfig, AnimationDirection, AnimationFillMode, AnimationIterationCount,
    AnimationPlayState, EasingFactory, EasingFunction,
};
use std::collections::HashMap;
use std::time::Duration;

/// 动画预设集合
#[derive(Debug, Clone)]
pub struct AnimationPresets {
    presets: HashMap<String, AnimationConfig>,
}

impl AnimationPresets {
    /// 创建标准预设
    ///
    /// 初始化一个包含Ant Design标准动画预设的集合，包括淡入淡出、滑动、缩放等常用动画效果。
    ///
    /// # 返回值
    ///
    /// 返回一个包含标准预设的`AnimationPresets`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationPresets;
    ///
    /// // 创建标准预设集合
    /// let presets = AnimationPresets::standard();
    ///
    /// // 列出所有可用的预设
    /// let available_presets = presets.list_presets();
    /// println!("可用的预设: {:?}", available_presets);
    ///
    /// // 获取特定预设
    /// if let Some(fade_in) = presets.get("fade-in") {
    ///     println!("淡入动画持续时间: {:?}", fade_in.duration);
    /// }
    /// ```
    pub fn standard() -> Self {
        let mut presets = HashMap::new();

        // 淡入淡出动画
        presets.insert(
            "fade-in".to_string(),
            AnimationConfig {
                name: "fade-in".to_string(),
                duration: Duration::from_millis(200),
                easing: EasingFactory::exit(),
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
                easing: EasingFactory::enter(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFactory::exit(),
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
                easing: EasingFactory::enter(),
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
                easing: EasingFactory::bounce(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFactory::standard(),
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
                easing: EasingFunction::Css("linear".to_string()),
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
    ///
    /// 通过名称获取特定的预设动画配置。
    ///
    /// # 参数
    ///
    /// * `name` - 预设动画的名称
    ///
    /// # 返回值
    ///
    /// 如果找到匹配的预设，则返回`Some(AnimationConfig)`；否则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationPresets, AnimationEngine};
    ///
    /// // 创建预设集合
    /// let presets = AnimationPresets::standard();
    ///
    /// // 获取淡入动画预设
    /// if let Some(fade_in) = presets.get("fade-in") {
    ///     // 创建动画引擎
    ///     let engine = AnimationEngine::new();
    ///
    ///     // 生成CSS
    ///     let css = engine.generate_css(&fade_in);
    ///     println!("淡入动画CSS: {}", css);
    /// } else {
    ///     println!("未找到淡入动画预设");
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<AnimationConfig> {
        self.presets.get(name).cloned()
    }

    /// 获取所有预设名称
    ///
    /// 返回所有可用预设动画的名称列表。
    ///
    /// # 返回值
    ///
    /// 返回包含所有预设名称的字符串向量。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationPresets;
    ///
    /// // 创建预设集合
    /// let presets = AnimationPresets::standard();
    ///
    /// // 获取所有预设名称
    /// let names = presets.list_presets();
    ///
    /// // 打印可用预设
    /// println!("可用的动画预设:");
    /// for name in names {
    ///     println!("- {}", name);
    /// }
    ///
    /// // 可以用于UI中显示可选动画
    /// // 例如，生成下拉选择框选项
    /// ```
    pub fn list_presets(&self) -> Vec<String> {
        self.presets.keys().cloned().collect()
    }

    /// 添加自定义预设
    ///
    /// 向预设集合中添加一个自定义的动画配置。
    ///
    /// # 参数
    ///
    /// * `config` - 要添加的动画配置
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationPresets, AnimationConfig, EasingFactory};
    /// use css_in_rust::animation::{AnimationDirection, AnimationFillMode, AnimationIterationCount, AnimationPlayState};
    /// use std::time::Duration;
    ///
    /// // 创建预设集合
    /// let mut presets = AnimationPresets::standard();
    ///
    /// // 创建自定义动画配置
    /// let custom_animation = AnimationConfig {
    ///     name: "custom-fade".to_string(),
    ///     duration: Duration::from_millis(400),
    ///     easing: EasingFactory::cubic_bezier(0.1, 0.7, 1.0, 0.1),
    ///     delay: Duration::from_millis(100),
    ///     iteration_count: AnimationIterationCount::Count(2),
    ///     direction: AnimationDirection::Alternate,
    ///     fill_mode: AnimationFillMode::Forwards,
    ///     play_state: AnimationPlayState::Running,
    /// };
    ///
    /// // 添加自定义预设
    /// presets.add_preset(custom_animation);
    ///
    /// // 验证添加成功
    /// assert!(presets.has_preset("custom-fade"));
    /// ```
    pub fn add_preset(&mut self, config: AnimationConfig) {
        self.presets.insert(config.name.clone(), config);
    }

    /// 移除预设
    ///
    /// 从预设集合中移除指定名称的动画预设。
    ///
    /// # 参数
    ///
    /// * `name` - 要移除的预设名称
    ///
    /// # 返回值
    ///
    /// 如果成功移除预设，则返回`Some(AnimationConfig)`，包含被移除的预设；
    /// 如果预设不存在，则返回`None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationPresets;
    ///
    /// // 创建预设集合
    /// let mut presets = AnimationPresets::standard();
    ///
    /// // 移除一个预设
    /// if let Some(removed) = presets.remove_preset("shake") {
    ///     println!("已移除预设: {}", removed.name);
    /// } else {
    ///     println!("预设不存在");
    /// }
    ///
    /// // 验证移除成功
    /// assert!(!presets.has_preset("shake"));
    /// ```
    pub fn remove_preset(&mut self, name: &str) -> Option<AnimationConfig> {
        self.presets.remove(name)
    }

    /// 检查预设是否存在
    ///
    /// 检查指定名称的动画预设是否存在于集合中。
    ///
    /// # 参数
    ///
    /// * `name` - 要检查的预设名称
    ///
    /// # 返回值
    ///
    /// 如果预设存在，则返回`true`；否则返回`false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationPresets;
    ///
    /// // 创建预设集合
    /// let presets = AnimationPresets::standard();
    ///
    /// // 检查预设是否存在
    /// if presets.has_preset("fade-in") {
    ///     println!("淡入动画预设可用");
    /// } else {
    ///     println!("淡入动画预设不可用");
    /// }
    ///
    /// // 可以用于条件渲染或功能检查
    /// let animation_name = if presets.has_preset("bounce-in") {
    ///     "bounce-in"
    /// } else {
    ///     "fade-in" // 回退到默认动画
    /// };
    /// ```
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
    ///
    /// 返回特定动画类别中包含的所有预设动画名称列表。
    ///
    /// # 返回值
    ///
    /// 返回包含该类别下所有预设名称的静态字符串数组。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{AnimationCategory, AnimationPresets};
    ///
    /// // 获取淡入淡出类别下的所有预设
    /// let fade_presets = AnimationCategory::Fade.get_presets();
    /// println!("淡入淡出预设: {:?}", fade_presets);
    ///
    /// // 创建预设集合
    /// let presets = AnimationPresets::standard();
    ///
    /// // 获取滑动类别下的所有预设，并检查它们是否可用
    /// for preset_name in AnimationCategory::Slide.get_presets() {
    ///     if presets.has_preset(preset_name) {
    ///         println!("可用的滑动动画: {}", preset_name);
    ///     }
    /// }
    /// ```
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
    ///
    /// 返回动画类别的中文描述，用于UI显示或日志记录。
    ///
    /// # 返回值
    ///
    /// 返回表示该类别的中文描述字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::AnimationCategory;
    ///
    /// // 获取各类别的描述
    /// let categories = vec![
    ///     AnimationCategory::Fade,
    ///     AnimationCategory::Slide,
    ///     AnimationCategory::Zoom,
    ///     AnimationCategory::Bounce,
    /// ];
    ///
    /// // 打印类别描述
    /// println!("可用的动画类别:");
    /// for category in categories {
    ///     println!("- {}", category.description());
    /// }
    ///
    /// // 可以用于生成UI选择器
    /// // 例如，创建类别下拉菜单
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            AnimationCategory::Fade => "淡入淡出",
            AnimationCategory::Slide => "滑动",
            AnimationCategory::Zoom => "缩放",
            AnimationCategory::Bounce => "弹性",
            AnimationCategory::Rotate => "旋转",
            AnimationCategory::Shake => "摇摆",
            AnimationCategory::Pulse => "脉冲",
            AnimationCategory::Custom => "自定义",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_presets() {
        let presets = AnimationPresets::standard();
        assert!(presets.has_preset("fade-in"));
        assert!(presets.has_preset("fade-out"));
        assert!(presets.has_preset("slide-up"));
        assert!(presets.has_preset("slide-down"));
        assert!(presets.has_preset("zoom-in"));
        assert!(presets.has_preset("zoom-out"));
        assert!(presets.has_preset("bounce-in"));
        assert!(presets.has_preset("shake"));
        assert!(presets.has_preset("pulse"));
        assert!(presets.has_preset("rotate"));
    }

    #[test]
    fn test_get_preset() {
        let presets = AnimationPresets::standard();
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
