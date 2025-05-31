//! # 缓动函数模块
//!
//! 提供各种动画缓动函数，包括 Ant Design 标准缓动和自定义贝塞尔曲线。

use serde::{Deserialize, Serialize};

/// 缓动函数枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EasingFunction {
    /// 线性
    Linear,
    /// 缓入
    EaseIn,
    /// 缓出
    EaseOut,
    /// 缓入缓出
    EaseInOut,
    /// 自定义贝塞尔曲线
    CubicBezier(f32, f32, f32, f32),
    /// Ant Design 标准缓动
    AntDesign(AntDesignEasing),
}

/// Ant Design 标准缓动函数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AntDesignEasing {
    /// 标准缓动 - 用于大多数动画
    Standard,
    /// 强调缓动 - 用于重要元素
    Emphasized,
    /// 减速缓动 - 用于进入动画
    Decelerated,
    /// 加速缓动 - 用于退出动画
    Accelerated,
    /// 弹性缓动 - 用于反馈动画
    Bounce,
    /// 回弹缓动 - 用于交互反馈
    Spring,
}

impl EasingFunction {
    /// 转换为 CSS 值
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::Linear => "linear".to_string(),
            EasingFunction::EaseIn => "ease-in".to_string(),
            EasingFunction::EaseOut => "ease-out".to_string(),
            EasingFunction::EaseInOut => "ease-in-out".to_string(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            EasingFunction::AntDesign(easing) => easing.to_css(),
        }
    }

    /// 获取持续时间建议（毫秒）
    pub fn suggested_duration_ms(&self) -> u64 {
        match self {
            EasingFunction::Linear => 200,
            EasingFunction::EaseIn => 250,
            EasingFunction::EaseOut => 200,
            EasingFunction::EaseInOut => 300,
            EasingFunction::CubicBezier(_, _, _, _) => 250,
            EasingFunction::AntDesign(easing) => easing.suggested_duration_ms(),
        }
    }
}

impl AntDesignEasing {
    /// 转换为 CSS 贝塞尔曲线
    pub fn to_css(&self) -> String {
        match self {
            AntDesignEasing::Standard => "cubic-bezier(0.34, 0.69, 0.1, 1)".to_string(),
            AntDesignEasing::Emphasized => "cubic-bezier(0.05, 0.7, 0.1, 1)".to_string(),
            AntDesignEasing::Decelerated => "cubic-bezier(0.0, 0.0, 0.2, 1)".to_string(),
            AntDesignEasing::Accelerated => "cubic-bezier(0.4, 0.0, 1, 1)".to_string(),
            AntDesignEasing::Bounce => "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            AntDesignEasing::Spring => "cubic-bezier(0.175, 0.885, 0.32, 1.275)".to_string(),
        }
    }

    /// 获取建议持续时间（毫秒）
    pub fn suggested_duration_ms(&self) -> u64 {
        match self {
            AntDesignEasing::Standard => 200,
            AntDesignEasing::Emphasized => 300,
            AntDesignEasing::Decelerated => 150,
            AntDesignEasing::Accelerated => 100,
            AntDesignEasing::Bounce => 400,
            AntDesignEasing::Spring => 350,
        }
    }

    /// 获取适用场景描述
    pub fn description(&self) -> &'static str {
        match self {
            AntDesignEasing::Standard => "标准缓动，适用于大多数动画场景",
            AntDesignEasing::Emphasized => "强调缓动，适用于重要元素的动画",
            AntDesignEasing::Decelerated => "减速缓动，适用于元素进入动画",
            AntDesignEasing::Accelerated => "加速缓动，适用于元素退出动画",
            AntDesignEasing::Bounce => "弹性缓动，适用于反馈和确认动画",
            AntDesignEasing::Spring => "回弹缓动，适用于交互反馈动画",
        }
    }
}

/// 缓动函数工厂
pub struct EasingFactory;

impl EasingFactory {
    /// 创建标准缓动
    pub fn standard() -> EasingFunction {
        EasingFunction::AntDesign(AntDesignEasing::Standard)
    }

    /// 创建强调缓动
    pub fn emphasized() -> EasingFunction {
        EasingFunction::AntDesign(AntDesignEasing::Emphasized)
    }

    /// 创建进入动画缓动
    pub fn enter() -> EasingFunction {
        EasingFunction::AntDesign(AntDesignEasing::Decelerated)
    }

    /// 创建退出动画缓动
    pub fn exit() -> EasingFunction {
        EasingFunction::AntDesign(AntDesignEasing::Accelerated)
    }

    /// 创建弹性缓动
    pub fn bounce() -> EasingFunction {
        EasingFunction::AntDesign(AntDesignEasing::Bounce)
    }

    /// 创建自定义贝塞尔曲线
    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> EasingFunction {
        EasingFunction::CubicBezier(x1, y1, x2, y2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_css_output() {
        assert_eq!(EasingFunction::Linear.to_css(), "linear");
        assert_eq!(EasingFunction::EaseIn.to_css(), "ease-in");
        assert_eq!(
            EasingFunction::CubicBezier(0.1, 0.2, 0.3, 0.4).to_css(),
            "cubic-bezier(0.1, 0.2, 0.3, 0.4)"
        );
    }

    #[test]
    fn test_ant_design_easing() {
        let standard = AntDesignEasing::Standard;
        assert_eq!(standard.to_css(), "cubic-bezier(0.34, 0.69, 0.1, 1)");
        assert_eq!(standard.suggested_duration_ms(), 200);
    }

    #[test]
    fn test_easing_factory() {
        let standard = EasingFactory::standard();
        assert_eq!(standard.to_css(), "cubic-bezier(0.34, 0.69, 0.1, 1)");
    }
}
