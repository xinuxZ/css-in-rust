//! 响应式断点系统模块
//!
//! 提供完整的响应式设计支持，包括断点管理、媒体查询生成和运行时变体选择。

use super::{VariantConfig, VariantStyle};
use crate::theme::BreakpointTokens;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 响应式变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsiveVariant {
    /// 断点映射
    pub breakpoints: HashMap<String, VariantStyle>,
    /// 默认样式（无断点限制）
    pub default: Option<VariantStyle>,
    /// 是否启用移动优先
    pub mobile_first: bool,
}

/// 断点定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Breakpoint {
    /// 断点名称
    pub name: String,
    /// 最小宽度
    pub min_width: Option<u32>,
    /// 最大宽度
    pub max_width: Option<u32>,
    /// 媒体查询字符串
    pub media_query: String,
    /// 优先级
    pub priority: u32,
}

/// 响应式管理器
///
/// 负责响应式变体的管理和应用
#[derive(Debug, Clone)]
pub struct ResponsiveManager {
    /// 断点定义
    breakpoints: HashMap<String, Breakpoint>,
    /// 当前激活的断点
    active_breakpoints: Vec<String>,
    /// 是否启用移动优先
    mobile_first: bool,
}

/// 媒体查询构建器
#[derive(Debug, Clone)]
pub struct MediaQueryBuilder {
    /// 条件列表
    conditions: Vec<String>,
}

/// 响应式样式结果
#[derive(Debug, Clone)]
pub struct ResponsiveStyleResult {
    /// 基础样式
    pub base_styles: HashMap<String, String>,
    /// 媒体查询样式
    pub media_queries: HashMap<String, HashMap<String, String>>,
    /// 生成的 CSS
    pub css: String,
}

impl ResponsiveManager {
    /// 创建新的响应式管理器
    pub fn new() -> Self {
        let mut manager = Self {
            breakpoints: HashMap::new(),
            active_breakpoints: Vec::new(),
            mobile_first: true,
        };

        // 初始化默认断点
        manager.init_default_breakpoints();
        manager
    }

    /// 初始化默认断点
    fn init_default_breakpoints(&mut self) {
        let default_breakpoints = vec![
            Breakpoint {
                name: "xs".to_string(),
                min_width: None,
                max_width: Some(575),
                media_query: "(max-width: 575px)".to_string(),
                priority: 1,
            },
            Breakpoint {
                name: "sm".to_string(),
                min_width: Some(576),
                max_width: Some(767),
                media_query: "(min-width: 576px) and (max-width: 767px)".to_string(),
                priority: 2,
            },
            Breakpoint {
                name: "md".to_string(),
                min_width: Some(768),
                max_width: Some(991),
                media_query: "(min-width: 768px) and (max-width: 991px)".to_string(),
                priority: 3,
            },
            Breakpoint {
                name: "lg".to_string(),
                min_width: Some(992),
                max_width: Some(1199),
                media_query: "(min-width: 992px) and (max-width: 1199px)".to_string(),
                priority: 4,
            },
            Breakpoint {
                name: "xl".to_string(),
                min_width: Some(1200),
                max_width: Some(1599),
                media_query: "(min-width: 1200px) and (max-width: 1599px)".to_string(),
                priority: 5,
            },
            Breakpoint {
                name: "xxl".to_string(),
                min_width: Some(1600),
                max_width: None,
                media_query: "(min-width: 1600px)".to_string(),
                priority: 6,
            },
        ];

        for breakpoint in default_breakpoints {
            self.breakpoints.insert(breakpoint.name.clone(), breakpoint);
        }
    }

    /// 从断点令牌初始化
    pub fn from_breakpoint_tokens(tokens: &BreakpointTokens) -> Self {
        let mut manager = Self::new();
        manager.update_from_tokens(tokens);
        manager
    }

    /// 从断点令牌更新
    pub fn update_from_tokens(&mut self, tokens: &BreakpointTokens) {
        // 解析断点令牌并更新断点定义
        let breakpoint_values = vec![
            ("xs", &tokens.xs),
            ("sm", &tokens.sm),
            ("md", &tokens.md),
            ("lg", &tokens.lg),
            ("xl", &tokens.xl),
            ("xxl", &tokens.xxl),
        ];

        for (name, value) in breakpoint_values {
            if let Ok(width) = self.parse_breakpoint_value(value) {
                if let Some(breakpoint) = self.breakpoints.get_mut(name) {
                    breakpoint.min_width = Some(width);
                    breakpoint.media_query = format!("(min-width: {}px)", width);
                }
            }
        }
    }

    /// 解析断点值
    fn parse_breakpoint_value(&self, value: &str) -> Result<u32, String> {
        let cleaned = value.trim().replace("px", "");
        cleaned
            .parse::<u32>()
            .map_err(|_| format!("Invalid breakpoint value: {}", value))
    }

    /// 注册自定义断点
    pub fn register_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.insert(breakpoint.name.clone(), breakpoint);
    }

    /// 设置当前激活的断点
    pub fn set_active_breakpoints(&mut self, breakpoints: Vec<String>) {
        self.active_breakpoints = breakpoints;
    }

    /// 应用响应式变体
    pub fn apply_responsive_variants(
        &self,
        config: &VariantConfig,
        variants: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut responsive_styles = HashMap::new();

        // 处理响应式变体
        for (variant_key, variant_value) in variants {
            if let Some(responsive_variant) = config.responsive.get(variant_key) {
                // 应用默认样式
                if let Some(default_style) = &responsive_variant.default {
                    for (prop, value) in &default_style.properties {
                        responsive_styles.insert(prop.clone(), value.clone());
                    }
                }

                // 应用断点特定样式
                for breakpoint_name in &self.active_breakpoints {
                    if let Some(breakpoint_style) =
                        responsive_variant.breakpoints.get(breakpoint_name)
                    {
                        for (prop, value) in &breakpoint_style.properties {
                            responsive_styles
                                .insert(format!("{}@{}", prop, breakpoint_name), value.clone());
                        }
                    }
                }
            }
        }

        Ok(responsive_styles)
    }

    /// 生成响应式 CSS
    pub fn generate_responsive_css(
        &self,
        class_name: &str,
        responsive_styles: &HashMap<String, String>,
    ) -> ResponsiveStyleResult {
        let mut base_styles = HashMap::new();
        let mut media_queries = HashMap::new();
        let mut css = String::new();

        // 分离基础样式和媒体查询样式
        for (key, value) in responsive_styles {
            if let Some(at_pos) = key.find('@') {
                let (prop, breakpoint) = key.split_at(at_pos);
                let breakpoint = &breakpoint[1..]; // 移除 '@' 符号

                media_queries
                    .entry(breakpoint.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(prop.to_string(), value.clone());
            } else {
                base_styles.insert(key.clone(), value.clone());
            }
        }

        // 生成基础样式 CSS
        if !base_styles.is_empty() {
            css.push_str(&format!(".{} {{\n", class_name));
            for (prop, value) in &base_styles {
                css.push_str(&format!("  {}: {};\n", prop, value));
            }
            css.push_str("}\n\n");
        }

        // 生成媒体查询 CSS
        let mut sorted_breakpoints: Vec<_> = media_queries.keys().collect();
        sorted_breakpoints.sort_by(|a, b| {
            let priority_a = self.breakpoints.get(*a).map(|bp| bp.priority).unwrap_or(0);
            let priority_b = self.breakpoints.get(*b).map(|bp| bp.priority).unwrap_or(0);
            priority_a.cmp(&priority_b)
        });

        for breakpoint_name in sorted_breakpoints {
            if let Some(breakpoint) = self.breakpoints.get(breakpoint_name) {
                if let Some(styles) = media_queries.get(breakpoint_name) {
                    css.push_str(&format!("@media {} {{\n", breakpoint.media_query));
                    css.push_str(&format!("  .{} {{\n", class_name));
                    for (prop, value) in styles {
                        css.push_str(&format!("    {}: {};\n", prop, value));
                    }
                    css.push_str("  }\n");
                    css.push_str("}\n\n");
                }
            }
        }

        ResponsiveStyleResult {
            base_styles,
            media_queries,
            css,
        }
    }

    /// 获取断点信息
    pub fn get_breakpoint(&self, name: &str) -> Option<&Breakpoint> {
        self.breakpoints.get(name)
    }

    /// 获取所有断点
    pub fn get_all_breakpoints(&self) -> &HashMap<String, Breakpoint> {
        &self.breakpoints
    }

    /// 检查断点是否激活
    pub fn is_breakpoint_active(&self, name: &str) -> bool {
        self.active_breakpoints.contains(&name.to_string())
    }
}

impl MediaQueryBuilder {
    /// 创建新的媒体查询构建器
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// 添加最小宽度条件
    pub fn min_width(mut self, width: u32) -> Self {
        self.conditions.push(format!("min-width: {}px", width));
        self
    }

    /// 添加最大宽度条件
    pub fn max_width(mut self, width: u32) -> Self {
        self.conditions.push(format!("max-width: {}px", width));
        self
    }

    /// 添加自定义条件
    pub fn condition(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    /// 构建媒体查询字符串
    pub fn build(self) -> String {
        if self.conditions.is_empty() {
            "all".to_string()
        } else {
            format!("({})", self.conditions.join(") and ("))
        }
    }
}

impl Default for ResponsiveManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MediaQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷的响应式变体创建函数
pub fn responsive_variant() -> ResponsiveVariantBuilder {
    ResponsiveVariantBuilder::new()
}

/// 响应式变体构建器
#[derive(Debug, Clone)]
pub struct ResponsiveVariantBuilder {
    variant: ResponsiveVariant,
}

impl ResponsiveVariantBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            variant: ResponsiveVariant {
                breakpoints: HashMap::new(),
                default: None,
                mobile_first: true,
            },
        }
    }

    /// 设置默认样式
    pub fn default_style(mut self, style: VariantStyle) -> Self {
        self.variant.default = Some(style);
        self
    }

    /// 添加断点样式
    pub fn breakpoint(mut self, name: &str, style: VariantStyle) -> Self {
        self.variant.breakpoints.insert(name.to_string(), style);
        self
    }

    /// 设置移动优先
    pub fn mobile_first(mut self, mobile_first: bool) -> Self {
        self.variant.mobile_first = mobile_first;
        self
    }

    /// 构建响应式变体
    pub fn build(self) -> ResponsiveVariant {
        self.variant
    }
}

impl Default for ResponsiveVariantBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::BreakpointTokens;

    #[test]
    fn test_responsive_manager_creation() {
        let manager = ResponsiveManager::new();
        assert!(!manager.breakpoints.is_empty());
        assert!(manager.breakpoints.contains_key("md"));
    }

    #[test]
    fn test_breakpoint_tokens_integration() {
        let tokens = BreakpointTokens {
            xs: "480px".to_string(),
            sm: "576px".to_string(),
            md: "768px".to_string(),
            lg: "992px".to_string(),
            xl: "1200px".to_string(),
            xxl: "1600px".to_string(),
        };

        let manager = ResponsiveManager::from_breakpoint_tokens(&tokens);
        let md_breakpoint = manager.get_breakpoint("md").unwrap();
        assert_eq!(md_breakpoint.min_width, Some(768));
    }

    #[test]
    fn test_media_query_builder() {
        let query = MediaQueryBuilder::new()
            .min_width(768)
            .max_width(1199)
            .build();

        assert_eq!(query, "(min-width: 768px) and (max-width: 1199px)");
    }

    #[test]
    fn test_responsive_variant_builder() {
        let style = VariantStyle {
            properties: HashMap::from([("padding".to_string(), "16px".to_string())]),
            pseudo_classes: HashMap::new(),
            priority: 1,
        };

        let variant = responsive_variant()
            .default_style(style.clone())
            .breakpoint("md", style)
            .mobile_first(true)
            .build();

        assert!(variant.default.is_some());
        assert!(variant.breakpoints.contains_key("md"));
        assert!(variant.mobile_first);
    }
}
