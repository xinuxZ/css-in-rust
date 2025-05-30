//! 条件样式模块
//!
//! 提供基于 props 的动态样式生成系统，支持条件逻辑和动态样式计算。

use super::{VariantConfig, VariantStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 条件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    /// 等于
    Equals,
    /// 不等于
    NotEquals,
    /// 大于
    GreaterThan,
    /// 小于
    LessThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于等于
    LessThanOrEqual,
    /// 包含
    Contains,
    /// 不包含
    NotContains,
    /// 存在
    Exists,
    /// 不存在
    NotExists,
    /// 正则匹配
    Regex,
    /// 自定义函数
    Custom(String),
}

/// 条件值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionValue {
    /// 字符串值
    String(String),
    /// 数字值
    Number(f64),
    /// 布尔值
    Boolean(bool),
    /// 数组值
    Array(Vec<ConditionValue>),
    /// 对象值
    Object(HashMap<String, ConditionValue>),
    /// 空值
    Null,
}

/// 条件规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionRule {
    /// 属性名
    pub property: String,
    /// 条件类型
    pub condition_type: ConditionType,
    /// 期望值
    pub expected_value: ConditionValue,
    /// 是否取反
    pub negate: bool,
}

/// 条件组合
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionCombination {
    /// 单个条件
    Single(ConditionRule),
    /// AND 组合
    And(Vec<ConditionCombination>),
    /// OR 组合
    Or(Vec<ConditionCombination>),
    /// NOT 组合
    Not(Box<ConditionCombination>),
}

/// 条件样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionalStyle {
    /// 条件组合
    pub condition: ConditionCombination,
    /// 样式定义
    pub style: VariantStyle,
    /// 优先级
    pub priority: u32,
    /// 是否启用
    pub enabled: bool,
}

/// 动态样式规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicStyleRule {
    /// 规则名称
    pub name: String,
    /// 输入属性
    pub input_props: Vec<String>,
    /// 样式计算函数
    pub calculator: StyleCalculator,
    /// 缓存策略
    pub cache_strategy: CacheStrategy,
}

/// 样式计算器
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StyleCalculator {
    /// 简单映射
    SimpleMapping(HashMap<String, String>),
    /// 数值计算
    NumericCalculation {
        formula: String,
        unit: Option<String>,
    },
    /// 颜色计算
    ColorCalculation {
        base_color: String,
        adjustments: HashMap<String, f64>,
    },
    /// 自定义函数
    CustomFunction(String),
}

/// 缓存策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheStrategy {
    /// 不缓存
    None,
    /// 基于输入缓存
    InputBased,
    /// 基于时间缓存
    TimeBased { ttl_ms: u64 },
    /// 基于变更缓存
    ChangeBased,
}

/// 条件样式管理器
///
/// 负责条件样式的注册、评估和应用
#[derive(Debug, Clone)]
pub struct ConditionalStyleManager {
    /// 条件样式映射
    conditional_styles: HashMap<String, ConditionalStyle>,
    /// 动态样式规则
    dynamic_rules: HashMap<String, DynamicStyleRule>,
    /// 当前 props
    current_props: HashMap<String, ConditionValue>,
    /// 样式缓存
    style_cache: HashMap<String, (HashMap<String, String>, u64)>,
    /// 是否启用缓存
    cache_enabled: bool,
}

/// 条件评估结果
#[derive(Debug, Clone)]
pub struct ConditionEvaluationResult {
    /// 是否匹配
    pub matched: bool,
    /// 匹配的条件
    pub matched_conditions: Vec<String>,
    /// 应用的样式
    pub applied_styles: HashMap<String, String>,
}

/// 动态样式计算结果
#[derive(Debug, Clone)]
pub struct DynamicStyleResult {
    /// 计算的样式
    pub computed_styles: HashMap<String, String>,
    /// 是否来自缓存
    pub from_cache: bool,
    /// 计算时间（毫秒）
    pub computation_time_ms: u64,
}

impl ConditionType {
    /// 评估条件
    pub fn evaluate(&self, actual: &ConditionValue, expected: &ConditionValue) -> bool {
        match self {
            ConditionType::Equals => actual == expected,
            ConditionType::NotEquals => actual != expected,
            ConditionType::GreaterThan => self.compare_numeric(actual, expected, |a, b| a > b),
            ConditionType::LessThan => self.compare_numeric(actual, expected, |a, b| a < b),
            ConditionType::GreaterThanOrEqual => {
                self.compare_numeric(actual, expected, |a, b| a >= b)
            }
            ConditionType::LessThanOrEqual => self.compare_numeric(actual, expected, |a, b| a <= b),
            ConditionType::Contains => self.check_contains(actual, expected),
            ConditionType::NotContains => !self.check_contains(actual, expected),
            ConditionType::Exists => !matches!(actual, ConditionValue::Null),
            ConditionType::NotExists => matches!(actual, ConditionValue::Null),
            ConditionType::Regex => self.check_regex(actual, expected),
            ConditionType::Custom(_) => false, // 需要外部实现
        }
    }

    /// 数值比较
    fn compare_numeric<F>(&self, actual: &ConditionValue, expected: &ConditionValue, op: F) -> bool
    where
        F: Fn(f64, f64) -> bool,
    {
        match (actual, expected) {
            (ConditionValue::Number(a), ConditionValue::Number(b)) => op(*a, *b),
            _ => false,
        }
    }

    /// 包含检查
    fn check_contains(&self, actual: &ConditionValue, expected: &ConditionValue) -> bool {
        match (actual, expected) {
            (ConditionValue::String(s), ConditionValue::String(sub)) => s.contains(sub),
            (ConditionValue::Array(arr), val) => arr.contains(val),
            _ => false,
        }
    }

    /// 正则匹配
    fn check_regex(&self, actual: &ConditionValue, expected: &ConditionValue) -> bool {
        match (actual, expected) {
            (ConditionValue::String(s), ConditionValue::String(pattern)) => {
                // 简单的正则匹配实现
                // 在实际项目中应该使用 regex crate
                s.contains(pattern)
            }
            _ => false,
        }
    }
}

impl ConditionRule {
    /// 评估条件规则
    pub fn evaluate(&self, props: &HashMap<String, ConditionValue>) -> bool {
        let actual_value = props.get(&self.property).unwrap_or(&ConditionValue::Null);
        let result = self
            .condition_type
            .evaluate(actual_value, &self.expected_value);

        if self.negate {
            !result
        } else {
            result
        }
    }
}

impl ConditionCombination {
    /// 评估条件组合
    pub fn evaluate(&self, props: &HashMap<String, ConditionValue>) -> bool {
        match self {
            ConditionCombination::Single(rule) => rule.evaluate(props),
            ConditionCombination::And(combinations) => {
                combinations.iter().all(|c| c.evaluate(props))
            }
            ConditionCombination::Or(combinations) => {
                combinations.iter().any(|c| c.evaluate(props))
            }
            ConditionCombination::Not(combination) => !combination.evaluate(props),
        }
    }
}

impl ConditionalStyleManager {
    /// 创建新的条件样式管理器
    pub fn new() -> Self {
        Self {
            conditional_styles: HashMap::new(),
            dynamic_rules: HashMap::new(),
            current_props: HashMap::new(),
            style_cache: HashMap::new(),
            cache_enabled: true,
        }
    }

    /// 注册条件样式
    pub fn register_conditional_style(&mut self, name: &str, style: ConditionalStyle) {
        self.conditional_styles.insert(name.to_string(), style);
    }

    /// 注册动态样式规则
    pub fn register_dynamic_rule(&mut self, rule: DynamicStyleRule) {
        self.dynamic_rules.insert(rule.name.clone(), rule);
    }

    /// 设置当前 props
    pub fn set_props(&mut self, props: HashMap<String, ConditionValue>) {
        self.current_props = props;
        // 清除相关缓存
        if self.cache_enabled {
            self.invalidate_cache();
        }
    }

    /// 更新单个 prop
    pub fn update_prop(&mut self, key: &str, value: ConditionValue) {
        self.current_props.insert(key.to_string(), value);
        if self.cache_enabled {
            self.invalidate_prop_cache(key);
        }
    }

    /// 评估条件样式
    pub fn evaluate_conditional_styles(&self) -> ConditionEvaluationResult {
        let mut matched_conditions = Vec::new();
        let mut applied_styles = HashMap::new();
        let mut matched = false;

        // 按优先级排序
        let mut sorted_styles: Vec<_> = self.conditional_styles.iter().collect();
        sorted_styles.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));

        for (name, conditional_style) in sorted_styles {
            if conditional_style.enabled
                && conditional_style.condition.evaluate(&self.current_props)
            {
                matched = true;
                matched_conditions.push(name.clone());

                // 应用样式
                for (prop, value) in &conditional_style.style.properties {
                    applied_styles.insert(prop.clone(), value.clone());
                }
            }
        }

        ConditionEvaluationResult {
            matched,
            matched_conditions,
            applied_styles,
        }
    }

    /// 计算动态样式
    pub fn compute_dynamic_styles(&mut self) -> DynamicStyleResult {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut computed_styles = HashMap::new();
        let mut from_cache = false;

        // 克隆规则以避免借用冲突
        let rules_to_process: Vec<(String, DynamicStyleRule)> =
            self.dynamic_rules.clone().into_iter().collect();

        for (rule_name, rule) in rules_to_process {
            if self.cache_enabled {
                if let Some(cached_styles) = self.get_cached_styles(&rule_name, &rule) {
                    computed_styles.extend(cached_styles);
                    from_cache = true;
                    continue;
                }
            }

            // 计算样式
            if let Some(rule_styles) = self.compute_rule_styles(&rule) {
                computed_styles.extend(rule_styles.clone());

                if self.cache_enabled {
                    self.cache_styles(&rule_name, rule_styles, start_time);
                }
            }
        }

        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        DynamicStyleResult {
            computed_styles,
            from_cache,
            computation_time_ms: end_time - start_time,
        }
    }

    /// 计算规则样式
    fn compute_rule_styles(&self, rule: &DynamicStyleRule) -> Option<HashMap<String, String>> {
        let mut styles = HashMap::new();

        match &rule.calculator {
            StyleCalculator::SimpleMapping(mapping) => {
                for input_prop in &rule.input_props {
                    if let Some(prop_value) = self.current_props.get(input_prop) {
                        if let ConditionValue::String(value_str) = prop_value {
                            if let Some(mapped_value) = mapping.get(value_str) {
                                styles.insert(input_prop.clone(), mapped_value.clone());
                            }
                        }
                    }
                }
            }
            StyleCalculator::NumericCalculation { formula, unit } => {
                if let Some(computed_value) =
                    self.evaluate_numeric_formula(formula, &rule.input_props)
                {
                    let value_with_unit = if let Some(u) = unit {
                        format!("{}{}", computed_value, u)
                    } else {
                        computed_value.to_string()
                    };
                    styles.insert("computed-value".to_string(), value_with_unit);
                }
            }
            StyleCalculator::ColorCalculation {
                base_color,
                adjustments,
            } => {
                if let Some(adjusted_color) = self.adjust_color(base_color, adjustments) {
                    styles.insert("color".to_string(), adjusted_color);
                }
            }
            StyleCalculator::CustomFunction(_) => {
                // 自定义函数需要外部实现
            }
        }

        if styles.is_empty() {
            None
        } else {
            Some(styles)
        }
    }

    /// 评估数值公式
    fn evaluate_numeric_formula(&self, formula: &str, input_props: &[String]) -> Option<f64> {
        // 简单的公式评估实现
        // 在实际项目中应该使用更强大的表达式解析器

        let mut result = 0.0;
        let mut found_value = false;

        for prop in input_props {
            if let Some(ConditionValue::Number(value)) = self.current_props.get(prop) {
                if formula.contains(&format!("{{{}}}", prop)) {
                    result += value;
                    found_value = true;
                }
            }
        }

        if found_value {
            Some(result)
        } else {
            None
        }
    }

    /// 调整颜色
    fn adjust_color(&self, base_color: &str, adjustments: &HashMap<String, f64>) -> Option<String> {
        // 简单的颜色调整实现
        // 在实际项目中应该使用专门的颜色处理库

        if let Some(brightness) = adjustments.get("brightness") {
            // 简单的亮度调整
            Some(format!("brightness({}) {}", brightness, base_color))
        } else {
            Some(base_color.to_string())
        }
    }

    /// 获取缓存样式
    fn get_cached_styles(
        &self,
        rule_name: &str,
        rule: &DynamicStyleRule,
    ) -> Option<HashMap<String, String>> {
        if let Some((cached_styles, cache_time)) = self.style_cache.get(rule_name) {
            match &rule.cache_strategy {
                CacheStrategy::None => None,
                CacheStrategy::InputBased => Some(cached_styles.clone()),
                CacheStrategy::TimeBased { ttl_ms } => {
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    if current_time - cache_time < *ttl_ms {
                        Some(cached_styles.clone())
                    } else {
                        None
                    }
                }
                CacheStrategy::ChangeBased => Some(cached_styles.clone()),
            }
        } else {
            None
        }
    }

    /// 缓存样式
    fn cache_styles(&mut self, rule_name: &str, styles: HashMap<String, String>, timestamp: u64) {
        self.style_cache
            .insert(rule_name.to_string(), (styles, timestamp));
    }

    /// 清除缓存
    fn invalidate_cache(&mut self) {
        self.style_cache.clear();
    }

    /// 清除特定 prop 相关的缓存
    fn invalidate_prop_cache(&mut self, _prop: &str) {
        // 简单实现：清除所有缓存
        // 在实际项目中可以更精确地清除相关缓存
        self.style_cache.clear();
    }

    /// 启用/禁用缓存
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
        if !enabled {
            self.invalidate_cache();
        }
    }

    /// 获取当前 props
    pub fn get_props(&self) -> &HashMap<String, ConditionValue> {
        &self.current_props
    }

    /// 获取条件样式
    pub fn get_conditional_style(&self, name: &str) -> Option<&ConditionalStyle> {
        self.conditional_styles.get(name)
    }

    /// 获取动态规则
    pub fn get_dynamic_rule(&self, name: &str) -> Option<&DynamicStyleRule> {
        self.dynamic_rules.get(name)
    }
}

impl Default for ConditionalStyleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷的条件样式创建函数
pub fn conditional_style(condition: ConditionCombination) -> ConditionalStyleBuilder {
    ConditionalStyleBuilder::new(condition)
}

/// 条件样式构建器
#[derive(Debug, Clone)]
pub struct ConditionalStyleBuilder {
    style: ConditionalStyle,
}

impl ConditionalStyleBuilder {
    /// 创建新的构建器
    pub fn new(condition: ConditionCombination) -> Self {
        Self {
            style: ConditionalStyle {
                condition,
                style: VariantStyle {
                    properties: HashMap::new(),
                    pseudo_classes: HashMap::new(),
                    priority: 10,
                },
                priority: 10,
                enabled: true,
            },
        }
    }

    /// 设置样式属性
    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.style
            .style
            .properties
            .insert(name.to_string(), value.to_string());
        self
    }

    /// 设置多个样式属性
    pub fn properties(mut self, properties: HashMap<String, String>) -> Self {
        self.style.style.properties.extend(properties);
        self
    }

    /// 设置优先级
    pub fn priority(mut self, priority: u32) -> Self {
        self.style.priority = priority;
        self.style.style.priority = priority;
        self
    }

    /// 设置是否启用
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.style.enabled = enabled;
        self
    }

    /// 构建条件样式
    pub fn build(self) -> ConditionalStyle {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_evaluation() {
        let rule = ConditionRule {
            property: "size".to_string(),
            condition_type: ConditionType::Equals,
            expected_value: ConditionValue::String("large".to_string()),
            negate: false,
        };

        let mut props = HashMap::new();
        props.insert(
            "size".to_string(),
            ConditionValue::String("large".to_string()),
        );

        assert!(rule.evaluate(&props));
    }

    #[test]
    fn test_condition_combination() {
        let combination = ConditionCombination::And(vec![
            ConditionCombination::Single(ConditionRule {
                property: "size".to_string(),
                condition_type: ConditionType::Equals,
                expected_value: ConditionValue::String("large".to_string()),
                negate: false,
            }),
            ConditionCombination::Single(ConditionRule {
                property: "color".to_string(),
                condition_type: ConditionType::Equals,
                expected_value: ConditionValue::String("blue".to_string()),
                negate: false,
            }),
        ]);

        let mut props = HashMap::new();
        props.insert(
            "size".to_string(),
            ConditionValue::String("large".to_string()),
        );
        props.insert(
            "color".to_string(),
            ConditionValue::String("blue".to_string()),
        );

        assert!(combination.evaluate(&props));
    }

    #[test]
    fn test_conditional_style_manager() {
        let mut manager = ConditionalStyleManager::new();

        let condition = ConditionCombination::Single(ConditionRule {
            property: "variant".to_string(),
            condition_type: ConditionType::Equals,
            expected_value: ConditionValue::String("primary".to_string()),
            negate: false,
        });

        let style = conditional_style(condition)
            .property("background-color", "#1890ff")
            .property("color", "white")
            .priority(10)
            .build();

        manager.register_conditional_style("primary-variant", style);

        let mut props = HashMap::new();
        props.insert(
            "variant".to_string(),
            ConditionValue::String("primary".to_string()),
        );
        manager.set_props(props);

        let result = manager.evaluate_conditional_styles();
        assert!(result.matched);
        assert_eq!(
            result.applied_styles.get("background-color"),
            Some(&"#1890ff".to_string())
        );
    }
}
