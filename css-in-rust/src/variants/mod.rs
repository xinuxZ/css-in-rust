//! 变体系统模块
//!
//! 提供完整的变体管理功能，包括响应式断点、状态变体、条件样式和优先级管理。
//! 支持编译时和运行时的变体解析与应用。

// 子模块声明
pub mod conditional_styles;
pub mod minimal;
pub mod priority_manager;
pub mod responsive;
pub mod state_variants;
pub mod variant_resolver;
pub mod variant_types;

// 重新导出主要类型
pub use conditional_styles::*;
pub use minimal::*;
pub use priority_manager::*;
pub use responsive::*;
pub use state_variants::*;
pub use variant_resolver::*;
pub use variant_types::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 变体配置
///
/// 定义组件的所有可用变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariantConfig {
    /// 尺寸变体
    pub size: HashMap<String, VariantStyle>,
    /// 颜色变体
    pub color: HashMap<String, VariantStyle>,
    /// 状态变体
    pub state: HashMap<String, VariantStyle>,
    /// 响应式变体
    pub responsive: HashMap<String, VariantStyle>,
    /// 默认变体
    pub defaults: HashMap<String, String>,
}

/// 变体样式定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariantStyle {
    /// CSS 属性
    pub properties: HashMap<String, String>,
    /// 伪类样式
    pub pseudo_classes: HashMap<String, HashMap<String, String>>,
    /// 优先级
    pub priority: u32,
}

/// 变体应用结果
#[derive(Debug, Clone)]
pub struct VariantResult {
    /// 生成的 CSS 类名
    pub class_name: String,
    /// 生成的 CSS 规则
    pub css_rules: String,
    /// 应用的变体列表
    pub applied_variants: Vec<String>,
    /// 优先级分数
    pub priority_score: u32,
}

/// 变体管理器
///
/// 负责变体的注册、解析和应用
#[derive(Debug, Clone)]
pub struct VariantManager {
    /// 注册的变体配置
    configs: HashMap<String, VariantConfig>,
    /// 简单变体管理器
    simple_manager: SimpleVariantManager,
}

impl VariantManager {
    /// 创建新的变体管理器
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            simple_manager: SimpleVariantManager::new(),
        }
    }

    /// 注册变体配置
    ///
    /// # 参数
    /// * `component_name` - 组件名称
    /// * `config` - 变体配置
    pub fn register_variant_config(&mut self, component_name: &str, config: VariantConfig) {
        self.configs.insert(component_name.to_string(), config);
    }

    /// 应用变体
    ///
    /// # 参数
    /// * `component_name` - 组件名称
    /// * `variants` - 要应用的变体
    /// * `props` - 组件属性
    pub fn apply_variants(
        &self,
        component_name: &str,
        variants: &HashMap<String, String>,
        _props: &HashMap<String, serde_json::Value>,
    ) -> Result<VariantResult, String> {
        let config = self
            .configs
            .get(component_name)
            .ok_or_else(|| format!("Component '{}' not found", component_name))?;

        let mut applied_styles = HashMap::new();
        let mut applied_variants = Vec::new();
        let mut priority_score = 0;

        // 应用默认变体
        for (variant_type, default_value) in &config.defaults {
            if let Some(variant_style) = self.get_variant_style(config, variant_type, default_value)
            {
                self.merge_styles(&mut applied_styles, &variant_style.properties);
                applied_variants.push(format!("{}:{}", variant_type, default_value));
                priority_score += variant_style.priority;
            }
        }

        // 应用指定变体
        for (variant_type, variant_value) in variants {
            if let Some(variant_style) = self.get_variant_style(config, variant_type, variant_value)
            {
                self.merge_styles(&mut applied_styles, &variant_style.properties);
                applied_variants.push(format!("{}:{}", variant_type, variant_value));
                priority_score += variant_style.priority;
            }
        }

        // 使用简单变体管理器处理
        let simple_styles = self
            .simple_manager
            .apply_simple_variants(config, variants)?;
        self.merge_styles(&mut applied_styles, &simple_styles);

        // 生成 CSS
        let class_name = self.generate_class_name(component_name, &applied_variants);
        let css_rules = self.generate_css_rules(&class_name, &applied_styles);

        Ok(VariantResult {
            class_name,
            css_rules,
            applied_variants,
            priority_score,
        })
    }

    /// 获取变体样式
    fn get_variant_style<'a>(
        &self,
        config: &'a VariantConfig,
        variant_type: &str,
        variant_value: &str,
    ) -> Option<&'a VariantStyle> {
        match variant_type {
            "size" => config.size.get(variant_value),
            "color" => config.color.get(variant_value),
            "state" => config.state.get(variant_value),
            _ => None,
        }
    }

    /// 合并样式
    fn merge_styles(&self, target: &mut HashMap<String, String>, source: &HashMap<String, String>) {
        for (key, value) in source {
            target.insert(key.clone(), value.clone());
        }
    }

    /// 生成类名
    fn generate_class_name(&self, component_name: &str, variants: &[String]) -> String {
        let mut class_parts = vec![component_name.to_string()];
        for variant in variants {
            class_parts.push(variant.replace(':', "-"));
        }
        format!("css-{}", class_parts.join("-"))
    }

    /// 生成 CSS 规则
    fn generate_css_rules(&self, class_name: &str, styles: &HashMap<String, String>) -> String {
        let mut css = format!(".{} {{\n", class_name);
        for (property, value) in styles {
            css.push_str(&format!("  {}: {};\n", property, value));
        }
        css.push_str("}\n");
        css
    }
}

impl Default for VariantManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局变体管理器实例
static mut GLOBAL_VARIANT_MANAGER: Option<VariantManager> = None;
static VARIANT_MANAGER_INIT: std::sync::Once = std::sync::Once::new();

/// 获取全局变体管理器
pub fn global_variant_manager() -> &'static mut VariantManager {
    unsafe {
        VARIANT_MANAGER_INIT.call_once(|| {
            GLOBAL_VARIANT_MANAGER = Some(VariantManager::new());
        });
        GLOBAL_VARIANT_MANAGER.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_manager_creation() {
        let manager = VariantManager::new();
        assert!(manager.configs.is_empty());
    }

    #[test]
    fn test_variant_config_registration() {
        let mut manager = VariantManager::new();
        let config = VariantConfig {
            size: HashMap::new(),
            color: HashMap::new(),
            state: HashMap::new(),
            responsive: HashMap::new(),
            defaults: HashMap::new(),
        };

        manager.register_variant_config("button", config);
        assert!(manager.configs.contains_key("button"));
    }
}
