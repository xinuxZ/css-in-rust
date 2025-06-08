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
    ///
    /// 初始化一个新的关键帧定义，使用指定的名称。
    ///
    /// # 参数
    ///
    /// * `name` - 关键帧的名称，将用于CSS `@keyframes` 规则
    ///
    /// # 返回值
    ///
    /// 返回一个新的`Keyframes`实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::Keyframes;
    ///
    /// // 创建新的关键帧定义
    /// let mut keyframes = Keyframes::new("fade-in");
    ///
    /// // 现在可以添加关键帧步骤
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: BTreeMap::new(),
        }
    }

    /// 添加关键帧步骤
    ///
    /// 向关键帧定义中添加一个步骤，指定百分比位置和CSS属性。
    ///
    /// # 参数
    ///
    /// * `percentage` - 关键帧步骤的百分比位置（0-100）
    /// * `step` - 关键帧步骤定义，包含CSS属性和可选的缓动函数
    ///
    /// # 返回值
    ///
    /// 返回对`self`的可变引用，支持方法链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{Keyframes, KeyframeStep};
    /// use std::collections::BTreeMap;
    ///
    /// // 创建关键帧
    /// let mut keyframes = Keyframes::new("fade-in");
    ///
    /// // 创建步骤
    /// let mut start_step = KeyframeStep::new();
    /// start_step.add_property("opacity", "0");
    /// start_step.add_property("transform", "translateY(20px)");
    ///
    /// let mut end_step = KeyframeStep::new();
    /// end_step.add_property("opacity", "1");
    /// end_step.add_property("transform", "translateY(0)");
    /// end_step.with_easing("ease-out");
    ///
    /// // 添加步骤到关键帧
    /// keyframes.add_step(0, start_step)
    ///         .add_step(100, end_step);
    /// ```
    pub fn add_step(&mut self, percentage: u8, step: KeyframeStep) -> &mut Self {
        if percentage <= 100 {
            self.steps.insert(percentage, step);
        }
        self
    }

    /// 添加简单的关键帧步骤
    ///
    /// 向关键帧定义中添加一个简单的步骤，只指定CSS属性而不包含缓动函数。
    ///
    /// # 参数
    ///
    /// * `percentage` - 关键帧步骤的百分比位置（0-100）
    /// * `properties` - CSS属性和值的映射
    ///
    /// # 返回值
    ///
    /// 返回对`self`的可变引用，支持方法链式调用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::Keyframes;
    /// use std::collections::BTreeMap;
    ///
    /// // 创建关键帧
    /// let mut keyframes = Keyframes::new("slide-in");
    ///
    /// // 创建起始属性
    /// let mut start_props = BTreeMap::new();
    /// start_props.insert("opacity".to_string(), "0".to_string());
    /// start_props.insert("transform".to_string(), "translateX(-100%)".to_string());
    ///
    /// // 创建结束属性
    /// let mut end_props = BTreeMap::new();
    /// end_props.insert("opacity".to_string(), "1".to_string());
    /// end_props.insert("transform".to_string(), "translateX(0)".to_string());
    ///
    /// // 添加简单步骤
    /// keyframes.add_simple_step(0, start_props)
    ///         .add_simple_step(100, end_props);
    /// ```
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
    ///
    /// 将关键帧定义转换为完整的CSS `@keyframes` 规则字符串。
    ///
    /// # 返回值
    ///
    /// 返回表示CSS `@keyframes` 规则的字符串。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{Keyframes, KeyframeStep};
    /// use std::collections::BTreeMap;
    ///
    /// // 创建并配置关键帧
    /// let mut keyframes = Keyframes::new("bounce");
    ///
    /// // 添加步骤
    /// let mut start = KeyframeStep::new();
    /// start.add_property("transform", "scale(0.3)");
    /// start.add_property("opacity", "0");
    ///
    /// let mut middle = KeyframeStep::new();
    /// middle.add_property("transform", "scale(1.05)");
    /// middle.add_property("opacity", "1");
    ///
    /// let mut end = KeyframeStep::new();
    /// end.add_property("transform", "scale(1)");
    ///
    /// keyframes.add_step(0, start)
    ///         .add_step(50, middle)
    ///         .add_step(100, end);
    ///
    /// // 生成CSS
    /// let css = keyframes.to_css();
    /// println!("生成的CSS: {}", css);
    /// // 输出类似:
    /// // @keyframes bounce {
    /// //   0% {
    /// //     transform: scale(0.3);
    /// //     opacity: 0;
    /// //   }
    /// //   50% {
    /// //     transform: scale(1.05);
    /// //     opacity: 1;
    /// //   }
    /// //   100% {
    /// //     transform: scale(1);
    /// //   }
    /// // }
    /// ```
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
    ///
    /// 检查关键帧定义是否符合基本要求，如名称不为空、至少有一个步骤、
    /// 包含0%或100%的关键步骤等。
    ///
    /// # 返回值
    ///
    /// 如果关键帧有效，则返回`Ok(())`；否则返回包含错误信息的`Err(String)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::animation::{Keyframes, KeyframeStep};
    /// use std::collections::BTreeMap;
    ///
    /// // 创建关键帧
    /// let mut keyframes = Keyframes::new("fade-in");
    ///
    /// // 添加步骤
    /// let mut start = KeyframeStep::new();
    /// start.add_property("opacity", "0");
    /// keyframes.add_step(0, start);
    ///
    /// let mut end = KeyframeStep::new();
    /// end.add_property("opacity", "1");
    /// keyframes.add_step(100, end);
    ///
    /// // 验证关键帧
    /// match keyframes.validate() {
    ///     Ok(_) => println!("关键帧有效"),
    ///     Err(e) => println!("关键帧无效: {}", e),
    /// }
    /// ```
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
    pub fn step(self, percentage: u8) -> KeyframeStepBuilder {
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
