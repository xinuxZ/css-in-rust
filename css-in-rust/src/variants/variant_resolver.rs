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
        _variants: &HashMap<String, String>,
    ) -> Result<Option<ResponsiveStyleResult>, String> {
        // 这里需要根据实际的变体配置来解析响应式变体
        // 简化实现，返回空结果
        Ok(None)
    }

    /// 解析状态变体
    fn resolve_state_variants(
        &self,
        _variants: &HashMap<String, String>,
    ) -> Result<Option<StateVariantResult>, String> {
        // 这里需要根据实际的变体配置来解析状态变体
        // 简化实现，返回空结果
        Ok(None)
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
