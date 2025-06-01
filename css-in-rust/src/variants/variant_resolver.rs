//! 变体解析器模块
//!
//! 整合所有变体系统功能，提供统一的变体解析和应用接口。

use super::{
    conditional_styles::{
        ConditionEvaluationResult, ConditionValue, ConditionalStyleManager, DynamicStyleResult,
    },
    priority_manager::{PriorityManager, StyleResolutionResult},
    responsive::{ResponsiveManager, ResponsiveStyleResult},
    state_variants::{StateType, StateVariantManager, StateVariantResult},
    variant_types::{VariantCombination, VariantValue},
    VariantManager, VariantResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 变体解析上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResolutionContext {
    /// 当前断点
    pub current_breakpoints: Vec<String>,
    /// 当前状态
    pub current_states: Vec<StateType>,
    /// 当前 props
    pub current_props: HashMap<String, ConditionValue>,
    /// 主题上下文
    pub theme_context: Option<String>,
    /// 是否启用严格模式
    pub strict_mode: bool,
}

/// 变体解析选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResolutionOptions {
    /// 是否启用响应式
    pub enable_responsive: bool,
    /// 是否启用状态变体
    pub enable_state_variants: bool,
    /// 是否启用条件样式
    pub enable_conditional_styles: bool,
    /// 是否启用优先级管理
    pub enable_priority_management: bool,
    /// 是否启用缓存
    pub enable_caching: bool,
    /// 是否生成源映射
    pub generate_source_maps: bool,
}

/// 变体解析结果
#[derive(Debug, Clone)]
pub struct VariantResolutionResult {
    /// 最终样式
    pub final_styles: HashMap<String, String>,
    /// 生成的 CSS
    pub generated_css: String,
    /// 应用的变体
    pub applied_variants: Vec<String>,
    /// 响应式结果
    pub responsive_result: Option<ResponsiveStyleResult>,
    /// 状态变体结果
    pub state_result: Option<StateVariantResult>,
    /// 条件样式结果
    pub conditional_result: Option<ConditionEvaluationResult>,
    /// 动态样式结果
    pub dynamic_result: Option<DynamicStyleResult>,
    /// 优先级解析结果
    pub priority_result: Option<StyleResolutionResult>,
    /// 解析统计
    pub resolution_stats: VariantResolutionStats,
    /// 源映射
    pub source_maps: Option<SourceMap>,
}

/// 变体解析统计
#[derive(Debug, Clone)]
pub struct VariantResolutionStats {
    /// 总解析时间（微秒）
    pub total_resolution_time_us: u64,
    /// 各阶段耗时
    pub stage_timings: HashMap<String, u64>,
    /// 处理的变体数量
    pub processed_variants: usize,
    /// 生成的规则数量
    pub generated_rules: usize,
    /// 缓存命中次数
    pub cache_hits: usize,
    /// 缓存未命中次数
    pub cache_misses: usize,
}

/// 源映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    /// 样式到变体的映射
    pub style_to_variant: HashMap<String, String>,
    /// 变体到源的映射
    pub variant_to_source: HashMap<String, VariantSource>,
    /// CSS 行映射
    pub css_line_mappings: Vec<CssLineMapping>,
}

/// 变体源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantSource {
    /// 源类型
    pub source_type: String,
    /// 源文件
    pub source_file: Option<String>,
    /// 源行号
    pub source_line: Option<u32>,
    /// 源列号
    pub source_column: Option<u32>,
}

/// CSS 行映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssLineMapping {
    /// 生成的 CSS 行号
    pub generated_line: u32,
    /// 生成的 CSS 列号
    pub generated_column: u32,
    /// 源变体名称
    pub source_variant: String,
    /// 源属性名称
    pub source_property: String,
}

/// 变体解析器
///
/// 统一的变体解析和应用接口
#[derive(Debug, Clone)]
pub struct VariantResolver {
    /// 变体管理器
    variant_manager: VariantManager,
    /// 响应式管理器
    responsive_manager: ResponsiveManager,
    /// 状态变体管理器
    state_manager: StateVariantManager,
    /// 条件样式管理器
    conditional_manager: ConditionalStyleManager,
    /// 优先级管理器
    priority_manager: PriorityManager,
    /// 解析选项
    options: VariantResolutionOptions,
    /// 解析缓存
    resolution_cache: HashMap<String, VariantResolutionResult>,
    /// 是否启用缓存
    cache_enabled: bool,
    /// 解析上下文
    context: Option<VariantResolutionContext>,
}

impl VariantResolver {
    /// 创建新的变体解析器
    pub fn new() -> Self {
        Self {
            variant_manager: VariantManager::new(),
            responsive_manager: ResponsiveManager::new(),
            state_manager: StateVariantManager::new(),
            conditional_manager: ConditionalStyleManager::new(),
            priority_manager: PriorityManager::new(),
            options: VariantResolutionOptions::default(),
            resolution_cache: HashMap::new(),
            cache_enabled: true,
            context: None,
        }
    }

    /// 使用自定义选项创建
    pub fn with_options(options: VariantResolutionOptions) -> Self {
        let mut resolver = Self::new();
        resolver.options = options;
        resolver.cache_enabled = resolver.options.enable_caching;
        resolver
    }

    /// 解析变体
    pub fn resolve_variants(
        &mut self,
        variants: &HashMap<String, String>,
        context: &VariantResolutionContext,
    ) -> Result<VariantResolutionResult, String> {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        // 检查缓存
        let cache_key = self.generate_cache_key(variants, context);
        if self.cache_enabled {
            if let Some(cached_result) = self.resolution_cache.get(&cache_key) {
                return Ok(cached_result.clone());
            }
        }

        let mut stage_timings = HashMap::new();
        let mut final_styles = HashMap::new();
        let mut applied_variants = Vec::new();
        let mut generated_rules = 0;

        // 清理优先级管理器
        self.priority_manager.clear();

        // 1. 基础变体解析
        let base_start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let base_result = self.resolve_base_variants(variants)?;
        // 从CSS规则中解析样式属性
        let _css_rules = &base_result.css_rules;
        // 这里需要解析CSS规则来提取样式，暂时跳过
        // for (prop, value) in &base_result.styles {
        //     final_styles.insert(prop.clone(), value.clone());
        // }
        applied_variants.extend(base_result.applied_variants);

        let base_end = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        stage_timings.insert("base_variants".to_string(), base_end - base_start);

        // 2. 响应式变体解析
        let responsive_result = if self.options.enable_responsive {
            let responsive_start = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            self.responsive_manager
                .set_active_breakpoints(context.current_breakpoints.clone());
            let result = self.resolve_responsive_variants(variants)?;

            if let Some(ref result) = result {
                for (prop, value) in &result.base_styles {
                    final_styles.insert(prop.clone(), value.clone());
                }
                generated_rules += result.media_queries.len();
            }

            let responsive_end = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;
            stage_timings.insert(
                "responsive_variants".to_string(),
                responsive_end - responsive_start,
            );

            result
        } else {
            None
        };

        // 3. 状态变体解析
        let state_result = if self.options.enable_state_variants {
            let state_start = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            self.state_manager
                .set_active_states(context.current_states.clone());
            let result = self.resolve_state_variants(variants)?;

            if let Some(ref result) = result {
                for (prop, value) in &result.base_styles {
                    final_styles.insert(prop.clone(), value.clone());
                }
                generated_rules += result.state_styles.len();
            }

            let state_end = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;
            stage_timings.insert("state_variants".to_string(), state_end - state_start);

            result
        } else {
            None
        };

        // 4. 条件样式解析
        let (conditional_result, dynamic_result) = if self.options.enable_conditional_styles {
            let conditional_start = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            self.conditional_manager
                .set_props(context.current_props.clone());
            let conditional_result = self.conditional_manager.evaluate_conditional_styles();
            let dynamic_result = self.conditional_manager.compute_dynamic_styles();

            for (prop, value) in &conditional_result.applied_styles {
                final_styles.insert(prop.clone(), value.clone());
            }

            for (prop, value) in &dynamic_result.computed_styles {
                final_styles.insert(prop.clone(), value.clone());
            }

            let conditional_end = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;
            stage_timings.insert(
                "conditional_styles".to_string(),
                conditional_end - conditional_start,
            );

            (Some(conditional_result), Some(dynamic_result))
        } else {
            (None, None)
        };

        // 5. 优先级解析
        let priority_result = if self.options.enable_priority_management {
            let priority_start = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            let result = self.priority_manager.resolve_styles();

            // 使用优先级解析的最终样式
            final_styles = result.final_styles.clone();
            generated_rules += result.applied_rules.len();

            let priority_end = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;
            stage_timings.insert(
                "priority_resolution".to_string(),
                priority_end - priority_start,
            );

            Some(result)
        } else {
            None
        };

        // 6. 生成 CSS
        let css_start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let generated_css = self.generate_css(&final_styles, &responsive_result, &state_result);

        let css_end = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        stage_timings.insert("css_generation".to_string(), css_end - css_start);

        // 7. 生成源映射
        let source_maps = if self.options.generate_source_maps {
            Some(self.generate_source_maps(variants, &applied_variants))
        } else {
            None
        };

        let end_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let result = VariantResolutionResult {
            final_styles,
            generated_css,
            applied_variants,
            responsive_result,
            state_result,
            conditional_result,
            dynamic_result,
            priority_result,
            resolution_stats: VariantResolutionStats {
                total_resolution_time_us: end_time - start_time,
                stage_timings,
                processed_variants: variants.len(),
                generated_rules,
                cache_hits: 0,
                cache_misses: 1,
            },
            source_maps,
        };

        // 缓存结果
        if self.cache_enabled {
            self.resolution_cache.insert(cache_key, result.clone());
        }

        Ok(result)
    }

    /// 解析基础变体
    fn resolve_base_variants(
        &mut self,
        variants: &HashMap<String, String>,
    ) -> Result<VariantResult, String> {
        // 构建变体组合
        let mut variant_combination = VariantCombination {
            name: Some("base".to_string()),
            variants: HashMap::new(),
            priority: 10,
        };

        for (key, value) in variants {
            variant_combination
                .variants
                .insert(key.clone(), VariantValue::String(value.clone()));
        }

        // 应用变体
        let component_name = "base";
        let variant_map = variant_combination
            .variants
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect::<HashMap<String, String>>();
        let props = HashMap::new();
        self.variant_manager
            .apply_variants(component_name, &variant_map, &props)
    }

    /// 解析响应式变体
    fn resolve_responsive_variants(
        &self,
        variants: &HashMap<String, String>,
    ) -> Result<Option<ResponsiveStyleResult>, String> {
        // 检查是否有响应式变体
        let responsive_variants: HashMap<String, String> = variants
            .iter()
            .filter(|(key, _)| {
                // 检查是否为响应式变体（包含断点前缀）
                key.contains(":sm:")
                    || key.contains(":md:")
                    || key.contains(":lg:")
                    || key.contains(":xl:")
                    || key.contains(":2xl:")
                    || key.starts_with("sm:")
                    || key.starts_with("md:")
                    || key.starts_with("lg:")
                    || key.starts_with("xl:")
                    || key.starts_with("2xl:")
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if responsive_variants.is_empty() {
            return Ok(None);
        }

        // 使用响应式管理器解析
        {
            let mut breakpoint_styles = HashMap::new();

            for (variant_key, variant_value) in &responsive_variants {
                // 解析断点和样式
                let (breakpoint, style_key) = self.parse_responsive_variant(&variant_key)?;

                // 生成样式值
                let style_value = self.resolve_variant_value(&variant_value)?;

                // 添加到断点样式中
                breakpoint_styles
                    .entry(breakpoint)
                    .or_insert_with(HashMap::new)
                    .insert(style_key, style_value);
            }

            // 创建响应式样式结果
            let mut media_queries = HashMap::new();
            let generated_queries = self.generate_media_queries(&responsive_variants)?;
            for (breakpoint, query) in generated_queries {
                let mut styles = HashMap::new();
                styles.insert("query".to_string(), query);
                media_queries.insert(breakpoint, styles);
            }

            // 将断点样式转换为基础样式
            let mut base_styles = HashMap::new();
            for (_, styles) in &breakpoint_styles {
                for (key, value) in styles {
                    base_styles.insert(key.clone(), value.clone());
                }
            }

            let result = ResponsiveStyleResult {
                base_styles,
                media_queries: breakpoint_styles,
                css: self.generate_responsive_css(&responsive_variants)?,
            };

            Ok(Some(result))
        }
    }

    /// 解析响应式变体键
    fn parse_responsive_variant(&self, variant_key: &str) -> Result<(String, String), String> {
        // 解析格式如 "md:bg-blue-500" 或 "hover:md:text-red-500"
        let parts: Vec<&str> = variant_key.split(':').collect();

        for (i, part) in parts.iter().enumerate() {
            if ["sm", "md", "lg", "xl", "2xl"].contains(part) {
                let breakpoint = part.to_string();
                let style_key = if i + 1 < parts.len() {
                    parts[i + 1..].join(":")
                } else {
                    return Err(format!("无效的响应式变体格式: {}", variant_key));
                };
                return Ok((breakpoint, style_key));
            }
        }

        Err(format!("未找到有效的断点: {}", variant_key))
    }

    /// 生成媒体查询
    fn generate_media_queries(
        &self,
        variants: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut media_queries = HashMap::new();

        // 标准断点定义
        let breakpoints = [
            ("sm", "(min-width: 640px)"),
            ("md", "(min-width: 768px)"),
            ("lg", "(min-width: 1024px)"),
            ("xl", "(min-width: 1280px)"),
            ("2xl", "(min-width: 1536px)"),
        ];

        for (breakpoint, query) in breakpoints {
            if variants
                .keys()
                .any(|k| k.contains(&format!("{}:", breakpoint)))
            {
                media_queries.insert(breakpoint.to_string(), query.to_string());
            }
        }

        Ok(media_queries)
    }

    /// 获取应用的断点
    fn get_applied_breakpoints(&self) -> Result<Vec<String>, String> {
        // 从上下文中获取当前断点
        if let Some(context) = &self.context {
            Ok(context.current_breakpoints.clone())
        } else {
            Ok(vec!["base".to_string()]) // 默认断点
        }
    }

    /// 生成响应式CSS
    fn generate_responsive_css(
        &self,
        variants: &HashMap<String, String>,
    ) -> Result<String, String> {
        let mut css = String::new();
        let mut breakpoint_rules: HashMap<String, Vec<String>> = HashMap::new();

        for (variant_key, variant_value) in variants {
            let (breakpoint, style_key) = self.parse_responsive_variant(variant_key)?;
            let style_value = self.resolve_variant_value(variant_value)?;

            let css_rule = format!(
                "{}: {};",
                self.convert_to_css_property(&style_key)?,
                style_value
            );
            breakpoint_rules
                .entry(breakpoint)
                .or_insert_with(Vec::new)
                .push(css_rule);
        }

        // 生成媒体查询CSS
        let media_queries = self.generate_media_queries(variants)?;
        for (breakpoint, rules) in breakpoint_rules {
            if let Some(media_query) = media_queries.get(&breakpoint) {
                css.push_str(&format!("@media {} {{\n", media_query));
                for rule in rules {
                    css.push_str(&format!("  .responsive-{} {{ {} }}\n", breakpoint, rule));
                }
                css.push_str("}\n");
            }
        }

        Ok(css)
    }

    /// 解析状态变体
    fn resolve_state_variants(
        &self,
        variants: &HashMap<String, String>,
    ) -> Result<Option<StateVariantResult>, String> {
        // 检查是否有状态变体
        let state_variants: HashMap<String, String> = variants
            .iter()
            .filter(|(key, _)| {
                // 检查是否为状态变体（包含状态前缀）
                key.starts_with("hover:")
                    || key.starts_with("focus:")
                    || key.starts_with("active:")
                    || key.starts_with("disabled:")
                    || key.starts_with("visited:")
                    || key.starts_with("checked:")
                    || key.starts_with("selected:")
                    || key.starts_with("loading:")
                    || key.contains(":hover:")
                    || key.contains(":focus:")
                    || key.contains(":active:")
                    || key.contains(":disabled:")
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        if state_variants.is_empty() {
            return Ok(None);
        }

        // 使用状态变体管理器解析
        {
            let mut state_styles = HashMap::new();
            let mut pseudo_selectors = HashMap::new();

            for (variant_key, variant_value) in &state_variants {
                // 解析状态和样式
                let (state_type, style_key) = self.parse_state_variant(&variant_key)?;

                // 生成样式值
                let style_value = self.resolve_variant_value(&variant_value)?;

                // 添加到状态样式中
                state_styles
                    .entry(state_type.clone())
                    .or_insert_with(HashMap::new)
                    .insert(style_key.clone(), style_value.clone());

                // 生成伪选择器
                let pseudo_selector = self.generate_pseudo_selector(&state_type)?;
                pseudo_selectors.insert(state_type, pseudo_selector);
            }

            // 创建状态变体结果
            let result = StateVariantResult {
                base_styles: HashMap::new(),
                state_styles,
                pseudo_selectors,
                applied_states: self.get_applied_states()?,
                css_output: self.generate_state_css(&state_variants)?,
                interaction_handlers: self.generate_interaction_handlers(&state_variants)?,
                css: self.generate_state_css(&state_variants)?,
            };

            Ok(Some(result))
        }
    }

    /// 解析状态变体键
    fn parse_state_variant(&self, variant_key: &str) -> Result<(StateType, String), String> {
        // 解析格式如 "hover:bg-blue-500" 或 "md:hover:text-red-500"
        let parts: Vec<&str> = variant_key.split(':').collect();

        for (i, part) in parts.iter().enumerate() {
            let state_type = match *part {
                "hover" => StateType::Hover,
                "focus" => StateType::Focus,
                "active" => StateType::Active,
                "disabled" => StateType::Disabled,
                "visited" => StateType::Visited,
                "checked" => StateType::Checked,
                "selected" => StateType::Selected,
                "loading" => StateType::Loading,
                _ => continue,
            };

            let style_key = if i + 1 < parts.len() {
                parts[i + 1..].join(":")
            } else {
                return Err(format!("无效的状态变体格式: {}", variant_key));
            };

            return Ok((state_type, style_key));
        }

        Err(format!("未找到有效的状态类型: {}", variant_key))
    }

    /// 生成伪选择器
    fn generate_pseudo_selector(&self, state_type: &StateType) -> Result<String, String> {
        let selector = match state_type {
            StateType::Hover => ":hover",
            StateType::Focus => ":focus",
            StateType::Active => ":active",
            StateType::Disabled => ":disabled",
            StateType::Visited => ":visited",
            StateType::Checked => ":checked",
            StateType::Selected => "[aria-selected='true']",
            StateType::Loading => "[data-loading='true']",
            _ => return Err(format!("不支持的状态类型: {:?}", state_type)),
        };

        Ok(selector.to_string())
    }

    /// 获取应用的状态
    fn get_applied_states(&self) -> Result<Vec<StateType>, String> {
        // 从上下文中获取当前状态
        if let Some(context) = &self.context {
            Ok(context.current_states.clone())
        } else {
            Ok(vec![]) // 默认无状态
        }
    }

    /// 生成状态CSS
    fn generate_state_css(&self, variants: &HashMap<String, String>) -> Result<String, String> {
        let mut css = String::new();
        let mut state_rules: HashMap<StateType, Vec<String>> = HashMap::new();

        for (variant_key, variant_value) in variants {
            let (state_type, style_key) = self.parse_state_variant(variant_key)?;
            let style_value = self.resolve_variant_value(variant_value)?;

            let css_rule = format!(
                "{}: {};",
                self.convert_to_css_property(&style_key)?,
                style_value
            );
            state_rules
                .entry(state_type)
                .or_insert_with(Vec::new)
                .push(css_rule);
        }

        // 生成状态CSS规则
        for (state_type, rules) in state_rules {
            let pseudo_selector = self.generate_pseudo_selector(&state_type)?;
            css.push_str(&format!(".state-variant{} {{\n", pseudo_selector));
            for rule in rules {
                css.push_str(&format!("  {}\n", rule));
            }
            css.push_str("}\n");
        }

        Ok(css)
    }

    /// 生成交互处理器
    fn generate_interaction_handlers(
        &self,
        variants: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut handlers = HashMap::new();

        for (variant_key, _) in variants {
            let (state_type, _) = self.parse_state_variant(variant_key)?;

            let handler = match state_type {
                StateType::Hover => "onmouseenter, onmouseleave".to_string(),
                StateType::Focus => "onfocus, onblur".to_string(),
                StateType::Active => "onmousedown, onmouseup".to_string(),
                StateType::Checked => "onchange".to_string(),
                StateType::Selected => "onclick".to_string(),
                StateType::Loading => "data-loading".to_string(),
                _ => "none".to_string(),
            };

            handlers.insert(format!("{:?}", state_type), handler);
        }

        Ok(handlers)
    }

    /// 生成 CSS
    fn generate_css(
        &self,
        final_styles: &HashMap<String, String>,
        responsive_result: &Option<ResponsiveStyleResult>,
        state_result: &Option<StateVariantResult>,
    ) -> String {
        let mut css = String::new();

        // 基础样式
        if !final_styles.is_empty() {
            css.push_str(".variant-styles {\n");
            for (prop, value) in final_styles {
                css.push_str(&format!("  {}: {};\n", prop, value));
            }
            css.push_str("}\n\n");
        }

        // 响应式样式
        if let Some(responsive) = responsive_result {
            css.push_str(&responsive.css);
        }

        // 状态样式
        if let Some(state) = state_result {
            css.push_str(&state.css);
        }

        css
    }

    /// 生成源映射
    fn generate_source_maps(
        &self,
        _variants: &HashMap<String, String>,
        applied_variants: &[String],
    ) -> SourceMap {
        let style_to_variant = HashMap::new();
        let mut variant_to_source = HashMap::new();
        let css_line_mappings = Vec::new();

        for (i, variant_name) in applied_variants.iter().enumerate() {
            variant_to_source.insert(
                variant_name.clone(),
                VariantSource {
                    source_type: "variant".to_string(),
                    source_file: None,
                    source_line: Some(i as u32 + 1),
                    source_column: Some(1),
                },
            );
        }

        SourceMap {
            style_to_variant,
            variant_to_source,
            css_line_mappings,
        }
    }

    /// 生成缓存键
    fn generate_cache_key(
        &self,
        variants: &HashMap<String, String>,
        context: &VariantResolutionContext,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // 将HashMap转换为可hash的格式
        let mut variant_pairs: Vec<(&String, &String)> = variants.iter().collect();
        variant_pairs.sort_by_key(|(k, _)| *k);
        for (key, value) in variant_pairs {
            key.hash(&mut hasher);
            value.hash(&mut hasher);
        }

        context.current_breakpoints.hash(&mut hasher);
        context.current_states.hash(&mut hasher);
        // 暂时跳过theme_context的hash，因为可能也有类似问题
        // context.theme_context.hash(&mut hasher);

        format!("variant_cache_{}", hasher.finish())
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.resolution_cache.clear();
    }

    /// 设置解析选项
    pub fn set_options(&mut self, options: VariantResolutionOptions) {
        self.options = options;
        self.cache_enabled = self.options.enable_caching;
    }

    /// 获取变体管理器
    pub fn variant_manager(&mut self) -> &mut VariantManager {
        &mut self.variant_manager
    }

    /// 获取响应式管理器
    pub fn responsive_manager(&mut self) -> &mut ResponsiveManager {
        &mut self.responsive_manager
    }

    /// 获取状态管理器
    pub fn state_manager(&mut self) -> &mut StateVariantManager {
        &mut self.state_manager
    }

    /// 获取条件样式管理器
    pub fn conditional_manager(&mut self) -> &mut ConditionalStyleManager {
        &mut self.conditional_manager
    }

    /// 获取优先级管理器
    pub fn priority_manager(&mut self) -> &mut PriorityManager {
        &mut self.priority_manager
    }

    /// 将样式键转换为 CSS 属性名
    fn convert_to_css_property(&self, style_key: &str) -> Result<String, String> {
        // 处理常见的样式键到CSS属性的转换
        let css_property = match style_key {
            // 布局属性
            "w" | "width" => "width",
            "h" | "height" => "height",
            "m" | "margin" => "margin",
            "mt" | "margin-top" => "margin-top",
            "mr" | "margin-right" => "margin-right",
            "mb" | "margin-bottom" => "margin-bottom",
            "ml" | "margin-left" => "margin-left",
            "mx" => "margin-left", // 需要特殊处理
            "my" => "margin-top",  // 需要特殊处理
            "p" | "padding" => "padding",
            "pt" | "padding-top" => "padding-top",
            "pr" | "padding-right" => "padding-right",
            "pb" | "padding-bottom" => "padding-bottom",
            "pl" | "padding-left" => "padding-left",
            "px" => "padding-left", // 需要特殊处理
            "py" => "padding-top",  // 需要特殊处理

            // 颜色属性
            "bg" | "background" => "background-color",
            "color" | "text-color" => "color",
            "border-color" => "border-color",

            // 字体属性
            "font-size" | "text-size" => "font-size",
            "font-weight" => "font-weight",
            "font-family" => "font-family",
            "line-height" => "line-height",
            "text-align" => "text-align",

            // 显示属性
            "display" => "display",
            "position" => "position",
            "top" => "top",
            "right" => "right",
            "bottom" => "bottom",
            "left" => "left",
            "z-index" => "z-index",

            // Flexbox 属性
            "flex" => "flex",
            "flex-direction" => "flex-direction",
            "justify-content" => "justify-content",
            "align-items" => "align-items",
            "align-self" => "align-self",
            "flex-wrap" => "flex-wrap",

            // 边框属性
            "border" => "border",
            "border-width" => "border-width",
            "border-style" => "border-style",
            "border-radius" => "border-radius",

            // 其他属性
            "opacity" => "opacity",
            "transform" => "transform",
            "transition" => "transition",
            "animation" => "animation",
            "overflow" => "overflow",
            "cursor" => "cursor",

            // 如果没有匹配的，直接使用原始键名
            _ => style_key,
        };

        Ok(css_property.to_string())
    }

    /// 解析变体值
    fn resolve_variant_value(&self, variant_value: &str) -> Result<String, String> {
        // 处理变体值的解析，如颜色、尺寸等
        let resolved_value = match variant_value {
            // 颜色值
            value if value.starts_with('#') => value.to_string(),
            value if value.starts_with("rgb") => value.to_string(),
            value if value.starts_with("hsl") => value.to_string(),

            // 尺寸值
            value if value.ends_with("px") => value.to_string(),
            value if value.ends_with("rem") => value.to_string(),
            value if value.ends_with("em") => value.to_string(),
            value if value.ends_with("%") => value.to_string(),
            value if value.ends_with("vh") => value.to_string(),
            value if value.ends_with("vw") => value.to_string(),

            // 数字值（添加默认单位）
            value if value.parse::<f64>().is_ok() => {
                format!("{}px", value)
            }

            // 预定义的值
            "auto" => "auto".to_string(),
            "none" => "none".to_string(),
            "inherit" => "inherit".to_string(),
            "initial" => "initial".to_string(),
            "unset" => "unset".to_string(),

            // 其他值直接使用
            _ => variant_value.to_string(),
        };

        Ok(resolved_value)
    }

    // /// 获取应用的状态
    // fn get_applied_states(&self) -> Result<Vec<StateType>, String> {
    //     // 返回当前应用的状态列表
    //     // 这里可以从状态管理器中获取
    //     Ok(vec![])
    // }
}

impl Default for VariantResolutionOptions {
    fn default() -> Self {
        Self {
            enable_responsive: true,
            enable_state_variants: true,
            enable_conditional_styles: true,
            enable_priority_management: true,
            enable_caching: true,
            generate_source_maps: false,
        }
    }
}

impl Default for VariantResolutionContext {
    fn default() -> Self {
        Self {
            current_breakpoints: Vec::new(),
            current_states: Vec::new(),
            current_props: HashMap::new(),
            theme_context: None,
            strict_mode: false,
        }
    }
}

impl Default for VariantResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_resolver_creation() {
        let resolver = VariantResolver::new();
        assert!(resolver.cache_enabled);
    }

    #[test]
    fn test_variant_resolution_options() {
        let options = VariantResolutionOptions {
            enable_responsive: false,
            enable_state_variants: true,
            enable_conditional_styles: false,
            enable_priority_management: true,
            enable_caching: false,
            generate_source_maps: true,
        };

        let resolver = VariantResolver::with_options(options);
        assert!(!resolver.cache_enabled);
        assert!(!resolver.options.enable_responsive);
        assert!(resolver.options.generate_source_maps);
    }

    #[test]
    fn test_cache_key_generation() {
        let resolver = VariantResolver::new();
        let variants = HashMap::from([("size".to_string(), "large".to_string())]);
        let context = VariantResolutionContext::default();

        let key1 = resolver.generate_cache_key(&variants, &context);
        let key2 = resolver.generate_cache_key(&variants, &context);

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_source_map_generation() {
        let resolver = VariantResolver::new();
        let variants = HashMap::from([("size".to_string(), "large".to_string())]);
        let applied_variants = vec!["size-large".to_string()];

        let source_map = resolver.generate_source_maps(&variants, &applied_variants);
        assert!(source_map.variant_to_source.contains_key("size-large"));
    }
}
