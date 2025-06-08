//! # 间距系统模块
//!
//! 提供统一的间距管理功能，包括基础间距单位、间距比例和语义间距。
//! 间距系统是设计系统的重要组成部分，确保应用程序中的间距一致性。

use crate::theme::core::token::{DimensionUnit, DimensionValue};
use crate::theme::systems::semantic_system::SemanticSpacing;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 间距系统
///
/// 管理应用程序中使用的所有间距值，包括基础间距单位、间距比例和语义间距。
/// 间距系统使用一致的比例关系来定义不同大小的间距，确保整个应用程序的间距协调一致。
///
/// # 示例
///
/// ```
/// use css_in_rust::theme::systems::SpacingSystem;
///
/// // 创建默认间距系统
/// let mut spacing_system = SpacingSystem::new();
///
/// // 获取特定间距值
/// if let Some(spacing) = spacing_system.get_spacing("4") {
///     println!("间距4的值: {}", spacing);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingSystem {
    /// 基础间距单位
    pub base_unit: DimensionValue,
    /// 间距比例
    pub scale: BTreeMap<String, DimensionValue>,
    /// 语义间距
    pub semantic: SemanticSpacing,
}

impl Default for SpacingSystem {
    fn default() -> Self {
        let base_unit = DimensionValue::new(4.0, DimensionUnit::Px);

        let mut scale = BTreeMap::new();
        scale.insert("0".to_string(), DimensionValue::new(0.0, DimensionUnit::Px));
        scale.insert("1".to_string(), DimensionValue::new(4.0, DimensionUnit::Px));
        scale.insert("2".to_string(), DimensionValue::new(8.0, DimensionUnit::Px));
        scale.insert(
            "3".to_string(),
            DimensionValue::new(12.0, DimensionUnit::Px),
        );
        scale.insert(
            "4".to_string(),
            DimensionValue::new(16.0, DimensionUnit::Px),
        );
        scale.insert(
            "5".to_string(),
            DimensionValue::new(20.0, DimensionUnit::Px),
        );
        scale.insert(
            "6".to_string(),
            DimensionValue::new(24.0, DimensionUnit::Px),
        );
        scale.insert(
            "8".to_string(),
            DimensionValue::new(32.0, DimensionUnit::Px),
        );
        scale.insert(
            "10".to_string(),
            DimensionValue::new(40.0, DimensionUnit::Px),
        );
        scale.insert(
            "12".to_string(),
            DimensionValue::new(48.0, DimensionUnit::Px),
        );
        scale.insert(
            "16".to_string(),
            DimensionValue::new(64.0, DimensionUnit::Px),
        );
        scale.insert(
            "20".to_string(),
            DimensionValue::new(80.0, DimensionUnit::Px),
        );
        scale.insert(
            "24".to_string(),
            DimensionValue::new(96.0, DimensionUnit::Px),
        );

        Self {
            base_unit,
            scale,
            semantic: SemanticSpacing::default(),
        }
    }
}

impl SpacingSystem {
    /// 创建新的间距系统
    ///
    /// 初始化一个包含默认间距值的间距系统。默认间距基于4px的基础单位，
    /// 并包含一系列按比例递增的间距值。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `SpacingSystem` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::SpacingSystem;
    ///
    /// let spacing_system = SpacingSystem::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取间距值
    ///
    /// 根据键获取对应的间距值。
    ///
    /// # 参数
    ///
    /// * `key` - 间距键，例如 "0"、"1"、"2"、"4" 等
    ///
    /// # 返回值
    ///
    /// 如果找到间距值，则返回 `Some(&DimensionValue)`，否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::SpacingSystem;
    ///
    /// let spacing_system = SpacingSystem::new();
    ///
    /// // 获取基础间距
    /// let spacing_1 = spacing_system.get_spacing("1");
    /// assert!(spacing_1.is_some());
    ///
    /// // 获取中等间距
    /// let spacing_4 = spacing_system.get_spacing("4");
    /// assert!(spacing_4.is_some());
    ///
    /// // 获取不存在的间距
    /// let nonexistent = spacing_system.get_spacing("99");
    /// assert!(nonexistent.is_none());
    /// ```
    pub fn get_spacing(&self, key: &str) -> Option<&DimensionValue> {
        self.scale.get(key)
    }

    /// 设置间距值
    ///
    /// 设置指定键的间距值。如果键已存在，则更新值；如果键不存在，则添加新的键值对。
    ///
    /// # 参数
    ///
    /// * `key` - 间距键，例如 "0"、"1"、"2"、"4" 等
    /// * `value` - 间距值，包含数值和单位
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::SpacingSystem;
    /// use css_in_rust::theme::core::token::{DimensionValue, DimensionUnit};
    ///
    /// let mut spacing_system = SpacingSystem::new();
    ///
    /// // 设置自定义间距
    /// spacing_system.set_spacing("custom", DimensionValue::new(10.0, DimensionUnit::Px));
    ///
    /// // 更新现有间距
    /// spacing_system.set_spacing("4", DimensionValue::new(20.0, DimensionUnit::Px));
    ///
    /// // 验证更新
    /// assert_eq!(spacing_system.get_spacing("custom").unwrap().value, 10.0);
    /// assert_eq!(spacing_system.get_spacing("4").unwrap().value, 20.0);
    /// ```
    pub fn set_spacing(&mut self, key: impl Into<String>, value: DimensionValue) {
        self.scale.insert(key.into(), value);
    }

    /// 获取语义间距
    ///
    /// 返回语义间距系统的引用，用于访问基于语义的间距值，如组件间距、布局间距等。
    ///
    /// # 返回值
    ///
    /// 返回 `SemanticSpacing` 的引用。
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::SpacingSystem;
    ///
    /// let spacing_system = SpacingSystem::new();
    /// let semantic_spacing = spacing_system.get_semantic_spacing();
    ///
    /// // 使用语义间距
    /// // let component_xs = semantic_spacing.component.xs;
    /// ```
    pub fn get_semantic_spacing(&self) -> &SemanticSpacing {
        &self.semantic
    }

    /// 设置语义间距
    ///
    /// 替换当前的语义间距系统为新的语义间距系统。
    ///
    /// # 参数
    ///
    /// * `semantic` - 新的语义间距系统
    ///
    /// # 示例
    ///
    /// ```
    /// use css_in_rust::theme::systems::SpacingSystem;
    /// use css_in_rust::theme::systems::semantic_system::SemanticSpacing;
    ///
    /// let mut spacing_system = SpacingSystem::new();
    /// let semantic_spacing = SemanticSpacing::default();
    ///
    /// spacing_system.set_semantic_spacing(semantic_spacing);
    /// ```
    pub fn set_semantic_spacing(&mut self, semantic: SemanticSpacing) {
        self.semantic = semantic;
    }
}
