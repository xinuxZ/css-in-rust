//! 状态变体模块
//!
//! 提供完整的状态变体支持，包括伪类状态、交互状态和自定义状态的管理。

use super::{VariantConfig, VariantStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 状态类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StateType {
    /// 悬停状态
    Hover,
    /// 焦点状态
    Focus,
    /// 激活状态
    Active,
    /// 禁用状态
    Disabled,
    /// 选中状态
    Checked,
    /// 访问过状态
    Visited,
    /// 选择状态
    Selected,
    /// 加载状态
    Loading,
    /// 第一个子元素
    FirstChild,
    /// 最后一个子元素
    LastChild,
    /// 奇数子元素
    NthChildOdd,
    /// 偶数子元素
    NthChildEven,
    /// 自定义状态
    Custom(String),
}

/// 状态变体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateVariant {
    /// 状态类型
    pub state_type: StateType,
    /// 样式定义
    pub style: VariantStyle,
    /// 是否可组合
    pub combinable: bool,
    /// 优先级
    pub priority: u32,
}

/// 状态组合
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateCombination {
    /// 状态列表
    pub states: Vec<StateType>,
    /// 组合样式
    pub style: VariantStyle,
    /// 组合优先级
    pub priority: u32,
}

/// 状态变体管理器
///
/// 负责状态变体的注册、管理和应用
#[derive(Debug, Clone)]
pub struct StateVariantManager {
    /// 状态变体映射
    variants: HashMap<StateType, StateVariant>,
    /// 状态组合映射
    combinations: HashMap<Vec<StateType>, StateCombination>,
    /// 当前激活的状态
    active_states: Vec<StateType>,
    /// 是否启用状态组合
    enable_combinations: bool,
}

/// 状态变体应用结果
#[derive(Debug, Clone)]
pub struct StateVariantResult {
    /// 基础样式
    pub base_styles: HashMap<String, String>,
    /// 状态样式映射
    pub state_styles: HashMap<StateType, HashMap<String, String>>,
    /// 伪选择器
    pub pseudo_selectors: HashMap<StateType, String>,
    /// 应用的状态
    pub applied_states: Vec<StateType>,
    /// CSS 输出
    pub css_output: String,
    /// 交互处理器
    pub interaction_handlers: HashMap<String, String>,
    /// 生成的 CSS
    pub css: String,
}

/// 状态选择器生成器
#[derive(Debug, Clone)]
pub struct StateSelector {
    /// 基础选择器
    base_selector: String,
    /// 状态修饰符
    state_modifiers: Vec<String>,
}

impl StateType {
    /// 转换为 CSS 伪类选择器
    pub fn to_css_selector(&self) -> String {
        match self {
            StateType::Hover => ":hover".to_string(),
            StateType::Focus => ":focus".to_string(),
            StateType::Active => ":active".to_string(),
            StateType::Disabled => ":disabled".to_string(),
            StateType::Checked => ":checked".to_string(),
            StateType::Visited => ":visited".to_string(),
            StateType::Selected => "[aria-selected='true']".to_string(),
            StateType::Loading => "[data-loading='true']".to_string(),
            StateType::FirstChild => ":first-child".to_string(),
            StateType::LastChild => ":last-child".to_string(),
            StateType::NthChildOdd => ":nth-child(odd)".to_string(),
            StateType::NthChildEven => ":nth-child(even)".to_string(),
            StateType::Custom(name) => format!(":{}", name),
        }
    }

    /// 获取状态名称
    pub fn name(&self) -> String {
        match self {
            StateType::Hover => "hover".to_string(),
            StateType::Focus => "focus".to_string(),
            StateType::Active => "active".to_string(),
            StateType::Disabled => "disabled".to_string(),
            StateType::Checked => "checked".to_string(),
            StateType::Visited => "visited".to_string(),
            StateType::Selected => "selected".to_string(),
            StateType::Loading => "loading".to_string(),
            StateType::FirstChild => "first-child".to_string(),
            StateType::LastChild => "last-child".to_string(),
            StateType::NthChildOdd => "nth-child-odd".to_string(),
            StateType::NthChildEven => "nth-child-even".to_string(),
            StateType::Custom(name) => name.clone(),
        }
    }

    /// 检查是否可以与其他状态组合
    pub fn can_combine_with(&self, other: &StateType) -> bool {
        match (self, other) {
            // 悬停和焦点可以组合
            (StateType::Hover, StateType::Focus) | (StateType::Focus, StateType::Hover) => true,
            // 悬停和激活可以组合
            (StateType::Hover, StateType::Active) | (StateType::Active, StateType::Hover) => true,
            // 禁用状态不能与交互状态组合
            (StateType::Disabled, StateType::Hover | StateType::Focus | StateType::Active) => false,
            (StateType::Hover | StateType::Focus | StateType::Active, StateType::Disabled) => false,
            // 其他情况默认可以组合
            _ => true,
        }
    }
}

impl StateVariantManager {
    /// 创建新的状态变体管理器
    pub fn new() -> Self {
        let mut manager = Self {
            variants: HashMap::new(),
            combinations: HashMap::new(),
            active_states: Vec::new(),
            enable_combinations: true,
        };

        // 初始化默认状态变体
        manager.init_default_variants();
        manager
    }

    /// 初始化默认状态变体
    fn init_default_variants(&mut self) {
        let default_variants = vec![
            StateVariant {
                state_type: StateType::Hover,
                style: VariantStyle {
                    properties: HashMap::from([
                        ("opacity".to_string(), "0.8".to_string()),
                        ("transition".to_string(), "all 0.2s ease".to_string()),
                    ]),
                    pseudo_classes: HashMap::new(),
                    priority: 10,
                },
                combinable: true,
                priority: 10,
            },
            StateVariant {
                state_type: StateType::Focus,
                style: VariantStyle {
                    properties: HashMap::from([
                        ("outline".to_string(), "2px solid #1890ff".to_string()),
                        ("outline-offset".to_string(), "2px".to_string()),
                    ]),
                    pseudo_classes: HashMap::new(),
                    priority: 15,
                },
                combinable: true,
                priority: 15,
            },
            StateVariant {
                state_type: StateType::Active,
                style: VariantStyle {
                    properties: HashMap::from([
                        ("transform".to_string(), "scale(0.98)".to_string()),
                        ("transition".to_string(), "transform 0.1s ease".to_string()),
                    ]),
                    pseudo_classes: HashMap::new(),
                    priority: 20,
                },
                combinable: true,
                priority: 20,
            },
            StateVariant {
                state_type: StateType::Disabled,
                style: VariantStyle {
                    properties: HashMap::from([
                        ("opacity".to_string(), "0.5".to_string()),
                        ("cursor".to_string(), "not-allowed".to_string()),
                        ("pointer-events".to_string(), "none".to_string()),
                    ]),
                    pseudo_classes: HashMap::new(),
                    priority: 30,
                },
                combinable: false,
                priority: 30,
            },
        ];

        for variant in default_variants {
            self.variants.insert(variant.state_type.clone(), variant);
        }
    }

    /// 注册状态变体
    pub fn register_variant(&mut self, variant: StateVariant) {
        self.variants.insert(variant.state_type.clone(), variant);
    }

    /// 注册状态组合
    pub fn register_combination(&mut self, combination: StateCombination) {
        self.combinations
            .insert(combination.states.clone(), combination);
    }

    /// 设置激活状态
    pub fn set_active_states(&mut self, states: Vec<StateType>) {
        self.active_states = states;
    }

    /// 添加激活状态
    pub fn add_active_state(&mut self, state: StateType) {
        if !self.active_states.contains(&state) {
            self.active_states.push(state);
        }
    }

    /// 移除激活状态
    pub fn remove_active_state(&mut self, state: &StateType) {
        self.active_states.retain(|s| s != state);
    }

    /// 应用状态变体
    pub fn apply_state_variants(
        &self,
        config: &VariantConfig,
        props: &HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, String>, String> {
        let mut state_styles = HashMap::new();

        // 处理状态变体
        for (variant_key, _variant_value) in props {
            if let Some(state_variant) = config.state.get(variant_key) {
                // 应用状态样式
                for (prop, value) in &state_variant.properties {
                    state_styles.insert(format!("{}:{}", prop, variant_key), value.clone());
                }
            }
        }

        // 处理激活状态
        for active_state in &self.active_states {
            if let Some(variant) = self.variants.get(active_state) {
                for (prop, value) in &variant.style.properties {
                    state_styles.insert(
                        format!("{}:{}", prop, active_state.to_css_selector()),
                        value.clone(),
                    );
                }
            }
        }

        // 处理状态组合
        if self.enable_combinations {
            self.apply_state_combinations(&mut state_styles);
        }

        Ok(state_styles)
    }

    /// 应用状态组合
    fn apply_state_combinations(&self, state_styles: &mut HashMap<String, String>) {
        // 查找匹配的状态组合
        for (combination_states, combination) in &self.combinations {
            if self.states_match(combination_states) {
                for (prop, value) in &combination.style.properties {
                    let selector = combination_states
                        .iter()
                        .map(|s| s.to_css_selector())
                        .collect::<Vec<_>>()
                        .join("");

                    state_styles.insert(format!("{}:{}", prop, selector), value.clone());
                }
            }
        }
    }

    /// 检查状态是否匹配
    fn states_match(&self, target_states: &[StateType]) -> bool {
        target_states
            .iter()
            .all(|state| self.active_states.contains(state))
    }

    /// 生成状态变体 CSS
    pub fn generate_state_css(
        &self,
        class_name: &str,
        state_styles: &HashMap<String, String>,
    ) -> StateVariantResult {
        let mut base_styles = HashMap::new();
        let mut state_style_map = HashMap::new();
        let mut css = String::new();

        // 分离基础样式和状态样式
        for (key, value) in state_styles {
            if let Some(colon_pos) = key.find(':') {
                let (prop, selector) = key.split_at(colon_pos);
                let selector = &selector[1..]; // 移除 ':' 符号

                // 解析状态类型
                if let Some(state_type) = self.parse_state_from_selector(selector) {
                    state_style_map
                        .entry(state_type)
                        .or_insert_with(HashMap::new)
                        .insert(prop.to_string(), value.clone());
                }
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

        // 生成状态样式 CSS
        let mut sorted_states: Vec<_> = state_style_map.keys().collect();
        sorted_states.sort_by(|a, b| {
            let priority_a = self.variants.get(a).map(|v| v.priority).unwrap_or(0);
            let priority_b = self.variants.get(b).map(|v| v.priority).unwrap_or(0);
            priority_a.cmp(&priority_b)
        });

        for state_type in sorted_states {
            if let Some(styles) = state_style_map.get(state_type) {
                css.push_str(&format!(
                    ".{}{} {{\n",
                    class_name,
                    state_type.to_css_selector()
                ));
                for (prop, value) in styles {
                    css.push_str(&format!("  {}: {};\n", prop, value));
                }
                css.push_str("}\n\n");
            }
        }

        StateVariantResult {
            base_styles,
            state_styles: state_style_map,
            pseudo_selectors: HashMap::new(),
            applied_states: Vec::new(),
            css_output: css.clone(),
            interaction_handlers: HashMap::new(),
            css,
        }
    }

    /// 从选择器解析状态类型
    fn parse_state_from_selector(&self, selector: &str) -> Option<StateType> {
        match selector {
            "hover" => Some(StateType::Hover),
            "focus" => Some(StateType::Focus),
            "active" => Some(StateType::Active),
            "disabled" => Some(StateType::Disabled),
            "checked" => Some(StateType::Checked),
            "visited" => Some(StateType::Visited),
            "first-child" => Some(StateType::FirstChild),
            "last-child" => Some(StateType::LastChild),
            "nth-child(odd)" => Some(StateType::NthChildOdd),
            "nth-child(even)" => Some(StateType::NthChildEven),
            _ => Some(StateType::Custom(selector.to_string())),
        }
    }

    /// 获取状态变体
    pub fn get_variant(&self, state_type: &StateType) -> Option<&StateVariant> {
        self.variants.get(state_type)
    }

    /// 获取所有状态变体
    pub fn get_all_variants(&self) -> &HashMap<StateType, StateVariant> {
        &self.variants
    }

    /// 检查状态是否激活
    pub fn is_state_active(&self, state_type: &StateType) -> bool {
        self.active_states.contains(state_type)
    }

    /// 启用/禁用状态组合
    pub fn set_combinations_enabled(&mut self, enabled: bool) {
        self.enable_combinations = enabled;
    }
}

impl StateSelector {
    /// 创建新的状态选择器
    pub fn new(base_selector: &str) -> Self {
        Self {
            base_selector: base_selector.to_string(),
            state_modifiers: Vec::new(),
        }
    }

    /// 添加状态修饰符
    pub fn with_state(mut self, state: &StateType) -> Self {
        self.state_modifiers.push(state.to_css_selector());
        self
    }

    /// 构建完整选择器
    pub fn build(self) -> String {
        if self.state_modifiers.is_empty() {
            self.base_selector
        } else {
            format!("{}{}", self.base_selector, self.state_modifiers.join(""))
        }
    }
}

impl Default for StateVariantManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷的状态变体创建函数
pub fn state_variant(state_type: StateType) -> StateVariantBuilder {
    StateVariantBuilder::new(state_type)
}

/// 状态变体构建器
#[derive(Debug, Clone)]
pub struct StateVariantBuilder {
    variant: StateVariant,
}

impl StateVariantBuilder {
    /// 创建新的构建器
    pub fn new(state_type: StateType) -> Self {
        Self {
            variant: StateVariant {
                state_type,
                style: VariantStyle {
                    properties: HashMap::new(),
                    pseudo_classes: HashMap::new(),
                    priority: 10,
                },
                combinable: true,
                priority: 10,
            },
        }
    }

    /// 设置样式属性
    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.variant
            .style
            .properties
            .insert(name.to_string(), value.to_string());
        self
    }

    /// 设置多个样式属性
    pub fn properties(mut self, properties: HashMap<String, String>) -> Self {
        self.variant.style.properties.extend(properties);
        self
    }

    /// 设置是否可组合
    pub fn combinable(mut self, combinable: bool) -> Self {
        self.variant.combinable = combinable;
        self
    }

    /// 设置优先级
    pub fn priority(mut self, priority: u32) -> Self {
        self.variant.priority = priority;
        self.variant.style.priority = priority;
        self
    }

    /// 构建状态变体
    pub fn build(self) -> StateVariant {
        self.variant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_type_css_selector() {
        assert_eq!(StateType::Hover.to_css_selector(), ":hover");
        assert_eq!(StateType::Focus.to_css_selector(), ":focus");
        assert_eq!(StateType::Active.to_css_selector(), ":active");
        assert_eq!(StateType::Disabled.to_css_selector(), ":disabled");
    }

    #[test]
    fn test_state_variant_manager_creation() {
        let manager = StateVariantManager::new();
        assert!(!manager.variants.is_empty());
        assert!(manager.variants.contains_key(&StateType::Hover));
        assert!(manager.variants.contains_key(&StateType::Focus));
    }

    #[test]
    fn test_state_combination() {
        assert!(StateType::Hover.can_combine_with(&StateType::Focus));
        assert!(StateType::Focus.can_combine_with(&StateType::Hover));
        assert!(!StateType::Disabled.can_combine_with(&StateType::Hover));
    }

    #[test]
    fn test_state_variant_builder() {
        let variant = state_variant(StateType::Hover)
            .property("opacity", "0.8")
            .property("transition", "all 0.2s ease")
            .combinable(true)
            .priority(10)
            .build();

        assert_eq!(variant.state_type, StateType::Hover);
        assert_eq!(
            variant.style.properties.get("opacity"),
            Some(&"0.8".to_string())
        );
        assert!(variant.combinable);
        assert_eq!(variant.priority, 10);
    }

    #[test]
    fn test_state_selector() {
        let selector = StateSelector::new(".button")
            .with_state(&StateType::Hover)
            .with_state(&StateType::Focus)
            .build();

        assert_eq!(selector, ".button:hover:focus");
    }
}
