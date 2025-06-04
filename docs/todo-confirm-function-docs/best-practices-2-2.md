# CSS-in-Rust 最佳实践指南（二-2）：组件设计与状态管理

本指南介绍 CSS-in-Rust 项目中组件设计、状态管理和动画系统的最佳实践。

## 🧩 组件设计最佳实践

### 1. 组件样式架构

#### 组件样式封装模式

```rust
use css_in_rust::{css, css_variants, theme_var};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 组件样式配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyleConfig {
    pub base_styles: String,
    pub variants: HashMap<String, VariantConfig>,
    pub states: HashMap<String, StateConfig>,
    pub modifiers: HashMap<String, ModifierConfig>,
}

/// 变体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantConfig {
    pub name: String,
    pub styles: String,
    pub description: String,
}

/// 状态配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub name: String,
    pub styles: String,
    pub trigger: String,
}

/// 修饰符配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierConfig {
    pub name: String,
    pub styles: String,
    pub combinable: bool,
}

/// 组件样式构建器
pub struct ComponentStyleBuilder {
    config: ComponentStyleConfig,
}

impl ComponentStyleBuilder {
    /// 创建新的组件样式构建器
    pub fn new() -> Self {
        Self {
            config: ComponentStyleConfig {
                base_styles: String::new(),
                variants: HashMap::new(),
                states: HashMap::new(),
                modifiers: HashMap::new(),
            },
        }
    }

    /// 设置基础样式
    pub fn base_styles(mut self, styles: &str) -> Self {
        self.config.base_styles = styles.to_string();
        self
    }

    /// 添加变体
    pub fn add_variant(mut self, name: &str, styles: &str, description: &str) -> Self {
        self.config.variants.insert(
            name.to_string(),
            VariantConfig {
                name: name.to_string(),
                styles: styles.to_string(),
                description: description.to_string(),
            },
        );
        self
    }

    /// 添加状态
    pub fn add_state(mut self, name: &str, styles: &str, trigger: &str) -> Self {
        self.config.states.insert(
            name.to_string(),
            StateConfig {
                name: name.to_string(),
                styles: styles.to_string(),
                trigger: trigger.to_string(),
            },
        );
        self
    }

    /// 添加修饰符
    pub fn add_modifier(mut self, name: &str, styles: &str, combinable: bool) -> Self {
        self.config.modifiers.insert(
            name.to_string(),
            ModifierConfig {
                name: name.to_string(),
                styles: styles.to_string(),
                combinable,
            },
        );
        self
    }

    /// 构建组件样式
    pub fn build(self) -> ComponentStyleConfig {
        self.config
    }

    /// 生成完整的 CSS
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // 基础样式
        css.push_str(&self.config.base_styles);
        css.push('\n');

        // 变体样式
        for (_, variant) in &self.config.variants {
            css.push_str(&variant.styles);
            css.push('\n');
        }

        // 状态样式
        for (_, state) in &self.config.states {
            css.push_str(&state.styles);
            css.push('\n');
        }

        // 修饰符样式
        for (_, modifier) in &self.config.modifiers {
            css.push_str(&modifier.styles);
            css.push('\n');
        }

        css
    }
}
```

#### 按钮组件设计示例

```rust
/// 按钮组件样式系统
pub fn create_button_component() -> ComponentStyleConfig {
    ComponentStyleBuilder::new()
        .base_styles(&css! {
            .button {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                border: none;
                border-radius: $theme(borders.radius.base);
                font-family: $theme(typography.font_families.sans);
                font-weight: $theme(typography.font_weights.medium);
                text-decoration: none;
                cursor: pointer;
                transition: all $theme(animations.durations.normal) $theme(animations.easings.ease_in_out);
                user-select: none;
                outline: none;
                position: relative;
                overflow: hidden;
            }

            .button:focus-visible {
                outline: 2px solid $theme(colors.primary.500);
                outline-offset: 2px;
            }
        }.to_string())

        // 尺寸变体
        .add_variant("small", &css! {
            .button--small {
                padding: $theme(spacing.1) $theme(spacing.3);
                font-size: $theme(typography.font_sizes.sm);
                line-height: $theme(typography.line_heights.tight);
                min-height: 2rem;
            }

            .button--small .button__icon {
                width: 1rem;
                height: 1rem;
                margin-right: $theme(spacing.1);
            }
        }.to_string(), "小尺寸按钮")

        .add_variant("medium", &css! {
            .button--medium {
                padding: $theme(spacing.2) $theme(spacing.4);
                font-size: $theme(typography.font_sizes.sm);
                line-height: $theme(typography.line_heights.normal);
                min-height: 2.5rem;
            }

            .button--medium .button__icon {
                width: 1.25rem;
                height: 1.25rem;
                margin-right: $theme(spacing.2);
            }
        }.to_string(), "中等尺寸按钮")

        .add_variant("large", &css! {
            .button--large {
                padding: $theme(spacing.3) $theme(spacing.6);
                font-size: $theme(typography.font_sizes.base);
                line-height: $theme(typography.line_heights.normal);
                min-height: 3rem;
            }

            .button--large .button__icon {
                width: 1.5rem;
                height: 1.5rem;
                margin-right: $theme(spacing.2);
            }
        }.to_string(), "大尺寸按钮")

        // 颜色变体
        .add_variant("primary", &css! {
            .button--primary {
                background-color: $theme(colors.primary.500);
                color: $theme(colors.text.inverse);
                border: 1px solid $theme(colors.primary.500);
            }

            .button--primary:hover {
                background-color: $theme(colors.primary.600);
                border-color: $theme(colors.primary.600);
                transform: translateY(-1px);
                box-shadow: $theme(shadows.md);
            }

            .button--primary:active {
                background-color: $theme(colors.primary.700);
                border-color: $theme(colors.primary.700);
                transform: translateY(0);
                box-shadow: $theme(shadows.sm);
            }
        }.to_string(), "主要按钮")

        .add_variant("secondary", &css! {
            .button--secondary {
                background-color: $theme(colors.background.surface);
                color: $theme(colors.text.primary);
                border: 1px solid $theme(colors.border.default);
            }

            .button--secondary:hover {
                background-color: $theme(colors.gray.50);
                border-color: $theme(colors.border.strong);
                transform: translateY(-1px);
                box-shadow: $theme(shadows.md);
            }

            .button--secondary:active {
                background-color: $theme(colors.gray.100);
                transform: translateY(0);
                box-shadow: $theme(shadows.sm);
            }
        }.to_string(), "次要按钮")

        .add_variant("outline", &css! {
            .button--outline {
                background-color: transparent;
                color: $theme(colors.primary.500);
                border: 1px solid $theme(colors.primary.500);
            }

            .button--outline:hover {
                background-color: $theme(colors.primary.50);
                color: $theme(colors.primary.600);
                border-color: $theme(colors.primary.600);
            }

            .button--outline:active {
                background-color: $theme(colors.primary.100);
                color: $theme(colors.primary.700);
                border-color: $theme(colors.primary.700);
            }
        }.to_string(), "轮廓按钮")

        .add_variant("ghost", &css! {
            .button--ghost {
                background-color: transparent;
                color: $theme(colors.text.primary);
                border: 1px solid transparent;
            }

            .button--ghost:hover {
                background-color: $theme(colors.gray.100);
                color: $theme(colors.text.primary);
            }

            .button--ghost:active {
                background-color: $theme(colors.gray.200);
            }
        }.to_string(), "幽灵按钮")

        // 状态
        .add_state("loading", &css! {
            .button--loading {
                pointer-events: none;
                opacity: 0.7;
            }

            .button--loading .button__text {
                opacity: 0;
            }

            .button--loading::after {
                content: "";
                position: absolute;
                width: 1rem;
                height: 1rem;
                border: 2px solid transparent;
                border-top: 2px solid currentColor;
                border-radius: 50%;
                animation: button-spin 1s linear infinite;
            }

            @keyframes button-spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
            }
        }.to_string(), "hover")

        .add_state("disabled", &css! {
            .button--disabled {
                opacity: 0.5;
                cursor: not-allowed;
                pointer-events: none;
            }
        }.to_string(), "disabled")

        // 修饰符
        .add_modifier("full-width", &css! {
            .button--full-width {
                width: 100%;
            }
        }.to_string(), true)

        .add_modifier("rounded", &css! {
            .button--rounded {
                border-radius: $theme(borders.radius.full);
            }
        }.to_string(), true)

        .add_modifier("elevated", &css! {
            .button--elevated {
                box-shadow: $theme(shadows.md);
            }

            .button--elevated:hover {
                box-shadow: $theme(shadows.lg);
            }
        }.to_string(), true)

        .build()
}
```

#### 卡片组件设计示例

```rust
/// 卡片组件样式系统
pub fn create_card_component() -> ComponentStyleConfig {
    ComponentStyleBuilder::new()
        .base_styles(&css! {
            .card {
                background-color: $theme(colors.background.surface);
                border: 1px solid $theme(colors.border.default);
                border-radius: $theme(borders.radius.lg);
                overflow: hidden;
                transition: all $theme(animations.durations.normal) $theme(animations.easings.ease_in_out);
            }

            .card__header {
                padding: $theme(spacing.6);
                border-bottom: 1px solid $theme(colors.border.muted);
            }

            .card__title {
                font-size: $theme(typography.font_sizes.lg);
                font-weight: $theme(typography.font_weights.semibold);
                color: $theme(colors.text.primary);
                margin: 0;
                line-height: $theme(typography.line_heights.tight);
            }

            .card__subtitle {
                font-size: $theme(typography.font_sizes.sm);
                color: $theme(colors.text.secondary);
                margin: $theme(spacing.1) 0 0 0;
                line-height: $theme(typography.line_heights.normal);
            }

            .card__content {
                padding: $theme(spacing.6);
            }

            .card__footer {
                padding: $theme(spacing.4) $theme(spacing.6);
                background-color: $theme(colors.background.secondary);
                border-top: 1px solid $theme(colors.border.muted);
                display: flex;
                align-items: center;
                justify-content: space-between;
            }

            .card__actions {
                display: flex;
                gap: $theme(spacing.2);
            }
        }.to_string())

        // 尺寸变体
        .add_variant("compact", &css! {
            .card--compact .card__header {
                padding: $theme(spacing.4);
            }

            .card--compact .card__content {
                padding: $theme(spacing.4);
            }

            .card--compact .card__footer {
                padding: $theme(spacing.3) $theme(spacing.4);
            }

            .card--compact .card__title {
                font-size: $theme(typography.font_sizes.base);
            }
        }.to_string(), "紧凑卡片")

        .add_variant("spacious", &css! {
            .card--spacious .card__header {
                padding: $theme(spacing.8);
            }

            .card--spacious .card__content {
                padding: $theme(spacing.8);
            }

            .card--spacious .card__footer {
                padding: $theme(spacing.6) $theme(spacing.8);
            }

            .card--spacious .card__title {
                font-size: $theme(typography.font_sizes.xl);
            }
        }.to_string(), "宽松卡片")

        // 样式变体
        .add_variant("outlined", &css! {
            .card--outlined {
                border: 2px solid $theme(colors.border.strong);
                background-color: transparent;
            }
        }.to_string(), "轮廓卡片")

        .add_variant("filled", &css! {
            .card--filled {
                background-color: $theme(colors.background.tertiary);
                border: none;
            }
        }.to_string(), "填充卡片")

        // 状态
        .add_state("hoverable", &css! {
            .card--hoverable {
                cursor: pointer;
            }

            .card--hoverable:hover {
                transform: translateY(-2px);
                box-shadow: $theme(shadows.lg);
                border-color: $theme(colors.border.focus);
            }

            .card--hoverable:active {
                transform: translateY(-1px);
                box-shadow: $theme(shadows.md);
            }
        }.to_string(), "hover")

        .add_state("selected", &css! {
            .card--selected {
                border-color: $theme(colors.primary.500);
                box-shadow: 0 0 0 1px $theme(colors.primary.500);
            }
        }.to_string(), "selected")

        // 修饰符
        .add_modifier("elevated", &css! {
            .card--elevated {
                box-shadow: $theme(shadows.md);
                border: none;
            }

            .card--elevated:hover {
                box-shadow: $theme(shadows.lg);
            }
        }.to_string(), true)

        .add_modifier("borderless", &css! {
            .card--borderless {
                border: none;
            }

            .card--borderless .card__header {
                border-bottom: none;
            }

            .card--borderless .card__footer {
                border-top: none;
            }
        }.to_string(), true)

        .build()
}
```

### 2. 组件状态管理

#### 状态驱动的样式系统

```rust
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 组件状态管理器
#[derive(Debug, Clone)]
pub struct ComponentStateManager {
    states: HashMap<String, ComponentState>,
    transitions: HashMap<String, StateTransition>,
}

/// 组件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub name: String,
    pub styles: String,
    pub data: HashMap<String, String>,
    pub is_active: bool,
}

/// 状态转换
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub trigger: String,
    pub animation: Option<String>,
    pub duration: Option<String>,
}

impl ComponentStateManager {
    /// 创建新的状态管理器
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            transitions: HashMap::new(),
        }
    }

    /// 添加状态
    pub fn add_state(&mut self, state: ComponentState) {
        self.states.insert(state.name.clone(), state);
    }

    /// 添加状态转换
    pub fn add_transition(&mut self, transition: StateTransition) {
        let key = format!("{}->{}", transition.from, transition.to);
        self.transitions.insert(key, transition);
    }

    /// 激活状态
    pub fn activate_state(&mut self, state_name: &str) -> Result<String, StateError> {
        // 先停用所有状态
        for state in self.states.values_mut() {
            state.is_active = false;
        }

        // 激活指定状态
        if let Some(state) = self.states.get_mut(state_name) {
            state.is_active = true;
            Ok(state.styles.clone())
        } else {
            Err(StateError::StateNotFound(state_name.to_string()))
        }
    }

    /// 获取当前活动状态的样式
    pub fn get_active_styles(&self) -> String {
        let mut styles = String::new();

        for state in self.states.values() {
            if state.is_active {
                styles.push_str(&state.styles);
                styles.push('\n');
            }
        }

        styles
    }

    /// 检查状态转换是否有效
    pub fn can_transition(&self, from: &str, to: &str) -> bool {
        let key = format!("{}->{}", from, to);
        self.transitions.contains_key(&key)
    }

    /// 执行状态转换
    pub fn transition_to(&mut self, to_state: &str) -> Result<TransitionResult, StateError> {
        let current_state = self.get_current_state_name();

        if let Some(current) = current_state {
            if !self.can_transition(&current, to_state) {
                return Err(StateError::InvalidTransition(current, to_state.to_string()));
            }

            let transition_key = format!("{}->{}", current, to_state);
            let transition = self.transitions.get(&transition_key).cloned();

            self.activate_state(to_state)?;

            Ok(TransitionResult {
                from: current,
                to: to_state.to_string(),
                styles: self.get_active_styles(),
                transition,
            })
        } else {
            self.activate_state(to_state)?;
            Ok(TransitionResult {
                from: "none".to_string(),
                to: to_state.to_string(),
                styles: self.get_active_styles(),
                transition: None,
            })
        }
    }

    /// 获取当前状态名称
    fn get_current_state_name(&self) -> Option<String> {
        self.states
            .values()
            .find(|state| state.is_active)
            .map(|state| state.name.clone())
    }
}

/// 状态转换结果
#[derive(Debug, Clone)]
pub struct TransitionResult {
    pub from: String,
    pub to: String,
    pub styles: String,
    pub transition: Option<StateTransition>,
}

/// 状态错误类型
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("状态 '{0}' 未找到")]
    StateNotFound(String),

    #[error("无效的状态转换: {0} -> {1}")]
    InvalidTransition(String, String),
}
```

#### 表单组件状态管理示例

```rust
/// 创建表单输入组件的状态管理
pub fn create_input_state_manager() -> ComponentStateManager {
    let mut manager = ComponentStateManager::new();

    // 默认状态
    manager.add_state(ComponentState {
        name: "default".to_string(),
        styles: css! {
            .input {
                border-color: $theme(colors.border.default);
                background-color: $theme(colors.background.surface);
                color: $theme(colors.text.primary);
            }

            .input::placeholder {
                color: $theme(colors.text.tertiary);
            }
        }.to_string(),
        data: HashMap::new(),
        is_active: true,
    });

    // 聚焦状态
    manager.add_state(ComponentState {
        name: "focused".to_string(),
        styles: css! {
            .input--focused {
                border-color: $theme(colors.primary.500);
                box-shadow: 0 0 0 1px $theme(colors.primary.500);
                outline: none;
            }

            .input--focused::placeholder {
                color: $theme(colors.text.secondary);
            }
        }.to_string(),
        data: HashMap::new(),
        is_active: false,
    });

    // 错误状态
    manager.add_state(ComponentState {
        name: "error".to_string(),
        styles: css! {
            .input--error {
                border-color: $theme(colors.error.500);
                background-color: $theme(colors.error.50);
                color: $theme(colors.error.700);
            }

            .input--error:focus {
                box-shadow: 0 0 0 1px $theme(colors.error.500);
            }
        }.to_string(),
        data: HashMap::new(),
        is_active: false,
    });

    // 成功状态
    manager.add_state(ComponentState {
        name: "success".to_string(),
        styles: css! {
            .input--success {
                border-color: $theme(colors.success.500);
                background-color: $theme(colors.success.50);
                color: $theme(colors.success.700);
            }

            .input--success:focus {
                box-shadow: 0 0 0 1px $theme(colors.success.500);
            }
        }.to_string(),
        data: HashMap::new(),
        is_active: false,
    });

    // 禁用状态
    manager.add_state(ComponentState {
        name: "disabled".to_string(),
        styles: css! {
            .input--disabled {
                background-color: $theme(colors.gray.100);
                border-color: $theme(colors.border.muted);
                color: $theme(colors.text.disabled);
                cursor: not-allowed;
                opacity: 0.6;
            }

            .input--disabled::placeholder {
                color: $theme(colors.text.disabled);
            }
        }.to_string(),
        data: HashMap::new(),
        is_active: false,
    });

    // 添加状态转换
    manager.add_transition(StateTransition {
        from: "default".to_string(),
        to: "focused".to_string(),
        trigger: "focus".to_string(),
        animation: Some("ease-in-out".to_string()),
        duration: Some("150ms".to_string()),
    });

    manager.add_transition(StateTransition {
        from: "focused".to_string(),
        to: "default".to_string(),
        trigger: "blur".to_string(),
        animation: Some("ease-in-out".to_string()),
        duration: Some("150ms".to_string()),
    });

    manager.add_transition(StateTransition {
        from: "default".to_string(),
        to: "error".to_string(),
        trigger: "validation_error".to_string(),
        animation: Some("ease-in-out".to_string()),
        duration: Some("200ms".to_string()),
    });

    manager.add_transition(StateTransition {
        from: "error".to_string(),
        to: "success".to_string(),
        trigger: "validation_success".to_string(),
        animation: Some("ease-in-out".to_string()),
        duration: Some("200ms".to_string()),
    });

    manager
}
```

### 3. 组件组合模式

#### 复合组件设计

```rust
/// 复合组件管理器
pub struct CompositeComponentManager {
    components: HashMap<String, ComponentStyleConfig>,
    relationships: Vec<ComponentRelationship>,
}

/// 组件关系
#[derive(Debug, Clone)]
pub struct ComponentRelationship {
    pub parent: String,
    pub child: String,
    pub relationship_type: RelationshipType,
    pub styles: String,
}

/// 关系类型
#[derive(Debug, Clone)]
pub enum RelationshipType {
    Contains,     // 包含关系
    Extends,      // 继承关系
    Modifies,     // 修饰关系
    Depends,      // 依赖关系
}

impl CompositeComponentManager {
    /// 创建复合组件管理器
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            relationships: Vec::new(),
        }
    }

    /// 注册组件
    pub fn register_component(&mut self, name: &str, config: ComponentStyleConfig) {
        self.components.insert(name.to_string(), config);
    }

    /// 添加组件关系
    pub fn add_relationship(&mut self, relationship: ComponentRelationship) {
        self.relationships.push(relationship);
    }

    /// 生成复合组件样式
    pub fn generate_composite_styles(&self, component_name: &str) -> Option<String> {
        let mut styles = String::new();

        // 获取基础组件样式
        if let Some(component) = self.components.get(component_name) {
            styles.push_str(&component.base_styles);
            styles.push('\n');

            // 添加变体样式
            for variant in component.variants.values() {
                styles.push_str(&variant.styles);
                styles.push('\n');
            }
        }

        // 添加关系样式
        for relationship in &self.relationships {
            if relationship.parent == component_name || relationship.child == component_name {
                styles.push_str(&relationship.styles);
                styles.push('\n');
            }
        }

        if styles.is_empty() {
            None
        } else {
            Some(styles)
        }
    }
}
```

#### 模态框复合组件示例

```rust
/// 创建模态框复合组件
pub fn create_modal_composite() -> CompositeComponentManager {
    let mut manager = CompositeComponentManager::new();

    // 注册模态框组件
    manager.register_component("modal", ComponentStyleBuilder::new()
        .base_styles(&css! {
            .modal {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                z-index: 1000;
                display: flex;
                align-items: center;
                justify-content: center;
                padding: $theme(spacing.4);
            }

            .modal__backdrop {
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background-color: $theme(colors.background.overlay);
                backdrop-filter: blur(4px);
            }

            .modal__content {
                position: relative;
                background-color: $theme(colors.background.surface);
                border-radius: $theme(borders.radius.lg);
                box-shadow: $theme(shadows.xl);
                max-width: 90vw;
                max-height: 90vh;
                overflow: hidden;
                animation: modal-enter $theme(animations.durations.normal) $theme(animations.easings.ease_out);
            }

            @keyframes modal-enter {
                from {
                    opacity: 0;
                    transform: scale(0.95) translateY(-10px);
                }
                to {
                    opacity: 1;
                    transform: scale(1) translateY(0);
                }
            }
        }.to_string())
        .build());

    // 注册模态框头部组件
    manager.register_component("modal-header", ComponentStyleBuilder::new()
        .base_styles(&css! {
            .modal__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: $theme(spacing.6);
                border-bottom: 1px solid $theme(colors.border.muted);
            }

            .modal__title {
                font-size: $theme(typography.font_sizes.lg);
                font-weight: $theme(typography.font_weights.semibold);
                color: $theme(colors.text.primary);
                margin: 0;
            }

            .modal__close {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 2rem;
                height: 2rem;
                border: none;
                background: none;
                border-radius: $theme(borders.radius.base);
                color: $theme(colors.text.secondary);
                cursor: pointer;
                transition: all $theme(animations.durations.fast) $theme(animations.easings.ease_in_out);
            }

            .modal__close:hover {
                background-color: $theme(colors.gray.100);
                color: $theme(colors.text.primary);
            }
        }.to_string())
        .build());

    // 注册模态框主体组件
    manager.register_component("modal-body", ComponentStyleBuilder::new()
        .base_styles(&css! {
            .modal__body {
                padding: $theme(spacing.6);
                overflow-y: auto;
                max-height: 60vh;
            }

            .modal__body::-webkit-scrollbar {
                width: 6px;
            }

            .modal__body::-webkit-scrollbar-track {
                background: $theme(colors.gray.100);
                border-radius: 3px;
            }

            .modal__body::-webkit-scrollbar-thumb {
                background: $theme(colors.gray.300);
                border-radius: 3px;
            }

            .modal__body::-webkit-scrollbar-thumb:hover {
                background: $theme(colors.gray.400);
            }
        }.to_string())
        .build());

    // 注册模态框底部组件
    manager.register_component("modal-footer", ComponentStyleBuilder::new()
        .base_styles(&css! {
            .modal__footer {
                display: flex;
                align-items: center;
                justify-content: flex-end;
                gap: $theme(spacing.3);
                padding: $theme(spacing.4) $theme(spacing.6);
                border-top: 1px solid $theme(colors.border.muted);
                background-color: $theme(colors.background.secondary);
            }
        }.to_string())
        .build());

    // 添加组件关系
    manager.add_relationship(ComponentRelationship {
        parent: "modal".to_string(),
        child: "modal-header".to_string(),
        relationship_type: RelationshipType::Contains,
        styles: css! {
            .modal .modal__header:first-child {
                border-top-left-radius: $theme(borders.radius.lg);
                border-top-right-radius: $theme(borders.radius.lg);
            }
        }.to_string(),
    });

    manager.add_relationship(ComponentRelationship {
        parent: "modal".to_string(),
        child: "modal-footer".to_string(),
        relationship_type: RelationshipType::Contains,
        styles: css! {
            .modal .modal__footer:last-child {
                border-bottom-left-radius: $theme(borders.radius.lg);
                border-bottom-right-radius: $theme(borders.radius.lg);
            }
        }.to_string(),
    });

    manager
}
```

## 🎭 动画系统最佳实践

### 1. 动画配置系统

#### 动画配置结构

```rust
/// 动画配置系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub durations: AnimationDurations,
    pub easings: AnimationEasings,
    pub keyframes: HashMap<String, String>,
    pub transitions: HashMap<String, TransitionConfig>,
}

/// 动画持续时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDurations {
    pub instant: String,   // 0ms
    pub fast: String,      // 150ms
    pub normal: String,    // 300ms
    pub slow: String,      // 500ms
    pub slower: String,    // 750ms
    pub slowest: String,   // 1000ms
}

/// 动画缓动函数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationEasings {
    pub linear: String,
    pub ease: String,
    pub ease_in: String,
    pub ease_out: String,
    pub ease_in_out: String,
    pub ease_in_back: String,
    pub ease_out_back: String,
    pub ease_in_out_back: String,
    pub bounce: String,
    pub elastic: String,
}

/// 过渡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionConfig {
    pub property: String,
    pub duration: String,
    pub easing: String,
    pub delay: Option<String>,
}
```

#### 动画管理器

```rust
/// 动画管理器
pub struct AnimationManager {
    config: AnimationConfig,
    active_animations: HashMap<String, AnimationInstance>,
}

/// 动画实例
#[derive(Debug, Clone)]
pub struct AnimationInstance {
    pub name: String,
    pub target: String,
    pub keyframes: String,
    pub duration: String,
    pub easing: String,
    pub iteration_count: String,
    pub fill_mode: String,
    pub play_state: AnimationPlayState,
}

/// 动画播放状态
#[derive(Debug, Clone)]
pub enum AnimationPlayState {
    Running,
    Paused,
    Finished,
}

impl AnimationManager {
    /// 创建动画管理器
    pub fn new(config: AnimationConfig) -> Self {
        Self {
            config,
            active_animations: HashMap::new(),
        }
    }

    /// 创建关键帧动画
    pub fn create_keyframe_animation(
        &mut self,
        name: &str,
        keyframes: &str,
        duration: &str,
        easing: &str,
    ) -> String {
        let animation_css = format!(
            r#"
            @keyframes {} {{
                {}
            }}

            .animate-{} {{
                animation: {} {} {} both;
            }}
            "#,
            name, keyframes, name, name, duration, easing
        );

        // 注册动画
        self.active_animations.insert(
            name.to_string(),
            AnimationInstance {
                name: name.to_string(),
                target: format!(".animate-{}", name),
                keyframes: keyframes.to_string(),
                duration: duration.to_string(),
                easing: easing.to_string(),
                iteration_count: "1".to_string(),
                fill_mode: "both".to_string(),
                play_state: AnimationPlayState::Running,
            },
        );

        animation_css
    }

    /// 创建过渡动画
    pub fn create_transition(
        &self,
        properties: &[&str],
        duration: &str,
        easing: &str,
        delay: Option<&str>,
    ) -> String {
        let transitions: Vec<String> = properties
            .iter()
            .map(|prop| {
                if let Some(d) = delay {
                    format!("{} {} {} {}", prop, duration, easing, d)
                } else {
                    format!("{} {} {}", prop, duration, easing)
                }
            })
            .collect();

        format!("transition: {};", transitions.join(", "))
    }

    /// 获取预定义动画
    pub fn get_predefined_animation(&self, name: &str) -> Option<String> {
        match name {
            "fade-in" => Some(self.create_fade_in_animation()),
            "fade-out" => Some(self.create_fade_out_animation()),
            "slide-in-up" => Some(self.create_slide_in_up_animation()),
            "slide-in-down" => Some(self.create_slide_in_down_animation()),
            "slide-in-left" => Some(self.create_slide_in_left_animation()),
            "slide-in-right" => Some(self.create_slide_in_right_animation()),
            "scale-in" => Some(self.create_scale_in_animation()),
            "scale-out" => Some(self.create_scale_out_animation()),
            "bounce-in" => Some(self.create_bounce_in_animation()),
            "shake" => Some(self.create_shake_animation()),
            "pulse" => Some(self.create_pulse_animation()),
            "spin" => Some(self.create_spin_animation()),
            _ => None,
        }
    }

    /// 淡入动画
    fn create_fade_in_animation(&self) -> String {
        css! {
            @keyframes fade-in {
                from {
                    opacity: 0;
                }
                to {
                    opacity: 1;
                }
            }

            .animate-fade-in {
                animation: fade-in $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 淡出动画
    fn create_fade_out_animation(&self) -> String {
        css! {
            @keyframes fade-out {
                from {
                    opacity: 1;
                }
                to {
                    opacity: 0;
                }
            }

            .animate-fade-out {
                animation: fade-out $theme(animations.durations.normal) $theme(animations.easings.ease_in) both;
            }
        }.to_string()
    }

    /// 向上滑入动画
    fn create_slide_in_up_animation(&self) -> String {
        css! {
            @keyframes slide-in-up {
                from {
                    opacity: 0;
                    transform: translateY(100%);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }

            .animate-slide-in-up {
                animation: slide-in-up $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 向下滑入动画
    fn create_slide_in_down_animation(&self) -> String {
        css! {
            @keyframes slide-in-down {
                from {
                    opacity: 0;
                    transform: translateY(-100%);
                }
                to {
                    opacity: 1;
                    transform: translateY(0);
                }
            }

            .animate-slide-in-down {
                animation: slide-in-down $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 从左滑入动画
    fn create_slide_in_left_animation(&self) -> String {
        css! {
            @keyframes slide-in-left {
                from {
                    opacity: 0;
                    transform: translateX(-100%);
                }
                to {
                    opacity: 1;
                    transform: translateX(0);
                }
            }

            .animate-slide-in-left {
                animation: slide-in-left $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 从右滑入动画
    fn create_slide_in_right_animation(&self) -> String {
        css! {
            @keyframes slide-in-right {
                from {
                    opacity: 0;
                    transform: translateX(100%);
                }
                to {
                    opacity: 1;
                    transform: translateX(0);
                }
            }

            .animate-slide-in-right {
                animation: slide-in-right $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 缩放进入动画
    fn create_scale_in_animation(&self) -> String {
        css! {
            @keyframes scale-in {
                from {
                    opacity: 0;
                    transform: scale(0.9);
                }
                to {
                    opacity: 1;
                    transform: scale(1);
                }
            }

            .animate-scale-in {
                animation: scale-in $theme(animations.durations.normal) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 缩放退出动画
    fn create_scale_out_animation(&self) -> String {
        css! {
            @keyframes scale-out {
                from {
                    opacity: 1;
                    transform: scale(1);
                }
                to {
                    opacity: 0;
                    transform: scale(0.9);
                }
            }

            .animate-scale-out {
                animation: scale-out $theme(animations.durations.normal) $theme(animations.easings.ease_in) both;
            }
        }.to_string()
    }

    /// 弹跳进入动画
    fn create_bounce_in_animation(&self) -> String {
        css! {
            @keyframes bounce-in {
                0% {
                    opacity: 0;
                    transform: scale(0.3);
                }
                50% {
                    opacity: 1;
                    transform: scale(1.05);
                }
                70% {
                    transform: scale(0.9);
                }
                100% {
                    opacity: 1;
                    transform: scale(1);
                }
            }

            .animate-bounce-in {
                animation: bounce-in $theme(animations.durations.slow) $theme(animations.easings.ease_out) both;
            }
        }.to_string()
    }

    /// 摇摆动画
    fn create_shake_animation(&self) -> String {
        css! {
            @keyframes shake {
                0%, 100% {
                    transform: translateX(0);
                }
                10%, 30%, 50%, 70%, 90% {
                    transform: translateX(-10px);
                }
                20%, 40%, 60%, 80% {
                    transform: translateX(10px);
                }
            }

            .animate-shake {
                animation: shake $theme(animations.durations.slow) $theme(animations.easings.ease_in_out) both;
            }
        }.to_string()
    }

    /// 脉冲动画
    fn create_pulse_animation(&self) -> String {
        css! {
            @keyframes pulse {
                0%, 100% {
                    opacity: 1;
                }
                50% {
                    opacity: 0.5;
                }
            }

            .animate-pulse {
                animation: pulse 2s $theme(animations.easings.ease_in_out) infinite;
            }
        }.to_string()
    }

    /// 旋转动画
    fn create_spin_animation(&self) -> String {
        css! {
            @keyframes spin {
                from {
                    transform: rotate(0deg);
                }
                to {
                    transform: rotate(360deg);
                }
            }

            .animate-spin {
                animation: spin 1s $theme(animations.easings.linear) infinite;
            }
        }.to_string()
    }
}
```

### 2. 动画工具类

#### 通用动画工具

```rust
/// 生成动画工具类
pub fn generate_animation_utilities() -> String {
    css! {
        /* 动画持续时间工具类 */
        .duration-75 { animation-duration: 75ms; }
        .duration-100 { animation-duration: 100ms; }
        .duration-150 { animation-duration: 150ms; }
        .duration-200 { animation-duration: 200ms; }
        .duration-300 { animation-duration: 300ms; }
        .duration-500 { animation-duration: 500ms; }
        .duration-700 { animation-duration: 700ms; }
        .duration-1000 { animation-duration: 1000ms; }

        /* 动画延迟工具类 */
        .delay-75 { animation-delay: 75ms; }
        .delay-100 { animation-delay: 100ms; }
        .delay-150 { animation-delay: 150ms; }
        .delay-200 { animation-delay: 200ms; }
        .delay-300 { animation-delay: 300ms; }
        .delay-500 { animation-delay: 500ms; }
        .delay-700 { animation-delay: 700ms; }
        .delay-1000 { animation-delay: 1000ms; }

        /* 动画缓动工具类 */
        .ease-linear { animation-timing-function: linear; }
        .ease-in { animation-timing-function: cubic-bezier(0.4, 0, 1, 1); }
        .ease-out { animation-timing-function: cubic-bezier(0, 0, 0.2, 1); }
        .ease-in-out { animation-timing-function: cubic-bezier(0.4, 0, 0.2, 1); }

        /* 动画填充模式工具类 */
        .fill-none { animation-fill-mode: none; }
        .fill-forwards { animation-fill-mode: forwards; }
        .fill-backwards { animation-fill-mode: backwards; }
        .fill-both { animation-fill-mode: both; }

        /* 动画播放状态工具类 */
        .animate-paused { animation-play-state: paused; }
        .animate-running { animation-play-state: running; }

        /* 动画迭代次数工具类 */
        .repeat-1 { animation-iteration-count: 1; }
        .repeat-2 { animation-iteration-count: 2; }
        .repeat-3 { animation-iteration-count: 3; }
        .repeat-infinite { animation-iteration-count: infinite; }

        /* 过渡工具类 */
        .transition-none { transition: none; }
        .transition-all { transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1); }
        .transition-colors { transition: color, background-color, border-color 150ms cubic-bezier(0.4, 0, 0.2, 1); }
        .transition-opacity { transition: opacity 150ms cubic-bezier(0.4, 0, 0.2, 1); }
        .transition-shadow { transition: box-shadow 150ms cubic-bezier(0.4, 0, 0.2, 1); }
        .transition-transform { transition: transform 150ms cubic-bezier(0.4, 0, 0.2, 1); }

        /* 变换工具类 */
        .transform { transform: translateX(0) translateY(0) rotate(0) skewX(0) skewY(0) scaleX(1) scaleY(1); }
        .transform-gpu { transform: translate3d(0, 0, 0); }
        .transform-none { transform: none; }

        /* 变换原点工具类 */
        .origin-center { transform-origin: center; }
        .origin-top { transform-origin: top; }
        .origin-top-right { transform-origin: top right; }
        .origin-right { transform-origin: right; }
        .origin-bottom-right { transform-origin: bottom right; }
        .origin-bottom { transform-origin: bottom; }
        .origin-bottom-left { transform-origin: bottom left; }
        .origin-left { transform-origin: left; }
        .origin-top-left { transform-origin: top left; }
    }.to_string()
}
```

通过这些组件设计、状态管理和动画系统的最佳实践，您可以构建出功能丰富、交互流畅的现代化组件库！🧩🎭
