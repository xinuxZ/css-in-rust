//! # 关键帧模块
//!
//! 提供动画关键帧的定义和管理功能，支持复杂的动画序列。

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 关键帧定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keyframes {
    /// 关键帧名称
    pub name: String,
    /// 关键帧步骤（百分比 -> CSS 属性）
    pub steps: BTreeMap<u8, KeyframeStep>,
}

/// 关键帧步骤
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyframeStep {
    /// CSS 属性和值
    pub properties: BTreeMap<String, String>,
    /// 可选的缓动函数（仅适用于此步骤）
    pub easing: Option<String>,
}

impl Keyframes {
    /// 创建新的关键帧
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: BTreeMap::new(),
        }
    }

    /// 添加关键帧步骤
    pub fn add_step(&mut self, percentage: u8, step: KeyframeStep) -> &mut Self {
        if percentage <= 100 {
            self.steps.insert(percentage, step);
        }
        self
    }

    /// 添加简单的关键帧步骤
    pub fn add_simple_step(
        &mut self,
        percentage: u8,
        properties: BTreeMap<String, String>,
    ) -> &mut Self {
        self.add_step(
            percentage,
            KeyframeStep {
                properties,
                easing: None,
            },
        )
    }

    /// 生成 CSS @keyframes 规则
    pub fn to_css(&self) -> String {
        let mut css = format!("@keyframes {} {{\n", self.name);

        for (percentage, step) in &self.steps {
            css.push_str(&format!("  {}% {{\n", percentage));

            for (property, value) in &step.properties {
                css.push_str(&format!("    {}: {};\n", property, value));
            }

            if let Some(easing) = &step.easing {
                css.push_str(&format!("    animation-timing-function: {};\n", easing));
            }

            css.push_str("  }\n");
        }

        css.push_str("}\n");
        css
    }

    /// 验证关键帧是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("关键帧名称不能为空".to_string());
        }

        if self.steps.is_empty() {
            return Err("关键帧必须包含至少一个步骤".to_string());
        }

        // 检查是否有 0% 或 100% 的关键帧
        if !self.steps.contains_key(&0) && !self.steps.contains_key(&100) {
            return Err("关键帧应该包含 0% 或 100% 的步骤".to_string());
        }

        Ok(())
    }
}

impl KeyframeStep {
    /// 创建新的关键帧步骤
    pub fn new() -> Self {
        Self {
            properties: BTreeMap::new(),
            easing: None,
        }
    }

    /// 添加 CSS 属性
    pub fn add_property(
        &mut self,
        property: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        self.properties.insert(property.into(), value.into());
        self
    }

    /// 设置缓动函数
    pub fn with_easing(&mut self, easing: impl Into<String>) -> &mut Self {
        self.easing = Some(easing.into());
        self
    }

    /// 批量添加属性
    pub fn add_properties(&mut self, properties: BTreeMap<String, String>) -> &mut Self {
        self.properties.extend(properties);
        self
    }
}

/// 关键帧构建器
pub struct KeyframesBuilder {
    keyframes: Keyframes,
}

impl KeyframesBuilder {
    /// 创建新的构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            keyframes: Keyframes::new(name),
        }
    }

    /// 添加步骤
    pub fn step(mut self, percentage: u8) -> KeyframeStepBuilder {
        KeyframeStepBuilder {
            builder: self,
            percentage,
            step: KeyframeStep::new(),
        }
    }

    /// 构建关键帧
    pub fn build(self) -> Keyframes {
        self.keyframes
    }
}

/// 关键帧步骤构建器
pub struct KeyframeStepBuilder {
    builder: KeyframesBuilder,
    percentage: u8,
    step: KeyframeStep,
}

impl KeyframeStepBuilder {
    /// 添加属性
    pub fn property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.step.add_property(name, value);
        self
    }

    /// 设置缓动
    pub fn easing(mut self, easing: impl Into<String>) -> Self {
        self.step.with_easing(easing);
        self
    }

    /// 完成步骤并返回构建器
    pub fn done(mut self) -> KeyframesBuilder {
        self.builder.keyframes.add_step(self.percentage, self.step);
        self.builder
    }

    /// 添加下一个步骤
    pub fn step(self, percentage: u8) -> KeyframeStepBuilder {
        self.done().step(percentage)
    }

    /// 构建关键帧
    pub fn build(self) -> Keyframes {
        self.done().build()
    }
}

/// 预定义关键帧
pub struct PredefinedKeyframes;

impl PredefinedKeyframes {
    /// 淡入动画关键帧
    pub fn fade_in() -> Keyframes {
        KeyframesBuilder::new("fade-in")
            .step(0)
            .property("opacity", "0")
            .step(100)
            .property("opacity", "1")
            .build()
    }

    /// 淡出动画关键帧
    pub fn fade_out() -> Keyframes {
        KeyframesBuilder::new("fade-out")
            .step(0)
            .property("opacity", "1")
            .step(100)
            .property("opacity", "0")
            .build()
    }

    /// 向上滑入关键帧
    pub fn slide_up() -> Keyframes {
        KeyframesBuilder::new("slide-up")
            .step(0)
            .property("transform", "translateY(100%)")
            .property("opacity", "0")
            .step(100)
            .property("transform", "translateY(0)")
            .property("opacity", "1")
            .build()
    }

    /// 向下滑入关键帧
    pub fn slide_down() -> Keyframes {
        KeyframesBuilder::new("slide-down")
            .step(0)
            .property("transform", "translateY(-100%)")
            .property("opacity", "0")
            .step(100)
            .property("transform", "translateY(0)")
            .property("opacity", "1")
            .build()
    }

    /// 缩放进入关键帧
    pub fn zoom_in() -> Keyframes {
        KeyframesBuilder::new("zoom-in")
            .step(0)
            .property("transform", "scale(0)")
            .property("opacity", "0")
            .step(100)
            .property("transform", "scale(1)")
            .property("opacity", "1")
            .build()
    }

    /// 弹性进入关键帧
    pub fn bounce_in() -> Keyframes {
        KeyframesBuilder::new("bounce-in")
            .step(0)
            .property("transform", "scale(0.3)")
            .property("opacity", "0")
            .step(50)
            .property("transform", "scale(1.05)")
            .property("opacity", "1")
            .step(70)
            .property("transform", "scale(0.9)")
            .step(100)
            .property("transform", "scale(1)")
            .build()
    }

    /// 摇摆动画关键帧
    pub fn shake() -> Keyframes {
        KeyframesBuilder::new("shake")
            .step(0)
            .property("transform", "translateX(0)")
            .step(10)
            .property("transform", "translateX(-10px)")
            .step(20)
            .property("transform", "translateX(10px)")
            .step(30)
            .property("transform", "translateX(-10px)")
            .step(40)
            .property("transform", "translateX(10px)")
            .step(50)
            .property("transform", "translateX(-10px)")
            .step(60)
            .property("transform", "translateX(10px)")
            .step(70)
            .property("transform", "translateX(-10px)")
            .step(80)
            .property("transform", "translateX(10px)")
            .step(90)
            .property("transform", "translateX(-10px)")
            .step(100)
            .property("transform", "translateX(0)")
            .build()
    }

    /// 旋转动画关键帧
    pub fn rotate() -> Keyframes {
        KeyframesBuilder::new("rotate")
            .step(0)
            .property("transform", "rotate(0deg)")
            .step(100)
            .property("transform", "rotate(360deg)")
            .build()
    }

    /// 脉冲动画关键帧
    pub fn pulse() -> Keyframes {
        KeyframesBuilder::new("pulse")
            .step(0)
            .property("transform", "scale(1)")
            .step(50)
            .property("transform", "scale(1.1)")
            .step(100)
            .property("transform", "scale(1)")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframes_creation() {
        let keyframes = KeyframesBuilder::new("test")
            .step(0)
            .property("opacity", "0")
            .step(100)
            .property("opacity", "1")
            .build();

        assert_eq!(keyframes.name, "test");
        assert_eq!(keyframes.steps.len(), 2);
        assert!(keyframes.validate().is_ok());
    }

    #[test]
    fn test_css_generation() {
        let keyframes = PredefinedKeyframes::fade_in();
        let css = keyframes.to_css();
        assert!(css.contains("@keyframes fade-in"));
        assert!(css.contains("opacity: 0"));
        assert!(css.contains("opacity: 1"));
    }

    #[test]
    fn test_validation() {
        let mut keyframes = Keyframes::new("test");
        assert!(keyframes.validate().is_err());

        keyframes.add_simple_step(50, BTreeMap::new());
        assert!(keyframes.validate().is_err());

        keyframes.add_simple_step(0, BTreeMap::new());
        assert!(keyframes.validate().is_ok());
    }
}
