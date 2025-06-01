//! 样式优先级管理模块
//!
//! 提供完整的样式优先级计算、冲突解决和覆盖机制。

use super::VariantStyle;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// 优先级类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PriorityType {
    /// 基础样式（最低优先级）
    Base = 0,
    /// 主题样式
    Theme = 100,
    /// 变体样式
    Variant = 200,
    /// 状态样式
    State = 300,
    /// 响应式样式
    Responsive = 400,
    /// 条件样式
    Conditional = 500,
    /// 用户自定义样式
    Custom = 600,
    /// 内联样式
    Inline = 700,
    /// 重要样式（最高优先级）
    Important = 1000,
}

/// 样式来源
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StyleSource {
    /// 基础样式
    Base,
    /// 主题
    Theme(String),
    /// 变体
    Variant(String),
    /// 状态
    State(String),
    /// 响应式
    Responsive(String),
    /// 条件样式
    Conditional(String),
    /// 用户自定义
    Custom(String),
    /// 内联样式
    Inline,
}

/// 样式规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleRule {
    /// 样式属性
    pub property: String,
    /// 样式值
    pub value: String,
    /// 优先级类型
    pub priority_type: PriorityType,
    /// 具体优先级值
    pub priority_value: u32,
    /// 样式来源
    pub source: StyleSource,
    /// 是否重要
    pub important: bool,
    /// 选择器特异性
    pub specificity: Specificity,
    /// 创建时间戳
    pub timestamp: u64,
}

/// CSS 选择器特异性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    /// 内联样式数量
    pub inline: u32,
    /// ID 选择器数量
    pub ids: u32,
    /// 类选择器、属性选择器、伪类数量
    pub classes: u32,
    /// 元素选择器、伪元素数量
    pub elements: u32,
}

/// 优先级冲突
#[derive(Debug, Clone)]
pub struct PriorityConflict {
    /// 冲突的属性
    pub property: String,
    /// 冲突的规则
    pub conflicting_rules: Vec<StyleRule>,
    /// 解决策略
    pub resolution_strategy: ConflictResolutionStrategy,
}

/// 冲突解决策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolutionStrategy {
    /// 最高优先级获胜
    HighestPriority,
    /// 最后定义获胜
    LastDefined,
    /// 特异性获胜
    Specificity,
    /// 重要性获胜
    Importance,
    /// 自定义策略
    Custom(String),
}

/// 样式优先级管理器
///
/// 负责样式优先级的计算、冲突检测和解决
#[derive(Debug, Clone)]
pub struct PriorityManager {
    /// 样式规则映射（按属性分组）
    rules_by_property: HashMap<String, Vec<StyleRule>>,
    /// 优先级配置
    priority_config: PriorityConfig,
    /// 冲突解决策略
    default_resolution_strategy: ConflictResolutionStrategy,
    /// 是否启用严格模式
    strict_mode: bool,
}

/// 优先级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityConfig {
    /// 优先级权重映射
    pub priority_weights: HashMap<PriorityType, u32>,
    /// 是否启用特异性计算
    pub enable_specificity: bool,
    /// 是否启用时间戳排序
    pub enable_timestamp_ordering: bool,
    /// 重要样式权重倍数
    pub important_multiplier: u32,
}

/// 样式解析结果
#[derive(Debug, Clone)]
pub struct StyleResolutionResult {
    /// 最终样式映射
    pub final_styles: HashMap<String, String>,
    /// 应用的规则
    pub applied_rules: Vec<StyleRule>,
    /// 检测到的冲突
    pub conflicts: Vec<PriorityConflict>,
    /// 解析统计
    pub resolution_stats: ResolutionStats,
}

/// 解析统计
#[derive(Debug, Clone)]
pub struct ResolutionStats {
    /// 总规则数
    pub total_rules: usize,
    /// 冲突数
    pub conflicts_count: usize,
    /// 解析时间（微秒）
    pub resolution_time_us: u64,
    /// 按来源分组的规则数
    pub rules_by_source: HashMap<StyleSource, usize>,
}

impl Specificity {
    /// 创建新的特异性
    pub fn new(inline: u32, ids: u32, classes: u32, elements: u32) -> Self {
        Self {
            inline,
            ids,
            classes,
            elements,
        }
    }

    /// 计算总特异性值
    pub fn total_value(&self) -> u32 {
        self.inline * 1000 + self.ids * 100 + self.classes * 10 + self.elements
    }

    /// 从选择器字符串解析特异性
    pub fn from_selector(selector: &str) -> Self {
        let mut specificity = Self::new(0, 0, 0, 0);

        // 简单的选择器解析
        if selector.contains("style=") {
            specificity.inline += 1;
        }

        specificity.ids += selector.matches('#').count() as u32;
        specificity.classes += selector.matches('.').count() as u32;
        specificity.classes += selector.matches(':').count() as u32;
        specificity.classes += selector.matches('[').count() as u32;

        // 计算元素选择器（完整实现）
        let element_count = Self::calculate_element_selectors(selector);
        specificity.elements += element_count;

        specificity
    }

    /// 计算元素选择器数量（完整实现）
    fn calculate_element_selectors(selector: &str) -> u32 {
        let mut element_count = 0u32;

        // 移除伪元素和伪类，只计算真正的元素选择器
        let cleaned_selector = selector
            .replace("::", " ") // 移除伪元素
            .replace(":", " "); // 移除伪类

        // 按空格、>、+、~分割选择器
        let parts: Vec<&str> = cleaned_selector
            .split(|c: char| c.is_whitespace() || c == '>' || c == '+' || c == '~')
            .filter(|part| !part.is_empty())
            .collect();

        for part in parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            // 跳过类选择器、ID选择器、属性选择器
            if trimmed.starts_with('.') || trimmed.starts_with('#') || trimmed.starts_with('[') {
                continue;
            }

            // 检查是否是有效的HTML元素名
            if Self::is_valid_element_name(trimmed) {
                element_count += 1;
            }
        }

        element_count
    }

    /// 检查是否是有效的HTML元素名
    fn is_valid_element_name(name: &str) -> bool {
        // 常见的HTML元素列表
        const HTML_ELEMENTS: &[&str] = &[
            "a",
            "abbr",
            "address",
            "area",
            "article",
            "aside",
            "audio",
            "b",
            "base",
            "bdi",
            "bdo",
            "blockquote",
            "body",
            "br",
            "button",
            "canvas",
            "caption",
            "cite",
            "code",
            "col",
            "colgroup",
            "data",
            "datalist",
            "dd",
            "del",
            "details",
            "dfn",
            "dialog",
            "div",
            "dl",
            "dt",
            "em",
            "embed",
            "fieldset",
            "figcaption",
            "figure",
            "footer",
            "form",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "head",
            "header",
            "hr",
            "html",
            "i",
            "iframe",
            "img",
            "input",
            "ins",
            "kbd",
            "label",
            "legend",
            "li",
            "link",
            "main",
            "map",
            "mark",
            "meta",
            "meter",
            "nav",
            "noscript",
            "object",
            "ol",
            "optgroup",
            "option",
            "output",
            "p",
            "param",
            "picture",
            "pre",
            "progress",
            "q",
            "rp",
            "rt",
            "ruby",
            "s",
            "samp",
            "script",
            "section",
            "select",
            "small",
            "source",
            "span",
            "strong",
            "style",
            "sub",
            "summary",
            "sup",
            "table",
            "tbody",
            "td",
            "template",
            "textarea",
            "tfoot",
            "th",
            "thead",
            "time",
            "title",
            "tr",
            "track",
            "u",
            "ul",
            "var",
            "video",
            "wbr",
        ];

        // 检查是否在HTML元素列表中，或者符合自定义元素命名规则
        HTML_ELEMENTS.contains(&name.to_lowercase().as_str())
            || (name.contains('-') && name.chars().all(|c| c.is_alphanumeric() || c == '-'))
    }
}

impl StyleRule {
    /// 创建新的样式规则
    pub fn new(
        property: String,
        value: String,
        priority_type: PriorityType,
        source: StyleSource,
    ) -> Self {
        Self {
            property,
            value,
            priority_type: priority_type.clone(),
            priority_value: priority_type as u32,
            source,
            important: false,
            specificity: Specificity::new(0, 0, 0, 0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        }
    }

    /// 设置重要性
    pub fn with_important(mut self, important: bool) -> Self {
        self.important = important;
        self
    }

    /// 设置特异性
    pub fn with_specificity(mut self, specificity: Specificity) -> Self {
        self.specificity = specificity;
        self
    }

    /// 设置优先级值
    pub fn with_priority_value(mut self, priority_value: u32) -> Self {
        self.priority_value = priority_value;
        self
    }

    /// 计算最终优先级
    pub fn calculate_final_priority(&self, config: &PriorityConfig) -> u64 {
        let base_priority = config
            .priority_weights
            .get(&self.priority_type)
            .unwrap_or(&(self.priority_type.clone() as u32))
            .clone() as u64;

        let specificity_value = if config.enable_specificity {
            self.specificity.total_value() as u64
        } else {
            0
        };

        let timestamp_value = if config.enable_timestamp_ordering {
            self.timestamp / 1000 // 转换为毫秒
        } else {
            0
        };

        let important_multiplier = if self.important {
            config.important_multiplier as u64
        } else {
            1
        };

        (base_priority + self.priority_value as u64 + specificity_value) * important_multiplier
            + timestamp_value
    }
}

impl PriorityManager {
    /// 创建新的优先级管理器
    pub fn new() -> Self {
        Self {
            rules_by_property: HashMap::new(),
            priority_config: PriorityConfig::default(),
            default_resolution_strategy: ConflictResolutionStrategy::HighestPriority,
            strict_mode: false,
        }
    }

    /// 使用自定义配置创建
    pub fn with_config(config: PriorityConfig) -> Self {
        Self {
            rules_by_property: HashMap::new(),
            priority_config: config,
            default_resolution_strategy: ConflictResolutionStrategy::HighestPriority,
            strict_mode: false,
        }
    }

    /// 添加样式规则
    pub fn add_rule(&mut self, rule: StyleRule) {
        self.rules_by_property
            .entry(rule.property.clone())
            .or_insert_with(Vec::new)
            .push(rule);
    }

    /// 批量添加样式规则
    pub fn add_rules(&mut self, rules: Vec<StyleRule>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }

    /// 从变体样式添加规则
    pub fn add_variant_style(
        &mut self,
        variant_style: &VariantStyle,
        priority_type: PriorityType,
        source: StyleSource,
    ) {
        for (property, value) in &variant_style.properties {
            let rule = StyleRule::new(
                property.clone(),
                value.clone(),
                priority_type.clone(),
                source.clone(),
            )
            .with_priority_value(variant_style.priority);

            self.add_rule(rule);
        }
    }

    /// 解析样式优先级
    pub fn resolve_styles(&self) -> StyleResolutionResult {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let mut final_styles = HashMap::new();
        let mut applied_rules = Vec::new();
        let mut conflicts = Vec::new();
        let mut rules_by_source = HashMap::new();
        let mut total_rules = 0;

        // 按属性解析冲突
        for (property, rules) in &self.rules_by_property {
            total_rules += rules.len();

            // 统计规则来源
            for rule in rules {
                *rules_by_source.entry(rule.source.clone()).or_insert(0) += 1;
            }

            if rules.len() > 1 {
                // 检测冲突
                let conflict = PriorityConflict {
                    property: property.clone(),
                    conflicting_rules: rules.clone(),
                    resolution_strategy: self.default_resolution_strategy.clone(),
                };
                conflicts.push(conflict);
            }

            // 解析最终值
            if let Some(winning_rule) = self.resolve_property_conflict(rules) {
                final_styles.insert(property.clone(), winning_rule.value.clone());
                applied_rules.push(winning_rule.clone());
            }
        }

        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        StyleResolutionResult {
            final_styles,
            applied_rules,
            conflicts: conflicts.clone(),
            resolution_stats: ResolutionStats {
                total_rules,
                conflicts_count: conflicts.len(),
                resolution_time_us: end_time - start_time,
                rules_by_source,
            },
        }
    }

    /// 解析属性冲突
    fn resolve_property_conflict<'a>(&self, rules: &'a [StyleRule]) -> Option<&'a StyleRule> {
        if rules.is_empty() {
            return None;
        }

        if rules.len() == 1 {
            return Some(&rules[0]);
        }

        match self.default_resolution_strategy {
            ConflictResolutionStrategy::HighestPriority => self.resolve_by_priority(rules),
            ConflictResolutionStrategy::LastDefined => self.resolve_by_timestamp(rules),
            ConflictResolutionStrategy::Specificity => self.resolve_by_specificity(rules),
            ConflictResolutionStrategy::Importance => self.resolve_by_importance(rules),
            ConflictResolutionStrategy::Custom(_) => {
                // 自定义策略需要外部实现
                self.resolve_by_priority(rules)
            }
        }
    }

    /// 按优先级解析
    fn resolve_by_priority<'a>(&self, rules: &'a [StyleRule]) -> Option<&'a StyleRule> {
        rules
            .iter()
            .max_by_key(move |rule| rule.calculate_final_priority(&self.priority_config))
    }

    /// 按时间戳解析
    fn resolve_by_timestamp<'a>(&self, rules: &'a [StyleRule]) -> Option<&'a StyleRule> {
        rules.iter().max_by_key(move |rule| rule.timestamp)
    }

    /// 按特异性解析
    fn resolve_by_specificity<'a>(&self, rules: &'a [StyleRule]) -> Option<&'a StyleRule> {
        rules.iter().max_by_key(move |rule| &rule.specificity)
    }

    /// 按重要性解析
    fn resolve_by_importance<'a>(&self, rules: &'a [StyleRule]) -> Option<&'a StyleRule> {
        // 首先按重要性排序，然后按优先级
        rules
            .iter()
            .max_by(move |a, b| match (a.important, b.important) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => a
                    .calculate_final_priority(&self.priority_config)
                    .cmp(&b.calculate_final_priority(&self.priority_config)),
            })
    }

    /// 清除所有规则
    pub fn clear(&mut self) {
        self.rules_by_property.clear();
    }

    /// 清除特定属性的规则
    pub fn clear_property(&mut self, property: &str) {
        self.rules_by_property.remove(property);
    }

    /// 清除特定来源的规则
    pub fn clear_source(&mut self, source: &StyleSource) {
        for rules in self.rules_by_property.values_mut() {
            rules.retain(|rule| &rule.source != source);
        }

        // 清除空的属性条目
        self.rules_by_property.retain(|_, rules| !rules.is_empty());
    }

    /// 设置解决策略
    pub fn set_resolution_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.default_resolution_strategy = strategy;
    }

    /// 设置严格模式
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// 获取属性的所有规则
    pub fn get_property_rules(&self, property: &str) -> Option<&Vec<StyleRule>> {
        self.rules_by_property.get(property)
    }

    /// 获取冲突检测结果
    pub fn detect_conflicts(&self) -> Vec<PriorityConflict> {
        let mut conflicts = Vec::new();

        for (property, rules) in &self.rules_by_property {
            if rules.len() > 1 {
                conflicts.push(PriorityConflict {
                    property: property.clone(),
                    conflicting_rules: rules.clone(),
                    resolution_strategy: self.default_resolution_strategy.clone(),
                });
            }
        }

        conflicts
    }
}

impl Default for PriorityConfig {
    fn default() -> Self {
        let mut priority_weights = HashMap::new();
        priority_weights.insert(PriorityType::Base, 0);
        priority_weights.insert(PriorityType::Theme, 100);
        priority_weights.insert(PriorityType::Variant, 200);
        priority_weights.insert(PriorityType::State, 300);
        priority_weights.insert(PriorityType::Responsive, 400);
        priority_weights.insert(PriorityType::Conditional, 500);
        priority_weights.insert(PriorityType::Custom, 600);
        priority_weights.insert(PriorityType::Inline, 700);
        priority_weights.insert(PriorityType::Important, 1000);

        Self {
            priority_weights,
            enable_specificity: true,
            enable_timestamp_ordering: true,
            important_multiplier: 10,
        }
    }
}

impl Default for PriorityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specificity_calculation() {
        let specificity = Specificity::new(0, 1, 2, 3);
        assert_eq!(specificity.total_value(), 123);
    }

    #[test]
    fn test_specificity_from_selector() {
        let specificity = Specificity::from_selector(".class1 #id1 div:hover");
        assert_eq!(specificity.ids, 1);
        assert_eq!(specificity.classes, 2); // .class1 + :hover
    }

    #[test]
    fn test_style_rule_priority() {
        let rule = StyleRule::new(
            "color".to_string(),
            "red".to_string(),
            PriorityType::State,
            StyleSource::State("hover".to_string()),
        )
        .with_important(true);

        let config = PriorityConfig::default();
        let priority = rule.calculate_final_priority(&config);

        // 应该包含重要性倍数
        assert!(priority > 3000); // 300 * 10
    }

    #[test]
    fn test_priority_manager() {
        let mut manager = PriorityManager::new();

        let rule1 = StyleRule::new(
            "color".to_string(),
            "red".to_string(),
            PriorityType::Base,
            StyleSource::Base,
        );

        let rule2 = StyleRule::new(
            "color".to_string(),
            "blue".to_string(),
            PriorityType::State,
            StyleSource::State("hover".to_string()),
        );

        manager.add_rule(rule1);
        manager.add_rule(rule2);

        let result = manager.resolve_styles();

        // 状态样式应该获胜
        assert_eq!(result.final_styles.get("color"), Some(&"blue".to_string()));
        assert_eq!(result.conflicts.len(), 1);
    }

    #[test]
    fn test_conflict_detection() {
        let mut manager = PriorityManager::new();

        manager.add_rule(StyleRule::new(
            "margin".to_string(),
            "10px".to_string(),
            PriorityType::Base,
            StyleSource::Base,
        ));

        manager.add_rule(StyleRule::new(
            "margin".to_string(),
            "20px".to_string(),
            PriorityType::Variant,
            StyleSource::Variant("large".to_string()),
        ));

        let conflicts = manager.detect_conflicts();
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].property, "margin");
    }
}
