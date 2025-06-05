use crate::theme::core::token::{DimensionUnit, DimensionValue};
use crate::theme::systems::semantic_system::SemanticSpacing;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 间距系统
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
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取间距值
    pub fn get_spacing(&self, key: &str) -> Option<&DimensionValue> {
        self.scale.get(key)
    }

    /// 设置间距值
    pub fn set_spacing(&mut self, key: impl Into<String>, value: DimensionValue) {
        self.scale.insert(key.into(), value);
    }

    /// 获取语义间距
    pub fn get_semantic_spacing(&self) -> &SemanticSpacing {
        &self.semantic
    }

    /// 设置语义间距
    pub fn set_semantic_spacing(&mut self, semantic: SemanticSpacing) {
        self.semantic = semantic;
    }
}
