//! # 缓动函数模块
//!
//! 提供各种动画缓动函数，包括标准缓动和自定义贝塞尔曲线。

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
    /// CSS 标准缓动函数
    Css(String),
    /// 自定义贝塞尔曲线缓动函数
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    /// 转换为 CSS 字符串
    ///
    /// 将缓动函数转换为可在CSS中使用的字符串表示形式。
    ///
    /// # 返回值
    ///
    /// 返回表示缓动函数的CSS字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::EasingFunction;
    ///
    /// // 使用CSS标准缓动函数
    /// let ease_in = EasingFunction::Css("ease-in".to_string());
    /// assert_eq!(ease_in.to_css(), "ease-in");
    ///
    /// // 使用贝塞尔曲线缓动函数
    /// let custom = EasingFunction::CubicBezier(0.42, 0.0, 0.58, 1.0);
    /// assert_eq!(custom.to_css(), "cubic-bezier(0.42, 0, 0.58, 1)");
    /// ```
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::Css(css) => css.clone(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
        }
    }

    /// 获取建议的动画持续时间（毫秒）
    ///
    /// 根据缓动函数类型返回推荐的动画持续时间。
    ///
    /// # 返回值
    ///
    /// 返回推荐的动画持续时间，单位为毫秒。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::EasingFunction;
    ///
    /// let ease_out = EasingFunction::Css("ease-out".to_string());
    /// let duration = ease_out.suggested_duration_ms();
    /// println!("推荐的动画持续时间: {}毫秒", duration); // 输出: 推荐的动画持续时间: 200毫秒
    /// ```
    pub fn suggested_duration_ms(&self) -> u32 {
        200 // 默认持续时间
    }
}

/// 缓动函数工厂
pub struct EasingFactory;

impl EasingFactory {
    /// 创建标准缓动函数
    ///
    /// 创建Ant Design标准缓动函数，适用于大多数常规过渡动画。
    ///
    /// # 返回值
    ///
    /// 返回标准缓动函数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{EasingFactory, AnimationConfig};
    /// use std::time::Duration;
    ///
    /// // 在动画配置中使用标准缓动函数
    /// let mut config = AnimationConfig {
    ///     // 其他字段...
    ///     easing: EasingFactory::standard(),
    ///     duration: Duration::from_millis(300),
    ///     // 其他字段...
    /// };
    /// ```
    pub fn standard() -> EasingFunction {
        EasingFunction::CubicBezier(0.34, 0.69, 0.1, 1.0)
    }

    /// 创建强调缓动函数
    ///
    /// 创建用于强调重要元素的缓动函数，提供更强的视觉冲击力。
    ///
    /// # 返回值
    ///
    /// 返回强调缓动函数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{EasingFactory, AnimationEngine};
    ///
    /// // 创建动画引擎
    /// let engine = AnimationEngine::new();
    ///
    /// // 使用强调缓动函数创建重要元素的动画
    /// let emphasized_easing = EasingFactory::emphasized();
    /// println!("强调缓动函数: {}", emphasized_easing.to_css());
    /// ```
    pub fn emphasized() -> EasingFunction {
        EasingFunction::CubicBezier(0.05, 0.7, 0.1, 1.0)
    }

    /// 创建进入缓动函数
    ///
    /// 创建适用于元素进入视图的缓动函数，提供平滑的加速效果。
    ///
    /// # 返回值
    ///
    /// 返回进入缓动函数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{EasingFactory, AnimationConfig};
    /// use std::time::Duration;
    ///
    /// // 为元素进入动画创建配置
    /// let fade_in_config = AnimationConfig {
    ///     name: "fade-in".to_string(),
    ///     duration: Duration::from_millis(200),
    ///     easing: EasingFactory::enter(), // 使用进入缓动函数
    ///     // 其他字段...
    /// };
    /// ```
    pub fn enter() -> EasingFunction {
        EasingFunction::CubicBezier(0.0, 0.0, 0.2, 1.0)
    }

    /// 创建退出缓动函数
    ///
    /// 创建适用于元素退出视图的缓动函数，提供平滑的减速效果。
    ///
    /// # 返回值
    ///
    /// 返回退出缓动函数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{EasingFactory, AnimationConfig};
    /// use std::time::Duration;
    ///
    /// // 为元素退出动画创建配置
    /// let fade_out_config = AnimationConfig {
    ///     name: "fade-out".to_string(),
    ///     duration: Duration::from_millis(150),
    ///     easing: EasingFactory::exit(), // 使用退出缓动函数
    ///     // 其他字段...
    /// };
    /// ```
    pub fn exit() -> EasingFunction {
        EasingFunction::CubicBezier(0.4, 0.0, 1.0, 1.0)
    }

    /// 创建弹性缓动函数
    ///
    /// 创建带有弹性效果的缓动函数，适用于需要吸引注意力的元素。
    ///
    /// # 返回值
    ///
    /// 返回弹性缓动函数。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{EasingFactory, AnimationConfig};
    /// use std::time::Duration;
    ///
    /// // 创建带有弹性效果的按钮动画
    /// let bounce_button = AnimationConfig {
    ///     name: "bounce-button".to_string(),
    ///     duration: Duration::from_millis(400),
    ///     easing: EasingFactory::bounce(), // 使用弹性缓动函数
    ///     // 其他字段...
    /// };
    /// ```
    pub fn bounce() -> EasingFunction {
        EasingFunction::CubicBezier(0.68, -0.55, 0.265, 1.55)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_function() {
        let css_easing = EasingFunction::Css("ease-in-out".to_string());
        assert_eq!(css_easing.to_css(), "ease-in-out");
        assert_eq!(css_easing.suggested_duration_ms(), 200);

        let cubic_easing = EasingFunction::CubicBezier(0.34, 0.69, 0.1, 1.0);
        assert_eq!(cubic_easing.to_css(), "cubic-bezier(0.34, 0.69, 0.1, 1)");
        assert_eq!(cubic_easing.suggested_duration_ms(), 200);
    }

    #[test]
    fn test_easing_factory() {
        let standard = EasingFactory::standard();
        assert_eq!(standard.to_css(), "cubic-bezier(0.34, 0.69, 0.1, 1)");

        let emphasized = EasingFactory::emphasized();
        assert_eq!(emphasized.to_css(), "cubic-bezier(0.05, 0.7, 0.1, 1)");
    }
}
