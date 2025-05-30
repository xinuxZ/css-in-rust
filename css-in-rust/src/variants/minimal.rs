//! 最小化变体系统实现
//!
//! 提供基础的变体功能，避免复杂的依赖关系

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 简化的变体类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimpleVariant {
    /// 尺寸变体
    Size(String),
    /// 颜色变体
    Color(String),
    /// 状态变体
    State(String),
    /// 自定义变体
    Custom(String, String),
}

/// 简化的变体样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleVariantStyle {
    /// CSS 属性
    pub properties: HashMap<String, String>,
    /// 优先级
    pub priority: u32,
}

/// 简化的变体管理器
#[derive(Debug, Clone)]
pub struct SimpleVariantManager {
    /// 注册的变体
    variants: HashMap<String, SimpleVariantStyle>,
    /// 当前活动的变体
    active_variants: Vec<String>,
}

impl SimpleVariantManager {
    /// 创建新的变体管理器
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            active_variants: Vec::new(),
        }
    }

    /// 注册变体
    pub fn register_variant(&mut self, name: String, style: SimpleVariantStyle) {
        self.variants.insert(name, style);
    }

    /// 激活变体
    pub fn activate_variant(&mut self, name: String) {
        if !self.active_variants.contains(&name) {
            self.active_variants.push(name);
        }
    }

    /// 停用变体
    pub fn deactivate_variant(&mut self, name: &str) {
        self.active_variants.retain(|v| v != name);
    }

    /// 应用简单变体
    pub fn apply_simple_variants(
        &self,
        config: &super::VariantConfig,
        variants: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut applied_styles = HashMap::new();

        // 处理传入的变体
        for (variant_type, variant_value) in variants {
            // 尝试从配置中获取对应的样式
            let style_map = match variant_type.as_str() {
                "size" => &config.size,
                "color" => &config.color,
                "state" => &config.state,
                _ => continue,
            };

            if let Some(variant_style) = style_map.get(variant_value) {
                for (prop, value) in &variant_style.properties {
                    applied_styles.insert(prop.clone(), value.clone());
                }
            }
        }

        Ok(applied_styles)
    }

    /// 获取当前样式
    pub fn get_current_styles(&self) -> HashMap<String, String> {
        let mut final_styles = HashMap::new();

        // 按优先级排序
        let mut sorted_variants: Vec<_> = self
            .active_variants
            .iter()
            .filter_map(|name| self.variants.get(name).map(|style| (name, style)))
            .collect();

        sorted_variants.sort_by_key(|(_, style)| style.priority);

        // 合并样式
        for (_, style) in sorted_variants {
            for (prop, value) in &style.properties {
                final_styles.insert(prop.clone(), value.clone());
            }
        }

        final_styles
    }

    /// 生成 CSS
    pub fn generate_css(&self, class_name: &str) -> String {
        let styles = self.get_current_styles();

        if styles.is_empty() {
            return String::new();
        }

        let mut css = format!(".{} {{\n", class_name);

        for (prop, value) in &styles {
            css.push_str(&format!("  {}: {};\n", prop, value));
        }

        css.push_str("}\n");
        css
    }

    /// 清除所有变体
    pub fn clear(&mut self) {
        self.active_variants.clear();
    }

    /// 获取已注册的变体列表
    pub fn get_registered_variants(&self) -> Vec<String> {
        self.variants.keys().cloned().collect()
    }

    /// 获取活动变体列表
    pub fn get_active_variants(&self) -> &[String] {
        &self.active_variants
    }
}

impl Default for SimpleVariantManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 创建预定义的变体样式
pub fn create_size_variants() -> HashMap<String, SimpleVariantStyle> {
    let mut variants = HashMap::new();

    variants.insert(
        "xs".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("padding".to_string(), "4px 8px".to_string()),
                ("font-size".to_string(), "12px".to_string()),
            ]),
            priority: 10,
        },
    );

    variants.insert(
        "sm".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("padding".to_string(), "6px 12px".to_string()),
                ("font-size".to_string(), "14px".to_string()),
            ]),
            priority: 10,
        },
    );

    variants.insert(
        "md".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("padding".to_string(), "8px 16px".to_string()),
                ("font-size".to_string(), "16px".to_string()),
            ]),
            priority: 10,
        },
    );

    variants.insert(
        "lg".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("padding".to_string(), "12px 24px".to_string()),
                ("font-size".to_string(), "18px".to_string()),
            ]),
            priority: 10,
        },
    );

    variants.insert(
        "xl".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("padding".to_string(), "16px 32px".to_string()),
                ("font-size".to_string(), "20px".to_string()),
            ]),
            priority: 10,
        },
    );

    variants
}

/// 创建预定义的颜色变体
pub fn create_color_variants() -> HashMap<String, SimpleVariantStyle> {
    let mut variants = HashMap::new();

    variants.insert(
        "primary".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "#007bff".to_string()),
                ("color".to_string(), "white".to_string()),
                ("border-color".to_string(), "#007bff".to_string()),
            ]),
            priority: 20,
        },
    );

    variants.insert(
        "secondary".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "#6c757d".to_string()),
                ("color".to_string(), "white".to_string()),
                ("border-color".to_string(), "#6c757d".to_string()),
            ]),
            priority: 20,
        },
    );

    variants.insert(
        "success".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "#28a745".to_string()),
                ("color".to_string(), "white".to_string()),
                ("border-color".to_string(), "#28a745".to_string()),
            ]),
            priority: 20,
        },
    );

    variants.insert(
        "warning".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "#ffc107".to_string()),
                ("color".to_string(), "#212529".to_string()),
                ("border-color".to_string(), "#ffc107".to_string()),
            ]),
            priority: 20,
        },
    );

    variants.insert(
        "danger".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "#dc3545".to_string()),
                ("color".to_string(), "white".to_string()),
                ("border-color".to_string(), "#dc3545".to_string()),
            ]),
            priority: 20,
        },
    );

    variants
}

/// 创建预定义的状态变体
pub fn create_state_variants() -> HashMap<String, SimpleVariantStyle> {
    let mut variants = HashMap::new();

    variants.insert(
        "hover".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("transform".to_string(), "scale(1.05)".to_string()),
                (
                    "box-shadow".to_string(),
                    "0 4px 8px rgba(0,0,0,0.1)".to_string(),
                ),
            ]),
            priority: 30,
        },
    );

    variants.insert(
        "focus".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("outline".to_string(), "2px solid #007bff".to_string()),
                ("outline-offset".to_string(), "2px".to_string()),
            ]),
            priority: 30,
        },
    );

    variants.insert(
        "active".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([("transform".to_string(), "scale(0.95)".to_string())]),
            priority: 30,
        },
    );

    variants.insert(
        "disabled".to_string(),
        SimpleVariantStyle {
            properties: HashMap::from([
                ("opacity".to_string(), "0.5".to_string()),
                ("cursor".to_string(), "not-allowed".to_string()),
            ]),
            priority: 40,
        },
    );

    variants
}

/// 创建完整的变体管理器
pub fn create_full_variant_manager() -> SimpleVariantManager {
    let mut manager = SimpleVariantManager::new();

    // 注册尺寸变体
    for (name, style) in create_size_variants() {
        manager.register_variant(format!("size-{}", name), style);
    }

    // 注册颜色变体
    for (name, style) in create_color_variants() {
        manager.register_variant(format!("color-{}", name), style);
    }

    // 注册状态变体
    for (name, style) in create_state_variants() {
        manager.register_variant(format!("state-{}", name), style);
    }

    manager
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_variant_manager() {
        let mut manager = SimpleVariantManager::new();

        let style = SimpleVariantStyle {
            properties: HashMap::from([("color".to_string(), "red".to_string())]),
            priority: 10,
        };

        manager.register_variant("test".to_string(), style);
        manager.activate_variant("test".to_string());

        let styles = manager.get_current_styles();
        assert_eq!(styles.get("color"), Some(&"red".to_string()));
    }

    #[test]
    fn test_css_generation() {
        let mut manager = SimpleVariantManager::new();

        let style = SimpleVariantStyle {
            properties: HashMap::from([
                ("background-color".to_string(), "blue".to_string()),
                ("color".to_string(), "white".to_string()),
            ]),
            priority: 10,
        };

        manager.register_variant("button".to_string(), style);
        manager.activate_variant("button".to_string());

        let css = manager.generate_css("my-button");
        assert!(css.contains("background-color: blue"));
        assert!(css.contains("color: white"));
    }

    #[test]
    fn test_priority_ordering() {
        let mut manager = SimpleVariantManager::new();

        let low_priority = SimpleVariantStyle {
            properties: HashMap::from([("color".to_string(), "red".to_string())]),
            priority: 10,
        };

        let high_priority = SimpleVariantStyle {
            properties: HashMap::from([("color".to_string(), "blue".to_string())]),
            priority: 20,
        };

        manager.register_variant("low".to_string(), low_priority);
        manager.register_variant("high".to_string(), high_priority);

        manager.activate_variant("low".to_string());
        manager.activate_variant("high".to_string());

        let styles = manager.get_current_styles();
        assert_eq!(styles.get("color"), Some(&"blue".to_string()));
    }

    #[test]
    fn test_predefined_variants() {
        let size_variants = create_size_variants();
        assert!(size_variants.contains_key("md"));

        let color_variants = create_color_variants();
        assert!(color_variants.contains_key("primary"));

        let state_variants = create_state_variants();
        assert!(state_variants.contains_key("hover"));
    }

    #[test]
    fn test_full_variant_manager() {
        let mut manager = create_full_variant_manager();

        manager.activate_variant("size-md".to_string());
        manager.activate_variant("color-primary".to_string());
        manager.activate_variant("state-hover".to_string());

        let styles = manager.get_current_styles();
        assert!(!styles.is_empty());

        let css = manager.generate_css("button");
        assert!(css.contains("background-color: #007bff"));
    }
}
