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
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::Css(css) => css.clone(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
        }
    }

    /// 获取建议的动画持续时间（毫秒）
    pub fn suggested_duration_ms(&self) -> u32 {
        200 // 默认持续时间
    }
}

/// 缓动函数工厂
pub struct EasingFactory;

impl EasingFactory {
    /// 创建标准缓动函数
    pub fn standard() -> EasingFunction {
        EasingFunction::CubicBezier(0.34, 0.69, 0.1, 1.0)
    }

    /// 创建强调缓动函数
    pub fn emphasized() -> EasingFunction {
        EasingFunction::CubicBezier(0.05, 0.7, 0.1, 1.0)
    }

    /// 创建进入缓动函数
    pub fn enter() -> EasingFunction {
        EasingFunction::CubicBezier(0.0, 0.0, 0.2, 1.0)
    }

    /// 创建退出缓动函数
    pub fn exit() -> EasingFunction {
        EasingFunction::CubicBezier(0.4, 0.0, 1.0, 1.0)
    }

    /// 创建弹性缓动函数
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
